//! A shaft along z through a ball's centre, a hair under its radius, meets
//! the sphere along two circles a few ten-thousandths either side of its
//! equator about z, and decision 12 lays its wall onto the touch. Where the
//! shaft's end plane stands inside the band between those circles, the
//! boolean is built as it is beside a ball turned about z, the shaft's own
//! axis, whatever axis the ball was turned about — short of one that lays
//! those circles near the poles of its chart, which is #962's.
//!
//! Closes #1109.
//! - A ball of radius 5 turned at a slant, about (1, 0, 1)/√2, and a shaft
//!   along z through its centre of radius 5 − 2, 5 and 15 tolerances whose
//!   end planes stand at z = ±(a − 1e-5): joined and cut, both orders,
//!   built, sound, the volume the coaxial case gives. (The ball turned about
//!   x is #962's: there the shaft's circles z = ±a run within `FRONT` of the
//!   x chart's poles, and #956 declines them on purpose.) —
//!   `a_ball_turned_at_a_slant_joined_with_or_cut_by_a_shaft_along_z_through_its_centre_a_hair_under_its_radius_ending_inside_the_band_is_built_sound_and_holds_what_the_coaxial_case_holds`.
//!   Read as: a tolerance the kernel's at the reach the ball and the shaft
//!   are read at, as #956's shafts take it — 10 for a shaft standing out to
//!   z = ±10, the ball's 5 for one standing between the two planes; a where
//!   the shaft's wall meets the sphere, √(r² − b²); the shaft standing from
//!   the plane z = −(a − 1e-5) up past the ball, from past the ball up to
//!   the plane z = a − 1e-5, or between the two, so that either end, or
//!   both, stands inside the band; joined either way, the ball cut by the
//!   shaft and the shaft by the ball; each step built, never declined, its
//!   body exact, its triangles closed and uncrossed, as #956 reads "built,
//!   sound" for its shafts, and holding within a billionth of the matter in
//!   play what the same step leaves beside the ball turned about z — that
//!   one built and sound too, and holding what the arithmetic leaves with
//!   the shaft as drawn or laid onto the touch
//! - #956's tests keep their bodies — no test: they are the tests of
//!   `spheres_of_one_centre_and_radius_are_one_surface_whatever_their_axes.rs`,
//!   and those #956 added to `src/brep/relation/tests/round.rs`,
//!   `src/brep/selection/tests/round.rs`,
//!   `src/brep/tessellation/sampling/tests/rows.rs` and
//!   `crates/part/tests/the_exact_kernel_turns_the_part.rs`; running them
//!   unchanged is the answer
//!
//! The ball turned about x is left to #962. Beside it every one of these
//! shafts is declined `Unsupported`, in every order, the shaft first too:
//! the circles z = ±a run within `FRONT` — 0.05 of the radius, about 2.9°,
//! in `tessellation/outline/central.rs` — of the poles (±5, 0, 0) of its
//! chart, and
//! `a_ball_turned_about_x_joined_with_or_cut_by_a_shaft_along_z_through_its_centre_is_declined_where_the_rims_pass_near_its_chart_s_poles_and_built_further_off`,
//! in #956's file, holds that decline for the shafts from z = −(a − 1e-5).
//! Building them here would change that test's body.

use std::f64::consts::{PI, TAU};

use cao_solid::brep::Scale;
use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Declined, Loop};
use glam::{DVec2, DVec3};

/// How far the matter a body holds may stand from another's, as a share of
/// the matter in play: the kernel's billionth.
const EXACT: f64 = 1e-9;

/// The radius of every ball here, about the origin.
const RADIUS: f64 = 5.0;

/// The ball of [`RADIUS`].
const BALL: f64 = 4.0 * PI / 3.0 * RADIUS * RADIUS * RADIUS;

/// How far inside the band the shaft's end plane stands, from the circle the
/// shaft's wall meets the sphere along.
const INSIDE: f64 = 1e-5;

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
/// the ball's radius less 2, 5 and 15 of the tolerance it and the ball are
/// read at, its wall meeting the sphere along the circles z = ±a, a =
/// √(r² − b²), a few ten-thousandths either side of the equator about z;
/// standing from the plane [`INSIDE`] above z = −a, within the band between
/// those circles, up to z = 10, past the ball; from z = −10 up to the plane
/// [`INSIDE`] under z = a; or between the two planes. The first two are read
/// at the reach of 10 they stand to, the third at the ball's.
fn shafts() -> Vec<(f64, f64, f64)> {
    let under = |reach: f64| [2.0, 5.0, 15.0].map(|hair| RADIUS - hair * Scale::of(reach).eps());
    let inside = |radius: f64| (RADIUS * RADIUS - radius * radius).sqrt() - INSIDE;
    under(10.0)
        .into_iter()
        .flat_map(|radius| {
            [
                (radius, -inside(radius), 10.0),
                (radius, -10.0, inside(radius)),
            ]
        })
        .chain(under(RADIUS).map(|radius| (radius, -inside(radius), inside(radius))))
        .collect()
}

/// What a shaft of `radius` along z through the ball's centre, from z =
/// `from` to `to`, holds, and what it takes of the ball: π b² (`to` −
/// `from`), and the ball's slices between, each the disc of the shaft or the
/// ball's own where that is the smaller — the shaft's within a of the
/// equator, a = √(r² − b²), the ball's π (r² − z²) beyond. #956's, with an
/// end of the shaft's own on either side.
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

/// What the ball and a shaft of `radius` through its centre from z = `from`
/// to `to` leave, in the order of [`booleans`]: joined either way, the ball
/// cut by the shaft and the shaft by the ball.
fn ball_and_shaft(radius: f64, from: f64, to: f64) -> [f64; 4] {
    let [shaft, shared] = shaft_and_its_share_of_the_ball(radius, from, to);
    [
        BALL + shaft - shared,
        BALL + shaft - shared,
        BALL - shared,
        shaft - shared,
    ]
}

/// The four booleans of `ball` and `shaft`: joined either way, and each cut
/// by the other. #956's.
fn booleans(ball: &Body, shaft: &Body) -> [(&'static str, Result<Body, Declined>); 4] {
    [
        ("joined with", ball.union(shaft)),
        ("joined to, the shaft first,", shaft.union(ball)),
        ("cut by", ball.difference(shaft)),
        ("cut from", shaft.difference(ball)),
    ]
}

/// Why `body` is not sound, if it is not: not exact, or its triangles open
/// or crossing.
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

/// The ball turned at a slant, about (1, 0, 1)/√2, and each of [`shafts`]:
/// the circles the shaft's wall meets the sphere along stand 45° or more
/// from its chart's poles. Joined either way, the ball cut by the shaft and
/// the shaft by the ball, against the same step beside the ball turned about
/// z, the shaft's own axis: that one built, exact, its triangles closed and
/// uncrossed, holding what the arithmetic leaves with the shaft as drawn or
/// laid onto the touch; and this one built, exact, its triangles closed and
/// uncrossed, holding what that one holds within a billionth of the ball and
/// the shaft. Every step that is not is named.
#[test]
fn a_ball_turned_at_a_slant_joined_with_or_cut_by_a_shaft_along_z_through_its_centre_a_hair_under_its_radius_ending_inside_the_band_is_built_sound_and_holds_what_the_coaxial_case_holds()
 {
    let coaxial = ball_about(DVec3::Z);
    let ball = ball_about(DVec3::new(1.0, 0.0, 1.0).normalize());
    let mut wrong = Vec::new();
    for (radius, from, to) in shafts() {
        let drawn = ball_and_shaft(radius, from, to);
        let laid = ball_and_shaft(RADIUS, from, to);
        let [shaft, _] = shaft_and_its_share_of_the_ball(radius, from, to);
        let within = EXACT * (BALL + shaft);
        let references = booleans(&coaxial, &shaft_along_z(&coaxial, radius, from, to));
        let steps = booleans(&ball, &shaft_along_z(&ball, radius, from, to));
        for (index, ((way, reference), (_, built))) in references.into_iter().zip(steps).enumerate()
        {
            let what = format!(
                "the ball turned at a slant {way} a shaft along z through its centre of radius \
                 {radius} from z = {from} to {to}"
            );
            let reference = reference.unwrap_or_else(|declined| {
                panic!("{what}: the same beside the ball turned about z declined, {declined:?}")
            });
            if let Some(why) = unsound(&reference) {
                panic!("{what}: the same beside the ball turned about z is {why}");
            }
            assert!(
                [drawn[index], laid[index]]
                    .iter()
                    .any(|volume| (reference.volume() - volume).abs() <= within),
                "{what}: the same beside the ball turned about z holds {}, where the arithmetic \
                 leaves {} as drawn or {} laid onto the touch",
                reference.volume(),
                drawn[index],
                laid[index]
            );
            match built {
                Err(declined) => wrong.push(format!("{what}: declined, {declined:?}")),
                Ok(body) => {
                    if let Some(why) = unsound(&body) {
                        wrong.push(format!("{what}: {why}"));
                    } else if (body.volume() - reference.volume()).abs() > within {
                        wrong.push(format!(
                            "{what}: holds {}, where beside the ball turned about z it holds {}",
                            body.volume(),
                            reference.volume()
                        ));
                    }
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a shaft a hair under the radius of the ball turned at a slant, ending inside the band, \
         is built sound and holds what it holds beside the ball turned about z; not so:\n{}",
        wrong.join("\n")
    );
}
