//! A mirror lays a copy that stays the mirror image of what it was made from.
//!
//! Closes #320.
//! - Each copied point is held symmetric to its original about the axis.
//!   Dragging or re-dimensioning the original moves the copy; moving the axis
//!   moves the copy — `dragging_the_original_moves_its_copy_the_mirrored_way`,
//!   `a_length_typed_on_the_original_moves_its_copy`,
//!   `moving_the_axis_moves_the_copy`,
//!   `a_radius_typed_on_a_mirrored_circle_is_taken_by_its_copy`,
//!   `an_arc_s_radius_typed_on_the_original_is_taken_by_its_copy`,
//!   `an_ellipse_s_width_typed_on_the_original_is_taken_by_its_copy`
//! - The link works both ways. Dragging a point of the copy moves the original
//!   the mirrored way; a dimension put on the copy drives the original too —
//!   `dragging_the_copy_moves_the_original_the_mirrored_way`,
//!   `a_length_typed_on_the_copy_drives_the_original`,
//!   `a_radius_typed_on_the_copy_of_a_circle_drives_its_original`
//! - A point shared by the original and its copy — one lying on the axis — is
//!   symmetric to itself and stays on the axis: it slides along it and cannot
//!   leave it. A half-profile closed by a mirror stays closed —
//!   `a_point_on_the_axis_is_shared_by_the_original_and_its_copy`,
//!   `a_point_shared_across_the_axis_slides_along_it_and_cannot_leave_it`,
//!   `a_half_profile_closed_by_a_mirror_stays_closed_when_dragged`; across a
//!   trait, a point in its middle stays on it —
//!   `a_point_shared_in_the_middle_of_an_axis_trait_stays_on_it_when_dragged`
//!   — and one of its ends is held by nothing more —
//!   `a_point_ending_the_axis_trait_is_held_by_nothing_more`
//! - The copy carries the rules internal to the original — horizontal,
//!   parallel, tangent, … — when every element the rule speaks of was copied,
//!   so a mirrored rectangle stays a rectangle. A rule from the original to
//!   something that was not copied is not carried over. Dimensions are not
//!   duplicated where the symmetry already holds the value —
//!   `a_mirrored_rectangle_stays_a_rectangle`,
//!   `a_rule_to_something_left_out_of_the_copy_is_not_carried_over`,
//!   `a_length_the_symmetry_already_holds_is_not_typed_again_on_the_copy`,
//!   `a_rule_to_the_sketch_s_own_axes_is_not_carried_across_a_trait`,
//!   `a_rule_to_a_trait_lying_on_the_axis_is_carried_with_that_trait_as_its_own_image`,
//!   `a_rule_on_a_fillet_whose_corner_was_copied_is_carried_over`,
//!   `a_point_held_where_a_fillet_meets_its_first_side_stays_there_on_the_copy`,
//!   `parallel_equal_and_tangent_rules_inside_the_original_are_carried_over`,
//!   `rules_on_copied_curves_are_carried_over`,
//!   `rules_to_the_sketch_s_axes_are_carried_across_one_of_them`,
//!   `rules_to_the_sketch_s_axes_are_not_carried_across_a_slanted_trait`;
//!   a fixed element is not carried, the copy being held by the symmetry —
//!   `a_fixed_point_of_the_original_does_not_fix_its_copy`
//! - Erasing a trait erases its twin, from either side; while one of them is
//!   selected, its twin is highlighted with it — what the twin of each piece
//!   is, here: `every_mirrored_piece_names_its_twin_and_is_named_by_it`,
//!   `a_trait_ending_on_the_axis_names_the_copy_ending_there_too_and_the_shared_point_names_none`,
//!   `a_piece_mirrored_twice_names_its_three_images`; the erasing goes through
//!   the part's `EraseMany`, not `Sketch::erase`, which trimming and merging
//!   call too, and is answered under a `Closes #320` block of its own in
//!   `crates/part/tests/mirroring_a_selection.rs`, the highlight in
//!   `crates/app/src/screens/viewport/render/emphasis/tests.rs`
//! - Erasing the axis frees the copy. Both drawings stay where they are; the
//!   symmetry goes with the axis. The carried-over rules stay —
//!   `erasing_the_axis_frees_the_copy_where_it_stands`,
//!   `the_rules_carried_over_stay_when_the_axis_goes`,
//!   `erasing_the_axis_frees_a_mirrored_circle_s_radius`
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
    ChosenAxis, CircleId, CircleSymmetry, Constraint, CutPoint, CutShape, DimensionTarget,
    Duplicated, Element, LaidFrom, LengthOutcome, PointId, Repeats, SegmentId, Sketch, SketchAxis,
    Symmetry, WorkPlane,
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

fn image_of(
    sketch: &Sketch,
    made: &Duplicated,
    side: SegmentId,
    through: DVec2,
    towards: DVec2,
) -> SegmentId {
    let (from, to) = sketch.endpoints(side);
    let wanted = [from, to].map(|end| mirrored_about(end, through, towards));
    made.segments
        .iter()
        .copied()
        .find(|copy| {
            let (a, b) = sketch.endpoints(*copy);
            (a.distance(wanted[0]) < 1e-9 && b.distance(wanted[1]) < 1e-9)
                || (a.distance(wanted[1]) < 1e-9 && b.distance(wanted[0]) < 1e-9)
        })
        .expect("a copy of the side laid at its mirror image")
}

fn square_corners(sketch: &Sketch, sides: &[SegmentId]) -> f64 {
    sides
        .windows(2)
        .map(|pair| {
            let (a, b) = sketch.endpoints(pair[0]);
            let (c, d) = sketch.endpoints(pair[1]);
            (b - a).normalize().dot((d - c).normalize()).abs()
        })
        .fold(0.0, f64::max)
}

fn perpendicular(first: SegmentId, second: SegmentId) -> Constraint {
    Constraint::Perpendicular { first, second }
}

#[test]
fn a_mirrored_rectangle_stays_a_rectangle() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(2.0, 1.0),
        DVec2::new(6.0, 1.0),
        DVec2::new(6.0, 4.0),
        DVec2::new(2.0, 4.0),
    ]
    .map(|at| sketch.add_point(at));
    let sides: [SegmentId; 4] =
        std::array::from_fn(|side| sketch.add_segment(corners[side], corners[(side + 1) % 4]));
    for pair in 0..3 {
        sketch.add_constraint(perpendicular(sides[pair], sides[pair + 1]));
    }
    sketch.add_constraint(Constraint::AxisParallel {
        segment: sides[0],
        axis: SketchAxis::U,
    });

    let made = sketch
        .mirror(
            &sides.map(Element::Segment),
            ChosenAxis::Sketch(SketchAxis::V),
        )
        .expect("an axis to mirror across");

    let copies = sides.map(|side| image_of(&sketch, &made, side, V_THROUGH, V_TOWARDS));
    for pair in 0..3 {
        assert!(
            sketch.carries(perpendicular(copies[pair], copies[pair + 1])),
            "the copy of side {pair} and the next one are not held square: {:?}",
            sketch.constraints()
        );
    }
    assert!(
        sketch.carries(Constraint::AxisParallel {
            segment: copies[0],
            axis: SketchAxis::U,
        }),
        "the copy of the base is not held horizontal, as its original is: {:?}",
        sketch.constraints()
    );
    let corner = sketch.segments()[copies[1].0].end;
    assert_eq!(
        drag(&mut sketch, corner, DVec2::new(-1.0, 1.0)),
        LengthOutcome::Exact,
        "the copy carrying its rules beside the symmetry refuses a drag"
    );
    let off_square = square_corners(&sketch, &copies);
    assert!(
        off_square < AFTER_THE_SOLVER,
        "the copy is no longer a rectangle: {off_square}"
    );
}

#[test]
fn a_rule_to_something_left_out_of_the_copy_is_not_carried_over() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(2.0, 1.0));
    let east = sketch.add_point(DVec2::new(8.0, 1.0));
    let north = sketch.add_point(DVec2::new(2.0, 4.0));
    let base = sketch.add_segment(corner, east);
    let up = sketch.add_segment(corner, north);
    sketch.add_constraint(perpendicular(base, up));
    let centre = sketch.add_point(DVec2::new(5.0, -1.0));
    let circle = sketch.add_circle(centre, 2.0);
    sketch.add_constraint(Constraint::Tangent {
        circle,
        segment: base,
        at: None,
        from: LaidFrom::Nowhere,
    });

    let made = sketch
        .mirror(
            &[Element::Segment(base), Element::Segment(up)],
            ChosenAxis::Sketch(SketchAxis::V),
        )
        .expect("an axis to mirror across");

    let (copied_base, copied_up) = (
        image_of(&sketch, &made, base, V_THROUGH, V_TOWARDS),
        image_of(&sketch, &made, up, V_THROUGH, V_TOWARDS),
    );
    assert!(
        sketch.carries(perpendicular(copied_base, copied_up)),
        "the rule between two copied traits did not come along: {:?}",
        sketch.constraints()
    );
    let tangent_to_the_copy = sketch
        .constraints()
        .iter()
        .any(|rule| matches!(rule, Constraint::Tangent { segment, .. } if *segment == copied_base));
    assert!(
        !tangent_to_the_copy,
        "the copy was held tangent to a circle the mirror left out: {:?}",
        sketch.constraints()
    );
}

#[test]
fn a_length_the_symmetry_already_holds_is_not_typed_again_on_the_copy() {
    let mut triangle = a_triangle_east_of_the_v_axis();
    let base = DimensionTarget::Length(triangle.sides[0]);
    triangle.sketch.set_dimension(base, 4.0, false);
    let values = triangle.sketch.dimensions().len();
    let held = the_sides(&triangle);

    let made = triangle
        .sketch
        .mirror(&held, ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");

    let copied_base = image_of(
        &triangle.sketch,
        &made,
        triangle.sides[0],
        V_THROUGH,
        V_TOWARDS,
    );
    assert_eq!(
        triangle.sketch.dimensions().len(),
        values,
        "a value was laid again on the copy"
    );
    assert!(
        triangle
            .sketch
            .would_be_redundant(DimensionTarget::Length(copied_base), 1.0),
        "the copy's base is not held by the symmetry, so its length is free"
    );
}

#[test]
fn a_rule_to_the_sketch_s_own_axes_is_not_carried_across_a_trait() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low = sketch.add_point(DVec2::new(-2.0, -6.0));
    let high = sketch.add_point(DVec2::new(2.0, 6.0));
    let axis = sketch.add_segment(low, high);
    let corner = sketch.add_point(DVec2::new(4.0, 0.0));
    let east = sketch.add_point(DVec2::new(8.0, 0.0));
    let north = sketch.add_point(DVec2::new(4.0, 3.0));
    let base = sketch.add_segment(corner, east);
    let up = sketch.add_segment(corner, north);
    sketch.add_constraint(perpendicular(base, up));
    sketch.add_constraint(Constraint::AxisParallel {
        segment: base,
        axis: SketchAxis::U,
    });
    let (through, towards) = (sketch.point(low), sketch.point(high));

    let made = sketch
        .mirror(
            &[Element::Segment(base), Element::Segment(up)],
            ChosenAxis::Trait(axis),
        )
        .expect("a trait to mirror across");

    let (copied_base, copied_up) = (
        image_of(&sketch, &made, base, through, towards),
        image_of(&sketch, &made, up, through, towards),
    );
    assert!(
        sketch.carries(perpendicular(copied_base, copied_up)),
        "the rule between two copied traits did not come along: {:?}",
        sketch.constraints()
    );
    let held_to_an_axis = sketch.constraints().iter().any(|rule| {
        matches!(
            rule,
            Constraint::AxisParallel { segment, .. }
                | Constraint::AxisPerpendicular { segment, .. }
                | Constraint::AxisCollinear { segment, .. }
                if *segment == copied_base
        )
    });
    assert!(
        !held_to_an_axis,
        "across a slanted trait the image of a horizontal is not horizontal, and the copy \
         was held to the sketch's axes all the same: {:?}",
        sketch.constraints()
    );
}

#[test]
fn a_rule_to_a_trait_lying_on_the_axis_is_carried_with_that_trait_as_its_own_image() {
    let mut half = half_a_profile_standing_on_the_v_axis();
    let [foot, head] = half.on_the_axis;
    let closing = half.sketch.add_segment(head, foot);
    let [base, right, top] = half.sides;
    for (first, second) in [(base, right), (base, closing), (top, closing)] {
        half.sketch.add_constraint(perpendicular(first, second));
    }
    let held = [base, right, top, closing].map(Element::Segment);

    let made = half
        .sketch
        .mirror(&held, ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");

    let [copied_base, copied_right, copied_top] = half
        .sides
        .map(|side| image_of(&half.sketch, &made, side, V_THROUGH, V_TOWARDS));
    for (first, second) in [
        (copied_base, copied_right),
        (copied_base, closing),
        (copied_top, closing),
    ] {
        assert!(
            half.sketch.carries(perpendicular(first, second)),
            "{first:?} and {second:?} are not held square on the copy's side: {:?}",
            half.sketch.constraints()
        );
    }
    let corner = half.sketch.segments()[copied_right.0].end;
    assert_eq!(
        drag(&mut half.sketch, corner, DVec2::new(-1.0, 1.0)),
        LengthOutcome::Exact,
        "the copy carrying its rules beside the symmetry refuses a drag"
    );
}

struct AcrossATrait {
    sketch: Sketch,
    axis: SegmentId,
    twins: Vec<(PointId, PointId)>,
}

fn a_triangle_mirrored_across_a_trait() -> AcrossATrait {
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
        V_THROUGH,
        V_TOWARDS,
    );
    AcrossATrait {
        sketch: triangle.sketch,
        axis,
        twins,
    }
}

#[test]
fn erasing_the_axis_frees_the_copy_where_it_stands() {
    let AcrossATrait {
        mut sketch,
        axis,
        twins,
    } = a_triangle_mirrored_across_a_trait();
    let before: Vec<(PointId, DVec2)> = sketch.live_points().collect();

    sketch.erase(Element::Segment(axis));
    sketch.resolve(1.0);

    for (point, was) in &before {
        assert!(
            sketch.point(*point).distance(*was) < AFTER_THE_SOLVER,
            "erasing the axis moved the point at {was} to {}",
            sketch.point(*point)
        );
    }
    let (original, copy) = twins[1];
    let copy_was = sketch.point(copy);
    let to = sketch.point(original) + DVec2::new(1.0, 2.0);
    assert_eq!(
        drag(&mut sketch, original, DVec2::new(1.0, 2.0)),
        LengthOutcome::Exact
    );
    assert!(
        sketch.point(original).distance(to) < 1e-3,
        "the dragged corner is at {}, not where the hand took it, {to}",
        sketch.point(original)
    );
    assert!(
        sketch.point(copy).distance(copy_was) < AFTER_THE_SOLVER,
        "with the axis gone the copy still followed its original, from {copy_was} to {}",
        sketch.point(copy)
    );
}

#[test]
fn the_rules_carried_over_stay_when_the_axis_goes() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(2.0, 1.0),
        DVec2::new(6.0, 1.0),
        DVec2::new(6.0, 4.0),
        DVec2::new(2.0, 4.0),
    ]
    .map(|at| sketch.add_point(at));
    let sides: [SegmentId; 4] =
        std::array::from_fn(|side| sketch.add_segment(corners[side], corners[(side + 1) % 4]));
    for pair in 0..3 {
        sketch.add_constraint(perpendicular(sides[pair], sides[pair + 1]));
    }
    let low = sketch.add_point(DVec2::new(0.0, -5.0));
    let high = sketch.add_point(DVec2::new(0.0, 5.0));
    let axis = sketch.add_segment(low, high);
    let made = sketch
        .mirror(&sides.map(Element::Segment), ChosenAxis::Trait(axis))
        .expect("a trait to mirror across");
    let copies = sides.map(|side| image_of(&sketch, &made, side, V_THROUGH, V_TOWARDS));
    let originals_before: Vec<DVec2> = corners.iter().map(|at| sketch.point(*at)).collect();

    sketch.erase(Element::Segment(axis));
    sketch.resolve(1.0);

    for pair in 0..3 {
        assert!(
            sketch.carries(perpendicular(copies[pair], copies[pair + 1])),
            "the copy lost a right angle with the axis: {:?}",
            sketch.constraints()
        );
    }
    let corner = sketch.segments()[copies[1].0].end;
    assert_eq!(
        drag(&mut sketch, corner, DVec2::new(-1.0, 1.0)),
        LengthOutcome::Exact
    );
    let off_square = square_corners(&sketch, &copies);
    assert!(
        off_square < AFTER_THE_SOLVER,
        "the freed copy is no longer a rectangle: {off_square}"
    );
    for (corner, was) in corners.iter().zip(&originals_before) {
        assert!(
            sketch.point(*corner).distance(*was) < AFTER_THE_SOLVER,
            "the freed copy pulled the original's corner from {was} to {}",
            sketch.point(*corner)
        );
    }
}

fn a_circle_mirrored_across_the_v_axis() -> (Sketch, CircleId, CircleId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(4.0, 2.0));
    let original = sketch.add_circle(centre, 1.5);
    let made = sketch
        .mirror(
            &[Element::Circle(original)],
            ChosenAxis::Sketch(SketchAxis::V),
        )
        .expect("an axis to mirror across");
    (sketch, original, made.circles[0])
}

fn typed(sketch: &mut Sketch, target: DimensionTarget, value: f64) {
    sketch.set_dimension(target, value, false);
    assert_eq!(
        sketch.land_value(target, 1.0),
        LengthOutcome::Exact,
        "the value typed was refused"
    );
}

#[test]
fn a_radius_typed_on_a_mirrored_circle_is_taken_by_its_copy() {
    let (mut sketch, original, copy) = a_circle_mirrored_across_the_v_axis();

    typed(&mut sketch, DimensionTarget::Radius(original), 2.5);

    let radius = sketch.circles()[copy.0].radius;
    assert!(
        (radius - 2.5).abs() < AFTER_THE_SOLVER,
        "the copy kept a radius of {radius} while its original took 2.5"
    );
}

#[test]
fn a_radius_typed_on_the_copy_of_a_circle_drives_its_original() {
    let (mut sketch, original, copy) = a_circle_mirrored_across_the_v_axis();

    typed(&mut sketch, DimensionTarget::Radius(copy), 2.5);

    let radius = sketch.circles()[original.0].radius;
    assert!(
        (radius - 2.5).abs() < AFTER_THE_SOLVER,
        "the original kept a radius of {radius} while its copy took 2.5"
    );
}

#[test]
fn an_arc_s_radius_typed_on_the_original_is_taken_by_its_copy() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(5.0, 1.0));
    let east = sketch.add_point(DVec2::new(8.0, 1.0));
    let north = sketch.add_point(DVec2::new(5.0, 4.0));
    let original = sketch.add_arc(centre, east, north);
    let made = sketch
        .mirror(&[Element::Arc(original)], ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");

    typed(&mut sketch, DimensionTarget::ArcRadius(original), 4.0);

    let radius = sketch.arc_radius(made.arcs[0]);
    assert!(
        (radius - 4.0).abs() < AFTER_THE_SOLVER,
        "the copy kept a radius of {radius} while its original took 4"
    );
}

#[test]
fn an_ellipse_s_width_typed_on_the_original_is_taken_by_its_copy() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let [centre, west, east, south, north] = [
        (20.0, 10.0),
        (5.0, 10.0),
        (35.0, 10.0),
        (20.0, 5.0),
        (20.0, 15.0),
    ]
    .map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
    let original = sketch.add_ellipse(centre, [west, east], [south, north]);
    let made = sketch
        .mirror(
            &[Element::Ellipse(original)],
            ChosenAxis::Sketch(SketchAxis::V),
        )
        .expect("an axis to mirror across");
    let width = sketch.ellipses()[original.0].first;
    let copied_width = sketch.ellipses()[made.ellipses[0].0].first;

    typed(&mut sketch, DimensionTarget::Length(width), 40.0);

    let length = sketch.segment_length(copied_width);
    assert!(
        (length - 40.0).abs() < AFTER_THE_SOLVER,
        "the copy is {length} wide while its original took 40"
    );
}

#[test]
fn a_rule_on_a_fillet_whose_corner_was_copied_is_carried_over() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(2.0, 1.0));
    let east = sketch.add_point(DVec2::new(8.0, 1.0));
    let north = sketch.add_point(DVec2::new(2.0, 6.0));
    let base = sketch.add_segment(corner, east);
    let up = sketch.add_segment(corner, north);
    let cut = sketch
        .cut_corner(base, up, CutShape::Fillet { radius: 1.0 }, false, 0)
        .expect("the two traits meet at the corner");
    let centre = DVec2::new(3.0, 2.0);
    let towards_the_corner = DVec2::new(-1.0, -1.0).normalize();
    let touching = centre + towards_the_corner;
    let along = towards_the_corner.perp();
    let at_the_centre = sketch.add_point(centre);
    let on_the_curve = sketch.add_point(touching);
    let (tangent, _, _) = {
        let start = sketch.add_point(touching - along * 2.0);
        let end = sketch.add_point(touching + along * 2.0);
        (sketch.add_segment(start, end), start, end)
    };
    let rules = [
        Constraint::AtCutPoint {
            point: at_the_centre,
            cut,
            at: CutPoint::Centre,
        },
        Constraint::OnCut {
            point: on_the_curve,
            cut,
        },
        Constraint::CutTangent {
            cut,
            segment: tangent,
            from: LaidFrom::Nowhere,
        },
    ];
    for rule in rules {
        sketch.add_constraint(rule);
        assert!(sketch.carries(rule), "the fixture could not lay {rule:?}");
    }

    let made = sketch
        .mirror(
            &[
                Element::Segment(base),
                Element::Segment(up),
                Element::Point(at_the_centre),
                Element::Point(on_the_curve),
                Element::Segment(tangent),
            ],
            ChosenAxis::Sketch(SketchAxis::V),
        )
        .expect("an axis to mirror across");

    let copied_cut = *made.cuts.first().expect("the copy's corner is cut too");
    let image = |place: DVec2| {
        let wanted = mirrored_about(place, V_THROUGH, V_TOWARDS);
        made.points
            .iter()
            .copied()
            .find(|point| sketch.point(*point).distance(wanted) < 1e-9)
            .expect("a copy at the mirror image")
    };
    let carried = [
        Constraint::AtCutPoint {
            point: image(centre),
            cut: copied_cut,
            at: CutPoint::Centre,
        },
        Constraint::OnCut {
            point: image(touching),
            cut: copied_cut,
        },
        Constraint::CutTangent {
            cut: copied_cut,
            segment: image_of(&sketch, &made, tangent, V_THROUGH, V_TOWARDS),
            from: LaidFrom::Nowhere,
        },
    ];
    for rule in carried {
        assert!(
            sketch.carries(rule),
            "{rule:?} did not come along with the fillet it speaks of: {:?}",
            sketch.constraints()
        );
    }
}

fn a_corner_on_a_centre() -> (Sketch, PointId, [SegmentId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(1.0, 1.0));
    let east = sketch.add_point(DVec2::new(4.0, 1.0));
    let north = sketch.add_point(DVec2::new(4.0, 3.0));
    let sides = [
        sketch.add_segment(centre, east),
        sketch.add_segment(east, north),
    ];
    sketch.add_constraint(perpendicular(sides[0], sides[1]));
    (sketch, centre, sides)
}

fn assert_bare(sketch: &Sketch, rules: &[Constraint], centre: PointId, made: &Duplicated) {
    assert_eq!(
        sketch.constraints(),
        rules,
        "a pattern laid rules beside its copies"
    );
    for copy in &made.segments {
        let ends = sketch.segments()[copy.0];
        assert!(
            ends.start != centre && ends.end != centre,
            "a copy of a pattern ends on the very point it turned about"
        );
    }
}

#[test]
fn a_pattern_still_lays_copies_bare_of_every_rule() {
    let (mut sketch, centre, sides) = a_corner_on_a_centre();
    let rules = sketch.constraints().to_vec();
    let made = sketch
        .pattern_around(&sides.map(Element::Segment), centre, 90.0, 4)
        .expect("a centre to turn about");
    assert_eq!(made.segments.len(), 6);
    assert_bare(&sketch, &rules, centre, &made);

    let (mut sketch, centre, sides) = a_corner_on_a_centre();
    let rules = sketch.constraints().to_vec();
    let made = sketch
        .pattern_along(
            &sides.map(Element::Segment),
            ChosenAxis::Sketch(SketchAxis::U),
            Repeats {
                step: 6.0,
                count: 2,
            },
            Repeats {
                step: 5.0,
                count: 2,
            },
        )
        .expect("a direction to step along");
    assert_eq!(made.segments.len(), 6);
    assert_bare(&sketch, &rules, centre, &made);
}

#[test]
fn a_mirrored_arc_still_sweeps_as_its_original_once_the_drawing_settles() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(5.0, 1.0));
    let east = sketch.add_point(DVec2::new(8.0, 1.0));
    let north = sketch.add_point(DVec2::new(5.0, 4.0));
    let original = sketch.add_arc(centre, east, north);
    let made = sketch
        .mirror(&[Element::Arc(original)], ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");
    let copy = made.arcs[0];

    sketch.resolve(1.0);
    drag(&mut sketch, centre, DVec2::new(1.0, 1.0));

    let (was, now) = (sketch.arcs()[original.0], sketch.arcs()[copy.0]);
    assert!(
        (sketch.arc_sweep(copy) - sketch.arc_sweep(original)).abs() < AFTER_THE_SOLVER,
        "the copy sweeps {} against its original's {}",
        sketch.arc_sweep(copy),
        sketch.arc_sweep(original)
    );
    for (copy_end, original_end) in [(now.start, was.end), (now.end, was.start)] {
        let wanted = mirrored_about(sketch.point(original_end), V_THROUGH, V_TOWARDS);
        assert!(
            sketch.point(copy_end).distance(wanted) < AFTER_THE_SOLVER,
            "a reflection turns the curve over, so the copy's start is the image of the \
             original's end: {} against {wanted}",
            sketch.point(copy_end)
        );
    }
}

#[test]
fn a_point_held_where_a_fillet_meets_its_first_side_stays_there_on_the_copy() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(2.0, 1.0));
    let east = sketch.add_point(DVec2::new(8.0, 1.0));
    let north = sketch.add_point(DVec2::new(2.0, 6.0));
    let base = sketch.add_segment(corner, east);
    let up = sketch.add_segment(corner, north);
    let cut = sketch
        .cut_corner(base, up, CutShape::Fillet { radius: 1.0 }, false, 0)
        .expect("the two traits meet at the corner");
    let first_touch = sketch
        .cut_point(cut, CutPoint::First)
        .expect("a fillet touches its first side");
    let held = sketch.add_point(first_touch);
    let rule = Constraint::AtCutPoint {
        point: held,
        cut,
        at: CutPoint::First,
    };
    sketch.add_constraint(rule);
    assert!(sketch.carries(rule), "the fixture could not lay {rule:?}");

    let made = sketch
        .mirror(
            &[
                Element::Segment(base),
                Element::Segment(up),
                Element::Point(held),
            ],
            ChosenAxis::Sketch(SketchAxis::V),
        )
        .expect("an axis to mirror across");

    let copied_cut = made.cuts[0];
    let wanted = mirrored_about(first_touch, V_THROUGH, V_TOWARDS);
    let copied_point = made
        .points
        .iter()
        .copied()
        .find(|point| sketch.point(*point).distance(wanted) < 1e-9)
        .expect("a copy of the point at its mirror image");
    assert!(
        sketch.carries(Constraint::AtCutPoint {
            point: copied_point,
            cut: copied_cut,
            at: CutPoint::First,
        }),
        "the copy's point is not held where its fillet meets its first side: {:?}",
        sketch.constraints()
    );
    sketch.resolve(1.0);
    assert!(
        sketch.point(copied_point).distance(wanted) < AFTER_THE_SOLVER,
        "the copy's first side is not the image of the original's: its point went from \
         {wanted} to {}",
        sketch.point(copied_point)
    );
}

#[test]
fn erasing_the_axis_frees_a_mirrored_circle_s_radius() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low = sketch.add_point(DVec2::new(0.0, -5.0));
    let high = sketch.add_point(DVec2::new(0.0, 5.0));
    let axis = sketch.add_segment(low, high);
    let centre = sketch.add_point(DVec2::new(4.0, 2.0));
    let original = sketch.add_circle(centre, 1.5);
    let made = sketch
        .mirror(&[Element::Circle(original)], ChosenAxis::Trait(axis))
        .expect("a trait to mirror across");
    let copy = made.circles[0];

    sketch.erase(Element::Segment(axis));
    sketch.resolve(1.0);
    typed(&mut sketch, DimensionTarget::Radius(original), 2.5);

    let radius = sketch.circles()[copy.0].radius;
    assert!(
        (radius - 1.5).abs() < AFTER_THE_SOLVER,
        "with the axis gone the copy still took its original's radius: {radius}"
    );
}

#[test]
fn parallel_equal_and_tangent_rules_inside_the_original_are_carried_over() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (low, high) = (
        [DVec2::new(2.0, 1.0), DVec2::new(6.0, 2.0)].map(|at| sketch.add_point(at)),
        [DVec2::new(2.0, 4.0), DVec2::new(6.0, 5.0)].map(|at| sketch.add_point(at)),
    );
    let bottom = sketch.add_segment(low[0], low[1]);
    let top = sketch.add_segment(high[0], high[1]);
    let centre = sketch.add_point(DVec2::new(10.0, 3.0));
    let circle = sketch.add_circle(centre, 2.0);
    let side_start = sketch.add_point(DVec2::new(8.0, 1.0));
    let side_end = sketch.add_point(DVec2::new(12.0, 1.0));
    let touching = sketch.add_segment(side_start, side_end);
    sketch.add_constraint(Constraint::Parallel {
        first: bottom,
        second: top,
    });
    sketch.add_constraint(Constraint::Equal {
        first: bottom,
        second: top,
    });
    sketch.add_constraint(Constraint::Tangent {
        circle,
        segment: touching,
        at: None,
        from: LaidFrom::Nowhere,
    });

    let made = sketch
        .mirror(
            &[
                Element::Segment(bottom),
                Element::Segment(top),
                Element::Circle(circle),
                Element::Segment(touching),
            ],
            ChosenAxis::Sketch(SketchAxis::V),
        )
        .expect("an axis to mirror across");

    let [copied_bottom, copied_top, copied_touching] =
        [bottom, top, touching].map(|side| image_of(&sketch, &made, side, V_THROUGH, V_TOWARDS));
    let copied_circle = made.circles[0];
    for rule in [
        Constraint::Parallel {
            first: copied_bottom,
            second: copied_top,
        },
        Constraint::Equal {
            first: copied_bottom,
            second: copied_top,
        },
    ] {
        assert!(
            sketch.carries(rule),
            "{rule:?} did not come along: {:?}",
            sketch.constraints()
        );
    }
    let tangent_on_the_copy = sketch.constraints().iter().any(|rule| {
        matches!(
            rule,
            Constraint::Tangent { circle, segment, .. }
                if *circle == copied_circle && *segment == copied_touching
        )
    });
    assert!(
        tangent_on_the_copy,
        "the copied trait is not held tangent to the copied circle: {:?}",
        sketch.constraints()
    );
}

#[test]
fn a_fixed_point_of_the_original_does_not_fix_its_copy() {
    let mut triangle = a_triangle_east_of_the_v_axis();
    triangle.sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(triangle.corners[0]),
    });
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
    let copy = twins[0].1;
    let fixed: Vec<&Constraint> = triangle
        .sketch
        .constraints()
        .iter()
        .filter(|rule| matches!(rule, Constraint::Fixed { .. }))
        .collect();
    assert_eq!(
        fixed,
        vec![&Constraint::Fixed {
            element: Element::Point(triangle.corners[0]),
        }],
        "the copy of a fixed point was fixed too, beside the symmetry that already holds it"
    );
    assert!(!triangle.sketch.carries(Constraint::Fixed {
        element: Element::Point(copy),
    }));
}

fn point_at(sketch: &Sketch, place: DVec2) -> PointId {
    sketch
        .live_points()
        .find(|(_, at)| at.distance(place) < 1e-9)
        .map(|(id, _)| id)
        .unwrap_or_else(|| panic!("a point at {place}"))
}

#[test]
fn every_mirrored_piece_names_its_twin_and_is_named_by_it() {
    let mut triangle = a_triangle_east_of_the_v_axis();
    let sketch = &mut triangle.sketch;
    let centre = sketch.add_point(DVec2::new(10.0, 2.0));
    let circle = sketch.add_circle(centre, 1.5);
    let other_centre = sketch.add_point(DVec2::new(10.0, 6.0));
    let other_circle = sketch.add_circle(other_centre, 0.5);
    let [arc_centre, arc_start, arc_end] =
        [(15.0, 1.0), (18.0, 1.0), (15.0, 4.0)].map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
    let arc = sketch.add_arc(arc_centre, arc_start, arc_end);
    let [oval_centre, west, east, south, north] = [
        (30.0, 10.0),
        (25.0, 10.0),
        (35.0, 10.0),
        (30.0, 8.0),
        (30.0, 12.0),
    ]
    .map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
    let ellipse = sketch.add_ellipse(oval_centre, [west, east], [south, north]);
    let alone = sketch.add_point(DVec2::new(12.0, 7.0));
    let mut held = triangle.sides.map(Element::Segment).to_vec();
    held.extend([
        Element::Circle(circle),
        Element::Circle(other_circle),
        Element::Arc(arc),
        Element::Ellipse(ellipse),
        Element::Point(alone),
    ]);

    let made = sketch
        .mirror(&held, ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");

    let mut pairs: Vec<(Element, Element)> = triangle
        .sides
        .iter()
        .map(|side| {
            (
                Element::Segment(*side),
                Element::Segment(image_of(sketch, &made, *side, V_THROUGH, V_TOWARDS)),
            )
        })
        .collect();
    pairs.extend([
        (Element::Circle(circle), Element::Circle(made.circles[0])),
        (
            Element::Circle(other_circle),
            Element::Circle(made.circles[1]),
        ),
        (Element::Arc(arc), Element::Arc(made.arcs[0])),
        (
            Element::Ellipse(ellipse),
            Element::Ellipse(made.ellipses[0]),
        ),
        (
            Element::Point(alone),
            Element::Point(point_at(sketch, DVec2::new(-12.0, 7.0))),
        ),
    ]);
    for (original, copy) in pairs {
        assert_eq!(sketch.twins_of(original), vec![copy], "{original:?}");
        assert_eq!(sketch.twins_of(copy), vec![original], "{copy:?}");
    }
}

#[test]
fn a_trait_ending_on_the_axis_names_the_copy_ending_there_too_and_the_shared_point_names_none() {
    let (half, made) = mirrored_half_profile();
    let [foot, _] = half.on_the_axis;
    let base = half.sides[0];
    let copied_base = image_of(&half.sketch, &made, base, V_THROUGH, V_TOWARDS);

    assert_eq!(
        half.sketch.twins_of(Element::Segment(base)),
        vec![Element::Segment(copied_base)]
    );
    assert_eq!(
        half.sketch.twins_of(Element::Point(foot)),
        Vec::new(),
        "a point on the axis is its own image, and has no twin to go with it"
    );
}

#[test]
fn a_piece_mirrored_twice_names_its_three_images() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(2.0, 1.0));
    let end = sketch.add_point(DVec2::new(6.0, 3.0));
    let quarter = sketch.add_segment(start, end);
    let half = sketch
        .mirror(
            &[Element::Segment(quarter)],
            ChosenAxis::Sketch(SketchAxis::V),
        )
        .expect("an axis to mirror across");
    sketch
        .mirror(
            &[
                Element::Segment(quarter),
                Element::Segment(half.segments[0]),
            ],
            ChosenAxis::Sketch(SketchAxis::U),
        )
        .expect("an axis to mirror across");

    let mut twins = sketch.twins_of(Element::Segment(quarter));
    let mut wanted: Vec<Element> = [(-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0)]
        .map(|(x, y)| {
            let flip = DVec2::new(x, y);
            let (from, to) = (
                point_at(&sketch, DVec2::new(2.0, 1.0) * flip),
                point_at(&sketch, DVec2::new(6.0, 3.0) * flip),
            );
            sketch
                .live_segments()
                .find(|(_, side)| (side.start, side.end) == (from, to))
                .map(|(id, _)| Element::Segment(id))
                .expect("an image of the quarter")
        })
        .to_vec();
    twins.sort_by_key(|element| format!("{element:?}"));
    wanted.sort_by_key(|element| format!("{element:?}"));
    assert_eq!(twins, wanted);
}

#[test]
fn a_symmetry_is_one_rule_whichever_way_round_it_is_named() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let point = sketch.add_point(DVec2::new(4.0, 2.0));
    let centre = sketch.add_point(DVec2::new(8.0, 2.0));
    let circle = sketch.add_circle(centre, 1.0);
    sketch
        .mirror(
            &[Element::Point(point), Element::Circle(circle)],
            ChosenAxis::Sketch(SketchAxis::V),
        )
        .expect("an axis to mirror across");
    let reversed: Vec<Constraint> = sketch
        .constraints()
        .iter()
        .filter_map(|rule| match *rule {
            Constraint::Symmetric(pair) => Some(Constraint::Symmetric(Symmetry {
                point: pair.twin,
                twin: pair.point,
                axis: pair.axis,
            })),
            Constraint::SymmetricCircles(pair) => {
                Some(Constraint::SymmetricCircles(CircleSymmetry {
                    circle: pair.twin,
                    twin: pair.circle,
                    axis: pair.axis,
                }))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        reversed.len(),
        3,
        "the point and the centre each held to their twin, and the two radii"
    );

    for rule in reversed {
        assert!(
            sketch.carries(rule),
            "{rule:?} is the rule the mirror laid, named the other way round"
        );
        sketch.erase_constraint(rule);
        assert!(
            !sketch.carries(rule),
            "{rule:?} named the other way round could not be erased"
        );
    }
}

fn rules_to_the_axes_across(
    axis_of: impl Fn(&mut Sketch) -> ChosenAxis,
) -> (Sketch, [Constraint; 3]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let axis = axis_of(&mut sketch);
    let on_u = sketch.add_point(DVec2::new(3.0, 0.0));
    let (lying_start, lying_end) = (
        sketch.add_point(DVec2::new(5.0, 0.0)),
        sketch.add_point(DVec2::new(8.0, 0.0)),
    );
    let lying = sketch.add_segment(lying_start, lying_end);
    let (upright_start, upright_end) = (
        sketch.add_point(DVec2::new(4.0, 2.0)),
        sketch.add_point(DVec2::new(4.0, 5.0)),
    );
    let upright = sketch.add_segment(upright_start, upright_end);
    for rule in [
        Constraint::OnAxis {
            point: on_u,
            axis: SketchAxis::U,
        },
        Constraint::AxisCollinear {
            segment: lying,
            axis: SketchAxis::U,
        },
        Constraint::AxisPerpendicular {
            segment: upright,
            axis: SketchAxis::U,
        },
    ] {
        sketch.add_constraint(rule);
    }
    let held = [
        Element::Point(on_u),
        Element::Segment(lying),
        Element::Segment(upright),
    ];
    let mut made = sketch
        .mirror(&held, axis)
        .expect("an axis to mirror across");
    let (through, towards) = match axis {
        ChosenAxis::Sketch(_) => (V_THROUGH, V_TOWARDS),
        ChosenAxis::Trait(segment) => sketch.endpoints(segment),
    };
    let copied_point = made.points.remove(
        made.points
            .iter()
            .position(|point| {
                sketch.point(*point).distance(mirrored_about(
                    DVec2::new(3.0, 0.0),
                    through,
                    towards,
                )) < 1e-9
            })
            .expect("a copy of the point on the axis"),
    );
    let images = [
        Constraint::OnAxis {
            point: copied_point,
            axis: SketchAxis::U,
        },
        Constraint::AxisCollinear {
            segment: image_of(&sketch, &made, lying, through, towards),
            axis: SketchAxis::U,
        },
        Constraint::AxisPerpendicular {
            segment: image_of(&sketch, &made, upright, through, towards),
            axis: SketchAxis::U,
        },
    ];
    (sketch, images)
}

#[test]
fn rules_to_the_sketch_s_axes_are_carried_across_one_of_them() {
    let (sketch, images) = rules_to_the_axes_across(|_| ChosenAxis::Sketch(SketchAxis::V));

    for rule in images {
        assert!(
            sketch.carries(rule),
            "across V the image of {rule:?} holds, and the copy was not given it: {:?}",
            sketch.constraints()
        );
    }
}

#[test]
fn rules_to_the_sketch_s_axes_are_not_carried_across_a_slanted_trait() {
    let (sketch, images) = rules_to_the_axes_across(|sketch| {
        let low = sketch.add_point(DVec2::new(-2.0, -6.0));
        let high = sketch.add_point(DVec2::new(1.0, 6.0));
        ChosenAxis::Trait(sketch.add_segment(low, high))
    });

    for rule in images {
        assert!(
            !sketch.carries(rule),
            "across a slanted trait {rule:?} would no longer hold, and the copy was given it"
        );
    }
}

#[test]
fn rules_on_copied_curves_are_carried_over() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let [arc_centre, arc_start, arc_end] =
        [(5.0, 1.0), (8.0, 1.0), (5.0, 4.0)].map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
    let arc = sketch.add_arc(arc_centre, arc_start, arc_end);
    let grazing_start = sketch.add_point(DVec2::new(8.0, -3.0));
    let grazing_end = sketch.add_point(DVec2::new(8.0, -1.0));
    let grazing = sketch.add_segment(grazing_start, grazing_end);
    let on_the_arc_at = DVec2::new(5.0, 1.0) + DVec2::splat(3.0 / 2.0_f64.sqrt());
    let on_the_arc = sketch.add_point(on_the_arc_at);
    let [oval_centre, west, east, south, north] = [
        (20.0, 10.0),
        (5.0, 10.0),
        (35.0, 10.0),
        (20.0, 5.0),
        (20.0, 15.0),
    ]
    .map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
    let ellipse = sketch.add_ellipse(oval_centre, [west, east], [south, north]);
    let on_the_ellipse_at = DVec2::new(20.0 + 15.0 * 0.5, 10.0 + 5.0 * 3.0_f64.sqrt() / 2.0);
    let on_the_ellipse = sketch.add_point(on_the_ellipse_at);
    let rounds = [(30.0, 2.0), (36.0, 2.0)].map(|(x, y)| {
        let centre = sketch.add_point(DVec2::new(x, y));
        sketch.add_circle(centre, 1.5)
    });
    for rule in [
        Constraint::ArcTangent {
            arc,
            segment: grazing,
            at: None,
            from: LaidFrom::Nowhere,
        },
        Constraint::OnArc {
            point: on_the_arc,
            arc,
            from: LaidFrom::Nowhere,
        },
        Constraint::OnEllipse {
            point: on_the_ellipse,
            ellipse,
            from: LaidFrom::Nowhere,
        },
        Constraint::EqualRadius {
            first: rounds[0],
            second: rounds[1],
        },
    ] {
        sketch.add_constraint(rule);
        assert!(sketch.carries(rule), "the fixture could not lay {rule:?}");
    }

    let made = sketch
        .mirror(
            &[
                Element::Arc(arc),
                Element::Segment(grazing),
                Element::Point(on_the_arc),
                Element::Ellipse(ellipse),
                Element::Point(on_the_ellipse),
                Element::Circle(rounds[0]),
                Element::Circle(rounds[1]),
            ],
            ChosenAxis::Sketch(SketchAxis::V),
        )
        .expect("an axis to mirror across");

    let image = |place: DVec2| point_at(&sketch, mirrored_about(place, V_THROUGH, V_TOWARDS));
    let (copied_arc, copied_grazing) = (
        made.arcs[0],
        image_of(&sketch, &made, grazing, V_THROUGH, V_TOWARDS),
    );
    let grazing_on_the_copy = sketch.constraints().iter().any(|rule| {
        matches!(
            rule,
            Constraint::ArcTangent { arc, segment, .. }
                if *arc == copied_arc && *segment == copied_grazing
        )
    });
    assert!(
        grazing_on_the_copy,
        "the copied trait does not graze the copied arc: {:?}",
        sketch.constraints()
    );
    for rule in [
        Constraint::OnArc {
            point: image(on_the_arc_at),
            arc: copied_arc,
            from: LaidFrom::Nowhere,
        },
        Constraint::OnEllipse {
            point: image(on_the_ellipse_at),
            ellipse: made.ellipses[0],
            from: LaidFrom::Nowhere,
        },
        Constraint::EqualRadius {
            first: made.circles[0],
            second: made.circles[1],
        },
    ] {
        assert!(
            sketch.carries(rule),
            "{rule:?} did not come along with the curves it speaks of: {:?}",
            sketch.constraints()
        );
    }
}

#[test]
fn a_point_ending_the_axis_trait_is_held_by_nothing_more() {
    let mut half = half_a_profile_standing_on_the_v_axis();
    let [foot, head] = half.on_the_axis;
    let axis = half.sketch.add_segment(foot, head);

    half.sketch
        .mirror(&half.sides.map(Element::Segment), ChosenAxis::Trait(axis))
        .expect("a trait to mirror across");

    let held_on_the_axis: Vec<&Constraint> = half
        .sketch
        .constraints()
        .iter()
        .filter(|rule| matches!(rule, Constraint::OnSegment { segment, .. } if *segment == axis))
        .collect();
    assert!(
        held_on_the_axis.is_empty(),
        "the axis's own ends were held on it besides: {held_on_the_axis:?}"
    );
}

#[test]
fn a_point_shared_in_the_middle_of_an_axis_trait_stays_on_it_when_dragged() {
    let mut half = half_a_profile_standing_on_the_v_axis();
    let low = half.sketch.add_point(DVec2::new(0.0, -5.0));
    let high = half.sketch.add_point(DVec2::new(0.0, 10.0));
    let axis = half.sketch.add_segment(low, high);
    half.sketch
        .mirror(&half.sides.map(Element::Segment), ChosenAxis::Trait(axis))
        .expect("a trait to mirror across");
    let head = half.on_the_axis[1];

    drag(&mut half.sketch, head, DVec2::new(1.0, 1.0));

    let (through, towards) = half.sketch.endpoints(axis);
    let off = (towards - through)
        .normalize()
        .perp_dot(half.sketch.point(head) - through);
    assert!(
        off.abs() < AFTER_THE_SOLVER,
        "the point shared on the axis left it by {off}"
    );
}
