//! What prefs · window_place.rs is held to.
//!
//! Closes #780.
//! - Closed at a size and a position, not maximised, the application reopens
//!   at that size and that position —
//!   `a_window_closed_where_it_stood_reopens_there_at_that_size`, the place
//!   kept from one launch to the next by
//!   `a_place_written_on_close_reads_back_as_it_was`, and the one written
//!   after the window moved while the application ran by
//!   `a_window_dragged_and_closed_at_once_writes_the_rect_it_shows` and
//!   `a_minimised_or_full_screen_window_changes_nothing_to_write`
//! - Closed maximised, it reopens maximised; un-maximised then, it returns to
//!   the size and position it had before being maximised —
//!   `a_window_closed_maximised_reopens_maximised_over_the_rect_it_had_before`,
//!   the flag kept by `a_place_written_on_close_reads_back_as_it_was`; while
//!   the application runs, `a_zoom_animated_from_a_settled_rect_goes_back_to_that_rect`,
//!   `a_maximise_with_no_animation_goes_back_to_the_rect_before`,
//!   `a_window_restored_by_the_system_after_a_reopen_writes_the_rect_it_now_shows`
//!   and `maximised_again_after_a_drag_the_window_goes_back_to_where_it_was_dragged`;
//!   the loss accepted on the way,
//!   `a_window_resized_and_maximised_within_the_second_goes_back_to_the_rect_before_the_resize`
//! - At the first launch, with nothing remembered, it opens maximised;
//!   un-maximised then, it takes a default size, centred —
//!   `nothing_remembered_opens_maximised_over_the_default_size_centred_on_the_main_screen`,
//!   `the_default_size_is_counted_in_points_on_a_screen_of_scale_two`,
//!   `a_main_screen_smaller_than_the_default_size_still_opens_maximised_and_centred`,
//!   `a_place_that_cannot_be_read_back_reads_as_nothing_remembered`, and
//!   `a_window_opened_maximised_and_never_restored_writes_the_default_rect_maximised`
//!   for the rect it keeps to go back to
//! - A setting « Rouvrir à la dernière taille », on by default; off, the
//!   application opens maximised whatever was remembered — the off half here,
//!   `with_the_setting_off_a_remembered_place_opens_as_if_nothing_were_remembered`;
//!   on by default in `settings/tests.rs`, the box in
//!   `crates/app/tests/reopening_the_window.rs`
//! - A remembered position that fits no screen opens maximised on the main
//!   screen — `a_place_on_a_screen_since_unplugged_opens_maximised_on_the_main_screen`,
//!   against `a_place_on_the_second_screen_reopens_there_while_it_is_plugged_in`
//!
//! Closes #998.
//! - A window size saved smaller than that opens at the minimum — decided as
//!   the last minimum the application asked kept with the window's place, the
//!   window opening held to it, start menu included: a place saved shorter
//!   opens at it, `a_place_saved_shorter_than_its_least_height_opens_at_the_least_height`;
//!   opened maximised the window is still held to it,
//!   `opened_maximised_the_window_is_still_held_to_the_least_height_remembered`;
//!   the last height asked is the one written,
//!   `the_last_least_height_asked_while_the_application_ran_is_written_with_the_place`,
//!   and one never asked again is written as it was remembered,
//!   `a_least_height_not_asked_again_is_written_as_it_was_remembered`; it
//!   reads back, `a_place_written_on_close_reads_back_as_it_was`, and a place
//!   written before it was kept still does,
//!   `a_place_written_before_the_least_height_was_kept_reads_back_with_none`
//! - found in review: a least height taller than the screen opens the window
//!   as tall as the screen and on it, the least height itself capped there,
//!   since the window is held to it —
//!   `a_least_height_taller_than_the_screen_opens_the_window_as_tall_as_the_screen_and_on_it`;
//!   one the screen holds raises the window up rather than off its bottom —
//!   `a_window_raised_to_its_least_height_moves_up_rather_than_run_off_its_screen_s_bottom`
//!
//! Positions and sizes are in desktop units, the ones the system lays its
//! screens out in — points on macOS, pixels elsewhere: the place is the
//! window's outer top-left corner and its inner size. The default size is
//! 1280 × 800 points, times the main screen's scale, which is how many desktop
//! units a point takes there: 1 on macOS, 2 on a Windows screen at 200 %. The
//! main screen is the first one reported, and a place fits a screen when its
//! centre lies on it.
//! With no screen reported at all, nothing is decided against the screens —
//! `no_screen_reported_at_all_still_gives_a_place_to_open_at`.
//!
//! While the application runs, the window is followed frame by frame, each
//! sighting with its time, and what is written on close is where it stands:
//! not maximised, the rect it shows, settled or not; maximised, the rect to go
//! back to, which is the last one that stood unchanged for `SETTLE`, a second.
//! A window opened maximised goes back to the rect it opened over. macOS
//! animates a zoom and draws frames at rects in between that it does not call
//! maximised; they follow each other within milliseconds and never settle. The
//! price is a window resized and maximised within the second, which goes back
//! to the rect from before the resize. A minimised or full-screen window
//! changes nothing.
//!
//! What these cannot reach is the window itself. Reading it each frame is the
//! shell's, and un-maximising is the system's: macOS gives a window back the
//! frame it had when it was maximised, so the shell sets the rect first and
//! maximises after. Both happen on the real window, which no test of this
//! workspace has, and are tried in the app.

use std::path::Path;
use std::time::Duration;

use super::*;
use crate::adapters::InMemoryFiles;
use crate::locations::Locations;
use crate::ports::Files;

const REOPENING_AT_THE_LAST_SIZE: bool = true;
const ALWAYS_MAXIMISED: bool = false;

const FULL_HD_AT_SCALE_ONE: Monitor = Monitor {
    x: 0,
    y: 0,
    width: 1920,
    height: 1080,
    scale: 1.0,
};

const RETINA_AT_SCALE_TWO: Monitor = Monitor {
    x: 0,
    y: 0,
    width: 2880,
    height: 1800,
    scale: 2.0,
};

const A_SECOND_SCREEN_TO_THE_LEFT: Monitor = Monitor {
    x: -2560,
    y: 0,
    width: 2560,
    height: 1440,
    scale: 1.0,
};

const THE_DEFAULT_CENTRED_ON_FULL_HD_AND_MAXIMISED: WindowPlace = WindowPlace {
    x: 320,
    y: 140,
    width: 1280,
    height: 800,
    maximised: true,
    least_height: None,
};

const LEFT_ON_THE_SECOND_SCREEN: WindowPlace = WindowPlace {
    x: -2000,
    y: 300,
    width: 1200,
    height: 800,
    maximised: false,
    least_height: None,
};

fn at() -> Locations {
    Locations {
        config: "/config".into(),
        data: "/data".into(),
        documents: None,
    }
}

#[test]
fn a_window_closed_where_it_stood_reopens_there_at_that_size() {
    let closed = WindowPlace {
        x: 200,
        y: 120,
        width: 1000,
        height: 700,
        maximised: false,
        least_height: None,
    };

    let opened = where_the_window_opens(
        Some(closed),
        REOPENING_AT_THE_LAST_SIZE,
        &[FULL_HD_AT_SCALE_ONE],
    );

    assert_eq!(opened, closed);
}

#[test]
fn a_window_closed_maximised_reopens_maximised_over_the_rect_it_had_before() {
    let closed = WindowPlace {
        x: 400,
        y: 200,
        width: 1600,
        height: 1000,
        maximised: true,
        least_height: None,
    };

    let opened = where_the_window_opens(
        Some(closed),
        REOPENING_AT_THE_LAST_SIZE,
        &[RETINA_AT_SCALE_TWO],
    );

    assert_eq!(
        opened, closed,
        "maximised again, over the rect to go back to once restored rather than the \
         screen's",
    );
}

#[test]
fn nothing_remembered_opens_maximised_over_the_default_size_centred_on_the_main_screen() {
    let opened = where_the_window_opens(
        None,
        REOPENING_AT_THE_LAST_SIZE,
        &[FULL_HD_AT_SCALE_ONE, A_SECOND_SCREEN_TO_THE_LEFT],
    );

    assert_eq!(
        opened, THE_DEFAULT_CENTRED_ON_FULL_HD_AND_MAXIMISED,
        "1280 × 800 centred on the first screen reported, not on the second nor \
         across both",
    );
}

#[test]
fn the_default_size_is_counted_in_points_on_a_screen_of_scale_two() {
    let opened = where_the_window_opens(None, REOPENING_AT_THE_LAST_SIZE, &[RETINA_AT_SCALE_TWO]);

    assert_eq!(
        opened,
        WindowPlace {
            x: 160,
            y: 100,
            width: 2560,
            height: 1600,
            maximised: true,
            least_height: None,
        },
        "1280 × 800 points are 2560 × 1600 pixels at scale 2",
    );
}

#[test]
fn a_main_screen_smaller_than_the_default_size_still_opens_maximised_and_centred() {
    let small = Monitor {
        x: 0,
        y: 0,
        width: 1024,
        height: 768,
        scale: 1.0,
    };

    let opened = where_the_window_opens(None, REOPENING_AT_THE_LAST_SIZE, &[small]);

    assert!(opened.maximised, "{opened:?}");
    let centre = (
        opened.x + opened.width as i32 / 2,
        opened.y + opened.height as i32 / 2,
    );
    assert!(
        (centre.0 - 512).abs() <= 1 && (centre.1 - 384).abs() <= 1,
        "the rect to go back to is centred on the 1024 × 768 screen, whatever its size: \
         {opened:?} is centred on {centre:?}",
    );
}

#[test]
fn with_the_setting_off_a_remembered_place_opens_as_if_nothing_were_remembered() {
    let closed = WindowPlace {
        x: 200,
        y: 120,
        width: 1000,
        height: 700,
        maximised: false,
        least_height: None,
    };

    let opened = where_the_window_opens(Some(closed), ALWAYS_MAXIMISED, &[FULL_HD_AT_SCALE_ONE]);

    assert_eq!(opened, THE_DEFAULT_CENTRED_ON_FULL_HD_AND_MAXIMISED);
}

#[test]
fn a_place_on_the_second_screen_reopens_there_while_it_is_plugged_in() {
    let opened = where_the_window_opens(
        Some(LEFT_ON_THE_SECOND_SCREEN),
        REOPENING_AT_THE_LAST_SIZE,
        &[FULL_HD_AT_SCALE_ONE, A_SECOND_SCREEN_TO_THE_LEFT],
    );

    assert_eq!(opened, LEFT_ON_THE_SECOND_SCREEN);
}

#[test]
fn a_place_on_a_screen_since_unplugged_opens_maximised_on_the_main_screen() {
    let opened = where_the_window_opens(
        Some(LEFT_ON_THE_SECOND_SCREEN),
        REOPENING_AT_THE_LAST_SIZE,
        &[FULL_HD_AT_SCALE_ONE],
    );

    assert_eq!(
        opened, THE_DEFAULT_CENTRED_ON_FULL_HD_AND_MAXIMISED,
        "the screen the window was left on is gone: the remembered size is not moved \
         onto the main screen, the window opens there maximised",
    );
}

#[test]
fn no_screen_reported_at_all_still_gives_a_place_to_open_at() {
    let closed = WindowPlace {
        x: 200,
        y: 120,
        width: 1000,
        height: 700,
        maximised: false,
        least_height: None,
    };

    assert_eq!(
        where_the_window_opens(Some(closed), REOPENING_AT_THE_LAST_SIZE, &[]),
        closed,
        "with no screen to hold it against, the remembered place is kept and the system \
         places the window",
    );
    let first = where_the_window_opens(None, REOPENING_AT_THE_LAST_SIZE, &[]);
    assert_eq!(
        (first.width, first.height, first.maximised),
        (1280, 800, true),
        "with nothing remembered and no screen, the default size at scale 1, maximised",
    );
}

#[test]
fn a_place_written_on_close_reads_back_as_it_was() {
    let files = InMemoryFiles::default();
    let closed = WindowPlace {
        maximised: true,
        least_height: Some(THE_LEAST_HEIGHT_ASKED),
        ..LEFT_ON_THE_SECOND_SCREEN
    };
    assert_eq!(
        WindowPlace::load(&files, &at()),
        None,
        "before anything is written, nothing is remembered",
    );

    closed.save(&files, &at()).expect("the place is written");

    assert!(
        files.exists(Path::new("/config/window.json")),
        "the place is kept beside the preferences, in a file of its own",
    );
    assert_eq!(WindowPlace::load(&files, &at()), Some(closed));
}

#[test]
fn a_place_that_cannot_be_read_back_reads_as_nothing_remembered() {
    let files = InMemoryFiles::default();
    LEFT_ON_THE_SECOND_SCREEN
        .save(&files, &at())
        .expect("the place is written");
    assert_eq!(
        WindowPlace::load(&files, &at()),
        Some(LEFT_ON_THE_SECOND_SCREEN)
    );

    files
        .write(Path::new("/config/window.json"), b"\xff{ half a window")
        .expect("writes");

    assert_eq!(
        WindowPlace::load(&files, &at()),
        None,
        "a damaged place costs one launch at the default size, which is less than \
         refusing to open over it",
    );
}

const A_MACBOOK_SCREEN_IN_POINTS: Monitor = Monitor {
    x: 0,
    y: 0,
    width: 1512,
    height: 982,
    scale: 1.0,
};

const OPENED_OVER_THE_DEFAULT_RECT: WindowPlace = WindowPlace {
    x: 116,
    y: 91,
    width: 1280,
    height: 800,
    maximised: false,
    least_height: None,
};

const MAXIMISED_ON_THE_MACBOOK: WindowSeen = WindowSeen {
    x: 0,
    y: 33,
    width: 1512,
    height: 917,
    maximised: true,
    minimised: false,
    fullscreen: false,
};

const WHERE_THE_USER_LEFT_IT: WindowSeen = seen_at(200, 120, 1000, 668);

const THE_ZOOM_FROM_WHERE_THE_USER_LEFT_IT: [WindowSeen; 4] = [
    seen_at(120, 85, 1200, 760),
    seen_at(40, 50, 1420, 870),
    seen_at(6, 35, 1496, 910),
    MAXIMISED_ON_THE_MACBOOK,
];

const A_FRAME: Duration = Duration::from_millis(16);

const fn seen_at(x: i32, y: i32, width: u32, height: u32) -> WindowSeen {
    WindowSeen {
        x,
        y,
        width,
        height,
        maximised: false,
        minimised: false,
        fullscreen: false,
    }
}

fn after(seconds: f64) -> Duration {
    Duration::from_secs_f64(seconds)
}

/// Shows `window` these frames from `from`, a frame apart, as an animation
/// draws them.
fn drawn_one_frame_apart(window: &mut FollowedWindow, frames: &[WindowSeen], from: Duration) {
    let mut at = from;
    for seen in frames {
        window.saw(*seen, at);
        at += A_FRAME;
    }
}

fn written_at(seen: WindowSeen, maximised: bool) -> WindowPlace {
    WindowPlace {
        x: seen.x,
        y: seen.y,
        width: seen.width,
        height: seen.height,
        maximised,
        least_height: None,
    }
}

#[test]
fn a_zoom_animated_from_a_settled_rect_goes_back_to_that_rect() {
    let mut window = FollowedWindow::opened_at(OPENED_OVER_THE_DEFAULT_RECT);
    window.saw(seen_at(116, 91, 1280, 800), after(0.0));
    window.saw(WHERE_THE_USER_LEFT_IT, after(8.0));
    window.saw(WHERE_THE_USER_LEFT_IT, after(12.0));

    drawn_one_frame_apart(
        &mut window,
        &THE_ZOOM_FROM_WHERE_THE_USER_LEFT_IT,
        after(14.0),
    );

    assert_eq!(
        window.place_to_write(),
        written_at(WHERE_THE_USER_LEFT_IT, true),
        "maximised by a zoom, the window goes back to the rect it stood at, not to the \
         last frame of the animation, {{6, 35, 1496, 910}}, which is what the Mac wrote",
    );
}

#[test]
fn a_maximise_with_no_animation_goes_back_to_the_rect_before() {
    let mut window = FollowedWindow::opened_at(OPENED_OVER_THE_DEFAULT_RECT);
    window.saw(seen_at(116, 91, 1280, 800), after(0.0));
    window.saw(WHERE_THE_USER_LEFT_IT, after(8.0));

    window.saw(MAXIMISED_ON_THE_MACBOOK, after(12.0));

    assert_eq!(
        window.place_to_write(),
        written_at(WHERE_THE_USER_LEFT_IT, true),
        "with motion reduced, the window is maximised from one frame to the next, and \
         the rect it stood at for four seconds is the one it goes back to",
    );
}

#[test]
fn a_window_resized_and_maximised_within_the_second_goes_back_to_the_rect_before_the_resize() {
    let mut window = FollowedWindow::opened_at(OPENED_OVER_THE_DEFAULT_RECT);
    window.saw(seen_at(116, 91, 1280, 800), after(0.0));
    window.saw(WHERE_THE_USER_LEFT_IT, after(8.0));

    window.saw(MAXIMISED_ON_THE_MACBOOK, after(8.0) + SETTLE / 2);

    assert_eq!(
        window.place_to_write(),
        WindowPlace {
            maximised: true,
            ..OPENED_OVER_THE_DEFAULT_RECT
        },
        "the loss accepted: a rect that stood less than a second is not told apart from \
         a frame of the zoom, so the window goes back to the one before it",
    );
}

#[test]
fn a_window_restored_by_the_system_after_a_reopen_writes_the_rect_it_now_shows() {
    let mut window = FollowedWindow::opened_at(written_at(WHERE_THE_USER_LEFT_IT, true));
    window.saw(MAXIMISED_ON_THE_MACBOOK, after(0.0));

    drawn_one_frame_apart(
        &mut window,
        &[
            seen_at(40, 50, 1420, 870),
            seen_at(120, 85, 1200, 760),
            WHERE_THE_USER_LEFT_IT,
        ],
        after(5.0),
    );

    assert_eq!(
        window.place_to_write(),
        written_at(WHERE_THE_USER_LEFT_IT, false),
        "restored, the window is written as it stands, not maximised, though the rect \
         it shows has not stood a second yet",
    );
}

#[test]
fn a_minimised_or_full_screen_window_changes_nothing_to_write() {
    let minimised = WindowSeen {
        maximised: true,
        minimised: true,
        ..seen_at(-32000, -32000, 160, 28)
    };
    let full_screen = WindowSeen {
        fullscreen: true,
        ..seen_at(0, 0, 1512, 982)
    };

    let mut left_where_it_was =
        FollowedWindow::opened_at(written_at(WHERE_THE_USER_LEFT_IT, false));
    left_where_it_was.saw(WHERE_THE_USER_LEFT_IT, after(0.0));
    left_where_it_was.saw(minimised, after(5.0));
    left_where_it_was.saw(full_screen, after(9.0));
    assert_eq!(
        left_where_it_was.place_to_write(),
        written_at(WHERE_THE_USER_LEFT_IT, false),
        "a minimised window shows a rect nobody would reopen at, and full screen is \
         neither a place nor a maximised window",
    );

    let maximised = written_at(WHERE_THE_USER_LEFT_IT, true);
    let mut left_maximised = FollowedWindow::opened_at(maximised);
    left_maximised.saw(MAXIMISED_ON_THE_MACBOOK, after(0.0));
    left_maximised.saw(minimised, after(5.0));
    left_maximised.saw(full_screen, after(9.0));
    assert_eq!(left_maximised.place_to_write(), maximised);
}

#[test]
fn a_window_opened_maximised_and_never_restored_writes_the_default_rect_maximised() {
    let opened = where_the_window_opens(
        None,
        REOPENING_AT_THE_LAST_SIZE,
        &[A_MACBOOK_SCREEN_IN_POINTS],
    );
    let mut window = FollowedWindow::opened_at(opened);

    for second in [0.0, 1.0, 30.0] {
        window.saw(MAXIMISED_ON_THE_MACBOOK, after(second));
    }

    assert_eq!(
        window.place_to_write(),
        WindowPlace {
            maximised: true,
            ..OPENED_OVER_THE_DEFAULT_RECT
        },
        "the 1280 × 800 centred it opened over is still the rect to go back to",
    );
}

#[test]
fn a_window_dragged_and_closed_at_once_writes_the_rect_it_shows() {
    let mut window = FollowedWindow::opened_at(OPENED_OVER_THE_DEFAULT_RECT);
    window.saw(seen_at(116, 91, 1280, 800), after(0.0));

    drawn_one_frame_apart(
        &mut window,
        &[
            seen_at(140, 98, 1280, 800),
            seen_at(170, 110, 1280, 800),
            seen_at(200, 120, 1280, 800),
        ],
        after(5.0),
    );

    assert_eq!(
        window.place_to_write(),
        WindowPlace {
            x: 200,
            y: 120,
            width: 1280,
            height: 800,
            maximised: false,
            least_height: None,
        },
        "closed straight after a drag, the window reopens where it was let go",
    );
}

#[test]
fn maximised_again_after_a_drag_the_window_goes_back_to_where_it_was_dragged() {
    let opened = where_the_window_opens(
        None,
        REOPENING_AT_THE_LAST_SIZE,
        &[A_MACBOOK_SCREEN_IN_POINTS],
    );
    let mut window = FollowedWindow::opened_at(opened);
    window.saw(MAXIMISED_ON_THE_MACBOOK, after(0.0));

    drawn_one_frame_apart(
        &mut window,
        &[
            seen_at(60, 60, 1400, 860),
            seen_at(100, 82, 1320, 820),
            seen_at(116, 91, 1280, 800),
        ],
        after(10.0),
    );
    drawn_one_frame_apart(
        &mut window,
        &[
            seen_at(150, 100, 1180, 760),
            seen_at(180, 112, 1080, 700),
            WHERE_THE_USER_LEFT_IT,
        ],
        after(20.0),
    );
    drawn_one_frame_apart(
        &mut window,
        &THE_ZOOM_FROM_WHERE_THE_USER_LEFT_IT,
        after(30.0),
    );

    assert_eq!(
        window.place_to_write(),
        written_at(WHERE_THE_USER_LEFT_IT, true),
        "opened maximised, restored, dragged, then maximised again: closed now, the \
         window reopens maximised and goes back to where it was dragged",
    );
}

/// The least height the application asked of its window on the Part screen,
/// one tab of the ribbon open.
const THE_LEAST_HEIGHT_ASKED: u32 = 543;

/// The same on the sketch's tab, whose ribbon holds a row more.
const THE_LEAST_HEIGHT_ASKED_ON_THE_SKETCH_TAB: u32 = 621;

#[test]
fn a_place_saved_shorter_than_its_least_height_opens_at_the_least_height() {
    let short = WindowPlace {
        x: 200,
        y: 120,
        width: 1000,
        height: 400,
        maximised: false,
        least_height: Some(THE_LEAST_HEIGHT_ASKED),
    };
    let tall = WindowPlace {
        height: 700,
        ..short
    };

    let opened_short = where_the_window_opens(
        Some(short),
        REOPENING_AT_THE_LAST_SIZE,
        &[FULL_HD_AT_SCALE_ONE],
    );
    let opened_tall = where_the_window_opens(
        Some(tall),
        REOPENING_AT_THE_LAST_SIZE,
        &[FULL_HD_AT_SCALE_ONE],
    );

    assert_eq!(
        opened_short,
        WindowPlace {
            height: THE_LEAST_HEIGHT_ASKED,
            ..short
        },
        "a place saved 400 high under a least height of {THE_LEAST_HEIGHT_ASKED} opens at it",
    );
    assert_eq!(
        opened_tall, tall,
        "a place saved taller than its least height opens as it was saved",
    );
}

#[test]
fn opened_maximised_the_window_is_still_held_to_the_least_height_remembered() {
    let closed = WindowPlace {
        x: 200,
        y: 120,
        width: 1000,
        height: 700,
        maximised: false,
        least_height: Some(THE_LEAST_HEIGHT_ASKED),
    };

    let opened = where_the_window_opens(Some(closed), ALWAYS_MAXIMISED, &[FULL_HD_AT_SCALE_ONE]);

    assert_eq!(
        opened,
        WindowPlace {
            least_height: Some(THE_LEAST_HEIGHT_ASKED),
            ..THE_DEFAULT_CENTRED_ON_FULL_HD_AND_MAXIMISED
        },
        "the setting off decides where the window opens, not how short it may be made",
    );
}

#[test]
fn the_last_least_height_asked_while_the_application_ran_is_written_with_the_place() {
    let mut window = FollowedWindow::opened_at(OPENED_OVER_THE_DEFAULT_RECT);
    window.saw(WHERE_THE_USER_LEFT_IT, after(0.0));

    window.asked_no_shorter_than(THE_LEAST_HEIGHT_ASKED_ON_THE_SKETCH_TAB);
    window.asked_no_shorter_than(THE_LEAST_HEIGHT_ASKED);
    window.saw(WHERE_THE_USER_LEFT_IT, after(2.0));
    let restored = window.place_to_write();
    window.saw(MAXIMISED_ON_THE_MACBOOK, after(3.0));
    let maximised = window.place_to_write();

    assert_eq!(
        restored,
        WindowPlace {
            least_height: Some(THE_LEAST_HEIGHT_ASKED),
            ..written_at(WHERE_THE_USER_LEFT_IT, false)
        },
    );
    assert_eq!(
        maximised,
        WindowPlace {
            least_height: Some(THE_LEAST_HEIGHT_ASKED),
            ..written_at(WHERE_THE_USER_LEFT_IT, true)
        },
        "maximised, the place written still keeps the least height last asked",
    );
}

#[test]
fn a_least_height_not_asked_again_is_written_as_it_was_remembered() {
    let opened = WindowPlace {
        least_height: Some(THE_LEAST_HEIGHT_ASKED),
        ..OPENED_OVER_THE_DEFAULT_RECT
    };
    let mut window = FollowedWindow::opened_at(opened);

    window.saw(WHERE_THE_USER_LEFT_IT, after(0.0));

    assert_eq!(
        window.place_to_write(),
        WindowPlace {
            least_height: Some(THE_LEAST_HEIGHT_ASKED),
            ..written_at(WHERE_THE_USER_LEFT_IT, false)
        },
        "a session closed on the start menu asks nothing, and the next launch is held as \
         this one was",
    );
}

#[test]
fn a_place_written_before_the_least_height_was_kept_reads_back_with_none() {
    let files = InMemoryFiles::default();
    files
        .write(
            Path::new("/config/window.json"),
            br#"{ "x": 200, "y": 120, "width": 1000, "height": 700, "maximised": false }"#,
        )
        .expect("writes");

    assert_eq!(
        WindowPlace::load(&files, &at()),
        Some(WindowPlace {
            x: 200,
            y: 120,
            width: 1000,
            height: 700,
            maximised: false,
            least_height: None,
        }),
        "a place written before #998 opens as it did",
    );
}

/// A least height asked on a taller screen than the one the window reopens on.
const A_LEAST_HEIGHT_TALLER_THAN_FULL_HD: u32 = 1200;

#[test]
fn a_least_height_taller_than_the_screen_opens_the_window_as_tall_as_the_screen_and_on_it() {
    let closed = WindowPlace {
        x: 200,
        y: 120,
        width: 1000,
        height: 700,
        maximised: false,
        least_height: Some(A_LEAST_HEIGHT_TALLER_THAN_FULL_HD),
    };
    let screen = FULL_HD_AT_SCALE_ONE.height;

    let reopened = where_the_window_opens(
        Some(closed),
        REOPENING_AT_THE_LAST_SIZE,
        &[FULL_HD_AT_SCALE_ONE],
    );
    let maximised = where_the_window_opens(Some(closed), ALWAYS_MAXIMISED, &[FULL_HD_AT_SCALE_ONE]);

    assert_eq!(
        reopened,
        WindowPlace {
            y: 0,
            height: screen,
            least_height: Some(screen),
            ..closed
        },
        "a least height of {A_LEAST_HEIGHT_TALLER_THAN_FULL_HD} on a screen {screen} high: \
         the window is held no taller than the screen, and stands on it",
    );
    assert_eq!(
        maximised,
        WindowPlace {
            y: 0,
            height: screen,
            least_height: Some(screen),
            ..THE_DEFAULT_CENTRED_ON_FULL_HD_AND_MAXIMISED
        },
        "the rect a maximised window goes back to is held to the screen the same way",
    );
}

#[test]
fn a_window_raised_to_its_least_height_moves_up_rather_than_run_off_its_screen_s_bottom() {
    let closed = WindowPlace {
        x: 200,
        y: 400,
        width: 1000,
        height: 500,
        maximised: false,
        least_height: Some(900),
    };

    let opened = where_the_window_opens(
        Some(closed),
        REOPENING_AT_THE_LAST_SIZE,
        &[FULL_HD_AT_SCALE_ONE],
    );

    assert_eq!(
        opened,
        WindowPlace {
            y: 180,
            height: 900,
            ..closed
        },
        "raised from 500 to 900 at 400 from the top, the window would end at 1300 on a screen \
         1080 high; it moves up to end on the screen's bottom",
    );
}
