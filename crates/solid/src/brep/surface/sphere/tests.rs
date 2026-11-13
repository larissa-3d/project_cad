use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::Sphere;
use crate::brep::curve::Circle;
use crate::brep::scale::Scale;
use crate::brep::surface::Cylinder;

const CLOSE: f64 = 1e-12;

/// A ball of radius 3 about `(h = 5)` on a slanted axis through a point off
/// the origin.
fn slanted() -> Sphere {
    Sphere::through(
        DVec3::new(1.0, -2.0, 3.0),
        DVec3::new(0.0, 1.0, 1.0),
        5.0,
        3.0,
    )
}

#[test]
fn a_sphere_shares_origin_and_reference_with_a_cylinder_of_its_axis() {
    let point = DVec3::new(2.0, -5.0, 1.0);
    let axis = DVec3::new(1.0, 2.0, -0.5);
    let sphere = Sphere::through(point, axis, 4.0, 2.0);
    let cylinder = Cylinder::about(point, axis, 4.0);
    assert_eq!(
        [sphere.origin, sphere.axis, sphere.u, sphere.v],
        [cylinder.origin, cylinder.axis, cylinder.u, cylinder.v]
    );
    assert_eq!(sphere.circle(3.0, 4.0), Circle::on(&cylinder, 3.0));
}

#[test]
fn a_sphere_turned_about_the_axis_either_way_is_the_same_bits() {
    let point = DVec3::new(3.0, 7.0, -2.0);
    let axis = DVec3::new(0.3, -1.0, 0.2);
    let forwards = Sphere::through(point, axis, 2.5, 1.5);
    let backwards = Sphere::through(point, -axis, -2.5, 1.5);
    assert_eq!(forwards, backwards);
    assert!(forwards.middle().distance(point + axis.normalize() * 2.5) < CLOSE);
}

#[test]
fn a_point_of_a_sphere_reads_back_its_parameters() {
    let sphere = slanted();
    for at in [
        DVec2::new(0.3, 0.2),
        DVec2::new(-2.9, -1.2),
        DVec2::new(3.1, 1.5),
        DVec2::new(1.0, 0.0),
    ] {
        let point = sphere.point(at);
        assert!(sphere.distance(point).abs() < CLOSE, "{at}");
        let back = sphere.parameters(point);
        assert!((back - at).length() < CLOSE, "{at} came back as {back}");
        assert!(sphere.nearest(point).distance(point) < CLOSE);
    }
}

#[test]
fn a_sphere_s_parameters_turn_as_its_own_normal_the_poles_included() {
    let step = 1e-6;
    let sphere = slanted();
    for at in [
        DVec2::new(0.4, 0.3),
        DVec2::new(-2.0, -1.0),
        DVec2::new(1.0, FRAC_PI_2 - 1e-3),
        DVec2::new(1.0, -FRAC_PI_2 + 1e-3),
    ] {
        let here = sphere.point(at);
        let round = sphere.point(at + DVec2::new(step, 0.0)) - here;
        let along = sphere.point(at + DVec2::new(0.0, step)) - here;
        let turned = round.cross(along).normalize();
        let normal = sphere.normal(at);
        assert!((normal.length() - 1.0).abs() < CLOSE);
        assert!(
            turned.distance(normal) < 1e-5,
            "{turned} against {normal} at {at}"
        );
        assert!(
            normal.dot(here - sphere.middle()) > 0.0,
            "the normal points into the ball at {at}"
        );
    }
    for pole in [-FRAC_PI_2, FRAC_PI_2] {
        let normal = sphere.normal(DVec2::new(0.7, pole));
        assert!(normal.distance(sphere.axis * pole.signum()) < CLOSE);
    }
}

#[test]
fn a_sphere_s_distance_is_signed_outward() {
    let sphere = slanted();
    let middle = sphere.middle();
    let away = DVec3::new(1.0, 2.0, 3.0).normalize();
    assert!((sphere.distance(middle + away * 4.0) - 1.0).abs() < CLOSE);
    assert!((sphere.distance(middle + away * 2.0) + 1.0).abs() < CLOSE);
    assert!((sphere.distance(middle) + 3.0).abs() < CLOSE);
    assert_eq!(sphere.poles(), [-FRAC_PI_2, FRAC_PI_2]);
}

#[test]
fn a_latitude_circle_is_held_and_a_meridian_is_told_from_it() {
    let sphere = slanted();
    let eps = 1e-8;
    let phi: f64 = 0.4;
    let (h, rho) = (sphere.center + 3.0 * phi.sin(), 3.0 * phi.cos());
    let latitude = sphere.circle(h, rho);
    assert!(sphere.holds(&latitude, eps));
    assert!(!sphere.meridian(&latitude, eps));
    assert!(!sphere.holds(&sphere.circle(h, rho + 10.0 * eps), eps));
    let mut leaning = latitude;
    leaning.axis = (leaning.axis + leaning.u * 1e-6).normalize();
    assert!(!sphere.holds(&leaning, eps));

    let meridian = Circle::about(sphere.middle(), sphere.u, 3.0);
    assert!(sphere.meridian(&meridian, eps));
    assert!(!sphere.holds(&meridian, eps));
    let off = Circle::about(sphere.middle() + sphere.u * 10.0 * eps, sphere.u, 3.0);
    assert!(!sphere.meridian(&off, eps));
    let beside = Circle::about(sphere.middle() + sphere.axis * 10.0 * eps, sphere.v, 3.0);
    assert!(!sphere.meridian(&beside, eps));
    let smaller = Circle::about(sphere.middle(), sphere.radial(0.3), 3.0 - 10.0 * eps);
    assert!(!sphere.meridian(&smaller, eps));
}

#[test]
fn a_circle_lying_on_the_sphere_is_carried_whatever_line_it_turns_about_and_one_off_it_is_not() {
    let sphere = slanted();
    let eps = 1e-8;
    let middle = sphere.middle();
    let tilted = DVec3::new(0.3, -0.5, 0.8).normalize();
    let great = Circle::about(middle, tilted, 3.0);
    assert!(
        !sphere.holds(&great, eps) && !sphere.meridian(&great, eps),
        "the great circle is tilted across the axis, neither a latitude nor a meridian"
    );
    assert!(sphere.carries(&great, eps), "a tilted great circle");
    let h: f64 = 1.2;
    let rho = (9.0 - h * h).sqrt();
    let small = Circle::about(middle + tilted * h, tilted, rho);
    assert!(
        !sphere.holds(&small, eps),
        "the small circle is no latitude"
    );
    assert!(sphere.carries(&small, eps), "a tilted small circle");

    let off = Circle::about(middle + tilted * (h + 10.0 * eps), tilted, rho);
    assert!(
        !sphere.carries(&off, eps),
        "a circle moved off the sphere along its own axis"
    );
    let wider = Circle::about(middle + tilted * h, tilted, rho + 10.0 * eps);
    assert!(!sphere.carries(&wider, eps), "a circle of the wrong radius");
    let aside = tilted.any_orthonormal_vector() * 10.0 * eps;
    let beside = Circle::about(middle + tilted * h + aside, tilted, rho);
    assert!(
        !sphere.carries(&beside, eps),
        "a circle whose centre stands off the line through the sphere's centre"
    );
    let great_beside = Circle::about(middle + aside, tilted, 3.0);
    assert!(
        !sphere.carries(&great_beside, eps),
        "a great circle off-centre"
    );
}

#[test]
fn a_line_crosses_a_ball_twice_touches_it_once_and_passes_it_by() {
    let sphere = slanted();
    let middle = sphere.middle();
    let direction = DVec3::new(1.0, 0.5, -0.2).normalize();
    let origin = middle - direction * 10.0;
    let [Some(near), Some(far)] = sphere.roots(origin, direction) else {
        panic!("a line through the centre crosses twice");
    };
    assert!((near - 7.0).abs() < CLOSE && (far - 13.0).abs() < CLOSE);
    for t in [near, far] {
        assert!(sphere.distance(origin + direction * t).abs() < CLOSE);
    }

    let across = direction.cross(sphere.axis).normalize();
    let grazing = sphere.roots(origin + across * 3.0, direction);
    let [Some(touch), None] = grazing else {
        panic!("a tangent line touches once: {grazing:?}");
    };
    assert!(
        sphere
            .distance(origin + across * 3.0 + direction * touch)
            .abs()
            < 1e-9
    );

    assert_eq!(
        sphere.roots(origin + across * (3.0 + 1e-9), direction),
        [None, None]
    );
    let [Some(_), Some(_)] = sphere.roots(origin + across * (3.0 - 1e-6), direction) else {
        panic!("a line a hair inside crosses twice");
    };
}

#[test]
fn a_sphere_reaches_its_radius_either_way_along_any_direction() {
    let sphere = slanted();
    let direction = DVec3::new(0.2, -0.7, 0.4).normalize();
    let [low, high] = sphere.extent(direction);
    let at = sphere.middle().dot(direction);
    assert!((high - (at + 3.0)).abs() < CLOSE && (low - (at - 3.0)).abs() < CLOSE);
    for theta in [-PI, -1.0, 0.5, 3.0] {
        let point = sphere.point(DVec2::new(theta, 0.2));
        let along = point.dot(direction);
        assert!(low - CLOSE <= along && along <= high + CLOSE);
    }
}

/// An angle moved by whole turns into `[-π, π)`.
fn wrapped(angle: f64) -> f64 {
    (angle + PI).rem_euclid(TAU) - PI
}

/// The circle the plane `standing` from the centre of `sphere` along the unit
/// `normal` cuts it along, turning about `normal`.
fn cut_along(sphere: &Sphere, normal: DVec3, standing: f64) -> Circle {
    Circle::about(
        sphere.middle() + normal * standing,
        normal,
        (sphere.radius * sphere.radius - standing * standing).sqrt(),
    )
}

/// Holds that `seen_along` reads `circle` on `sphere` as `parameters` reads
/// its points, all round it: θ the same but for whole turns, and never
/// jumping by half a turn or more from one sample to the next; φ the same;
/// and, wherever the point stands a tenth of the radius or more off the
/// axis, the first and second derivatives the central differences of the
/// point and of the first derivative give. Gives back how far θ turns round
/// the circle.
fn read_as_the_parameters_read_it(sphere: &Sphere, circle: &Circle, what: &str) -> f64 {
    const SAMPLES: usize = 4096;
    const STEP: f64 = 1e-6;
    let mut before: Option<f64> = None;
    for index in 0..=SAMPLES {
        let t = TAU * index as f64 / SAMPLES as f64;
        let [at, speed, bend] = sphere.seen_along(circle, t);
        let point = circle.point(t);
        let read = sphere.parameters(point);
        let from = point - sphere.middle();
        let rho = (from - sphere.axis * from.dot(sphere.axis)).length();
        assert!(
            wrapped(at.x - read.x).abs() <= CLOSE * (1.0 + sphere.radius / rho),
            "{what}: at {t}, θ is {}, where the parameters read {}",
            at.x,
            read.x
        );
        assert!(
            (at.y - read.y).abs() <= CLOSE,
            "{what}: at {t}, φ is {}, where the parameters read {}",
            at.y,
            read.y
        );
        if let Some(before) = before {
            assert!(
                (at.x - before).abs() < PI,
                "{what}: θ jumps from {before} to {} by {t}",
                at.x
            );
        }
        before = Some(at.x);
        if rho < 0.1 * sphere.radius {
            continue;
        }
        let [behind, ahead] = [t - STEP, t + STEP].map(|t| sphere.seen_along(circle, t));
        let drift = (ahead[0] - behind[0]) / (2.0 * STEP);
        assert!(
            (drift - speed).length() <= 1e-6 * (1.0 + speed.length()),
            "{what}: at {t}, the first derivative is {speed}, the differences give {drift}"
        );
        let sway = (ahead[1] - behind[1]) / (2.0 * STEP);
        assert!(
            (sway - bend).length() <= 1e-6 * (1.0 + bend.length()),
            "{what}: at {t}, the second derivative is {bend}, the differences give {sway}"
        );
    }
    sphere.seen_along(circle, TAU)[0].x - sphere.seen_along(circle, 0.0)[0].x
}

/// Which way round the sphere's axis a circle turns: `+1` where its own
/// turn, `u` towards `v`, runs about the axis as θ does, `-1` against it.
fn way(sphere: &Sphere, circle: &Circle) -> f64 {
    circle.u.cross(circle.v).dot(sphere.axis).signum()
}

#[test]
fn a_circle_at_a_slant_is_seen_in_the_sphere_s_parameters_with_the_derivatives_the_differences_give()
 {
    let sphere = slanted();
    let square = sphere.axis.any_orthonormal_vector();
    let across = sphere.axis.cross(square);
    let pierced = [
        ("a great circle at 45°", sphere.axis + square, 0.0),
        (
            "a circle at a slant the axis runs through",
            sphere.axis * 2.0 + square - across,
            1.0,
        ),
    ];
    let beside = [
        (
            "a circle at a slant the axis passes by",
            sphere.axis * 0.3 + square,
            2.0,
        ),
        ("a circle along the axis, off it", across, -2.0),
    ];
    for (turns, cases) in [(true, pierced), (false, beside)] {
        for (what, normal, standing) in cases {
            for facing in [1.0, -1.0] {
                let normal = normal.normalize() * facing;
                let circle = cut_along(&sphere, normal, standing * facing);
                let what = format!("{what}, turning about {normal}");
                assert!(
                    sphere.carries(&circle, CLOSE) && !sphere.through_a_pole(&circle, CLOSE),
                    "{what}: on the sphere, through neither pole"
                );

                let turned = read_as_the_parameters_read_it(&sphere, &circle, &what);

                let expected = if turns {
                    way(&sphere, &circle) * TAU
                } else {
                    0.0
                };
                assert!(
                    (turned - expected).abs() <= CLOSE,
                    "{what}: θ turns {turned} round it, where it turns {expected}"
                );
            }
        }
    }
}

#[test]
fn a_circle_ten_hairs_off_a_pole_is_seen_round_the_pole_or_turning_back_by_the_side_it_passes() {
    let sphere = slanted();
    let eps = Scale::of(10.0).eps();
    let square = sphere.axis.any_orthonormal_vector();
    let normal = (sphere.axis + square).normalize();
    for (side, pole) in [(1.0, sphere.axis), (-1.0, -sphere.axis)] {
        let through = sphere.radius * normal.dot(pole);
        let at_the_pole = cut_along(&sphere, normal, through);
        assert!(
            sphere.through_a_pole(&at_the_pole, eps),
            "the circle through the pole {side} is read as through it"
        );
        for (inward, turns) in [(-1.0, true), (1.0, false)] {
            let standing = through + side * inward * 10.0 * eps;
            for facing in [normal, -normal] {
                let circle = Circle::about(
                    sphere.middle() + normal * standing,
                    facing,
                    (sphere.radius * sphere.radius - standing * standing).sqrt(),
                );
                let what = format!(
                    "a circle ten hairs {} of the pole {side}, turning about {facing}",
                    if turns { "inside" } else { "outside" }
                );
                assert!(
                    !sphere.through_a_pole(&circle, eps),
                    "{what}: not read as through the pole"
                );

                let turned = read_as_the_parameters_read_it(&sphere, &circle, &what);

                let expected = if turns {
                    way(&sphere, &circle) * TAU
                } else {
                    0.0
                };
                assert!(
                    (turned - expected).abs() <= 1e-9,
                    "{what}: θ turns {turned} round it, where it turns {expected}"
                );
            }
        }
    }
}
