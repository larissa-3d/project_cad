//! What app · screens/viewport/extrusion_handle.rs is held to.
//!
//! Closes #741.
//! - On the preview, an amber arrow along the direction of extrusion, its value
//!   beside it in a pill — the arrow rising from the middle of the areas
//!   picked to the depth, and the other way along the plane once reversed —
//!   `the_arrow_rises_from_the_middle_of_the_areas_to_the_depth_typed`,
//!   `reversed_the_arrow_points_the_other_way_along_the_plane`; its pill
//!   reading what the depth comes to, in millimetres as the panel's « = 30
//!   mm » writes them —
//!   `the_pill_beside_the_arrow_reads_what_the_depth_comes_to`; painted amber —
//!   `the_arrow_and_its_pill_are_painted_amber`
//! - Dragging it changes the depth, and the panel's field follows —
//!   `pulling_the_arrow_changes_the_depth_and_the_panel_s_field_follows`; in
//!   millimetres on a part drawn at a scale, as the field reads them —
//!   `on_a_part_drawn_at_a_scale_the_arrow_moves_the_depth_by_the_millimetres_it_spans`
//! - A depth written as a formula becomes a number once dragged; « Annuler » of
//!   the panel, or undo, gives the formula back —
//!   `a_depth_written_as_a_formula_is_a_number_once_dragged`,
//!   `cancelling_the_panel_after_a_drag_gives_the_formula_back`,
//!   `undo_after_a_drag_gives_the_formula_back`
//! - Dragging past the sketch's plane reverses the direction, as « Inverser le
//!   sens » does —
//!   `pulling_the_arrow_past_the_plane_reverses_the_direction_as_the_reverse_switch_does`;
//!   a pull landing on the plane, where nothing would be raised, leaves the
//!   depth and the direction where the last pull put them —
//!   `a_pull_landing_on_the_plane_leaves_the_depth_where_the_last_pull_put_it`;
//!   and « Annuler » gives the direction back with the depth —
//!   `cancelling_the_panel_after_a_drag_past_the_plane_gives_the_direction_back`
//! - For a revolution, the handle drags the angle along an arc —
//!   `for_a_revolution_the_arrow_stands_on_an_arc_about_the_axis_at_the_angle_typed`,
//!   `for_a_revolution_pulling_the_arrow_round_its_arc_changes_the_angle`;
//!   past the plane it turns the other way, as for a depth, and never past a
//!   whole turn —
//!   `for_a_revolution_pulling_back_past_the_plane_turns_the_other_way_and_never_past_a_whole_turn`;
//!   and « Annuler » gives the angle back as it does a depth —
//!   `cancelling_the_panel_after_a_drag_round_the_arc_gives_the_angle_back`
//! - Must not break: typing the depth; the formula field's completion — no
//!   test: both are held as they stand by
//!   `a_depth_written_with_a_variable_shows_it_as_a_chip_and_what_it_comes_to`
//!   in `crates/app/tests/extruding_in_a_panel.rs` and
//!   `the_start_of_a_name_in_the_extrusion_panel_is_completed_too` in
//!   `crates/app/tests/completion.rs`, which this branch leaves as they are
//!   and which stay green
//! - Must not break: orbiting the view while extruding: grabbing the handle is
//!   a drag on the handle only — `a_ray_far_from_the_arrow_takes_no_hold_of_it`,
//!   `a_drag_on_the_arrow_pulls_the_depth_and_leaves_the_view_as_it_was`,
//!   `an_orbit_over_the_arrow_turns_the_view_and_leaves_the_depth`,
//!   `only_the_primary_button_pulls_the_arrow`,
//!   `the_arrow_takes_no_gesture_while_nothing_is_previewed`. Each frame is
//!   handed to the camera's navigation and then to the arrow, in the order the
//!   frame loop hands them; the frame loop itself, `view.rs`, has no net
//!
//! Snapping the drag to faces of the part is out of the issue's scope.
//!
//! Read when the work opened: the arrow stands on the sketch's plane at the
//! middle of the areas picked and runs along its normal, its tip at the depth
//! as signed by the direction; a pull sets the depth to where the pointer's
//! ray passes nearest that line, written in the panel's field as a plain
//! number, the direction turned round once the pointer passes the plane.
//! For a revolution the arrow is an arc: from the middle of the areas, round
//! the axis by the angle, the way the kernel turns — by the right hand about
//! the axis. A pull reads the angle where the pointer's ray meets the plane
//! square to the axis through that middle, round the axis from the middle,
//! following the pointer from one pull to the next rather than jumping at
//! half a turn.

use cao_part::{ExtrusionMode, Formula, Operation, PartDocument, PointRef, VariableChange};
use cao_prefs::Command;
use cao_sketch::{DimensionTarget, SegmentId, WorkPlane};
use chrono::Utc;
use glam::{DVec2, DVec3, Vec3};

use super::*;
use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::blend::BlendState;
use crate::screens::extrusion::{ExtrusionState, Shape};
use crate::screens::ribbon::Ribbon;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::input::point_the_extrusion_arrow;
use crate::screens::viewport::navigation::handle_navigation;
use crate::screens::viewport::render::{build_frame, paint_extrusion_pill};
use crate::screens::viewport::{ViewScale, ViewportState, cube_rect};
use crate::ui::theme::tokens::{ACCENT_AMBER, ACCENT_AMBER_BORDER, ACCENT_AMBER_TEXT};

/// Every pull below lands on a whole millimetre of the normal, which the point
/// of a ray square to it nearest the normal finds exactly, and which the text
/// of the field keeps however it is rounded.
const TOLERANCE: f64 = 1e-9;

/// How near the arrow a ray has to pass to take hold of it, in the part's
/// units.
const REACH: f64 = 0.5;

/// The way every ray below runs: from the side, square to the square's normal.
const SIDEWAYS: DVec3 = DVec3::NEG_X;

/// A square 20 by 20 about the origin, drawn on XY with no dimension: its
/// middle is the origin, its normal Z, and a unit of the view is a millimetre.
fn a_part_with_a_square() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(-10.0, -10.0)),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    document
}

/// The square picked and the tool armed to add, the depth as it is offered:
/// 10, the arrow's tip ten above the square's middle.
fn extruding_the_square() -> ExtrusionState {
    let mut extrusion = ExtrusionState::default();
    extrusion.offer(0);
    extrusion.picks = vec![DVec2::ZERO];
    extrusion.arm(ExtrusionMode::Add);
    extrusion
}

/// Where a ray starts that crosses the square's normal `height` above it.
fn beside(height: f64) -> DVec3 {
    DVec3::new(30.0, 0.0, height)
}

/// What the panel's field reads, and how deep the volume previewed goes.
fn field_and_preview(
    extrusion: &ExtrusionState,
    document: &PartDocument,
) -> (Option<f64>, Option<f64>) {
    let variables = document.variables();
    let field = variables
        .size_of(&extrusion.distance_input)
        .ok()
        .map(|(_, value)| value);
    let previewed = extrusion
        .preview(document)
        .and_then(|preview| match preview.operation {
            Operation::Extrude { distance, .. } => distance.value(&variables.values()),
            _ => None,
        });
    (field, previewed)
}

fn both_read(shown: (Option<f64>, Option<f64>), depth: f64) -> bool {
    let near = |value: Option<f64>| value.is_some_and(|value| (value - depth).abs() <= TOLERANCE);
    near(shown.0) && near(shown.1)
}

#[test]
fn pulling_the_arrow_changes_the_depth_and_the_panel_s_field_follows() {
    let document = a_part_with_a_square();
    let mut extrusion = extruding_the_square();
    let mut handle = ExtrusionHandle::default();

    let taken = handle.grab(&extrusion, &document, beside(10.0), SIDEWAYS, REACH);
    handle.pull(&mut extrusion, beside(25.0), SIDEWAYS);
    let pulled_up = field_and_preview(&extrusion, &document);
    handle.pull(&mut extrusion, beside(4.0), SIDEWAYS);
    let pulled_down = field_and_preview(&extrusion, &document);
    handle.let_go();
    handle.pull(&mut extrusion, beside(18.0), SIDEWAYS);
    let let_go = field_and_preview(&extrusion, &document);

    assert!(
        taken,
        "a ray through the tip of the arrow, ten above the square's middle, takes no hold of it",
    );
    assert!(
        both_read(pulled_up, 25.0),
        "pulled from 10 up to 25, the field reads {:?} and the preview goes {:?} deep, \
         not 25 both",
        pulled_up.0,
        pulled_up.1,
    );
    assert!(
        both_read(pulled_down, 4.0),
        "pulled back down to 4, the field reads {:?} and the preview goes {:?} deep, \
         not 4 both",
        pulled_down.0,
        pulled_down.1,
    );
    assert!(
        both_read(let_go, 4.0),
        "let go at 4, the arrow still follows the pointer: the field reads {:?} and the \
         preview goes {:?} deep",
        let_go.0,
        let_go.1,
    );
}

/// A depth written with the part's one variable: it comes to 30.
const TWICE_THE_THICKNESS: &str = "thickness * 2";

/// The square, and a variable `thickness` of 15.
fn a_part_with_a_thickness_and_a_square() -> PartDocument {
    let mut document = a_part_with_a_square();
    document.apply(Operation::Variable(VariableChange::Added {
        name: "thickness".to_owned(),
        formula: Formula::Number(15.0),
    }));
    document
}

/// The arrow taken at its tip, `from` above the square, pulled to `to` and let
/// go: whether it was taken.
fn dragged(extrusion: &mut ExtrusionState, document: &PartDocument, from: f64, to: f64) -> bool {
    let mut handle = ExtrusionHandle::default();
    let taken = handle.grab(extrusion, document, beside(from), SIDEWAYS, REACH);
    handle.pull(extrusion, beside(to), SIDEWAYS);
    handle.let_go();
    taken
}

/// `command` run as a button of the interface runs it, on the part and the
/// extrusion's panel.
fn run_on(command: Command, document: &mut PartDocument, extrusion: &mut ExtrusionState) {
    let lang = Catalogue::french();
    let mut editor = SketchEditor::default();
    let mut part = SketchContext {
        document,
        editor: &mut editor,
        extrusion,
        lang: &lang,
    };
    crate::commands::run(
        command,
        &mut part,
        &mut Ribbon::default(),
        &mut ViewportState::default(),
        &mut BlendState::default(),
    );
}

/// What the depth field holds once the step at that place in the history is
/// reopened.
fn reopened_field(document: &PartDocument, at: usize) -> Option<String> {
    ExtrusionState::reopened(document, at).map(|reopened| reopened.distance_input)
}

#[test]
fn a_depth_written_as_a_formula_is_a_number_once_dragged() {
    let document = a_part_with_a_thickness_and_a_square();
    let mut extrusion = extruding_the_square();
    extrusion.distance_input = TWICE_THE_THICKNESS.to_owned();

    let taken = dragged(&mut extrusion, &document, 30.0, 42.0);

    let read = document.variables().read(&extrusion.distance_input);
    assert!(
        taken,
        "a ray through the tip of the arrow, at the 30 the formula comes to, takes no hold of it",
    );
    assert!(
        matches!(read, Ok(Formula::Number(depth)) if (depth - 42.0).abs() <= TOLERANCE),
        "dragged from « {TWICE_THE_THICKNESS} » to 42, the field holds « {} », read as {read:?}, \
         not the number 42",
        extrusion.distance_input,
    );
}

#[test]
fn cancelling_the_panel_after_a_drag_gives_the_formula_back() {
    let mut document = a_part_with_a_thickness_and_a_square();
    let operations = document.history.operations().len();
    let mut extrusion = extruding_the_square();
    extrusion.distance_input = TWICE_THE_THICKNESS.to_owned();
    let mut never_dragged = extruding_the_square();

    let taken = [
        dragged(&mut extrusion, &document, 30.0, 42.0),
        dragged(&mut extrusion, &document, 42.0, 12.0),
    ];
    run_on(Command::ExtrusionCancel, &mut document, &mut extrusion);
    let once = (
        extrusion.distance_input.clone(),
        extrusion.panel(&document).is_some(),
    );
    run_on(Command::ExtrusionCancel, &mut document, &mut extrusion);
    let twice = extrusion.panel(&document).is_some();
    run_on(Command::ExtrusionCancel, &mut document, &mut never_dragged);

    assert_eq!(
        taken,
        [true, true],
        "the arrow was not taken at its tip, at 30 then at 42",
    );
    assert_eq!(
        once,
        (TWICE_THE_THICKNESS.to_owned(), true),
        "dragged twice, then « Annuler »: the panel is not left open on the formula typed \
         before the drags",
    );
    assert!(!twice, "a second « Annuler » leaves the panel open");
    assert!(
        never_dragged.panel(&document).is_none(),
        "a panel never dragged is left open by its first « Annuler »",
    );
    assert_eq!(
        document.history.operations().len(),
        operations,
        "cancelling the panel recorded something in the part",
    );
}

#[test]
fn undo_after_a_drag_gives_the_formula_back() {
    let mut document = a_part_with_a_thickness_and_a_square();
    let raise = document.history.operations().len();
    let twice = document
        .variables()
        .read(TWICE_THE_THICKNESS)
        .expect("twice the thickness reads");
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::ZERO]),
        distance: twice,
        mode: ExtrusionMode::Add,
    });
    let mut extrusion =
        ExtrusionState::reopened(&document, raise).expect("the square's raise reopens");

    let taken = dragged(&mut extrusion, &document, 30.0, 42.0);
    run_on(Command::ExtrusionApply, &mut document, &mut extrusion);
    let applied = reopened_field(&document, raise);
    run_on(Command::Undo, &mut document, &mut extrusion);
    let undone = reopened_field(&document, raise);

    assert!(
        taken,
        "the arrow was not taken at its tip, at the 30 the formula comes to",
    );
    assert!(
        applied
            .as_deref()
            .and_then(|field| document.variables().size_of(field).ok())
            .is_some_and(|(_, depth)| (depth - 42.0).abs() <= TOLERANCE),
        "dragged to 42 and applied, the step reopens on {applied:?}, not 42",
    );
    assert_eq!(
        undone.as_deref(),
        Some(TWICE_THE_THICKNESS),
        "undone, the step reopens on {undone:?}, not on the formula it was raised with",
    );
}

fn reads(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| (value - expected).abs() <= TOLERANCE)
}

/// The step the panel previews, as applying would record it.
fn previewed(extrusion: &ExtrusionState, document: &PartDocument) -> Option<Operation> {
    extrusion.preview(document).map(|preview| preview.operation)
}

#[test]
fn pulling_the_arrow_past_the_plane_reverses_the_direction_as_the_reverse_switch_does() {
    let document = a_part_with_a_square();
    let mut extrusion = extruding_the_square();
    let mut switched = extruding_the_square();
    switched.reversed = true;
    switched.distance_input = "7".to_owned();
    let mut handle = ExtrusionHandle::default();

    let taken = handle.grab(&extrusion, &document, beside(10.0), SIDEWAYS, REACH);
    handle.pull(&mut extrusion, beside(-7.0), SIDEWAYS);
    let below = (
        extrusion.reversed,
        field_and_preview(&extrusion, &document),
        previewed(&extrusion, &document),
    );
    handle.pull(&mut extrusion, beside(5.0), SIDEWAYS);
    let back_above = (extrusion.reversed, field_and_preview(&extrusion, &document));
    handle.let_go();

    assert!(
        taken,
        "a ray through the tip of the arrow, ten above the square's middle, takes no hold of it",
    );
    let (reversed, (field, depth), step) = below;
    assert!(
        reversed && reads(field, 7.0) && reads(depth, -7.0),
        "pulled from 10 to 7 below the plane, the direction reads reversed {reversed}, the \
         field {field:?} and the preview goes {depth:?}: not reversed, 7 written and 7 below",
    );
    assert_eq!(
        step,
        previewed(&switched, &document),
        "pulled 7 below the plane, the step previewed is not the one « Inverser le sens » and \
         7 typed give",
    );
    let (reversed, shown) = back_above;
    assert!(
        !reversed && both_read(shown, 5.0),
        "pulled back to 5 above the plane, the direction reads reversed {reversed}, the field \
         {:?} and the preview {:?}: not 5 the way it first went",
        shown.0,
        shown.1,
    );
}

#[test]
fn a_pull_landing_on_the_plane_leaves_the_depth_where_the_last_pull_put_it() {
    let document = a_part_with_a_square();
    let mut extrusion = extruding_the_square();
    let mut handle = ExtrusionHandle::default();

    let taken = handle.grab(&extrusion, &document, beside(10.0), SIDEWAYS, REACH);
    handle.pull(&mut extrusion, beside(4.0), SIDEWAYS);
    handle.pull(&mut extrusion, beside(0.0), SIDEWAYS);
    let on_the_plane = (extrusion.reversed, field_and_preview(&extrusion, &document));
    handle.pull(&mut extrusion, beside(1e-7), SIDEWAYS);
    let a_hair_above = (extrusion.reversed, field_and_preview(&extrusion, &document));

    assert!(
        taken,
        "a ray through the tip of the arrow, ten above the square's middle, takes no hold of it",
    );
    for (landed, (reversed, shown)) in [
        ("the plane", on_the_plane),
        ("a ten-millionth above the plane", a_hair_above),
    ] {
        assert!(
            !reversed && both_read(shown, 4.0),
            "pulled from 4 to {landed}, the direction reads reversed {reversed}, the field {:?} \
             and the preview {:?}: not 4 the way it went",
            shown.0,
            shown.1,
        );
    }
}

#[test]
fn cancelling_the_panel_after_a_drag_past_the_plane_gives_the_direction_back() {
    let mut document = a_part_with_a_square();
    let mut extrusion = extruding_the_square();

    let taken = dragged(&mut extrusion, &document, 10.0, -7.0);
    let reversed_by_the_drag = extrusion.reversed;
    run_on(Command::ExtrusionCancel, &mut document, &mut extrusion);
    let shown = field_and_preview(&extrusion, &document);

    assert!(
        taken && reversed_by_the_drag,
        "the arrow taken at 10 and pulled to 7 below the plane did not reverse the direction",
    );
    assert!(
        !extrusion.reversed && both_read(shown, 10.0),
        "« Annuler » after a drag past the plane leaves the direction reversed {}, the field \
         {:?} and the preview {:?}: not 10 the way it went before the drag",
        extrusion.reversed,
        shown.0,
        shown.1,
    );
}

/// The square with its first side, 20 units long, dimensioned 40 mm: a unit
/// of the view is 2 mm.
fn a_square_drawn_at_two_millimetres_a_unit() -> PartDocument {
    let mut document = a_part_with_a_square();
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 40.0.into(),
        placement: None,
    });
    document
}

#[test]
fn on_a_part_drawn_at_a_scale_the_arrow_moves_the_depth_by_the_millimetres_it_spans() {
    let document = a_square_drawn_at_two_millimetres_a_unit();
    let mut extrusion = extruding_the_square();

    let taken = dragged(&mut extrusion, &document, 5.0, 12.0);
    let shown = field_and_preview(&extrusion, &document);

    assert!(
        (document.scale() - 2.0).abs() <= TOLERANCE,
        "the square is drawn at {} mm a unit, not 2",
        document.scale(),
    );
    assert!(
        taken,
        "a ray five units above the square, where 10 mm at 2 mm a unit puts the tip, takes no \
         hold of the arrow",
    );
    assert!(
        both_read(shown, 24.0),
        "pulled from 5 units up to 12, seven units of 2 mm on the 10 it read, the field reads \
         {:?} and the preview {:?}, not 24",
        shown.0,
        shown.1,
    );
}

/// A plane standing upright, three along Y: its `u` is X, its `v` Z, and its
/// normal points down Y.
const UPRIGHT: WorkPlane = WorkPlane {
    origin: DVec3::new(0.0, 3.0, 0.0),
    u: DVec3::X,
    v: DVec3::Z,
};

/// Inside the rectangle of `a_part_with_an_upright_rectangle`, at its middle.
const IN_THE_RECTANGLE: DVec2 = DVec2::new(7.0, 7.0);

/// A rectangle 10 by 6 drawn on `UPRIGHT` from (2, 4): its middle stands at
/// (7, 3, 7) in the world.
fn a_part_with_an_upright_rectangle() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: UPRIGHT,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(2.0, 4.0)),
        opposite: PointRef::New(DVec2::new(12.0, 10.0)),
        construction: false,
    });
    document
}

/// The rectangle picked and the tool armed to add, 10 deep as offered.
fn extruding_the_rectangle() -> ExtrusionState {
    let mut extrusion = ExtrusionState::default();
    extrusion.offer(0);
    extrusion.picks = vec![IN_THE_RECTANGLE];
    extrusion.arm(ExtrusionMode::Add);
    extrusion
}

fn stands_at(place: DVec3, expected: DVec3, what: &str) {
    assert!(
        place.distance(expected) <= TOLERANCE,
        "{what} stands at {place}, not {expected}",
    );
}

#[test]
fn the_arrow_rises_from_the_middle_of_the_areas_to_the_depth_typed() {
    let document = a_part_with_an_upright_rectangle();
    let extrusion = extruding_the_rectangle();

    let arrow = ExtrusionHandle::drawn(&extrusion, &document)
        .expect("no arrow is drawn on the rectangle previewed 10 deep");
    let before_any_area = ExtrusionHandle::drawn(&ExtrusionState::default(), &document);

    stands_at(
        arrow.foot,
        DVec3::new(7.0, 3.0, 7.0),
        "the arrow's foot, on the plane at the rectangle's middle,",
    );
    stands_at(
        arrow.tip,
        DVec3::new(7.0, -7.0, 7.0),
        "the arrow's tip, 10 down Y along the plane's normal,",
    );
    assert!(
        before_any_area.is_none(),
        "an arrow is drawn while nothing is previewed",
    );
}

#[test]
fn reversed_the_arrow_points_the_other_way_along_the_plane() {
    let document = a_part_with_an_upright_rectangle();
    let mut extrusion = extruding_the_rectangle();
    extrusion.reversed = true;

    let arrow = ExtrusionHandle::drawn(&extrusion, &document)
        .expect("no arrow is drawn on the rectangle previewed 10 deep the other way");

    stands_at(
        arrow.foot,
        DVec3::new(7.0, 3.0, 7.0),
        "reversed, the arrow's foot, on the plane at the rectangle's middle,",
    );
    stands_at(
        arrow.tip,
        DVec3::new(7.0, 13.0, 7.0),
        "reversed, the arrow's tip, 10 up Y against the plane's normal,",
    );
}

#[test]
fn the_pill_beside_the_arrow_reads_what_the_depth_comes_to() {
    let document = a_part_with_a_thickness_and_a_square();
    let pill = |typed: &str, reversed: bool| {
        let mut extrusion = extruding_the_square();
        extrusion.distance_input = typed.to_owned();
        extrusion.reversed = reversed;
        ExtrusionHandle::drawn(&extrusion, &document).map(|arrow| arrow.pill)
    };

    let read = [
        pill(TWICE_THE_THICKNESS, false),
        pill("12.5", false),
        pill("10", true),
    ];

    assert_eq!(
        read,
        [
            Some("30 mm".to_owned()),
            Some("12.5 mm".to_owned()),
            Some("10 mm".to_owned()),
        ],
        "the pill does not read what « {TWICE_THE_THICKNESS} », « 12.5 » and « 10 » the other \
         way come to, in millimetres as the panel writes them",
    );
}

/// Inside the rectangle of `a_part_with_a_rectangle_beside_the_v_axis`, at
/// its middle: three from the V axis.
const BESIDE_THE_AXIS: DVec2 = DVec2::new(3.0, 1.0);

/// A rectangle 2 by 2 drawn on XY from (2, 0), beside the V axis, and a
/// variable `quarter` of 45. Its middle stands at (3, 1, 0).
fn a_part_with_a_rectangle_beside_the_v_axis() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(2.0, 0.0)),
        opposite: PointRef::New(DVec2::new(4.0, 2.0)),
        construction: false,
    });
    document.apply(Operation::Variable(VariableChange::Added {
        name: "quarter".to_owned(),
        formula: Formula::Number(45.0),
    }));
    document
}

/// An angle written with the part's one variable: it comes to 90.
const TWICE_THE_QUARTER: &str = "quarter * 2";

/// The rectangle picked and turned about V by `angle`, as typed.
fn turning_the_rectangle(angle: &str) -> ExtrusionState {
    let mut extrusion = ExtrusionState::default();
    extrusion.offer(0);
    extrusion.picks = vec![BESIDE_THE_AXIS];
    extrusion.arm(ExtrusionMode::Add);
    extrusion.shape = Shape::Revolution;
    extrusion.angle_input = angle.to_owned();
    extrusion
}

/// Where the rectangle's middle stands once turned `degrees` about V, the way
/// the kernel turns: by the right hand about +Y, which takes +X towards −Z.
fn turned_to(degrees: f64) -> DVec3 {
    let (sin, cos) = degrees.to_radians().sin_cos();
    DVec3::new(3.0 * cos, 1.0, -3.0 * sin)
}

/// How far round V a place stands from the rectangle's middle, the way the
/// kernel turns, in degrees.
fn round_the_axis(place: DVec3) -> f64 {
    (-place.z).atan2(place.x).to_degrees()
}

/// The way every ray of a drag round the arc runs: down the axis, square to
/// the arc.
const DOWN_THE_AXIS: DVec3 = DVec3::NEG_Y;

/// Where a ray down the axis starts that passes through `place`.
fn above(place: DVec3) -> DVec3 {
    place + DVec3::Y * 30.0
}

/// Whether the panel turns the other way, what its angle field reads, and
/// what the revolution previewed turns by.
fn angle_shown(
    extrusion: &ExtrusionState,
    document: &PartDocument,
) -> (bool, Option<f64>, Option<f64>) {
    let variables = document.variables();
    let field = variables
        .size_of(&extrusion.angle_input)
        .ok()
        .map(|(_, value)| value);
    let previewed = extrusion
        .preview(document)
        .and_then(|preview| match preview.operation {
            Operation::Revolve { angle, .. } => angle.value(&variables.values()),
            _ => None,
        });
    (extrusion.reversed, field, previewed)
}

fn turns(shown: (bool, Option<f64>, Option<f64>), reversed: bool, field: f64) -> bool {
    let signed = if reversed { -field } else { field };
    shown.0 == reversed && reads(shown.1, field) && reads(shown.2, signed)
}

/// The arrow taken at the rectangle's middle turned `degrees[0]`, pulled
/// round to each of the others in turn, and let go: whether it was taken.
fn dragged_round(extrusion: &mut ExtrusionState, document: &PartDocument, degrees: &[f64]) -> bool {
    let mut handle = ExtrusionHandle::default();
    let taken = handle.grab(
        extrusion,
        document,
        above(turned_to(degrees[0])),
        DOWN_THE_AXIS,
        REACH,
    );
    for degrees in &degrees[1..] {
        handle.pull(extrusion, above(turned_to(*degrees)), DOWN_THE_AXIS);
    }
    handle.let_go();
    taken
}

#[test]
fn for_a_revolution_the_arrow_stands_on_an_arc_about_the_axis_at_the_angle_typed() {
    let document = a_part_with_a_rectangle_beside_the_v_axis();
    let mut the_other_way = turning_the_rectangle("90");
    the_other_way.reversed = true;

    let arrows = [turning_the_rectangle("90"), the_other_way].map(|extrusion| {
        ExtrusionHandle::drawn(&extrusion, &document)
            .expect("no arrow is drawn on the revolution previewed")
    });

    for (arrow, degrees) in arrows.iter().zip([90.0, -90.0]) {
        stands_at(
            arrow.foot,
            turned_to(0.0),
            &format!("turned {degrees}°, the arrow's foot, at the rectangle's middle,"),
        );
        stands_at(
            arrow.tip,
            turned_to(degrees),
            &format!("turned {degrees}°, the arrow's tip, the middle turned as far,"),
        );
        assert_eq!(
            arrow.pill, "90°",
            "turned {degrees}°, the pill does not read the angle typed"
        );
        let run = arrow.run();
        assert!(
            run.len() > 2
                && run
                    .first()
                    .is_some_and(|first| first.distance(arrow.foot) <= TOLERANCE)
                && run
                    .last()
                    .is_some_and(|last| last.distance(arrow.tip) <= TOLERANCE),
            "turned {degrees}°, the arrow runs {run:?}: not from its foot to its tip, round more \
             than a chord",
        );
        let (low, high) = if degrees > 0.0 {
            (0.0, degrees)
        } else {
            (degrees, 0.0)
        };
        for place in &run {
            let off_the_arc = (place.x.hypot(place.z) - 3.0)
                .abs()
                .max((place.y - 1.0).abs());
            let round = round_the_axis(*place);
            assert!(
                off_the_arc <= TOLERANCE && round >= low - 1e-6 && round <= high + 1e-6,
                "turned {degrees}°, the arrow passes through {place}, {round}° round the axis \
                 and {off_the_arc} off the arc three from it",
            );
        }
    }
}

#[test]
fn for_a_revolution_pulling_the_arrow_round_its_arc_changes_the_angle() {
    let document = a_part_with_a_rectangle_beside_the_v_axis();
    let mut extrusion = turning_the_rectangle(TWICE_THE_QUARTER);
    let mut handle = ExtrusionHandle::default();

    let taken = handle.grab(
        &extrusion,
        &document,
        above(turned_to(90.0)),
        DOWN_THE_AXIS,
        REACH,
    );
    handle.pull(&mut extrusion, above(turned_to(45.0)), DOWN_THE_AXIS);
    let pulled_back = angle_shown(&extrusion, &document);
    let written = document.variables().read(&extrusion.angle_input);
    for degrees in [135.0, 225.0, 300.0] {
        handle.pull(&mut extrusion, above(turned_to(degrees)), DOWN_THE_AXIS);
    }
    let pulled_round = angle_shown(&extrusion, &document);

    assert!(
        taken,
        "a ray down the axis through the arc's tip, a quarter turn round, takes no hold of it",
    );
    assert!(
        turns(pulled_back, false, 45.0),
        "pulled back round to 45°, the panel reads {pulled_back:?}: not 45 the way it turned",
    );
    assert!(
        matches!(written, Ok(Formula::Number(angle)) if (angle - 45.0).abs() <= TOLERANCE),
        "dragged from « {TWICE_THE_QUARTER} », the angle field holds « {} », read as \
         {written:?}, not the number 45",
        extrusion.angle_input,
    );
    assert!(
        turns(pulled_round, false, 300.0),
        "pulled on round through 135° and 225° to 300°, the panel reads {pulled_round:?}: not \
         300 the way it turned, the pointer followed past half a turn",
    );
}

#[test]
fn for_a_revolution_pulling_back_past_the_plane_turns_the_other_way_and_never_past_a_whole_turn() {
    let document = a_part_with_a_rectangle_beside_the_v_axis();
    let mut extrusion = turning_the_rectangle("90");
    let mut handle = ExtrusionHandle::default();

    let taken = handle.grab(
        &extrusion,
        &document,
        above(turned_to(90.0)),
        DOWN_THE_AXIS,
        REACH,
    );
    for degrees in [45.0, 0.5, -30.0] {
        handle.pull(&mut extrusion, above(turned_to(degrees)), DOWN_THE_AXIS);
    }
    let past_the_plane = angle_shown(&extrusion, &document);
    for degrees in [-120.0, -210.0, -300.0, -350.0, -400.0] {
        handle.pull(&mut extrusion, above(turned_to(degrees)), DOWN_THE_AXIS);
    }
    let past_a_whole_turn = angle_shown(&extrusion, &document);

    assert!(
        taken,
        "a ray down the axis through the arc's tip, a quarter turn round, takes no hold of it",
    );
    assert!(
        turns(past_the_plane, true, 30.0),
        "pulled back past the plane to 30° the other way, the panel reads {past_the_plane:?}: \
         not 30 written, the other way",
    );
    assert!(
        turns(past_a_whole_turn, true, 360.0),
        "pulled on the other way to 400°, the panel reads {past_a_whole_turn:?}: not held at a \
         whole turn, the other way",
    );
}

#[test]
fn cancelling_the_panel_after_a_drag_round_the_arc_gives_the_angle_back() {
    let mut document = a_part_with_a_rectangle_beside_the_v_axis();
    let mut extrusion = turning_the_rectangle(TWICE_THE_QUARTER);

    let taken = dragged_round(&mut extrusion, &document, &[90.0, 45.0, 0.5, -30.0]);
    let turned_back = extrusion.reversed;
    run_on(Command::ExtrusionCancel, &mut document, &mut extrusion);

    assert!(
        taken && turned_back,
        "the arc taken at 90° and pulled to 30° the other way did not turn the revolution round",
    );
    assert_eq!(
        (
            extrusion.angle_input.as_str(),
            extrusion.reversed,
            extrusion.panel(&document).is_some(),
        ),
        (TWICE_THE_QUARTER, false, true),
        "« Annuler » after a drag round the arc does not leave the panel open on the angle \
         typed before it, the way it went",
    );
}

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

fn on_the_screen(state: &ViewportState, at: DVec3) -> egui::Pos2 {
    let view_projection = state
        .camera
        .view_projection(SCREEN.width() / SCREEN.height());
    let clip = view_projection * at.as_vec3().extend(1.0);
    let ndc = clip.truncate() / clip.w;
    egui::pos2(
        SCREEN.center().x + ndc.x * SCREEN.width() * 0.5,
        SCREEN.center().y - ndc.y * SCREEN.height() * 0.5,
    )
}

/// The amber of the chrome as the scene carries it.
fn amber() -> [f32; 4] {
    let [r, g, b, a] = ACCENT_AMBER.to_normalized_gamma_f32();
    cao_render::srgb(r, g, b, a)
}

/// Where one frame of the square, its extrusion as `extrusion` says, draws
/// its lines amber over everything else.
fn amber_lines(mut extrusion: ExtrusionState) -> Vec<Vec3> {
    let mut document = a_part_with_a_square();
    let mut editor = SketchEditor::default();
    let lang = Catalogue::french();
    let state = ViewportState::default();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let scale = ViewScale::of(&state.camera, SCREEN, 1.0, &state.config, 1.0);
    let frame = build_frame(&state, SCREEN, SCREEN, scale, &context, None, None);
    frame
        .scene_lines
        .iter()
        .filter(|vertex| {
            vertex
                .color
                .iter()
                .zip(amber())
                .all(|(channel, wanted)| (channel - wanted).abs() <= 1e-3)
        })
        .map(|vertex| Vec3::from(vertex.position))
        .collect()
}

/// What one pass of egui paints of the pill over the square, its extrusion as
/// `extrusion` says, with no window: each text with its colour and place, each
/// border with its colour and the frame it runs round, and where the arrow's
/// tip stands on the screen.
#[allow(clippy::type_complexity)]
fn pill_painted(
    mut extrusion: ExtrusionState,
) -> (
    Vec<(String, egui::Color32, egui::Pos2)>,
    Vec<(egui::Color32, egui::Rect)>,
    egui::Pos2,
) {
    let mut document = a_part_with_a_square();
    let mut editor = SketchEditor::default();
    let lang = Catalogue::french();
    let state = ViewportState::default();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let egui = egui::Context::default();
    let mut output = egui::FullOutput::default();
    for _ in 0..2 {
        output.textures_delta.clear();
        egui.begin_pass(egui::RawInput {
            screen_rect: Some(SCREEN),
            ..Default::default()
        });
        egui::Area::new(egui::Id::new("the canvas"))
            .fixed_pos(SCREEN.min)
            .fade_in(false)
            .show(&egui, |ui| {
                paint_extrusion_pill(ui, &state, SCREEN, &context);
            });
        output = egui.end_pass();
    }
    output.textures_delta.clear();
    let mut pending: Vec<egui::Shape> = output
        .shapes
        .into_iter()
        .map(|clipped| clipped.shape)
        .collect();
    let (mut texts, mut borders) = (Vec::new(), Vec::new());
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(shapes) => pending.extend(shapes),
            egui::Shape::Text(text) => {
                let colour = text.override_text_color.unwrap_or_else(|| {
                    text.galley
                        .job
                        .sections
                        .first()
                        .map(|section| section.format.color)
                        .filter(|colour| *colour != egui::Color32::PLACEHOLDER)
                        .unwrap_or(text.fallback_color)
                });
                texts.push((text.galley.text().to_string(), colour, text.pos));
            }
            egui::Shape::Rect(frame) if frame.stroke.width > 0.0 => {
                borders.push((frame.stroke.color, frame.rect));
            }
            _ => {}
        }
    }
    (
        texts,
        borders,
        on_the_screen(&state, DVec3::new(0.0, 0.0, 10.0)),
    )
}

fn alike(one: egui::Color32, other: egui::Color32) -> bool {
    one.to_array()
        .iter()
        .zip(other.to_array())
        .all(|(a, b)| a.abs_diff(b) <= 2)
}

/// How far from the tip a pill still stands beside the arrow, in points.
const BESIDE: f32 = 48.0;

#[test]
fn the_arrow_and_its_pill_are_painted_amber() {
    let offered = || {
        let mut extrusion = ExtrusionState::default();
        extrusion.offer(0);
        extrusion
    };

    let arrow = amber_lines(extruding_the_square());
    let no_arrow = amber_lines(offered());
    let (texts, borders, tip) = pill_painted(extruding_the_square());
    let (no_texts, _, _) = pill_painted(offered());

    let drawn_at = |place: Vec3| arrow.iter().any(|at| at.distance(place) <= 1e-4);
    assert!(
        drawn_at(Vec3::ZERO) && drawn_at(Vec3::new(0.0, 0.0, 10.0)),
        "the frame of the square previewed 10 deep draws no amber line from the arrow's foot \
         to its tip, over the matter: amber at {arrow:?}",
    );
    assert!(
        no_arrow.is_empty(),
        "with nothing previewed, the frame still draws amber lines at {no_arrow:?}",
    );
    let pill = texts
        .iter()
        .find(|(said, _, _)| said == "10 mm")
        .unwrap_or_else(|| panic!("no pill reads « 10 mm »: the texts painted are {texts:?}"));
    assert!(
        alike(pill.1, ACCENT_AMBER_TEXT),
        "the pill's « 10 mm » is written in {:?}, not amber",
        pill.1,
    );
    assert!(
        borders
            .iter()
            .any(|(colour, frame)| alike(*colour, ACCENT_AMBER_BORDER)
                && frame.contains(pill.2)
                && frame.distance_to_pos(tip) <= BESIDE),
        "no amber border runs round the « 10 mm » beside the arrow's tip at {tip:?}: the \
         borders painted are {borders:?}",
    );
    assert!(
        no_texts.is_empty(),
        "with nothing previewed, a pill still reads {no_texts:?}",
    );
}

#[test]
fn a_ray_far_from_the_arrow_takes_no_hold_of_it() {
    let document = a_part_with_a_square();
    let mut extrusion = extruding_the_square();
    let mut handle = ExtrusionHandle::default();

    let over = handle.grab(&extrusion, &document, beside(11.0), SIDEWAYS, REACH);
    let aside = handle.grab(
        &extrusion,
        &document,
        DVec3::new(30.0, 4.0, 10.0),
        SIDEWAYS,
        REACH,
    );
    handle.pull(&mut extrusion, beside(15.0), SIDEWAYS);

    assert!(
        !over,
        "a ray passing a unit over the arrow's tip takes hold of it"
    );
    assert!(
        !aside,
        "a ray passing four units beside the arrow's tip takes hold of it"
    );
    assert!(
        both_read(field_and_preview(&extrusion, &document), 10.0),
        "with no hold taken, pulling moved the depth to {}",
        extrusion.distance_input,
    );
}

fn pressed(
    at: egui::Pos2,
    button: egui::PointerButton,
    modifiers: egui::Modifiers,
    pressed: bool,
) -> Vec<egui::Event> {
    vec![egui::Event::PointerButton {
        pos: at,
        button,
        pressed,
        modifiers,
    }]
}

/// The square extruded 10 deep, or only offered when not `previewed`, and a
/// drag of `button` with `modifiers` from where the arrow's tip is drawn,
/// thirty points up the screen. Each frame goes to the camera's navigation,
/// then to the arrow, in the order the frame loop hands them. What the depth
/// field reads after, whether the arrow took any frame of the gesture, and
/// whether the view turned.
fn the_arrow_dragged(
    button: egui::PointerButton,
    modifiers: egui::Modifiers,
    previewed: bool,
) -> (Option<f64>, bool, bool) {
    let mut document = a_part_with_a_square();
    let mut editor = SketchEditor::default();
    let mut extrusion = match previewed {
        true => extruding_the_square(),
        false => {
            let mut offered = ExtrusionState::default();
            offered.offer(0);
            offered
        }
    };
    let lang = Catalogue::french();
    let mut state = ViewportState::default();
    let at = on_the_screen(&state, DVec3::new(0.0, 0.0, 10.0));
    assert!(
        SCREEN.contains(at),
        "the arrow's tip is drawn off the screen, at {at:?}"
    );
    let up = |points: f32| at - egui::vec2(0.0, points);
    let aspect = SCREEN.width() / SCREEN.height();
    let before = state.camera.view_projection(aspect);

    let egui = egui::Context::default();
    let mut took = false;
    for events in [
        vec![],
        vec![
            egui::Event::ModifiersChanged(modifiers),
            egui::Event::PointerMoved(at),
        ],
        pressed(at, button, modifiers, true),
        vec![egui::Event::PointerMoved(up(15.0))],
        vec![egui::Event::PointerMoved(up(30.0))],
        pressed(up(30.0), button, modifiers, false),
    ] {
        egui.begin_pass(egui::RawInput {
            screen_rect: Some(SCREEN),
            events,
            ..Default::default()
        });
        egui::Area::new(egui::Id::new("the canvas"))
            .fixed_pos(SCREEN.min)
            .show(&egui, |ui| {
                ui.set_min_size(SCREEN.size());
                let response = ui.interact(
                    SCREEN,
                    egui::Id::new("the canvas's surface"),
                    egui::Sense::click_and_drag(),
                );
                let cube = cube_rect(SCREEN, &state.config);
                handle_navigation(ui, &mut state, &response, cube);
                let scale = ViewScale::of(&state.camera, SCREEN, 1.0, &state.config, 1.0);
                let mut context = SketchContext {
                    document: &mut document,
                    editor: &mut editor,
                    extrusion: &mut extrusion,
                    lang: &lang,
                };
                took |= point_the_extrusion_arrow(
                    ui,
                    &mut state,
                    &response,
                    (SCREEN, scale),
                    &mut context,
                );
            });
        egui.end_pass().textures_delta.clear();
    }
    let turned = !state
        .camera
        .view_projection(aspect)
        .abs_diff_eq(before, 1e-6);
    let field = document
        .variables()
        .size_of(&extrusion.distance_input)
        .ok()
        .map(|(_, value)| value);
    (field, took, turned)
}

#[test]
fn a_drag_on_the_arrow_pulls_the_depth_and_leaves_the_view_as_it_was() {
    let (field, took, turned) =
        the_arrow_dragged(egui::PointerButton::Primary, egui::Modifiers::NONE, true);

    assert!(
        took && field.is_some_and(|depth| (depth - 10.0).abs() > 0.1),
        "the arrow dragged thirty points up with the primary button left the depth at \
         {field:?}, the arrow taking the gesture: {took}",
    );
    assert!(!turned, "dragging the arrow turned the view as well");
}

#[test]
fn an_orbit_over_the_arrow_turns_the_view_and_leaves_the_depth() {
    for (said, button, modifiers) in [
        (
            "an Alt drag",
            egui::PointerButton::Primary,
            egui::Modifiers::ALT,
        ),
        (
            "a Shift drag of the middle button",
            egui::PointerButton::Middle,
            egui::Modifiers::SHIFT,
        ),
    ] {
        let (field, took, turned) = the_arrow_dragged(button, modifiers, true);

        assert!(
            !took && reads(field, 10.0),
            "{said} over the arrow, which orbits, moved the depth to {field:?}, the arrow \
             taking the gesture: {took}",
        );
        assert!(turned, "{said} over the arrow left the view where it was");
    }
}

#[test]
fn only_the_primary_button_pulls_the_arrow() {
    for (said, button) in [
        ("the middle button", egui::PointerButton::Middle),
        ("the secondary button", egui::PointerButton::Secondary),
    ] {
        let (field, took, _) = the_arrow_dragged(button, egui::Modifiers::NONE, true);

        assert!(
            !took && reads(field, 10.0),
            "a drag of {said} over the arrow moved the depth to {field:?}, the arrow taking \
             the gesture: {took}",
        );
    }
}

#[test]
fn the_arrow_takes_no_gesture_while_nothing_is_previewed() {
    let (field, took, _) =
        the_arrow_dragged(egui::PointerButton::Primary, egui::Modifiers::NONE, false);

    assert!(
        !took && reads(field, 10.0),
        "with the tool only offered, a drag where the arrow's tip would stand moved the depth \
         to {field:?}, the arrow taking the gesture: {took}",
    );
}
