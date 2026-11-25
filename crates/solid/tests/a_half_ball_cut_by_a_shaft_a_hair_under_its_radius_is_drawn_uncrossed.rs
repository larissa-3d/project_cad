//! A half ball cut by a shaft a hair under its radius, up to its centre, is
//! drawn uncrossed whatever the hair. The sphere crosses the shaft's wall
//! along a rim √(2rh) under its equator for a hair h, and the cut keeps the
//! ring of ball between that rim, the wall and the shaft's top: a ring a hair
//! thick at the equator, thinner than the chords of either face sag.
//!
//! Closes #1123.
//! - The half ball of 5 about z, cut by the shaft along z from −10 to 0 of
//!   radius 5 − h, for h in 4e-7, 1e-6 and a few between up to where
//!   decision 12 no longer lays it: built, exact, closed, uncrossed, the
//!   arithmetic's volume —
//!   `a_half_ball_cut_up_to_its_centre_by_a_shaft_a_hair_too_far_under_its_radius_to_be_laid_onto_the_touch_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume`.
//!   Read as: decision 12 lays the shaft onto the ball's equator only within
//!   `Scale::HAIR` tolerances of it, 2e-7 at the pair's reach of 10, the
//!   shaft reaching z = −10 — and none of the issue's hairs lies within that.
//!   Measured on `main` (8567010a), the cut is laid up to 2e-7 and drawn
//!   sound there, as #956 pins it. Past 2e-7 it is not laid: from 2.02e-7 to
//!   4e-4 it is built, exact, closed, of the volume at the shaft's radius,
//!   and crossed, always at the seam where the half ball's flat face meets
//!   the ring under the equator. From 5e-4 it is drawn sound. The words
//!   "up to where decision 12 no longer lays it" would ask for nothing past
//!   2e-7, so they are read as the band the issue's hairs belong to, and
//!   held from end to end: 2.1e-7, just past decision 12's reach; the
//!   issue's 4e-7 and 1e-6 with 6e-7 and 8e-7 between; and on up the band to
//!   its top, 1e-5, 1e-4 and 4e-4. The arithmetic is the one at the shaft's
//!   radius at every one of them, since no decision lays the shaft there,
//!   and it is held to the kernel's billionth: from 1e-5 on, that tells it
//!   apart from what the ball would hold with the shaft laid onto the touch
//! - The hairs #956 pins keep their bodies — no test: they are #956's,
//!   `a_half_ball_on_a_shaft_s_top_a_hair_wider_than_it_whose_meridian_arcs_end_a_hair_from_a_row_of_its_grid_is_drawn_closed`
//!   in `spheres_of_one_centre_and_radius_are_one_surface_whatever_their_axes.rs`,
//!   which holds the hairs 5e-8, 1e-7 and 2e-7 built, exact, closed,
//!   uncrossed and of the arithmetic's volume, joined either way and each
//!   cut by the other. Running it is the answer

use std::f64::consts::{PI, TAU};

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Loop};
use glam::{DVec2, DVec3};

/// How far the matter a body holds may stand from what the arithmetic
/// leaves, as a share of it: the kernel's billionth.
const EXACT: f64 = 1e-9;

/// The radius of the ball, about the origin.
const RADIUS: f64 = 5.0;

/// The ball of [`RADIUS`].
const BALL: f64 = 4.0 * PI / 3.0 * RADIUS * RADIUS * RADIUS;

/// How far under the ball's radius each shaft stands: past decision 12's
/// reach, the whole band over which the cut is crossed on `main`.
const HAIRS: [f64; 8] = [2.1e-7, 4e-7, 6e-7, 8e-7, 1e-6, 1e-5, 1e-4, 4e-4];

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
        .expect("the exact kernel turns the half ball")
}

/// A shaft of `radius` along z from z = −10 up to the ball's centre, a tool
/// of `part`. #956's.
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
            origin: DVec3::Z * -10.0,
            u: DVec3::X,
            v: DVec3::Y,
        },
        DVec3::Z * 10.0,
    )
    .expect("the shaft raises")
}

/// What the half ball cut by the shaft `hair` under its radius leaves: the
/// quarter of the ball above its centre, and half the ring the shaft leaves
/// of the ball under it, between z = −a and 0 outside the wall,
/// a = √(r² − (r − h)²): π ∫ (a² − z²) dz over it is 2π a³ / 3, the half
/// π a³ / 3.
fn kept(hair: f64) -> f64 {
    let reach = (hair * (2.0 * RADIUS - hair)).sqrt();
    BALL / 4.0 + PI * reach.powi(3) / 3.0
}

/// The half ball y ≥ 0 cut by a shaft along z from z = −10 up to its centre,
/// each of [`HAIRS`] under its radius: too far off the ball's equator for
/// decision 12 to lay the shaft onto it, so the sphere crosses the wall
/// along a rim a thousandth or a few hundredths under the equator, and the
/// cut keeps the ring of ball the shaft leaves between that rim and its top.
/// Each built, exact, holding what the arithmetic leaves at the shaft's
/// radius, its triangles closed and uncrossed.
#[test]
fn a_half_ball_cut_up_to_its_centre_by_a_shaft_a_hair_too_far_under_its_radius_to_be_laid_onto_the_touch_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let half = half_ball();
    for hair in HAIRS {
        let what =
            format!("the half ball cut up to its centre by a shaft {hair:e} under its radius");
        let shaft = shaft_up_to_the_centre(&half, RADIUS - hair);

        let cut = half
            .difference(&shaft)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        let kept = kept(hair);
        assert!(
            cut.is_exact() && (cut.volume() - kept).abs() <= EXACT * kept,
            "{what}: holds {}, where the arithmetic leaves {kept}",
            cut.volume()
        );
        let triangles = cut.triangles();
        assert_eq!(closed(&triangles), Ok(()), "{what}");
        assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
    }
}
