//! A face of a sphere whose loop holds a pole of its chart inside it: its
//! flux is integrated with the pole's cap accounted for, whichever axis the
//! chart was laid about.
//!
//! Closes #945.
//! - the #804 apex piece with its chart axis along the inward direction (a
//!   test-only construction: the piece laid again on the same sphere read
//!   about that axis) has the volume 164.84162207554385 on the pyramid on its
//!   base —
//!   `the_pyramid_s_apex_piece_read_about_the_way_its_hips_run_in_holds_the_volume_the_arithmetic_gives`
//! - a ball cut by a plane square to its axis, keeping the cap holding a pole,
//!   and the same with the cap's loop a tilted circle round the pole: the
//!   exact volume, the cap's —
//!   `a_cap_of_a_ball_holding_a_pole_of_its_chart_holds_the_cap_s_volume_its_circle_square_to_the_axis_or_tilted_round_the_pole`

use crate::Body as Part;
use crate::EdgeName;
use crate::brep::Face;
use crate::brep::Sphere;
use crate::brep::turned::tests::round::{ball, turned};
use crate::profile::Profile;
use crate::sweep::Loop;

use super::*;

/// The apex of the square pyramid of #804: over the middle of its base, 10 by
/// 10, and 5 high.
const APEX: DVec3 = DVec3::new(5.0, 5.0, 5.0);

/// The numbers the pyramid's four slopes answer to, front, back, left and
/// right: each the floor of the slab cut from the block, numbered past the
/// part's.
const SLOPES: [u32; 4] = [6, 12, 18, 24];

/// An outline with its runs, as the drawing hands it to the exact kernel.
fn outline(contour: Contour) -> Profile<'static> {
    Profile {
        exact: Some((contour, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

/// The square pyramid standing on its own base, as #804's tests build it: a
/// block one wider than the base all round and one higher than the apex,
/// raised as `cao_part` raises a step, cut by the four planes through the
/// apex and an edge of the base, each the floor of a slab.
fn pyramid() -> Part {
    let empty = Part::default();
    let tool = empty
        .tool_raised(
            &outline(Contour::rectangle(DVec2::splat(-1.0), DVec2::splat(11.0))),
            ground(),
            DVec3::Z * 6.0,
        )
        .expect("the block raises");
    let block = empty.union(&tool).expect("the block is built");
    [[0.0, -1.0], [0.0, 1.0], [-1.0, 0.0], [1.0, 0.0]]
        .into_iter()
        .fold(block, |part, [x, y]| {
            let away = DVec3::new(x, y, 1.0).normalize();
            let u = away.any_orthonormal_vector();
            let slab = part
                .tool_raised(
                    &outline(Contour::rectangle(DVec2::splat(-50.0), DVec2::splat(50.0))),
                    Frame {
                        origin: APEX,
                        u,
                        v: away.cross(u),
                    },
                    away * 50.0,
                )
                .expect("the slab raises");
            part.difference(&slab)
                .expect("a plane through the apex cuts the block")
        })
}

/// The pyramid's four hips: the edges between two of its slopes.
fn hips(pyramid: &Part) -> Vec<EdgeName> {
    let slope = |side: &Vec<u32>| matches!(side[..], [face] if SLOPES.contains(&face));
    let hips: Vec<EdgeName> = pyramid
        .edge_names()
        .into_iter()
        .filter(|name| name.between.iter().all(slope))
        .collect();
    assert_eq!(
        hips.len(),
        4,
        "four hips, each between two slopes: {hips:?}"
    );
    hips
}

/// What the square pyramid keeps once its four hips are rounded at 1 and its
/// apex closed by a piece of sphere, as
/// `tests/a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs`
/// derives it: each round's section along its hip, less what the ground
/// spares at its foot, taken away, then the cell the end sections bound at the
/// apex, the ball's sector in it given back.
fn pyramid_closed_by_a_sphere_of_1() -> f64 {
    let section = 1.0 / 3f64.sqrt() - PI / 6.0;
    let along = 5.0 * 3f64.sqrt() - (2.0f64 / 3.0).sqrt();
    let at_the_foot = -(2f64.sqrt()) * (11.0 / 18.0 - PI / (3.0 * 3f64.sqrt()));
    let cell = 4.0 * 2f64.sqrt() / 9.0;
    let sector = 4.0 * (1.0f64 / 3.0).asin() / 3.0;
    500.0 / 3.0 - 4.0 * (section * along + at_the_foot) - cell + sector
}

/// The way the pyramid's four hips run from its apex on the whole, as the
/// step reads a corner's: the sum of the way each runs down to its corner of
/// the base, of length one.
fn inward() -> DVec3 {
    [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]
        .map(|[x, y]| (DVec3::new(x, y, 0.0) - APEX).normalize())
        .into_iter()
        .sum::<DVec3>()
        .normalize()
}

/// The piece of sphere the fillet closes the apex with, read about the axis
/// the step lays it about, square to [`inward`], and laid again on the same
/// sphere read about [`inward`] itself: the axis #804 first gave it, whose
/// pole above the centre stands inside the piece, on the apex's side of every
/// round's end section. The same matter either way, the volume the arithmetic
/// gives. On the kernel #945 was written against, the second read 158.99: the
/// piece's arcs, circles of the sphere neither latitudes nor meridians of its
/// chart, could not be traced until #805 read them as tilted traces, so the
/// piece held no pole and its flux lost the pole's cap.
#[test]
fn the_pyramid_s_apex_piece_read_about_the_way_its_hips_run_in_holds_the_volume_the_arithmetic_gives()
 {
    let promised = pyramid_closed_by_a_sphere_of_1();
    let pyramid = pyramid();
    let filleted = pyramid
        .fillet_edges(&hips(&pyramid), 1.0)
        .expect("the fillet at 1 on the four hips closes the apex")
        .body;
    let laid = filleted
        .brep()
        .expect("the exact kernel computed the fillet");
    let pieces: Vec<FaceId> = (laid.face_ids())
        .filter(|&face| matches!(laid.surface(laid.face(face).surface), Surface::Sphere(_)))
        .collect();
    let [piece] = pieces[..] else {
        panic!("one piece of sphere closes the apex: {pieces:?}");
    };
    let Surface::Sphere(sphere) = *laid.surface(laid.face(piece).surface) else {
        unreachable!("the piece lies on a sphere");
    };
    let about_inward = Sphere::through(sphere.middle(), inward(), 0.0, sphere.radius);
    let read = laid
        .replaced(
            &[piece],
            Surface::Sphere(about_inward),
            Face {
                poles: Vec::new(),
                ..laid.face(piece).clone()
            },
        )
        .expect("the piece lies on the same sphere read about another axis");

    for (what, body) in [
        ("about the axis the step lays", laid),
        ("about the way the hips run in", &read),
    ] {
        assert!(
            (body.volume() - promised).abs() <= 1e-9 * promised,
            "the pyramid with its apex piece read {what} holds {}, where the arithmetic gives {promised}, off by {}",
            body.volume(),
            body.volume() - promised
        );
    }
}

/// The slab 10 thick on the side of the plane `offset` along `normal` that
/// `normal` points away from, its sides 20 from the normal's line either
/// way: cut from the ball of radius 5 about the origin, it leaves the cap past
/// the plane.
fn all_but_the_cap(normal: DVec3, offset: f64) -> Body {
    let away = -normal.normalize();
    let u = away.any_orthonormal_vector();
    Body::raised(
        &Contour::rectangle(DVec2::splat(-20.0), DVec2::splat(20.0)),
        &[],
        Frame {
            origin: away * -offset,
            u,
            v: away.cross(u),
        },
        away * 10.0,
    )
    .expect("a slab raises")
}

/// What a cap `height` high of the ball of radius 5 holds.
fn cap_of_5(height: f64) -> f64 {
    PI * height * height * (15.0 - height) / 3.0
}

/// The ball of radius 5 turned about Y, cut down to the cap past a plane
/// whose normal leans `tilt` from the axis towards X, `offset` along it from
/// the centre: square to the axis where the tilt is nought, else a circle at
/// a slant to it. Each cap holds the pole at y = 5, which stands 5 cos `tilt`
/// along the normal, past the plane, and not the other: a smaller cap than a
/// half, a half and a larger one.
#[test]
fn a_cap_of_a_ball_holding_a_pole_of_its_chart_holds_the_cap_s_volume_its_circle_square_to_the_axis_or_tilted_round_the_pole()
 {
    for (tilt, offset) in [
        (0.0, 2.0),
        (0.0, -1.5),
        (0.3, 2.0),
        (0.4, 0.0),
        (0.5, -2.0),
        (1.0, 1.0),
    ] {
        let normal = DVec3::new(f64::sin(tilt), f64::cos(tilt), 0.0);
        let what = format!("the cap of the ball past {offset} along a normal leaning {tilt}");
        let cap = turned(&ball(), 360.0)
            .cut_by(&all_but_the_cap(normal, offset))
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
        let promised = cap_of_5(5.0 - offset);
        assert!(
            (cap.volume() - promised).abs() <= 1e-9 * promised,
            "{what} holds {}, where the arithmetic gives {promised}, off by {}",
            cap.volume(),
            cap.volume() - promised
        );
    }
}
