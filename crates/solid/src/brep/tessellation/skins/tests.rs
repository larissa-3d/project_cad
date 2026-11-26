use std::collections::BTreeSet;
use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::{SHOWN, THIN, cancelled, rings, thinner};
use crate::Body as Part;
use crate::Loop;
use crate::brep::Scale;
use crate::brep::curve::Curve;
use crate::brep::surface::{Cylinder, Sphere, Surface};
use crate::brep::topology::{Body, EdgeId, FaceId};
use crate::profile::{Contour, Frame, Profile, Run};
use crate::turning::{Axis, Turn};

#[test]
fn two_faces_drawn_on_one_triangle_facing_opposite_ways_are_left_out() {
    let mut oriented = vec![
        (FaceId(0), [1, 2, 3]),
        (FaceId(1), [2, 1, 3]),
        (FaceId(1), [3, 4, 5]),
    ];
    cancelled(&mut oriented);
    assert_eq!(oriented, vec![(FaceId(1), [3, 4, 5])]);
}

#[test]
fn two_faces_drawn_on_one_triangle_facing_the_same_way_are_kept() {
    let mut oriented = vec![(FaceId(0), [1, 2, 3]), (FaceId(1), [2, 3, 1])];
    cancelled(&mut oriented);
    assert_eq!(oriented.len(), 2);
}

#[test]
fn one_face_drawn_twice_on_one_triangle_is_kept() {
    let mut oriented = vec![(FaceId(0), [1, 2, 3]), (FaceId(0), [3, 2, 1])];
    cancelled(&mut oriented);
    assert_eq!(oriented.len(), 2);
}

/// The radius of every ball here.
const RADIUS: f64 = 5.0;

/// An outline with its runs, as the drawing hands it to the exact kernel.
fn exact(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

/// A ball of [`RADIUS`] about `centre`: a half disc turned whole about the
/// line through `centre` along z.
fn ball_at(centre: DVec3) -> Part {
    let sampled: &'static [DVec2] = Box::leak(
        (0..=32)
            .map(|step| DVec2::from_angle(-PI / 2.0 + PI * f64::from(step) / 32.0) * RADIUS)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let half = Contour {
        corners: vec![DVec2::new(0.0, -RADIUS), DVec2::new(0.0, RADIUS)],
        runs: vec![
            Run::Round {
                center: DVec2::ZERO,
                turn: PI,
            },
            Run::Straight,
        ],
    };
    let profile = Profile {
        exact: Some((half, Vec::new())),
        sampled: Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let frame = Frame {
        origin: centre,
        u: DVec3::X,
        v: DVec3::Z,
    };
    let along_v = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    Part::default()
        .tool_turned(&profile, frame, &Turn::of(along_v, TAU, 0.0, &profile))
        .expect("the ball turns")
}

/// A shaft of `radius` along the z axis, from z = `from` to `to`, a tool of
/// `part`.
fn shaft(part: &Part, radius: f64, from: f64, to: f64) -> Part {
    part.tool_raised(
        &exact(Contour {
            corners: vec![DVec2::X * radius],
            runs: vec![Run::Round {
                center: DVec2::ZERO,
                turn: TAU,
            }],
        }),
        Frame {
            origin: DVec3::Z * from,
            u: DVec3::X,
            v: DVec3::Y,
        },
        DVec3::Z * (to - from),
    )
    .expect("the shaft raises")
}

/// The ball of [`RADIUS`] about the origin less a shaft along z through it
/// from z = −10 to 10, `under` its radius: a ring between the shaft's wall
/// and the sphere, `under` thick at the equator, read at the shaft's reach
/// of 10. The kernel's own cut, the shaft's numbers moved past the ball's as
/// a step moves them, and nothing drawn.
fn ring(under: f64) -> Body {
    let ball = ball_at(DVec3::ZERO);
    let tool = shaft(&ball, RADIUS - under, -10.0, 10.0);
    let ring = ball
        .brep()
        .expect("the ball is exact")
        .cut_by(
            &tool
                .brep()
                .expect("the shaft is exact")
                .clone()
                .renumbered_by(|number| number + 100),
        )
        .expect("the ball is bored");
    assert_eq!(
        ring.scale().reach(),
        10.0,
        "the ring is read at the shaft's reach"
    );
    ring
}

/// The faces of `body` on a sphere and on a cylinder: every face of the
/// ring a shaft through the ball leaves, one of each.
fn faces_of_the_ring(body: &Body) -> BTreeSet<FaceId> {
    let faces: BTreeSet<FaceId> = body.face_ids().collect();
    let [spheres, cylinders] = [
        |surface: &Surface| matches!(surface, Surface::Sphere(_)),
        |surface: &Surface| matches!(surface, Surface::Cylinder(_)),
    ]
    .map(|kind| {
        faces
            .iter()
            .filter(|face| kind(body.surface(body.face(**face).surface)))
            .count()
    });
    assert_eq!(
        (faces.len(), spheres, cylinders),
        (2, 1, 1),
        "the ring is a face of the sphere and one of the wall"
    );
    faces
}

/// The ring a shaft through the ball leaves, drawn at a tolerance coarse
/// enough that a hundredth of it stands past [`THIN`] of the kernel's
/// tolerances at the ring's reach: left undrawn up to those, a tenth under,
/// and drawn a tenth past.
#[test]
fn a_ring_drawn_coarse_is_left_undrawn_up_to_four_hundred_of_the_kernel_s_tolerances_and_drawn_past_them()
 {
    let hairs = THIN * Scale::of(10.0).eps();
    let tolerance = 1e-2;
    assert!(
        SHOWN * tolerance > 10.0 * hairs,
        "the kernel's tolerances bind"
    );
    let within = ring(0.9 * hairs);
    assert_eq!(
        rings(&within, tolerance),
        faces_of_the_ring(&within),
        "a ring {} thick, under {hairs}",
        0.9 * hairs
    );
    let past = ring(1.1 * hairs);
    faces_of_the_ring(&past);
    assert_eq!(
        rings(&past, tolerance),
        BTreeSet::new(),
        "a ring {} thick, past {hairs}",
        1.1 * hairs
    );
}

/// The ring a shaft through the ball leaves, drawn at a tolerance fine
/// enough that a hundredth of it, [`SHOWN`], stands under [`THIN`] of the
/// kernel's tolerances at the ring's reach: left undrawn up to that
/// hundredth, a tenth under, and drawn a tenth past.
#[test]
fn a_ring_drawn_fine_is_left_undrawn_up_to_a_hundredth_of_the_drawing_s_tolerance_and_drawn_past_it()
 {
    let tolerance = 1e-4;
    let shown = SHOWN * tolerance;
    assert!(
        THIN * Scale::of(10.0).eps() > 2.0 * shown,
        "the drawing's tolerance binds"
    );
    let within = ring(0.9 * shown);
    assert_eq!(
        rings(&within, tolerance),
        faces_of_the_ring(&within),
        "a ring {} thick, under {shown}",
        0.9 * shown
    );
    let past = ring(1.1 * shown);
    faces_of_the_ring(&past);
    assert_eq!(
        rings(&past, tolerance),
        BTreeSet::new(),
        "a ring {} thick, past {shown}",
        1.1 * shown
    );
}

/// The sphere of a ball of [`RADIUS`] about `centre`.
fn sphere_at(centre: DVec3) -> Sphere {
    let ball = ball_at(centre);
    let body = ball.brep().expect("the ball is exact");
    body.face_ids()
        .find_map(|face| match body.surface(body.face(face).surface) {
            Surface::Sphere(sphere) => Some(*sphere),
            _ => None,
        })
        .expect("the ball has a face on its sphere")
}

/// A disc of radius 4.75 about the z axis raised from z = `from` to `to`, its
/// wall's cylinder, and the circles that bound the wall, each with its
/// height along z.
fn wall(from: f64, to: f64) -> (Body, Cylinder, Vec<(EdgeId, f64)>) {
    let disc = shaft(&Part::default(), 4.75, from, to);
    let body = disc.brep().expect("the disc is exact").clone();
    let (cylinder, edges) = body
        .face_ids()
        .find_map(|face| match body.surface(body.face(face).surface) {
            Surface::Cylinder(cylinder) => Some((
                *cylinder,
                body.face(face)
                    .loops
                    .iter()
                    .flatten()
                    .map(|coedge| coedge.edge)
                    .collect::<Vec<EdgeId>>(),
            )),
            _ => None,
        })
        .expect("the disc has a wall");
    let heights = edges
        .into_iter()
        .map(|edge| match body.curve(body.edge(edge).curve) {
            Curve::Circle(circle) => (edge, circle.center.z),
            other => panic!("the wall is bounded by circles, not {other:?}"),
        })
        .collect();
    (body, cylinder, heights)
}

/// The wall of 4.75 from 1 to 2 above the centre of a ball of 5, clear of
/// its equator: the sphere stands 4.90 from the axis at the lower circle and
/// 4.58 at the upper, 0.15 and 0.17 off the wall, and 5 at the equator,
/// 0.25 off. Within 0.2 of the wall between its two circles, though not
/// within 0.1 — and never weighed at the equator, which the band does not
/// reach.
#[test]
fn a_band_clear_of_the_equator_is_weighed_between_its_circles_and_not_at_the_equator() {
    let sphere = sphere_at(DVec3::ZERO);
    let (body, cylinder, circles) = wall(1.0, 2.0);
    let edges: Vec<EdgeId> = circles.iter().map(|(edge, _)| *edge).collect();
    assert!(
        thinner(&body, &sphere, &cylinder, &edges, 0.2),
        "within 0.2 between z = 1 and 2"
    );
    assert!(
        !thinner(&body, &sphere, &cylinder, &edges, 0.1),
        "not within 0.1 at z = 2"
    );
}

/// The same wall and the same ball, the ball about the point 3 up the axis
/// and the wall from 4 to 5: the band is read from the sphere's centre, 1 to
/// 2 above it, and stands within 0.2 of the wall as about the origin.
#[test]
fn a_band_is_read_from_the_sphere_s_centre_wherever_the_ball_stands() {
    let sphere = sphere_at(DVec3::Z * 3.0);
    let (body, cylinder, circles) = wall(4.0, 5.0);
    let edges: Vec<EdgeId> = circles.iter().map(|(edge, _)| *edge).collect();
    assert!(
        thinner(&body, &sphere, &cylinder, &edges, 0.2),
        "within 0.2 between 1 and 2 above the centre"
    );
}

/// The lower circle of the same wall alone, where the sphere stands 0.15
/// off it: one circle bounds no band, however close the sphere stands.
#[test]
fn one_circle_alone_bounds_no_ring() {
    let sphere = sphere_at(DVec3::ZERO);
    let (body, cylinder, circles) = wall(1.0, 2.0);
    let (lower, _) = circles
        .iter()
        .copied()
        .min_by(|(_, first), (_, second)| first.total_cmp(second))
        .expect("the wall has circles");
    assert!(!thinner(&body, &sphere, &cylinder, &[lower], 0.2));
}
