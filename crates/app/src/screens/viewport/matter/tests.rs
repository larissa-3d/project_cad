//! What app · screens/viewport/matter.rs is held to.
//!
//! Closes #526.
//! - the section view works on an exact body: what stands between the eye and
//!   the sketch plane is not drawn, and the rest is —
//!   `an_exact_body_is_drawn_behind_the_sketch_plane_only`
//!
//! Closes #319.
//! - only in the plane view: the matter between the camera and the sketch
//!   plane is treated only while the view sits on the sketch plane, reached by
//!   starting the sketch or by the re-align button —
//!   `the_matter_between_the_eye_and_the_sketch_plane_is_not_drawn`
//! - as soon as the camera is orbited out of it, the whole part is drawn again,
//!   as usual; no angle tolerance —
//!   `orbiting_out_of_the_plane_view_draws_the_whole_part_again`,
//!   `orbited_out_of_the_plane_view_nothing_is_drawn_see_through`
//! - the plane view is the view sitting on the sketch plane, not any face of
//!   the cube — `a_cube_face_with_no_sketch_open_cuts_nothing`,
//!   `a_view_sitting_on_another_plane_than_the_sketch_cuts_nothing`
//! - three ways to show that matter, each click moving to the next state —
//!   `each_click_moves_the_matter_in_front_to_the_next_state`
//! - Hidden: the matter in front of the plane is not drawn —
//!   `hidden_draws_nothing_in_front_of_the_plane`
//! - See-through: it is drawn translucent, and the sketch shows through it —
//!   `see_through_hands_the_matter_in_front_over_apart`; that it reaches the
//!   painter translucent is held under its own `Closes #319` in
//!   `crates/app/src/screens/viewport/render/tests.rs`
//! - what is drawn see-through is only what stands in front: a face lying on
//!   the sketch plane stays opaque, under the sketch's own fills — found in
//!   review, not in the issue —
//!   `see_through_carries_no_face_lying_on_the_sketch_plane`,
//!   `see_through_hands_over_nothing_when_nothing_stands_in_front`
//! - Opaque: it is drawn as usual — `opaque_draws_the_matter_in_front_as_usual`
//! - the button, next to the re-align button, is there only while a sketch is
//!   open; its icon shows the current state — no test: here; it is a ribbon
//!   command, held under its own `Closes #319` in
//!   `crates/app/src/screens/ribbon/state/tests.rs`, which says how the issue's
//!   "shown" and "icon" were read
//! - what lies behind the plane is always drawn as usual —
//!   `what_lies_behind_the_plane_is_drawn_in_every_state`
//! - every sketch opens on Hidden, and Hidden stays the default —
//!   `every_sketch_opens_on_hidden`
//! - the choice lasts as long as that sketch is open, and is not remembered for
//!   the next one — `the_state_chosen_is_not_carried_to_the_next_sketch`,
//!   `opening_another_sketch_without_closing_it_starts_on_hidden`
//! - closing the sketch draws the whole part again —
//!   `closing_the_sketch_puts_the_whole_body_back`
//! - clicks always reach the sketch plane, whatever the state, picking and
//!   snapping on it included — no test: here; a click is met with the plane and
//!   never with the body, and the input wants a window, so it is held in each
//!   state by the driver, in `crates/app/tests/drawing.rs` under its own
//!   `Closes #319`
//! - the part's picture in the explorer carries no cut — no test: `picture::of`
//!   reads `document.body()` and is handed neither the editor nor the view, so
//!   there is nothing it could cut with
//!
//! Closes #578.
//! - the matter in front of the plane is still hidden while a sketch is
//!   edited — `the_matter_between_the_eye_and_the_sketch_plane_is_not_drawn`
//! - leaving the view puts the part back whole —
//!   `closing_the_sketch_puts_the_whole_body_back`
//! - the cut is computed once each time the plane changes, never every frame
//!   — `the_section_is_cut_once_while_the_plane_stays_put`,
//!   `the_section_is_cut_again_when_the_plane_moves`; and again when the part
//!   or the side the eye is on changes —
//!   `the_section_is_cut_again_when_the_part_or_the_side_changes`; how the
//!   matter in front is shown cuts each half once, found in review: Hidden
//!   cuts what lies behind, See-through reuses it and cuts only what stands in
//!   front, and going round the three states cuts nothing more —
//!   `going_round_the_states_of_the_matter_in_front_cuts_each_half_once`
//! - the rest of #578 is the kernel's, under its own `Closes #578` in
//!   `crates/solid/tests/the_section_of_an_exact_part_is_filled.rs` — no test:
//!   here; this block holds only what the view decides
//!
//! Closes #970.
//! - With a section on and a sketch edited, turning the camera past the
//!   sketch's plane does not bring back the matter the section hides; a test
//!   on the presenter that decides what is drawn checks it —
//!   `turning_the_camera_while_a_sketch_is_edited_never_brings_back_the_matter_the_section_hides`
//! - Without a section, #319's behaviour is unchanged; the existing tests stay
//!   green — `orbiting_out_of_the_plane_view_draws_the_whole_part_again`,
//!   `orbited_out_of_the_plane_view_nothing_is_drawn_see_through`, and the rest
//!   of #319's block above, none of which turns a section on
//! - found in review: the section view open in its panel, and not only a kept
//!   section, keeps what it hides hidden as the camera turns —
//!   `turning_the_camera_while_a_sketch_is_edited_never_brings_back_the_matter_the_section_view_hides`
//! - found in review: a section and a face-on sketch cut each body once, not
//!   every frame — a section missing the part by
//!   `a_face_on_sketch_over_a_section_missing_the_part_cuts_it_once_not_every_frame`,
//!   and the part with eight volumes shown see-through beside it by
//!   `a_face_on_sketch_over_a_section_cuts_the_part_and_eight_see_through_volumes_once_not_every_frame`
//! - found in review: a sketch face-on over a section cuts what the section
//!   draws as it would cut the part — no test: here; what reaches the painter
//!   is read under its own `Closes #970` in
//!   `crates/app/src/screens/viewport/render/tests.rs`
//!
//! Closes #356 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests/reader.rs`; this bullet is
//! the half of a criterion the canvas draws.
//! - A click sets the reading point. Clicking a box makes the part computed up
//!   to that step included; the boxes after it are marked as not applied, and
//!   so are the arrows between them. Clicking the last box brings the part
//!   back whole — the part drawn up to the step read to, and whole again from
//!   the last box, never the part drawn for a reading point set before it,
//!   the part itself left whole —
//!   `a_part_read_to_a_step_is_drawn_up_to_it_and_whole_again_from_the_last`
//! - found in review: the reader's boxes and the part's own reading count the
//!   key steps apart, the one over the steps applied, the other over the
//!   design; tied together, a section between two of them and a step undone
//!   after them, each box read to draws what the part read through that box
//!   holds — `each_box_read_to_draws_the_part_read_through_that_box`
//! - found in review: the last box clicked, or any past it, reads the part as
//!   unread, the canvas drawing the part itself rather than a copy of it read
//!   through every step — `the_last_box_clicked_reads_the_part_as_unread`
//!
//! A step undone always stands after every step applied, in the list and in
//! the design alike: an undo moves the cursor back, and what is done next
//! drops what was undone. So the part's own reading could count undone steps
//! and still read every box applied to the same place; the boxes could not,
//! since one box more would name a step the part does not have. A section
//! between two key steps is what set the two counts apart while a section was
//! no box; since #968 a saved section is a box of its own, and a section
//! deleted is what can set them apart.
//!
//! Closes #968 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests/reader.rs`; this bullet is
//! the half the canvas draws.
//! - decided in the issue: deleting a section stays what #692 made it, and
//!   its step follows — a section deleted is no box, and each box after it
//!   draws the part read through its own step, the part's own reading
//!   counting the same boxes —
//!   `a_deleted_section_is_no_box_and_each_box_after_it_draws_the_part_read_through_it`

use std::collections::BTreeMap;

use cao_part::history::{SectionAsked, SideDrawing};
use cao_part::{ExtrusionMode, Operation, PointRef};
use cao_render::CubeFace;
use cao_render::camera::CubeZone;
use cao_sketch::WorkPlane;
use cao_solid::profile::{Contour, Frame, Profile};
use glam::{DVec2, DVec3, Vec2};

use super::*;
use crate::lang::Catalogue;
use crate::screens::part_tree::state::StepReader;
use crate::screens::viewport::{ViewMode, ViewportState};

const ABOVE: Vec3 = Vec3::new(0.0, 0.0, 10.0);

fn shown_body<'a>(
    editor: &SketchEditor,
    mode: ViewMode,
    body: &'a Body,
    eye: Vec3,
) -> Cow<'a, Body> {
    drawn(editor, mode, body, eye).opaque
}

fn see_through_body(editor: &SketchEditor, mode: ViewMode, body: &Body, eye: Vec3) -> Option<Body> {
    drawn(editor, mode, body, eye).see_through
}

/// The view a sketch on `plane` lands in when it is started.
fn sitting_on(plane: WorkPlane) -> ViewMode {
    let mut view = ViewportState::default();
    view.look_at_plane(plane, DVec3::ZERO, 5.0);
    view.mode()
}

/// A box straddling the XY plane, from one unit under it to one unit over,
/// raised by the exact kernel as the part raises a block rather than laid out
/// face by face.
fn a_body_through_the_plane() -> Body {
    let corners = [
        DVec2::new(-1.0, -1.0),
        DVec2::new(1.0, -1.0),
        DVec2::new(1.0, 1.0),
        DVec2::new(-1.0, 1.0),
    ];
    let triangles = [
        [corners[0], corners[1], corners[2]],
        [corners[0], corners[2], corners[3]],
    ];
    let profile = Profile {
        exact: Some((Contour::straight(corners.to_vec()), Vec::new())),
        sampled: cao_solid::Loop::straight(&corners),
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let frame = Frame {
        origin: DVec3::Z * -1.0,
        u: DVec3::X,
        v: DVec3::Y,
    };
    Body::default()
        .tool_raised(&profile, frame, DVec3::Z * 2.0)
        .expect("the exact kernel raises a lone profile")
}

fn reaches_above(body: &Body) -> bool {
    body.triangles().iter().flatten().any(|at| at.z > 1e-9)
}

fn reaches_below(body: &Body) -> bool {
    body.triangles().iter().flatten().any(|at| at.z < -1e-9)
}

fn reaches_the_far_end(body: &Body) -> bool {
    body.triangles()
        .iter()
        .flatten()
        .any(|at| (at.z + 1.0).abs() <= 1e-9)
}

/// How the matter in front of the plane is drawn, read from what the frame is
/// handed: the opaque matter, and the matter drawn see-through apart from it.
#[derive(Debug, PartialEq)]
enum Look {
    Hidden,
    SeeThrough,
    Opaque,
}

fn look_of(editor: &SketchEditor) -> Look {
    let body = a_body_through_the_plane();
    let mode = sitting_on(WorkPlane::XY);
    let opaque = shown_body(editor, mode, &body, ABOVE);
    let see_through = see_through_body(editor, mode, &body, ABOVE);
    match (reaches_above(&opaque), see_through) {
        (true, None) => Look::Opaque,
        (false, None) => Look::Hidden,
        (false, Some(_)) => Look::SeeThrough,
        (true, Some(_)) => panic!("the matter in front is drawn twice, opaque and see-through"),
    }
}

#[test]
fn nothing_is_cut_away_while_no_sketch_is_being_edited() {
    let editor = SketchEditor::default();
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, sitting_on(WorkPlane::XY), &body, ABOVE);

    assert_eq!(*shown, body);
}

#[test]
fn the_matter_between_the_eye_and_the_sketch_plane_is_not_drawn() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, sitting_on(WorkPlane::XY), &body, ABOVE);

    assert!(!reaches_above(&shown), "the near side is gone");
    assert!(
        shown
            .triangles()
            .iter()
            .flatten()
            .any(|at| (at.z + 1.0).abs() <= 1e-9),
        "and the far side is what one draws against"
    );
}

#[test]
fn closing_the_sketch_puts_the_whole_body_back() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor.next_matter_in_front();
    editor.close();
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, sitting_on(WorkPlane::XY), &body, ABOVE);

    assert_eq!(
        *shown, body,
        "the sketch is closed, and the part is still drawn cut open"
    );
    assert!(
        see_through_body(&editor, sitting_on(WorkPlane::XY), &body, ABOVE).is_none(),
        "a sketch closed see-through leaves the part drawn half see-through",
    );
}

#[test]
fn looking_from_under_the_plane_cuts_away_the_other_half() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, sitting_on(WorkPlane::XY), &body, -ABOVE);

    assert!(
        shown.triangles().iter().flatten().all(|at| at.z >= -1e-9),
        "what stands in front depends on where one looks from"
    );
}

/// A disc 10 across standing on its axis, from one unit under the XY plane to
/// one unit over, raised by the exact kernel.
fn a_disc_through_the_plane() -> Body {
    let points: Vec<DVec2> = (0..48)
        .map(|step| DVec2::from_angle(std::f64::consts::TAU * step as f64 / 48.0) * 5.0)
        .collect();
    let curves = vec![Some(0); points.len()];
    let triangles: Vec<[DVec2; 3]> = (1..points.len() - 1)
        .map(|index| [points[0], points[index], points[index + 1]])
        .collect();
    let profile = Profile {
        exact: Some((Contour::circle(DVec2::ZERO, 5.0), Vec::new())),
        sampled: cao_solid::Loop {
            points: &points,
            curves: &curves,
        },
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let frame = Frame {
        origin: DVec3::Z * -1.0,
        u: DVec3::X,
        v: DVec3::Y,
    };
    Body::default()
        .tool_raised(&profile, frame, DVec3::Z * 2.0)
        .expect("the exact kernel raises a lone profile")
}

#[test]
fn an_exact_body_is_drawn_behind_the_sketch_plane_only() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let body = a_disc_through_the_plane();

    let shown = shown_body(&editor, sitting_on(WorkPlane::XY), &body, ABOVE);
    let corners: Vec<DVec3> = shown.triangles().into_iter().flatten().collect();

    assert!(!corners.is_empty(), "the far half is drawn");
    assert!(!reaches_above(&shown), "the near half is gone");
    assert!(
        corners.iter().any(|at| (at.z + 1.0).abs() <= 1e-9),
        "the far end of the disc is drawn",
    );
    assert!(
        corners
            .iter()
            .any(|at| at.z.abs() <= 1e-9 && (at.truncate().length() - 5.0).abs() <= 1e-9),
        "the wall is drawn right up to the plane",
    );
    assert!(
        corners
            .iter()
            .all(|at| at.truncate().length() <= 5.0 + 1e-9),
        "nothing is drawn outside the true cylinder",
    );
}

#[test]
fn orbiting_out_of_the_plane_view_draws_the_whole_part_again() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let mut view = ViewportState::default();
    view.look_at_plane(WorkPlane::XY, DVec3::ZERO, 5.0);
    view.orbit(Vec2::new(1.0, 0.0), 0.01);
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, view.mode(), &body, ABOVE);

    assert_eq!(
        *shown, body,
        "orbited off the sketch plane, the part is still drawn opened, and reads as broken",
    );
}

#[test]
fn a_cube_face_with_no_sketch_open_cuts_nothing() {
    let editor = SketchEditor::default();
    let mut view = ViewportState::default();
    view.snap_to_zone(CubeZone::Face(CubeFace::PlusZ));
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, view.mode(), &body, ABOVE);

    assert_eq!(
        *shown, body,
        "a face of the cube lands on a plane with its grid, and no sketch is there to cut for",
    );
}

#[test]
fn a_view_sitting_on_another_plane_than_the_sketch_cuts_nothing() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let mut view = ViewportState::default();
    view.snap_to_zone(CubeZone::Face(CubeFace::MinusY));
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, view.mode(), &body, Vec3::new(0.0, -10.0, 1.0));

    assert_eq!(
        *shown, body,
        "seen from the front, the sketch plane is edge-on, and cutting along it halves the part",
    );
}

#[test]
fn orbited_out_of_the_plane_view_nothing_is_drawn_see_through() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor.next_matter_in_front();
    let mut view = ViewportState::default();
    view.look_at_plane(WorkPlane::XY, DVec3::ZERO, 5.0);
    view.orbit(Vec2::new(1.0, 0.0), 0.01);
    let body = a_body_through_the_plane();

    assert_eq!(*shown_body(&editor, view.mode(), &body, ABOVE), body);
    assert!(
        see_through_body(&editor, view.mode(), &body, ABOVE).is_none(),
        "orbited off the plane, the part is drawn as usual, not half see-through",
    );
}

#[test]
fn hidden_draws_nothing_in_front_of_the_plane() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let body = a_body_through_the_plane();
    let mode = sitting_on(WorkPlane::XY);

    let opaque = shown_body(&editor, mode, &body, ABOVE);
    let see_through = see_through_body(&editor, mode, &body, ABOVE);

    assert!(
        !reaches_above(&opaque),
        "the matter in front is drawn opaque"
    );
    assert!(
        see_through.is_none(),
        "hidden matter is still handed over to be drawn see-through",
    );
}

#[test]
fn see_through_hands_the_matter_in_front_over_apart() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor.next_matter_in_front();
    let body = a_body_through_the_plane();
    let mode = sitting_on(WorkPlane::XY);

    let opaque = shown_body(&editor, mode, &body, ABOVE);
    let see_through = see_through_body(&editor, mode, &body, ABOVE)
        .expect("see-through hands over no matter to draw see-through");

    assert!(
        !reaches_above(&opaque),
        "the matter in front is drawn opaque, and hides the sketch",
    );
    assert!(
        reaches_above(&see_through),
        "what is drawn see-through is not the matter in front",
    );
    assert!(
        !reaches_below(&see_through),
        "the matter behind the plane is drawn see-through too",
    );
}

#[test]
fn opaque_draws_the_matter_in_front_as_usual() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor.next_matter_in_front();
    editor.next_matter_in_front();
    let body = a_body_through_the_plane();
    let mode = sitting_on(WorkPlane::XY);

    assert_eq!(*shown_body(&editor, mode, &body, ABOVE), body);
    assert!(see_through_body(&editor, mode, &body, ABOVE).is_none());
}

#[test]
fn each_click_moves_the_matter_in_front_to_the_next_state() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);

    let mut looks = vec![look_of(&editor)];
    for _ in 0..3 {
        editor.next_matter_in_front();
        looks.push(look_of(&editor));
    }

    assert_eq!(
        looks,
        [Look::Hidden, Look::SeeThrough, Look::Opaque, Look::Hidden],
    );
}

#[test]
fn what_lies_behind_the_plane_is_drawn_in_every_state() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let body = a_body_through_the_plane();
    let mode = sitting_on(WorkPlane::XY);

    for click in 0..3 {
        let opaque = shown_body(&editor, mode, &body, ABOVE);
        assert!(
            reaches_the_far_end(&opaque),
            "after {click} clicks, the matter behind the plane is not drawn opaque",
        );
        editor.next_matter_in_front();
    }
}

#[test]
fn every_sketch_opens_on_hidden() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);

    assert_eq!(look_of(&editor), Look::Hidden);
}

#[test]
fn the_state_chosen_is_not_carried_to_the_next_sketch() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor.next_matter_in_front();
    editor.next_matter_in_front();
    editor.close();

    editor.begin_editing(1, WorkPlane::XY);

    assert_eq!(look_of(&editor), Look::Hidden);
}

#[test]
fn opening_another_sketch_without_closing_it_starts_on_hidden() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor.next_matter_in_front();

    editor.begin_editing(1, WorkPlane::XY);

    assert_eq!(
        look_of(&editor),
        Look::Hidden,
        "a sketch opened from the tree while another was open inherits its choice",
    );
}

fn lies_on(body: &Body, height: f64) -> bool {
    body.triangles()
        .iter()
        .any(|corners| corners.iter().all(|at| (at.z - height).abs() <= 1e-9))
}

fn see_through_on(height: f64) -> Option<Body> {
    let plane = WorkPlane {
        origin: DVec3::Z * height,
        ..WorkPlane::XY
    };
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, plane);
    editor.next_matter_in_front();
    see_through_body(
        &editor,
        sitting_on(plane),
        &a_body_through_the_plane(),
        ABOVE,
    )
}

#[test]
fn see_through_carries_no_face_lying_on_the_sketch_plane() {
    let through_the_middle = see_through_on(0.0).expect("the upper half stands in front");
    let on_the_base = see_through_on(-1.0).expect("the whole block stands in front");

    assert!(
        !lies_on(&through_the_middle, 0.0),
        "the cut across the block is drawn see-through over the sketch drawn on it",
    );
    assert!(
        !lies_on(&on_the_base, -1.0),
        "the face the sketch is laid on is drawn see-through over the sketch's own fills",
    );
}

#[test]
fn see_through_hands_over_nothing_when_nothing_stands_in_front() {
    let on_the_top = see_through_on(1.0);

    assert!(
        on_the_top.is_none_or(|body| body.triangles().is_empty()),
        "a sketch on the top face has nothing in front of it, and the top face \
         is drawn see-through over it",
    );
}

#[test]
fn the_section_is_cut_once_while_the_plane_stays_put() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let body = a_disc_through_the_plane();
    let mode = sitting_on(WorkPlane::XY);
    let sections = Sections::default();

    let first = sections
        .drawn(&editor, mode, &body, ABOVE)
        .opaque
        .into_owned();
    let second = sections
        .drawn(&editor, mode, &body, ABOVE)
        .opaque
        .into_owned();

    assert_eq!(
        (sections.behind_cuts(), sections.front_cuts()),
        (1, 0),
        "a second frame with nothing changed cut the part again"
    );
    assert!(!reaches_above(&first), "the near half is gone");
    assert!(reaches_the_far_end(&first), "the far half is drawn");
    assert_eq!(first, second, "the second frame draws another section");
}

#[test]
fn the_section_is_cut_again_when_the_plane_moves() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let body = a_disc_through_the_plane();
    let mode = sitting_on(WorkPlane::XY);
    let sections = Sections::default();
    sections.drawn(&editor, mode, &body, ABOVE);

    editor.plane = Some(WorkPlane::from_normal(DVec3::Z * 0.5, DVec3::Z));
    let moved = sections
        .drawn(&editor, mode, &body, ABOVE)
        .opaque
        .into_owned();

    assert_eq!(
        sections.behind_cuts(),
        2,
        "the plane moved and the part was not cut again"
    );
    let heights: Vec<f64> = moved
        .triangles()
        .into_iter()
        .flatten()
        .map(|at| at.z)
        .collect();
    assert!(
        heights.iter().all(|&z| z <= 0.5 + 1e-9),
        "something stands in front of the plane where it now is"
    );
    assert!(
        heights.iter().any(|&z| z > 0.25),
        "what lies between where the plane was and where it is is still cut away"
    );
}

#[test]
fn the_section_is_cut_again_when_the_part_or_the_side_changes() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let mode = sitting_on(WorkPlane::XY);
    let sections = Sections::default();
    let mut body = a_disc_through_the_plane();
    sections.drawn(&editor, mode, &body, ABOVE);

    body = a_body_through_the_plane();
    let other_part = sections
        .drawn(&editor, mode, &body, ABOVE)
        .opaque
        .into_owned();
    assert_eq!(
        sections.behind_cuts(),
        2,
        "the part changed and was not cut again"
    );
    assert!(
        other_part
            .triangles()
            .iter()
            .flatten()
            .all(|at| at.x.abs() <= 1.0 + 1e-9),
        "the section drawn is still the disc's"
    );

    let other_side = sections
        .drawn(&editor, mode, &body, -ABOVE)
        .opaque
        .into_owned();
    assert_eq!(
        sections.behind_cuts(),
        3,
        "the eye crossed the plane and the part was not cut again"
    );
    assert!(
        other_side
            .triangles()
            .iter()
            .flatten()
            .all(|at| at.z >= -1e-9),
        "the half kept is still the one the eye stood in front of"
    );
}

#[test]
fn going_round_the_states_of_the_matter_in_front_cuts_each_half_once() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let mode = sitting_on(WorkPlane::XY);
    let body = a_body_through_the_plane();
    let sections = Sections::default();
    let cuts = |sections: &Sections| (sections.behind_cuts(), sections.front_cuts());

    let hidden = sections.drawn(&editor, mode, &body, ABOVE);
    assert_eq!(
        cuts(&sections),
        (1, 0),
        "Hidden cut more than what lies behind"
    );
    assert!(
        !reaches_above(&hidden.opaque),
        "Hidden draws the matter in front"
    );
    assert!(hidden.see_through.is_none());
    let behind = hidden.opaque.into_owned();

    editor.next_matter_in_front();
    let see_through = sections.drawn(&editor, mode, &body, ABOVE);
    assert_eq!(
        cuts(&sections),
        (1, 1),
        "See-through cut again what Hidden had cut behind the plane"
    );
    assert_eq!(
        *see_through.opaque, behind,
        "See-through draws another half behind the plane than Hidden"
    );
    let in_front = see_through
        .see_through
        .expect("See-through hands over no matter in front");
    assert!(reaches_above(&in_front) && !reaches_below(&in_front));

    editor.next_matter_in_front();
    let opaque = sections.drawn(&editor, mode, &body, ABOVE);
    assert_eq!(cuts(&sections), (1, 1), "Opaque cut the part");
    assert_eq!(*opaque.opaque, body, "Opaque does not draw the part whole");
    assert!(opaque.see_through.is_none());

    editor.next_matter_in_front();
    let hidden_again = sections.drawn(&editor, mode, &body, ABOVE);
    assert_eq!(
        cuts(&sections),
        (1, 1),
        "Hidden again cut what it had cut before Opaque"
    );
    assert_eq!(*hidden_again.opaque, behind);
    assert!(hidden_again.see_through.is_none());

    editor.next_matter_in_front();
    let see_through_again = sections.drawn(&editor, mode, &body, ABOVE);
    assert_eq!(
        cuts(&sections),
        (1, 1),
        "See-through again cut what it had cut before"
    );
    assert_eq!(*see_through_again.opaque, behind);
    assert_eq!(see_through_again.see_through, Some(in_front));
}

/// A square of side 10 drawn on XY and raised 10 into a block, then a second
/// sketch on XY and a square of side 10 at x = 40 raised 5 into a slab: four
/// key steps, the first sketch, the block, the second sketch, the slab.
fn a_block_and_a_slab_beside_it() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    for (sketch, corner, distance) in [(0, DVec2::ZERO, 10.0), (1, DVec2::new(40.0, 0.0), 5.0)] {
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        document.apply(Operation::AddRectangle {
            sketch,
            corner: PointRef::New(corner),
            opposite: PointRef::New(corner + DVec2::splat(10.0)),
            construction: false,
        });
        document.apply(Operation::Extrude {
            sketch,
            areas: document.areas_at(sketch, &[corner + DVec2::splat(5.0)]),
            distance: distance.into(),
            mode: ExtrusionMode::Add,
        });
    }
    document
}

/// How much matter a frame draws of `document`, every piece of it; nothing
/// when it draws nothing.
fn matter_drawn(viewport: &ViewportState, document: &PartDocument) -> Option<f64> {
    seen(viewport, &SketchEditor::default(), document)
        .map(|drawn| drawn.pieces().map(Body::volume).sum())
}

#[test]
fn a_part_read_to_a_step_is_drawn_up_to_it_and_whole_again_from_the_last() {
    const VOLUME: f64 = 1e-6;
    let document = a_block_and_a_slab_beside_it();
    let mut viewport = ViewportState::default();
    let mut read = Vec::new();
    let mut drawn_and_kept = |viewport: &ViewportState| {
        read.push(matter_drawn(viewport, &document));
        let whole = document.body().volume();
        assert!(
            (whole - 1500.0).abs() <= VOLUME * 1500.0,
            "the part itself holds {whole} once read, not the block and the slab's 1500",
        );
    };

    drawn_and_kept(&viewport);
    for key_step in [1, 0, 3, 2] {
        viewport.read_to(key_step, 4);
        drawn_and_kept(&viewport);
    }

    let expected = [Some(1500.0), Some(1000.0), None, Some(1500.0), Some(1000.0)];
    let matches = read.len() == expected.len()
        && read.iter().zip(expected).all(|pair| match pair {
            (Some(drawn), Some(wanted)) => (drawn - wanted).abs() <= VOLUME * wanted,
            (None, None) => true,
            _ => false,
        });
    assert!(
        matches,
        "unread, then read to the block, to the first sketch, to the slab and to the second \
         sketch, a frame draws {read:?}, not {expected:?}: the whole part, the block alone, \
         nothing, the whole part again and the block alone",
    );
}

/// A square of side 10 drawn on XY at `corner` as `sketch`, raised `distance`.
fn a_square_raised(document: &mut PartDocument, sketch: usize, corner: DVec2, distance: f64) {
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch,
        corner: PointRef::New(corner),
        opposite: PointRef::New(corner + DVec2::splat(10.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch,
        areas: document.areas_at(sketch, &[corner + DVec2::splat(5.0)]),
        distance: distance.into(),
        mode: ExtrusionMode::Add,
    });
}

/// « Enregistrer la coupe » on a section across YZ, 5 from it.
fn a_section_across_yz() -> Operation {
    Operation::SaveSection(SectionAsked {
        plane: WorkPlane::YZ,
        on: None,
        offset: 5.0,
        behind: SideDrawing::SeeThrough,
        in_front: SideDrawing::Opaque,
        tilt_about_u_degrees: 0.0,
        tilt_about_v_degrees: 0.0,
        pivot: DVec3::ZERO,
    })
}

/// A square of side 10 raised 10 into a block, a section kept across it, a
/// square of side 10 at x = 40 raised 5 into a slab, then a third square at
/// x = 80 raised 3 and that raise undone: six boxes, the section one of them
/// and the undone raise none.
fn a_part_with_a_section_between_its_steps_and_a_step_undone() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    a_square_raised(&mut document, 0, DVec2::ZERO, 10.0);
    document.apply(a_section_across_yz());
    a_square_raised(&mut document, 1, DVec2::new(40.0, 0.0), 5.0);
    a_square_raised(&mut document, 2, DVec2::new(80.0, 0.0), 3.0);
    document.undo();
    document
}

/// The name of each of the reader's boxes, in the row's order.
fn the_boxes(document: &PartDocument) -> Vec<String> {
    StepReader::of(document, &Catalogue::french())
        .boxes
        .into_iter()
        .map(|step| step.name)
        .collect()
}

/// Each box, by its rank, read to whose matter is not what the part read
/// through that box holds, drawn or read: what each holds is `expected`, by
/// rank, nothing for no matter.
fn boxes_drawn_apart(
    document: &PartDocument,
    boxes: &[String],
    expected: &[Option<f64>],
) -> Vec<String> {
    const VOLUME: f64 = 1e-6;
    let same = |one: Option<f64>, other: Option<f64>| match (one, other) {
        (Some(one), Some(other)) => (one - other).abs() <= VOLUME * other.max(1.0),
        (None, None) => true,
        _ => false,
    };
    let mut apart = Vec::new();
    for (rank, holds) in expected.iter().copied().enumerate() {
        let mut viewport = ViewportState::default();
        viewport.read_to(rank, boxes.len());
        let drawn = matter_drawn(&viewport, document);
        let read = document.shown_through(&BTreeMap::new(), rank).body;
        let read = (!read.is_empty()).then(|| read.volume());
        if !(same(drawn, holds) && same(read, holds)) {
            apart.push(format!(
                "« {} »: drawn {drawn:?}, the part read through it {read:?}, not {holds:?}",
                boxes.get(rank).map_or("?", String::as_str),
            ));
        }
    }
    apart
}

#[test]
fn each_box_read_to_draws_the_part_read_through_that_box() {
    let document = a_part_with_a_section_between_its_steps_and_a_step_undone();
    let boxes = the_boxes(&document);
    assert_eq!(
        boxes,
        [
            "Esquisse 1",
            "Extrusion 1",
            "Coupe 1",
            "Esquisse 2",
            "Extrusion 2",
            "Esquisse 3"
        ],
        "the reader does not list the six steps applied, the section among them and the \
         raise undone left out",
    );

    let apart = boxes_drawn_apart(
        &document,
        &boxes,
        &[
            None,
            Some(1000.0),
            Some(1000.0),
            Some(1000.0),
            Some(1500.0),
            Some(1500.0),
        ],
    );
    assert!(
        apart.is_empty(),
        "a box read to does not draw what the part read through that box holds:\n{}",
        apart.join("\n"),
    );
}

/// A square of side 10 raised 10 into a block, a section kept across it, a
/// square of side 10 at x = 40 raised 5 into a slab, a third sketch left
/// empty, then the section deleted: five boxes, the section deleted none of
/// them.
fn a_part_whose_section_between_its_steps_was_deleted() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    a_square_raised(&mut document, 0, DVec2::ZERO, 10.0);
    document.apply(a_section_across_yz());
    a_square_raised(&mut document, 1, DVec2::new(40.0, 0.0), 5.0);
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let section = document.sections()[0].id;
    document.apply(Operation::DeleteSection { section });
    document
}

#[test]
fn a_deleted_section_is_no_box_and_each_box_after_it_draws_the_part_read_through_it() {
    let document = a_part_whose_section_between_its_steps_was_deleted();
    let boxes = the_boxes(&document);
    assert_eq!(
        boxes,
        [
            "Esquisse 1",
            "Extrusion 1",
            "Esquisse 2",
            "Extrusion 2",
            "Esquisse 3"
        ],
        "the reader does not list the five steps left once the section between them was \
         deleted",
    );

    let apart = boxes_drawn_apart(
        &document,
        &boxes,
        &[None, Some(1000.0), Some(1000.0), Some(1500.0), Some(1500.0)],
    );
    assert!(
        apart.is_empty(),
        "once the section between its steps is deleted, a box read to does not draw what \
         the part read through that box holds:\n{}",
        apart.join("\n"),
    );
}

#[test]
fn the_last_box_clicked_reads_the_part_as_unread() {
    let document = a_block_and_a_slab_beside_it();
    let mut viewport = ViewportState::default();
    viewport.read_to(1, 4);
    assert!(
        viewport.part_shown(&document).is_some(),
        "the fixture wants the part read to its block drawn apart from the part itself",
    );

    let mut copied = Vec::new();
    for clicked in [3, 9] {
        viewport.read_to(1, 4);
        viewport.read_to(clicked, 4);
        if viewport.part_shown(&document).is_some() {
            copied.push(clicked);
        }
    }
    assert!(
        copied.is_empty(),
        "read to the box of rank {copied:?} of four, the last or past it, the canvas draws a \
         copy of the part read through every step rather than the part itself, as it does \
         unread",
    );
}

const HALF_HEIGHT: f64 = 5.0;

/// A square of side 10 drawn on XY at `corner` and raised 10 into a block, as
/// `sketch`: three steps of the history.
fn a_square_raised_into_a_block(document: &mut PartDocument, sketch: usize, corner: DVec2) {
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch,
        corner: PointRef::New(corner),
        opposite: PointRef::New(corner + DVec2::splat(10.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch,
        areas: document.areas_at(sketch, &[corner + DVec2::splat(5.0)]),
        distance: 10.0_f64.into(),
        mode: ExtrusionMode::Add,
    });
}

/// The plane parallel to XY at `height`.
fn across_at(height: f64) -> WorkPlane {
    WorkPlane {
        origin: DVec3::Z * height,
        ..WorkPlane::XY
    }
}

/// A block 10 across raised 10 from XY, a section kept across it at half its
/// height that draws what lies under its cut opaque and hides what stands over
/// it, then a second sketch laid on that cut.
fn a_block_and_a_sketch_on_the_cut_of_its_kept_section() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    a_square_raised_into_a_block(&mut document, 0, DVec2::ZERO);
    document.apply(Operation::SaveSection(SectionAsked {
        plane: WorkPlane::XY,
        on: None,
        offset: HALF_HEIGHT,
        behind: SideDrawing::Opaque,
        in_front: SideDrawing::Hidden,
        tilt_about_u_degrees: 0.0,
        tilt_about_v_degrees: 0.0,
        pivot: DVec3::ZERO,
    }));
    document.apply(Operation::CreateSketchOnSection {
        section: 0,
        plane: across_at(HALF_HEIGHT),
    });
    document
}

/// A block 10 across raised 10 from XY, then a second sketch across it at
/// `height`.
fn a_block_and_a_sketch_across_it_at(height: f64) -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    a_square_raised_into_a_block(&mut document, 0, DVec2::ZERO);
    document.apply(Operation::CreateSketch {
        plane: across_at(height),
        on: None,
    });
    document
}

/// The view as starting a sketch on `plane` leaves it once its turn is over:
/// sitting on the plane, the eye on the side its normal points to.
fn face_on(viewport: &mut ViewportState, plane: WorkPlane) {
    viewport.look_at_plane(plane, DVec3::new(5.0, 5.0, HALF_HEIGHT), 10.0);
    if let Some(mut turn) = viewport.transition.take() {
        while turn.advance(&mut viewport.camera, 1.0) {}
    }
}

/// What goes wrong as the camera turns down from face-on on `plane`, 0.4 rad
/// at a time until it looks up at the block from under it: matter drawn over
/// `HALF_HEIGHT`, which the section hides, or the base of the block no longer
/// drawn opaque.
fn turning_down_from_face_on(
    viewport: &mut ViewportState,
    editor: &SketchEditor,
    document: &PartDocument,
    plane: WorkPlane,
) -> Vec<String> {
    face_on(viewport, plane);
    let mut wrong = Vec::new();
    for step in 0..=8 {
        if step > 0 {
            viewport.orbit(Vec2::new(0.0, -0.4), 1.0);
        }
        let height = f64::from(viewport.camera.eye().z);
        let side = if height > HALF_HEIGHT {
            "over"
        } else {
            "under"
        };
        let drawn = seen(viewport, editor, document);
        let highest = drawn
            .iter()
            .flat_map(Drawn::pieces)
            .flat_map(Body::triangles)
            .flatten()
            .map(|at| at.z)
            .fold(f64::NEG_INFINITY, f64::max);
        if highest > HALF_HEIGHT + 1e-9 {
            wrong.push(format!(
                "turned down {step} times, the eye {side} the cut at z = {height:.1}: matter is \
                 drawn up to z = {highest:.1}",
            ));
        }
        if !drawn.is_some_and(|drawn| lies_on(&drawn.opaque, 0.0)) {
            wrong.push(format!(
                "turned down {step} times, the eye {side} the cut at z = {height:.1}: the base \
                 of the block, under the cut, is not drawn opaque",
            ));
        }
    }
    wrong
}

#[test]
fn turning_the_camera_while_a_sketch_is_edited_never_brings_back_the_matter_the_section_hides() {
    let document = a_block_and_a_sketch_on_the_cut_of_its_kept_section();
    let on_the_cut = document.sketches()[1].plane;
    let mut editor = SketchEditor::default();
    editor.begin_editing(1, on_the_cut);
    let mut viewport = ViewportState::default();
    viewport
        .kept_sections
        .keep_up_with(&document, editor.active_sketch());
    assert!(
        viewport.section_shown().is_some(),
        "the fixture wants the sketch laid on the cut to switch its section on",
    );

    let wrong = turning_down_from_face_on(&mut viewport, &editor, &document, on_the_cut);

    assert!(
        wrong.is_empty(),
        "the section hides what stands over z = {HALF_HEIGHT}; the camera turned down from \
         face-on on the sketch laid on its cut, 0.4 rad at a time, past the sketch's plane:\n{}",
        wrong.join("\n"),
    );
}

#[test]
fn turning_the_camera_while_a_sketch_is_edited_never_brings_back_the_matter_the_section_view_hides()
{
    let document = a_block_and_a_sketch_across_it_at(2.0);
    let across = document.sketches()[1].plane;
    let mut viewport = ViewportState::default();
    viewport.toggle_section(document.body());
    let mut editor = SketchEditor::default();
    editor.begin_editing(1, across);

    let wrong = turning_down_from_face_on(&mut viewport, &editor, &document, across);

    assert!(
        wrong.is_empty(),
        "the section view open through the middle of the block hides what stands over \
         z = {HALF_HEIGHT}; the camera turned down from face-on on a sketch at z = 2, 0.4 rad \
         at a time, past the sketch's plane:\n{}",
        wrong.join("\n"),
    );
}

fn cuts(viewport: &ViewportState) -> usize {
    viewport.sections.behind_cuts() + viewport.sections.front_cuts()
}

#[test]
fn a_face_on_sketch_over_a_section_missing_the_part_cuts_it_once_not_every_frame() {
    let document = a_block_and_a_sketch_across_it_at(HALF_HEIGHT);
    let across = document.sketches()[1].plane;
    let mut viewport = ViewportState::default();
    let mut over_the_part = Section::new(WorkPlane::XY);
    over_the_part.offset = 20.0;
    viewport.section = Some(over_the_part);
    let mut editor = SketchEditor::default();
    editor.begin_editing(1, across);
    face_on(&mut viewport, across);

    let first = seen(&viewport, &editor, &document).expect("the block is drawn");
    assert!(
        !first.pieces().any(reaches_over_half_height) && lies_on(&first.opaque, 0.0),
        "the fixture wants the sketch's own cut drawn, the block under it opaque",
    );
    let after_the_first_frame = cuts(&viewport);
    for _ in 0..3 {
        seen(&viewport, &editor, &document);
    }

    assert_eq!(
        cuts(&viewport),
        after_the_first_frame,
        "a section over the block and a sketch face-on across it: three frames more with \
         nothing changed cut the block again",
    );
}

fn reaches_over_half_height(body: &Body) -> bool {
    body.triangles()
        .iter()
        .flatten()
        .any(|at| at.z > HALF_HEIGHT + 1e-9)
}

/// The history steps one block takes: its sketch, its square, its raise.
const STEPS_PER_BLOCK: usize = 3;

/// Nine blocks 10 across raised 10 from XY side by side, 20 apart along X,
/// then a sketch across them all at `height`.
fn nine_blocks_and_a_sketch_across_them_at(height: f64) -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    for block in 0..9 {
        a_square_raised_into_a_block(&mut document, block, DVec2::X * 20.0 * block as f64);
    }
    document.apply(Operation::CreateSketch {
        plane: across_at(height),
        on: None,
    });
    document
}

#[test]
fn a_face_on_sketch_over_a_section_cuts_the_part_and_eight_see_through_volumes_once_not_every_frame()
 {
    let document = nine_blocks_and_a_sketch_across_them_at(2.0);
    let across = document.sketches()[9].plane;
    let mut viewport = ViewportState::default();
    for block in 1..9 {
        viewport.next_volume_mode(&document, STEPS_PER_BLOCK * block + 2);
    }
    assert_eq!(
        viewport.part_shown(&document).map(|part| part.ghosts.len()),
        Some(8),
        "the fixture wants eight volumes shown see-through beside the part",
    );
    let mut through_the_middle = Section::new(WorkPlane::XY);
    through_the_middle.offset = HALF_HEIGHT;
    viewport.section = Some(through_the_middle);
    let mut editor = SketchEditor::default();
    editor.begin_editing(9, across);
    face_on(&mut viewport, across);

    seen(&viewport, &editor, &document);
    let after_the_first_frame = cuts(&viewport);
    seen(&viewport, &editor, &document);

    assert!(
        after_the_first_frame > 0,
        "the fixture wants the section and the sketch to cut something",
    );
    assert_eq!(
        cuts(&viewport),
        after_the_first_frame,
        "the part and eight see-through volumes under a section and a face-on sketch: the \
         second frame, with nothing changed, cut again",
    );
}
