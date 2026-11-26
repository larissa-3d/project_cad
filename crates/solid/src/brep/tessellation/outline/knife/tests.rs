//! The knife-edge rims of a face of a sphere, read directly (#1138): what a
//! rim reads off the body it is drawn on, which points it passes, which rows
//! the band beside it clears, which rims are knife edges at all, and which
//! faces the central chart leaves to the charts after it.

use std::f64::consts::{FRAC_1_SQRT_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::super::laps;
use super::{Band, Rim, bands, rims, within_the_walls};
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
    let body = step
        .expect("the step is built")
        .brep()
        .expect("the exact kernel computed the step")
        .clone();
    let samples = Samples::of(&body, TOLERANCE);
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
