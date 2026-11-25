//! A circle of a wall lying on a sphere about another line is sampled where
//! it crosses the steps of the sphere's grid (#940). Those crossings go on
//! to the wall's other circles, and to every circle of a wall beside it, only
//! where contact pairs the wall with another standing within a chord's sag of
//! it (#1109): a shaft laid onto a ball's touch, its end a hair inside the
//! ball, takes them on its rim far from the ball as on the ball's circle a
//! hair inside its end; a shaft through the ball, close to no other wall,
//! keeps them on its circles lying on the ball.

use std::collections::BTreeSet;
use std::f64::consts::{FRAC_1_SQRT_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::super::super::contact::{self, Zones};
use super::super::Samples;
use super::super::steps::span;
use crate::Body as Part;
use crate::Loop;
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::topology::{Body, EdgeId, SurfaceId};
use crate::profile::{Contour, Frame, Profile, Run};
use crate::turning::{Axis, Turn};

/// The radius of the ball, about the origin.
const RADIUS: f64 = 5.0;

/// The axis the ball is turned about, at a slant: (1, 0, 1)/√2.
const SLANT: DVec3 = DVec3::new(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2);

/// The drawing's tolerance at the reach of 10 the shafts stand to, and a
/// coarser one.
const TOLERANCES: [f64; 2] = [2e-3, 0.01];

/// An outline with its runs, as the drawing hands it to the exact kernel.
fn exact(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

/// A ball of [`RADIUS`] about the origin: a half disc turned whole about the
/// line through the origin along `axis`, its sphere's chart laid about that
/// line.
fn ball_about(axis: DVec3) -> Part {
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
        origin: DVec3::ZERO,
        u: axis.any_orthonormal_vector(),
        v: axis,
    };
    let along_v = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    Part::default()
        .tool_turned(&profile, frame, &Turn::of(along_v, TAU, 0.0, &profile))
        .expect("the ball turns")
}

/// A shaft of `radius` along z through the origin, from z = `from` to `to`,
/// a tool of `part`.
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

/// The ball turned at a [`SLANT`] joined with a shaft of 4.9 along z
/// through its centre, from z = −10 to 10: the shaft's wall meets the sphere
/// along the circles z = ±√(25 − 4.9²), of another axis than the sphere's
/// chart, and stands close to no other wall.
pub(in crate::brep::tessellation::sampling) fn ball_through_a_shaft() -> Part {
    let ball = ball_about(SLANT);
    ball.union(&shaft(&ball, 4.9, -10.0, 10.0))
        .expect("the ball and the shaft join")
}

/// The ball turned at a [`SLANT`] joined with a shaft along z through its
/// centre two tolerances under its radius, from the plane 1e-5 above the
/// lower circle its wall meets the sphere along, inside the band, to z = 10:
/// the shaft laid onto the touch, and the ball's circle on its end plane a
/// hair inside its rim, on a wall of its own.
fn ball_on_a_shaft_laid_onto_its_touch() -> Part {
    let radius = RADIUS - 2.0 * Scale::of(10.0).eps();
    let inside = (RADIUS * RADIUS - radius * radius).sqrt() - 1e-5;
    let ball = ball_about(SLANT);
    ball.union(&shaft(&ball, radius, -inside, 10.0))
        .expect("the ball and the shaft join")
}

/// A shaft of radius 4 along z from z = 0 to 10 bored along its axis with a
/// bore of radius 1: two coaxial walls 3 apart, facing each other.
fn tube() -> Part {
    let rod = Part::default()
        .union(&shaft(&Part::default(), 4.0, 0.0, 10.0))
        .expect("the rod is built");
    rod.difference(&shaft(&rod, 1.0, -1.0, 11.0))
        .expect("the rod is bored")
}

/// The exact body of `part`, and its walls as contact reads them.
fn walls_of(part: &Part) -> (&Body, Vec<contact::Wall>) {
    let body = part.brep().expect("the body is exact");
    (body, contact::walls(body))
}

/// The walls `paired` reads as standing close to another in `body`, at
/// `tolerance`.
fn paired(body: &Body, walls: &[contact::Wall], tolerance: f64) -> BTreeSet<SurfaceId> {
    let zones = Zones::of(body, walls);
    contact::paired(walls, &zones, (tolerance, span(body)), body.scale().eps())
}

/// The tube's two walls, parallel and facing each other 3 apart — a pair
/// of [`Zones`], as every two parallel walls are — are not paired. A shaft
/// laid onto the touch of a ball turned at a slant, its end inside the band,
/// and the ball's circle on its end plane a hair inside its rim stand within
/// a chord's sag of each other, and are both paired.
#[test]
fn two_coaxial_walls_far_apart_are_not_paired_and_a_shaft_laid_onto_a_ball_s_touch_and_the_ball_s_circle_a_hair_inside_its_end_are()
 {
    let tube = tube();
    let (body, walls) = walls_of(&tube);
    assert_eq!(walls.len(), 2, "the tube has two walls: {walls:?}");
    let laid = ball_on_a_shaft_laid_onto_its_touch();
    let (on_the_touch, beside) = walls_of(&laid);
    let both: BTreeSet<SurfaceId> = beside.iter().map(|(id, _)| *id).collect();
    assert_eq!(
        both.len(),
        2,
        "the laid shaft's wall and the ball's circle's: {beside:?}"
    );
    for tolerance in TOLERANCES {
        assert_eq!(
            paired(body, &walls, tolerance),
            BTreeSet::new(),
            "the tube's walls, 3 apart, at {tolerance}"
        );
        assert_eq!(
            paired(on_the_touch, &beside, tolerance),
            both,
            "the laid shaft's wall and the ball's circle's, a hair apart, at {tolerance}"
        );
    }
}

/// The angles round z, from x, of every sample of `edge`, sorted, each once.
fn angles(samples: &Samples, circle: &Circle, edge: EdgeId) -> Vec<f64> {
    let mut angles: Vec<f64> = samples
        .edge(edge)
        .iter()
        .map(|id| {
            let from = samples.point(*id) - circle.center;
            from.y.atan2(from.x).rem_euclid(TAU)
        })
        .collect();
    angles.sort_by(f64::total_cmp);
    angles.dedup_by(|later, earlier| *later - *earlier <= 1e-12);
    angles
}

/// Whether `angles`, sorted, stand evenly round the turn: every gap between
/// one and the next the same, the last round to the first too.
fn even(angles: &[f64]) -> bool {
    let gaps: Vec<f64> = angles
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .chain([angles[0] + TAU - angles[angles.len() - 1]])
        .collect();
    gaps.iter().all(|gap| (gap - gaps[0]).abs() <= 1e-9)
}

/// The circles of `body`, each with its samples' angles round z, at
/// `tolerance`.
fn sampled(body: &Body, tolerance: f64) -> Vec<(Circle, Vec<f64>)> {
    let samples = Samples::of(body, tolerance);
    body.edge_ids()
        .filter_map(|edge| match body.curve(body.edge(edge).curve) {
            Curve::Circle(circle) => Some((*circle, angles(&samples, circle, edge))),
            Curve::Line(_) | Curve::Meet(_) | Curve::Conic(_) => None,
        })
        .collect()
}

/// [`ball_through_a_shaft`]: the shaft's two circles on the sphere are
/// sampled where they cross the steps of the sphere's grid, unevenly, and
/// its wall stands close to no other, so its circles on the end planes, at
/// z = ±10, take none of those crossings — sampled evenly, on the wall's
/// grid alone, at fewer angles. [`ball_on_a_shaft_laid_onto_its_touch`]: the
/// laid shaft's wall and the ball's circle's stand a hair apart, and every
/// circle of both takes the crossings — the shaft's rim at z = 10, far from
/// the ball, at the very angles of the ball's circle.
#[test]
fn a_wall_close_to_no_other_keeps_the_crossings_of_a_slanted_sphere_s_grid_on_its_circles_on_the_sphere_and_one_close_to_another_hands_them_to_every_circle()
 {
    let through = ball_through_a_shaft();
    let laid = ball_on_a_shaft_laid_onto_its_touch();
    let (through, laid) = (
        through.brep().expect("the ball and the shaft are exact"),
        laid.brep().expect("the ball and the laid shaft are exact"),
    );
    for tolerance in TOLERANCES {
        let circles = sampled(through, tolerance);
        let (ends, rims): (Vec<_>, Vec<_>) = circles
            .iter()
            .partition(|(circle, _)| circle.center.z.abs() > RADIUS);
        assert_eq!(
            (ends.len(), rims.len()),
            (2, 2),
            "the shaft through the ball has two circles on its end planes and two on the sphere"
        );
        for (rim, on_the_sphere) in &rims {
            assert!(
                !even(on_the_sphere) && on_the_sphere.len() > ends[0].1.len(),
                "the circle at z = {} on the sphere takes the crossings of its grid, at \
                 {tolerance}: {} angles",
                rim.center.z,
                on_the_sphere.len()
            );
        }
        for (end, on_its_grid) in &ends {
            assert!(
                even(on_its_grid),
                "the shaft's circle at z = {}, its wall close to no other, takes no crossing at \
                 {tolerance}: {} angles, unevenly",
                end.center.z,
                on_its_grid.len()
            );
        }

        let circles = sampled(laid, tolerance);
        let far = circles
            .iter()
            .find(|(circle, _)| circle.center.z > RADIUS)
            .expect("the laid shaft has a rim at z = 10");
        let ball_s = circles
            .iter()
            .find(|(circle, _)| circle.radius < RADIUS)
            .expect("the ball leaves a circle on the laid shaft's end plane");
        assert!(
            !even(&far.1)
                && far.1.len() == ball_s.1.len()
                && far
                    .1
                    .iter()
                    .zip(&ball_s.1)
                    .all(|(one, other)| (one - other).abs() <= 1e-9),
            "the laid shaft's rim at z = 10 takes the crossings the ball's circle takes, at \
             {tolerance}: {} angles, where the ball's circle has {}",
            far.1.len(),
            ball_s.1.len()
        );
    }
}
