//! A shaft along z through a ball's centre, a few tens of the kernel's
//! tolerances under its radius, meets the sphere along two circles a
//! thousandth or two either side of its equator about z. That is further off
//! than the hair decision 12 lays onto the touch, so the shaft is cut as it
//! was drawn, and the ball it bores keeps a ring between the shaft's wall and
//! its sphere that thins to nothing along those circles. Where the shaft's
//! end plane stands inside the band between them, the ball less the shaft is
//! drawn uncrossed, in the time the same cut takes beside a shaft a hair
//! under the radius that decision 12 lays onto the touch, whatever axis the
//! ball was turned about.
//!
//! Closes #1137.
//! - The ball of 5 about z, and turned at a slant about (1, 0, 1)/√2, less
//!   the shaft of 5 − 40 tolerances ending at z = ±(a − 1e-5): built, exact,
//!   closed, uncrossed, the arithmetic's volume, each in under a second in
//!   release —
//!   `a_ball_turned_about_z_or_at_a_slant_less_a_shaft_forty_tolerances_under_its_radius_ending_inside_the_band_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume_each_cut_in_under_a_second`.
//!   Read as: the ball a half disc of radius 5 turned whole about the line
//!   through the origin along z, or along (1, 0, 1)/√2, its sphere's chart
//!   laid about that line, as #956's and #1109's balls are; the shaft a round
//!   about the z axis of radius b, 5 less 40 of the kernel's tolerances at
//!   the reach the ball and the shaft are read at, as #1109's file reckons
//!   its 2, 5 and 15 — 10 for a shaft standing out to z = ±10, the ball's 5
//!   for one standing between the two planes; a where the shaft's wall meets
//!   the sphere, √(5² − b²); the shaft standing from the plane z = −(a − 1e-5)
//!   up to z = 10, from z = −10 up to the plane z = a − 1e-5, or between the
//!   two, so that either end, or both, stands inside the band, as #1109's
//!   shafts stand; less, the ball cut by the shaft; built, never declined;
//!   exact, the exact kernel computed the step; closed and uncrossed, its
//!   triangles so by `soundness`; the arithmetic's volume, within a
//!   billionth of the matter in play, the ball and the shaft, of the ball
//!   less the slices of it the shaft takes — the shaft's whole disc within a
//!   of the equator, the ball's π (5² − z²) beyond — or of that with the
//!   shaft laid onto the touch, as #1109 allows, the two standing well within
//!   that billionth of each other here; each in under a second in release,
//!   each cut timed alone on the wall clock, once, the shaft raised before
//!   and the checks run after, and held under [`QUICK`] in the profile the
//!   tests run in. The cut draws the body as it builds it, so the time is
//!   that of the drawing too. That profile builds `cao_solid` and every
//!   dependency at `opt-level` 2 (the workspace's `Cargo.toml`): it is no
//!   faster than release, and the same cuts beside the shafts of 2, 5 and 15
//!   tolerances took as long there as in release, about a tenth of a second,
//!   so a cut held under a second there is under a second in release, and
//!   its neighbours meet that bound with room several times over
//! - #956's and #1109's tests keep their bodies — no test: they are the
//!   tests of `spheres_of_one_centre_and_radius_are_one_surface_whatever_their_axes.rs`
//!   and of
//!   `a_shaft_a_hair_under_a_ball_s_radius_its_end_inside_the_band_is_laid_onto_the_touch_whatever_axis_the_ball_was_turned_about.rs`,
//!   those #956 added to `src/brep/relation/tests/round.rs`,
//!   `src/brep/selection/tests/round.rs`,
//!   `src/brep/tessellation/sampling/tests/rows.rs` and
//!   `crates/part/tests/the_exact_kernel_turns_the_part.rs`, and those #1109
//!   added to `src/brep/tessellation/sampling/axis/tests.rs`,
//!   `src/brep/tessellation/sampling/tests/conic.rs` and
//!   `src/brep/tessellation/sampling/tests/crossed.rs`; running them
//!   unchanged is the answer
//!
//! Beyond the criteria: the same cuts drawn within the drawing's tolerance of
//! their surfaces, as #1138 holds the knife-edge rim a shaft less a ball
//! leaves — every triangle of a curved face within [`SAG`] of it, and the
//! matter the triangles enclose within [`SPREAD`] of it over their area. A
//! cut made uncrossed by drawing the ring coarser than the drawing's
//! tolerance would not be drawn the way the issue asks —
//! `a_ball_turned_about_z_or_at_a_slant_less_a_shaft_forty_tolerances_under_its_radius_ending_inside_the_band_is_drawn_within_the_drawing_s_tolerance_of_its_surfaces`.
//! The cuts are drawn so already, crossed as they are; the test is a net
//! under the change.

use std::f64::consts::{PI, TAU};
use std::time::{Duration, Instant};

use cao_solid::brep::Scale;
use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Loop};
use glam::{DVec2, DVec3};

/// How far the matter a body holds may stand from the arithmetic's, as a
/// share of the matter in play: the kernel's billionth.
const EXACT: f64 = 1e-9;

/// The radius of every ball here, about the origin.
const RADIUS: f64 = 5.0;

/// The ball of [`RADIUS`].
const BALL: f64 = 4.0 * PI / 3.0 * RADIUS * RADIUS * RADIUS;

/// How many of the kernel's tolerances the shaft stands under the ball's
/// radius: twice the hair, [`Scale::HAIR`] of them, within which decision 12
/// lays a shaft onto the touch.
const UNDER: f64 = 40.0;

/// How far inside the band the shaft's end plane stands, from the circle the
/// shaft's wall meets the sphere along.
const INSIDE: f64 = 1e-5;

/// How long one cut may take on the wall clock, the shaft raised before it
/// and the checks run after: the criterion's second.
const QUICK: Duration = Duration::from_secs(1);

/// The balls the criterion names, each with how it was turned.
fn balls() -> [(&'static str, DVec3); 2] {
    [
        ("about z", DVec3::Z),
        (
            "at a slant, about (1, 0, 1)/√2,",
            DVec3::new(1.0, 0.0, 1.0).normalize(),
        ),
    ]
}

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
/// line, its poles 5 along it either way. #956's.
fn ball_about(axis: DVec3) -> Body {
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
    Body::default()
        .tool_turned(&profile, frame, &Turn::of(along_v, TAU, 0.0, &profile))
        .expect("the exact kernel turns the ball")
}

/// A shaft of `radius` along z through the origin, from z = `from` to `to`,
/// a tool of `part`.
fn shaft_along_z(part: &Body, radius: f64, from: f64, to: f64) -> Body {
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

/// The shafts the criterion names, each with where it starts and ends: of
/// the ball's radius less [`UNDER`] of the tolerance it and the ball are read
/// at, its wall meeting the sphere along the circles z = ±a, a = √(r² − b²),
/// a thousandth or two either side of the equator about z; standing from the
/// plane [`INSIDE`] above z = −a, within the band between those circles, up
/// to z = 10, past the ball; from z = −10 up to the plane [`INSIDE`] under
/// z = a; or between the two planes. The first two are read at the reach of
/// 10 they stand to, the third at the ball's. #1109's, further under.
fn shafts() -> [(f64, f64, f64); 3] {
    let under = |reach: f64| RADIUS - UNDER * Scale::of(reach).eps();
    let inside = |radius: f64| (RADIUS * RADIUS - radius * radius).sqrt() - INSIDE;
    let [far, near] = [under(10.0), under(RADIUS)];
    [
        (far, -inside(far), 10.0),
        (far, -10.0, inside(far)),
        (near, -inside(near), inside(near)),
    ]
}

/// What a shaft of `radius` along z through the ball's centre, from z =
/// `from` to `to`, holds, and what it takes of the ball: π b² (`to` −
/// `from`), and the ball's slices between, each the disc of the shaft or the
/// ball's own where that is the smaller — the shaft's within a of the
/// equator, a = √(r² − b²), the ball's π (r² − z²) beyond. #1109's.
fn shaft_and_its_share_of_the_ball(radius: f64, from: f64, to: f64) -> [f64; 2] {
    let reach = (RADIUS * RADIUS - radius * radius).max(0.0).sqrt();
    let round = |low: f64, high: f64| {
        PI * (RADIUS * RADIUS * (high - low) - (high.powi(3) - low.powi(3)) / 3.0)
    };
    let below = |height: f64| {
        let height = height.clamp(-RADIUS, RADIUS);
        round(-RADIUS, height.min(-reach))
            + PI * radius * radius * (height.clamp(-reach, reach) + reach)
            + round(reach, height.max(reach))
    };
    [PI * radius * radius * (to - from), below(to) - below(from)]
}

/// Why `body` is not sound, if it is not: not exact, or its triangles open
/// or crossing. #1109's.
fn unsound(body: &Body) -> Option<String> {
    if !body.is_exact() {
        return Some("not exact".to_string());
    }
    let triangles = body.triangles();
    if let Err(open) = closed(&triangles) {
        return Some(format!("its triangles open, {open:?}"));
    }
    if let Err(crossing) = uncrossed(&triangles) {
        return Some(format!("its triangles crossing, {crossing:?}"));
    }
    None
}

/// What the cut of the ball by each shaft is called in a failure.
fn named(about: &str, radius: f64, from: f64, to: f64) -> String {
    format!(
        "the ball of {RADIUS} turned {about} less a shaft along z through its centre of radius \
         {radius} from z = {from} to {to}"
    )
}

/// The ball about z and the ball turned at a slant, each less each of
/// [`shafts`]: every cut built, exact, its triangles closed and uncrossed,
/// holding within a billionth of the ball and the shaft what the arithmetic
/// leaves with the shaft as drawn or laid onto the touch, and made, drawing
/// and all, within [`QUICK`]. Every cut that is not is named, with each way
/// it is not.
#[test]
fn a_ball_turned_about_z_or_at_a_slant_less_a_shaft_forty_tolerances_under_its_radius_ending_inside_the_band_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume_each_cut_in_under_a_second()
 {
    let mut wrong = Vec::new();
    for (about, axis) in balls() {
        let ball = ball_about(axis);
        for (radius, from, to) in shafts() {
            let what = named(about, radius, from, to);
            let shaft = shaft_along_z(&ball, radius, from, to);
            let started = Instant::now();
            let built = ball.difference(&shaft);
            let took = started.elapsed();
            if took > QUICK {
                wrong.push(format!("{what}: cut in {took:?}, past {QUICK:?}"));
            }
            let body = match built {
                Err(declined) => {
                    wrong.push(format!("{what}: declined, {declined:?}"));
                    continue;
                }
                Ok(body) => body,
            };
            if let Some(why) = unsound(&body) {
                wrong.push(format!("{what}: {why}"));
            }
            let [held, drawn] = shaft_and_its_share_of_the_ball(radius, from, to);
            let [_, laid] = shaft_and_its_share_of_the_ball(RADIUS, from, to);
            let [drawn, laid] = [BALL - drawn, BALL - laid];
            if [drawn, laid]
                .iter()
                .all(|volume| (body.volume() - volume).abs() > EXACT * (BALL + held))
            {
                wrong.push(format!(
                    "{what}: holds {}, where the arithmetic leaves {drawn} as drawn or {laid} \
                     laid onto the touch",
                    body.volume()
                ));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a ball less a shaft a few tens of tolerances under its radius, ending inside the band, \
         is built exact, closed, uncrossed and of the arithmetic's volume, each cut in under a \
         second, whatever axis the ball was turned about; not so:\n{}",
        wrong.join("\n")
    );
}

/// How far the triangles of an exact body may stand from its surfaces, as a
/// share of its reach: `body/drawn.rs`'s `DRAWN`, which no public constant
/// gives.
const DRAWN: f64 = 2e-4;

/// How many of the drawing's tolerances a triangle drawn on a curved face
/// may stand off that face's surface, at its centre or the middle of a side:
/// #1138's. The drawing holds its chords to the tolerance, not the middles of
/// its triangles, and the ball turned at a slant, alone, is drawn some 1.4
/// of it off there.
const SAG: f64 = 1.5;

/// How many of the drawing's tolerances, spread over every triangle's area,
/// the matter the triangles enclose may stand off the matter the body holds:
/// one, as though every triangle stood a whole tolerance off its surface on
/// the same side. #1138's.
const SPREAD: f64 = 1.0;

/// Closer than this to a surface, a corner of a triangle stands on it: a
/// two-thousandth of the drawing's tolerance at a reach of 10, and a
/// thousand times the kernel's own. #1138's.
const ON: f64 = 1e-6;

/// How far the triangles of `body` may stand from its surfaces: [`DRAWN`] of
/// the reach its bounds stand at, as [`Scale`] reads a reach.
fn tolerance(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a body holding matter has bounds");
    DRAWN * Scale::of(low.abs().max(high.abs()).max_element()).reach()
}

/// Where `body`'s drawing stands further from its surfaces than [`SAG`] of
/// the drawing's tolerance allows, or encloses further from what it holds
/// than [`SPREAD`] allows. Each curved face is read as drawn on the sphere
/// of the ball when every corner of its triangles stands on that sphere, and
/// as drawn on a wall about z when every corner stands as far from z as the
/// furthest does; a curved face drawn on neither is named. #1138's, but that
/// a ball less a shaft is never empty, and a body drawn with nothing is
/// named.
fn drawn_off(body: &Body, what: &str) -> Vec<String> {
    if body.is_empty() {
        return vec![format!(
            "{what}: empty, where the ball less the shaft holds matter"
        )];
    }
    let tolerance = tolerance(body);
    let mut wrong = Vec::new();
    for face in (0..body.faces_end()).chain(body.pieces()) {
        if !body.has_face(face) || body.is_flat(face) {
            continue;
        }
        let triangles: Vec<[DVec3; 3]> = body.triangles_of(face).collect();
        let corners = || triangles.iter().flatten();
        let wall = corners()
            .map(|corner| corner.truncate().length())
            .fold(0.0, f64::max);
        let (surface, off): (String, Box<dyn Fn(DVec3) -> f64>) =
            if corners().all(|corner| (corner.length() - RADIUS).abs() < ON) {
                (
                    "the ball's sphere".to_string(),
                    Box::new(|point: DVec3| (point.length() - RADIUS).abs()),
                )
            } else if corners().all(|corner| wall - corner.truncate().length() < ON) {
                (
                    format!("a wall of radius {wall} about z"),
                    Box::new(move |point: DVec3| (point.truncate().length() - wall).abs()),
                )
            } else {
                wrong.push(format!(
                    "{what}: its face {face} is curved and drawn on neither the ball's sphere \
                     nor a wall about z"
                ));
                continue;
            };
        let worst = triangles
            .iter()
            .flat_map(|&[a, b, c]| {
                [
                    (a + b + c) / 3.0,
                    (a + b) / 2.0,
                    (b + c) / 2.0,
                    (c + a) / 2.0,
                ]
            })
            .map(|point| (off(point), point))
            .max_by(|(first, _), (second, _)| first.total_cmp(second));
        if let Some((off, point)) = worst
            && off > SAG * tolerance
        {
            wrong.push(format!(
                "{what}: its face {face}, drawn on {surface}, stands {off:.5} off it at \
                 {point:.4}, {:.2} of the drawing's tolerance {tolerance}",
                off / tolerance
            ));
        }
    }
    let triangles = body.triangles();
    let area: f64 = triangles
        .iter()
        .map(|&[a, b, c]| (b - a).cross(c - a).length() / 2.0)
        .sum();
    let drawn = enclosed(&triangles);
    if (drawn - body.volume()).abs() > SPREAD * tolerance * area {
        wrong.push(format!(
            "{what}: its triangles enclose {drawn:.6} where it holds {:.6}, {:.3} of the \
             drawing's tolerance {tolerance} over their area {area:.2}",
            body.volume(),
            (drawn - body.volume()).abs() / (tolerance * area)
        ));
    }
    wrong
}

/// The cuts of the criterion, each drawn with every triangle of a curved
/// face within [`SAG`] of the drawing's tolerance of its surface, and
/// enclosing what the body holds within [`SPREAD`] of that tolerance over
/// the triangles' area — never the ring between the shaft's wall and the
/// sphere drawn coarser to stand its triangles apart. Every cut that is not
/// is named, with each way it is not.
#[test]
fn a_ball_turned_about_z_or_at_a_slant_less_a_shaft_forty_tolerances_under_its_radius_ending_inside_the_band_is_drawn_within_the_drawing_s_tolerance_of_its_surfaces()
 {
    let mut wrong = Vec::new();
    for (about, axis) in balls() {
        let ball = ball_about(axis);
        for (radius, from, to) in shafts() {
            let what = named(about, radius, from, to);
            match ball.difference(&shaft_along_z(&ball, radius, from, to)) {
                Err(declined) => wrong.push(format!("{what}: declined, {declined:?}")),
                Ok(body) => wrong.extend(drawn_off(&body, &what)),
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a ball less a shaft a few tens of tolerances under its radius, ending inside the band, \
         is drawn within the drawing's tolerance of its surfaces whatever axis the ball was \
         turned about; not so:\n{}",
        wrong.join("\n")
    );
}
