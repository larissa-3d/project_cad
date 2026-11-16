//! Where a circle lying on a sphere about another line crosses the
//! half-plane of an angle of the sphere's grid (#940), read without a body.

use std::f64::consts::TAU;

use glam::DVec3;

use super::*;

/// The circle a plane `offset` from `centre` along `normal` cuts the sphere
/// of `radius` about `centre` along.
fn cut_along(centre: DVec3, radius: f64, normal: DVec3, offset: f64) -> Circle {
    let normal = normal.normalize();
    Circle::about(
        centre + normal * offset,
        normal,
        (radius * radius - offset * offset).sqrt(),
    )
}

/// How many times `circle` passes through the half-plane from the line
/// through `origin` along `axis` out along `way`, counted from a dense run of
/// its points: each change of side of the plane, where the point stands out
/// along `way`.
fn passes(circle: &Circle, (origin, axis): (DVec3, DVec3), way: DVec3) -> usize {
    let across = axis.cross(way);
    let steps = 1 << 16;
    let at = |step: usize| circle.center + circle.radius * direction(circle, step, steps);
    (0..steps)
        .filter(|&step| {
            let (from, to) = (at(step), at(step + 1));
            let sides = (from - origin).dot(across) * (to - origin).dot(across);
            sides <= 0.0 && (from - origin).dot(way) > 0.0
        })
        .count()
}

fn direction(circle: &Circle, step: usize, steps: usize) -> DVec3 {
    let angle = TAU * step as f64 / steps as f64;
    circle.u * angle.cos() + circle.v * angle.sin()
}

/// Holds that every direction `crossings` gives leads from the circle's
/// centre to a point of it on the half-plane, out along `way`.
fn on_the_half_plane(
    circle: &Circle,
    (origin, axis): (DVec3, DVec3),
    way: DVec3,
    found: &[DVec3],
    what: &str,
) {
    for ray in found {
        let point = circle.center + *ray * circle.radius;
        assert!(
            (ray.length() - 1.0).abs() <= 1e-12 && ray.dot(circle.axis).abs() <= 1e-12,
            "{what}: {ray} leads off the circle"
        );
        assert!(
            (point - origin).dot(axis.cross(way)).abs() <= 1e-9,
            "{what}: {point} stands off the plane of the angle"
        );
        assert!(
            (point - origin).dot(way) > 0.0,
            "{what}: {point} stands behind the axis"
        );
    }
}

/// The sphere of radius 5 about the origin, about Y, and the circle x = 3 on
/// it, about (3, 0, 0) of radius 4: the half-plane out along X it crosses at
/// (3, ±4, 0), the one along (1, 0, 1) at (3, ±√7, 3), and a hair inside the
/// angle at which it touches, at (3, 0, 4), twice close by that point. It
/// crosses the plane holding the axis and X, but behind the axis, nowhere
/// along −X; it passes the plane x = 0 by, nowhere along Z; and a hair past
/// the touch, nowhere.
#[test]
fn a_circle_beside_a_sphere_s_axis_crosses_an_angle_s_half_plane_twice_where_it_reaches_it_and_nowhere_behind_the_axis_or_past_its_reach()
 {
    let sphere = (DVec3::ZERO, DVec3::Y);
    let circle = cut_along(DVec3::ZERO, 5.0, DVec3::X, 3.0);
    let near = |found: &[DVec3], expected: [DVec3; 2], what: &str| {
        let mut found = found.to_vec();
        found.sort_by(|one, other| one.y.total_cmp(&other.y));
        assert!(
            found.len() == 2
                && (found[0] - expected[0]).length() <= 1e-12
                && (found[1] - expected[1]).length() <= 1e-12,
            "{what}: {found:?}, where {expected:?}"
        );
    };
    let along_x = crossings(&circle, sphere, DVec3::X);
    near(&along_x, [DVec3::NEG_Y, DVec3::Y], "along X");
    let slant = DVec3::new(1.0, 0.0, 1.0).normalize();
    near(
        &crossings(&circle, sphere, slant),
        [
            DVec3::new(0.0, -7f64.sqrt(), 3.0) / 4.0,
            DVec3::new(0.0, 7f64.sqrt(), 3.0) / 4.0,
        ],
        "along (1, 0, 1)",
    );
    let inside = DVec3::new(3.0, 0.0, 3.99).normalize();
    let close = crossings(&circle, sphere, inside);
    assert_eq!(close.len(), 2, "a hair inside the touch: {close:?}");
    for ray in &close {
        assert!(
            ray.distance(DVec3::Z) <= 0.1,
            "a hair inside the touch: {ray}"
        );
    }
    on_the_half_plane(&circle, sphere, inside, &close, "a hair inside the touch");
    for (way, what) in [
        (DVec3::NEG_X, "along −X"),
        (DVec3::Z, "along Z"),
        (DVec3::NEG_Z, "along −Z"),
        (
            DVec3::new(3.0, 0.0, 4.01).normalize(),
            "a hair past the touch",
        ),
    ] {
        assert_eq!(
            crossings(&circle, sphere, way),
            Vec::<DVec3>::new(),
            "{what}"
        );
    }
}

/// A sphere of radius 5 off the origin, read about a line at a slant, and
/// circles of other axes on it: one beside its axis, crossing some angles'
/// half-planes twice and the others not at all; one winding round it,
/// crossing each once; and one a cap holding a pole, nearly a latitude, which
/// winds round it too. At each of 64 angles of the grid, as many crossings as
/// a dense run of the circle's points passes through, each a point of the
/// circle on that half-plane.
#[test]
fn a_circle_on_a_sphere_about_another_line_crosses_each_angle_s_half_plane_as_often_as_it_passes_through_it()
 {
    let centre = DVec3::new(28.0, 2.0, 13.0);
    let axis = DVec3::new(1.0, 2.0, -3.0).normalize();
    let first = axis.any_orthonormal_vector();
    let second = axis.cross(first);
    for (normal, offset, winds, what) in [
        (first + axis * 0.2, 3.0, false, "beside the axis"),
        (axis + first * 0.3, 1.0, true, "winding round the axis"),
        (axis + second * 0.1, 4.0, true, "a cap holding a pole"),
    ] {
        let circle = cut_along(centre, 5.0, normal, offset);
        let (mut twice, mut missed) = (0, 0);
        for step in 0..64 {
            let angle = TAU * f64::from(step) / 64.0;
            let way = first * angle.cos() + second * angle.sin();
            let found = crossings(&circle, (centre, axis), way);
            let what = format!("{what}, at the angle {angle}");
            assert_eq!(
                found.len(),
                passes(&circle, (centre, axis), way),
                "{what}: {found:?}"
            );
            on_the_half_plane(&circle, (centre, axis), way, &found, &what);
            match found.len() {
                0 => missed += 1,
                2 => twice += 1,
                _ => {}
            }
        }
        if winds {
            assert_eq!((twice, missed), (0, 0), "{what}: each angle crossed once");
        } else {
            assert!(
                twice > 0 && missed > 0 && twice + missed == 64,
                "{what}: {twice} angles crossed twice, {missed} not at all, out of 64"
            );
        }
    }
}
