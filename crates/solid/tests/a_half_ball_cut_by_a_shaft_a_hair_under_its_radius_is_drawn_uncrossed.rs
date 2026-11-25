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
//!   the ring under the equator; from 5e-4 to 1.5e-3 it is drawn sound. The
//!   words "up to where decision 12 no longer lays it" would ask for nothing
//!   past 2e-7, so they are read as the band the issue's hairs belong to,
//!   and held from end to end: 2.1e-7, just past decision 12's reach; the
//!   issue's 4e-7 and 1e-6 with 6e-7 and 8e-7 between; and on up the band to
//!   its top, 1e-5, 1e-4 and 4e-4. The arithmetic is the one at the shaft's
//!   radius at every one of them, since no decision lays the shaft there,
//!   and it is held to the kernel's billionth: from 1e-5 on, that tells it
//!   apart from what the ball would hold with the shaft laid onto the touch
//! - The hairs #956 pins keep their bodies —
//!   `a_shaft_within_decision_twelve_s_reach_of_a_half_ball_s_radius_is_laid_onto_its_equator_with_the_half_ball_first_and_left_as_drawn_with_the_shaft_first`,
//!   which holds the bodies those hairs leave on `main`, joined either way
//!   and each cut by the other: their volumes, and as many flat and curved
//!   faces. #956's own pin,
//!   `a_half_ball_on_a_shaft_s_top_a_hair_wider_than_it_whose_meridian_arcs_end_a_hair_from_a_row_of_its_grid_is_drawn_closed`
//!   in `spheres_of_one_centre_and_radius_are_one_surface_whatever_their_axes.rs`,
//!   holds them drawn: built, exact, closed, uncrossed, and of the volume at
//!   the shaft's radius or the ball's. That pin alone takes either volume,
//!   and so would not tell a hair that stopped being laid from one that
//!   still is
//!
//! Beyond the criteria — pins, not criteria of #1123 — the same family
//! elsewhere: a coaxial wall a hair from the circle of a row of a round's
//! grid. They widen the criterion's family because they are what the fix
//! answers, not the issue's hairs alone. The drawing lines a round's grid
//! up with the grid of each coaxial wall that touches it along an equator
//! within the tolerance, stands within twice its chords' sag of an equator,
//! or shares a rim with it within twice its sag of the circle of either row
//! about that rim. The walls are taken one at a time, touching ones first,
//! then the nearest; one whose multiple with those taken would pass the
//! most a turn is cut into is left out alone (`kept_by_walls`, in
//! `tessellation/sampling/cone.rs`). The probe of the fix's first rule,
//! which took a wall within that sag of any row's circle, found the three
//! cases below, and the first two were crossed on `main`.
//!
//! The half ball cut by the shaft 1.9e-3 and 2.2e-3 under its radius, its
//! rim a hair under the row below the equator rather than under the
//! equator itself:
//! `a_half_ball_cut_up_to_its_centre_by_a_shaft_whose_rim_falls_a_hair_under_the_row_below_its_equator_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume`.
//!
//! A ring cut by a coaxial shaft a hair inside its outer equator, crossed on
//! `main` from 1e-6 to 1e-3:
//! `a_ring_cut_up_to_its_centre_plane_by_a_coaxial_shaft_a_hair_inside_its_outer_equator_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume`.
//!
//! The same ring joined with or cut by a coaxial post a hair outside its
//! inner equator, drawn sound on `main` already. Its grid is lined up with
//! the post's now, and the pin holds it sound there:
//! `a_ring_joined_with_or_cut_by_a_coaxial_post_up_to_its_centre_plane_a_hair_outside_its_inner_equator_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume`.
//!
//! The rule counts a wall only where it stands beside the round, and a
//! wall counted must not undo the grid it lines up with another. The review
//! of the fix found the pins below. A rod whose radius is a hair from the
//! circle of a row of a ring's grid, but which stands far along the axis
//! from it, on the end of a post the ring touches along its inner equator:
//! drawn sound on `main`, crossed at the post's touch by the fix's first
//! rule (8b973844), which read only the rod's radius, counted it, and gave
//! the ring back its own steps since no grid a multiple of the post's and
//! the rod's stood under the most a turn is cut into —
//! `a_ring_round_a_post_of_its_inner_radius_with_a_rod_on_the_post_s_end_clear_of_the_ring_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume`.
//! Under the current rule the rod shares no rim with the ring and is not
//! counted at all, so that pin no longer holds which wall is taken first.
//! This one does: the same ring and post, bored from above by a wall that
//! crosses the tube a hair from a row, counted with the post, their grids
//! with no multiple under that most. The post, touching, is taken first and
//! the bore left out; taken the other way round, the post's touch crossed —
//! `a_ring_round_a_post_of_its_inner_radius_then_bored_from_above_across_its_tube_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume`.
//! The same undoing in the criterion's own case: the half ball cut by the
//! shaft 1e-6 under its radius, then bored by a pin beside another row,
//! whose grid and the shaft's have no multiple under that most, crossed at
//! the shaft's rim again —
//! `a_half_ball_cut_up_to_its_centre_by_a_shaft_a_hair_under_its_radius_then_bored_blind_from_its_top_by_a_pin_beside_a_row_of_its_grid_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume`.
//! And a wall a hair off an equator but standing nowhere beside the round:
//! the ring cut by the shaft a hair inside its outer equator, then joined
//! with a hub above its tube and a rod on the hub, a millionth outside the
//! ring's inner equator and 3 above its tube. The fix's third rule
//! (f92cb341) counted the rod by its gap to the equator alone, took it
//! before the shaft for that gap being the smaller, and left the shaft out,
//! and the shaft's rim crossed again —
//! `a_ring_cut_by_a_shaft_a_hair_inside_its_outer_equator_then_joined_with_a_rod_a_hair_outside_its_inner_one_standing_above_it_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume`.
//!
//! A wall within twice its chords' sag of a round's equator counts too,
//! though the two share no rim: a bore a hair inside a groove's bottom, or
//! a wall on one ball's equator a hair outside a smaller one's. Four seeds of
//! `a_campaign_of_rounded_solids_on_the_exact_kernel_keeps_every_rule`
//! were crossed on the fix's second rule (816767ba), which counted a wall
//! only where it shares a rim with the round or touches an equator within
//! the tolerance; each is held as the campaign draws it and as the triage
//! shrinks it —
//! `seed_956000000_a_half_cylinder_on_a_ball_s_equator_added_to_the_shell_a_ball_a_hair_smaller_leaves_of_it_keeps_every_rule`,
//! `seed_956002773_a_groove_bowed_down_to_a_round_s_inner_equator_cut_by_a_coaxial_bore_a_hair_inside_it_keeps_every_rule`,
//! `seed_956002803_the_same_groove_larger_cut_by_a_coaxial_bore_a_hair_inside_its_bottom_keeps_every_rule`,
//! `seed_956006352_a_round_bowed_down_to_an_equator_of_one_and_a_half_cut_by_a_coaxial_bore_a_hair_inside_it_keeps_every_rule`.

// The campaigns' harness, of which only the seeds below use a corner: the
// rest of it is dead code here.
#[allow(dead_code, unused_imports)]
mod random_solids;

use std::collections::BTreeSet;
use std::f64::consts::{PI, TAU};

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Declined, Loop};
use glam::{DVec2, DVec3};
use random_solids::{Case, Leaf, Plane, Section, Step};

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

/// The radius of the ring's axis circle, about z through the origin.
const MAJOR: f64 = 10.0;

/// The radius of the ring's tube.
const MINOR: f64 = 2.0;

/// The ring: 2π² R r².
const RING: f64 = 2.0 * PI * PI * MAJOR * MINOR * MINOR;

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
    turned_whole_or_half(half, sampled, PI)
}

/// The disc of `minor` about (`major`, 0) in the plane y = 0, turned whole
/// about z: a ring about the origin, its outer equator of radius
/// `major + minor` and its inner one of `major − minor` on the plane z = 0.
/// Of [`MAJOR`] and [`MINOR`], those are 12 and 8.
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
    turned_whole_or_half(disc, sampled, TAU)
}

/// `outline`, drawn in the plane y = 0 with x across and z up, turned
/// `angle` about z from x towards y.
fn turned_whole_or_half(outline: Contour, sampled: &'static [DVec2], angle: f64) -> Body {
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
        .tool_turned(&profile, frame, &Turn::of(along_v, angle, 0.0, &profile))
        .expect("the exact kernel turns the profile")
}

/// A shaft of `radius` along z from z = `from` up to the origin, a tool of
/// `part`. #956's.
fn shaft_up_to_the_centre(part: &Body, radius: f64, from: f64) -> Body {
    shaft_along_z(part, radius, from, 0.0)
}

/// A shaft of `radius` along z from z = `from` to z = `to`, a tool of
/// `part`.
fn shaft_along_z(part: &Body, radius: f64, from: f64, to: f64) -> Body {
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
            origin: DVec3::Z * from,
            u: DVec3::X,
            v: DVec3::Y,
        },
        DVec3::Z * (to - from),
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

/// The matter of the ring under z = 0 a coaxial wall `hair` inside its
/// outer equator leaves outside it, and the matter a wall `hair` outside its
/// inner equator takes inside it: the half segment of the tube's disc past
/// the chord `hair` in from its edge, turned about z. Its area is
/// r² (2θ − sin 2θ) / 4, cos θ = 1 − h / r, and its moment about the
/// tube's centre r³ sin³θ / 3, outwards or inwards; by Pappus, 2π (R A ± M).
fn sliver(hair: f64) -> [f64; 2] {
    let angle = 2.0 * (hair / (2.0 * MINOR)).sqrt().asin();
    let area = MINOR * MINOR * (2.0 * angle - (2.0 * angle).sin()) / 4.0;
    let moment = MINOR.powi(3) * angle.sin().powi(3) / 3.0;
    [TAU * (MAJOR * area + moment), TAU * (MAJOR * area - moment)]
}

/// What a pin of `radius` along z from z = 1 up past the ball takes of the
/// half ball: half what it takes of the ball, the cylinder from z = 1 up to
/// where its wall meets the sphere, z = √(r² − b²), and the cap above that,
/// π H² (3r − H) / 3 for H = r − z.
fn pinned(radius: f64) -> f64 {
    let meets = (RADIUS * RADIUS - radius * radius).sqrt();
    let cap = RADIUS - meets;
    (PI * radius * radius * (meets - 1.0) + PI * cap * cap * (3.0 * RADIUS - cap) / 3.0) / 2.0
}

/// What a wall of radius `wall` about the axis of the ring of `major` and
/// `minor`, between its inner equator and its tube's centre, holds of the
/// tube above the plane z = `from`, from there up to where the wall leaves
/// the tube: π ∫ (H² − (R − √(r² − z²))²) dz, with H the wall's radius,
/// which is π [(H² − R² − r²) z + z³ / 3 + 2R ((z / 2) √(r² − z²)
/// + (r² / 2) asin(z / r))] from `from` up to √(r² − (R − H)²).
fn wall_in_the_tube(major: f64, minor: f64, wall: f64, from: f64) -> f64 {
    let primitive = |z: f64| {
        (wall * wall - major * major - minor * minor) * z
            + z.powi(3) / 3.0
            + 2.0
                * major
                * (z / 2.0 * (minor * minor - z * z).sqrt()
                    + minor * minor / 2.0 * (z / minor).asin())
    };
    let leaves = (minor * minor - (major - wall).powi(2)).sqrt();
    PI * (primitive(leaves) - primitive(from))
}

/// Holds that a boolean built a body that is exact, holds `volume` to the
/// kernel's billionth, its triangles closed and uncrossed.
fn built_sound_holding(built: Result<Body, Declined>, volume: f64, what: &str) {
    let body = built.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    assert!(
        body.is_exact() && (body.volume() - volume).abs() <= EXACT * volume,
        "{what}: holds {}, where the arithmetic leaves {volume}",
        body.volume()
    );
    let triangles = body.triangles();
    assert_eq!(closed(&triangles), Ok(()), "{what}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
}

/// How many faces of `body` are flat, and how many are not.
fn kinds(body: &Body) -> [usize; 2] {
    let faces: BTreeSet<usize> = (0..body.faces_end())
        .filter(|&face| body.has_face(face))
        .chain(body.pieces())
        .collect();
    let flat = faces.iter().filter(|&&face| body.is_flat(face)).count();
    [flat, faces.len() - flat]
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
        let shaft = shaft_up_to_the_centre(&half, RADIUS - hair, -10.0);

        let cut = half.difference(&shaft);

        built_sound_holding(
            cut,
            kept(hair),
            &format!("the half ball cut up to its centre by a shaft {hair:e} under its radius"),
        );
    }
}

/// The shafts #956 pins, 5, 10 and 20 hundred-millionths under the half
/// ball's radius, along z from z = −10 up to its centre: within decision
/// 12's reach, 20 tolerances at the pair's reach of 10. With the half ball
/// first, the shaft is laid onto the equator: joined, the ball's quarter
/// above its centre on a shaft of its radius, 250π + the quarter; cut, the
/// quarter alone, its three flat faces — the two halves of the meridian
/// plane and the equator's — and one on the sphere, no wall left. With the
/// shaft first, it carries no round and the sphere is not moved onto it, so
/// it is left as drawn: joined, the shaft and what the cut leaves beside it,
/// the quarter and the ring under the equator; cut, the shaft less the half
/// ball's part inside it. Each exact, holding that to
/// the kernel's billionth, which tells the shaft laid from the shaft as
/// drawn for every one but the cut, and with as many flat and curved faces
/// as on `main` — which tells them apart for the cut.
#[test]
fn a_shaft_within_decision_twelve_s_reach_of_a_half_ball_s_radius_is_laid_onto_its_equator_with_the_half_ball_first_and_left_as_drawn_with_the_shaft_first()
 {
    let half = half_ball();
    for hair in [5e-8, 1e-7, 2e-7] {
        let radius = RADIUS - hair;
        let shaft = shaft_up_to_the_centre(&half, radius, -10.0);
        let drawn = PI * radius * radius * 10.0;
        for (way, built, volume, faces) in [
            (
                "the half ball joined with",
                half.union(&shaft),
                250.0 * PI + BALL / 4.0,
                [4, 2],
            ),
            (
                "the half ball cut by",
                half.difference(&shaft),
                BALL / 4.0,
                [3, 1],
            ),
            (
                "the half ball joined to, the shaft first,",
                shaft.union(&half),
                drawn + kept(hair),
                [4, 2],
            ),
            (
                "the half ball cut from",
                shaft.difference(&half),
                drawn - (BALL / 2.0 - kept(hair)),
                [4, 2],
            ),
        ] {
            let what = format!("{way} a shaft {hair:e} under its radius up to its centre");
            let body = built.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
            assert!(
                body.is_exact() && (body.volume() - volume).abs() <= EXACT * volume,
                "{what}: holds {}, where the arithmetic leaves {volume}",
                body.volume()
            );
            assert_eq!(
                kinds(&body),
                faces,
                "{what}: as many flat and curved faces as on main"
            );
        }
    }
}

/// The half ball cut by a shaft from z = −10 up to its centre 1.9e-3 and
/// 2.2e-3 under its radius: the sphere crosses the wall along a rim 0.138
/// and 0.148 under the equator, a hair under the row of the sphere's grid
/// below it, at z = −0.133 — the same ring thinner than the chords sag, beside
/// another row than the equator. Each built, exact, holding what the
/// arithmetic leaves at the shaft's radius, its triangles closed and
/// uncrossed.
#[test]
fn a_half_ball_cut_up_to_its_centre_by_a_shaft_whose_rim_falls_a_hair_under_the_row_below_its_equator_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let half = half_ball();
    for hair in [1.9e-3, 2.2e-3] {
        let shaft = shaft_up_to_the_centre(&half, RADIUS - hair, -10.0);

        let cut = half.difference(&shaft);

        built_sound_holding(
            cut,
            kept(hair),
            &format!("the half ball cut up to its centre by a shaft {hair:e} under its radius"),
        );
    }
}

/// The ring cut by a shaft along z from z = −5 up to its centre plane, a
/// millionth to a thousandth inside its outer equator, of radius 12 − h: the
/// tube crosses the wall along a rim under the equator, and the cut keeps
/// the upper half of the ring and the sliver of the lower half outside the
/// wall. Each built, exact, holding what the arithmetic leaves, its
/// triangles closed and uncrossed.
#[test]
fn a_ring_cut_up_to_its_centre_plane_by_a_coaxial_shaft_a_hair_inside_its_outer_equator_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let ring = ring_of(MAJOR, MINOR);
    for hair in [1e-6, 1e-5, 1e-4, 4e-4, 1e-3] {
        let shaft = shaft_up_to_the_centre(&ring, MAJOR + MINOR - hair, -5.0);
        let [outside, _] = sliver(hair);

        let cut = ring.difference(&shaft);

        built_sound_holding(
            cut,
            RING / 2.0 + outside,
            &format!("the ring cut by a shaft {hair:e} inside its outer equator"),
        );
    }
}

/// The ring joined with, and cut by, a post along z from z = −5 up to its
/// centre plane, a millionth to a thousandth outside its inner equator, of
/// radius 8 + h: the tube crosses the post's wall along a rim under the
/// equator. Joined, the ring and the post less the sliver they share; cut,
/// the ring less that sliver. Each built, exact, holding what the arithmetic
/// leaves, its triangles closed and uncrossed.
#[test]
fn a_ring_joined_with_or_cut_by_a_coaxial_post_up_to_its_centre_plane_a_hair_outside_its_inner_equator_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let ring = ring_of(MAJOR, MINOR);
    for hair in [1e-6, 1e-4, 1e-3] {
        let radius = MAJOR - MINOR + hair;
        let post = shaft_up_to_the_centre(&ring, radius, -5.0);
        let [_, inside] = sliver(hair);
        for (way, built, volume) in [
            (
                "joined with",
                ring.union(&post),
                RING + PI * radius * radius * 5.0 - inside,
            ),
            ("cut by", ring.difference(&post), RING - inside),
        ] {
            built_sound_holding(
                built,
                volume,
                &format!("the ring {way} a post {hair:e} outside its inner equator"),
            );
        }
    }
}

/// 5576855166's ring, of tube 45 about a ring radius of 60, joined to a
/// coaxial post of radius 15 along z from z = −45 to 45, which touches it
/// along its inner equator, and then to a rod of radius 24.035 from z = 45
/// to 80 standing on the post's end, clear of the ring. The ring's grid
/// holds a row whose circle, of radius 24.025, stands 0.01 inside the rod's
/// wall but 27 up the axis, 18 under the rod's foot: the rod stands beside
/// no row of the ring. Built, exact, holding the ring, the post and the rod
/// — the post touches the ring along a circle and the rod stands clear of
/// it, so they share no matter — its triangles closed and uncrossed, the
/// post's touch along the inner equator among them.
#[test]
fn a_ring_round_a_post_of_its_inner_radius_with_a_rod_on_the_post_s_end_clear_of_the_ring_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let (major, minor, rod) = (60.0, 45.0, 24.035);
    let ring = ring_of(major, minor);
    let post = shaft_along_z(&ring, major - minor, -minor, minor);
    let posted = ring
        .union(&post)
        .unwrap_or_else(|declined| panic!("the ring round its post: declined, {declined:?}"));
    let rod_on_the_post = shaft_along_z(&posted, rod, minor, 80.0);

    let built = posted.union(&rod_on_the_post);

    built_sound_holding(
        built,
        2.0 * PI * PI * major * minor * minor
            + PI * (major - minor).powi(2) * 2.0 * minor
            + PI * rod * rod * (80.0 - minor),
        "the ring round a post of its inner radius, a rod of 24.035 on the post's end",
    );
}

/// The half ball cut by a shaft from z = −10 up to its centre 1e-6 under its
/// radius, as the criterion asks, then bored blind from its top by a pin of
/// radius 1.0554 from z = 1 up past the ball. The pin's wall meets the
/// sphere at z = 4.887, within a thousandth of the circle of a row of the
/// sphere's grid, of radius 1.0564: the pin stands beside a row as the shaft
/// does beside the equator, and the grid a multiple of both walls' would
/// need more columns than a turn is cut into. Built, exact, holding what the
/// cut leaves less what the pin takes, its triangles closed and uncrossed —
/// the shaft's rim under the equator among them.
#[test]
fn a_half_ball_cut_up_to_its_centre_by_a_shaft_a_hair_under_its_radius_then_bored_blind_from_its_top_by_a_pin_beside_a_row_of_its_grid_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let (hair, pin) = (1e-6, 1.0554);
    let half = half_ball();
    let shaft = shaft_up_to_the_centre(&half, RADIUS - hair, -10.0);
    let cut = half.difference(&shaft).unwrap_or_else(|declined| {
        panic!("the half ball cut by its shaft: declined, {declined:?}")
    });
    let bore = shaft_along_z(&cut, pin, 1.0, 10.0);

    let bored = cut.difference(&bore);

    built_sound_holding(
        bored,
        kept(hair) - pinned(pin),
        "the half ball cut by a shaft 1e-6 under its radius, then bored by a pin of 1.0554",
    );
}

/// Seed 956000000, rounded. Shrunk: a ball of 3.5 turned about the line
/// along y through (0, 0, −2), cut by a ball of its centre and axis
/// 6e-8 smaller, which leaves a shell 6e-8 thick, then joined with a half
/// cylinder of 3.5 about that line, from y = −4.5 to 4.5: its wall stands on
/// the outer sphere's equator, and 6e-8 outside the inner one's, with which
/// it shares no rim. The two spheres of the shell, drawn on grids of their
/// own, crossed near the pole.
#[test]
fn seed_956000000_a_half_cylinder_on_a_ball_s_equator_added_to_the_shell_a_ball_a_hair_smaller_leaves_of_it_keeps_every_rule()
 {
    random_solids::holds_exactly(&Case::drawn_rounded_off_the_lattice(956000000));
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-2.0),
            random_solids::Axis::second(0.0),
            Section::bands(-3.5, &[[7.0, 0.0, 0.0]]).bowed(&[[[0.0, 0.0, 0.0], [0.0, 0.0, 3.5]]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::yz(0.0),
                random_solids::Axis::first(-2.0),
                Section::bands(-3.49999994, &[[6.99999988, 0.0, 0.0]])
                    .bowed(&[[[0.0, 0.0, 0.0], [-0.0, 0.0, 3.49999994]]]),
                360.0,
            )),
            Step::add(Leaf::turned(
                Plane::xy(-2.0),
                random_solids::Axis::second(0.0),
                Section::bands(-4.5, &[[9.0, 0.0, 3.5]]),
                180.0,
            )),
        ],
    ));
}

/// Seed 956002773, rounded. Shrunk: a rod of 17.5 along x, grooved by a
/// round of 5 whose bottom is a circle of 12.5 about the axis, the round's
/// inner equator, then bored along the axis at 12.49999: the bore's
/// wall passes a hundred-thousandth inside that equator and shares no rim
/// with the round. The round's triangles crossed the bore's.
#[test]
fn seed_956002773_a_groove_bowed_down_to_a_round_s_inner_equator_cut_by_a_coaxial_bore_a_hair_inside_it_keeps_every_rule()
 {
    random_solids::holds_exactly(&Case::drawn_rounded_off_the_lattice(956002773));
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            random_solids::Axis::first(0.0),
            Section::bands(20.0, &[[5.0, 0.0, 17.5], [10.0, 0.0, 17.5]]).bowed(&[
                [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
                [[0.0, 0.0, 0.0], [30.0, 17.5, -5.0]],
            ]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(0.0),
            random_solids::Axis::first(0.0),
            Section::bands(17.5, &[[25.0, 0.0, 12.49999]]),
            360.0,
        ))],
    ));
}

/// Seed 956002803, rounded. Shrunk: the same groove on a rod of 120, its
/// round of 30 bowed down to an inner equator of 90 about the axis, bored at
/// 89.99999 from the other end. The round's triangles crossed the bore's.
#[test]
fn seed_956002803_the_same_groove_larger_cut_by_a_coaxial_bore_a_hair_inside_its_bottom_keeps_every_rule()
 {
    random_solids::holds_exactly(&Case::drawn_rounded_off_the_lattice(956002803));
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(150.0),
            random_solids::Axis::first(0.0),
            Section::bands(75.0, &[[30.0, -120.0, 0.0], [60.0, -120.0, 0.0]]).bowed(&[
                [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
                [[135.0, -120.0, 30.0], [0.0, 0.0, 0.0]],
            ]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(150.0),
            random_solids::Axis::first(0.0).backwards(),
            Section::bands(-195.0, &[[120.0, -89.99999, -0.0]]),
            360.0,
        ))],
    ));
}

/// Seed 956006352, rounded. Shrunk: a rod of 4 ending in a neck of 2,
/// grooved by a round of 0.5 down to an inner equator of 1.5 about its axis,
/// bored along the axis at 1.4999998, two ten-millionths inside that equator, sharing no rim
/// with the round. The round's triangles crossed the bore's.
#[test]
fn seed_956006352_a_round_bowed_down_to_an_equator_of_one_and_a_half_cut_by_a_coaxial_bore_a_hair_inside_it_keeps_every_rule()
 {
    random_solids::holds_exactly(&Case::drawn_rounded_off_the_lattice(956006352));
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(-2.0),
            random_solids::Axis::second(-3.0).backwards(),
            Section::bands(1.0, &[[3.0, -4.0, 0.0], [1.0, -2.0, 0.0]]).bowed(&[
                [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
                [[4.5, -2.0, 0.5], [0.0, 0.0, 0.0]],
            ]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xz(-3.0),
            random_solids::Axis::second(-2.0).backwards(),
            Section::bands(3.5, &[[2.0, -1.4999998, -0.0]]),
            360.0,
        ))],
    ));
}

/// 5576855166's ring, of tube 45 about a ring radius of 60, joined to a
/// coaxial post of radius 15 along z from z = −45 to 45, which touches it
/// along its inner equator, then bored from above by a wall of radius
/// 24.035 from z = 20 to 60: the bore crosses the tube along a rim at
/// z ≈ 27, a hair from a row of the ring's grid, and takes the post's top.
/// The post's grid and the bore's have no multiple under the most a turn is
/// cut into, so only one of them can line the ring up; the post touches the
/// ring and is taken first. Built, exact, holding the ring and the post less
/// what the bore takes of each, its triangles closed and uncrossed, the
/// post's touch along the inner equator among them.
#[test]
fn a_ring_round_a_post_of_its_inner_radius_then_bored_from_above_across_its_tube_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let (major, minor, bore) = (60.0, 45.0, 24.035);
    let post_radius = major - minor;
    let ring = ring_of(major, minor);
    let post = shaft_along_z(&ring, post_radius, -minor, minor);
    let posted = ring
        .union(&post)
        .unwrap_or_else(|declined| panic!("the ring round its post: declined, {declined:?}"));
    let from_above = shaft_along_z(&posted, bore, 20.0, 60.0);

    let bored = posted.difference(&from_above);

    built_sound_holding(
        bored,
        2.0 * PI * PI * major * minor * minor + PI * post_radius * post_radius * (2.0 * minor)
            - PI * post_radius * post_radius * (minor - 20.0)
            - wall_in_the_tube(major, minor, bore, 20.0),
        "the ring round a post of its inner radius, bored from above at 24.035",
    );
}

/// The ring of [`MAJOR`] and [`MINOR`] cut by a shaft along z from z = −5 up
/// to its centre plane, 1e-5 and 1e-4 inside its outer equator, as pinned
/// above, then joined with a hub of radius 9.05 from z = 1 to 5, which
/// takes in the top of the tube's inner side, and with a rod of radius
/// 8 + 1e-6 from z = 5 to 10 on the hub: a millionth outside the ring's
/// inner equator, 3 above the tube's top, beside the ring nowhere. Built,
/// exact, holding the cut ring, the hub less what it shares with the tube,
/// and the rod, its triangles closed and uncrossed — the shaft's rim under
/// the outer equator among them.
#[test]
fn a_ring_cut_by_a_shaft_a_hair_inside_its_outer_equator_then_joined_with_a_rod_a_hair_outside_its_inner_one_standing_above_it_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let (hub, rod) = (9.05, MAJOR - MINOR + 1e-6);
    let ring = ring_of(MAJOR, MINOR);
    for hair in [1e-5, 1e-4] {
        let shaft = shaft_up_to_the_centre(&ring, MAJOR + MINOR - hair, -5.0);
        let cut = ring
            .difference(&shaft)
            .unwrap_or_else(|declined| panic!("the ring cut by its shaft: declined, {declined:?}"));
        let hub_on_the_tube = shaft_along_z(&cut, hub, 1.0, 5.0);
        let hubbed = cut
            .union(&hub_on_the_tube)
            .unwrap_or_else(|declined| panic!("the ring with its hub: declined, {declined:?}"));
        let rod_on_the_hub = shaft_along_z(&hubbed, rod, 5.0, 10.0);
        let [outside, _] = sliver(hair);

        let built = hubbed.union(&rod_on_the_hub);

        built_sound_holding(
            built,
            RING / 2.0 + outside + PI * hub * hub * 4.0 - wall_in_the_tube(MAJOR, MINOR, hub, 1.0)
                + PI * rod * rod * 5.0,
            &format!("the ring cut by a shaft {hair:e} inside its outer equator, a hub and a rod"),
        );
    }
}
