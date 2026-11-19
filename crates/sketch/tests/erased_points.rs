//! What a point erased leaves of the drawing: nothing.
//!
//! Closes #1045.
//! - adding then erasing a point leaves the areas unchanged, on a crossing and
//!   a hair off it —
//!   `a_point_added_then_erased_where_two_curves_cross_leaves_the_areas_as_they_were`,
//!   `a_point_added_then_erased_a_hair_off_where_two_curves_cross_leaves_the_areas_as_they_were`
//! - the cases above hold in #501's campaign — no test: not in this file;
//!   `what_random_sketches_found.rs` holds the campaign's drawings, their
//!   `ignore` taken off

use cao_sketch::{DimensionTarget, Element, PointId, Sketch, SketchAxis, WorkPlane};
use glam::DVec2;

const SCALE: f64 = 1.0;

/// The origin and one point held to it by a distance along each axis: nothing
/// in it can move any more.
fn pinned_pair() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(3.0, 4.0));
    for axis in [SketchAxis::U, SketchAxis::V] {
        sketch.set_dimension(
            DimensionTarget::Projected {
                from: PointId(0),
                to: corner,
                axis,
            },
            30.0,
            false,
        );
    }
    sketch
}

#[test]
fn an_erased_point_says_nothing_about_the_rest_of_the_drawing() {
    let mut sketch = pinned_pair();
    let before = sketch.settled_points(SCALE);
    let freedom = sketch.freedom(SCALE);

    let orphan = sketch.add_point(DVec2::new(50.0, 50.0));
    sketch.erase(Element::Point(orphan));

    let after = sketch.settled_points(SCALE);

    assert_eq!(
        after[..before.len()],
        before[..],
        "the points still drawn keep the verdict they had",
    );
    assert_eq!(
        sketch.freedom(SCALE).degrees_of_freedom,
        freedom.degrees_of_freedom,
        "an erased point adds no freedom",
    );
    assert!(
        after[orphan.0],
        "an erased point cannot move, so it is not what is left to do",
    );
}

#[test]
fn a_point_still_drawn_is_the_one_that_reads_as_free() {
    let mut sketch = pinned_pair();
    let orphan = sketch.add_point(DVec2::new(50.0, 50.0));

    let settled = sketch.settled_points(SCALE);

    assert!(!settled[orphan.0], "nothing holds it, so it can still move");
    assert_eq!(sketch.freedom(SCALE).degrees_of_freedom, 2);
}

fn a_circle_across_a_triangle_s_base() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(5.0, 5.0));
    sketch.add_circle(centre, 3.0);
    let corners =
        [[1.0, 5.0], [10.0, 5.0], [5.5, 12.0]].map(|at| sketch.add_point(DVec2::from(at)));
    for (rank, corner) in corners.iter().enumerate() {
        sketch.add_segment(*corner, corners[(rank + 1) % corners.len()]);
    }
    sketch
}

const WHERE_THE_BASE_CROSSES_THE_CIRCLE: DVec2 = DVec2::new(2.0, 5.0);

/// Far under the ten-millionth at which the walk calls two places one, so a
/// point standing that far off a crossing is welded to it.
const A_HAIR: f64 = 3e-8;

const SAME_AREA: f64 = 1e-9;

fn areas(sketch: &Sketch) -> Vec<f64> {
    let mut found: Vec<f64> = sketch
        .regions()
        .iter()
        .map(|region| region.area())
        .collect();
    found.sort_by(f64::total_cmp);
    found
}

fn a_point_added_then_erased_at(place: DVec2) -> (Vec<f64>, Vec<f64>) {
    let mut sketch = a_circle_across_a_triangle_s_base();
    let before = areas(&sketch);
    assert_eq!(
        before.len(),
        3,
        "two half-discs and the triangle around one: {before:?}"
    );

    let point = sketch.add_point(place);
    sketch.erase(Element::Point(point));

    (before, areas(&sketch))
}

fn assert_the_same_areas(before: &[f64], after: &[f64]) {
    assert_eq!(
        after.len(),
        before.len(),
        "an erased point closes and opens nothing: {before:?} before, {after:?} after",
    );
    for (was, is) in before.iter().zip(after) {
        assert!(
            (was - is).abs() <= SAME_AREA * was.abs().max(1.0),
            "an erased point changes no area: {before:?} before, {after:?} after",
        );
    }
}

#[test]
fn a_point_added_then_erased_where_two_curves_cross_leaves_the_areas_as_they_were() {
    let (before, after) = a_point_added_then_erased_at(WHERE_THE_BASE_CROSSES_THE_CIRCLE);

    assert_the_same_areas(&before, &after);
}

#[test]
fn a_point_added_then_erased_a_hair_off_where_two_curves_cross_leaves_the_areas_as_they_were() {
    let along_the_base = WHERE_THE_BASE_CROSSES_THE_CIRCLE + DVec2::new(A_HAIR, 0.0);

    let (before, after) = a_point_added_then_erased_at(along_the_base);

    assert_the_same_areas(&before, &after);
}
