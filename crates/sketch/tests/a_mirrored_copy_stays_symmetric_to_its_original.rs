//! A mirror lays a copy that stays the mirror image of what it was made from.
//!
//! Closes #320.
//! - Each copied point is held symmetric to its original about the axis.
//!   Dragging or re-dimensioning the original moves the copy; moving the axis
//!   moves the copy — `dragging_the_original_moves_its_copy_the_mirrored_way`,
//!   `a_length_typed_on_the_original_moves_its_copy`,
//!   `moving_the_axis_moves_the_copy`
//! - The link works both ways. Dragging a point of the copy moves the original
//!   the mirrored way; a dimension put on the copy drives the original too —
//!   `dragging_the_copy_moves_the_original_the_mirrored_way`,
//!   `a_length_typed_on_the_copy_drives_the_original`
//! - A point shared by the original and its copy — one lying on the axis — is
//!   symmetric to itself and stays on the axis: it slides along it and cannot
//!   leave it. A half-profile closed by a mirror stays closed —
//!   `a_point_on_the_axis_is_shared_by_the_original_and_its_copy`,
//!   `a_point_shared_across_the_axis_slides_along_it_and_cannot_leave_it`,
//!   `a_half_profile_closed_by_a_mirror_stays_closed_when_dragged`
//! - The copy carries the rules internal to the original — horizontal,
//!   parallel, tangent, … — when every element the rule speaks of was copied,
//!   so a mirrored rectangle stays a rectangle. A rule from the original to
//!   something that was not copied is not carried over. Dimensions are not
//!   duplicated where the symmetry already holds the value —
//!   `a_mirrored_rectangle_stays_a_rectangle`,
//!   `a_rule_to_something_left_out_of_the_copy_is_not_carried_over`,
//!   `a_length_the_symmetry_already_holds_is_not_typed_again_on_the_copy`
//! - Erasing a trait erases its twin, from either side; while one of them is
//!   selected, its twin is highlighted with it —
//!   `erasing_a_trait_of_the_original_erases_its_twin`,
//!   `erasing_a_trait_of_the_copy_erases_its_twin`; the highlight is drawn by
//!   `cao_app`, and the app test that draws it carries a `Closes #320` block of
//!   its own for that half
//! - Erasing the axis frees the copy. Both drawings stay where they are; the
//!   symmetry goes with the axis. The carried-over rules stay —
//!   `erasing_the_axis_frees_the_copy_where_it_stands`,
//!   `the_rules_carried_over_stay_when_the_axis_goes`
//! - The patterns (#306, #307) keep laying bare copies —
//!   `a_pattern_still_lays_copies_bare_of_every_rule`
//! - What would land on its original is still left out of the copy — no test:
//!   held unchanged by `a_point_sitting_on_the_axis_is_not_copied_onto_itself`,
//!   `the_trait_serving_as_the_axis_is_not_copied_onto_itself`,
//!   `a_circle_centred_on_the_axis_is_not_copied_onto_itself` and
//!   `an_arc_symmetric_about_the_axis_is_not_copied_onto_itself`, in
//!   `crates/sketch/src/mirroring/tests.rs`
//! - A mirrored arc comes back as an arc (#317) —
//!   `a_mirrored_arc_still_sweeps_as_its_original_once_the_drawing_settles`
//! - A mirror is still one step in the history, undone in one go — no test:
//!   the history is `cao_part`'s, out of this crate's reach; answered under a
//!   `Closes #320` block of its own in `crates/part/tests/mirroring_a_selection.rs`
//! - Compacting the history (#191) carries the symmetry along — no test: same,
//!   answered under that block in `crates/part/tests/mirroring_a_selection.rs`
//! - The mirror's panel (#321) — no test: held unchanged by
//!   `enter_on_validate_lays_the_mirror_as_one_step` and
//!   `an_arc_mirrored_from_the_panel_comes_back_an_arc`, in
//!   `crates/app/src/screens/viewport/input/copying/tests/panel.rs`

use cao_sketch::{
    ChosenAxis, DimensionTarget, Duplicated, Element, LengthOutcome, PointId, SegmentId, Sketch,
    SketchAxis, WorkPlane,
};
use glam::DVec2;

const AFTER_THE_SOLVER: f64 = 1e-6;

struct Triangle {
    sketch: Sketch,
    corners: [PointId; 3],
    sides: [SegmentId; 3],
}

fn a_triangle_east_of_the_v_axis() -> Triangle {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        sketch.add_point(DVec2::new(2.0, 1.0)),
        sketch.add_point(DVec2::new(6.0, 1.0)),
        sketch.add_point(DVec2::new(2.0, 4.0)),
    ];
    let sides = [
        sketch.add_segment(corners[0], corners[1]),
        sketch.add_segment(corners[1], corners[2]),
        sketch.add_segment(corners[2], corners[0]),
    ];
    Triangle {
        sketch,
        corners,
        sides,
    }
}

fn the_sides(triangle: &Triangle) -> Vec<Element> {
    triangle.sides.map(Element::Segment).to_vec()
}

fn mirrored_about(place: DVec2, through: DVec2, towards: DVec2) -> DVec2 {
    let along = (towards - through).normalize();
    let offset = place - through;
    through + along * 2.0 * offset.dot(along) - offset
}

/// Each original corner with the copy laid at its mirror image, read off the
/// places the moment the mirror is laid.
fn twins(
    sketch: &Sketch,
    originals: &[PointId],
    made: &Duplicated,
    through: DVec2,
    towards: DVec2,
) -> Vec<(PointId, PointId)> {
    originals
        .iter()
        .map(|original| {
            let image = mirrored_about(sketch.point(*original), through, towards);
            let copy = made
                .points
                .iter()
                .copied()
                .find(|copy| sketch.point(*copy).distance(image) < 1e-9)
                .expect("a copy laid at the mirror image of every corner");
            (*original, copy)
        })
        .collect()
}

fn assert_symmetric(sketch: &Sketch, twins: &[(PointId, PointId)], through: DVec2, towards: DVec2) {
    for (original, copy) in twins {
        let wanted = mirrored_about(sketch.point(*original), through, towards);
        assert!(
            sketch.point(*copy).distance(wanted) < AFTER_THE_SOLVER,
            "the copy of the corner now at {} stands at {}, not at its mirror image {wanted}",
            sketch.point(*original),
            sketch.point(*copy),
        );
    }
}

fn drag(sketch: &mut Sketch, point: PointId, by: DVec2) -> LengthOutcome {
    let pull = sketch.pull(point, 1.0);
    let to = sketch.point(point) + by;
    let landing = sketch.slide(point, to);
    sketch.settle_pulled(&pull, landing, 1.0)
}

const V_THROUGH: DVec2 = DVec2::ZERO;
const V_TOWARDS: DVec2 = DVec2::Y;

#[test]
fn dragging_the_original_moves_its_copy_the_mirrored_way() {
    let mut triangle = a_triangle_east_of_the_v_axis();
    let held = the_sides(&triangle);
    let made = triangle
        .sketch
        .mirror(&held, ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");
    let twins = twins(
        &triangle.sketch,
        &triangle.corners,
        &made,
        V_THROUGH,
        V_TOWARDS,
    );
    let dragged = triangle.corners[1];
    let to = triangle.sketch.point(dragged) + DVec2::new(1.0, 2.0);

    drag(&mut triangle.sketch, dragged, DVec2::new(1.0, 2.0));

    assert!(
        triangle.sketch.point(dragged).distance(to) < 1e-3,
        "the dragged corner is at {}, not where the hand took it, {to}",
        triangle.sketch.point(dragged)
    );
    assert_symmetric(&triangle.sketch, &twins, V_THROUGH, V_TOWARDS);
}

#[test]
fn a_length_typed_on_the_original_moves_its_copy() {
    let mut triangle = a_triangle_east_of_the_v_axis();
    let held = the_sides(&triangle);
    let made = triangle
        .sketch
        .mirror(&held, ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");
    let twins = twins(
        &triangle.sketch,
        &triangle.corners,
        &made,
        V_THROUGH,
        V_TOWARDS,
    );
    let base = DimensionTarget::Length(triangle.sides[0]);

    triangle.sketch.set_dimension(base, 5.0, false);
    let outcome = triangle.sketch.land_value(base, 1.0);

    assert_eq!(outcome, LengthOutcome::Exact);
    assert!(
        (triangle.sketch.segment_length(triangle.sides[0]) - 5.0).abs() < AFTER_THE_SOLVER,
        "the base did not take the length typed: {}",
        triangle.sketch.segment_length(triangle.sides[0])
    );
    assert_symmetric(&triangle.sketch, &twins, V_THROUGH, V_TOWARDS);
}

#[test]
fn moving_the_axis_moves_the_copy() {
    let mut triangle = a_triangle_east_of_the_v_axis();
    let low = triangle.sketch.add_point(DVec2::new(0.0, -5.0));
    let high = triangle.sketch.add_point(DVec2::new(0.0, 5.0));
    let axis = triangle.sketch.add_segment(low, high);
    let held = the_sides(&triangle);
    let made = triangle
        .sketch
        .mirror(&held, ChosenAxis::Trait(axis))
        .expect("a trait to mirror across");
    let twins = twins(
        &triangle.sketch,
        &triangle.corners,
        &made,
        triangle.sketch.point(low),
        triangle.sketch.point(high),
    );
    let copies_before: Vec<DVec2> = twins
        .iter()
        .map(|(_, copy)| triangle.sketch.point(*copy))
        .collect();
    let to = triangle.sketch.point(high) + DVec2::new(1.0, 0.0);

    drag(&mut triangle.sketch, high, DVec2::new(1.0, 0.0));

    assert!(
        triangle.sketch.point(high).distance(to) < 1e-3,
        "the end of the axis is at {}, not where the hand took it, {to}",
        triangle.sketch.point(high)
    );
    let travelled = twins
        .iter()
        .zip(&copies_before)
        .map(|((_, copy), was)| triangle.sketch.point(*copy).distance(*was))
        .fold(0.0, f64::max);
    assert!(
        travelled > 0.1,
        "the axis turned and not one corner of the copy moved with it"
    );
    assert_symmetric(
        &triangle.sketch,
        &twins,
        triangle.sketch.point(low),
        triangle.sketch.point(high),
    );
}

#[test]
fn dragging_the_copy_moves_the_original_the_mirrored_way() {
    let mut triangle = a_triangle_east_of_the_v_axis();
    let held = the_sides(&triangle);
    let made = triangle
        .sketch
        .mirror(&held, ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");
    let twins = twins(
        &triangle.sketch,
        &triangle.corners,
        &made,
        V_THROUGH,
        V_TOWARDS,
    );
    let (original, dragged) = twins[1];
    let to = triangle.sketch.point(dragged) + DVec2::new(-1.0, 2.0);

    drag(&mut triangle.sketch, dragged, DVec2::new(-1.0, 2.0));

    assert!(
        triangle.sketch.point(dragged).distance(to) < 1e-3,
        "the dragged corner of the copy is at {}, not where the hand took it, {to}",
        triangle.sketch.point(dragged)
    );
    assert!(
        triangle
            .sketch
            .point(original)
            .distance(DVec2::new(7.0, 3.0))
            < AFTER_THE_SOLVER,
        "the original corner stands at {}, not at the mirror image of the hand",
        triangle.sketch.point(original)
    );
    assert_symmetric(&triangle.sketch, &twins, V_THROUGH, V_TOWARDS);
}

#[test]
fn a_length_typed_on_the_copy_drives_the_original() {
    let mut triangle = a_triangle_east_of_the_v_axis();
    let held = the_sides(&triangle);
    let made = triangle
        .sketch
        .mirror(&held, ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");
    let twins = twins(
        &triangle.sketch,
        &triangle.corners,
        &made,
        V_THROUGH,
        V_TOWARDS,
    );
    let (copy_start, copy_end) = (twins[0].1, twins[1].1);
    let copied_base = made
        .segments
        .iter()
        .copied()
        .find(|side| {
            let ends = triangle.sketch.segments()[side.0];
            [ends.start, ends.end] == [copy_start, copy_end]
                || [ends.end, ends.start] == [copy_start, copy_end]
        })
        .expect("the copy of the base");
    let typed = DimensionTarget::Length(copied_base);

    triangle.sketch.set_dimension(typed, 5.0, false);
    let outcome = triangle.sketch.land_value(typed, 1.0);

    assert_eq!(outcome, LengthOutcome::Exact);
    assert!(
        (triangle.sketch.segment_length(triangle.sides[0]) - 5.0).abs() < AFTER_THE_SOLVER,
        "the original base is {} long, not the length typed on its copy",
        triangle.sketch.segment_length(triangle.sides[0])
    );
    assert_symmetric(&triangle.sketch, &twins, V_THROUGH, V_TOWARDS);
}

struct HalfProfile {
    sketch: Sketch,
    on_the_axis: [PointId; 2],
    off_the_axis: [PointId; 2],
    sides: [SegmentId; 3],
}

fn half_a_profile_standing_on_the_v_axis() -> HalfProfile {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let foot = sketch.add_point(DVec2::new(0.0, 1.0));
    let low = sketch.add_point(DVec2::new(3.0, 1.0));
    let high = sketch.add_point(DVec2::new(3.0, 4.0));
    let head = sketch.add_point(DVec2::new(0.0, 4.0));
    let sides = [
        sketch.add_segment(foot, low),
        sketch.add_segment(low, high),
        sketch.add_segment(high, head),
    ];
    HalfProfile {
        sketch,
        on_the_axis: [foot, head],
        off_the_axis: [low, high],
        sides,
    }
}

fn mirrored_half_profile() -> (HalfProfile, Duplicated) {
    let mut half = half_a_profile_standing_on_the_v_axis();
    let held = half.sides.map(Element::Segment);
    let made = half
        .sketch
        .mirror(&held, ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");
    (half, made)
}

#[test]
fn a_point_on_the_axis_is_shared_by_the_original_and_its_copy() {
    let (half, made) = mirrored_half_profile();

    assert_eq!(
        made.points.len(),
        2,
        "only the two corners off the axis have a copy; the two on it serve both halves"
    );
    for shared in half.on_the_axis {
        let ending_there = made
            .segments
            .iter()
            .filter(|side| {
                let ends = half.sketch.segments()[side.0];
                ends.start == shared || ends.end == shared
            })
            .count();
        assert_eq!(
            ending_there,
            1,
            "the copy ends on the original's own point at {}, not on one of its own laid there",
            half.sketch.point(shared)
        );
    }
}

#[test]
fn a_point_shared_across_the_axis_slides_along_it_and_cannot_leave_it() {
    let (mut half, made) = mirrored_half_profile();
    let twins = twins(
        &half.sketch,
        &half.off_the_axis,
        &made,
        V_THROUGH,
        V_TOWARDS,
    );
    let head = half.on_the_axis[1];

    drag(&mut half.sketch, head, DVec2::new(1.0, 1.0));

    let at = half.sketch.point(head);
    assert!(
        at.x.abs() < AFTER_THE_SOLVER,
        "the point on the axis left it for {at}"
    );
    assert!(
        (at.y - 5.0).abs() < 1e-3,
        "the point on the axis did not slide along it with the hand: {at}"
    );
    assert_symmetric(&half.sketch, &twins, V_THROUGH, V_TOWARDS);
}

#[test]
fn a_half_profile_closed_by_a_mirror_stays_closed_when_dragged() {
    let (mut half, _) = mirrored_half_profile();
    assert_eq!(
        half.sketch.regions().len(),
        1,
        "the mirror closes the half-profile into one area"
    );

    drag(&mut half.sketch, half.on_the_axis[0], DVec2::new(1.0, -0.5));
    drag(&mut half.sketch, half.off_the_axis[1], DVec2::new(1.0, 1.0));

    assert_eq!(
        half.sketch.regions().len(),
        1,
        "the profile came open where it crosses the axis"
    );
}
