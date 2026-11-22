//! What app · screens/viewport/render/drawing.rs is held to.
//!
//! Closes #286.
//! - the dimensions that will go with the stretch are drawn in the alert
//!   colour — `a_value_the_cut_would_take_is_drawn_in_the_alert_colour`
//! - their written value follows — no test: it is the same list read one file
//!   away, in `dimensions.rs`, and egui paints it as text on a window this
//!   test does not open
//!
//! The tint's share of #621, whose criteria live with the part's tests in
//! `crates/part/tests/a_fillet_rounds_a_corner_with_a_curve.rs`: a plate
//! rounded beside an arc is tinted over its rounded area —
//! `a_plate_rounded_beside_an_arc_is_tinted_along_its_rounded_outline`.
//!
//! Closes #321.
//! - the preview updates as soon as the panel holds enough to lay something,
//!   wherever the pointer is — over the panel, the canvas has no cursor —
//!   `a_copy_s_preview_shows_with_the_pointer_off_the_drawing`
//!
//! Closes #742. The issue's "What it does" and "What must not break", word for
//! word, each with the test that holds it; every frame here is drawn with the
//! « Par défaut » profile unless a bullet says otherwise.
//! - **Strokes**: settled off-white (`trait_regle`), free blue (`trait_libre`),
//!   construction grey and dashed, 2 px; vertices as 8 px squares; areas
//!   tinted a light blue — a settled trait, off-white and 2 px wide —
//!   `a_settled_stroke_is_drawn_off_white_two_pixels_wide`; a free one 2 px
//!   wide and, decided by Tom once he had tried the branch, in the
//!   orange-yellow it had before the redesign rather than the redesign's blue —
//!   `a_free_stroke_is_drawn_orange_two_pixels_wide`; a
//!   construction one, grey and in dashes —
//!   `a_construction_stroke_is_drawn_grey_and_dashed`; a vertex, a square 8 px
//!   across, as it already was — `a_vertex_is_marked_by_a_square_eight_pixels_across`;
//!   an area, the light blue at the redesign's opacity —
//!   `a_closed_area_is_tinted_the_redesign_s_light_blue`
//! - **The tool in progress is amber**, and only it: the provisional trait —
//!   `the_provisional_trait_of_the_tool_in_progress_is_drawn_amber`; "and only
//!   it" is the strokes above, each held to a colour of its own and none amber
//! - a dashed snap guide — no test: set aside with the human when the work
//!   opened, to an issue of its own, with the « Tangence » snap
//! - a snap ring with its name (« Coïncidence », « Milieu », « Tangence »…) —
//!   read as: each snap the sketch offers, a point included, is ringed in the
//!   tool's amber, a ring of constant size on screen —
//!   `each_snap_is_ringed_in_the_tool_s_amber`; and named beside its ring in a
//!   pill bordered amber, a point « Coïncidence », a middle « Milieu », a
//!   crossing « Intersection », a centre « Centre », a place on a curve « Sur
//!   la courbe » — held in `crates/app/tests/the_tool_in_progress.rs`, under
//!   its own block, by `each_snap_is_named_beside_its_ring`. « Tangence » has
//!   no snap to name: the sketch crate offers none, and it went with the snap
//!   guide
//! - an angle arc with its value on the line (today only on the arc) — read
//!   as: while a line is drawn, an amber arc at its start, of constant size on
//!   screen, from the trait it is drawn on from — the one the chain laid just
//!   before, or else the only trait ending where it starts — or else from the
//!   horizontal, round to the line, the short way —
//!   `a_line_in_progress_shows_its_angle_as_an_amber_arc`; the angle that arc
//!   spans written beside it in a pill bordered amber, « 90° » when it is
//!   square — held in `crates/app/tests/the_tool_in_progress.rs` by
//!   `a_line_in_progress_writes_its_angle_on_its_arc`
//! - the length field by the cursor with an amber border — no test: in this
//!   file; egui paints the field, held in
//!   `crates/app/tests/the_tool_in_progress.rs` by
//!   `the_length_field_by_the_cursor_has_an_amber_border`
//! - **Dimensions**: arrows — an arrowhead of two strokes at each end of the
//!   dimension line, opening back along it, as it already was —
//!   `a_dimension_is_drawn_with_an_arrow_at_each_end`; the value in a
//!   cartouche, its text in the dimension colour, which a profile sets — held
//!   in `crates/app/src/screens/viewport/render/dimensions/tests.rs`, under
//!   its own block, by `a_dimension_s_value_sits_in_a_cartouche` and
//!   `a_dimension_s_value_is_written_in_the_dimension_colour`: egui paints the
//!   value, and no frame of the scene carries it
//! - **Constraint marks** in small 18 px boxes (⊥, ∥…) — no test: in this
//!   file; egui paints the marks, held in
//!   `crates/app/src/screens/viewport/render/overlays/tests.rs`, under its own
//!   block, by `a_rule_s_mark_sits_in_a_box_18_px_across`,
//!   `the_rule_marks_box_colours_a_profile_sets_are_the_ones_painted` and
//!   `a_rule_s_mark_answers_anywhere_on_its_box_and_no_two_boxes_overlap`
//! - **Grid**: today's step in millimetres that follows the zoom, in the
//!   redesign's colours; axes X red, Y green, Z blue — no test: in this file;
//!   true since #724 and left as it is, the colours held by
//!   `the_default_grid_and_axes_are_the_redesign_s` in
//!   `crates/prefs/src/theme/tests.rs`, the step by
//!   `adaptive_step_grows_with_distance` in `crates/render/src/grid/tests.rs`
//!   and `the_scale_bar_says_how_long_one_grid_step_is` in
//!   `crates/app/src/screens/viewport/render/overlays/tests.rs`
//! - The « Par défaut » profile takes these colours — no test: in this file;
//!   held by `the_default_profile_draws_settled_strokes_and_areas_as_the_redesign_and_free_ones_orange`
//!   and `the_default_profile_carries_a_construction_grey_and_a_tool_amber`,
//!   with the contrasts the issue decided by
//!   `each_stroke_stands_off_the_sketch_background_by_the_contrast_decided`,
//!   the dimensions' colour by
//!   `the_default_profile_draws_dimensions_in_the_redesign_s_colour`, the
//!   marks' boxes by
//!   `the_default_profile_boxes_the_rule_marks_in_the_redesign_s_colours`, all
//!   in `crates/prefs/src/theme/tests.rs`, under its own block
//! - Preferences › Apparence still sets each one: what a profile sets is what
//!   is drawn — `the_construction_and_tool_colours_a_profile_sets_are_the_ones_drawn`;
//!   the page's rows for the new colours are held by
//!   `the_appearance_page_offers_a_row_for_the_construction_and_the_tool_in_progress`
//!   and `the_appearance_page_offers_a_row_for_the_box_of_the_rule_marks` in
//!   `crates/app/tests/appearance.rs`, under its own block
//! - must not break: the length and angle fields by the cursor, and typing in
//!   them — no test: in this file; held as they were by
//!   `a_number_typed_while_drawing_goes_into_the_field` and
//!   `a_name_typed_straight_into_the_field_lays_the_line_to_it` in
//!   `crates/app/tests/variables.rs`, and
//!   `a_line_typed_at_thirty_and_clicked_behind_runs_and_reads_thirty` in
//!   `crates/app/src/screens/viewport/input/angle_arm/tests.rs`
//! - must not break: every snap today offers, and which one wins — no test: in
//!   this file; the rule is `Sketch::magnetise`, which this issue does not
//!   touch, held by `crates/sketch/src/snap/tests.rs` —
//!   `a_crossing_holds_the_cursor_against_a_midpoint_that_is_nearer`,
//!   `an_existing_point_wins_over_the_grid`,
//!   `the_middle_of_a_trait_pulls_harder_than_its_body` among them
//! - must not break: a read-only dimension still looks read-only — no test: in
//!   this file; egui paints the value, held in
//!   `crates/app/src/screens/viewport/render/dimensions/tests.rs`, under its
//!   own block, by `a_read_only_dimension_still_reads_apart_from_a_driving_one`
//! - decided by Tom for the whole redesign: old profiles and settings are
//!   overwritten or ignored, with no migration — no test: in this file; read as
//!   a theme saved before the new colours still reading, held by
//!   `a_theme_saved_before_the_construction_and_tool_colours_existed_still_reads`
//!   and `a_theme_saved_before_the_rule_marks_box_colours_existed_still_reads`
//!   in `crates/prefs/src/theme/tests.rs`; found in review, a « Par défaut »
//!   profile already on disk kept the yellow and the green, so a settings file
//!   of the version before opens on the defaults — held by
//!   `a_default_profile_saved_with_the_old_sketch_colours_takes_the_new_ones_at_load`
//!   in `crates/prefs/src/profiles/tests.rs`
//! - found in review: a mark's box stood over the trait it marks, and a click
//!   along the trait took the rule — no test: in this file; held by
//!   `a_rule_s_mark_stands_clear_of_the_traits_so_a_click_on_a_trait_is_the_trait_s`
//!   in `crates/app/src/screens/viewport/render/overlays/tests.rs`, the whole
//!   box still answering by
//!   `a_rule_s_mark_answers_anywhere_on_its_box_and_no_two_boxes_overlap`
//! - found in review: a snap's ring outlived the cursor — none is drawn once
//!   the cursor has left the canvas, whichever snap it was —
//!   `no_ring_is_drawn_once_the_cursor_has_left_the_canvas`; its name, and the
//!   name standing clear of the fields the cursor carries, held in
//!   `crates/app/tests/the_tool_in_progress.rs` by
//!   `no_snap_is_named_once_the_pointer_has_left_the_canvas` and
//!   `a_snap_s_name_stands_clear_of_the_fields_the_cursor_carries`
//! - found in review: a fillet's and a chamfer's values stayed in amber, which
//!   is the tool in progress's alone — written as a dimension's value is, in
//!   the dimension colour and in its cartouche — no test: in this file; held in
//!   `crates/app/src/screens/viewport/render/dimensions/tests.rs` by
//!   `a_cut_s_value_is_written_like_a_dimension_s_in_its_colour_and_its_cartouche`
//! - found in the real window: on a rectangle drawn with its tool, the size
//!   the capture shows, a mark's box covered a side and hid a corner — no
//!   test: in this file; held by
//!   `the_marks_of_a_rectangle_drawn_with_its_tool_stand_clear_of_its_sides_and_corners`
//!   in `crates/app/src/screens/viewport/render/overlays/tests.rs`
//! - found in the real window: the angle on the arc was written to four
//!   places, and stood under the fields the cursor carries — no test: in this
//!   file; held by `the_angle_on_the_arc_is_written_to_a_tenth_of_a_degree`
//!   and `the_angle_s_pill_stands_clear_of_the_fields_the_cursor_carries` in
//!   `crates/app/tests/the_tool_in_progress.rs`
//! - found in the real window: the fillet tool, picking a corner, named its
//!   snap — only a tool that lays a point rings its snap, one that picks what
//!   is drawn rings nothing — `only_a_tool_that_places_a_point_rings_its_snap`;
//!   and names nothing, held by `a_tool_that_picks_what_is_drawn_names_no_snap`
//!   in `crates/app/tests/the_tool_in_progress.rs`
//! - found in the real window: the trait in progress was thin and faint beside
//!   the laid ones and the mockup's solid amber — a line, a rectangle and a
//!   circle in progress are drawn at the sketch's width and near opaque, in
//!   the tool's colour — `the_provisional_trait_is_drawn_solid_at_the_sketch_width`
//! - decided by Tom once he had tried the branch: a right angle's mark stands
//!   as near its corner as clearing its sides allows — no test: in this file;
//!   held by `a_right_angle_s_mark_stands_as_near_its_corner_as_clearing_allows`
//!   in `crates/app/src/screens/viewport/render/overlays/tests.rs`
//!
//! The grid's spacing is out of scope: it stays in millimetres. The two colours
//! the theme gains are read by the names a profile writes them under,
//! `sketch_construction` and `tool_in_progress`, in the one test where a
//! profile sets them.

use cao_part::PartDocument;
use cao_part::history::{Operation, PointRef};
use cao_prefs::config::ViewportConfig;
use cao_prefs::theme::Rgba;
use cao_render::camera::OrbitCamera;
use cao_sketch::{
    AxisToward, ChainAnchor, DimensionTarget, SegmentId, SketchAxis, Snap, Stretch, ToolState,
    Toward, WorkPlane,
};
use chrono::Utc;
use glam::Vec3;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, Tool};
use crate::screens::viewport::input::copying::{copy, enter_in_the_panel};

/// A vertex holds its position in `f32`, so a place read back off one has lost
/// the tail of its `f64`. Well under the width of anything drawn here.
const TOLERANCE: f64 = 1e-5;

const CORNER: DVec2 = DVec2::ZERO;
const OPPOSITE: DVec2 = DVec2::new(10.0, 20.0);

fn a_part_with_a_rectangle() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(CORNER),
        opposite: PointRef::New(OPPOSITE),
        construction: false,
    });
    document
}

fn a_view() -> ViewScale {
    ViewScale::of(
        &OrbitCamera::default(),
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 800.0)),
        1.0,
        &ViewportConfig::default(),
        1.0,
    )
}

/// What one frame paints of the drawing, as lines and as surfaces.
fn painted(active: bool) -> (Vec<cao_render::Vertex>, Vec<cao_render::Vertex>) {
    let mut document = a_part_with_a_rectangle();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let sketch = document.sketches()[0].clone();

    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let shown = Shown {
        sketch: &sketch,
        laid: &[],
        cuts: &[],
        going: None,
        active,
    };
    let (mut lines, mut surfaces) = (Vec::new(), Vec::new());
    push_sketch(
        &mut lines,
        &mut surfaces,
        &shown,
        &Theme::default(),
        a_view(),
        &context,
    );
    (lines, surfaces)
}

/// A sketch drawn whole, as one with its eye lit and nothing lit from the
/// tree is.
fn push_sketch(
    out: &mut Vec<cao_render::Vertex>,
    surfaces: &mut Vec<cao_render::Vertex>,
    shown: &Shown<'_>,
    theme: &Theme,
    scale: ViewScale,
    context: &SketchContext<'_>,
) {
    Seen::default().draw(out, surfaces, shown, theme, scale, context);
}

/// The areas of a sketch tinted, as they are with its eye lit and nothing lit
/// from the tree.
fn push_regions(
    surfaces: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    active: bool,
) {
    tint_regions(surfaces, sketch, theme, active, Seen::default());
}

fn place(vertex: &cao_render::Vertex, sketch: &Sketch) -> DVec2 {
    sketch
        .plane
        .to_local(Vec3::from(vertex.position).as_dvec3())
}

#[test]
fn every_corner_of_what_was_drawn_is_painted_where_it_was_put() {
    let sketch = a_part_with_a_rectangle().sketches()[0].clone();
    let (lines, _) = painted(true);

    for corner in [
        CORNER,
        DVec2::new(OPPOSITE.x, CORNER.y),
        OPPOSITE,
        DVec2::new(CORNER.x, OPPOSITE.y),
    ] {
        assert!(
            lines
                .iter()
                .any(|vertex| place(vertex, &sketch).distance(corner) < TOLERANCE),
            "nothing is painted at {corner:?}, which is a corner of what was drawn",
        );
    }
}

#[test]
fn a_closed_contour_is_tinted_so_it_reads_as_a_face() {
    let (_, surfaces) = painted(true);

    assert!(
        !surfaces.is_empty(),
        "a closed contour is drawn as four separate lines and nothing else",
    );
    assert_eq!(
        surfaces.len() % 3,
        0,
        "an area is filled with something other than whole triangles",
    );
}

#[test]
fn a_drawing_set_aside_keeps_its_contour_and_loses_its_points() {
    let sketch = a_part_with_a_rectangle().sketches()[0].clone();
    let (active, active_fill) = painted(true);
    let (idle, idle_fill) = painted(false);

    assert_ne!(
        active[0].color, idle[0].color,
        "the drawing being worked on looks exactly like one set aside",
    );
    assert_ne!(
        active_fill[0].color, idle_fill[0].color,
        "the area of a drawing set aside is tinted like the active one",
    );
    assert!(
        active.len() > idle.len(),
        "a drawing set aside is painted everything the active one is",
    );
    for corner in [CORNER, OPPOSITE] {
        assert!(
            idle.iter()
                .any(|vertex| place(vertex, &sketch).distance(corner) < TOLERANCE),
            "a drawing set aside lost the corner at {corner:?} from its contour",
        );
    }
}

#[test]
fn a_shape_drawn_inside_another_takes_more_of_the_tint_than_the_one_around_it() {
    let mut document = a_part_with_a_rectangle();
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(2.0, 2.0)),
        opposite: PointRef::New(DVec2::new(8.0, 18.0)),
        construction: false,
    });
    let sketch = document.sketches()[0].clone();

    let mut surfaces = Vec::new();
    push_regions(&mut surfaces, &sketch, &Theme::default(), true);

    let depths: Vec<usize> = sketch.regions().iter().map(|region| region.depth).collect();
    assert!(
        depths.iter().any(|depth| *depth > 0),
        "a rectangle inside another gives no nested area at all: {depths:?}",
    );
    let shades: Vec<f32> = surfaces.iter().map(|vertex| vertex.color[3]).collect();
    assert!(
        shades.iter().any(|shade| *shade > shades[0]),
        "every area is tinted the same, so an outline and its pocket wash into one another",
    );
}

/// The rectangle with a length on one of its sides, and a cut that would take
/// that side away — the length said to go with it or not, and nothing else
/// told apart, so that what changes between the two is the value alone.
fn painted_with_the_length_going(said_to_go: bool) -> Vec<cao_render::Vertex> {
    let mut document = a_part_with_a_rectangle();
    let mut sketch = document.sketches()[0].clone();
    let side = SegmentId(0);
    sketch.set_dimension(DimensionTarget::Length(side), 10.0, false);
    let going = Going {
        stretch: Stretch::Straight {
            from: CORNER,
            to: OPPOSITE,
        },
        construction: false,
        rules: Vec::new(),
        values: match said_to_go {
            true => vec![DimensionTarget::Length(side)],
            false => Vec::new(),
        },
    };

    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let shown = Shown {
        sketch: &sketch,
        laid: &[],
        cuts: &[],
        going: Some(&going),
        active: true,
    };
    let (mut lines, mut surfaces) = (Vec::new(), Vec::new());
    push_sketch(
        &mut lines,
        &mut surfaces,
        &shown,
        &Theme::default(),
        a_view(),
        &context,
    );
    lines
}

#[test]
fn a_value_the_cut_would_take_is_drawn_in_the_alert_colour() {
    let alert = tint(Theme::default().going);
    let in_alert = |lines: Vec<cao_render::Vertex>| {
        lines.iter().filter(|vertex| vertex.color == alert).count()
    };

    let going = in_alert(painted_with_the_length_going(true));
    let staying = in_alert(painted_with_the_length_going(false));

    assert!(
        going > staying,
        "the length about to go is drawn exactly like one that stays — the \
         stretch is red in both, so only the value can tell them apart",
    );
}

/// A plate whose corner at (10, 10) stands between a trait running east to
/// (50, 10) and an arc round (2, 10) of radius 8 rising a quarter turn to
/// (2, 18), closed by traits up to (50, 30) and across to the arc's end — and
/// the same plate with that corner rounded by five.
fn a_plate_beside_an_arc(rounded: bool) -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(10.0, 10.0),
        on: Vec::new(),
    });
    let corner = cao_sketch::PointId(1);
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(corner),
        end: PointRef::New(DVec2::new(50.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(2.0, 10.0)),
        start: PointRef::Existing(corner),
        end: PointRef::New(DVec2::new(2.0, 18.0)),
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(cao_sketch::PointId(2)),
        end: PointRef::New(DVec2::new(50.0, 30.0)),
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(cao_sketch::PointId(5)),
        end: PointRef::Existing(cao_sketch::PointId(4)),
        construction: false,
    });
    if rounded {
        document.apply(Operation::Fillet {
            sketch: 0,
            corners: vec![cao_sketch::Corner::At(corner)],
            radius: 5.0.into(),
            construction: false,
        });
    }
    document
}

/// How much of the plane the frame tints, and the area the drawing reads.
fn tinted(mut document: PartDocument) -> (f64, f64) {
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let sketch = document.sketches()[0].clone();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let shown = Shown {
        sketch: &sketch,
        laid: &[],
        cuts: &[],
        going: None,
        active: true,
    };
    let (mut lines, mut surfaces) = (Vec::new(), Vec::new());
    push_sketch(
        &mut lines,
        &mut surfaces,
        &shown,
        &Theme::default(),
        a_view(),
        &context,
    );
    let tinted = surfaces
        .chunks(3)
        .map(|corners| {
            let [a, b, c] = [0, 1, 2].map(|rank| place(&corners[rank], &sketch));
            (b - a).perp_dot(c - a).abs() * 0.5
        })
        .sum();
    let read: f64 = sketch.regions().iter().map(|region| region.area()).sum();
    (tinted, read)
}

#[test]
fn a_plate_rounded_beside_an_arc_is_tinted_along_its_rounded_outline() {
    let (sharp, sharp_area) = tinted(a_plate_beside_an_arc(false));
    let (rounded, rounded_area) = tinted(a_plate_beside_an_arc(true));

    assert!(
        sharp_area - rounded_area > 2.0,
        "the fillet takes its sliver off the area: {sharp_area} against {rounded_area}"
    );
    assert!(
        (rounded - rounded_area).abs() < 0.2,
        "the rounded plate is tinted over its rounded area, {rounded_area}, not {rounded}"
    );
    assert!(
        (sharp - rounded) > 2.0,
        "less is tinted once the corner is rounded: {sharp} against {rounded}"
    );
}

/// The design file of the redesign, taken in when the tests are built: a token
/// delivered anew fails here as it fails in the theme's tests.
const DESIGN: &str = include_str!("../../../../../../../docs/redesign/tokens.json");

/// A colour of the design file's `color.`, written `#RRGGBB` or
/// `rgba(r,g,b,a)`, as the bytes a hex code writes and the opacity.
///
/// The path comes as its keys one by one rather than dotted: a dotted literal
/// in a source of `cao_app` reads as a key of the language file to
/// `language_keys.rs`, and these are keys of the design file.
fn redesign(keys: &[&str]) -> ([u8; 3], f32) {
    let path = keys.join(".");
    let tokens: serde_json::Value = serde_json::from_str(DESIGN).expect("the design file is JSON");
    let text = keys
        .iter()
        .try_fold(&tokens["color"], |value, key| value.get(key))
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| panic!("color.{path} is no colour in the design file"));
    if let Some(hex) = text.strip_prefix('#') {
        let channel = |at: usize| {
            u8::from_str_radix(&hex[at..at + 2], 16)
                .unwrap_or_else(|_| panic!("color.{path} is no #RRGGBB in the design file"))
        };
        return ([channel(0), channel(2), channel(4)], 1.0);
    }
    let channels: Vec<f32> = text
        .strip_prefix("rgba(")
        .and_then(|text| text.strip_suffix(')'))
        .map(|inside| {
            inside
                .split(',')
                .filter_map(|channel| channel.trim().parse().ok())
                .collect()
        })
        .unwrap_or_default();
    let [r, g, b, a] = channels[..] else {
        panic!("color.{path} is neither #RRGGBB nor rgba(r,g,b,a) in the design file");
    };
    ([r as u8, g as u8, b as u8], a)
}

/// A colour as the shader takes it, read back to the bytes a hex code writes:
/// the design writes bytes, so a byte is as near as either side can mean.
fn hex_of(shaded: [f32; 4]) -> [u8; 3] {
    let channel = |linear: f32| {
        let srgb = match linear <= 0.003_130_8 {
            true => linear * 12.92,
            false => 1.055 * linear.powf(1.0 / 2.4) - 0.055,
        };
        (srgb * 255.0).round() as u8
    };
    [channel(shaded[0]), channel(shaded[1]), channel(shaded[2])]
}

/// A part whose only sketch holds one trait, from `from` to `to`.
fn a_part_with_a_trait(from: DVec2, to: DVec2, construction: bool) -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(from),
        end: PointRef::New(to),
        construction,
    });
    document
}

/// A part whose only sketch holds one trait out of the origin, 30 long and
/// 30° up from the first axis: nothing about it is left to move.
fn a_part_with_a_settled_trait() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(Sketch::ORIGIN),
        end: PointRef::New(DVec2::new(26.0, 15.0)),
        construction: false,
    });
    for (target, value) in [
        (DimensionTarget::Length(SegmentId(0)), 30.0),
        (
            DimensionTarget::AxisAngle {
                segment: SegmentId(0),
                segment_toward: Toward::End,
                axis: SketchAxis::U,
                axis_toward: AxisToward::Positive,
            },
            30.0,
        ),
    ] {
        document.apply(Operation::SetDimension {
            sketch: 0,
            target,
            value: value.into(),
            placement: None,
        });
    }
    document
}

/// What one frame paints of the first sketch of a part being edited, the
/// editor and the theme as given: the sketch it was read off, its lines and
/// its surfaces.
fn a_frame_of(
    mut document: PartDocument,
    mut editor: SketchEditor,
    theme: &Theme,
) -> (Sketch, Vec<cao_render::Vertex>, Vec<cao_render::Vertex>) {
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let sketch = document.sketches()[0].clone();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let shown = Shown {
        sketch: &sketch,
        laid: &[],
        cuts: &[],
        going: None,
        active: true,
    };
    let (mut lines, mut surfaces) = (Vec::new(), Vec::new());
    push_sketch(&mut lines, &mut surfaces, &shown, theme, a_view(), &context);
    (sketch, lines, surfaces)
}

/// One straight step of what was painted, in the coordinates of the drawing,
/// with what it was painted in.
#[derive(Clone, Copy, Debug)]
struct Step {
    from: DVec2,
    to: DVec2,
    color: [f32; 4],
    width: f32,
}

/// The steps painted along the stretch from `from` to `to`: both their ends on
/// it, between its two ends. A dashed stretch comes back as several.
fn painted_along(
    lines: &[cao_render::Vertex],
    sketch: &Sketch,
    from: DVec2,
    to: DVec2,
) -> Vec<Step> {
    let span = to - from;
    let on_it = |at: DVec2| {
        let offset = at - from;
        let reach = offset.dot(span) / span.length_squared();
        (offset - span * reach).length() < TOLERANCE
            && (-TOLERANCE..=1.0 + TOLERANCE).contains(&reach)
    };
    lines
        .chunks(2)
        .map(|ends| Step {
            from: place(&ends[0], sketch),
            to: place(&ends[1], sketch),
            color: ends[0].color,
            width: ends[0].width,
        })
        .filter(|step| {
            step.from.distance(step.to) > TOLERANCE && on_it(step.from) && on_it(step.to)
        })
        .collect()
}

/// The steps that are painted in another colour than `hex`, by what they are
/// painted in.
fn not_in(steps: &[Step], hex: [u8; 3]) -> Vec<([u8; 3], f32)> {
    steps
        .iter()
        .filter(|step| hex_of(step.color) != hex)
        .map(|step| (hex_of(step.color), step.width))
        .collect()
}

#[test]
fn a_free_stroke_is_drawn_orange_two_pixels_wide() {
    let (from, to) = (DVec2::new(4.0, 5.0), DVec2::new(40.0, 25.0));
    let document = a_part_with_a_trait(from, to, false);
    let settled = document.sketches()[0].settled_points(document.scale());
    assert!(
        !settled.iter().skip(1).any(|held| *held),
        "the fixture's trait is meant to be free, and something of it is settled: {settled:?}",
    );

    let (sketch, lines, _) = a_frame_of(document, SketchEditor::default(), &Theme::default());

    let stroke = painted_along(&lines, &sketch, from, to);
    let orange = hex_of(tint(Rgba::opaque(0.98, 0.85, 0.35)));
    assert!(
        !stroke.is_empty(),
        "nothing is painted along the free trait"
    );
    let wrong = not_in(&stroke, orange);
    assert!(
        wrong.is_empty(),
        "a free trait is painted the orange-yellow Tom kept, {orange:?}, not {wrong:?}",
    );
    assert!(
        stroke.iter().all(|step| step.width == 2.0),
        "a free trait is painted 2 px wide: {stroke:?}",
    );
}

#[test]
fn a_settled_stroke_is_drawn_off_white_two_pixels_wide() {
    let document = a_part_with_a_settled_trait();
    let sketch = &document.sketches()[0];
    let (from, to) = sketch.endpoints(SegmentId(0));
    let settled = sketch.settled_points(document.scale());
    assert!(
        settled.iter().all(|held| *held),
        "the fixture's trait is meant to be settled, and something of it can still move: \
         {settled:?}",
    );

    let (sketch, lines, _) = a_frame_of(document, SketchEditor::default(), &Theme::default());

    let stroke: Vec<Step> = painted_along(&lines, &sketch, from, to)
        .into_iter()
        .filter(|step| {
            [step.from, step.to]
                .iter()
                .any(|end| end.distance(from) < TOLERANCE)
                && [step.from, step.to]
                    .iter()
                    .any(|end| end.distance(to) < TOLERANCE)
        })
        .collect();
    let (off_white, _) = redesign(&["sketch", "settled_stroke"]);
    assert!(
        !stroke.is_empty(),
        "nothing is painted from one end of the settled trait to the other",
    );
    let wrong = not_in(&stroke, off_white);
    assert!(
        wrong.is_empty(),
        "a settled trait is painted the redesign's off-white, {off_white:?}, not {wrong:?}",
    );
    assert!(
        stroke.iter().all(|step| step.width == 2.0),
        "a settled trait is painted 2 px wide: {stroke:?}",
    );
}

#[test]
fn a_construction_stroke_is_drawn_grey_and_dashed() {
    let (from, to) = (DVec2::new(4.0, 5.0), DVec2::new(40.0, 25.0));
    let document = a_part_with_a_trait(from, to, true);

    let (sketch, lines, _) = a_frame_of(document, SketchEditor::default(), &Theme::default());

    let stroke = painted_along(&lines, &sketch, from, to);
    let (grey, _) = redesign(&["sketch", "construction"]);
    assert!(
        stroke.len() > 2,
        "a construction trait is painted in dashes, and comes back as {} step(s)",
        stroke.len(),
    );
    let wrong = not_in(&stroke, grey);
    assert!(
        wrong.is_empty(),
        "a construction trait is painted the redesign's grey, {grey:?}, not {wrong:?}",
    );
}

#[test]
fn a_closed_area_is_tinted_the_redesign_s_light_blue() {
    let (_, surfaces) = painted(true);
    let (blue, opacity) = redesign(&["sketch", "area_fill"]);

    assert!(
        !surfaces.is_empty(),
        "the rectangle's area is not tinted at all"
    );
    let tinted: Vec<([u8; 3], f32)> = surfaces
        .iter()
        .map(|vertex| (hex_of(vertex.color), vertex.color[3]))
        .collect();
    assert!(
        tinted
            .iter()
            .all(|(hex, alpha)| *hex == blue && (alpha - opacity).abs() < 1e-3),
        "an area no other one holds is tinted the redesign's {blue:?} at {opacity}, not {:?}",
        tinted[0],
    );
}

#[test]
fn a_vertex_is_marked_by_a_square_eight_pixels_across() {
    let (from, to) = (DVec2::new(4.0, 5.0), DVec2::new(40.0, 25.0));
    let (sketch, lines, _) = a_frame_of(
        a_part_with_a_trait(from, to, false),
        SketchEditor::default(),
        &Theme::default(),
    );
    let half = 4.0 * a_view().units_per_pixel;

    let painted: Vec<DVec2> = lines.iter().map(|vertex| place(vertex, &sketch)).collect();
    for corner in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
        let at = to + DVec2::from(corner) * half;
        assert!(
            painted.iter().any(|place| place.distance(at) < TOLERANCE),
            "no corner of a square 8 px across stands at {at:?}, around the vertex at {to:?}",
        );
    }
}

/// An editor drawing a line from `from`, the cursor at `cursor`.
fn drawing_a_line(from: DVec2, cursor: DVec2) -> SketchEditor {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor.tool = Tool::Line;
    editor.tool_state = ToolState::Line {
        anchor: ChainAnchor::Pending(from),
        previous: None,
    };
    editor.cursor = Some(cursor);
    editor
}

#[test]
fn the_provisional_trait_of_the_tool_in_progress_is_drawn_amber() {
    let (from, cursor) = (DVec2::new(-30.0, -10.0), DVec2::new(-2.0, 12.0));
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });

    let (sketch, lines, _) = a_frame_of(document, drawing_a_line(from, cursor), &Theme::default());

    let provisional = painted_along(&lines, &sketch, from, cursor);
    let (amber, _) = redesign(&["sketch", "tool_in_progress"]);
    assert!(
        !provisional.is_empty(),
        "nothing is painted from where the line starts to the cursor",
    );
    let wrong = not_in(&provisional, amber);
    assert!(
        wrong.is_empty(),
        "the trait the next click would lay is painted the redesign's amber, {amber:?}, \
         whatever its opacity, not {wrong:?}",
    );
}

/// The default theme with colours of it set as a profile sets them, each under
/// the name the profile's file writes it under.
fn a_theme_setting(colours: &[(&str, Rgba)]) -> Theme {
    let mut written = serde_json::to_value(Theme::default()).expect("a theme writes out");
    for (name, colour) in colours {
        written[*name] = serde_json::to_value(colour).expect("a colour writes out");
    }
    serde_json::from_value(written).expect("a theme reads back")
}

#[test]
fn the_construction_and_tool_colours_a_profile_sets_are_the_ones_drawn() {
    let (construction, tool) = (
        Rgba::opaque(0.20, 0.80, 0.30),
        Rgba::opaque(0.90, 0.10, 0.60),
    );
    let theme = a_theme_setting(&[
        ("sketch_construction", construction),
        ("tool_in_progress", tool),
    ]);
    let (from, to) = (DVec2::new(4.0, 5.0), DVec2::new(40.0, 25.0));
    let (start, cursor) = (DVec2::new(-30.0, -10.0), DVec2::new(-2.0, 12.0));

    let (sketch, lines, _) = a_frame_of(
        a_part_with_a_trait(from, to, true),
        drawing_a_line(start, cursor),
        &theme,
    );

    let dashes = painted_along(&lines, &sketch, from, to);
    let provisional = painted_along(&lines, &sketch, start, cursor);
    assert!(
        !dashes.is_empty() && !provisional.is_empty(),
        "the construction trait or the line in progress is not painted at all",
    );
    let wrong = (
        not_in(&dashes, hex_of(tint(construction))),
        not_in(&provisional, hex_of(tint(tool))),
    );
    assert!(
        wrong.0.is_empty() && wrong.1.is_empty(),
        "a profile's construction colour, {construction:?}, and its tool colour, {tool:?}, \
         are the ones the construction trait and the line in progress are painted in, \
         not {wrong:?}",
    );
}

#[test]
fn a_copy_s_preview_shows_with_the_pointer_off_the_drawing() {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(4.0, 5.0)),
        end: PointRef::New(DVec2::new(8.0, 5.0)),
        construction: false,
    });
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    crate::commands::run(
        cao_prefs::Command::ToolMirror,
        &mut context,
        &mut crate::screens::ribbon::Ribbon::default(),
        &mut crate::screens::viewport::ViewportState::default(),
        &mut crate::screens::blend::BlendState::default(),
    );
    let (snap, pixel) = (0.5, 0.05);
    copy(&mut context, 0, DVec2::new(6.0, 5.0), snap, pixel);
    enter_in_the_panel(&mut context);
    copy(&mut context, 0, DVec2::new(20.0, 0.0), snap, pixel);

    context.editor.cursor = None;
    let sketch = context.document.sketches()[0].clone();

    assert!(
        what_would_be_laid(&sketch, &context, a_view()).is_some(),
        "the trait held, mirrored across the axis the panel holds, is shown with \
         the pointer over the panel"
    );
}

#[test]
fn a_dimension_is_drawn_with_an_arrow_at_each_end() {
    let mut document = a_part_with_a_rectangle();
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 10.0.into(),
        placement: None,
    });
    let theme = Theme::default();
    let (sketch, lines, _) = a_frame_of(document, SketchEditor::default(), &theme);
    let (start, end) = sketch.endpoints(SegmentId(0));
    let along = (end - start).normalize();
    let pixel = a_view().units_per_pixel;

    let dimension = tint(theme.dimension);
    let drawn: Vec<(DVec2, DVec2)> = lines
        .chunks(2)
        .filter(|ends| ends[0].color == dimension)
        .map(|ends| (place(&ends[0], &sketch), place(&ends[1], &sketch)))
        .collect();
    let Some(&(from, to)) = drawn.iter().find(|(from, to)| {
        let run = *to - *from;
        run.perp_dot(along).abs() < TOLERANCE && (run.length() - start.distance(end)).abs() < 1e-3
    }) else {
        panic!("no dimension line runs alongside the side the length holds: {drawn:?}");
    };

    for (tip, inward) in [(from, to - from), (to, from - to)] {
        let heads: Vec<DVec2> = drawn
            .iter()
            .filter_map(
                |&(one, other)| match (one.distance(tip), other.distance(tip)) {
                    (near, _) if near < TOLERANCE && other.distance(tip) > TOLERANCE => Some(other),
                    (_, near) if near < TOLERANCE && one.distance(tip) > TOLERANCE => Some(one),
                    _ => None,
                },
            )
            .map(|back| back - tip)
            .filter(|back| back.perp_dot(inward).abs() > TOLERANCE)
            .collect();
        assert_eq!(
            heads.len(),
            2,
            "two strokes make the arrow at {tip:?}, the end of the dimension line: {heads:?}",
        );
        let [one, other] = [heads[0], heads[1]];
        assert!(
            one.dot(inward) > 0.0 && other.dot(inward) > 0.0,
            "the arrow at {tip:?} opens back along the line, not out past its end: {heads:?}",
        );
        assert!(
            one.perp_dot(inward).signum() != other.perp_dot(inward).signum(),
            "the arrow at {tip:?} has its two strokes on the same side of the line: {heads:?}",
        );
        for stroke in heads {
            let long = stroke.length() / pixel;
            assert!(
                (6.0..=12.0).contains(&long),
                "a stroke of the arrow at {tip:?} is {long:.1} px long on screen, no arrowhead",
            );
        }
    }
}

/// The steps one frame paints in `hex`, in the coordinates of the drawing.
fn painted_in(lines: &[cao_render::Vertex], sketch: &Sketch, hex: [u8; 3]) -> Vec<(DVec2, DVec2)> {
    lines
        .chunks(2)
        .filter(|ends| hex_of(ends[0].color) == hex)
        .map(|ends| (place(&ends[0], sketch), place(&ends[1], sketch)))
        .collect()
}

/// The steps among `steps` whose two ends stand at one same distance from
/// `centre`, that distance between `near` and `far` pixels on screen: the
/// pieces of a ring or of an arc drawn round it.
fn round(steps: &[(DVec2, DVec2)], centre: DVec2, near: f64, far: f64) -> Vec<(DVec2, DVec2)> {
    let pixel = a_view().units_per_pixel;
    steps
        .iter()
        .copied()
        .filter(|(one, other)| {
            let (one, other) = (one.distance(centre), other.distance(centre));
            (one - other).abs() <= 1e-3 * one.max(other)
                && (near..=far).contains(&(one / pixel))
                && (near..=far).contains(&(other / pixel))
        })
        .collect()
}

/// Whether some of `steps`, at one radius, go all the way round `centre`, no
/// gap between them wider than a sixth of a turn. Other shapes drawn round the
/// same place at another radius — the square the point tool draws at the
/// cursor — are no part of the ring and are left aside.
fn goes_all_the_way_round(steps: &[(DVec2, DVec2)], centre: DVec2) -> bool {
    steps.iter().any(|(one, _)| {
        let radius = one.distance(centre);
        let ring: Vec<(DVec2, DVec2)> = steps
            .iter()
            .copied()
            .filter(|(end, _)| (end.distance(centre) - radius).abs() <= 0.02 * radius)
            .collect();
        let mut angles: Vec<f64> = ring
            .iter()
            .flat_map(|(one, other)| [*one, *other])
            .map(|end| (end - centre).to_angle())
            .collect();
        angles.sort_by(f64::total_cmp);
        let widest_gap = angles
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .chain(
                angles
                    .first()
                    .zip(angles.last())
                    .map(|(first, last)| first + std::f64::consts::TAU - last),
            )
            .fold(0.0, f64::max);
        ring.len() >= 6 && widest_gap <= std::f64::consts::TAU / 6.0
    })
}

/// A theme whose tool colour is a teal of its own, amber being only its
/// default: what is drawn in the tool's colour has to follow the profile, and
/// the chrome's amber, which some tokens already wear, would hide one that
/// does not.
fn a_theme_with_its_own_tool_colour() -> Theme {
    Theme {
        tool_in_progress: Rgba::opaque(40.0 / 255.0, 180.0 / 255.0, 160.0 / 255.0),
        ..Theme::default()
    }
}

#[test]
fn each_snap_is_ringed_in_the_tool_s_amber() {
    let theme = a_theme_with_its_own_tool_colour();
    let amber = hex_of(tint(theme.tool_in_progress));
    let snaps = [
        (Snap::Point, DVec2::new(20.0, 10.0)),
        (
            Snap::Midpoint(DVec2::new(40.0, 10.0)),
            DVec2::new(40.0, 10.0),
        ),
        (
            Snap::OnCurve(DVec2::new(50.0, 10.0)),
            DVec2::new(50.0, 10.0),
        ),
        (
            Snap::Crossing(DVec2::new(30.0, 10.0)),
            DVec2::new(30.0, 10.0),
        ),
        (Snap::Centre(DVec2::new(35.0, 25.0)), DVec2::new(35.0, 25.0)),
    ];

    let unringed: Vec<String> = snaps
        .into_iter()
        .filter(|(snap, at)| {
            let mut editor = SketchEditor::default();
            editor.begin_editing(0, WorkPlane::XY);
            editor.tool = Tool::Line;
            editor.cursor = Some(*at);
            editor.snap = Some(*snap);
            let document =
                a_part_with_a_trait(DVec2::new(20.0, 10.0), DVec2::new(60.0, 10.0), false);
            let (sketch, lines, _) = a_frame_of(document, editor, &theme);
            let ring = round(&painted_in(&lines, &sketch, amber), *at, 4.0, 16.0);
            !goes_all_the_way_round(&ring, *at)
        })
        .map(|(snap, at)| format!("{snap:?} at {at:?}"))
        .collect();

    assert!(
        unringed.is_empty(),
        "no ring 8 to 32 px across, in the tool's amber {amber:?}, goes round the place the \
         cursor was pulled onto, for:\n  {}",
        unringed.join("\n  "),
    );
}

/// An editor drawing a line from where `anchor` says, the chain having laid
/// `previous` just before, the cursor at `cursor`.
fn drawing_on_from(
    anchor: ChainAnchor,
    previous: Option<SegmentId>,
    cursor: DVec2,
) -> SketchEditor {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor.tool = Tool::Line;
    editor.tool_state = ToolState::Line { anchor, previous };
    editor.cursor = Some(cursor);
    editor
}

#[test]
fn a_line_in_progress_shows_its_angle_as_an_amber_arc() {
    let theme = a_theme_with_its_own_tool_colour();
    let amber = hex_of(tint(theme.tool_in_progress));
    let (corner, cursor) = (DVec2::new(10.0, 10.0), DVec2::new(10.0, 40.0));
    let a_trait_into_the_corner = || a_part_with_a_trait(DVec2::new(40.0, 40.0), corner, false);
    let its_end = a_trait_into_the_corner().sketches()[0].segments()[0].end;
    let cases = [
        (
            "from no trait, against the horizontal",
            a_part_with_a_trait(DVec2::new(80.0, 80.0), DVec2::new(90.0, 80.0), false),
            drawing_on_from(ChainAnchor::Pending(corner), None, DVec2::new(-20.0, 40.0)),
            DVec2::X,
            DVec2::new(-1.0, 1.0),
            135.0,
        ),
        (
            "on from the trait the chain laid just before",
            a_trait_into_the_corner(),
            drawing_on_from(ChainAnchor::Point(its_end), Some(SegmentId(0)), cursor),
            DVec2::new(1.0, 1.0),
            DVec2::Y,
            45.0,
        ),
        (
            "on from the only trait ending where it starts",
            a_trait_into_the_corner(),
            drawing_on_from(ChainAnchor::Point(its_end), None, cursor),
            DVec2::new(1.0, 1.0),
            DVec2::Y,
            45.0,
        ),
    ];

    for (how, document, editor, reference, line, sweep) in cases {
        let (sketch, lines, _) = a_frame_of(document, editor, &theme);
        let arc = round(&painted_in(&lines, &sketch, amber), corner, 12.0, 80.0);
        assert!(
            arc.len() >= 3,
            "drawing a line {how}, no amber arc 12 to 80 px from its start at {corner:?}: \
             the frame paints in amber {:?}",
            painted_in(&lines, &sketch, amber),
        );
        let reference = reference.normalize();
        let turning = reference.perp_dot(line).signum();
        let swept: Vec<f64> = arc
            .iter()
            .flat_map(|(one, other)| [*one, *other])
            .map(|end| {
                let towards = end - corner;
                turning
                    * reference
                        .perp_dot(towards)
                        .atan2(reference.dot(towards))
                        .to_degrees()
            })
            .collect();
        let (least, most) = swept
            .iter()
            .fold((f64::MAX, f64::MIN), |(low, high), angle| {
                (low.min(*angle), high.max(*angle))
            });
        assert!(
            least.abs() <= 2.0 && (most - sweep).abs() <= 2.0,
            "drawing a line {how}, its arc runs from {least:.1}° to {most:.1}° off the \
             reference, where it spans the {sweep}° from the reference round to the line",
        );
        let radii: Vec<f64> = arc.iter().map(|(one, _)| one.distance(corner)).collect();
        let (smallest, largest) = radii
            .iter()
            .fold((f64::MAX, 0.0_f64), |(low, high), radius| {
                (low.min(*radius), high.max(*radius))
            });
        assert!(
            largest - smallest <= 0.02 * largest,
            "drawing a line {how}, the arc is no arc of one circle: radii {smallest} to {largest}",
        );
    }
}

#[test]
fn no_ring_is_drawn_once_the_cursor_has_left_the_canvas() {
    let theme = a_theme_with_its_own_tool_colour();
    let tool = hex_of(tint(theme.tool_in_progress));
    let snaps = [
        (Snap::Point, DVec2::new(20.0, 10.0)),
        (
            Snap::Midpoint(DVec2::new(40.0, 10.0)),
            DVec2::new(40.0, 10.0),
        ),
        (
            Snap::OnCurve(DVec2::new(50.0, 10.0)),
            DVec2::new(50.0, 10.0),
        ),
        (
            Snap::Crossing(DVec2::new(30.0, 10.0)),
            DVec2::new(30.0, 10.0),
        ),
        (Snap::Centre(DVec2::new(35.0, 25.0)), DVec2::new(35.0, 25.0)),
    ];

    let still_ringed: Vec<String> = snaps
        .into_iter()
        .filter(|(snap, at)| {
            let mut editor = SketchEditor::default();
            editor.begin_editing(0, WorkPlane::XY);
            editor.tool = Tool::Line;
            editor.cursor = None;
            editor.snap = Some(*snap);
            let document =
                a_part_with_a_trait(DVec2::new(20.0, 10.0), DVec2::new(60.0, 10.0), false);
            let (sketch, lines, _) = a_frame_of(document, editor, &theme);
            !round(&painted_in(&lines, &sketch, tool), *at, 4.0, 16.0).is_empty()
        })
        .map(|(snap, at)| format!("{snap:?} at {at:?}"))
        .collect();

    assert!(
        still_ringed.is_empty(),
        "with the cursor off the canvas, the last snap it was pulled onto is still ringed:\n  {}",
        still_ringed.join("\n  "),
    );
}

#[test]
fn only_a_tool_that_places_a_point_rings_its_snap() {
    let theme = a_theme_with_its_own_tool_colour();
    let tool_colour = hex_of(tint(theme.tool_in_progress));
    let at = DVec2::new(40.0, 10.0);
    let rings = |tool: Tool| {
        let mut editor = SketchEditor::default();
        editor.begin_editing(0, WorkPlane::XY);
        editor.tool = tool;
        editor.cursor = Some(at);
        editor.snap = Some(Snap::Midpoint(at));
        let document = a_part_with_a_trait(DVec2::new(20.0, 10.0), DVec2::new(60.0, 10.0), false);
        let (sketch, lines, _) = a_frame_of(document, editor, &theme);
        goes_all_the_way_round(
            &round(&painted_in(&lines, &sketch, tool_colour), at, 4.0, 16.0),
            at,
        )
    };

    let placing = [
        Tool::Line,
        Tool::LineSymmetric,
        Tool::Rectangle,
        Tool::Circle,
        Tool::Arc,
        Tool::Ellipse,
        Tool::Point,
    ];
    let picking = [
        Tool::Dimension,
        Tool::Measure,
        Tool::Trim,
        Tool::Split,
        Tool::Chamfer,
        Tool::Fillet,
        Tool::Constrain(cao_sketch::Rule::Coincident),
    ];
    let unringed: Vec<Tool> = placing.into_iter().filter(|tool| !rings(*tool)).collect();
    let ringed: Vec<Tool> = picking.into_iter().filter(|tool| rings(*tool)).collect();

    assert!(
        unringed.is_empty() && ringed.is_empty(),
        "the ring says where a point is about to be laid: a tool that lays one rings its \
         snap, and one that only picks what is drawn rings nothing — rings missing for \
         {unringed:?}, rings drawn for {ringed:?}",
    );
}

#[test]
fn the_provisional_trait_is_drawn_solid_at_the_sketch_width() {
    let theme = a_theme_with_its_own_tool_colour();
    let tool_colour = hex_of(tint(theme.tool_in_progress));
    let pixel = a_view().units_per_pixel;
    let (start, cursor) = (DVec2::new(-30.0, -10.0), DVec2::new(-2.0, 12.0));
    let empty = || {
        let mut document = PartDocument::new("part", Utc::now());
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        document
    };
    let in_the_tool_colour = |steps: Vec<Step>| -> Vec<Step> {
        steps
            .into_iter()
            .filter(|step| hex_of(step.color) == tool_colour)
            .collect()
    };

    let mut drawn: Vec<(&str, Vec<Step>)> = Vec::new();

    let (sketch, lines, _) = a_frame_of(empty(), drawing_a_line(start, cursor), &theme);
    drawn.push((
        "a line",
        in_the_tool_colour(painted_along(&lines, &sketch, start, cursor)),
    ));

    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor.tool = Tool::Rectangle;
    editor.tool_state = ToolState::Rectangle { start };
    editor.cursor = Some(cursor);
    let (sketch, lines, _) = a_frame_of(empty(), editor, &theme);
    let far = [
        DVec2::new(cursor.x, start.y),
        cursor,
        DVec2::new(start.x, cursor.y),
        start,
    ];
    let sides = [start, far[0], far[1], far[2]]
        .into_iter()
        .zip(far)
        .flat_map(|(from, to)| painted_along(&lines, &sketch, from, to))
        .collect();
    drawn.push(("a rectangle", in_the_tool_colour(sides)));

    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor.tool = Tool::Circle;
    editor.tool_state = ToolState::Circle {
        points: vec![start],
        segments: Vec::new(),
    };
    editor.cursor = Some(cursor);
    let (sketch, lines, _) = a_frame_of(empty(), editor, &theme);
    let radius = start.distance(cursor) / pixel;
    let rim: Vec<Step> = lines
        .chunks(2)
        .map(|ends| Step {
            from: place(&ends[0], &sketch),
            to: place(&ends[1], &sketch),
            color: ends[0].color,
            width: ends[0].width,
        })
        .filter(|step| {
            [step.from, step.to]
                .iter()
                .all(|end| (end.distance(start) / pixel - radius).abs() <= 0.02 * radius)
        })
        .collect();
    drawn.push(("a circle", in_the_tool_colour(rim)));

    let mut wrong = Vec::new();
    for (what, steps) in drawn {
        if steps.is_empty() {
            wrong.push(format!(
                "{what} in progress is not drawn in the tool's colour at all"
            ));
            continue;
        }
        let faint: Vec<(f32, f32)> = steps
            .iter()
            .filter(|step| step.width != theme.sketch_width || step.color[3] < 0.9)
            .map(|step| (step.width, step.color[3]))
            .collect();
        if !faint.is_empty() {
            wrong.push(format!(
                "{what} in progress is drawn {} px wide at an opacity of {}, where a laid \
                 stroke is {} px and the trait in progress is solid",
                faint[0].0, faint[0].1, theme.sketch_width,
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "the trait in progress is drawn as solid and as wide as a laid one:\n  {}",
        wrong.join("\n  "),
    );
}
