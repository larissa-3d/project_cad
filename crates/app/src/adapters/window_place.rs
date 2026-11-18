//! The application's window, put where it was last closed and followed until
//! it closes again, for the next launch to read.
//!
//! What winit reports is in physical pixels, each value counted at the scale
//! of the screen or the window it was read from. The desktop lays its screens
//! out in its own units — points on macOS, pixels elsewhere — and that is what
//! `cao_prefs` decides in, so every value is converted on the way in and set
//! back in those units on the way out.

use std::sync::{Arc, Mutex};
use std::time::Instant;

use cao_prefs::{
    FollowedWindow, Locations, Monitor, WindowPlace, WindowSeen, where_the_window_opens,
};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize};
use winit::window::Window;

use crate::adapters::files::DiskFiles;

/// The units a desktop lays its screens out in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Desktop {
    /// macOS: one space in points, whatever the scale of each screen.
    Points,
    /// Windows and X11: one space in pixels, as reported.
    Pixels,
}

const DESKTOP: Desktop = if cfg!(target_os = "macos") {
    Desktop::Points
} else {
    Desktop::Pixels
};

/// A rect as winit reports it, with the scale of the screen or the window it
/// was read from.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PhysicalRect {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) scale_factor: f64,
}

/// A rect in the desktop's own units.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DesktopRect {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Desktop {
    /// A screen in the desktop's units, with how many of them a point takes.
    pub(crate) fn monitor(self, screen: PhysicalRect) -> Monitor {
        let DesktopRect {
            x,
            y,
            width,
            height,
        } = self.in_units(screen);
        let scale = match self {
            Desktop::Points => 1.0,
            Desktop::Pixels => screen.scale_factor,
        };
        Monitor {
            x,
            y,
            width,
            height,
            scale,
        }
    }

    /// The window's outer corner and inner size in the desktop's units.
    pub(crate) fn window(self, window: PhysicalRect) -> DesktopRect {
        self.in_units(window)
    }

    /// A length egui counts in points, in the desktop's units on a window of
    /// this scale, rounded up so that the window holds all of it.
    pub(crate) fn length(self, points: f32, scale_factor: f64) -> u32 {
        let per_point = match self {
            Desktop::Points => 1.0,
            Desktop::Pixels => scale_factor,
        };
        (f64::from(points) * per_point).ceil() as u32
    }

    fn in_units(self, physical: PhysicalRect) -> DesktopRect {
        let per_unit = match self {
            Desktop::Points => physical.scale_factor,
            Desktop::Pixels => 1.0,
        };
        let signed = |value: i32| (f64::from(value) / per_unit).round() as i32;
        let unsigned = |value: u32| (f64::from(value) / per_unit).round() as u32;
        DesktopRect {
            x: signed(physical.x),
            y: signed(physical.y),
            width: unsigned(physical.width),
            height: unsigned(physical.height),
        }
    }
}

/// The window followed from the place it opened at to the one it closes at.
/// Holds nothing under the test harness, which has no window. Where the system
/// does not say where a window stands, or offers nowhere to keep its place,
/// the window opens maximised and nothing is followed or written.
pub struct WindowKeeper {
    followed: Option<Followed>,
}

struct Followed {
    window: Arc<Window>,
    at: Locations,
    seen: FollowedWindow,
    started: Instant,
    asked: LeastHeightAsked,
}

/// The least inner height the application last asked of its window, in
/// points, read off each frame's output and taken once followed.
#[derive(Clone, Default)]
struct LeastHeightAsked(Arc<Mutex<Option<f32>>>);

impl egui::Plugin for LeastHeightAsked {
    fn debug_name(&self) -> &'static str {
        "least_height_asked"
    }

    fn output_hook(&mut self, _ctx: &egui::Context, output: &mut egui::FullOutput) {
        let asked = output
            .viewport_output
            .get(&egui::ViewportId::ROOT)
            .into_iter()
            .flat_map(|root| &root.commands)
            .rev()
            .find_map(|command| match command {
                egui::ViewportCommand::MinInnerSize(size) => Some(size.y),
                _ => None,
            });
        if let Some(asked) = asked
            && let Ok(mut held) = self.0.lock()
        {
            *held = Some(asked);
        }
    }
}

impl LeastHeightAsked {
    fn take(&self) -> Option<f32> {
        self.0.lock().ok()?.take()
    }
}

impl WindowKeeper {
    /// Puts the window, still hidden, where it opens: the remembered place, or
    /// maximised over the default size. The rect is set before the window is
    /// maximised, for un-maximising to go back to it.
    pub fn open(
        cc: &eframe::CreationContext<'_>,
        at: Option<&Locations>,
        reopen_at_last_size: bool,
    ) -> Self {
        let Some(window) = cc.winit_window() else {
            return Self { followed: None };
        };
        let kept_at = at.filter(|_| window.outer_position().is_ok());
        let remembered = kept_at.and_then(|at| WindowPlace::load(&DiskFiles, at));
        let place = where_the_window_opens(remembered, reopen_at_last_size, &monitors(window));
        put(window, place);
        let followed = kept_at.map(|at| {
            let asked = LeastHeightAsked::default();
            cc.egui_ctx.add_plugin(asked.clone());
            Followed {
                window: Arc::clone(window),
                at: at.clone(),
                seen: FollowedWindow::opened_at(place),
                started: Instant::now(),
                asked,
            }
        });
        Self { followed }
    }

    /// Reads where the window stands now, and the least height last asked of
    /// it, once a frame.
    pub fn follow(&mut self) {
        let Some(followed) = &mut self.followed else {
            return;
        };
        if let Some(points) = followed.asked.take() {
            let scale = followed.window.scale_factor();
            followed
                .seen
                .asked_no_shorter_than(DESKTOP.length(points, scale));
        }
        if let Some(seen) = seen(&followed.window) {
            followed.seen.saw(seen, followed.started.elapsed());
        }
    }

    /// Reads the window a last time, as a move may not have been followed by a
    /// frame, and writes its place down. A place that cannot be written is
    /// lost rather than an error: the application closes all the same.
    pub fn put_down(&mut self) {
        self.follow();
        if let Some(followed) = &self.followed {
            let _ = followed
                .seen
                .place_to_write()
                .save(&DiskFiles, &followed.at);
        }
    }
}

/// The screens, the main one first.
fn monitors(window: &Window) -> Vec<Monitor> {
    let primary = window.primary_monitor();
    let mut screens: Vec<_> = window.available_monitors().collect();
    screens.sort_by_key(|screen| Some(screen) != primary.as_ref());
    screens
        .into_iter()
        .map(|screen| {
            let (position, size) = (screen.position(), screen.size());
            DESKTOP.monitor(PhysicalRect {
                x: position.x,
                y: position.y,
                width: size.width,
                height: size.height,
                scale_factor: screen.scale_factor(),
            })
        })
        .collect()
}

fn seen(window: &Window) -> Option<WindowSeen> {
    let (position, size) = (window.outer_position().ok()?, window.inner_size());
    let rect = DESKTOP.window(PhysicalRect {
        x: position.x,
        y: position.y,
        width: size.width,
        height: size.height,
        scale_factor: window.scale_factor(),
    });
    Some(WindowSeen {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
        maximised: window.is_maximized(),
        minimised: window.is_minimized().unwrap_or(false),
        fullscreen: window.fullscreen().is_some(),
    })
}

fn put(window: &Window, place: WindowPlace) {
    match DESKTOP {
        Desktop::Points => {
            let least = place.least_height.map(|height| LogicalSize::new(0, height));
            window.set_min_inner_size(least);
            window.set_outer_position(LogicalPosition::new(place.x, place.y));
            let _ = window.request_inner_size(LogicalSize::new(place.width, place.height));
        }
        Desktop::Pixels => {
            let least = place
                .least_height
                .map(|height| PhysicalSize::new(0, height));
            window.set_min_inner_size(least);
            window.set_outer_position(PhysicalPosition::new(place.x, place.y));
            let _ = window.request_inner_size(PhysicalSize::new(place.width, place.height));
        }
    }
    window.set_maximized(place.maximised);
}

#[cfg(test)]
mod tests;
