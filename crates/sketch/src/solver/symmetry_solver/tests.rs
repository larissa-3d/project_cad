//! What sketch · solver/symmetry_solver.rs is held to: the two statements a
//! symmetry asks of the solver, read off known places, and their gradients
//! against the change each coordinate actually makes.

use glam::DVec2;

use crate::axis::ChosenAxis;
use crate::constraints::SketchAxis;
use crate::plane::WorkPlane;
use crate::sketch::{PointId, Sketch};
use crate::symmetry::Symmetry;

const EXACT: f64 = 1e-12;
const STEP: f64 = 1e-6;
const SAMPLED: f64 = 1e-6;

fn errors(sketch: &Sketch, pair: Symmetry) -> Vec<f64> {
    let mut into = Vec::new();
    sketch.symmetric_equations(pair, &mut into);
    into.iter().map(|equation| equation.error).collect()
}

fn a_pair(point: DVec2, twin: DVec2) -> (Sketch, PointId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (point, twin) = (sketch.add_point(point), sketch.add_point(twin));
    (sketch, point, twin)
}

fn across_v(point: PointId, twin: PointId) -> Symmetry {
    Symmetry {
        point,
        twin,
        axis: ChosenAxis::Sketch(SketchAxis::V),
    }
}

/// A slanted trait three units by four, so that its length is not one and its
/// direction no axis of the sketch.
fn a_slanted_axis(sketch: &mut Sketch) -> ChosenAxis {
    let start = sketch.add_point(DVec2::new(1.0, -1.0));
    let end = sketch.add_point(DVec2::new(4.0, 3.0));
    ChosenAxis::Trait(sketch.add_segment(start, end))
}

fn mirrored_about(place: DVec2, through: DVec2, towards: DVec2) -> DVec2 {
    let along = (towards - through).normalize();
    let offset = place - through;
    through + along * 2.0 * offset.dot(along) - offset
}

fn assert_gradients_match_the_change_they_predict(sketch: &Sketch, pair: Symmetry) {
    let mut into = Vec::new();
    sketch.symmetric_equations(pair, &mut into);
    assert_eq!(into.len(), 2, "a symmetry asks two things of the solver");
    for (row, equation) in into.iter().enumerate() {
        for (coordinate, written) in equation.gradient.iter().enumerate() {
            let point = PointId(coordinate / 2);
            let nudge = match coordinate % 2 {
                0 => DVec2::X * STEP,
                _ => DVec2::Y * STEP,
            };
            let at = sketch.point(point);
            let mut ahead = sketch.clone();
            ahead.move_point(point, at + nudge);
            let mut behind = sketch.clone();
            behind.move_point(point, at - nudge);
            let sampled = (errors(&ahead, pair)[row] - errors(&behind, pair)[row]) / (2.0 * STEP);
            assert!(
                (written - sampled).abs() < SAMPLED,
                "row {row}, coordinate {coordinate} of {point:?}: the gradient says {written}, \
                 nudging the point says {sampled}"
            );
        }
    }
}

#[test]
fn a_pair_standing_mirror_images_across_the_v_axis_asks_nothing_more() {
    let (sketch, point, twin) = a_pair(DVec2::new(3.0, 2.0), DVec2::new(-3.0, 2.0));

    for error in errors(&sketch, across_v(point, twin)) {
        assert!(
            error.abs() < EXACT,
            "a pair already symmetric is off by {error}"
        );
    }
}

#[test]
fn a_pair_off_its_axis_is_off_by_how_far_its_middle_strays_and_how_far_it_leans() {
    let (sketch, point, twin) = a_pair(DVec2::new(3.0, 1.0), DVec2::new(-2.0, 2.0));

    let off = errors(&sketch, across_v(point, twin));

    assert!(
        (off[0].abs() - 0.5).abs() < EXACT,
        "the pair's middle stands half a unit off the axis, and the first error reads {}",
        off[0]
    );
    assert!(
        (off[1].abs() - 1.0).abs() < EXACT,
        "the twin stands one unit further along the axis, and the second error reads {}",
        off[1]
    );
}

#[test]
fn a_pair_mirror_images_across_a_slanted_trait_asks_nothing_more() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let axis = a_slanted_axis(&mut sketch);
    let place = DVec2::new(5.0, 0.5);
    let image = mirrored_about(place, DVec2::new(1.0, -1.0), DVec2::new(4.0, 3.0));
    let (point, twin) = (sketch.add_point(place), sketch.add_point(image));

    for error in errors(&sketch, Symmetry { point, twin, axis }) {
        assert!(
            error.abs() < EXACT,
            "a pair already symmetric is off by {error}"
        );
    }
}

#[test]
fn a_symmetry_s_gradients_say_what_moving_each_point_does_across_the_v_axis() {
    let (sketch, point, twin) = a_pair(DVec2::new(3.0, 1.0), DVec2::new(-2.0, 2.5));

    assert_gradients_match_the_change_they_predict(&sketch, across_v(point, twin));
}

#[test]
fn a_symmetry_s_gradients_say_what_moving_each_point_and_the_axis_does_across_a_trait() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let axis = a_slanted_axis(&mut sketch);
    let point = sketch.add_point(DVec2::new(5.0, 0.5));
    let twin = sketch.add_point(DVec2::new(-0.5, 3.0));

    assert_gradients_match_the_change_they_predict(&sketch, Symmetry { point, twin, axis });
}

#[test]
fn a_trait_with_no_length_serves_as_no_axis() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let only = sketch.add_point(DVec2::new(1.0, 1.0));
    let axis = ChosenAxis::Trait(sketch.add_segment(only, only));
    let point = sketch.add_point(DVec2::new(3.0, 1.0));
    let twin = sketch.add_point(DVec2::new(-1.0, 1.0));

    assert!(errors(&sketch, Symmetry { point, twin, axis }).is_empty());
}

#[test]
fn a_symmetry_naming_a_point_the_drawing_does_not_have_asks_nothing() {
    let (sketch, point, _) = a_pair(DVec2::new(3.0, 1.0), DVec2::new(-3.0, 1.0));

    for pair in [across_v(point, PointId(99)), across_v(PointId(99), point)] {
        assert!(errors(&sketch, pair).is_empty(), "{pair:?}");
    }
}
