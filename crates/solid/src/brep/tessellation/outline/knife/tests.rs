//! The knife-edge rims of a face of a sphere, read directly (#1138): what a
//! rim reads off the body it is drawn on, which points it passes, which rows
//! the band beside it clears, which rims are knife edges at all, and which
//! faces the central chart leaves to the charts after it.
//!
//! And the walls running beside a face from a rim a hair under its equator
//! up to where they end on its row (#1127): which rims and ends [`ends`]
//! finds — beside matter or beside void, never beside a wall running away
//! from the face — and which faces are cut at that row, the strips keeping
//! it only where every sample of the rim stands on the angle of one of the
//! row's points. A row kept where the rim's samples stand between the
//! face's angles joins a point of the grid to a sample of the rim a part of
//! a step aside, the sliver #956 left the row out for.

use std::f64::consts::{FRAC_1_SQRT_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::super::{Outline, laps};
use super::{Band, Rim, bands, ends, rims, within_the_walls};
use crate::brep::Declined;
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Sphere, Surface};
use crate::brep::tessellation::sampling::{Rounded, Samples};
use crate::brep::topology::{Body, EdgeId, FaceId};
use crate::profile::{Contour, Frame, Profile, Run};
use crate::turning::{Axis, Turn};

/// The drawing's tolerance at the reach of 10 the shafts below stand to.
const TOLERANCE: f64 = 2e-3;

/// The radius of the ball.
const RADIUS: f64 = 5.0;

/// The radius of the shaft, a little under the ball's: their surfaces meet
/// cos⁻¹ 0.98 apart.
const SHAFT: f64 = 4.9;

/// The ball's centre, on the shaft's axis, off the origin and above it: the
/// wall's canonical origin, the foot of the origin on its axis, then stands
/// off the axis's own frame and below the ball's centre, and a corner or a
/// height read from the wrong end of either shows.
const CENTRE: DVec3 = DVec3::new(1.0, 0.5, 3.0);

/// How far from the ball's centre along z the shaft's wall meets the
/// sphere: a = √(r² − b²).
fn reach() -> f64 {
    (RADIUS * RADIUS - SHAFT * SHAFT).sqrt()
}

fn exact(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: crate::Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

/// A ball of [`RADIUS`] about [`CENTRE`], a half disc turned `turn` about
/// the line through it along (1, 0, 1)/√2. The turn reads the half disc's
/// samples to tell which side of its axis it lies on.
fn ball(turn: f64) -> crate::Body {
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
    let axis = DVec3::new(1.0, 0.0, 1.0).normalize();
    let frame = Frame {
        origin: CENTRE,
        u: axis.any_orthonormal_vector(),
        v: axis,
    };
    let along_v = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    let profile = Profile {
        exact: Some((half, Vec::new())),
        sampled: crate::Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    crate::Body::default()
        .tool_turned(&profile, frame, &Turn::of(along_v, turn, 0.0, &profile))
        .expect("the exact kernel turns the ball")
}

/// A shaft of [`SHAFT`] along z about [`CENTRE`], from 10 under the ball's
/// centre to `to` over it, a tool of `part`.
fn shaft(part: &crate::Body, to: f64) -> crate::Body {
    shaft_between(part, SHAFT, -10.0, to)
}

/// A shaft of `radius` along z about [`CENTRE`], from `from` over the ball's
/// centre to `to` over it, a tool of `part`.
fn shaft_between(part: &crate::Body, radius: f64, from: f64, to: f64) -> crate::Body {
    let centre = CENTRE.truncate();
    part.tool_raised(
        &exact(Contour {
            corners: vec![centre + DVec2::X * radius],
            runs: vec![Run::Round {
                center: centre,
                turn: TAU,
            }],
        }),
        Frame {
            origin: DVec3::Z * (CENTRE.z + from),
            u: DVec3::X,
            v: DVec3::Y,
        },
        DVec3::Z * (to - from),
    )
    .expect("the shaft raises")
}

/// What `step` leaves, as the exact kernel holds it, sampled at
/// [`TOLERANCE`].
fn read(step: Result<crate::Body, Declined>) -> (Body, Samples) {
    read_at(step, TOLERANCE)
}

/// What `step` leaves, as the exact kernel holds it, sampled at
/// `tolerance`.
fn read_at(step: Result<crate::Body, Declined>, tolerance: f64) -> (Body, Samples) {
    let body = step
        .expect("the step is built")
        .brep()
        .expect("the exact kernel computed the step")
        .clone();
    let samples = Samples::of(&body, tolerance);
    (body, samples)
}

/// The shaft from 10 under the ball's centre up to it, less the ball: the
/// face on the sphere, a dimple from the ball's foot up to a under its
/// centre, meets the shaft's wall there along a knife-edge rim.
fn knife() -> (Body, Samples) {
    let ball = ball(TAU);
    read(shaft(&ball, 0.0).difference(&ball))
}

/// The face of `body` on the ball's sphere, and the sphere.
fn on_the_sphere(body: &Body) -> (FaceId, Sphere) {
    body.face_ids()
        .find_map(|id| match body.surface(body.face(id).surface) {
            Surface::Sphere(sphere) => Some((id, *sphere)),
            _ => None,
        })
        .expect("a face on the sphere")
}

/// The edges of the face `id` with each other face along them.
fn along(body: &Body, id: FaceId) -> Vec<(EdgeId, FaceId)> {
    (body.face(id).loops.iter().flatten())
        .flat_map(|coedge| {
            (body.uses(coedge.edge).into_iter())
                .filter(|&(other, _)| other != id)
                .map(|(other, _)| (coedge.edge, other))
        })
        .collect()
}

/// The one rim the face on the sphere of `body` meets a wall along.
fn the_rim(body: &Body, samples: &Samples) -> Rim {
    let (id, _) = on_the_sphere(body);
    let mut found: Vec<Rim> = rims(body, samples, id).collect();
    assert_eq!(found.len(), 1, "the face on the sphere meets one wall");
    found.remove(0)
}

#[test]
fn the_rim_of_a_shaft_less_a_ball_stands_at_its_circle_s_height_on_the_wall_and_the_wall_runs_from_it_to_the_shaft_s_foot()
 {
    let (body, samples) = knife();
    let rim = the_rim(&body, &samples);
    assert!(
        (rim.level - rim.height(rim.circle.center)).abs() < 1e-12
            && (rim.level - (CENTRE.z - reach())).abs() < 1e-9,
        "the rim stands at {}, its circle at {} along the wall, {} under the ball's centre",
        rim.level,
        rim.height(rim.circle.center),
        reach()
    );
    assert!(
        (rim.side - (-10.0 + reach())).abs() < 1e-9,
        "the wall runs {} from the rim, where its foot stands {} from it",
        rim.side,
        -10.0 + reach()
    );
    for corner in &rim.corners {
        assert!(
            (corner.length() - SHAFT).abs() < 1e-9,
            "a sample of the rim stands {} from the wall's axis, not on the wall",
            corner.length()
        );
    }
    assert_eq!(
        rim.corners.first(),
        rim.corners.last(),
        "a whole circle's samples close back onto the first"
    );
    assert_eq!(
        rim.corners.len(),
        samples
            .edge(along(&body, on_the_sphere(&body).0)[0].0)
            .len()
            + 1
    );
    assert!(
        rim.nearest < SHAFT && SHAFT - rim.nearest <= TOLERANCE,
        "the chords come {} near the axis, the wall {} from it, the drawing's tolerance {TOLERANCE}",
        rim.nearest,
        SHAFT
    );
}

#[test]
fn the_circle_at_the_shaft_s_foot_is_a_rim_of_its_wall_running_up_from_it_and_none_of_the_foot_s_plane()
 {
    let (body, samples) = knife();
    let (id, _) = on_the_sphere(&body);
    let [(_, wall)] = along(&body, id)[..] else {
        panic!("the face on the sphere meets one other face");
    };
    let foot = body
        .face_ids()
        .find(|&face| matches!(body.surface(body.face(face).surface), Surface::Plane(_)))
        .expect("the shaft's foot, a plane");
    let [(edge, _)] = along(&body, foot)[..] else {
        panic!("the foot meets one other face");
    };
    assert!(Rim::of(&body, &samples, edge, foot).is_none());
    let rim =
        Rim::of(&body, &samples, edge, wall).expect("the foot's circle is about the wall's axis");
    assert!(
        (rim.level - (CENTRE.z - 10.0)).abs() < 1e-9 && (rim.side - (10.0 - reach())).abs() < 1e-9,
        "the foot's rim stands at {} and the wall runs {} from it",
        rim.level,
        rim.side
    );
}

/// The knife's rim, its circle's axis set to `axis`: a body no step builds,
/// to read where a circle stops being about the wall's axis.
fn tilted_to(axis: impl Fn(&Circle) -> DVec3) -> Option<Rim> {
    let (body, samples) = knife();
    let (id, _) = on_the_sphere(&body);
    let [(edge, wall)] = along(&body, id)[..] else {
        panic!("the face on the sphere meets one other face");
    };
    let mut tilted = body.clone();
    let curve = tilted.edge(edge).curve;
    let Curve::Circle(circle) = *tilted.curve(curve) else {
        panic!("the rim is a circle");
    };
    tilted.curves[curve.0 as usize] = Curve::Circle(Circle {
        axis: axis(&circle),
        ..circle
    });
    Rim::of(&tilted, &samples, edge, wall)
}

/// A circle a thousandth of a radian off the wall's axis makes no rim; one
/// exactly [`Scale::RELATIVE`] off it is about the axis still, as the
/// strips about a face's circles read them one axis within it.
#[test]
fn a_wall_bounded_by_a_circle_more_than_the_kernel_s_tolerance_off_its_axis_makes_no_rim() {
    assert!(tilted_to(|circle| circle.axis).is_some());
    assert!(tilted_to(|circle| (circle.axis + circle.u * 1e-3).normalize()).is_none());
    let off = DVec3::new(Scale::RELATIVE, 0.0, 1.0);
    assert_eq!(off.length(), 1.0);
    assert_eq!(off.cross(DVec3::Z).length(), Scale::RELATIVE);
    let rim = tilted_to(|_| off).expect("a circle the kernel's tolerance off the axis is about it");
    assert_eq!(rim.wall.axis, DVec3::Z);
}

#[test]
fn a_rim_drawn_along_an_arc_is_not_closed_back_onto_its_first_sample() {
    let half = ball(PI);
    let (body, samples) = read(shaft(&half, 0.0).difference(&half));
    let (id, _) = on_the_sphere(&body);
    let open: Vec<Rim> = rims(&body, &samples, id).collect();
    assert!(!open.is_empty(), "the half dimple meets the wall");
    for rim in &open {
        assert_ne!(rim.corners.first(), rim.corners.last());
    }
}

/// A rim drawn by hand: the circle z = −1 of a wall of radius 5 about the
/// line x = 0.5, y = 0.25 along z, the wall running down from it to z = −10,
/// sampled at (5, 0), (0, 5), (−5, 0) and (0, −5) about its axis — anticlockwise
/// seen from above, or clockwise — the chords between them coming within
/// 5/√2 of the axis.
fn square(anticlockwise: bool) -> Rim {
    let wall = Cylinder::about(DVec3::new(0.5, 0.25, 0.0), DVec3::Z, 5.0);
    let mut corners =
        [[5.0, 0.0], [0.0, 5.0], [-5.0, 0.0], [0.0, -5.0], [5.0, 0.0]].map(DVec2::from);
    if !anticlockwise {
        corners.reverse();
    }
    Rim {
        wall,
        circle: Circle {
            center: wall.origin - DVec3::Z,
            axis: DVec3::Z,
            radius: 5.0,
            u: DVec3::X,
            v: DVec3::Y,
        },
        level: -1.0,
        side: -9.0,
        corners: corners.to_vec(),
        nearest: 5.0 * FRAC_1_SQRT_2,
    }
}

/// The point at `at` about the square rim's axis, `z` high.
fn about_the_square([x, y]: [f64; 2], z: f64) -> DVec3 {
    DVec3::new(0.5 + x, 0.25 + y, z)
}

#[test]
fn a_point_stands_as_high_along_the_wall_as_it_stands_along_its_axis() {
    let rim = square(true);
    assert_eq!(rim.height(about_the_square([3.0, 3.0], -4.0)), -4.0);
    assert_eq!(rim.height(about_the_square([-2.0, 1.0], 7.5)), 7.5);
}

#[test]
fn a_point_inside_the_wall_beyond_a_chord_on_the_side_the_wall_runs_to_is_passed_whichever_way_the_rim_runs()
 {
    for anticlockwise in [true, false] {
        let rim = square(anticlockwise);
        for at in [[3.0, 3.0], [4.0, 3.0], [-3.0, -3.0], [1.0, -4.5]] {
            assert!(
                rim.passed_by(about_the_square(at, -2.0)),
                "{at:?} below the rim, anticlockwise {anticlockwise}"
            );
        }
    }
}

#[test]
fn a_point_on_the_rim_s_level_or_beyond_it_from_the_wall_is_not_passed() {
    for anticlockwise in [true, false] {
        let rim = square(anticlockwise);
        for z in [-1.0, 0.0, 3.0] {
            assert!(
                !rim.passed_by(about_the_square([3.0, 3.0], z)),
                "at z = {z}, anticlockwise {anticlockwise}"
            );
        }
    }
}

#[test]
fn a_point_outside_the_wall_within_the_chords_or_on_one_is_not_passed() {
    for anticlockwise in [true, false] {
        let rim = square(anticlockwise);
        for at in [[4.0, 4.0], [4.0, 0.5], [4.0, 1.0], [1.0, 1.0], [0.0, 0.0]] {
            assert!(
                !rim.passed_by(about_the_square(at, -2.0)),
                "{at:?} below the rim, anticlockwise {anticlockwise}"
            );
        }
    }
}

/// `rows` cleared by `band` over a face from `low` to `high`, sorted as the
/// strips sort them.
fn cleared(band: &Band, rows: &[f64], ends: (f64, f64), levels: &[f64], room: f64) -> Vec<f64> {
    let mut kept = band.cleared(rows.to_vec(), ends, levels, room);
    kept.sort_by(f64::total_cmp);
    kept
}

#[test]
fn a_row_strictly_inside_a_band_is_taken_out_and_one_put_at_its_edge_on_either_side_of_the_equator()
{
    let above = Band {
        rim: 0.4,
        edge: 0.5,
    };
    assert_eq!(
        cleared(
            &above,
            &[-0.7, -0.2, 0.0, 0.42, 0.47, 0.7],
            (-1.0, 1.0),
            &[0.9],
            0.05
        ),
        [-0.7, -0.2, 0.0, 0.5, 0.7]
    );
    let below = Band {
        rim: -0.4,
        edge: -0.5,
    };
    assert_eq!(
        cleared(
            &below,
            &[-0.7, -0.47, -0.42, 0.0, 0.2, 0.7],
            (-1.0, 1.0),
            &[-0.9],
            0.05
        ),
        [-0.7, -0.5, 0.0, 0.2, 0.7]
    );
}

#[test]
fn a_row_on_either_end_of_a_band_is_not_inside_it() {
    let below = Band {
        rim: -0.4,
        edge: -0.5,
    };
    assert_eq!(
        cleared(
            &below,
            &[-0.7, -0.5, -0.4, -0.2],
            (-1.0, 1.0),
            &[-0.5],
            0.05
        ),
        [-0.7, -0.5, -0.4, -0.2]
    );
    let above = Band {
        rim: 0.4,
        edge: 0.5,
    };
    assert_eq!(
        cleared(&above, &[0.2, 0.4, 0.5, 0.7], (-1.0, 1.0), &[0.5], 0.05),
        [0.2, 0.4, 0.5, 0.7]
    );
}

#[test]
fn no_row_is_put_at_the_edge_of_a_band_no_row_fell_in() {
    let band = Band {
        rim: 0.4,
        edge: 0.5,
    };
    assert_eq!(
        cleared(&band, &[0.2, 0.7], (0.0, 1.0), &[0.9], 0.1),
        [0.2, 0.7]
    );
}

/// A face reaching from `ends.0` to `ends.1` along the meridian, the
/// `levels` in it and the `room` the sampling leaves about them.
struct Reach {
    ends: (f64, f64),
    levels: &'static [f64],
    room: f64,
}

/// Whether the row a band from 0.4 to 0.5 took out is put at its edge, over
/// faces of each reach: only where the edge stands further than the room
/// from both ends of the face and from every level, never at the room
/// exactly.
#[test]
fn the_edge_row_is_put_only_further_than_the_room_from_both_ends_of_the_face_and_from_every_level()
{
    let band = Band {
        rim: 0.4,
        edge: 0.5,
    };
    let reach = |ends: (f64, f64), levels: &'static [f64], room: f64| Reach { ends, levels, room };
    let cases = [
        (reach((0.0, 1.0), &[0.9], 0.1), true),
        (reach((0.45, 1.0), &[], 0.1), false),
        (reach((0.25, 1.0), &[], 0.25), false),
        (reach((0.0, 0.55), &[], 0.1), false),
        (reach((0.0, 0.75), &[], 0.25), false),
        (reach((0.0, 1.0), &[0.52], 0.1), false),
        (reach((0.0, 1.0), &[0.75], 0.25), false),
    ];
    for (Reach { ends, levels, room }, put) in cases {
        let expected: &[f64] = if put { &[0.5] } else { &[] };
        assert_eq!(
            cleared(&band, &[0.45], ends, levels, room),
            expected,
            "a face from {} to {}, levels {levels:?}, room {room}",
            ends.0,
            ends.1
        );
    }
}

/// The bands beside the rims of the face on the sphere of `body`, laid in
/// strips about z, from the samples of its laps and `pole`, a point of the
/// sphere inside the face, as the face is laid out from them.
fn bands_of(body: &Body, samples: &mut Samples, pole: DVec3) -> Vec<Band> {
    let (id, sphere) = on_the_sphere(body);
    let round = Rounded::Sphere(sphere.about(DVec3::Z));
    let mut laid: Vec<usize> = laps(body, samples, id).expect("the face's laps").concat();
    laid.push(samples.added(pole));
    bands(body, samples, id, (&sphere, &round), &laid)
}

#[test]
fn a_shaft_less_a_ball_meets_it_along_a_knife_edge_rim_with_a_band_reaching_away_from_the_equator()
{
    let (body, mut samples) = knife();
    let rim = the_rim(&body, &samples);
    let found = bands_of(&body, &mut samples, CENTRE - DVec3::Z * RADIUS);
    let [band] = &found[..] else {
        panic!("one band, found {}", found.len());
    };
    assert!(
        (band.rim + (reach() / RADIUS).asin()).abs() < 1e-9,
        "the band starts at {} along the meridian, the rim at −{}",
        band.rim,
        (reach() / RADIUS).asin()
    );
    assert!(
        band.edge < band.rim && (RADIUS * band.edge.cos() - rim.nearest).abs() < 1e-9,
        "the band reaches {} along the meridian, where the sphere stands {} from the axis, \
         the chords {}",
        band.edge,
        RADIUS * band.edge.cos(),
        rim.nearest
    );
}

#[test]
fn a_ball_joined_with_a_shaft_or_bored_by_it_meets_it_along_no_knife_edge_rim() {
    let ball = ball(TAU);
    for (what, step) in [
        ("joined with", ball.union(&shaft(&ball, 0.0))),
        ("bored by", ball.difference(&shaft(&ball, 0.0))),
    ] {
        let (body, mut samples) = read(step);
        let found = bands_of(&body, &mut samples, CENTRE + DVec3::Z * RADIUS);
        assert!(
            found.is_empty(),
            "the ball {what} the shaft: {} bands",
            found.len()
        );
    }
}

/// The point of the sphere between the rim's first two samples, halfway
/// from their chord to the wall, on the side of the rim the face lies on.
fn in_the_band(rim: &Rim) -> DVec3 {
    let middle = (rim.corners[0] + rim.corners[1]) / 2.0;
    let at = middle.normalize() * (middle.length() + SHAFT) / 2.0;
    CENTRE + rim.wall.u * at.x + rim.wall.v * at.y
        - DVec3::Z * (RADIUS * RADIUS - at.length_squared()).sqrt()
}

#[test]
fn a_face_on_the_sphere_is_kept_while_every_point_added_stands_within_the_wall_s_chords_and_refused_for_one_beyond_them()
 {
    let (body, mut samples) = knife();
    let (id, _) = on_the_sphere(&body);
    let rim = the_rim(&body, &samples);
    let beyond = samples.added(in_the_band(&rim));
    let within = samples.added(CENTRE + rim.wall.u * 4.0 - DVec3::Z * 3.0);
    assert!(rim.passed_by(samples.point(beyond)));
    assert!(within_the_walls(&body, &samples, id, (&[within], within)));
    assert!(!within_the_walls(&body, &samples, id, (&[beyond], beyond)));
    assert!(!within_the_walls(
        &body,
        &samples,
        id,
        (&[within, beyond], beyond)
    ));
    assert!(within_the_walls(
        &body,
        &samples,
        id,
        (&[beyond, within], within)
    ));
}

/// A shaft of the ball's radius from the ball's centre up past it, less the
/// ball: the wall touches the sphere along its equator, and the face on the
/// sphere, the dimple over it, meets the wall there. Both run up from that
/// rim, but it stands level with the sphere's middle, where the sphere runs
/// square to the axis: no band reaches from it.
#[test]
fn a_rim_on_the_sphere_s_equator_reaches_no_band_though_the_wall_and_the_face_run_one_way_from_it()
{
    let ball = ball(TAU);
    let (body, mut samples) = read(shaft_between(&ball, RADIUS, 0.0, 10.0).difference(&ball));
    let (_, sphere) = on_the_sphere(&body);
    let rim = the_rim(&body, &samples);
    assert_eq!(
        rim.level,
        rim.height(sphere.middle()),
        "the rim stands on the sphere's equator"
    );
    assert!(rim.side > 0.0, "the wall runs {} from the rim", rim.side);
    let found = bands_of(&body, &mut samples, CENTRE + DVec3::Z * RADIUS);
    assert!(found.is_empty(), "{} bands beside the equator", found.len());
}

/// How far under the radius of a ball about the origin the short shafts of
/// #1127 stand: past decision 12's reach at the reach of 5 the ball and a
/// shaft from its south pole stand to, 1e-7, and within a hundredth of a
/// step of the equator's row of its sphere, drawn at that reach, 1.772e-7.
const HAIR: f64 = 1.2e-7;

/// The drawing's tolerance, 2e-4 of the reach, at the reach of 5 of a ball
/// of [`RADIUS`] about the origin.
const AT_FIVE: f64 = 1e-3;

/// The radius of the ball the shell is cut from, about the origin.
const SHELL: f64 = 8.0;

/// The drawing's tolerance at the shell's reach of [`SHELL`].
const AT_EIGHT: f64 = 1.6e-3;

/// How far under its cavity's radius the shaft joined with the shell
/// stands: past decision 12's reach at the reach of 8, 1.6e-7, and within a
/// hundredth of a step of the row of the cavity's equator, drawn at that
/// reach, 2.79e-7.
const SHELL_HAIR: f64 = 2e-7;

/// Nearer than this, a sample stands on a circle or a sphere: a millionth of
/// the drawing's tolerance at the reach of 5.
const ON: f64 = 1e-9;

/// A ball of `radius` about the origin, a half disc turned `turn` about the
/// line through it along `axis` from x, or from a direction square to a
/// slanted axis: the half ball y ≥ 0 for half a turn about z.
fn ball_at_the_origin(radius: f64, turn: f64, axis: DVec3) -> crate::Body {
    let sampled: &'static [DVec2] = Box::leak(
        (0..=32)
            .map(|step| DVec2::from_angle(-PI / 2.0 + PI * f64::from(step) / 32.0) * radius)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let half = Contour {
        corners: vec![DVec2::new(0.0, -radius), DVec2::new(0.0, radius)],
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
        sampled: crate::Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let frame = Frame {
        origin: DVec3::ZERO,
        u: if axis == DVec3::Z {
            DVec3::X
        } else {
            axis.any_orthonormal_vector()
        },
        v: axis,
    };
    let along_v = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    crate::Body::default()
        .tool_turned(&profile, frame, &Turn::of(along_v, turn, 0.0, &profile))
        .expect("the exact kernel turns the ball")
}

/// A shaft of `radius` along z through the origin from z = `from` to `to`,
/// a tool of `part`.
fn short_shaft(part: &crate::Body, radius: f64, from: f64, to: f64) -> crate::Body {
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

/// How far under the equator about z a sphere of `radius` about the origin
/// crosses a coaxial wall `hair` inside it, and that rim's angle along the
/// meridian: a = √(r² − (r − h)²), at −atan(a / (r − h)).
fn rim_under(radius: f64, hair: f64) -> (f64, f64) {
    let reach = (hair * (2.0 * radius - hair)).sqrt();
    (reach, (-reach).atan2(radius - hair))
}

/// The face of `body` on the sphere of `radius`, and the sphere.
fn on_the_sphere_of(body: &Body, radius: f64) -> (FaceId, Sphere) {
    body.face_ids()
        .find_map(|id| match body.surface(body.face(id).surface) {
            Surface::Sphere(sphere) if (sphere.radius - radius).abs() <= ON => Some((id, *sphere)),
            _ => None,
        })
        .unwrap_or_else(|| panic!("a face on the sphere of {radius}"))
}

/// The rims and ends [`ends`] finds beside the face of `body` on the sphere
/// of `radius`, read in its own chart.
fn ends_beside(body: &Body, radius: f64) -> Vec<[f64; 2]> {
    let (id, sphere) = on_the_sphere_of(body, radius);
    ends(
        body,
        body.face(id),
        &Rounded::Sphere(sphere),
        body.scale().eps(),
    )
}

/// That `found` is one rim at the angle `rim` along the meridian, with the
/// wall from it ending on the equator, both within the rounding of an
/// angle a ten-thousandth of a radian or less.
fn one_end_on_the_equator(found: &[[f64; 2]], rim: f64, what: &str) {
    let [[at, end]] = found[..] else {
        panic!("{what}: one wall ending beside a rim, found {found:?}");
    };
    assert!(
        (at - rim).abs() <= 1e-12 && end.abs() <= 1e-12,
        "{what}: the rim at {at}, where the sphere crosses the wall at {rim}, the wall ending at \
         {end}, where the shaft's top stands on the equator"
    );
}

#[test]
fn a_half_ball_less_a_short_shaft_a_hair_under_its_radius_has_its_rim_found_with_the_shaft_s_top_its_wall_ends_on()
 {
    let half = ball_at_the_origin(RADIUS, PI, DVec3::Z);
    let shaft = short_shaft(&half, RADIUS - HAIR, -RADIUS, 0.0);
    let (body, _) = read_at(half.difference(&shaft), AT_FIVE);
    let (_, rim) = rim_under(RADIUS, HAIR);
    one_end_on_the_equator(
        &ends_beside(&body, RADIUS),
        rim,
        "the half ball less the shaft",
    );
}

/// The ball of [`SHELL`] less the ball of [`RADIUS`], joined with a shaft of
/// the cavity's radius less [`SHELL_HAIR`] from the cavity's south pole up to
/// its centre: the wall runs from the rim on the cavity's sphere up to the
/// shaft's top, on the equator, their outward normals opposed, and what lies
/// between them is the void the shaft leaves of the cavity, not matter.
#[test]
fn a_shell_joined_with_a_short_shaft_a_hair_under_its_cavity_s_radius_has_its_rim_found_with_the_shaft_s_top_though_the_wedge_between_them_is_void()
 {
    let shell = ball_at_the_origin(SHELL, TAU, DVec3::Z)
        .difference(&ball_at_the_origin(RADIUS, TAU, DVec3::Z))
        .expect("the shell is built");
    let shaft = short_shaft(&shell, RADIUS - SHELL_HAIR, -RADIUS, 0.0);
    let (body, _) = read_at(shell.union(&shaft), AT_EIGHT);
    let (_, rim) = rim_under(RADIUS, SHELL_HAIR);
    one_end_on_the_equator(
        &ends_beside(&body, RADIUS),
        rim,
        "the shell joined with the shaft",
    );
    let outer = ends_beside(&body, SHELL);
    assert!(
        outer.is_empty(),
        "the shell's outer sphere meets no wall: {outer:?}"
    );
}

/// The half ball joined with the same shaft: the wall the join keeps runs
/// from the rim down to the shaft's foot, away from the face on the sphere,
/// their outward normals alike there.
#[test]
fn a_half_ball_joined_with_a_short_shaft_a_hair_under_its_radius_has_no_wall_ending_beside_its_rim_the_wall_running_away_from_the_face()
 {
    let half = ball_at_the_origin(RADIUS, PI, DVec3::Z);
    let shaft = short_shaft(&half, RADIUS - HAIR, -RADIUS, 0.0);
    let (body, _) = read_at(half.union(&shaft), AT_FIVE);
    let found = ends_beside(&body, RADIUS);
    assert!(found.is_empty(), "{found:?}");
}

/// The face on the sphere of [`RADIUS`] of `body`, a ball about the origin
/// less a short shaft `hair` under its radius, cut as the drawing cuts it at
/// `tolerance`: the samples of its rim, the circle a under the equator
/// about z, and the points it is cut at on that equator, the row of the
/// strips about z within a hundredth of a step of the rim.
fn beside_the_rim(
    body: &Body,
    samples: &mut Samples,
    tolerance: f64,
    hair: f64,
) -> (Vec<DVec3>, Vec<DVec3>) {
    let (id, _) = on_the_sphere_of(body, RADIUS);
    let (under, _) = rim_under(RADIUS, hair);
    let edge = (body.face(id).loops.iter().flatten())
        .find_map(|coedge| match body.curve(body.edge(coedge.edge).curve) {
            Curve::Circle(circle)
                if circle.axis.cross(DVec3::Z).length() <= ON
                    && (circle.center + DVec3::Z * under).length() <= ON =>
            {
                Some(coedge.edge)
            }
            _ => None,
        })
        .expect("the face on the sphere is bounded by the rim");
    let rim: Vec<DVec3> = (samples.edge(edge).iter())
        .map(|&sample| samples.point(sample))
        .collect();
    let (outline, _) =
        Outline::cut_of(body, samples, id, tolerance).expect("the face on the sphere is cut");
    let row: Vec<DVec3> = (outline.samples.iter())
        .map(|&sample| samples.point(sample))
        .filter(|point| point.z.abs() <= ON && (point.length() - RADIUS).abs() <= ON)
        .collect();
    (rim, row)
}

/// The samples of `rim` standing at an angle about z no point of `row`
/// stands at.
fn between_the_row_s_points(rim: &[DVec3], row: &[DVec3]) -> Vec<DVec3> {
    let way = |point: &DVec3| point.truncate().normalize();
    (rim.iter())
        .filter(|sample| {
            !row.iter()
                .any(|point| way(point).distance(way(sample)) <= ON)
        })
        .copied()
        .collect()
}

/// The half ball about z less the short shaft: its grid of angles is lined
/// up with the shaft's wall (#1123), each sample of the rim on one of its
/// angles, so the strips cut the face at the equator's row the wall ends on
/// — at its points off the half ball's flat face, not at the meridians'
/// samples alone, which every row has — and every sample of the rim stands
/// under one of them.
#[test]
fn a_half_ball_about_z_less_a_short_shaft_is_cut_at_the_row_beside_its_rim_every_sample_of_the_rim_standing_at_the_angle_of_a_point_of_the_row()
 {
    let half = ball_at_the_origin(RADIUS, PI, DVec3::Z);
    let shaft = short_shaft(&half, RADIUS - HAIR, -RADIUS, 0.0);
    let (body, mut samples) = read_at(half.difference(&shaft), AT_FIVE);
    let (rim, row) = beside_the_rim(&body, &mut samples, AT_FIVE, HAIR);
    assert!(
        row.iter().any(|point| point.y > ON),
        "the face is cut at the equator's row inside it, not at its meridians alone: {row:?}"
    );
    let between = between_the_row_s_points(&rim, &row);
    assert!(
        between.is_empty(),
        "{} of the rim's {} samples stand between the row's {} points: {between:?}",
        between.len(),
        rim.len(),
        row.len()
    );
}

/// The ball turned at a slant about (1, 0, 1)/√2 less the same shaft: its
/// face on the sphere is read about z (`charted.rs`'s `about`), on a grid of
/// angles cut for the sphere alone and not lined up with the wall's, the
/// samples of the rim between its angles. The face is cut at the equator's
/// row only if every sample of the rim stands at the angle of one of the
/// row's points; left out, as on `main`, it is not cut there at all.
#[test]
fn a_ball_turned_at_a_slant_less_a_short_shaft_is_cut_at_a_row_beside_its_rim_only_where_every_sample_of_the_rim_stands_at_the_angle_of_a_point_of_the_row()
 {
    let ball = ball_at_the_origin(RADIUS, TAU, DVec3::new(1.0, 0.0, 1.0).normalize());
    let shaft = short_shaft(&ball, RADIUS - HAIR, -RADIUS, 0.0);
    let (body, mut samples) = read_at(ball.difference(&shaft), AT_FIVE);
    let (rim, row) = beside_the_rim(&body, &mut samples, AT_FIVE, HAIR);
    let between = between_the_row_s_points(&rim, &row);
    assert!(
        row.is_empty() || between.is_empty(),
        "the face is cut at the equator's row, at {} points, and {} of the rim's {} samples \
         stand between them: {:?}",
        row.len(),
        between.len(),
        rim.len(),
        &between[..between.len().min(4)]
    );
}
