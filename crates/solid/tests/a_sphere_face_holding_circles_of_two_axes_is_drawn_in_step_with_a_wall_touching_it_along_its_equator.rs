//! A sphere face that holds circles about two axes is refused by the central,
//! `about` and `charted` layouts and drawn by `tessellation/outline/pierced.rs`.
//! A wall coaxial with the sphere's chart that touches it along its equator
//! is drawn by its chords there; the sphere face's triangles beside the
//! touched equator must stand inside those chords, as
//! `tessellation/contact.rs` keeps two cylinders' in step, or the two faces'
//! triangles cross.
//!
//! Closes #1112.
//! - The plane-section case above, both orders: built, sound, drawn
//!   uncrossed —
//!   `a_ball_zone_turned_about_y_cut_by_the_plane_x_6_and_joined_with_a_ring_whose_bore_touches_its_equator_either_way_is_built_sound_and_drawn_uncrossed`.
//!   The case is the issue's: the ball zone turned about y, cut by the plane
//!   x = 6 (a circle about x), then joined with `ring([5, 1], 2, 1.5)` whose
//!   inner wall touches the zone's equator. Read as: the zone of the ball of
//!   radius 1.5 about (5, 4, 1) between y = 2.8 and y = 5.2, turned about
//!   the line x = 5, z = 1, the cap beyond x = 6 cut away, and the ring
//!   raised from y = 2.5 to 11.5 about that line, its bore of the ball's
//!   radius; the cut zone joined with the ring, the ring joined with the cut
//!   zone, and the cut zone cut by the ring. Each step built, never
//!   declined, on the exact kernel and through the application's body; the
//!   cut zone joined with the ring and cut by it held to every rule of the
//!   campaigns, the arithmetic's matter along every line among them; the
//!   ring joined with the cut zone holding the arithmetic's volume to a
//!   billionth, its triangles closed and uncrossed
//! - Seeds 956010647 and 956011588 of
//!   `a_campaign_of_rounded_solids_on_the_exact_kernel_keeps_every_rule`
//!   hold, on #956's branch or after it lands —
//!   `seed_956010647_a_zone_joined_with_a_coaxial_tube_touching_its_equator_then_with_a_block_across_both_keeps_every_rule`,
//!   `seed_956011588_a_ball_slotted_across_and_joined_with_a_ring_whose_bore_touches_its_equator_keeps_every_rule`.
//!   #956 has landed; each seed is held on the exact kernel as the campaign
//!   draws it and as the triage shrinks it. Both are the issue's family
//!   rather than its plane section: a sphere face bounded by circles of two
//!   axes — a bore or a block across the ball, a block beside the zone —
//!   beside a coaxial wall touching it along its equator
//! - #962's near-pole drawing tests keep their bodies — no test: they are
//!   `a_ball_cut_by_two_flats_whose_circle_passes_two_degrees_by_a_pole_of_its_chart_is_declined_as_undrawn`,
//!   in `src/body/tests.rs`, and
//!   `a_ball_turned_about_x_joined_with_or_cut_by_a_shaft_along_z_through_its_centre_is_declined_where_the_rims_pass_near_its_chart_s_poles_and_built_further_off`,
//!   in `spheres_of_one_centre_and_radius_are_one_surface_whatever_their_axes.rs`;
//!   running them, unedited, is the answer

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use std::f64::consts::PI;

use cao_solid::soundness::{closed, uncrossed};
use random_solids::{
    Application, Axis, Case, Exact, Kernel, Leaf, Mode, Outline, Plane, Section, Step,
};

/// How far the matter a body holds may stand from what the arithmetic
/// leaves, as a share of it: the kernel's billionth.
const EXACT: f64 = 1e-9;

/// The radius of the ball the zone is cut from, about (5, 4, 1).
const RADIUS: f64 = 1.5;

/// How far either side of the ball's centre, along y, the zone reaches: a
/// lattice point of its circle, 0.9 from the axis there.
const HALF: f64 = 1.2;

/// The zone of the ball of [`RADIUS`] about (5, 4, 1) between y = 2.8 and
/// y = 5.2, turned whole about the line x = 5, z = 1, along y.
fn zone() -> Leaf {
    Leaf::turned(
        Plane::xy(1.0),
        Axis::second(5.0),
        Section::bands(4.0 - HALF, &[[2.0 * HALF, 0.0, 0.9]])
            .bowed(&[[[0.0, 0.0, 0.0], [4.0, 0.0, RADIUS]]]),
        360.0,
    )
}

/// A block standing on the plane x = 6, from there to x = 8, over all of
/// the zone past it: cut from the zone, it leaves the zone this side of the
/// plane, a disc on the plane where the cap was.
fn beyond_the_plane() -> Leaf {
    Leaf::prism(
        Plane::yz(6.0),
        Outline::rectangle([2.0, -1.0], [6.0, 3.0]),
        2.0,
    )
}

/// The tube about the zone's axis from y = 2.5 to 11.5, 2 across outside
/// and of the ball's radius inside: its bore touches the zone along the
/// ball's equator, y = 4.
fn ring() -> Leaf {
    Leaf::prism(Plane::xz(2.5), Outline::ring([5.0, 1.0], 2.0, RADIUS), -9.0)
}

/// What the zone holds once the cap beyond x = 6 is cut away: the cap,
/// 0.5 high, reaches at most √1.25 from the ball's centre along y, inside
/// the zone.
fn cut_zone() -> f64 {
    let zone = PI * (RADIUS * RADIUS * 2.0 * HALF - 2.0 * HALF.powi(3) / 3.0);
    let height = RADIUS - 1.0;
    let cap = PI * height * height * (3.0 * RADIUS - height) / 3.0;
    zone - cap
}

/// What the ring holds; it shares no matter with the zone, only the
/// equator.
fn tube() -> f64 {
    PI * (2.0 * 2.0 - RADIUS * RADIUS) * 9.0
}

/// The ring joined with the zone the plane x = 6 cut, on `kernel`: the
/// ring the body, the cut zone the tool.
fn the_ring_joined_with_the_cut_zone<K: Kernel>(kernel: &K, what: &str) -> K::Body {
    let raised = |leaf: &Leaf| {
        kernel
            .raised(leaf)
            .unwrap_or_else(|declined| panic!("{what}: a leaf declined, {declined:?}"))
    };
    let cut = kernel
        .combined(&raised(&zone()), &raised(&beyond_the_plane()), Mode::Cut)
        .unwrap_or_else(|declined| panic!("{what}: the plane's cut declined, {declined:?}"));
    kernel
        .combined(&raised(&ring()), &cut, Mode::Add)
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"))
}

/// Holds that `body` holds `volume`, the arithmetic's `kept` to a
/// billionth, and that `kernel` draws it closed and uncrossed.
fn holds_and_is_drawn_uncrossed<K: Kernel>(
    kernel: &K,
    body: &K::Body,
    volume: f64,
    kept: f64,
    what: &str,
) {
    assert!(
        (volume - kept).abs() <= EXACT * kept,
        "{what}: holds {volume}, where the arithmetic leaves {kept}"
    );
    let (triangles, _) = kernel.triangles(body);
    assert_eq!(closed(&triangles), Ok(()), "{what}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
}

/// The issue's plane section of a ball zone turned about y, beside a ring
/// whose bore touches its equator: the sphere face is bounded by the circle
/// the plane x = 6 cuts, about x, and by the zone's rims and the touched
/// equator, about y, so `pierced.rs` draws it, and its triangles beside the
/// equator stood outside the ring's chords there. Joined with the ring,
/// either one first, and cut by it, each is built and drawn uncrossed, on
/// the exact kernel and through the application's body.
#[test]
fn a_ball_zone_turned_about_y_cut_by_the_plane_x_6_and_joined_with_a_ring_whose_bore_touches_its_equator_either_way_is_built_sound_and_drawn_uncrossed()
 {
    for case in [
        Case::new(
            zone(),
            vec![Step::cut(beyond_the_plane()), Step::add(ring())],
        ),
        Case::new(
            zone(),
            vec![Step::cut(beyond_the_plane()), Step::cut(ring())],
        ),
    ] {
        random_solids::holds_exactly(&case);
        random_solids::holds_through_the_application(&case);
    }

    let what = "the ring joined with the zone cut by the plane x = 6, on the exact kernel";
    let exact = the_ring_joined_with_the_cut_zone(&Exact, what);
    holds_and_is_drawn_uncrossed(&Exact, &exact, exact.volume(), cut_zone() + tube(), what);
    let what = "the ring joined with the zone cut by the plane x = 6, through the application";
    let application = the_ring_joined_with_the_cut_zone(&Application, what);
    holds_and_is_drawn_uncrossed(
        &Application,
        &application,
        application.volume(),
        cut_zone() + tube(),
        what,
    );
}

/// Seed 956010647, rounded, on the exact kernel. As drawn: a zone of a ball
/// of 12.5 turned about z, a coaxial tube turned three quarters whose bore
/// touches its equator, a boss along z and a shaft along x through the
/// ball's centre, then the zone's room cut. Shrunk: the zone, the tube
/// turned whole, then a block joined across the zone's side and the tube,
/// whose sides x = −5 and y = −5 cut the sphere along circles about x and
/// y. Either way the sphere's triangles beside the equator crossed the
/// tube's bore.
#[test]
fn seed_956010647_a_zone_joined_with_a_coaxial_tube_touching_its_equator_then_with_a_block_across_both_keeps_every_rule()
 {
    random_solids::holds_exactly(&Case::drawn_rounded_off_the_lattice(956010647));
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xz(-10.0),
            Axis::second(0.0),
            Section::bands(-10.0, &[[20.0, 0.0, 7.5]])
                .bowed(&[[[0.0, 0.0, 0.0], [0.0, 0.0, 12.5]]]),
            360.0,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::yz(0.0),
                Axis::second(-10.0),
                Section::bands(-10.0, &[[20.0, -22.5, -12.5]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-33.0),
                Outline::rectangle([-15.0, -5.0], [-5.0, 5.0]),
                65.0,
            )),
        ],
    ));
}

/// Seed 956011588, rounded, on the exact kernel: a ball of 1.5 turned about
/// y, bored along x through its centre as drawn, slotted across by a block
/// whose sides y = 3.5 and 4.5 cut it along circles about y and whose floor
/// and ceiling z = 0.5 and 1.5 along circles about z as shrunk, then joined
/// with a ring about its axis whose bore touches its equator. The sphere's
/// triangles beside the equator crossed the ring's bore.
#[test]
fn seed_956011588_a_ball_slotted_across_and_joined_with_a_ring_whose_bore_touches_its_equator_keeps_every_rule()
 {
    random_solids::holds_exactly(&Case::drawn_rounded_off_the_lattice(956011588));
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(1.0),
            Axis::second(5.0),
            Section::bands(2.5, &[[3.0, 0.0, 0.0]]).bowed(&[[[0.0, 0.0, 0.0], [4.0, 0.0, 1.5]]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(1.5),
                Outline::rectangle([3.5, 0.5], [4.5, 1.5]),
                7.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(2.5),
                Outline::ring([5.0, 1.0], 2.0, 1.5),
                -9.0,
            )),
        ],
    ));
}
