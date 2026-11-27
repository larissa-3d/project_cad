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
//!
//! And the same rim beside the same row elsewhere, which the fix's rule
//! cures as well: a wall running from a rim within a hundredth of a step of
//! a row, the two faces' outward normals opposed there, and ending on that
//! row. The review of the fix (b8fbd6f3) measured each crossed on `main` and
//! sound on the fix; each is held as the criterion holds its cuts, built,
//! exact, closed, uncrossed, of the volume at the shaft's radius or laid
//! onto the touch, and each built and first drawn in under [`QUICK`]. The
//! band is reckoned as the criterion's: past decision 12's reach at the
//! pair's reach, and up to where the rim's latitude stands a hundredth of a
//! step of the round's rows at the drawing's tolerance from the row. Hairs
//! of the ball of 5 at its reach stand in the criterion's band.
//!
//! The ball of 5 about z, whole, less the shaft from z = −5 up to its centre,
//! 1.2e-7 and 1.5e-7 under its radius, crossed in some 60 s a cut on `main`
//! and sound in 80 to 88 ms on the fix —
//! `a_ball_about_z_cut_up_to_its_centre_by_a_shaft_from_its_pole_whose_rim_falls_within_a_hundredth_of_a_step_of_its_equator_s_row_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume_each_in_under_a_second`.
//!
//! The half ball less the shaft from its centre up to z = 5, the criterion's
//! cut mirrored across the equator, its rim a hair over the row: 21.5 s and
//! crossed at 1.2e-7 on `main`, sound in 43 ms on the fix —
//! `a_half_ball_cut_down_to_its_centre_by_a_shaft_from_its_pole_whose_rim_falls_within_a_hundredth_of_a_step_of_its_equator_s_row_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume_each_in_under_a_second`.
//!
//! The whole ball less the half y ≥ 0 of the shaft, the wall's rim an arc:
//! 53 s and crossed on `main`, sound in 123 to 135 ms on the fix —
//! `a_ball_cut_up_to_its_centre_by_a_half_shaft_whose_rim_falls_within_a_hundredth_of_a_step_of_its_equator_s_row_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume_each_in_under_a_second`.
//!
//! #1123's ring, of tube 2 about a ring radius of 10, less a coaxial shaft
//! from z = −5 up to its centre plane, of 12 − h for h = 3e-7, 3.5e-7 and
//! 4e-7: past decision 12's reach at the pair's reach of 12, 2.4e-7, and
//! within a hundredth of a step of the outer equator's row, the tube's grid
//! of 96 rows a turn at the drawing's tolerance of 2.4e-3, up to
//! 2 (1 − cos(0.01 × 2π/96)) = 4.28e-7. Some 102 s and crossed a cut on
//! `main`, sound in about 70 ms on the fix; 2e-7, laid, and 5e-7, past the
//! band, are sound on both —
//! `a_ring_cut_up_to_its_centre_plane_by_a_coaxial_shaft_whose_rim_falls_within_a_hundredth_of_a_step_of_its_outer_equator_s_row_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume_each_in_under_a_second`.
//!
//! A shell, the ball of 8 less the ball of 5 about the origin, joined with
//! the shaft from z = −5 up to its centre, of 5 − h for h = 1.7e-7, 2e-7 and
//! 2.5e-7: past decision 12's reach at the pair's reach of 8, 1.6e-7, and
//! within a hundredth of a step of the cavity's equator's row, its grid of
//! 188 rows a turn at the drawing's tolerance of 1.6e-3, up to
//! 5 (1 − cos(0.01 × 2π/188)) = 2.79e-7. What lies between the wall and the
//! cavity's sphere is void here, not matter, and the rule cures it all the
//! same: some 60 s and crossed a cut on `main`, sound in 165 to 175 ms on
//! the fix —
//! `a_shell_joined_with_a_shaft_up_to_its_centre_whose_rim_falls_within_a_hundredth_of_a_step_of_its_cavity_s_equator_s_row_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume_each_in_under_a_second`.
//!
//! Which walls the rule reads, and which rows it keeps, are held beside it:
//! `src/brep/tessellation/sampling/tests/rows.rs` and
//! `src/brep/tessellation/outline/knife/tests.rs`.

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

/// The half disc of `radius` about the origin standing on the side of x,
/// turned `turn` about z from x towards y: a ball, or for half a turn the
/// half ball y ≥ 0, its sphere's chart laid about z. #956's.
fn ball_of(radius: f64, turn: f64) -> Body {
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
    turned(half, sampled, turn)
}

/// The half ball of [`RADIUS`], y ≥ 0.
fn half_ball() -> Body {
    ball_of(RADIUS, PI)
}

/// The disc of `minor` about (`major`, 0) in the plane y = 0, turned whole
/// about z: a ring about the origin, its outer equator of radius
/// `major + minor` on the plane z = 0. #1123's.
fn ring_of(major: f64, minor: f64) -> Body {
    let sampled: &'static [DVec2] = Box::leak(
        (0..64)
            .map(|step| {
                DVec2::new(major, 0.0) + DVec2::from_angle(TAU * f64::from(step) / 64.0) * minor
            })
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let disc = Contour {
        corners: vec![DVec2::new(major + minor, 0.0)],
        runs: vec![Run::Round {
            center: DVec2::new(major, 0.0),
            turn: TAU,
        }],
    };
    turned(disc, sampled, TAU)
}

/// `outline`, drawn in the plane y = 0 with x across and z up, turned
/// `turn` about z from x towards y.
fn turned(outline: Contour, sampled: &'static [DVec2], turn: f64) -> Body {
    let profile = Profile {
        exact: Some((outline, Vec::new())),
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
        .tool_turned(&profile, frame, &Turn::of(along_v, turn, 0.0, &profile))
        .expect("the exact kernel turns the profile")
}

/// `outline`, drawn in the plane z = `from` with x and y across, raised along
/// z to z = `to`, a tool of `part`.
fn raised(part: &Body, outline: Contour, from: f64, to: f64) -> Body {
    part.tool_raised(
        &Profile {
            exact: Some((outline, Vec::new())),
            sampled: Loop::straight(&[]),
            sampled_holes: Vec::new(),
            triangles: &[],
        },
        Frame {
            origin: DVec3::Z * from,
            u: DVec3::X,
            v: DVec3::Y,
        },
        DVec3::Z * (to - from),
    )
    .expect("the shaft raises")
}

/// A shaft of `radius` along z from z = `from` to z = `to`, a tool of
/// `part`.
fn shaft_along_z(part: &Body, radius: f64, from: f64, to: f64) -> Body {
    let disc = Contour {
        corners: vec![DVec2::X * radius],
        runs: vec![Run::Round {
            center: DVec2::ZERO,
            turn: TAU,
        }],
    };
    raised(part, disc, from, to)
}

/// The half y ≥ 0 of a shaft of `radius` along z from z = `from` to z =
/// `to`, a tool of `part`.
fn half_shaft_along_z(part: &Body, radius: f64, from: f64, to: f64) -> Body {
    let half = Contour {
        corners: vec![DVec2::X * radius, DVec2::X * -radius],
        runs: vec![
            Run::Round {
                center: DVec2::ZERO,
                turn: PI,
            },
            Run::Straight,
        ],
    };
    raised(part, half, from, to)
}

/// A shaft of `radius` along z from z = [`FROM`] up to the origin, a tool of
/// `part`.
fn shaft_up_to_the_centre(part: &Body, radius: f64) -> Body {
    shaft_along_z(part, radius, FROM, 0.0)
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

/// How far from the equator about z a sphere of `radius` crosses a coaxial
/// wall `hair` inside it: a = √(r² − (r − h)²).
fn rim_under(radius: f64, hair: f64) -> f64 {
    (hair * (2.0 * radius - hair)).sqrt()
}

/// What the half ball cut by the shaft `hair` under its radius leaves: the
/// quarter of the ball above its centre, and half the ring the shaft leaves
/// of the ball under it, between z = −a and 0 outside the wall: π ∫ (a² −
/// z²) dz over it is 2π a³ / 3, the half π a³ / 3. #1123's.
fn kept(hair: f64) -> f64 {
    BALL / 4.0 + PI * rim_under(RADIUS, hair).powi(3) / 3.0
}

/// What is wrong with the body `built`, called `what`, if anything:
/// declined, not exact, holding neither of `volumes` — what the arithmetic
/// leaves with the shaft at its own radius, and with it laid onto the touch
/// — to the kernel's billionth, its triangles open or crossing.
fn off(what: &str, built: Result<Body, Declined>, volumes: [f64; 2]) -> Option<String> {
    let body = match built {
        Err(declined) => return Some(format!("{what}: declined, {declined:?}")),
        Ok(body) => body,
    };
    if !body.is_exact() {
        return Some(format!("{what}: not exact"));
    }
    let [drawn, laid] = volumes;
    if volumes
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

/// What is wrong with the half ball cut by the shaft `hair` under its
/// radius, if anything, as [`off`] reads it, at the shaft's radius or with
/// the quarter alone.
fn wrong_with(built: Result<Body, Declined>, hair: f64) -> Option<String> {
    let what = format!("the half ball cut up to its centre by a shaft {hair:e} under its radius");
    off(&what, built, [kept(hair), BALL / 4.0])
}

/// What is wrong with the body `make` builds, called `what`, if anything:
/// built and first drawn past [`QUICK`] on the wall clock, and each way
/// [`off`] finds.
fn held_quickly(
    what: &str,
    make: impl FnOnce() -> Result<Body, Declined>,
    volumes: [f64; 2],
) -> Vec<String> {
    let started = Instant::now();
    let built = make();
    if let Ok(body) = &built {
        body.triangles();
    }
    let took = started.elapsed();
    let mut wrong = Vec::new();
    if took > QUICK {
        wrong.push(format!(
            "{what}: built and drawn in {took:?}, past {QUICK:?}"
        ));
    }
    wrong.extend(off(what, built, volumes));
    wrong
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

/// Every way `case` finds the body of each of `hairs` wrong, gathered and
/// asserted at once for the `family` they belong to.
fn every_hair(family: &str, hairs: &[f64], case: impl Fn(f64) -> Vec<String>) {
    let wrong: Vec<String> = hairs.iter().flat_map(|&hair| case(hair)).collect();
    assert!(
        wrong.is_empty(),
        "{family} is built exact, closed, uncrossed and of the arithmetic's volume, each in \
         under a second; not so:\n{}",
        wrong.join("\n")
    );
}

/// The ball of [`RADIUS`] about z cut by a shaft along z from its south pole
/// up to its centre, 1.2e-7 and 1.5e-7 under its radius: the half ball above
/// the equator, and the ring the shaft leaves under it, 2π a³ / 3.
#[test]
fn a_ball_about_z_cut_up_to_its_centre_by_a_shaft_from_its_pole_whose_rim_falls_within_a_hundredth_of_a_step_of_its_equator_s_row_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume_each_in_under_a_second()
 {
    let ball = ball_of(RADIUS, TAU);
    every_hair(
        "a ball about z cut by a shaft from z = -5 up to its centre",
        &[1.2e-7, 1.5e-7],
        |hair| {
            let shaft = shaft_up_to_the_centre(&ball, RADIUS - hair);
            let ring = TAU * rim_under(RADIUS, hair).powi(3) / 3.0;
            held_quickly(
                &format!("the ball cut up to its centre by a shaft {hair:e} under its radius"),
                || ball.difference(&shaft),
                [BALL / 2.0 + ring, BALL / 2.0],
            )
        },
    );
}

/// The half ball cut by a shaft along z from its centre up past its north
/// pole, to z = 5, 1.2e-7 and 1.5e-7 under its radius: the mirror of the
/// criterion's cut across the equator, its rim a hair over the equator's
/// row, and what it leaves the same.
#[test]
fn a_half_ball_cut_down_to_its_centre_by_a_shaft_from_its_pole_whose_rim_falls_within_a_hundredth_of_a_step_of_its_equator_s_row_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume_each_in_under_a_second()
 {
    let half = half_ball();
    every_hair(
        "a half ball cut by a shaft from its centre up to z = 5",
        &[1.2e-7, 1.5e-7],
        |hair| {
            let shaft = shaft_along_z(&half, RADIUS - hair, 0.0, -FROM);
            held_quickly(
                &format!(
                    "the half ball cut down to its centre by a shaft {hair:e} under its radius"
                ),
                || half.difference(&shaft),
                [kept(hair), BALL / 4.0],
            )
        },
    );
}

/// The ball of [`RADIUS`] about z cut by the half y ≥ 0 of a shaft along z
/// from its south pole up to its centre, 1.2e-7 and 1.5e-7 under its radius:
/// three quarters of the ball, and half the ring the shaft leaves under the
/// equator, π a³ / 3.
#[test]
fn a_ball_cut_up_to_its_centre_by_a_half_shaft_whose_rim_falls_within_a_hundredth_of_a_step_of_its_equator_s_row_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume_each_in_under_a_second()
 {
    let ball = ball_of(RADIUS, TAU);
    every_hair(
        "a ball cut by a half shaft from z = -5 up to its centre",
        &[1.2e-7, 1.5e-7],
        |hair| {
            let shaft = half_shaft_along_z(&ball, RADIUS - hair, FROM, 0.0);
            let ring = PI * rim_under(RADIUS, hair).powi(3) / 3.0;
            held_quickly(
                &format!("the ball cut up to its centre by a half shaft {hair:e} under its radius"),
                || ball.difference(&shaft),
                [3.0 * BALL / 4.0 + ring, 3.0 * BALL / 4.0],
            )
        },
    );
}

/// The radius of the ring's axis circle, about z through the origin.
const MAJOR: f64 = 10.0;

/// The radius of the ring's tube.
const MINOR: f64 = 2.0;

/// The ring: 2π² R r².
const RING: f64 = 2.0 * PI * PI * MAJOR * MINOR * MINOR;

/// The matter of the ring under z = 0 a coaxial wall `hair` inside its outer
/// equator leaves outside it: the half segment of the tube's disc past the
/// chord `hair` in from its edge, turned about z. Its area is r² (2θ −
/// sin 2θ) / 4, cos θ = 1 − h / r, and its moment about the tube's centre
/// r³ sin³θ / 3, outwards; by Pappus, 2π (R A + M). #1123's.
fn outside(hair: f64) -> f64 {
    let angle = 2.0 * (hair / (2.0 * MINOR)).sqrt().asin();
    let area = MINOR * MINOR * (2.0 * angle - (2.0 * angle).sin()) / 4.0;
    let moment = MINOR.powi(3) * angle.sin().powi(3) / 3.0;
    TAU * (MAJOR * area + moment)
}

/// The ring of [`MAJOR`] and [`MINOR`] cut by a coaxial shaft along z from
/// z = −5 up to its centre plane, 3e-7, 3.5e-7 and 4e-7 inside its outer
/// equator: the upper half of the ring, and the sliver of the lower half
/// the shaft leaves outside it.
#[test]
fn a_ring_cut_up_to_its_centre_plane_by_a_coaxial_shaft_whose_rim_falls_within_a_hundredth_of_a_step_of_its_outer_equator_s_row_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume_each_in_under_a_second()
 {
    let ring = ring_of(MAJOR, MINOR);
    every_hair(
        "a ring cut by a coaxial shaft from z = -5 up to its centre plane",
        &[3e-7, 3.5e-7, 4e-7],
        |hair| {
            let shaft = shaft_up_to_the_centre(&ring, MAJOR + MINOR - hair);
            held_quickly(
                &format!("the ring cut by a shaft {hair:e} inside its outer equator"),
                || ring.difference(&shaft),
                [RING / 2.0 + outside(hair), RING / 2.0],
            )
        },
    );
}

/// The radius of the ball the shell is cut from, about the origin.
const SHELL: f64 = 8.0;

/// The ball of [`SHELL`] less the ball of [`RADIUS`], both about z, joined
/// with a shaft along z from z = −5 up to its centre, 1.7e-7, 2e-7 and
/// 2.5e-7 under the cavity's radius: the shaft fills the cavity under the
/// rim, and leaves void its upper half and the ring between the shaft's
/// wall and the cavity's sphere under the equator, 2π a³ / 3.
#[test]
fn a_shell_joined_with_a_shaft_up_to_its_centre_whose_rim_falls_within_a_hundredth_of_a_step_of_its_cavity_s_equator_s_row_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume_each_in_under_a_second()
 {
    let shell = ball_of(SHELL, TAU)
        .difference(&ball_of(RADIUS, TAU))
        .unwrap_or_else(|declined| panic!("the shell: declined, {declined:?}"));
    let solid = 4.0 * PI / 3.0 * SHELL.powi(3) - BALL / 2.0;
    every_hair(
        "a shell of 8 about a cavity of 5 joined with a shaft from z = -5 up to its centre",
        &[1.7e-7, 2e-7, 2.5e-7],
        |hair| {
            let shaft = shaft_up_to_the_centre(&shell, RADIUS - hair);
            let ring = TAU * rim_under(RADIUS, hair).powi(3) / 3.0;
            held_quickly(
                &format!("the shell joined with a shaft {hair:e} under its cavity's radius"),
                || shell.union(&shaft),
                [solid - ring, solid],
            )
        },
    );
}
