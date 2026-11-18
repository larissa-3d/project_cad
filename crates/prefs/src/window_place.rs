//! Where the window stood when the application closed, and where it opens next.
//!
//! Everything here is in the desktop's own units — points on macOS, pixels
//! elsewhere — the ones the system lays its monitors out in. The shell maps
//! what the windowing library reports onto them.

use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::locations::Locations;
use crate::ports::Files;
use crate::storage::StorageError;

/// The size a window takes when nothing else decides it, in points.
const DEFAULT_SIZE_IN_POINTS: (u32, u32) = (1280, 800);

/// The window's outer top-left corner and inner size, and whether it is
/// maximised. Maximised, the rect is still the one it goes back to once
/// restored, never the screen's. `least_height` is the inner height the
/// application last held the window to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowPlace {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub maximised: bool,
    #[serde(default)]
    pub least_height: Option<u32>,
}

/// A monitor as the system lays it out. `scale` is how many desktop units a
/// point takes on it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Monitor {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale: f64,
}

/// Where the window opens: the remembered place while the setting is on and
/// its centre lies on one of the `monitors`; otherwise maximised, over the
/// default size centred on the main monitor, the first of the slice. With no
/// monitor reported at all, the remembered place stands, or the default size
/// at the origin, and the system places the window. Whichever it is, it keeps
/// the remembered least height, held to the height of its screen, and is no
/// shorter than that; a window raised to it moves up rather than run off the
/// screen's bottom.
pub fn where_the_window_opens(
    remembered: Option<WindowPlace>,
    reopen_at_last_size: bool,
    monitors: &[Monitor],
) -> WindowPlace {
    let holding = |place: WindowPlace| {
        monitors
            .iter()
            .find(|monitor| monitor.holds_the_centre_of(place))
    };
    let (place, screen) = match remembered {
        Some(place) if reopen_at_last_size && (monitors.is_empty() || holding(place).is_some()) => {
            (place, holding(place))
        }
        _ => (
            maximised_over_the_default_size(monitors.first()),
            monitors.first(),
        ),
    };
    let on_the_screen = |height: u32| screen.map_or(height, |screen| height.min(screen.height));
    let least_height = remembered
        .and_then(|place| place.least_height)
        .map(on_the_screen);
    let height = place.height.max(least_height.unwrap_or(0));
    let y = match screen {
        Some(screen) if height != place.height => {
            let bottom = i64::from(screen.y) + i64::from(screen.height) - i64::from(height);
            i64::from(place.y).min(bottom) as i32
        }
        _ => place.y,
    };
    WindowPlace {
        y,
        height,
        least_height,
        ..place
    }
}

fn maximised_over_the_default_size(main: Option<&Monitor>) -> WindowPlace {
    let scale = main.map_or(1.0, |main| main.scale);
    let in_units = |points: u32| (f64::from(points) * scale).round() as u32;
    let (width, height) = (
        in_units(DEFAULT_SIZE_IN_POINTS.0),
        in_units(DEFAULT_SIZE_IN_POINTS.1),
    );
    let (x, y) = main.map_or((0, 0), |main| {
        (
            centred(main.x, main.width, width),
            centred(main.y, main.height, height),
        )
    });
    WindowPlace {
        x,
        y,
        width,
        height,
        maximised: true,
        least_height: None,
    }
}

fn centred(start: i32, span: u32, length: u32) -> i32 {
    (i64::from(start) + (i64::from(span) - i64::from(length)) / 2) as i32
}

/// What the window shows of itself on a frame: its outer top-left corner, its
/// inner size, and what state the system has it in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowSeen {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub maximised: bool,
    pub minimised: bool,
    pub fullscreen: bool,
}

/// How long a rect stands unchanged before it is the one to go back to. macOS
/// animates a zoom through rects it does not call maximised, a frame apart:
/// none of them stands this long.
pub const SETTLE: Duration = Duration::from_secs(1);

/// The window followed frame by frame, for the place to write on close.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FollowedWindow {
    /// The last rect that settled; its `maximised` is never read.
    to_go_back_to: WindowPlace,
    standing: Standing,
    least_height: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Standing {
    Maximised,
    Restored { rect: WindowPlace, since: Duration },
}

impl FollowedWindow {
    /// From the place the window opened at, which is the rect it goes back to.
    pub fn opened_at(place: WindowPlace) -> Self {
        let rect = WindowPlace {
            maximised: false,
            least_height: None,
            ..place
        };
        let standing = if place.maximised {
            Standing::Maximised
        } else {
            Standing::Restored {
                rect,
                since: Duration::ZERO,
            }
        };
        Self {
            to_go_back_to: rect,
            standing,
            least_height: place.least_height,
        }
    }

    /// The application held the window to an inner height of at least
    /// `height`, the one to write with the place unless another follows.
    pub fn asked_no_shorter_than(&mut self, height: u32) {
        self.least_height = Some(height);
    }

    /// The window `seen` `at` a time counted from the origin `opened_at`
    /// stands for. Minimised or full screen, it shows no place and changes
    /// nothing.
    pub fn saw(&mut self, seen: WindowSeen, at: Duration) {
        if seen.minimised || seen.fullscreen {
            return;
        }
        if let Standing::Restored { rect, since } = self.standing
            && at.saturating_sub(since) >= SETTLE
        {
            self.to_go_back_to = rect;
        }
        let rect = WindowPlace {
            x: seen.x,
            y: seen.y,
            width: seen.width,
            height: seen.height,
            maximised: false,
            least_height: None,
        };
        self.standing = match self.standing {
            _ if seen.maximised => Standing::Maximised,
            Standing::Restored { rect: last, since } if last == rect => {
                Standing::Restored { rect, since }
            }
            _ => Standing::Restored { rect, since: at },
        };
    }

    /// Maximised, the rect to go back to; otherwise the window as it stands,
    /// settled or not. Either way, with the least height last held to.
    pub fn place_to_write(&self) -> WindowPlace {
        let place = match self.standing {
            Standing::Maximised => WindowPlace {
                maximised: true,
                ..self.to_go_back_to
            },
            Standing::Restored { rect, .. } => rect,
        };
        WindowPlace {
            least_height: self.least_height,
            ..place
        }
    }
}

impl Monitor {
    fn holds_the_centre_of(&self, place: WindowPlace) -> bool {
        let within = |start: i32, span: u32, corner: i32, length: u32| {
            let centre = i64::from(corner) + i64::from(length) / 2;
            (i64::from(start)..i64::from(start) + i64::from(span)).contains(&centre)
        };
        within(self.x, self.width, place.x, place.width)
            && within(self.y, self.height, place.y, place.height)
    }
}

impl WindowPlace {
    fn state_path(at: &Locations) -> PathBuf {
        at.config.join("window.json")
    }

    /// The place written on the last close; `None` when there is none, or
    /// when it cannot be read back, which costs one launch at the default.
    pub fn load(files: &impl Files, at: &Locations) -> Option<Self> {
        serde_json::from_slice(&files.read(&Self::state_path(at)).ok()?).ok()
    }

    pub fn save(&self, files: &impl Files, at: &Locations) -> Result<(), StorageError> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(files.write(&Self::state_path(at), json.as_bytes())?)
    }
}

#[cfg(test)]
mod tests;
