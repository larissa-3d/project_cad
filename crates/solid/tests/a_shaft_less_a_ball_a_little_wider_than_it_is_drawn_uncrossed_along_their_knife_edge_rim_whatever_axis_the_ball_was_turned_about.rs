//! A shaft along z through a ball's centre, a little narrower than the ball,
//! meets its sphere along two circles either side of its equator about z.
//! Cut by the ball, the shaft keeps what stands outside the sphere, and that
//! matter thins to nothing along those circles: a knife-edge rim, where the
//! sphere and the shaft's wall meet at a shallow angle, cos⁻¹ 0.98 for a
//! shaft of 4.9 in a ball of 5. The boolean is drawn uncrossed along that rim
//! whatever axis the ball was turned about.
//!
//! Closes #1138.
//! - The shaft of 4.9 from −10 to 0 (and to 2) less the ball of 5 turned
//!   about z and at the three slants: built, exact, closed, uncrossed, the
//!   arithmetic's volume —
//!   `a_shaft_less_a_ball_a_little_wider_than_it_turned_about_z_or_at_a_slant_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume`.
//!   Read as: the shaft a round of radius 4.9 about the z axis, raised from
//!   the plane z = −10 to the plane z = 0, or z = 2; the ball of radius 5
//!   about the origin, a half disc turned whole about the line through the
//!   origin along z, (1, 0, 1)/√2, (−1, 0, 1)/√2 or (0, 1, 1)/√2, its
//!   sphere's chart laid about that line; less, the shaft cut by the ball;
//!   built, never declined; exact, the exact kernel computed the step; closed
//!   and uncrossed, its triangles so by `soundness`; the arithmetic's volume,
//!   within a billionth of the matter in play, the ball and the shaft, of
//!   the shaft's π 4.9² by its length less the slices of it inside the ball —
//!   the shaft's whole disc within a = √(5² − 4.9²), about 0.995, of the
//!   equator, the ball's π (5² − z²) beyond.
//! - #956's and #1109's tests keep their bodies — no test: they are the
//!   tests of `spheres_of_one_centre_and_radius_are_one_surface_whatever_their_axes.rs`
//!   and of
//!   `a_shaft_a_hair_under_a_ball_s_radius_its_end_inside_the_band_is_laid_onto_the_touch_whatever_axis_the_ball_was_turned_about.rs`,
//!   and those #956 added to `src/brep/relation/tests/round.rs`,
//!   `src/brep/selection/tests/round.rs`,
//!   `src/brep/tessellation/sampling/tests/rows.rs` and
//!   `crates/part/tests/the_exact_kernel_turns_the_part.rs`; running them
//!   unchanged is the answer

use std::f64::consts::{PI, TAU};

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Loop};
use glam::{DVec2, DVec3};

/// How far the matter a body holds may stand from the arithmetic's, as a
/// share of the matter in play: the kernel's billionth.
const EXACT: f64 = 1e-9;

/// The radius of the ball, about the origin.
const RADIUS: f64 = 5.0;

/// The ball of [`RADIUS`].
const BALL: f64 = 4.0 * PI / 3.0 * RADIUS * RADIUS * RADIUS;

/// The radius of the shaft, a little under the ball's.
const SHAFT: f64 = 4.9;

/// Where the shaft starts, under the ball.
const FROM: f64 = -10.0;

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

/// A shaft of [`SHAFT`] along z through the origin, from z = [`FROM`] to
/// `to`, a tool of `part`.
fn shaft_along_z(part: &Body, to: f64) -> Body {
    part.tool_raised(
        &exact(Contour {
            corners: vec![DVec2::X * SHAFT],
            runs: vec![Run::Round {
                center: DVec2::ZERO,
                turn: TAU,
            }],
        }),
        Frame {
            origin: DVec3::Z * FROM,
            u: DVec3::X,
            v: DVec3::Y,
        },
        DVec3::Z * (to - FROM),
    )
    .expect("the shaft raises")
}

/// What the shaft from z = [`FROM`] to `to` holds, and what the ball takes
/// of it: π b² (`to` − [`FROM`]), and its slices inside the ball, each the
/// shaft's disc or the ball's where that is the smaller — the shaft's
/// within a of the equator, a = √(r² − b²), the ball's π (r² − z²) beyond.
/// #1109's.
fn shaft_and_its_share_of_the_ball(to: f64) -> [f64; 2] {
    let reach = (RADIUS * RADIUS - SHAFT * SHAFT).sqrt();
    let round = |low: f64, high: f64| {
        PI * (RADIUS * RADIUS * (high - low) - (high.powi(3) - low.powi(3)) / 3.0)
    };
    let below = |height: f64| {
        let height = height.clamp(-RADIUS, RADIUS);
        round(-RADIUS, height.min(-reach))
            + PI * SHAFT * SHAFT * (height.clamp(-reach, reach) + reach)
            + round(reach, height.max(reach))
    };
    [PI * SHAFT * SHAFT * (to - FROM), below(to) - below(FROM)]
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

/// The shaft from z = −10 to the ball's centre, or past it to z = 2, cut by
/// the ball turned about z, the shaft's own axis, and about each of the
/// three slants: where the shaft's wall meets the sphere, the circle z =
/// −a, and z = a for the shaft to 2, is a parallel of the sphere's chart
/// beside the first and runs across its parallels beside the others. Each
/// built, exact, its triangles closed and uncrossed, holding what the
/// arithmetic leaves within a billionth of the ball and the shaft. Every
/// step that is not is named, with each way it is not.
#[test]
fn a_shaft_less_a_ball_a_little_wider_than_it_turned_about_z_or_at_a_slant_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let axes = [
        ("about z", DVec3::Z),
        ("about (1, 0, 1)/√2", DVec3::new(1.0, 0.0, 1.0).normalize()),
        (
            "about (−1, 0, 1)/√2",
            DVec3::new(-1.0, 0.0, 1.0).normalize(),
        ),
        ("about (0, 1, 1)/√2", DVec3::new(0.0, 1.0, 1.0).normalize()),
    ];
    let mut wrong = Vec::new();
    for (about, axis) in axes {
        let ball = ball_about(axis);
        for to in [0.0, 2.0] {
            let what = format!(
                "a shaft of {SHAFT} along z from z = {FROM} to {to} less the ball of {RADIUS} \
                 turned {about}"
            );
            let [shaft, shared] = shaft_and_its_share_of_the_ball(to);
            let left = shaft - shared;
            let within = EXACT * (BALL + shaft);
            match shaft_along_z(&ball, to).difference(&ball) {
                Err(declined) => wrong.push(format!("{what}: declined, {declined:?}")),
                Ok(body) => {
                    if let Some(why) = unsound(&body) {
                        wrong.push(format!("{what}: {why}"));
                    }
                    if (body.volume() - left).abs() > within {
                        wrong.push(format!(
                            "{what}: holds {}, where the arithmetic leaves {left}",
                            body.volume()
                        ));
                    }
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a shaft less a ball a little wider than it is built exact, closed, uncrossed and of \
         the arithmetic's volume whatever axis the ball was turned about; not so:\n{}",
        wrong.join("\n")
    );
}
