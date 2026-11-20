//! What sketch · solver/angle_solver.rs is held to.
//!
//! Closes #1060.
//! - Also, what the fix implies: an angle between two traits whose one side is
//!   shrunk to nothing still owes its miss, and pushes nothing rather than
//!   dividing by that side's length, whichever of the two it is —
//!   `an_angle_between_two_traits_owes_its_miss_with_its_first_side_shrunk_to_nothing`,
//!   `an_angle_between_two_traits_owes_its_miss_with_its_second_side_shrunk_to_nothing`;
//!   and with neither side shrunk, it pushes —
//!   `an_angle_between_two_traits_of_ordinary_length_pushes_their_ends`

use glam::DVec2;

use super::*;
use crate::plane::WorkPlane;

/// An upright trait 10 long and, beside it, one at 45° to it `side` long.
fn an_upright_trait_and_a_slanted_one(side: f64) -> (Sketch, SegmentId, SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let foot = sketch.add_point(DVec2::new(0.0, 0.0));
    let head = sketch.add_point(DVec2::new(0.0, 10.0));
    let upright = sketch.add_segment(foot, head);
    let from = sketch.add_point(DVec2::new(5.0, 5.0));
    let to = sketch.add_point(DVec2::new(5.0, 5.0) + DVec2::splat(side / 2f64.sqrt()));
    let slanted = sketch.add_segment(from, to);
    (sketch, upright, slanted)
}

fn sixty_degrees_typed_between(sketch: &Sketch, first: SegmentId, second: SegmentId) -> Equation {
    let target = DimensionTarget::AngleBetween {
        first,
        first_toward: Toward::End,
        second,
        second_toward: Toward::End,
    };
    sketch
        .angle_between_equation(target, 60.0, sketch.drawing_size())
        .expect("an angle between two drawn traits has an equation")
}

fn owes_fifteen_degrees_and_pushes_nothing(equation: &Equation) {
    let miss = (45.0f64 - 60.0).to_radians();
    assert!(
        (equation.error - miss).abs() < 1e-9,
        "the angle reads 45° where 60° is typed, a miss of {miss} rad; the equation owes {}",
        equation.error
    );
    assert!(
        equation.gradient.iter().all(|push| *push == 0.0),
        "a side shrunk to nothing is pushed by nothing: {:?}",
        equation.gradient
    );
}

#[test]
fn an_angle_between_two_traits_owes_its_miss_with_its_first_side_shrunk_to_nothing() {
    let (sketch, upright, slanted) = an_upright_trait_and_a_slanted_one(1e-9);

    let equation = sixty_degrees_typed_between(&sketch, slanted, upright);

    owes_fifteen_degrees_and_pushes_nothing(&equation);
}

#[test]
fn an_angle_between_two_traits_owes_its_miss_with_its_second_side_shrunk_to_nothing() {
    let (sketch, upright, slanted) = an_upright_trait_and_a_slanted_one(1e-9);

    let equation = sixty_degrees_typed_between(&sketch, upright, slanted);

    owes_fifteen_degrees_and_pushes_nothing(&equation);
}

#[test]
fn an_angle_between_two_traits_of_ordinary_length_pushes_their_ends() {
    let (sketch, upright, slanted) = an_upright_trait_and_a_slanted_one(1.0);

    let equation = sixty_degrees_typed_between(&sketch, upright, slanted);

    assert!(
        equation.gradient.iter().all(|push| push.is_finite())
            && equation.gradient.iter().any(|push| *push != 0.0),
        "both traits can be turned: {:?}",
        equation.gradient
    );
}
