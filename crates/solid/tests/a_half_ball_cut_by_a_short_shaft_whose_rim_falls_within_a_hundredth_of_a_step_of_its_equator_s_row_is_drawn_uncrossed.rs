//! A half ball cut by a short shaft a hair under its radius, up to its
//! centre, is drawn uncrossed, and quickly, where the rim the sphere crosses
//! the shaft's wall along falls within a hundredth of a step of the row of
//! the sphere's grid on its equator. #956's `SLIVER` rule leaves out of a
//! face every row within that hundredth of a step of one of the face's
//! levels, the rim's latitude among them: the equator's row is left out,
//! and the strip from the rim runs over the equator up to the next row,
//! through the shaft's top disc at z = 0 (#1127's review). Beside the shaft
//! from z = −10 that #1123 holds, decision 12 lays every such shaft onto the
//! touch; from z = −5 it lays them only half as far, and leaves the band.
//!
//! Closes #1127.
//! - The half ball of 5 about z cut by the shaft from z = −5 to 0 of radius
//!   5 − h, for h = 1.2e-7 and 1.5e-7 (and the band where the rim falls
//!   within `SLIVER` of the equator's row past decision 12's reach): built,
//!   exact, closed, uncrossed, the arithmetic's volume —
//!   `a_half_ball_cut_up_to_its_centre_by_a_shaft_from_five_under_it_whose_rim_falls_within_a_hundredth_of_a_step_of_its_equator_s_row_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume`.
//!   Read as: the half ball #956's and #1123's, the half disc of 5 about the
//!   origin turned half a turn about z, y ≥ 0, its sphere's chart laid about
//!   z; the shaft a round of 5 − h about z raised from z = −5 up to the
//!   ball's centre, a tool of the half ball; cut, the half ball less the
//!   shaft; built, never declined; exact, the exact kernel computed it;
//!   closed and uncrossed, its triangles so by `soundness`. Decision 12 lays
//!   the shaft onto the touch within [`Scale::HAIR`] of the kernel's
//!   tolerances at the pair's reach, the ball and the shaft both reaching 5:
//!   20 × 5e-9, 1e-7. The cut is drawn at 2e-4 of its reach, a thousandth
//!   (`body/drawn.rs`'s `DRAWN`), and its sphere's grid along the meridian
//!   is cut for 0.45 of that (`SHARE`): [`ROWS`] rows a turn, a step of
//!   0.026624 rad, the row above the equator at z = 0.1331, the review's
//!   0.133. The rim stands at the latitude acos(1 − h/5) under the equator,
//!   within [`SLIVER`] of a step of it, 2.6624e-4 rad, for every h up to
//!   5 (1 − cos(0.01 × 2π/236)) = 1.772e-7 ([`band_s_top`]). The band is h
//!   past 1e-7 ([`decision_twelve_s_reach`]) up to 1.772e-7, and [`HAIRS`]
//!   samples it end to end: 1e-7, whose shaft of 5 − 1e-7 stands
//!   1.0000000028e-7 under in `f64`, past the reach by the rounding —
//!   measured on `main` (0ce1e756) not laid, as many faces as at 1.2e-7,
//!   three flat and two curved, and crossed, as the triage found it; the
//!   issue's 1.2e-7 and 1.5e-7, with 1.1e-7, 1.35e-7
//!   and 1.65e-7 between; 1.75e-7, the triage's last crossed hair; and
//!   1.77e-7, just under the band's top, its rim at 0.9994 of the hundredth.
//!   The triage found every hair from 1.0e-7 to 1.75e-7 crossed on `main`
//!   and every one from 1.8e-7 to 2.0e-7 sound, the band's top between;
//!   measured on 0ce1e756, every one of [`HAIRS`] is built, exact, not laid
//!   and of the volume at the shaft's radius, closed, and crossed at the
//!   rim, a column or two from the half ball's flat face. The
//!   arithmetic's volume is the quarter of the ball above its centre with
//!   half the ring the shaft leaves under it, at the shaft's radius
//!   ([`kept`]), or the quarter alone with the shaft laid onto the touch: the
//!   two differ by π a³ / 3, a = √(2rh) under 1.34e-3, some 2e-11 of the
//!   quarter, so the kernel's billionth cannot tell them apart, and either is
//!   taken, as #1109's and #1123's tests take it
//! - #956's and #1123's tests keep their bodies — no test: they are the
//!   tests of `spheres_of_one_centre_and_radius_are_one_surface_whatever_their_axes.rs`
//!   and of `a_half_ball_cut_by_a_shaft_a_hair_under_its_radius_is_drawn_uncrossed.rs`,
//!   those #956 added to `src/brep/relation/tests/round.rs`,
//!   `src/brep/selection/tests/round.rs`,
//!   `src/brep/tessellation/sampling/tests/rows.rs` and
//!   `crates/part/tests/the_exact_kernel_turns_the_part.rs`, and those #1123
//!   added to `src/brep/tessellation/sampling/axis/tests.rs` and
//!   `src/brep/tessellation/sampling/cone/lined_up/tests.rs`; this file is a
//!   new one beside them, and running them unchanged is the answer
//!
//! Beyond the criteria: each of the band's cuts in under a second. Every
//! crossed cut of the band took 18 to 21 s on `main` in the triage, and 17.6
//! to 17.7 s each measured on 0ce1e756, against some 0.05 s for the sound
//! ones past the band's top — 42 to 44 ms at 1.8e-7 and 2e-7 — and 37 ms for
//! the same hairs beside the shaft from z = −10; a cut made uncrossed but
//! still a quarter of a minute long would not be drawn the way the view
//! needs it. Each cut is timed alone on the wall clock, once, its first read of
//! its triangles with it, and held under [`QUICK`] in the profile the tests
//! run in, which builds `cao_solid` at `opt-level` 2 —
//! `a_half_ball_cut_up_to_its_centre_by_a_shaft_from_five_under_it_whose_rim_falls_within_a_hundredth_of_a_step_of_its_equator_s_row_is_cut_and_drawn_in_under_a_second`.
//!
//! And the hairs beside the band, drawn sound on `main`, which a change to
//! the rows a face leaves out must not cross: 5e-8, within decision 12's
//! reach, the shaft laid onto the touch; 1.8e-7 and 2e-7, just past the
//! band's top; and 1e-6, further out —
//! `a_half_ball_cut_up_to_its_centre_by_a_shaft_from_five_under_it_whose_rim_falls_just_beside_that_hundredth_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume`.

use std::f64::consts::{PI, TAU};
use std::time::{Duration, Instant};

use cao_solid::brep::Scale;
use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Declined, Loop};
use glam::{DVec2, DVec3};

/// How far the matter a body holds may stand from what the arithmetic
/// leaves, as a share of it: the kernel's billionth.
const EXACT: f64 = 1e-9;

/// The radius of the ball, about the origin.
const RADIUS: f64 = 5.0;

/// The ball of [`RADIUS`].
const BALL: f64 = 4.0 * PI / 3.0 * RADIUS * RADIUS * RADIUS;

/// Where the shaft starts along z: the ball's south pole, so that the pair
/// reaches no further than the ball does.
const FROM: f64 = -5.0;

/// How many rows a turn the grid of the ball's sphere is cut into along its
/// meridian at the drawing's tolerance: `divisions_in(5, 0.45 × 1e-3)`, 235
/// steps rounded up to a multiple of four, which no public function gives.
const ROWS: f64 = 236.0;

/// How near a level of a face, as a share of a step of its grid, a row the
/// face is cut at is left out: `tessellation/sampling/round.rs`'s `SLIVER`,
/// which no public constant gives.
const SLIVER: f64 = 0.01;

/// How far under the ball's radius each shaft stands: the band from just
/// past decision 12's reach up to just under [`band_s_top`], the issue's two
/// hairs among them.
const HAIRS: [f64; 8] = [
    1e-7, 1.1e-7, 1.2e-7, 1.35e-7, 1.5e-7, 1.65e-7, 1.75e-7, 1.77e-7,
];

/// The hairs beside the band: one within decision 12's reach, two just past
/// the band's top, one further out.
const BESIDE: [f64; 4] = [5e-8, 1.8e-7, 2e-7, 1e-6];

/// How long one cut may take on the wall clock, its drawing with it.
const QUICK: Duration = Duration::from_secs(1);

/// The half disc of [`RADIUS`] about the origin standing on the side of x,
/// turned half a turn about z from x towards y: the half ball y ≥ 0, its
/// sphere's chart laid about z. #956's.
fn half_ball() -> Body {
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
        u: DVec3::X,
        v: DVec3::Z,
    };
    let along_v = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    Body::default()
        .tool_turned(&profile, frame, &Turn::of(along_v, PI, 0.0, &profile))
        .expect("the exact kernel turns the half disc")
}

/// A shaft of `radius` along z from z = [`FROM`] up to the origin, a tool of
/// `part`.
fn shaft_up_to_the_centre(part: &Body, radius: f64) -> Body {
    part.tool_raised(
        &Profile {
            exact: Some((
                Contour {
                    corners: vec![DVec2::X * radius],
                    runs: vec![Run::Round {
                        center: DVec2::ZERO,
                        turn: TAU,
                    }],
                },
                Vec::new(),
            )),
            sampled: Loop::straight(&[]),
            sampled_holes: Vec::new(),
            triangles: &[],
        },
        Frame {
            origin: DVec3::Z * FROM,
            u: DVec3::X,
            v: DVec3::Y,
        },
        DVec3::Z * -FROM,
    )
    .expect("the shaft raises")
}

/// The furthest under the ball's radius decision 12 lays the shaft onto the
/// touch: [`Scale::HAIR`] of the kernel's tolerances at the pair's reach,
/// the ball's.
fn decision_twelve_s_reach() -> f64 {
    Scale::HAIR * Scale::of(RADIUS).eps()
}

/// The largest hair whose rim falls within [`SLIVER`] of a step of the
/// equator's row: the rim's latitude, acos(1 − h/r), at a hundredth of
/// 2π / [`ROWS`].
fn band_s_top() -> f64 {
    RADIUS * (1.0 - (SLIVER * TAU / ROWS).cos())
}

/// What the half ball cut by the shaft `hair` under its radius leaves: the
/// quarter of the ball above its centre, and half the ring the shaft leaves
/// of the ball under it, between z = −a and 0 outside the wall,
/// a = √(r² − (r − h)²): π ∫ (a² − z²) dz over it is 2π a³ / 3, the half
/// π a³ / 3. #1123's.
fn kept(hair: f64) -> f64 {
    let reach = (hair * (2.0 * RADIUS - hair)).sqrt();
    BALL / 4.0 + PI * reach.powi(3) / 3.0
}

/// What is wrong with the half ball cut by the shaft `hair` under its
/// radius, if anything: declined, not exact, holding neither what the
/// arithmetic leaves at the shaft's radius nor the quarter alone, its
/// triangles open or crossing.
fn wrong_with(built: Result<Body, Declined>, hair: f64) -> Option<String> {
    let what = format!("the half ball cut up to its centre by a shaft {hair:e} under its radius");
    let body = match built {
        Err(declined) => return Some(format!("{what}: declined, {declined:?}")),
        Ok(body) => body,
    };
    if !body.is_exact() {
        return Some(format!("{what}: not exact"));
    }
    let [drawn, laid] = [kept(hair), BALL / 4.0];
    if [drawn, laid]
        .iter()
        .all(|volume| (body.volume() - volume).abs() > EXACT * volume)
    {
        return Some(format!(
            "{what}: holds {}, where the arithmetic leaves {drawn} at the shaft's radius or \
             {laid} laid onto the touch",
            body.volume()
        ));
    }
    let triangles = body.triangles();
    if let Err(open) = closed(&triangles) {
        return Some(format!("{what}: its triangles open, {open:?}"));
    }
    if let Err(crossing) = uncrossed(&triangles) {
        return Some(format!("{what}: its triangles crossing, {crossing:?}"));
    }
    None
}

/// The half ball y ≥ 0 cut by a shaft along z from z = −5 up to its centre,
/// each of [`HAIRS`] under its radius: past decision 12's reach, so the
/// sphere crosses the wall along a rim a thousandth or so under the equator,
/// within a hundredth of a step of the equator's row. Each built, exact,
/// holding what the arithmetic leaves, its triangles closed and uncrossed;
/// every hair that is not is named, with the first way it is not.
#[test]
fn a_half_ball_cut_up_to_its_centre_by_a_shaft_from_five_under_it_whose_rim_falls_within_a_hundredth_of_a_step_of_its_equator_s_row_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let half = half_ball();
    let wrong: Vec<String> = HAIRS
        .iter()
        .filter_map(|&hair| {
            let shaft = shaft_up_to_the_centre(&half, RADIUS - hair);
            wrong_with(half.difference(&shaft), hair)
        })
        .collect();
    assert!(
        wrong.is_empty(),
        "a half ball cut by a shaft from z = -5 whose rim falls within a hundredth of a step of \
         its equator's row, past {:e} and up to {:e} under its radius, is built exact, closed, \
         uncrossed and of the arithmetic's volume; not so:\n{}",
        decision_twelve_s_reach(),
        band_s_top(),
        wrong.join("\n")
    );
}

/// The same cuts, each timed from the cut to its first read of its
/// triangles: built, drawn not empty, within [`QUICK`]. Every hair that is
/// not is named.
#[test]
fn a_half_ball_cut_up_to_its_centre_by_a_shaft_from_five_under_it_whose_rim_falls_within_a_hundredth_of_a_step_of_its_equator_s_row_is_cut_and_drawn_in_under_a_second()
 {
    let half = half_ball();
    let mut wrong = Vec::new();
    for hair in HAIRS {
        let what =
            format!("the half ball cut up to its centre by a shaft {hair:e} under its radius");
        let shaft = shaft_up_to_the_centre(&half, RADIUS - hair);
        let started = Instant::now();
        let drawn = half.difference(&shaft).map(|body| body.triangles());
        let took = started.elapsed();
        match drawn {
            Err(declined) => wrong.push(format!("{what}: declined, {declined:?}")),
            Ok(triangles) if triangles.is_empty() => wrong.push(format!("{what}: drawn empty")),
            Ok(_) if took > QUICK => {
                wrong.push(format!("{what}: cut and drawn in {took:?}, past {QUICK:?}"))
            }
            Ok(_) => {}
        }
    }
    assert!(
        wrong.is_empty(),
        "a half ball cut by a shaft from z = -5 whose rim falls within a hundredth of a step of \
         its equator's row is cut and drawn in under a second; not so:\n{}",
        wrong.join("\n")
    );
}

/// The half ball cut by the shaft from z = −5 up to its centre each of
/// [`BESIDE`] under its radius, outside the band: each built, exact, holding
/// what the arithmetic leaves, its triangles closed and uncrossed, as on
/// `main`.
#[test]
fn a_half_ball_cut_up_to_its_centre_by_a_shaft_from_five_under_it_whose_rim_falls_just_beside_that_hundredth_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let half = half_ball();
    let wrong: Vec<String> = BESIDE
        .iter()
        .filter_map(|&hair| {
            let shaft = shaft_up_to_the_centre(&half, RADIUS - hair);
            wrong_with(half.difference(&shaft), hair)
        })
        .collect();
    assert!(
        wrong.is_empty(),
        "a half ball cut by a shaft from z = -5 whose rim falls beside the band is built exact, \
         closed, uncrossed and of the arithmetic's volume; not so:\n{}",
        wrong.join("\n")
    );
}
