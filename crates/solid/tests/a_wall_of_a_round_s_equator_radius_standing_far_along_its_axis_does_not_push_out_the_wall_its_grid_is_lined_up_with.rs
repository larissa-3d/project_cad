//! A wall of a round's equator radius, standing far along the round's axis,
//! does not push out the wall the round's grid is lined up with. The drawing
//! lines a round's grid up with the coaxial walls a hair off one of its rows
//! or equators, one wall at a time, touching walls first, and leaves out
//! alone a wall whose multiple with those taken would pass the most a turn is
//! cut into (#1123). A wall of exactly an equator's radius counts as touching
//! wherever it stands along the axis, so a rod of a ring's inner radius
//! standing above the ring was taken first and left out the shaft a hair
//! inside the ring's outer equator, whose rim crossed.
//!
//! Closes #1130.
//! - The case above, rod of exactly 8 on the hub and disjoint: built, exact,
//!   closed, uncrossed, the arithmetic's volume —
//!   `a_ring_cut_by_a_shaft_a_hair_inside_its_outer_equator_then_joined_with_a_rod_of_exactly_its_inner_radius_standing_above_it_on_a_hub_or_alone_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume`.
//!   Read as: the case above, the issue's, built in this order, each
//!   boolean's part the body built so far and its tool the solid it takes
//!   in: the ring of [`MAJOR`] 10 and [`MINOR`] 2, a disc of 2 about
//!   (10, 0, 0) in the plane y = 0 turned whole about z, its equators of 8
//!   and 12 on the plane z = 0, cut by the shaft of radius 12 − h along z
//!   from z = −5 up to that plane, for h 1e-5 and 1e-4, the issue's two; on
//!   the hub, the cut joined with the hub of radius 9.05 along z from z = 1
//!   to 5, which takes in the top of the tube's inner side, and that joined
//!   with the rod of radius exactly 8, the ring's inner equator's, along z
//!   from z = 5 to 10, standing on the hub's top; disjoint, the cut joined
//!   with the same rod alone, standing 3 above the tube's top and sharing
//!   nothing with it; built, never declined; exact, the exact kernel computed
//!   the step; closed and uncrossed, its triangles so by `soundness`; the
//!   arithmetic's volume, within the kernel's billionth of it: the ring,
//!   2π² R r², less the shaft's part, which is the half of the ring under
//!   z = 0 but for the sliver outside the shaft's wall — the half segment of
//!   the tube's disc past the chord h in from its edge, of area
//!   A = r² (2θ − sin 2θ) / 4 for cos θ = 1 − h / r and of moment
//!   M = r³ sin³θ / 3 outwards about the tube's centre, turned about z,
//!   2π (R A + M) by Pappus — so the half ring and that sliver; plus, on the
//!   hub, the hub, π 9.05² · 4, less the tube's matter inside its wall from
//!   z = 1 up to where the wall leaves the tube, at √(r² − (R − 9.05)²),
//!   π ∫ (9.05² − (R − √(r² − z²))²) dz; plus the rod, π 8² · 5, which shares
//!   nothing with what stands under it, on the hub or alone. Measured on
//!   `main` (0ce1e756), all four are built, exact, closed and of that
//!   volume, and crossed at (11.9966, 0.2856, 0), on the shaft's rim
//! - #1123's tests keep their bodies, the post of a ring's inner radius
//!   stopping a hair short of its equator (z from 0.01) among them — no test:
//!   they are the tests of
//!   `a_half_ball_cut_by_a_shaft_a_hair_under_its_radius_is_drawn_uncrossed.rs`,
//!   that post's
//!   `a_ring_beside_a_post_of_its_inner_radius_that_stops_a_hair_short_of_its_equator_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume`
//!   among them, and those #1123 added to
//!   `src/brep/tessellation/sampling/cone/lined_up/tests.rs` and
//!   `src/brep/tessellation/sampling/axis/tests.rs`; running them unchanged
//!   is the answer
//!
//! Beyond the criteria — pins, not criteria of #1130. The criterion's bodies
//! built in other orders, of the same solids: on the hub, the rod joined
//! before the hub, the rod the part and the rest the tool, and the shaft cut
//! last from the ring joined with its hub and its rod; alone, the rod the
//! part, and the shaft cut last from the ring joined with its rod. Measured
//! on `main` (0ce1e756), each of the five, at either hair, is built, exact,
//! closed and of the criterion's volume, and crossed at the same point of
//! the shaft's rim —
//! `a_ring_cut_by_a_shaft_a_hair_inside_its_outer_equator_and_joined_with_a_rod_of_exactly_its_inner_radius_standing_above_it_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume_in_whichever_order_its_solids_are_taken`.
//!
//! The same rod a millionth outside the ring's inner equator, of radius
//! 8 + 1e-6, is too far off the equator's radius to be taken for touching,
//! and stands beside the ring nowhere, so it is not counted at all: drawn
//! sound on `main`, on the hub as alone and in each order above. On the hub
//! it is #1123's pin
//! `a_ring_cut_by_a_shaft_a_hair_inside_its_outer_equator_then_joined_with_a_rod_a_hair_outside_its_inner_one_standing_above_it_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume`;
//! alone, it is held here —
//! `a_ring_cut_by_a_shaft_a_hair_inside_its_outer_equator_then_joined_with_a_rod_a_millionth_outside_its_inner_radius_standing_alone_above_it_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume`.

use std::f64::consts::{PI, TAU};

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Declined, Loop};
use glam::{DVec2, DVec3};

/// How far the matter a body holds may stand from what the arithmetic
/// leaves, as a share of it: the kernel's billionth.
const EXACT: f64 = 1e-9;

/// The radius of the ring's axis circle, about z through the origin.
const MAJOR: f64 = 10.0;

/// The radius of the ring's tube.
const MINOR: f64 = 2.0;

/// The ring: 2π² R r².
const RING: f64 = 2.0 * PI * PI * MAJOR * MINOR * MINOR;

/// How far inside the ring's outer equator each shaft stands: the issue's.
const HAIRS: [f64; 2] = [1e-5, 1e-4];

/// The radius of the hub, from z = 1 to 5.
const HUB: f64 = 9.05;

/// The disc of [`MINOR`] about ([`MAJOR`], 0) in the plane y = 0, turned
/// whole about z: a ring about the origin, its outer equator of radius 12 and
/// its inner one of 8 on the plane z = 0. #1123's.
fn ring() -> Body {
    let sampled: &'static [DVec2] = Box::leak(
        (0..64)
            .map(|step| {
                DVec2::new(MAJOR, 0.0) + DVec2::from_angle(TAU * f64::from(step) / 64.0) * MINOR
            })
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let disc = Contour {
        corners: vec![DVec2::new(MAJOR + MINOR, 0.0)],
        runs: vec![Run::Round {
            center: DVec2::new(MAJOR, 0.0),
            turn: TAU,
        }],
    };
    let profile = Profile {
        exact: Some((disc, Vec::new())),
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
        .tool_turned(&profile, frame, &Turn::of(along_v, TAU, 0.0, &profile))
        .expect("the exact kernel turns the profile")
}

/// A shaft of `radius` along z from z = `from` to z = `to`, a tool of
/// `part`. #1123's.
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

/// The matter of the ring under z = 0 a coaxial wall `hair` inside its
/// outer equator leaves outside it: the half segment of the tube's disc past
/// the chord `hair` in from its edge, turned about z. Its area is
/// r² (2θ − sin 2θ) / 4, cos θ = 1 − h / r, and its moment about the tube's
/// centre r³ sin³θ / 3, outwards; by Pappus, 2π (R A + M). #1123's.
fn sliver(hair: f64) -> f64 {
    let angle = 2.0 * (hair / (2.0 * MINOR)).sqrt().asin();
    let area = MINOR * MINOR * (2.0 * angle - (2.0 * angle).sin()) / 4.0;
    let moment = MINOR.powi(3) * angle.sin().powi(3) / 3.0;
    TAU * (MAJOR * area + moment)
}

/// What the hub's wall, of radius [`HUB`] between the ring's inner equator
/// and its tube's centre, holds of the tube above z = 1, from there up to
/// where the wall leaves the tube: π ∫ (H² − (R − √(r² − z²))²) dz, which is
/// π [(H² − R² − r²) z + z³ / 3 + 2R ((z / 2) √(r² − z²)
/// + (r² / 2) asin(z / r))] from 1 up to √(r² − (R − H)²). #1123's.
fn hub_in_the_tube() -> f64 {
    let primitive = |z: f64| {
        (HUB * HUB - MAJOR * MAJOR - MINOR * MINOR) * z
            + z.powi(3) / 3.0
            + 2.0
                * MAJOR
                * (z / 2.0 * (MINOR * MINOR - z * z).sqrt()
                    + MINOR * MINOR / 2.0 * (z / MINOR).asin())
    };
    let leaves = (MINOR * MINOR - (MAJOR - HUB).powi(2)).sqrt();
    PI * (primitive(leaves) - primitive(1.0))
}

/// A body's booleans, run only when its turn to be checked comes, so that
/// the first body drawn crossed stops the test before the others are built.
type Building<'a> = Box<dyn Fn() -> Result<Body, Declined> + 'a>;

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

/// The ring cut by the shaft `hair` inside its outer equator along z from
/// z = −5 up to its equators' plane, and what the arithmetic leaves of it:
/// the half ring above that plane and the sliver outside the shaft's wall.
fn cut_by_the_shaft(hair: f64) -> (Body, f64) {
    let ring = ring();
    let shaft = shaft_along_z(&ring, MAJOR + MINOR - hair, -5.0, 0.0);
    let cut = ring
        .difference(&shaft)
        .unwrap_or_else(|declined| panic!("the ring cut by its shaft: declined, {declined:?}"));
    (cut, RING / 2.0 + sliver(hair))
}

/// `cut` joined with the hub along z from z = 1 to 5, and what the
/// arithmetic leaves: `holds`, and the hub less what it shares with the
/// tube.
fn hubbed(cut: &Body, holds: f64) -> (Body, f64) {
    let hub = shaft_along_z(cut, HUB, 1.0, 5.0);
    let hubbed = cut
        .union(&hub)
        .unwrap_or_else(|declined| panic!("the ring with its hub: declined, {declined:?}"));
    (hubbed, holds + PI * HUB * HUB * 4.0 - hub_in_the_tube())
}

/// The ring cut by the shaft 1e-5 and 1e-4 inside its outer equator, then
/// joined with the hub of radius 9.05 from z = 1 to 5 and with a rod of
/// radius exactly 8 from z = 5 to 10 on the hub, or with the same rod alone,
/// 3 above the tube's top: the rod is of the ring's inner equator's radius,
/// and beside the ring nowhere along its axis. Each built, exact, holding
/// the cut ring, the hub less what it shares with the tube where there is
/// one, and the rod, its triangles closed and uncrossed — the shaft's rim
/// under the outer equator among them.
#[test]
fn a_ring_cut_by_a_shaft_a_hair_inside_its_outer_equator_then_joined_with_a_rod_of_exactly_its_inner_radius_standing_above_it_on_a_hub_or_alone_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let rod = MAJOR - MINOR;
    for hair in HAIRS {
        let (cut, cut_holds) = cut_by_the_shaft(hair);
        let (hubbed, hubbed_holds) = hubbed(&cut, cut_holds);
        for (way, part, holds) in [
            ("on the hub", &hubbed, hubbed_holds),
            ("alone", &cut, cut_holds),
        ] {
            let rod_above = shaft_along_z(part, rod, 5.0, 10.0);

            let built = part.union(&rod_above);

            built_sound_holding(
                built,
                holds + PI * rod * rod * 5.0,
                &format!(
                    "the ring cut by a shaft {hair:e} inside its outer equator, a rod of 8 {way}"
                ),
            );
        }
    }
}

/// The criterion's bodies built in other orders, of the same solids: on the
/// hub, the rod joined to the cut ring before the hub, which then bridges
/// them; the rod the part and the cut ring with its hub the tool; and the
/// ring joined with its hub and its rod first, cut by the shaft last. Alone,
/// the rod the part; and the ring joined with the rod first, cut by the
/// shaft last. Each built, exact, holding what the criterion's order holds,
/// its triangles closed and uncrossed.
#[test]
fn a_ring_cut_by_a_shaft_a_hair_inside_its_outer_equator_and_joined_with_a_rod_of_exactly_its_inner_radius_standing_above_it_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume_in_whichever_order_its_solids_are_taken()
 {
    let rod = MAJOR - MINOR;
    let rod_holds = PI * rod * rod * 5.0;
    let joined = |part: &Body, tool: &Body, what: &str| {
        part.union(tool)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"))
    };
    let rod_on = |part: &Body| shaft_along_z(part, rod, 5.0, 10.0);
    for hair in HAIRS {
        let shaft_radius = MAJOR + MINOR - hair;
        let cut_last = |part: &Body| part.difference(&shaft_along_z(part, shaft_radius, -5.0, 0.0));
        let (cut, cut_holds) = cut_by_the_shaft(hair);
        let (hubbed, hubbed_holds) = hubbed(&cut, cut_holds);
        let ways: [(&str, Building, f64); 5] = [
            (
                "the rod joined before the hub",
                Box::new(|| {
                    let rodded = joined(&cut, &rod_on(&cut), "the cut ring and its rod");
                    rodded.union(&shaft_along_z(&rodded, HUB, 1.0, 5.0))
                }),
                hubbed_holds + rod_holds,
            ),
            (
                "the rod the part, on the hub",
                Box::new(|| rod_on(&hubbed).union(&hubbed)),
                hubbed_holds + rod_holds,
            ),
            (
                "the shaft cut last, on the hub",
                Box::new(|| {
                    let ring = ring();
                    let with_hub = joined(
                        &ring,
                        &shaft_along_z(&ring, HUB, 1.0, 5.0),
                        "the ring and its hub",
                    );
                    cut_last(&joined(
                        &with_hub,
                        &rod_on(&with_hub),
                        "the ring, hub and rod",
                    ))
                }),
                hubbed_holds + rod_holds,
            ),
            (
                "the rod the part, alone",
                Box::new(|| rod_on(&cut).union(&cut)),
                cut_holds + rod_holds,
            ),
            (
                "the shaft cut last, alone",
                Box::new(|| {
                    let ring = ring();
                    cut_last(&joined(&ring, &rod_on(&ring), "the ring and its rod"))
                }),
                cut_holds + rod_holds,
            ),
        ];
        for (way, build, volume) in ways {
            let built = build();

            built_sound_holding(
                built,
                volume,
                &format!(
                    "the ring cut by a shaft {hair:e} inside its outer equator, a rod of 8, {way}"
                ),
            );
        }
    }
}

/// The ring cut by the shaft 1e-5 and 1e-4 inside its outer equator, then
/// joined with a rod of radius 8 + 1e-6 from z = 5 to 10, alone: a millionth
/// outside the ring's inner equator, 3 above the tube's top, beside the ring
/// nowhere. Built, exact, holding the cut ring and the rod, its triangles
/// closed and uncrossed — the shaft's rim under the outer equator among
/// them.
#[test]
fn a_ring_cut_by_a_shaft_a_hair_inside_its_outer_equator_then_joined_with_a_rod_a_millionth_outside_its_inner_radius_standing_alone_above_it_is_built_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let rod = MAJOR - MINOR + 1e-6;
    for hair in HAIRS {
        let (cut, cut_holds) = cut_by_the_shaft(hair);
        let rod_above = shaft_along_z(&cut, rod, 5.0, 10.0);

        let built = cut.union(&rod_above);

        built_sound_holding(
            built,
            cut_holds + PI * rod * rod * 5.0,
            &format!(
                "the ring cut by a shaft {hair:e} inside its outer equator, a rod of 8 + 1e-6 alone"
            ),
        );
    }
}
