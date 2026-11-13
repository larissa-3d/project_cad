//! What a sphere or a torus makes with another surface (#576): the coaxial
//! pairs decided once in the meridian half-plane; a sphere and a plane at a
//! slant to its axis, or square to it off its centre, along the circle about
//! the foot of the centre on the plane (#805); everything else declined.

use glam::{DVec2, DVec3};

use super::{REACH, assert_on_both, cylinder, plane, scale};
use crate::brep::curve::{Circle, Curve};
use crate::brep::relation::{Relation, Rim, at_a_slant, relation};
use crate::brep::surface::{Cone, EllipticCylinder, Sphere, Surface, Torus};

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
fn a_cylinder_a_hair_off_coaxial_is_coaxial_with_a_ball_and_further_off_is_unsupported() {
    let eps = scale().eps();
    let bore = cylinder(DVec3::new(eps / 2.0, 0.0, 0.0), DVec3::Z, 2.0);
    let found = rims(&ball(), &bore, eps);
    assert_eq!(found.len(), 2, "{found:?}");
    for off in [2.0 * eps, 0.5, 30.0] {
        let bore = cylinder(DVec3::new(0.0, off, 0.0), DVec3::Z, 2.0);
        assert_eq!(between(&ball(), &bore), Relation::Unsupported, "{off}");
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

#[test]
fn a_plane_within_a_hair_of_touching_a_ball_off_its_poles_is_unsupported() {
    let eps = scale().eps();
    for normal in slanted_and_square(DVec3::Z) {
        for standing in [3.0 - eps / 2.0, 3.0, 3.0 + eps / 2.0, -3.0] {
            let flat = plane(DVec3::new(0.0, 0.0, 5.0) + normal * standing, normal);
            assert_eq!(
                between(&ball(), &flat),
                Relation::Unsupported,
                "a plane facing {normal} {standing} from the centre, touching the ball off its \
                 poles, which #939 is to read as a touch"
            );
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
    let others = [
        cylinder(DVec3::ZERO, DVec3::X, 2.0),
        cylinder(DVec3::new(0.0, 2.0 * eps, 0.0), DVec3::Z, 2.0),
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
        Surface::Sphere(Sphere::through(
            DVec3::new(0.0, 0.0, 5.0),
            DVec3::X,
            0.0,
            3.0,
        )),
    ];
    for (round, unsupported) in [
        (ball(), others.to_vec()),
        (ring(), planes.iter().chain(&others).copied().collect()),
    ] {
        for other in &unsupported {
            assert_eq!(
                between(&round, other),
                Relation::Unsupported,
                "{round:?} {other:?}"
            );
        }
        for other in planes.iter().chain(&others) {
            assert!(!at_a_slant(&round, other, scale()));
            assert!(!at_a_slant(other, &round, scale()));
        }
    }
}
