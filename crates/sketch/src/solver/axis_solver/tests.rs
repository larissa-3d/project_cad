//! What sketch · solver/axis_solver.rs is held to.
//!
//! Closes #1060.
//! - Also, what the fix implies: an angle typed to an axis on a trait shrunk to
//!   nothing still owes its miss, and pushes nothing rather than dividing by
//!   the trait's length —
//!   `an_angle_to_an_axis_owes_its_miss_on_a_trait_shrunk_to_nothing`

use glam::DVec2;

use super::*;
use crate::axis_angle::AxisToward;
use crate::constraints::Toward;
use crate::plane::WorkPlane;

#[test]
fn an_angle_to_an_axis_owes_its_miss_on_a_trait_shrunk_to_nothing() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let foot = sketch.add_point(DVec2::new(0.0, 0.0));
    let head = sketch.add_point(DVec2::new(0.0, 10.0));
    sketch.add_segment(foot, head);
    let from = sketch.add_point(DVec2::new(5.0, 5.0));
    let to = sketch.add_point(DVec2::new(5.0, 5.0) + DVec2::splat(1e-9 / 2f64.sqrt()));
    let shrunk = sketch.add_segment(from, to);
    let target = DimensionTarget::AxisAngle {
        segment: shrunk,
        segment_toward: Toward::End,
        axis: SketchAxis::U,
        axis_toward: AxisToward::Positive,
    };

    let equation = sketch
        .axis_angle_equation(target, 20.0, sketch.drawing_size())
        .expect("an angle to an axis on a drawn trait has an equation");

    let miss = (45.0f64 - 20.0).to_radians();
    assert!(
        (equation.error - miss).abs() < 1e-9,
        "the trait reads 45° where 20° is typed, a miss of {miss} rad; the equation owes {}",
        equation.error
    );
    assert!(
        equation.gradient.iter().all(|push| *push == 0.0),
        "a trait shrunk to nothing is pushed by nothing: {:?}",
        equation.gradient
    );
}
