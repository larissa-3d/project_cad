//! Where a circle lying on a sphere about another line crosses the
//! half-plane of an angle of the sphere's grid (#940), read without a body.

use std::cell::RefCell;
use std::f64::consts::{FRAC_PI_4, TAU};

use glam::DVec3;

use super::*;
use crate::brep::surface::Sphere;

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

/// The circle about X of radius 4 its centre `height` along Z from (3, 0, 0),
/// its parameter's nought along Y: at `height` 0, the circle the plane
/// x = 3 cuts the sphere of radius 5 about the origin along.
fn about_x(height: f64, radius: f64) -> Circle {
    Circle {
        center: DVec3::new(3.0, 0.0, height),
        axis: DVec3::X,
        radius,
        u: DVec3::Y,
        v: DVec3::Z,
    }
}

/// The circle about X of radius 4 whose centre stands `height` from the
/// plane z = 0, either side of it, against the half-plane of that plane out
/// along X from the line along Y: it reaches past the plane by 4 − |height|.
/// By no more than the kernel's relative tolerance of its radius, a hair
/// inside it or a hair short of it, or exactly to it, it touches and takes no
/// crossing; twice that tolerance in, or more, it crosses twice.
#[test]
fn a_circle_reaching_past_a_half_plane_s_plane_by_no_more_than_the_kernel_s_tolerance_of_its_radius_takes_no_crossing_and_a_little_further_takes_two()
 {
    let line = (DVec3::ZERO, DVec3::Y);
    for side in [1.0, -1.0] {
        for (past, count) in [
            (0.5 * Scale::RELATIVE, 0),
            (0.0, 0),
            (-0.5 * Scale::RELATIVE, 0),
            (2.0 * Scale::RELATIVE, 2),
            (1e-6, 2),
        ] {
            let circle = about_x(side * 4.0 * (1.0 - past), 4.0);
            let found = crossings(&circle, line, DVec3::X);
            assert_eq!(
                found.len(),
                count,
                "a circle reaching {past} of its radius past the plane, on the side {side}: \
                 {found:?}"
            );
            on_the_half_plane(&circle, line, DVec3::X, &found, "a circle reaching past");
        }
    }
}

/// The sphere of radius 5 about the origin, about Y, and eight steps of its
/// grid, every eighth of a turn from X; read as the axes of a body holding
/// it, its steps its grid's rays too, with `group` the one group, of the wall
/// it names.
fn axes_of_a_sphere(group: Option<(SurfaceId, Vec<DVec3>)>) -> (Axes, Vec<DVec3>) {
    let round = Rounded::Sphere(Sphere::through(DVec3::ZERO, DVec3::Y, 0.0, 5.0));
    let steps: Vec<DVec3> = (0..8)
        .map(|step| {
            let angle = FRAC_PI_4 * f64::from(step);
            DVec3::new(angle.cos(), 0.0, angle.sin())
        })
        .collect();
    let mut axes = Axes {
        cones: vec![(Spun::Round(round), steps.clone())],
        steps: vec![(round, steps.clone())],
        groups: Vec::new(),
        of_wall: BTreeMap::new(),
    };
    if let Some((wall, rays)) = group {
        axes.groups.push(Group {
            axis: DVec3::X,
            u: DVec3::Y,
            v: DVec3::Z,
            rays,
        });
        axes.of_wall.insert(wall, 0);
    }
    (axes, steps)
}

/// What `axes` widen `circle` of `wall` by, from no ray of its own, at a
/// fineness of a hundredth over a span of 10, every place kept.
fn widened_alone(axes: &Axes, circle: &Circle, wall: SurfaceId) -> Option<Vec<DVec3>> {
    axes.widened(
        &Contact::default(),
        (circle, wall),
        (0.01, 10.0),
        Scale::of(5.0).eps(),
        |_| true,
    )
    .map(|contact| contact.rays)
}

/// Whether `found` and `expected` hold the same directions, each once.
fn same_rays(found: &[DVec3], expected: &[DVec3]) -> bool {
    found.len() == expected.len()
        && expected.iter().all(|way| {
            found
                .iter()
                .filter(|ray| ray.distance(*way) <= 1e-12)
                .count()
                == 1
        })
}

/// Every crossing of `circle` with the half-planes of the sphere's `steps`.
fn every_crossing(circle: &Circle, steps: &[DVec3]) -> Vec<DVec3> {
    steps
        .iter()
        .flat_map(|way| crossings(circle, (DVec3::ZERO, DVec3::Y), *way))
        .collect()
}

/// Against a sphere's grid, a circle lying on the sphere about another line,
/// x = 3, takes every crossing of the grid's steps and no other ray. A circle
/// the sphere does not carry, though it crosses those half-planes — x = 3 of
/// radius 2, inside the ball, or x = 9 of radius 4, on a ball of radius 5
/// about (12, 0, 0) — takes nothing; a latitude of the sphere takes its
/// grid's rays and no crossing; and the circle x = 3 standing in a group
/// takes the group's rays and no crossing.
#[test]
fn a_circle_is_widened_by_the_crossings_of_a_sphere_s_steps_only_when_the_sphere_carries_it_about_another_line_and_it_stands_in_no_group()
 {
    let (axes, steps) = axes_of_a_sphere(None);
    let wall = SurfaceId(7);
    let tilted = about_x(0.0, 4.0);
    let expected = every_crossing(&tilted, &steps);
    assert_eq!(expected.len(), 6, "x = 3 crosses three steps twice each");
    let found = widened_alone(&axes, &tilted, wall).expect("the tilted circle is widened");
    assert!(
        same_rays(&found, &expected),
        "the tilted circle takes {found:?}, where it crosses at {expected:?}"
    );
    for (circle, what) in [
        (about_x(0.0, 2.0), "x = 3 of radius 2, inside the ball"),
        (
            Circle {
                center: DVec3::new(9.0, 0.0, 0.0),
                ..about_x(0.0, 4.0)
            },
            "x = 9 of radius 4, on another ball",
        ),
    ] {
        assert!(
            !every_crossing(&circle, &steps).is_empty(),
            "{what} crosses the steps' half-planes"
        );
        assert_eq!(widened_alone(&axes, &circle, wall), None, "{what}");
    }
    let latitude = Circle::about(DVec3::new(0.0, 3.0, 0.0), DVec3::Y, 4.0);
    let found = widened_alone(&axes, &latitude, wall).expect("a latitude takes its grid");
    assert!(
        same_rays(&found, &steps),
        "the latitude takes {found:?}, where its grid is {steps:?}"
    );
    let rays = vec![DVec3::Y, DVec3::Z, DVec3::NEG_Y, DVec3::NEG_Z];
    let (grouped, _) = axes_of_a_sphere(Some((wall, rays.clone())));
    let found = widened_alone(&grouped, &tilted, wall).expect("a group widens its circle");
    assert!(
        same_rays(&found, &rays),
        "the tilted circle in a group takes {found:?}, where its group's rays are {rays:?}"
    );
}

/// The circle x = 3 against the sphere's grid, its parameter's nought along
/// Y, where it crosses the step along X: a crossing whose place `kept` says
/// no sample stands is left out, and every place it is asked about is the
/// circle's own point at that crossing; a crossing at a step of the circle's
/// grid its contact withholds at its level is left out. Every other crossing
/// is taken.
#[test]
fn a_crossing_whose_place_is_not_kept_or_whose_step_the_circle_withholds_is_left_out() {
    let (axes, steps) = axes_of_a_sphere(None);
    let circle = about_x(0.0, 4.0);
    let crossed = every_crossing(&circle, &steps);
    let eps = Scale::of(5.0).eps();
    let wall = SurfaceId(7);
    let dropped = crossed[0];
    let asked = RefCell::new(Vec::new());
    let widened = axes
        .widened(
            &Contact::default(),
            (&circle, wall),
            (0.01, 10.0),
            eps,
            |point| {
                asked.borrow_mut().push(point);
                point.distance(circle.center + dropped * circle.radius) > 1e-9
            },
        )
        .expect("the other crossings widen the circle");
    let others: Vec<DVec3> = crossed[1..].to_vec();
    assert!(
        same_rays(&widened.rays, &others),
        "with the place of {dropped} not kept, the circle takes {:?}, where {others:?}",
        widened.rays
    );
    let asked = asked.into_inner();
    assert_eq!(
        asked.len(),
        crossed.len(),
        "each crossing's place asked about once"
    );
    for point in asked {
        assert!(
            crossed
                .iter()
                .any(|way| point.distance(circle.center + *way * circle.radius) <= 1e-12),
            "the place {point} asked about is no crossing of the circle"
        );
    }

    let at_nought = Contact {
        withheld: vec![(0, [2.0, 4.0])],
        ..Contact::default()
    };
    let widened = axes
        .widened(&at_nought, (&circle, wall), (0.01, 10.0), eps, |_| true)
        .expect("the other crossings widen the circle");
    let others: Vec<DVec3> = crossed
        .iter()
        .copied()
        .filter(|way| way.distance(DVec3::Y) > 1e-12)
        .collect();
    assert_eq!(others.len(), crossed.len() - 1, "one crossing along Y");
    assert!(
        same_rays(&widened.rays, &others),
        "with the circle's step at nought withheld, it takes {:?}, where {others:?}",
        widened.rays
    );
}

/// A cone about z from a radius of 4 at h = 0 to 8 at h = 4, and a wall of
/// 6 about z meeting it along the circle at h = 2, the body's only circle:
/// no other wall stands close to the wall, and no circle stands a hair from
/// the cone between its rims. The wall takes no group of the cone's rays,
/// as it takes none where it stands close to nothing.
#[test]
fn a_wall_meeting_a_cone_along_its_one_circle_and_close_to_no_other_wall_takes_no_group_of_the_cone_s_rays()
 {
    use glam::DVec2;

    use crate::brep::surface::{Cone, Surface};
    use crate::brep::tessellation::tests::fixtures::Build;

    let mut build = Build::new();
    build.surface(Surface::Cone(Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, 4.0), DVec2::new(4.0, 8.0)],
    )));
    let wall = build.cylinder(DVec3::ZERO, DVec3::Z, 6.0);
    build.circle(wall, 2.0, None);
    let body = build.finish(10.0);
    let fineness = (1e-2, 10.0);
    let grids = super::super::cone::grids(&body, fineness);

    let axes = Axes::of(&body, &grids, &contact::walls(&body), fineness);

    assert!(
        !axes.given().contains_key(&wall),
        "the wall of 6 took the cone's rays: {:?}",
        axes.given().keys().collect::<Vec<_>>()
    );
}
