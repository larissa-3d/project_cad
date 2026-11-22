//! What prefs · theme.rs is held to.
//!
//! Closes #286.
//! - the alert colour is a colour of the theme, and a theme saved before it
//!   existed still reads —
//!   `a_theme_saved_before_the_alert_colour_existed_still_reads`
//! - it is edited in the settings like the others — no test: the row is one
//!   call in `settings/appearance.rs`, which draws and decides nothing
//!
//! Closes #724.
//! - the view's background: plain `vue_esquisse` — `background.sketch_view`
//!   since #749 put the tokens in English — in a sketch, a radial gradient in
//!   the free view, as the default of the existing background setting —
//!   `a_sketch_is_drawn_on_the_plain_sketch_view_colour_by_default`,
//!   `the_free_view_is_drawn_on_the_redesign_s_radial_gradient_by_default`;
//!   which of the two a frame draws is held in
//!   `crates/app/src/screens/viewport/render/tests.rs`, under its own block
//! - the « Par défaut » profile takes the redesign's viewport colours as
//!   defaults — `the_default_grid_and_axes_are_the_redesign_s`, and every
//!   colour the redesign does not name keeps the value it had —
//!   `every_colour_the_redesign_does_not_name_keeps_its_default`
//! - a profile Tom saved keeps its own — no test: in this file; held by
//!   `a_profile_saved_with_viewport_colours_of_its_own_reads_back_with_them`,
//!   in `crates/prefs/src/profiles/tests.rs`, under its own block
//!
//! Closes #736 — the whole transcription is in `crates/app/tests/the_alerts.rs`;
//! this bullet is the part the theme holds.
//! - Preferences › Apparence can set and reset them — the warning's and the
//!   error's colours are colours of the theme, and a theme saved before they
//!   existed still reads, with neither set —
//!   `a_theme_saved_before_the_warning_and_error_colours_existed_still_reads`
//!
//! Neither is set by default: the colour a level is seen in by default is the
//! chrome's, a token of `cao_app`, which this crate does not know. What the
//! theme holds is the colour a user chose over it.
//!
//! Closes #742 — the whole transcription is in
//! `crates/app/src/screens/viewport/render/drawing/tests.rs`; these bullets are
//! the colours the « Par défaut » profile carries.
//! - The « Par défaut » profile takes these colours: settled off-white
//!   (`trait_regle`, `sketch.settled_stroke` since #749), free blue
//!   (`trait_libre`, `sketch.free_stroke`), areas tinted a light blue, strokes
//!   2 px — decided by Tom once he had tried the branch: free strokes keep the
//!   orange-yellow they had before the redesign, the redesign's blue set
//!   aside; the rest stands —
//!   `the_default_profile_draws_settled_strokes_and_areas_as_the_redesign_and_free_ones_orange`;
//!   construction grey and the tool in progress amber, two colours the theme
//!   did not have — `the_default_profile_carries_a_construction_grey_and_a_tool_amber`
//! - decided in the issue: against the sketch background, settled 15.1:1, free
//!   8.6:1, construction 6.2:1, tool 9.1:1; settled against free 1.75:1 — read
//!   since Tom kept the orange: settled 15.1:1, construction 6.2:1 and tool
//!   9.1:1 as the issue decided, free 13.4:1, what the orange gives; settled
//!   against free falls to 1.13:1, the hue rather than the brightness telling
//!   the two apart, and is no longer held —
//!   `each_stroke_stands_off_the_sketch_background_by_the_contrast_decided`
//! - Grid: in the redesign's colours; axes X red, Y green, Z blue — true since
//!   #724, and held by `the_default_grid_and_axes_are_the_redesign_s`
//! - decided by Tom for the whole redesign: a profile saved before this change
//!   is overwritten or ignored, never migrated — read here as: a theme saved
//!   before the two new colours existed still reads, each at its default, as
//!   every colour added to the theme before them does —
//!   `a_theme_saved_before_the_construction_and_tool_colours_existed_still_reads`
//! - The « Par défaut » profile takes these colours, the dimensions' too: the
//!   redesign's `sketch.dimension`, which the value is written in as well as
//!   the lines — `the_default_profile_draws_dimensions_in_the_redesign_s_colour`;
//!   the boxes of the constraint marks filled and bordered in its
//!   `constraint_mark_background` and `constraint_mark_border`, two colours the
//!   theme did not have — `the_default_profile_boxes_the_rule_marks_in_the_redesign_s_colours`,
//!   and a theme saved before them still reads, each at its default —
//!   `a_theme_saved_before_the_rule_marks_box_colours_existed_still_reads`
//!
//! The new colours are read by the names a profile writes them under,
//! `sketch_construction`, `tool_in_progress`, `rule_background` and
//! `rule_border`: a profile is a file people hand to one another, and those
//! names are what it carries.

use super::*;

fn gradient() -> Background {
    Background::Linear {
        angle_degrees: 0.0,
        stops: vec![
            Stop::new(0.0, Rgba::opaque(0.0, 0.0, 0.0)),
            Stop::new(0.5, Rgba::opaque(1.0, 0.0, 0.0)),
            Stop::new(1.0, Rgba::opaque(1.0, 1.0, 1.0)),
        ],
    }
}

#[test]
fn a_gradient_reads_between_its_stops() {
    let background = gradient();
    assert_eq!(background.sample(0.0), Rgba::opaque(0.0, 0.0, 0.0));
    assert_eq!(background.sample(0.5), Rgba::opaque(1.0, 0.0, 0.0));
    assert_eq!(background.sample(1.0), Rgba::opaque(1.0, 1.0, 1.0));

    let quarter = background.sample(0.25);
    assert!((quarter.r - 0.5).abs() < 1e-5, "{quarter:?}");
    assert!(quarter.g.abs() < 1e-5);
}

/// Past either end, a gradient holds its last colour rather than fading to
/// nothing.
#[test]
fn a_gradient_holds_its_ends() {
    let background = gradient();
    assert_eq!(background.sample(-3.0), Rgba::opaque(0.0, 0.0, 0.0));
    assert_eq!(background.sample(9.0), Rgba::opaque(1.0, 1.0, 1.0));
}

#[test]
fn a_gradient_with_no_stops_still_gives_a_colour() {
    let empty = Background::Linear {
        angle_degrees: 0.0,
        stops: Vec::new(),
    };
    assert_eq!(empty.sample(0.5), Rgba::opaque(0.0, 0.0, 0.0));
}

#[test]
fn a_solid_background_is_the_same_everywhere() {
    let background = Background::Solid(Rgba::opaque(0.2, 0.3, 0.4));
    assert_eq!(background.sample(0.0), background.sample(1.0));
}

/// The alert colour landed after people already had themes on disk, so a file
/// that predates it has to read as a theme with the default.
#[test]
fn a_theme_saved_before_the_alert_colour_existed_still_reads() {
    let mut saved = serde_json::to_value(Theme::default()).expect("a theme writes out");
    saved
        .as_object_mut()
        .expect("a theme is an object")
        .remove("going")
        .expect("the alert colour is written out with the rest");

    let read: Theme = serde_json::from_value(saved).expect("a theme from before the colour");

    assert_eq!(read.going, Theme::default().going);
}

#[test]
fn a_theme_saved_before_the_warning_and_error_colours_existed_still_reads() {
    let mut saved = serde_json::to_value(Theme::default()).expect("a theme writes out");
    let fields = saved.as_object_mut().expect("a theme is an object");
    fields.remove("warning");
    fields.remove("error");

    let read: Theme = serde_json::from_value(saved).expect("a theme from before the colours");

    let default = Theme::default();
    assert_eq!(
        (read.warning, read.error, default.warning, default.error),
        (None, None, None, None),
        "a theme from before the warning's and the error's colours reads, or the default \
         theme holds, a colour chosen over the chrome's",
    );
}

/// A colour read back to the bytes it would be written with: the design file
/// writes its colours in hex, so a byte is as near as either side can mean.
fn bytes(color: Rgba) -> [u8; 4] {
    color
        .to_array()
        .map(|channel| (channel * 255.0).round() as u8)
}

/// The design file the chrome's tests read too, taken in when the tests are
/// built rather than opened from the disk: a token delivered anew fails here
/// as it fails there.
const DESIGN: &str = include_str!("../../../../docs/redesign/tokens.json");

/// A colour of `color.`, written `#RRGGBB`, as the bytes of an opaque colour.
fn written(path: &str) -> [u8; 4] {
    let tokens: serde_json::Value = serde_json::from_str(DESIGN).expect("the design file is JSON");
    let hex = path
        .split('.')
        .try_fold(&tokens["color"], |value, key| value.get(key))
        .and_then(serde_json::Value::as_str)
        .and_then(|text| text.strip_prefix('#'))
        .unwrap_or_else(|| panic!("color.{path} is no #RRGGBB in the design file"));
    let channel = |at: usize| {
        u8::from_str_radix(&hex[at..at + 2], 16)
            .unwrap_or_else(|_| panic!("color.{path} is no #RRGGBB in the design file"))
    };
    [channel(0), channel(2), channel(4), 0xFF]
}

/// The same, at the opacity the design file gives the axes.
fn written_at_the_axes_opacity(path: &str) -> [u8; 4] {
    let tokens: serde_json::Value = serde_json::from_str(DESIGN).expect("the design file is JSON");
    let opacity = tokens["color"]["axes"]["opacity"]
        .as_f64()
        .expect("color.axes.opacity is a number");
    let [r, g, b, _] = written(path);
    [r, g, b, (opacity * 255.0).round() as u8]
}

#[test]
fn a_sketch_is_drawn_on_the_plain_sketch_view_colour_by_default() {
    let background = Theme::default().sketch_background;

    assert!(
        matches!(background, Background::Solid(colour) if bytes(colour) == written("background.sketch_view")),
        "a sketch's default background is {background:?}, not background.sketch_view, \
         a plain {:?}",
        written("background.sketch_view"),
    );
}

#[test]
fn the_free_view_is_drawn_on_the_redesign_s_radial_gradient_by_default() {
    let background = Theme::default().background;
    let Background::Radial {
        center,
        radius,
        stops,
    } = background
    else {
        panic!("the free view's default background is {background:?}, not a radial gradient");
    };

    assert!(
        (center[0] - 0.5).abs() < 1e-6 && (center[1] - 0.55).abs() < 1e-6,
        "the gradient opens from {center:?}: the mockup's cy=45% counts from the top, \
         and a centre here counts from the bottom, so slightly above the middle is 0.55",
    );
    assert!((radius - 0.7).abs() < 1e-6, "the gradient reaches {radius}");
    let stops: Vec<(f32, [u8; 4])> = stops
        .iter()
        .map(|stop| (stop.at, bytes(stop.color)))
        .collect();
    assert_eq!(
        stops,
        [
            (0.0, written("background.view3d_centre")),
            (1.0, written("background.view3d_edge"))
        ],
        "background.view3d_centre to background.view3d_edge",
    );
}

#[test]
fn the_default_grid_and_axes_are_the_redesign_s() {
    let theme = Theme::default();

    let wrong: Vec<String> = [
        ("grid_minor", theme.grid_minor, written("sketch.grid_minor")),
        ("grid_major", theme.grid_major, written("sketch.grid_major")),
        (
            "axis_x",
            theme.axis_x,
            written_at_the_axes_opacity("axes.x"),
        ),
        (
            "axis_y",
            theme.axis_y,
            written_at_the_axes_opacity("axes.y"),
        ),
        (
            "axis_z",
            theme.axis_z,
            written_at_the_axes_opacity("axes.z"),
        ),
    ]
    .into_iter()
    .filter(|(_, worn, design)| bytes(*worn) != *design)
    .map(|(name, worn, design)| {
        format!(
            "{name} is {:?}, the redesign writes {design:?}",
            bytes(worn)
        )
    })
    .collect();

    assert!(
        wrong.is_empty(),
        "the default profile draws its grid and axes in other colours:\n  {}",
        wrong.join("\n  "),
    );
}

/// What the redesign leaves alone is pinned as it stood before it: the sketch
/// strokes and areas and the dimensions' colour moved with #742 and are held
/// below; the extrusion's colours are #740's. A read-only dimension's colour
/// stays: the redesign names none, and the one it had stands apart from the
/// redesign's dimension colour, which
/// `a_read_only_dimension_still_reads_apart_from_a_driving_one` holds on a
/// frame. An edit to any of them should show here rather than slip through
/// with another issue.
#[test]
fn every_colour_the_redesign_does_not_name_keeps_its_default() {
    let theme = Theme::default();

    let moved: Vec<String> = [
        (
            "sketch_inactive",
            theme.sketch_inactive,
            Rgba::new(0.70, 0.72, 0.76, 0.8),
        ),
        (
            "dimension_driven",
            theme.dimension_driven,
            Rgba::new(0.62, 0.65, 0.70, 0.8),
        ),
        ("fixed", theme.fixed, Rgba::new(0.95, 0.45, 0.25, 1.0)),
        ("rule", theme.rule, Rgba::new(0.55, 0.75, 0.95, 1.0)),
        ("picked", theme.picked, Rgba::new(0.85, 0.40, 0.95, 1.0)),
        ("solid", theme.solid, Rgba::new(0.78, 0.80, 0.84, 1.0)),
        (
            "highlight",
            theme.highlight,
            Rgba::new(0.30, 0.60, 0.95, 0.40),
        ),
        ("refused", theme.refused, Rgba::new(0.95, 0.35, 0.35, 0.40)),
        ("going", theme.going, Rgba::new(0.95, 0.25, 0.25, 1.0)),
        ("measure", theme.measure, Rgba::new(0.97, 0.97, 1.0, 1.0)),
        (
            "extrusion_add",
            theme.extrusion_add,
            Rgba::new(0.40, 0.85, 0.60, 0.45),
        ),
        (
            "extrusion_cut",
            theme.extrusion_cut,
            Rgba::new(0.95, 0.45, 0.40, 0.45),
        ),
    ]
    .into_iter()
    .filter(|(_, worn, before)| worn != before)
    .map(|(name, worn, before)| format!("{name} is {worn:?}, it was {before:?}"))
    .collect();

    assert!(
        moved.is_empty(),
        "colours the redesign does not name have moved:\n  {}",
        moved.join("\n  "),
    );
    assert_eq!(
        (
            theme.axis_width,
            theme.grid_minor_width,
            theme.grid_major_width,
            theme.grid_major_every,
        ),
        (2.0, 1.0, 1.5, 10),
        "the widths and the spacing of the major grid lines have moved too",
    );
}

/// The names a profile writes the construction's colour and the tool's under.
const CONSTRUCTION: &str = "sketch_construction";
const TOOL_IN_PROGRESS: &str = "tool_in_progress";

/// The theme of the profile that always exists, « Par défaut » on screen.
fn the_default_profile_s_theme() -> Theme {
    crate::profiles::Profiles::default().active().theme.clone()
}

/// The colour a theme writes out under `name`, as a profile carries it, or
/// nothing when it writes no such colour.
fn written_out(theme: &Theme, name: &str) -> Option<Rgba> {
    let written = serde_json::to_value(theme).expect("a theme writes out");
    written.get(name).map(|colour| {
        serde_json::from_value(colour.clone())
            .unwrap_or_else(|_| panic!("the theme writes `{name}` as no colour: {colour}"))
    })
}

/// A colour of `color.` written `rgba(r,g,b,a)`, as the bytes of that colour
/// at that opacity.
fn written_with_its_opacity(path: &str) -> [u8; 4] {
    let tokens: serde_json::Value = serde_json::from_str(DESIGN).expect("the design file is JSON");
    let channels: Vec<f64> = path
        .split('.')
        .try_fold(&tokens["color"], |value, key| value.get(key))
        .and_then(serde_json::Value::as_str)
        .and_then(|text| text.strip_prefix("rgba("))
        .and_then(|text| text.strip_suffix(')'))
        .map(|inside| {
            inside
                .split(',')
                .filter_map(|channel| channel.trim().parse().ok())
                .collect()
        })
        .unwrap_or_default();
    let [r, g, b, a] = channels[..] else {
        panic!("color.{path} is no rgba(r,g,b,a) in the design file");
    };
    [r as u8, g as u8, b as u8, (a * 255.0).round() as u8]
}

/// The orange-yellow free strokes had before the redesign, which Tom kept.
const THE_OLD_ORANGE: Rgba = Rgba::opaque(0.98, 0.85, 0.35);

#[test]
fn the_default_profile_draws_settled_strokes_and_areas_as_the_redesign_and_free_ones_orange() {
    let theme = the_default_profile_s_theme();

    let wrong: Vec<String> = [
        (
            "sketch_settled",
            theme.sketch_settled,
            written("sketch.settled_stroke"),
        ),
        ("sketch_free", theme.sketch_free, bytes(THE_OLD_ORANGE)),
        (
            "region_fill",
            theme.region_fill,
            written_with_its_opacity("sketch.area_fill"),
        ),
    ]
    .into_iter()
    .filter(|(_, worn, design)| bytes(*worn) != *design)
    .map(|(name, worn, design)| {
        format!(
            "{name} is {:?}, where it is decided {design:?}",
            bytes(worn)
        )
    })
    .collect();

    assert!(
        wrong.is_empty(),
        "the default profile draws a sketch in other colours:\n  {}",
        wrong.join("\n  "),
    );
    assert_eq!(
        theme.sketch_width, 2.0,
        "the redesign draws a sketch's strokes 2 px wide",
    );
}

#[test]
fn the_default_profile_carries_a_construction_grey_and_a_tool_amber() {
    let theme = the_default_profile_s_theme();

    assert_eq!(
        [CONSTRUCTION, TOOL_IN_PROGRESS].map(|name| written_out(&theme, name).map(bytes)),
        [
            Some(written("sketch.construction")),
            Some(written("sketch.tool_in_progress")),
        ],
        "the default profile writes the construction's colour under `{CONSTRUCTION}` and \
         the tool in progress's under `{TOOL_IN_PROGRESS}`, the redesign's grey and amber",
    );
}

/// The contrast ratio of WCAG 2 between two opaque colours.
fn contrast(one: Rgba, other: Rgba) -> f32 {
    let linear = |channel: f32| match channel <= 0.04045 {
        true => channel / 12.92,
        false => ((channel + 0.055) / 1.055).powf(2.4),
    };
    let luminance = |colour: Rgba| {
        0.2126 * linear(colour.r) + 0.7152 * linear(colour.g) + 0.0722 * linear(colour.b)
    };
    let (one, other) = (luminance(one), luminance(other));
    (one.max(other) + 0.05) / (one.min(other) + 0.05)
}

/// A figure written to a tenth holds any ratio that rounds to it.
const TO_A_TENTH: f32 = 0.05;

#[test]
fn each_stroke_stands_off_the_sketch_background_by_the_contrast_decided() {
    let theme = the_default_profile_s_theme();
    let Background::Solid(background) = theme.sketch_background else {
        panic!(
            "a sketch's default background is {:?}, no plain colour to measure against",
            theme.sketch_background
        );
    };

    let wrong: Vec<String> = [
        (
            "settled against the background",
            Some(theme.sketch_settled),
            background,
            15.1,
            TO_A_TENTH,
        ),
        (
            "free against the background",
            Some(theme.sketch_free),
            background,
            13.4,
            TO_A_TENTH,
        ),
        (
            "construction against the background",
            written_out(&theme, CONSTRUCTION),
            background,
            6.2,
            TO_A_TENTH,
        ),
        (
            "the tool in progress against the background",
            written_out(&theme, TOOL_IN_PROGRESS),
            background,
            9.1,
            TO_A_TENTH,
        ),
    ]
    .into_iter()
    .filter_map(|(what, stroke, against, decided, near)| match stroke {
        None => Some(format!("{what}: the default profile has no such colour")),
        Some(stroke) => {
            let ratio = contrast(stroke, against);
            ((ratio - decided).abs() > near)
                .then(|| format!("{what} is {ratio:.3}:1, where {decided}:1 is decided"))
        }
    })
    .collect();

    assert!(
        wrong.is_empty(),
        "the default profile's strokes stand off the sketch background by other contrasts \
         (settled against free reads {:.2}:1, no longer held since Tom kept the orange):\n  {}",
        contrast(theme.sketch_settled, theme.sketch_free),
        wrong.join("\n  "),
    );
}

#[test]
fn a_theme_saved_before_the_construction_and_tool_colours_existed_still_reads() {
    let defaults =
        [CONSTRUCTION, TOOL_IN_PROGRESS].map(|name| written_out(&Theme::default(), name));
    assert!(
        defaults.iter().all(Option::is_some),
        "the default theme writes no `{CONSTRUCTION}` or no `{TOOL_IN_PROGRESS}` colour: \
         {defaults:?}",
    );
    let mut saved = serde_json::to_value(Theme::default()).expect("a theme writes out");
    let fields = saved.as_object_mut().expect("a theme is an object");
    fields.remove(CONSTRUCTION);
    fields.remove(TOOL_IN_PROGRESS);

    let read: Theme = serde_json::from_value(saved)
        .expect("a theme from before the construction's and the tool's colours reads");

    assert_eq!(
        [CONSTRUCTION, TOOL_IN_PROGRESS].map(|name| written_out(&read, name)),
        defaults,
        "a theme from before the two colours reads with others than the defaults",
    );
}

#[test]
fn the_default_profile_draws_dimensions_in_the_redesign_s_colour() {
    let theme = the_default_profile_s_theme();

    assert_eq!(
        bytes(theme.dimension),
        written("sketch.dimension"),
        "the default profile draws its dimensions in another colour than the redesign's",
    );
}

/// The names a profile writes the colours of a rule mark's box under.
const RULE_BACKGROUND: &str = "rule_background";
const RULE_BORDER: &str = "rule_border";

#[test]
fn the_default_profile_boxes_the_rule_marks_in_the_redesign_s_colours() {
    let theme = the_default_profile_s_theme();

    assert_eq!(
        [RULE_BACKGROUND, RULE_BORDER].map(|name| written_out(&theme, name).map(bytes)),
        [
            Some(written("sketch.constraint_mark_background")),
            Some(written("sketch.constraint_mark_border")),
        ],
        "the default profile writes the fill of a rule mark's box under `{RULE_BACKGROUND}` \
         and its border under `{RULE_BORDER}`, the redesign's",
    );
}

#[test]
fn a_theme_saved_before_the_rule_marks_box_colours_existed_still_reads() {
    let defaults = [RULE_BACKGROUND, RULE_BORDER].map(|name| written_out(&Theme::default(), name));
    assert!(
        defaults.iter().all(Option::is_some),
        "the default theme writes no `{RULE_BACKGROUND}` or no `{RULE_BORDER}` colour: \
         {defaults:?}",
    );
    let mut saved = serde_json::to_value(Theme::default()).expect("a theme writes out");
    let fields = saved.as_object_mut().expect("a theme is an object");
    fields.remove(RULE_BACKGROUND);
    fields.remove(RULE_BORDER);

    let read: Theme =
        serde_json::from_value(saved).expect("a theme from before the box colours reads");

    assert_eq!(
        [RULE_BACKGROUND, RULE_BORDER].map(|name| written_out(&read, name)),
        defaults,
        "a theme from before the box colours reads with others than the defaults",
    );
}
