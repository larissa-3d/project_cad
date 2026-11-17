//! What app · screens/viewport/extrusion_handle.rs is held to.
//!
//! Closes #741.
//! - On the preview, an amber arrow along the direction of extrusion, its value
//!   beside it in a pill — the arrow rising from the middle of the areas
//!   picked to the depth, and the other way along the plane once reversed —
//!   `the_arrow_rises_from_the_middle_of_the_areas_to_the_depth_typed`,
//!   `reversed_the_arrow_points_the_other_way_along_the_plane`; painted amber,
//!   its pill reading what the depth comes to —
//!   `the_arrow_is_painted_amber_and_its_pill_reads_what_the_depth_comes_to`
//! - Dragging it changes the depth, and the panel's field follows —
//!   `pulling_the_arrow_changes_the_depth_and_the_panel_s_field_follows`
//! - A depth written as a formula becomes a number once dragged; « Annuler » of
//!   the panel, or undo, gives the formula back —
//!   `a_depth_written_as_a_formula_is_a_number_once_dragged`,
//!   `cancelling_the_panel_after_a_drag_gives_the_formula_back`,
//!   `undo_after_a_drag_gives_the_formula_back`
//! - Dragging past the sketch's plane reverses the direction, as « Inverser le
//!   sens » does —
//!   `pulling_the_arrow_past_the_plane_reverses_the_direction_as_the_reverse_switch_does`
//! - For a revolution, the handle drags the angle along an arc —
//!   `for_a_revolution_the_arrow_stands_on_an_arc_about_the_axis_at_the_angle_typed`,
//!   `for_a_revolution_pulling_the_arrow_round_its_arc_changes_the_angle`
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
//!   `an_orbit_over_the_arrow_turns_the_view_and_leaves_the_depth`
//!
//! Snapping the drag to faces of the part is out of the issue's scope.
//!
//! Read when the work opened: the arrow stands on the sketch's plane at the
//! middle of the areas picked and runs along its normal, its tip at the depth
//! as signed by the direction; a pull sets the depth to where the pointer's
//! ray passes nearest that line, written in the panel's field as a plain
//! number, the direction turned round once the pointer passes the plane.

use cao_part::{ExtrusionMode, Formula, Operation, PartDocument, PointRef, VariableChange};
use cao_prefs::Command;
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::{DVec2, DVec3};

use super::*;
use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::blend::BlendState;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::ribbon::Ribbon;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;

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
