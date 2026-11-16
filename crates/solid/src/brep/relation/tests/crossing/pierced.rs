//! Where the curve a cylinder of another axis meets a ball along, the loops
//! the cylinder's rulings pierce it in, crosses the ball's other curves
//! (#805).
//!
//! Closes #805 — its second slice's crossings only. The relation they stand
//! on is answered in `tests/round.rs`, the faces the curve bounds in
//! `body/tests.rs`, each file with its own criteria.
//! - That curve crosses the sphere face's other curves (a circle cut by a
//!   plane at a slant, the coaxial circles) where it really does —
//!   `a_cylinder_s_curve_on_a_ball_crosses_a_plane_at_a_slant_and_the_ball_s_coaxial_circles_where_it_really_does`
//!
//! Read as: for three cylinders, along the ball's axis, square to it and at a
//! slant, and five surfaces bounding a face of the ball, two planes at a
//! slant, a plane square to its axis, a coaxial cylinder and a coaxial cone,
//! each loop crosses the surface as often as its distance to it changes sign
//! along the loop, every crossing on the loop at its parameter and on the
//! surface, none a touch; and the loops' crossings are, within the tolerance,
//! where the circles the surface cuts the ball along cross the cylinder.

use glam::{DVec2, DVec3};

use crate::brep::relation::tests::{REACH, scale};
use crate::brep::relation::{Crossing, Crossings, crossings, relation};
use crate::brep::surface::{Cone, Cylinder, Plane, Sphere, Surface};

/// A ball of radius 3 about `(0, 0, 5)`, turned about Z.
fn ball() -> Surface {
    Surface::Sphere(Sphere::through(DVec3::ZERO, DVec3::Z, 5.0, 3.0))
}

/// Three cylinders of other axes than the ball's: along its axis 1.5 off its
/// centre, passing right through it; square to its axis 2.5 off, taking in
/// part of it; and at a slant 1 off, passing right through it.
fn walls_off_the_ball_s_centre() -> [Surface; 3] {
    let middle = DVec3::new(0.0, 0.0, 5.0);
    let slant = DVec3::new(1.0, 2.0, 2.0) / 3.0;
    [
        Surface::Cylinder(Cylinder::about(middle + DVec3::X * 1.5, DVec3::Z, 1.0)),
        Surface::Cylinder(Cylinder::about(middle + DVec3::Y * 2.5, DVec3::X, 2.0)),
        Surface::Cylinder(Cylinder::about(
            middle + slant.any_orthonormal_vector(),
            slant,
            1.2,
        )),
    ]
}

/// What else bounds a face of the ball: a plane at a slant through its
/// centre and one at another slant 1 off it, cutting circles of other axes
/// than the ball's; a plane square to its axis, a coaxial cylinder and a
/// coaxial cone, cutting circles about it.
fn bounding_the_ball() -> [Surface; 5] {
    let middle = DVec3::new(0.0, 0.0, 5.0);
    let tilted = DVec3::new(1.0, -1.0, 2.0).normalize();
    [
        Surface::Plane(Plane::through(middle, DVec3::new(1.0, 0.0, 1.0)).0),
        Surface::Plane(Plane::through(middle + tilted, tilted).0),
        Surface::Plane(Plane::through(DVec3::new(0.0, 0.0, 7.0), DVec3::Z).0),
        Surface::Cylinder(Cylinder::about(DVec3::ZERO, DVec3::Z, 2.0)),
        Surface::Cone(Cone::through(
            DVec3::ZERO,
            DVec3::Z,
            [DVec2::new(10.0, 0.0), DVec2::new(0.0, 5.0)],
        )),
    ]
}

/// Where the circles `other` cuts the ball along cross `wall`, read from the
/// circles' side, which a circle against a cylinder of any axis answers.
fn where_its_circles_cross(other: &Surface, wall: &Surface) -> Vec<Crossing> {
    relation(&ball(), other, scale())
        .curves()
        .iter()
        .flat_map(|circle| {
            let found = crossings(circle, wall, scale());
            let Crossings::At(found) = found else {
                panic!("{circle:?} {wall:?}: {found:?}");
            };
            found
        })
        .collect()
}

#[test]
fn a_cylinder_s_curve_on_a_ball_crosses_a_plane_at_a_slant_and_the_ball_s_coaxial_circles_where_it_really_does()
 {
    let eps = scale().eps();
    let mut missed = 0;
    for wall in walls_off_the_ball_s_centre() {
        let loops = relation(&ball(), &wall, scale()).curves();
        assert!(!loops.is_empty(), "{wall:?}");
        let mut crossed = 0;
        for other in bounding_the_ball() {
            let mut on_the_loops = Vec::new();
            for curve in &loops {
                let period = curve.period().expect("a loop closes");
                let found = crossings(curve, &other, scale());
                let Crossings::At(found) = &found else {
                    panic!("{wall:?} {other:?}: {found:?}");
                };
                assert!(
                    found
                        .windows(2)
                        .all(|pair| pair[0].parameter < pair[1].parameter),
                    "{found:?}"
                );
                for crossing in found {
                    let on_the_curve = curve.point(crossing.parameter);
                    assert!(
                        on_the_curve.distance(crossing.point) < 1e-12 * REACH
                            && other.distance(crossing.point).abs() < 1e-12 * REACH
                            && !crossing.tangent,
                        "{crossing:?} {on_the_curve} {other:?}"
                    );
                }
                let side = |t: f64| other.distance(curve.point(t));
                let changes = (0..4000)
                    .filter(|&step| {
                        let [from, to] =
                            [step, step + 1].map(|step| period * f64::from(step) / 4000.0);
                        side(from) * side(to) < 0.0
                    })
                    .count();
                assert_eq!(found.len(), changes, "{wall:?} {other:?} {found:?}");
                on_the_loops.extend(found.iter().copied());
            }
            let on_the_circles = where_its_circles_cross(&other, &wall);
            assert_eq!(
                on_the_loops.len(),
                on_the_circles.len(),
                "{wall:?} {other:?}: {on_the_loops:?} {on_the_circles:?}"
            );
            for crossing in &on_the_loops {
                assert!(
                    on_the_circles
                        .iter()
                        .any(|circled| circled.point.distance(crossing.point) <= eps),
                    "{wall:?} {other:?}: {crossing:?} {on_the_circles:?}"
                );
            }
            crossed += on_the_loops.len();
            missed += usize::from(on_the_loops.is_empty());
        }
        assert!(crossed >= 4, "{wall:?}: {crossed}");
    }
    assert!(missed >= 1, "{missed}");
}
