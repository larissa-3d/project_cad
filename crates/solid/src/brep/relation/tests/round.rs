//! What a sphere or a torus makes with another surface (#576): the coaxial
//! pairs decided once in the meridian half-plane; a sphere and a plane at a
//! slant to its axis, or square to it off its centre, along the circle about
//! the foot of the centre on the plane (#805), or, within the tolerance of
//! touching it off its poles, at the one point it touches (#939); a sphere
//! and a cylinder of another axis along the loops where the cylinder's
//! rulings pierce it (#805); a sphere and a round whose axis holds its
//! centre as the sphere turned about that axis, so that two spheres of one
//! centre and radius are one surface whatever their axes (#956, whose
//! criteria are in `tests/spheres_of_one_centre_and_radius_are_one_surface_whatever_their_axes.rs`);
//! everything else declined.
//!
//! Closes #805 — its second slice's relation only, a cylinder of another axis
//! against a sphere. The slice's crossings are answered in
//! `crossing/pierced.rs`, its faces drawn in `body/tests.rs`, and its fillet and
//! its STEP export in
//! `tests/a_chamfer_and_a_fillet_fill_a_concave_edge_up_to_a_curved_end.rs`,
//! each file with its own criteria.
//! - A cylinder whose axis does not hold a sphere's centre (another axis)
//!   meets the sphere along a curve: `relation` gives a meet whose curves'
//!   points lie on both surfaces to the kernel's tolerance — one closed loop
//!   where the cylinder takes in part of the ball,
//!   `a_cylinder_of_another_axis_taking_in_part_of_a_ball_meets_it_along_one_closed_loop`;
//!   two loops where it passes right through it (ρ + d < R, d the axis's
//!   distance to the centre),
//!   `a_cylinder_of_another_axis_passing_right_through_a_ball_meets_it_along_two_loops`;
//!   apart when it misses the ball (d > ρ + R) or the ball sits inside it
//!   (d + R < ρ),
//!   `a_cylinder_of_another_axis_missing_a_ball_or_holding_it_inside_is_apart_from_it`.
//!   A tangent case declines rather than answers wrong —
//!   `a_cylinder_of_another_axis_within_a_hair_of_touching_a_ball_is_unsupported`.
//!   The coaxial cases keep their answers —
//!   `a_cylinder_a_hair_off_coaxial_is_coaxial_with_a_ball_and_further_off_meets_it_in_two_loops_and_a_cone_further_off_is_unsupported`,
//!   which was the pin of the decline this slice turns round, and the coaxial
//!   tests above it, left as they were. Read as: the same answer whichever
//!   surface comes first, as every pair in this file; each loop closed after
//!   its period, its points on both surfaces within a thousandth of the
//!   kernel's tolerance, as the meets of two walls are; and the loops
//!   together passing through every point where a ruling of the cylinder
//!   pierces the ball, away from where a loop turns back along the ruling,
//!   for cylinders along the ball's axis, square to it and at two slants. A
//!   tangent case is a touch from outside (d = ρ + R), a figure of eight
//!   (ρ + d = R) or a touch from inside (d + R = ρ), each within half a
//!   tolerance; a cone of another axis stays declined, as does a torus
//!   against any wall of another axis

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::{REACH, assert_on_both, cylinder, plane, scale};
use crate::brep::curve::{Circle, Curve};
use crate::brep::relation::{Relation, Rim, at_a_slant, relation};
use crate::brep::surface::{Cone, Cylinder, EllipticCylinder, Sphere, Surface, Torus};
use crate::brep::tessellation::FRONT;

/// A ball of radius 3 about `(0, 0, 5)`, turned about Z.
fn ball() -> Surface {
    Surface::Sphere(Sphere::through(DVec3::ZERO, DVec3::Z, 5.0, 3.0))
}

/// A ring of tube radius 1 about the circle of radius 4 at `h = 2`, about Z.
fn ring() -> Surface {
    Surface::Torus(Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(2.0, 4.0),
        1.0,
        DVec2::new(2.0, 5.0),
    ))
}

/// A cone about Z through two corners `(h, ρ)`.
fn cone(from: [f64; 2], to: [f64; 2]) -> Surface {
    Surface::Cone(Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::from(from), DVec2::from(to)],
    ))
}

/// The relation of two surfaces, the same whichever comes first.
fn between(one: &Surface, other: &Surface) -> Relation {
    let found = relation(one, other, scale());
    assert_eq!(relation(other, one, scale()), found, "{one:?} {other:?}");
    found
}

/// The rims of a relation, each checked to lie on both surfaces.
fn rims(one: &Surface, other: &Surface, within: f64) -> Vec<Rim> {
    let found = between(one, other);
    let Relation::Rims(rims) = &found else {
        panic!("rims expected, got {found:?}");
    };
    assert_on_both(&found, one, other, within);
    rims.clone()
}

#[test]
fn the_rims_and_the_meridians_of_a_pair_are_its_curves_and_the_poles_its_points() {
    let Surface::Sphere(sphere) = ball() else {
        unreachable!()
    };
    let rims = Relation::Rims(vec![
        Rim {
            circle: sphere.circle(5.0, 3.0),
            touching: true,
        },
        Rim {
            circle: sphere.circle(7.0, 1.0),
            touching: false,
        },
    ]);
    assert_eq!(
        rims.curves(),
        vec![
            Curve::Circle(sphere.circle(5.0, 3.0)),
            Curve::Circle(sphere.circle(7.0, 1.0)),
        ]
    );
    assert!(rims.points().is_empty());

    let great = Circle::about(sphere.middle(), DVec3::X, 3.0);
    let poles = vec![DVec3::new(0.0, 0.0, 2.0), DVec3::new(0.0, 0.0, 8.0)];
    let meridians = Relation::Meridians {
        circles: vec![great],
        poles: poles.clone(),
    };
    assert_eq!(meridians.curves(), vec![Curve::Circle(great)]);
    assert_eq!(meridians.points(), poles);
}

#[test]
fn a_plane_square_to_a_ball_s_axis_cuts_the_circle_of_its_section_at_the_plane_s_height() {
    let Surface::Sphere(sphere) = ball() else {
        unreachable!()
    };
    for (height, expected) in [(6.0, 8f64.sqrt()), (5.0, 3.0), (3.0, 5f64.sqrt())] {
        let level = plane(DVec3::new(0.0, 0.0, height), -DVec3::Z);
        let found = rims(&ball(), &level, 1e-12 * REACH);
        let [rim] = found.as_slice() else {
            panic!("one circle at {height}, got {found:?}");
        };
        assert!(!rim.touching);
        assert_eq!(rim.circle, sphere.circle(height, rim.circle.radius));
        assert!(
            (rim.circle.radius - expected).abs() < 1e-12 * REACH,
            "{rim:?}"
        );
    }
}

#[test]
fn a_plane_through_a_ball_s_pole_touches_it_there_and_beyond_it_misses() {
    let eps = scale().eps();
    for (height, pole) in [
        (8.0, 8.0),
        (8.0 + eps / 2.0, 8.0),
        (8.0 - eps / 2.0, 8.0),
        (2.0, 2.0),
    ] {
        let level = plane(DVec3::new(0.0, 0.0, height), DVec3::Z);
        let found = between(&ball(), &level);
        let Relation::Apex(at) = found else {
            panic!("the pole expected at {height}, got {found:?}");
        };
        assert!(at.distance(DVec3::new(0.0, 0.0, pole)) <= eps, "{at}");
    }
    for height in [8.0 + 2.0 * eps, 12.0, 2.0 - 2.0 * eps] {
        let level = plane(DVec3::new(0.0, 0.0, height), DVec3::Z);
        assert_eq!(between(&ball(), &level), Relation::Apart, "{height}");
    }
}

#[test]
fn a_plane_square_through_a_ring_s_tube_cuts_two_circles_and_touches_its_top_along_one() {
    let eps = scale().eps();
    let Surface::Torus(torus) = ring() else {
        unreachable!()
    };
    for (height, inner, outer) in [
        (2.0, 3.0, 5.0),
        (2.5, 4.0 - 0.75f64.sqrt(), 4.0 + 0.75f64.sqrt()),
    ] {
        let level = plane(DVec3::new(0.0, 0.0, height), DVec3::Z);
        let found = rims(&ring(), &level, 1e-12 * REACH);
        let [near, far] = found.as_slice() else {
            panic!("two circles at {height}, got {found:?}");
        };
        assert!(!near.touching && !far.touching);
        assert_eq!(near.circle, torus.circle(height, near.circle.radius));
        assert_eq!(far.circle, torus.circle(height, far.circle.radius));
        assert!(
            (near.circle.radius - inner).abs() < 1e-12 * REACH,
            "{near:?}"
        );
        assert!((far.circle.radius - outer).abs() < 1e-12 * REACH, "{far:?}");
    }
    for height in [3.0, 3.0 + eps / 2.0, 3.0 - eps / 2.0] {
        let level = plane(DVec3::new(0.0, 0.0, height), DVec3::Z);
        let found = rims(&ring(), &level, eps);
        assert_eq!(
            found,
            vec![Rim {
                circle: torus.circle(height, 4.0),
                touching: true,
            }],
            "{height}"
        );
    }
    let level = plane(DVec3::new(0.0, 0.0, 3.0 + 2.0 * eps), DVec3::Z);
    assert_eq!(between(&ring(), &level), Relation::Apart);
}

#[test]
fn a_plane_holding_the_axis_cuts_a_ball_s_great_circle_through_its_poles() {
    let eps = scale().eps();
    for normal in [
        DVec3::X,
        DVec3::Y,
        DVec3::new(1.0, 1.0, 0.0),
        DVec3::new(-3.0, 1.0, 0.0),
        DVec3::new(1.0, 0.0, 0.25 * eps / REACH),
    ] {
        for off in [0.0, eps / 2.0] {
            let side = plane(normal.normalize() * off, normal);
            let found = between(&ball(), &side);
            let Relation::Meridians { circles, poles } = &found else {
                panic!("meridians expected for {normal} {off}, got {found:?}");
            };
            let [great] = circles.as_slice() else {
                panic!("one great circle for {normal}, got {circles:?}");
            };
            assert!(great.center.distance(DVec3::new(0.0, 0.0, 5.0)) < 1e-12 * REACH);
            assert_eq!(great.radius, 3.0);
            assert!(great.axis.cross(normal.normalize()).length() < 1e-12);
            assert_eq!(
                poles.as_slice(),
                [DVec3::new(0.0, 0.0, 2.0), DVec3::new(0.0, 0.0, 8.0)]
            );
            assert_on_both(&found, &ball(), &side, eps);
        }
    }
    let beside = plane(DVec3::new(2.0 * eps, 0.0, 0.0), DVec3::X);
    cut_about_the_foot(&ball(), &beside, "a plane two hairs off the axis");
}

#[test]
fn a_plane_holding_the_axis_cuts_a_ring_s_two_tube_circles_and_reaches_no_pole() {
    let side = plane(DVec3::ZERO, DVec3::X);
    let found = between(&ring(), &side);
    let Relation::Meridians { circles, poles } = &found else {
        panic!("meridians expected, got {found:?}");
    };
    assert!(poles.is_empty());
    let [one, other] = circles.as_slice() else {
        panic!("two tube circles, got {circles:?}");
    };
    assert_eq!(one.center, DVec3::new(0.0, -4.0, 2.0));
    assert_eq!(other.center, DVec3::new(0.0, 4.0, 2.0));
    for circle in circles {
        assert_eq!((circle.radius, circle.axis), (1.0, DVec3::X));
    }
    assert_on_both(&found, &ring(), &side, 1e-12 * REACH);
}

#[test]
fn a_coaxial_cylinder_cuts_a_ball_along_two_circles_of_its_radius_to_the_bit() {
    let bore = cylinder(DVec3::new(0.0, 0.0, -7.0), -DVec3::Z, 2.0);
    let Surface::Cylinder(wall) = bore else {
        unreachable!()
    };
    let found = rims(&ball(), &bore, 1e-12 * REACH);
    let [low, high] = found.as_slice() else {
        panic!("two circles, got {found:?}");
    };
    for (rim, height) in [(low, 5.0 - 5f64.sqrt()), (high, 5.0 + 5f64.sqrt())] {
        assert!(!rim.touching);
        let on_the_wall = Circle::on(&wall, height);
        assert_eq!(rim.circle.radius, 2.0);
        assert_eq!(
            [rim.circle.axis, rim.circle.u, rim.circle.v],
            [on_the_wall.axis, on_the_wall.u, on_the_wall.v]
        );
        assert!(rim.circle.center.distance(on_the_wall.center) < 1e-12 * REACH);
    }
}

#[test]
fn a_coaxial_cylinder_of_a_ball_s_radius_touches_it_at_the_equator() {
    let eps = scale().eps();
    let Surface::Sphere(sphere) = ball() else {
        unreachable!()
    };
    for radius in [3.0, 3.0 + eps / 2.0, 3.0 - eps / 2.0] {
        let shaft = cylinder(DVec3::ZERO, DVec3::Z, radius);
        let found = rims(&ball(), &shaft, eps);
        assert_eq!(
            found,
            vec![Rim {
                circle: sphere.circle(5.0, radius),
                touching: true,
            }],
            "{radius}"
        );
    }
    let wider = cylinder(DVec3::ZERO, DVec3::Z, 3.0 + 2.0 * eps);
    assert_eq!(between(&ball(), &wider), Relation::Apart);
    let narrower = cylinder(DVec3::ZERO, DVec3::Z, 3.0 - 2.0 * eps);
    let found = rims(&ball(), &narrower, eps);
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(found.iter().all(|rim| !rim.touching));
}

#[test]
fn a_coaxial_cylinder_touches_a_ring_at_its_crest_or_its_root_and_cuts_its_tube_between() {
    let Surface::Torus(torus) = ring() else {
        unreachable!()
    };
    for radius in [5.0, 3.0] {
        let wall = cylinder(DVec3::ZERO, DVec3::Z, radius);
        assert_eq!(
            rims(&ring(), &wall, 1e-12 * REACH),
            vec![Rim {
                circle: torus.circle(2.0, radius),
                touching: true,
            }],
            "{radius}"
        );
    }
    let through = cylinder(DVec3::ZERO, DVec3::Z, 4.0);
    let found = rims(&ring(), &through, 1e-12 * REACH);
    assert_eq!(found.len(), 2, "{found:?}");
    for (rim, height) in found.iter().zip([1.0, 3.0]) {
        assert!(!rim.touching);
        assert_eq!(rim.circle.radius, 4.0);
        assert!(rim.circle.center.distance(DVec3::new(0.0, 0.0, height)) < 1e-12 * REACH);
    }
    for radius in [2.5, 6.0] {
        let wall = cylinder(DVec3::ZERO, DVec3::Z, radius);
        assert_eq!(between(&ring(), &wall), Relation::Apart, "{radius}");
    }
}

#[test]
fn a_coaxial_cone_cuts_a_ball_along_two_circles_touches_it_along_one_or_rests_on_its_pole() {
    let crossing = cone([10.0, 0.0], [0.0, 5.0]);
    let found = rims(&ball(), &crossing, 1e-12 * REACH);
    assert_eq!(found.len(), 2, "{found:?}");
    for (rim, height) in found.iter().zip([6.0 - 3.2f64.sqrt(), 6.0 + 3.2f64.sqrt()]) {
        assert!(!rim.touching);
        assert!(rim.circle.center.distance(DVec3::new(0.0, 0.0, height)) < 1e-12 * REACH);
        assert!((rim.circle.radius - (10.0 - height) / 2.0).abs() < 1e-12 * REACH);
    }
    let tangent = cone([10.0, 0.0], [2.0, 6.0]);
    let found = rims(&ball(), &tangent, 1e-12 * REACH);
    let [rim] = found.as_slice() else {
        panic!("one touching circle, got {found:?}");
    };
    assert!(rim.touching);
    assert!(rim.circle.center.distance(DVec3::new(0.0, 0.0, 6.8)) < 1e-12 * REACH);
    assert!((rim.circle.radius - 2.4).abs() < 1e-12 * REACH);
    let resting = cone([8.0, 0.0], [10.0, 4.0]);
    let found = between(&ball(), &resting);
    let Relation::Apex(apex) = found else {
        panic!("the pole expected, got {found:?}");
    };
    assert!(apex.distance(DVec3::new(0.0, 0.0, 8.0)) < 1e-12 * REACH);
    let beyond = cone([12.0, 0.0], [20.0, 2.0]);
    assert_eq!(between(&ball(), &beyond), Relation::Apart);
}

#[test]
fn two_balls_of_one_centre_and_radius_are_one_surface_and_a_hair_apart_in_radius_are_not() {
    let eps = scale().eps();
    let same = |other: Sphere| between(&ball(), &Surface::Sphere(other));
    for other in [
        Sphere::through(DVec3::ZERO, DVec3::Z, 5.0, 3.0),
        Sphere::through(DVec3::ZERO, -DVec3::Z, -5.0, 3.0),
        Sphere::through(
            DVec3::new(eps / 3.0, 0.0, 0.0),
            DVec3::Z,
            5.0,
            3.0 + eps / 2.0,
        ),
        Sphere::through(DVec3::ZERO, DVec3::Z, 5.0 + eps / 2.0, 3.0),
    ] {
        assert_eq!(same(other), Relation::Same { agree: true }, "{other:?}");
    }
    assert_eq!(
        same(Sphere::through(DVec3::ZERO, DVec3::Z, 5.0, 3.0 + 3.0 * eps)),
        Relation::Apart
    );
    assert_eq!(
        same(Sphere::through(DVec3::ZERO, DVec3::Z, 5.0, 1.0)),
        Relation::Apart
    );
    assert_eq!(
        same(Sphere::through(
            DVec3::ZERO,
            DVec3::Z,
            5.0 + 0.6 * eps,
            3.0 + 1.5 * eps
        )),
        Relation::Unsupported
    );
}

#[test]
fn two_balls_on_one_axis_touching_meet_at_a_point_and_crossing_along_a_circle() {
    let eps = scale().eps();
    let other =
        |h: f64, radius: f64| Surface::Sphere(Sphere::through(DVec3::ZERO, DVec3::Z, h, radius));
    for (h, radius) in [
        (10.0, 2.0),
        (10.0 + eps / 2.0, 2.0),
        (6.0, 2.0),
        (6.0, 2.0 + eps / 2.0),
    ] {
        let found = between(&ball(), &other(h, radius));
        let Relation::Apex(apex) = found else {
            panic!("a touch at the pole expected for {h} {radius}, got {found:?}");
        };
        assert!(apex.distance(DVec3::new(0.0, 0.0, 8.0)) <= eps, "{apex}");
    }
    assert_eq!(
        between(&ball(), &other(10.0 + 2.0 * eps, 2.0)),
        Relation::Apart
    );
    assert_eq!(between(&ball(), &other(5.5, 1.0)), Relation::Apart);
    let found = rims(&ball(), &other(7.0, 3.0), 1e-12 * REACH);
    let [rim] = found.as_slice() else {
        panic!("one circle, got {found:?}");
    };
    assert!(!rim.touching);
    assert!(rim.circle.center.distance(DVec3::new(0.0, 0.0, 6.0)) < 1e-12 * REACH);
    assert!((rim.circle.radius - 8f64.sqrt()).abs() < 1e-12 * REACH);
}

#[test]
fn a_ball_touches_a_ring_about_its_axis_where_its_equator_meets_the_root_and_crosses_a_nearer_one()
{
    let Surface::Sphere(sphere) = ball() else {
        unreachable!()
    };
    let ring_of = |big: f64| {
        Surface::Torus(Torus::through(
            DVec3::ZERO,
            DVec3::Z,
            DVec2::new(5.0, big),
            1.0,
            DVec2::new(5.0, big + 1.0),
        ))
    };
    assert_eq!(
        rims(&ball(), &ring_of(4.0), 1e-12 * REACH),
        vec![Rim {
            circle: sphere.circle(5.0, 3.0),
            touching: true,
        }]
    );
    let found = rims(&ball(), &ring_of(3.5), 1e-12 * REACH);
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(found.iter().all(|rim| !rim.touching));
    assert_eq!(between(&ball(), &ring_of(6.0)), Relation::Apart);
}

#[test]
fn a_torus_sharing_the_tube_circle_is_one_surface_whatever_its_seam_or_the_way_its_axis_points() {
    let eps = scale().eps();
    for other in [
        Torus::through(
            DVec3::ZERO,
            DVec3::Z,
            DVec2::new(2.0, 4.0),
            1.0,
            DVec2::new(2.0, 3.0),
        ),
        Torus::through(
            DVec3::ZERO,
            -DVec3::Z,
            DVec2::new(-2.0, 4.0),
            1.0,
            DVec2::new(-2.0, 5.0),
        ),
        Torus::through(
            DVec3::new(0.0, eps / 3.0, 0.0),
            DVec3::Z,
            DVec2::new(2.0 + eps / 3.0, 4.0),
            1.0 - eps / 3.0,
            DVec2::new(2.0, 5.0),
        ),
    ] {
        assert_eq!(
            between(&ring(), &Surface::Torus(other)),
            Relation::Same { agree: true },
            "{other:?}"
        );
    }
    let wider = Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(2.0, 4.0 + 3.0 * eps),
        1.0,
        DVec2::new(2.0, 5.0),
    );
    let found = rims(&ring(), &Surface::Torus(wider), eps);
    assert_eq!(found.len(), 2, "{found:?}");
}

#[test]
fn a_spindle_torus_is_met_on_its_own_lobe_and_never_on_the_mirror_one() {
    let apple = Surface::Torus(Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(5.0, 1.0),
        2.0,
        DVec2::new(5.0, 3.0),
    ));
    let lemon = Surface::Torus(Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(5.0, -1.0),
        2.0,
        DVec2::new(5.0, 1.0),
    ));
    let level = plane(DVec3::new(0.0, 0.0, 5.0), DVec3::Z);
    for (round, radius) in [(&apple, 3.0), (&lemon, 1.0)] {
        let found = rims(round, &level, 1e-12 * REACH);
        let [rim] = found.as_slice() else {
            panic!("one circle, got {found:?}");
        };
        assert!(
            (rim.circle.radius - radius).abs() < 1e-12 * REACH,
            "{rim:?}"
        );
    }
    let narrow = cylinder(DVec3::ZERO, DVec3::Z, 0.5);
    for round in [&apple, &lemon] {
        let found = rims(round, &narrow, 1e-12 * REACH);
        assert_eq!(found.len(), 2, "{found:?}");
    }
    let wide = cylinder(DVec3::ZERO, DVec3::Z, 1.5);
    assert_eq!(between(&lemon, &wide), Relation::Apart);
    assert_eq!(rims(&apple, &wide, 1e-12 * REACH).len(), 2);
}

#[test]
fn a_cylinder_a_hair_off_coaxial_is_coaxial_with_a_ball_and_further_off_meets_it_in_two_loops_and_a_cone_further_off_is_unsupported()
 {
    let eps = scale().eps();
    let bore = cylinder(DVec3::new(eps / 2.0, 0.0, 0.0), DVec3::Z, 2.0);
    let found = rims(&ball(), &bore, eps);
    assert_eq!(found.len(), 2, "{found:?}");
    for off in [2.0 * eps, 0.5] {
        let bore = cylinder(DVec3::new(0.0, off, 0.0), DVec3::Z, 2.0);
        loops(&bore, 2, &format!("a bore {off} off the axis"));
    }
    let far = cylinder(DVec3::new(0.0, 30.0, 0.0), DVec3::Z, 2.0);
    assert_eq!(between(&ball(), &far), Relation::Apart);
    for off in [2.0 * eps, 0.5] {
        let point = Surface::Cone(Cone::through(
            DVec3::new(0.0, off, 0.0),
            DVec3::Z,
            [DVec2::new(10.0, 0.0), DVec2::new(0.0, 5.0)],
        ));
        assert_eq!(between(&ball(), &point), Relation::Unsupported, "{off}");
    }
}

/// How many points along a loop the polygon that stands for it runs through.
const SAMPLES: usize = 2048;

/// Cylinders of `radius` whose axes stand `off` from the ball's centre: along
/// the ball's axis, square to it and at two slants, each run back along its
/// own axis and set off the centre two ways.
fn off_the_centre(radius: f64, off: f64) -> Vec<Surface> {
    let middle = DVec3::new(0.0, 0.0, 5.0);
    [
        DVec3::Z,
        DVec3::X,
        DVec3::new(1.0, 2.0, 2.0),
        DVec3::new(-2.0, 1.0, 0.5),
    ]
    .into_iter()
    .flat_map(|axis| {
        let axis = axis.normalize();
        let square = axis.any_orthonormal_vector();
        let across = axis.cross(square);
        [square, (square - across * 2.0).normalize()]
            .map(|out| cylinder(middle + out * off - axis * 7.0, axis, radius))
    })
    .collect()
}

/// Where the ruling of `wall` at `angle` pierces `sphere`, and how far each
/// point stands along the ruling from the middle of the two: none where the
/// ruling passes the ball by.
fn pierced(wall: &Cylinder, sphere: &Sphere, angle: f64) -> Option<([DVec3; 2], f64)> {
    let (sin, cos) = angle.sin_cos();
    let foot = wall.origin + (wall.u * cos + wall.v * sin) * wall.radius;
    let from = foot - sphere.middle();
    let along = from.dot(wall.axis);
    let reach = along * along - from.length_squared() + sphere.radius * sphere.radius;
    (reach > 0.0).then(|| {
        let half = reach.sqrt();
        (
            [-along - half, -along + half].map(|t| foot + wall.axis * t),
            half,
        )
    })
}

/// How far `point` stands from the polygon through `samples`.
fn off_the_polygon(samples: &[DVec3], point: DVec3) -> f64 {
    samples
        .windows(2)
        .map(|pair| {
            let (from, run) = (pair[0], pair[1] - pair[0]);
            let share = if run.length_squared() > 0.0 {
                ((point - from).dot(run) / run.length_squared()).clamp(0.0, 1.0)
            } else {
                0.0
            };
            point.distance(from + run * share)
        })
        .fold(f64::INFINITY, f64::min)
}

/// Holds that the ball and `wall`, a cylinder of another axis, meet along
/// `count` loops: each closed after its period, its points on both surfaces,
/// and the loops together passing through every point where a ruling of the
/// cylinder pierces the ball, away from where a loop turns back along it —
/// the same whichever surface comes first.
fn loops(wall: &Surface, count: usize, what: &str) {
    let (Surface::Sphere(sphere), Surface::Cylinder(cylinder)) = (ball(), wall) else {
        unreachable!()
    };
    let found = between(&ball(), wall);
    let curves = found.curves();
    assert_eq!(
        curves.len(),
        count,
        "{what}: {count} loops expected, got {found:?}"
    );
    assert_on_both(&found, &ball(), wall, 1e-12 * REACH);
    let polygons: Vec<Vec<DVec3>> = curves
        .iter()
        .map(|curve| {
            let period = curve
                .period()
                .unwrap_or_else(|| panic!("{what}: a closed loop expected, got {curve:?}"));
            let (start, end) = (curve.point(0.0), curve.point(period));
            assert!(
                start.distance(end) < 1e-12 * REACH,
                "{what}: a loop starts at {start} and ends its period at {end}"
            );
            (0..=SAMPLES)
                .map(|step| curve.point(period * step as f64 / SAMPLES as f64))
                .collect()
        })
        .collect();
    let mut passed = 0;
    for step in 0..48 {
        let angle = TAU * f64::from(step) / 48.0;
        let Some((points, half)) = pierced(cylinder, &sphere, angle) else {
            continue;
        };
        if half < 0.1 {
            continue;
        }
        for point in points {
            let off = polygons
                .iter()
                .map(|polygon| off_the_polygon(polygon, point))
                .fold(f64::INFINITY, f64::min);
            assert!(
                off < 1e-3,
                "{what}: the ruling at {angle} pierces the ball at {point}, {off} off every loop"
            );
            passed += 1;
        }
    }
    assert!(
        passed >= 8,
        "{what}: only {passed} piercings to pass through"
    );
}

#[test]
fn a_cylinder_of_another_axis_taking_in_part_of_a_ball_meets_it_along_one_closed_loop() {
    for (radius, off) in [(2.0, 2.5), (4.0, 2.0), (0.5, 3.2), (1.0, 3.0)] {
        for wall in off_the_centre(radius, off) {
            loops(&wall, 1, &format!("radius {radius}, {off} off: {wall:?}"));
        }
    }
}

#[test]
fn a_cylinder_of_another_axis_passing_right_through_a_ball_meets_it_along_two_loops() {
    for (radius, off) in [(1.0, 1.5), (2.0, 0.5), (0.5, 0.25), (0.2, 2.7)] {
        for wall in off_the_centre(radius, off) {
            loops(&wall, 2, &format!("radius {radius}, {off} off: {wall:?}"));
        }
    }
}

#[test]
fn a_cylinder_of_another_axis_missing_a_ball_or_holding_it_inside_is_apart_from_it() {
    let eps = scale().eps();
    for (radius, off) in [
        (1.0, 5.0),
        (1.0, 4.0 + 2.0 * eps),
        (0.5, 30.0),
        (10.0, 2.0),
        (4.0, 0.5),
        (5.0, 2.0 - 2.0 * eps),
    ] {
        for wall in off_the_centre(radius, off) {
            assert_eq!(
                between(&ball(), &wall),
                Relation::Apart,
                "radius {radius}, {off} off: {wall:?}"
            );
        }
    }
}

#[test]
fn a_cylinder_of_another_axis_within_a_hair_of_touching_a_ball_is_unsupported() {
    let eps = scale().eps();
    for (radius, off) in [(1.0, 4.0), (1.0, 2.0), (2.0, 1.0), (5.0, 2.0)] {
        for hair in [0.0, eps / 2.0, -eps / 2.0] {
            for wall in off_the_centre(radius, off + hair) {
                assert_eq!(
                    between(&ball(), &wall),
                    Relation::Unsupported,
                    "radius {radius}, {} off, touching the ball: {wall:?}",
                    off + hair
                );
            }
        }
    }
}

#[test]
fn the_meridian_solver_gives_the_circle_plane_and_cylinder_give() {
    for axis in [
        DVec3::Z,
        DVec3::new(0.0, -1.0, 0.0),
        DVec3::new(2.0, -1.0, 3.0),
    ] {
        let through = DVec3::new(-4.0, 2.5, 1.0);
        let round = Surface::Sphere(Sphere::through(through, axis, 5.0, 3.0));
        for height in [3.0, 5.0, 6.5, 7.9] {
            let at = through + axis.normalize() * height;
            let level = plane(at, axis);
            let found = rims(&round, &level, 1e-12 * REACH);
            let [on_the_ball] = found.as_slice() else {
                panic!("a circle expected at {height}, got {found:?}");
            };
            let wall = cylinder(through, axis, on_the_ball.circle.radius);
            let Relation::Circle(on_the_wall) = relation(&wall, &level, scale()) else {
                panic!("a circle expected at {height}");
            };
            assert_eq!(on_the_ball.circle, on_the_wall, "{axis} {height}");
            let found = rims(&round, &wall, 1e-12 * REACH);
            assert!(
                found
                    .iter()
                    .any(|rim| rim.circle.radius == on_the_wall.radius
                        && rim.circle.center.distance(on_the_wall.center) < 1e-12 * REACH),
                "{found:?} against {on_the_wall:?}"
            );
        }
    }
}

#[test]
fn a_round_s_relation_is_the_same_whichever_comes_first() {
    let eps = scale().eps();
    let others = [
        plane(DVec3::new(0.0, 0.0, 6.0), DVec3::Z),
        plane(DVec3::new(0.0, 0.0, 8.0), -DVec3::Z),
        plane(DVec3::ZERO, DVec3::new(1.0, -1.0, 0.0)),
        plane(DVec3::new(0.0, 0.0, 5.0), DVec3::new(1.0, 0.0, 1.0)),
        cylinder(DVec3::new(eps / 3.0, 0.0, 0.0), -DVec3::Z, 2.0),
        cylinder(DVec3::ZERO, DVec3::Z, 3.0),
        cylinder(DVec3::ZERO, DVec3::X, 1.0),
        cone([10.0, 0.0], [0.0, 5.0]),
        cone([10.0, 0.0], [2.0, 6.0]),
        Surface::Sphere(Sphere::through(DVec3::ZERO, DVec3::Z, 7.0, 3.0)),
        Surface::Sphere(Sphere::through(DVec3::ZERO, DVec3::Z, 5.0, 3.0 + 3.0 * eps)),
        Surface::Torus(Torus::through(
            DVec3::new(0.0, eps / 3.0, 0.0),
            -DVec3::Z,
            DVec2::new(-5.0, 4.0),
            1.0,
            DVec2::new(-5.0, 5.0),
        )),
        ball(),
        ring(),
    ];
    for round in [ball(), ring()] {
        for other in &others {
            between(&round, other);
        }
    }
}

/// Holds that a plane cuts a ball along one circle, about the foot of the
/// ball's centre on the plane, square to the plane, of radius √(r² − d²),
/// `d` the centre's distance from the plane; that the circle lies on both
/// surfaces; and that no point of it need be a vertex — the same whichever
/// surface comes first.
fn cut_about_the_foot(round: &Surface, flat: &Surface, what: &str) {
    let (Surface::Sphere(sphere), Surface::Plane(cutting)) = (round, flat) else {
        unreachable!()
    };
    let middle = sphere.middle();
    let standing = cutting.distance(middle);
    let foot = middle - cutting.normal * standing;
    let radius = (sphere.radius * sphere.radius - standing * standing).sqrt();
    let found = between(round, flat);
    let [Curve::Circle(circle)] = found.curves()[..] else {
        panic!("{what}: one circle expected, got {found:?}");
    };
    assert!(
        circle.center.distance(foot) < 1e-12 * REACH,
        "{what}: the circle is about {}, the foot of the centre on the plane is {foot}",
        circle.center
    );
    assert!(
        (circle.radius - radius).abs() < 1e-12 * REACH,
        "{what}: the circle's radius is {}, where √(r² − d²) is {radius}",
        circle.radius
    );
    assert!(
        circle.axis.cross(cutting.normal).length() < 1e-12,
        "{what}: the circle turns about {}, the plane faces {}",
        circle.axis,
        cutting.normal
    );
    assert!(
        found.points().is_empty(),
        "{what}: no point to be a vertex, got {found:?}"
    );
    assert_on_both(&found, round, flat, 1e-12 * REACH);
}

/// Unit normals at a slant to a ball's axis, along `axis`, and square to it.
fn slanted_and_square(axis: DVec3) -> Vec<DVec3> {
    let square = axis.any_orthonormal_vector();
    let across = axis.cross(square);
    [
        axis + square,
        axis * 2.0 - square * 3.0 + across,
        -axis * 4.0 + across,
        axis * 1e-6 + square,
        square,
        -across,
        (square + across) * 0.5,
    ]
    .map(|normal| normal.normalize())
    .to_vec()
}

#[test]
fn a_plane_at_a_slant_or_off_a_ball_s_axis_cuts_it_along_the_circle_about_the_foot_of_its_centre() {
    let eps = scale().eps();
    for axis in [DVec3::Z, DVec3::new(2.0, -1.0, 3.0).normalize()] {
        let through = DVec3::new(-4.0, 2.5, 1.0);
        let round = Surface::Sphere(Sphere::through(through, axis, 5.0, 3.0));
        let middle = through + axis * 5.0;
        for normal in slanted_and_square(axis) {
            let slanted = normal.dot(axis).abs() * 2.0 * REACH > eps;
            for standing in [0.0, 1.0, -2.5, 2.9, 3.0 - 1e-3] {
                if standing == 0.0 && !slanted {
                    continue;
                }
                for facing in [normal, -normal] {
                    let flat = plane(middle + normal * standing, facing);
                    cut_about_the_foot(
                        &round,
                        &flat,
                        &format!(
                            "about {axis}, a plane facing {facing} {standing} from the centre"
                        ),
                    );
                }
            }
        }
    }
}

#[test]
fn a_plane_at_a_slant_further_from_a_ball_s_centre_than_its_radius_misses_it() {
    for normal in slanted_and_square(DVec3::Z) {
        for standing in [3.0 + 1e-3, 3.5, -4.0, 30.0] {
            let flat = plane(DVec3::new(0.0, 0.0, 5.0) + normal * standing, normal);
            assert_eq!(
                between(&ball(), &flat),
                Relation::Apart,
                "a plane facing {normal} {standing} from the centre"
            );
        }
    }
}

/// Unit directions from a ball's centre to points of its sphere off its
/// poles, about `axis`: on its equator, and 30° and 60° from it towards
/// either pole, each along three ways round the axis; with the slanted and
/// square ones the sections are cut along, and every one of them turned the
/// other way.
fn off_the_poles(axis: DVec3) -> Vec<DVec3> {
    let square = axis.any_orthonormal_vector();
    let across = axis.cross(square);
    let (sin, cos) = 250f64.to_radians().sin_cos();
    let ways = [square, across, square * cos + across * sin];
    [0.0f64, 30.0, 60.0, -30.0, -60.0]
        .into_iter()
        .flat_map(|latitude| {
            let (sin, cos) = latitude.to_radians().sin_cos();
            ways.map(|way| way * cos + axis * sin)
        })
        .chain(slanted_and_square(axis))
        .flat_map(|towards| [towards, -towards])
        .collect()
}

/// A ball of radius 3 about `axis`, its centre 5 along it from a point off
/// the origin, and that centre.
fn ball_about(axis: DVec3) -> (Surface, DVec3) {
    let through = DVec3::new(-4.0, 2.5, 1.0);
    (
        Surface::Sphere(Sphere::through(through, axis, 5.0, 3.0)),
        through + axis * 5.0,
    )
}

/// The axes a ball is turned about: one of the frame's, and one leaning
/// from all three.
const AXES: [DVec3; 2] = [DVec3::Z, DVec3::new(2.0, -1.0, 3.0)];

#[test]
fn a_plane_within_half_the_tolerance_of_touching_a_ball_off_its_poles_touches_it_at_that_point_alone()
 {
    let eps = scale().eps();
    for axis in AXES.map(DVec3::normalize) {
        let (round, middle) = ball_about(axis);
        for towards in off_the_poles(axis) {
            let touch = middle + towards * 3.0;
            for standing in [3.0 - eps / 2.0, 3.0, 3.0 + eps / 2.0] {
                for facing in [towards, -towards] {
                    let flat = plane(middle + towards * standing, facing);
                    let what = format!(
                        "about {axis}, a plane facing {facing}, {standing} from the centre \
                         towards {towards}"
                    );

                    let found = between(&round, &flat);

                    assert!(found.curves().is_empty(), "{what}: no curve, got {found:?}");
                    let [at] = found.points()[..] else {
                        panic!("{what}: the one point of touch expected, got {found:?}");
                    };
                    assert!(
                        at.distance(touch) <= eps,
                        "{what}: touches at {at}, {} from {touch}, the ball's point square to \
                         the plane",
                        at.distance(touch)
                    );
                    for surface in [&round, &flat] {
                        assert!(
                            surface.distance(at).abs() <= eps,
                            "{what}: {at} stands {} off {surface:?}",
                            surface.distance(at)
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn a_plane_twice_the_tolerance_beyond_touching_a_ball_off_its_poles_misses_it_and_twice_within_cuts_a_small_circle()
 {
    let eps = scale().eps();
    for axis in AXES.map(DVec3::normalize) {
        let (round, middle) = ball_about(axis);
        for towards in off_the_poles(axis) {
            for facing in [towards, -towards] {
                let beyond = plane(middle + towards * (3.0 + 2.0 * eps), facing);
                assert_eq!(
                    between(&round, &beyond),
                    Relation::Apart,
                    "about {axis}, a plane facing {facing} twice the tolerance beyond the ball \
                     towards {towards}"
                );
                let within = plane(middle + towards * (3.0 - 2.0 * eps), facing);
                cut_about_the_foot(
                    &round,
                    &within,
                    &format!(
                        "about {axis}, a plane facing {facing} twice the tolerance within the \
                         ball towards {towards}"
                    ),
                );
            }
        }
    }
}

#[test]
fn every_other_pair_with_a_round_is_unsupported_and_none_is_at_a_slant() {
    let eps = scale().eps();
    let planes = [
        plane(DVec3::ZERO, DVec3::new(1.0, 0.0, 1.0)),
        plane(DVec3::new(1.0, 0.0, 0.0), DVec3::X),
        plane(DVec3::new(0.0, 0.0, 5.0), DVec3::new(1.0, 0.0, 1e-6)),
    ];
    let walls = [
        cylinder(DVec3::ZERO, DVec3::X, 2.0),
        cylinder(DVec3::new(0.0, 2.0 * eps, 0.0), DVec3::Z, 2.0),
    ];
    let others = [
        Surface::Cone(Cone::through(
            DVec3::ZERO,
            DVec3::X,
            [DVec2::new(0.0, 1.0), DVec2::new(REACH, 3.0)],
        )),
        Surface::Elliptic(EllipticCylinder::about(
            DVec3::ZERO,
            DVec3::Z,
            DVec3::X * 3.0,
            2.0,
        )),
    ];
    let one_centre_about_x = Surface::Sphere(Sphere::through(
        DVec3::new(0.0, 0.0, 5.0),
        DVec3::X,
        0.0,
        3.0,
    ));
    for (round, unsupported) in [
        (ball(), others.to_vec()),
        (
            ring(),
            planes
                .iter()
                .chain(&walls)
                .chain(&others)
                .copied()
                .collect(),
        ),
    ] {
        for other in &unsupported {
            assert_eq!(
                between(&round, other),
                Relation::Unsupported,
                "{round:?} {other:?}"
            );
        }
        for other in planes
            .iter()
            .chain(&walls)
            .chain(&others)
            .chain([&one_centre_about_x])
        {
            assert!(!at_a_slant(&round, other, scale()));
            assert!(!at_a_slant(other, &round, scale()));
        }
    }
}

/// The sphere about `(0, 0, h)` of `radius`, turned about `axis`: at
/// `h = 5`, of [`ball`]'s centre.
fn ball_on_z_about(axis: DVec3, h: f64, radius: f64) -> Surface {
    Surface::Sphere(Sphere::through(DVec3::new(0.0, 0.0, h), axis, 0.0, radius))
}

/// Axes through a sphere's centre other than [`ball`]'s: square to it, at a
/// slant, and square to it the other way round.
const OTHER_AXES: [DVec3; 3] = [DVec3::X, DVec3::new(2.0, -1.0, 3.0), DVec3::NEG_Y];

#[test]
fn two_balls_of_one_centre_and_radius_are_one_surface_whatever_axes_they_were_turned_about_and_a_hair_apart_in_radius_are_not()
 {
    let eps = scale().eps();
    for axis in OTHER_AXES.map(DVec3::normalize) {
        for radius in [3.0, 3.0 + eps / 2.0] {
            assert_eq!(
                between(&ball(), &ball_on_z_about(axis, 5.0, radius)),
                Relation::Same { agree: true },
                "the ball about z and the sphere of its centre and radius {radius} about {axis}"
            );
        }
        for radius in [3.0 + 3.0 * eps, 1.0] {
            assert_eq!(
                between(&ball(), &ball_on_z_about(axis, 5.0, radius)),
                Relation::Apart,
                "the ball about z and the sphere of its centre and radius {radius} about {axis}"
            );
        }
    }
}

/// The [`ring`] against spheres whose centres its axis holds, turned about
/// another axis: about the tube's height, touching its root along one
/// circle, crossing the tube along two, and holding it inside; and
/// [`ball`]'s, clear above it. Each meets it as the same sphere turned about
/// the ring's axis does, in either order: apart, or along the same circles,
/// touching where those touch, on both surfaces — but the root circle, the
/// sphere's equator about z, which runs through the poles of its charts
/// about x and about y, and is unsupported there.
#[test]
fn a_ball_whose_centre_a_ring_s_axis_holds_meets_it_whatever_axis_it_was_turned_about_as_it_does_turned_about_the_ring_s()
 {
    let mut declined = 0;
    for (h, radius, circles) in [(2.0, 3.0, 1), (2.0, 3.5, 2), (2.0, 6.0, 0), (5.0, 3.0, 0)] {
        let coaxial = between(&ring(), &ball_on_z_about(DVec3::Z, h, radius));
        assert_eq!(
            coaxial.curves().len(),
            circles,
            "the ring and the sphere about (0, 0, {h}) of {radius}, about z: {coaxial:?}"
        );
        for axis in OTHER_AXES.map(DVec3::normalize) {
            let other = ball_on_z_about(axis, h, radius);
            declined += usize::from(as_read_about_the_axis(
                [&ring(), &other],
                &coaxial,
                (DVec3::Z, 1e-12 * REACH),
                &format!("the ring and the sphere about (0, 0, {h}) of {radius}, about {axis}"),
            ));
        }
    }
    assert_eq!(
        declined, 2,
        "the root circle, the equator about z of the sphere about (0, 0, 2) of 3, runs through \
         the poles of its charts about x and about y, and of none other"
    );
}

/// Whether a point of one of `rims` stands nearer a pole of `sphere`'s
/// chart than the drawing lays a circle of another axis, [`FRONT`]: within
/// the angle whose sine it is of the chart's axis, read at a tenth of a
/// degree all round each rim.
fn near_a_pole_of_its_chart(sphere: &Sphere, rims: &[Rim]) -> bool {
    let beyond = (1.0 - FRONT * FRONT).sqrt();
    rims.iter().any(|rim| {
        (0..3600).any(|step| {
            let point = rim.circle.point(TAU * f64::from(step) / 3600.0);
            ((point - sphere.middle()).dot(sphere.axis) / sphere.radius).abs() > beyond
        })
    })
}

/// Holds that `pair`, a sphere and a round whose axis holds its centre,
/// along `along`, the two of other axes, relate as `about` says the sphere
/// read about `along` relates to the round, in either order: apart, one
/// surface, touching at the same point, or meeting along the same circles,
/// touching where those touch, on both surfaces within `within`. Where one
/// of those circles comes nearer a pole of the sphere's own chart than the
/// drawing lays a circle of another axis (#962), the pair is unsupported:
/// through such a pole no trace of the sphere reads the circle, and the
/// drawing leaves the face it bounds open. Whether it was so.
fn as_read_about_the_axis(
    pair: [&Surface; 2],
    about: &Relation,
    (along, within): (DVec3, f64),
    what: &str,
) -> bool {
    let eps = scale().eps();
    let found = between(pair[0], pair[1]);
    let sphere = pair.iter().find_map(|surface| match surface {
        Surface::Sphere(sphere) if sphere.axis.cross(along).length() > 1e-12 => Some(sphere),
        _ => None,
    });
    if let (Some(sphere), Relation::Rims(rims)) = (sphere, about)
        && near_a_pole_of_its_chart(sphere, rims)
    {
        assert_eq!(
            found,
            Relation::Unsupported,
            "{what}: read about the axis it meets it along {rims:?}, near a pole of its chart"
        );
        return true;
    }
    match (about, &found) {
        (Relation::Apart, Relation::Apart) => {}
        (Relation::Same { agree }, Relation::Same { agree: found }) if agree == found => {}
        (Relation::Apex(expected), Relation::Apex(found)) => assert!(
            found.distance(*expected) <= eps,
            "{what}: touches at {found}, where read about the axis it touches at {expected}"
        ),
        (Relation::Rims(expected), Relation::Rims(_)) => {
            let mut found = rims(pair[0], pair[1], within);
            let mut expected = expected.clone();
            for rims in [&mut found, &mut expected] {
                rims.sort_by(|one, other| {
                    let height = one.circle.center.dot(along);
                    let height = height.total_cmp(&other.circle.center.dot(along));
                    height.then(one.circle.radius.total_cmp(&other.circle.radius))
                });
            }
            assert_eq!(found.len(), expected.len(), "{what}: {found:?}");
            for (found, expected) in found.iter().zip(&expected) {
                assert_eq!(found.touching, expected.touching, "{what}: {found:?}");
                assert!(
                    found.circle.center.distance(expected.circle.center) <= eps
                        && (found.circle.radius - expected.circle.radius).abs() <= eps
                        && found.circle.axis.cross(expected.circle.axis).length() <= 1e-12,
                    "{what}: {found:?}, where read about the axis it meets it along {expected:?}"
                );
            }
        }
        _ => panic!("{what}: {found:?}, where read about the axis it is {about:?}"),
    }
    false
}

/// A cone through `(0, 0, 5)`, [`ball`]'s centre, along `axis`, through two
/// corners `(h, ρ)` read from that centre.
fn cone_through_the_centre(axis: DVec3, corners: [[f64; 2]; 2]) -> Surface {
    Surface::Cone(Cone::through(
        DVec3::new(0.0, 0.0, 5.0),
        axis,
        corners.map(DVec2::from),
    ))
}

/// [`ball`], turned about z, against cones whose axis runs through its
/// centre along another: crossing it along two circles, touching it along
/// one, resting on it at a point, and clear of it — the cones of
/// `a_coaxial_cone_cuts_a_ball_along_two_circles_touches_it_along_one_or_rests_on_its_pole`,
/// laid along the other axis. Each relates to it as it relates to the ball
/// turned about that axis, in either order.
#[test]
fn a_cone_whose_axis_runs_through_a_ball_s_centre_meets_it_whatever_axis_the_ball_was_turned_about_as_it_meets_the_ball_turned_about_the_cone_s()
 {
    for (corners, circles) in [
        ([[5.0, 0.0], [-5.0, 5.0]], 2),
        ([[5.0, 0.0], [-3.0, 6.0]], 1),
        ([[3.0, 0.0], [5.0, 4.0]], 1),
        ([[7.0, 0.0], [15.0, 2.0]], 0),
    ] {
        for axis in OTHER_AXES.map(DVec3::normalize) {
            let cone = cone_through_the_centre(axis, corners);
            let about = between(&ball_on_z_about(axis, 5.0, 3.0), &cone);
            assert_eq!(
                about.curves().len() + about.points().len(),
                circles,
                "the ball turned about {axis} and the cone through {corners:?} along it: \
                 {about:?}"
            );
            let declined = as_read_about_the_axis(
                [&ball(), &cone],
                &about,
                (axis, 1e-12 * REACH),
                &format!("the ball about z and the cone through {corners:?} along {axis}"),
            );
            assert!(
                !declined,
                "the cone through {corners:?} along {axis} meets the ball off its chart's poles"
            );
        }
    }
}

/// [`ball`], turned about z, against cylinders whose axis runs through its
/// centre along another, or within half the tolerance of it: crossing it
/// along two circles, touching it at its equator about that axis, within
/// twice the tolerance of touching it inside, and apart a hair outside it
/// and further. Each relates to it as it relates to the ball turned about
/// that axis, in either order — but where the circles run within the
/// drawing's reach of the ball's poles about z, along x and along y, the
/// touch at the great circle through them and the two circles a hair either
/// side of it, which are unsupported; twice the tolerance off the centre, a
/// cylinder of another axis meets it along the two loops its rulings
/// pierce it along (#805), on both surfaces within the tolerance — at a
/// slant so near the centre, a loop's points stand a fifth of it off the
/// sphere, where `loops` holds them to a trillionth of the reach.
#[test]
fn a_cylinder_whose_axis_runs_through_a_ball_s_centre_meets_it_whatever_axis_the_ball_was_turned_about_as_it_meets_the_ball_turned_about_the_cylinder_s()
 {
    let eps = scale().eps();
    let mut declined = 0;
    for axis in OTHER_AXES.map(DVec3::normalize) {
        let square = axis.any_orthonormal_vector();
        for (off, radius, circles) in [
            (0.0, 2.0, 2),
            (eps / 2.0, 2.0, 2),
            (0.0, 3.0, 1),
            (eps / 2.0, 3.0, 1),
            (0.0, 3.0 - 2.0 * eps, 2),
            (0.0, 3.0 + 2.0 * eps, 0),
            (0.0, 4.0, 0),
        ] {
            let wall = cylinder(DVec3::new(0.0, 0.0, 5.0) + square * off, axis, radius);
            let about = between(&ball_on_z_about(axis, 5.0, 3.0), &wall);
            assert_eq!(
                about.curves().len(),
                circles,
                "the ball turned about {axis} and the cylinder of {radius} {off:e} off its \
                 centre along it: {about:?}"
            );
            declined += usize::from(as_read_about_the_axis(
                [&ball(), &wall],
                &about,
                (axis, eps),
                &format!(
                    "the ball about z and the cylinder of {radius} along {axis}, {off:e} off its \
                     centre"
                ),
            ));
        }
        let off = cylinder(DVec3::new(0.0, 0.0, 5.0) + square * (2.0 * eps), axis, 2.0);
        let found = between(&ball(), &off);
        assert!(
            matches!(&found, Relation::Meet(_)) && found.curves().len() == 2,
            "a cylinder of 2 along {axis}, twice the tolerance off the ball's centre: {found:?}"
        );
        assert_on_both(&found, &ball(), &off, eps);
    }
    assert_eq!(
        declined, 6,
        "the cylinders of 3 along x and along y, through the centre or half the tolerance off it, \
         touch the ball along a great circle through its poles, and the one twice the tolerance \
         under crosses it along two within a few ten-thousandths of them; along the slant, none"
    );
}
