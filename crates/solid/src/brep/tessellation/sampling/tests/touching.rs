//! Which samples the drawing lays on a plane touching a sphere at a point
//! (#939): only those a face of the sphere added as it was cut, numbered
//! from the count taken before it, that stand within the tolerance of a
//! plane of the body itself standing within the tolerance of touching the
//! sphere, as the relation reads a touch, band's edge included. Each is laid
//! at its foot on the plane.

use glam::DVec3;

use super::super::Samples;
use super::fixtures::{self, Build};
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Plane, Sphere, Surface};
use crate::brep::topology::Body;

/// A reach whose tolerance is 2⁻²⁶ to the bit: 10⁹ × 2⁻²⁶, exact, times the
/// relative tolerance rounds back onto the power of two.
const REACH: f64 = 1e9 / 67_108_864.0;

/// The tolerance at [`REACH`], checked to be the power of two.
fn eps() -> f64 {
    let eps = Scale::of(REACH).eps();
    assert_eq!(eps, 2f64.powi(-26), "the tolerance at the reach chosen");
    eps
}

/// The sphere of radius 5 about the origin, turned about Y.
fn sphere() -> Sphere {
    Sphere::through(DVec3::ZERO, DVec3::Y, 0.0, 5.0)
}

/// The plane x = `at`.
fn across_x(at: f64) -> Plane {
    Plane::through(DVec3::X * at, DVec3::X).0
}

/// A body of [`sphere`] and `planes`, at [`REACH`], with a vertex at each of
/// `corners`.
fn body_of(planes: &[Plane], corners: &[DVec3]) -> Body {
    let mut build = Build::new();
    build.surface(Surface::Sphere(sphere()));
    for plane in planes {
        build.surface(Surface::Plane(*plane));
    }
    for corner in corners {
        build.vertex(*corner);
    }
    build.finish(REACH)
}

/// The samples of `body` once a face of the sphere added `added` and was
/// laid on the planes touching it, and the number the first of them took.
fn laid(body: &Body, added: &[DVec3]) -> (Samples, usize) {
    let mut samples = Samples::of(body, 1e-3);
    let from = samples.count();
    for point in added {
        samples.added(*point);
    }
    samples.on_a_touching_plane(body, &sphere(), from);
    (samples, from)
}

#[test]
fn the_samples_a_face_adds_are_numbered_on_from_the_count_past_every_vertex_and_every_edge_s_samples()
 {
    for body in [fixtures::block(), fixtures::block_with_a_hole()] {
        let mut samples = Samples::of(&body, 1e-3);
        let count = samples.count();
        let taken = body
            .edge_ids()
            .flat_map(|edge| samples.edge(edge).to_vec())
            .chain(0..body.vertex_ids().count())
            .max()
            .expect("a body with vertices");

        let first = samples.added(DVec3::ZERO);
        let second = samples.added(DVec3::ONE);

        assert!(
            count > taken,
            "the count {count} stands past every sample the vertices and edges took, up to {taken}"
        );
        assert_eq!(
            [first, second],
            [count, count + 1],
            "the samples added next"
        );
        assert_eq!(samples.count(), count + 2);
    }
}

#[test]
fn a_sample_a_sphere_face_added_within_the_tolerance_of_a_plane_touching_it_is_laid_at_its_foot_on_the_plane()
 {
    let eps = eps();
    for gap in [0.0, eps / 2.0, -eps / 2.0] {
        let touching = across_x(5.0 + gap);
        let corners = [
            DVec3::new(5.0 + gap - eps / 2.0, 0.0, 1e-4),
            DVec3::new(5.0 + gap + eps / 2.0, 1e-4, 0.0),
        ];
        let body = body_of(&[touching], &corners);
        let near = [
            DVec3::new(5.0 + gap - eps * 0.75, 1e-4, 0.0),
            DVec3::new(5.0 + gap + eps / 3.0, -2e-4, 1e-4),
        ];
        let far = [
            DVec3::new(5.0 + gap - 2.0 * eps, 0.0, -1e-4),
            DVec3::new(0.0, 5.0, 0.0),
        ];
        let added: Vec<DVec3> = near.iter().chain(&far).copied().collect();

        let (samples, from) = laid(&body, &added);

        let what = format!("the plane x = 5 + {gap}");
        for (index, corner) in corners.iter().enumerate() {
            assert_eq!(
                samples.point(index),
                *corner,
                "{what}: a vertex, numbered before the face's samples, stays"
            );
        }
        for (index, point) in near.iter().enumerate() {
            let foot = *point - touching.normal * touching.distance(*point);
            let moved = samples.point(from + index);
            assert!(
                moved.distance(foot) <= 1e-15 * REACH
                    && touching.distance(moved).abs() <= 1e-15 * REACH,
                "{what}: {point} is laid at {moved}, where its foot on the plane is {foot}"
            );
        }
        for (index, point) in far.iter().enumerate() {
            assert_eq!(
                samples.point(from + near.len() + index),
                *point,
                "{what}: a sample further than the tolerance from the plane stays"
            );
        }
    }
}

#[test]
fn a_sample_within_the_tolerance_of_a_plane_that_cuts_the_sphere_or_misses_it_stays() {
    let eps = eps();
    let cutting = [
        Plane::through(DVec3::ZERO, DVec3::Z).0,
        across_x(3.0),
        across_x(5.0 - 4.0 * eps),
    ];
    let missing = [across_x(5.0 + 4.0 * eps), across_x(-7.0)];
    let planes: Vec<Plane> = cutting.iter().chain(&missing).copied().collect();
    let body = body_of(&planes, &[]);
    let added: Vec<DVec3> = planes
        .iter()
        .map(|plane| plane.origin + plane.u * 1e-3 + plane.normal * (eps / 2.0))
        .collect();

    let (samples, from) = laid(&body, &added);

    for (index, (point, plane)) in added.iter().zip(&planes).enumerate() {
        assert_eq!(
            samples.point(from + index),
            *point,
            "a sample {} from the plane through {} facing {}, which does not touch the sphere, stays",
            plane.distance(*point),
            plane.origin,
            plane.normal
        );
    }
}

#[test]
fn a_plane_standing_the_tolerance_itself_off_the_sphere_touches_it_for_the_drawing_as_for_the_relation()
 {
    let eps = eps();
    let scale = Scale::of(REACH);
    for (at, touches) in [
        (5.0 + eps, true),
        (-5.0 - eps, true),
        (5.0 - eps, true),
        (5.0 + 2.0 * eps, false),
    ] {
        let plane = across_x(at);
        assert_eq!(
            (plane.distance(sphere().middle()).abs() - 5.0).abs(),
            if touches { eps } else { 2.0 * eps },
            "the plane x = {at} stands that far off touching the sphere, to the bit"
        );
        let read = relation(&Surface::Sphere(sphere()), &Surface::Plane(plane), scale);
        assert_eq!(
            matches!(read, Relation::Apex(_)),
            touches,
            "the plane x = {at}: the relation reads {read:?}"
        );
        let body = body_of(&[plane], &[]);
        let point = DVec3::new(at - eps / 2.0 * at.signum(), 1e-4, 0.0);

        let (samples, from) = laid(&body, &[point]);

        let moved = samples.point(from);
        assert_eq!(
            plane.distance(moved).abs() <= 1e-15 * REACH,
            touches,
            "the plane x = {at}: the sample {point}, {} from it, is laid at {moved}",
            plane.distance(point)
        );
    }
}
