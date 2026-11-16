//! A sphere or a torus joined to or cut from what shares its axis (#576):
//! a plane square to the axis or holding it, a coaxial shaft, bore, cone,
//! sphere or torus; and a sphere cut or joined by a plane at a slant to its
//! axis, or square to it off its centre, along a circle (#805), or touched
//! by such a plane at a point off its poles (#939). Each body is listed
//! sound and holds the arithmetic's volume, by Pappus, to the bit the cone's
//! booleans hold; everything else declines as unsupported, unless the faces
//! stand clear of each other.
//!
//! Closes #625.
//! - a quarter of a torus and a slanted wall that meet the whole torus's box
//!   but not the quarter's are joined and cut without `Unsupported`: the
//!   union holds both apart, the cut leaves the quarter whole —
//!   `a_quarter_torus_joined_with_a_slanted_wall_past_its_quarter_holds_both_apart`,
//!   `a_slanted_wall_cut_from_a_quarter_torus_past_its_quarter_leaves_the_quarter_whole`

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::turned::tests::round::{ball, bent, ring, turned, zone};
use crate::brep::{Declined, Face};
use crate::soundness::{closed, uncrossed};
use crate::turning::{Corner, Straight};

/// The volume of the ball of radius 5 about the origin, turned about Y.
const BALL: f64 = 500.0 * PI / 3.0;

/// The volume of the ring of tube radius 2 about the circle of radius 6 at
/// `y = 3`.
const RING: f64 = 2.0 * PI * PI * 6.0 * 4.0;

/// A shaft of `radius` about the Y axis, from `from` to `to` along it,
/// `across` from the axis along X.
fn shaft(radius: f64, from: f64, to: f64, across: f64) -> Body {
    let frame = Frame {
        origin: DVec3::Y * from,
        u: DVec3::Z,
        v: DVec3::X,
    };
    let outline = Contour::circle(DVec2::new(0.0, across), radius);
    Body::raised(&outline, &[], frame, DVec3::Y * (to - from)).expect("a shaft raises")
}

/// A profile of straight runs laid in `(h, r)`, turned whole about Y.
fn turned_whole(corners: &[[f64; 2]]) -> Body {
    turned_part(corners, 360.0)
}

/// A profile of straight runs laid in `(h, r)`, turned about Y by `degrees`.
fn turned_part(corners: &[[f64; 2]], degrees: f64) -> Body {
    let count = corners.len();
    let contour = corners
        .iter()
        .enumerate()
        .map(|(run, at)| Corner {
            at: DVec2::from(*at),
            run: run as u32,
            bend: None,
        })
        .collect();
    let last_off_the_axis = (0..count)
        .rev()
        .find(|&run| corners[run][1] != 0.0 || corners[(run + 1) % count][1] != 0.0)
        .map(|run| run as u32);
    let straight = Straight {
        side: -1.0,
        contours: vec![contour],
        runs: count as u32,
        last_off_the_axis,
    };
    turned(&straight, degrees)
}

/// A ball of `radius` about the point of the Y axis at `h`.
fn ball_of(h: f64, radius: f64) -> Body {
    turned(
        &bent(&[&[
            ([h - radius, 0.0], Some(([h, 0.0], -PI))),
            ([h + radius, 0.0], None),
        ]]),
        360.0,
    )
}

/// A ring torus of tube radius `tube` about the circle of radius `big` at
/// `h`.
fn ring_of(h: f64, big: f64, tube: f64) -> Body {
    turned(
        &bent(&[&[
            ([h, big + tube], Some(([h, big], -PI))),
            ([h, big - tube], Some(([h, big], -PI))),
        ]]),
        360.0,
    )
}

/// The faces of a body lying on a sphere or a torus.
fn rounds(body: &Body) -> Vec<&Face> {
    body.face_ids()
        .filter(|&face| {
            matches!(
                body.surface(body.face(face).surface),
                Surface::Sphere(_) | Surface::Torus(_)
            )
        })
        .map(|face| body.face(face))
        .collect()
}

fn assert_sound(body: &Body, what: &str) {
    let listing = body.listing();
    let faces: Vec<String> = listing
        .faces
        .iter()
        .map(|face| {
            format!(
                "{:?}/{:?}/{:?}/{:?}",
                face.surface, face.outward, face.loops, face.poles
            )
        })
        .collect();
    let edges: Vec<String> = listing
        .edges
        .iter()
        .map(|edge| {
            format!(
                "{:?} {}..{} {:?}",
                edge.curve, edge.from, edge.to, edge.ends
            )
        })
        .collect();
    assert_eq!(
        listed(&listing, body.scale().reach()),
        Ok(()),
        "{what}\nFACES {faces:#?}\nEDGES {edges:#?}\nVERTICES {:?}",
        listing.vertices
    );
}

/// The body listed sound and holding `volume` to the bit.
fn held(body: &Body, volume: f64, what: &str) {
    assert_sound(body, what);
    assert!(
        (body.volume() - volume).abs() <= 1e-9 * volume.abs(),
        "{what}: the body holds {}, the arithmetic {volume}",
        body.volume()
    );
}

/// The volume of the matter a disc of the meridian half-plane, about
/// `center` and of `radius`, turns into about the axis on the side of the
/// line through `through` along `along` its normal `normal` points to, by
/// Pappus: the whole disc less the segment beyond the line where the centre
/// stands on that side, the segment alone otherwise.
fn turned_disc_beside(center: DVec2, radius: f64, (through, normal): (DVec2, DVec2)) -> f64 {
    let standing = normal.dot(center - through);
    let distance = standing.abs();
    assert!(distance < radius, "the line cuts the disc");
    let angle = 2.0 * (distance / radius).acos();
    let segment = radius * radius * (angle - angle.sin()) / 2.0;
    let centroid = 4.0 * radius * (angle / 2.0).sin().powi(3) / (3.0 * (angle - angle.sin()));
    let whole = PI * radius * radius;
    let (area, moment) = if standing >= 0.0 {
        (
            whole - segment,
            whole * center.y - segment * (center.y - normal.y * centroid),
        )
    } else {
        (segment, segment * (center.y + normal.y * centroid))
    };
    assert!(area > 0.0);
    TAU * moment
}

/// What a cap of the ball of radius 5 holds above `h`.
fn cap_above(h: f64) -> f64 {
    let height = 5.0 - h;
    PI * height * height * (15.0 - height) / 3.0
}

#[test]
fn a_ball_cut_by_a_plane_square_to_its_axis_keeps_what_lies_below_it_and_pappus_s_volume() {
    for (h, poles) in [(3.0, 1), (0.0, 1), (-2.0, 1)] {
        let slab = block([-10.0, h, -10.0], [10.0, 10.0, 10.0]);
        let cut = turned(&ball(), 360.0)
            .cut_by(&slab)
            .expect("the ball is cut");
        let what = format!("a ball cut above {h}");
        held(&cut, BALL - cap_above(h), &what);
        let [round] = rounds(&cut)[..] else {
            panic!("{what}: one round face");
        };
        assert_eq!(round.poles.len(), poles, "{what}");
        assert!(!round.flipped, "{what}");
        let joined = turned(&ball(), 360.0)
            .joined(&slab)
            .expect("the ball is joined");
        held(
            &joined,
            BALL + 400.0 * (10.0 - h) - cap_above(h),
            &format!("a ball joined to a slab above {h}"),
        );
    }
}

#[test]
fn a_plane_square_at_a_ball_s_pole_touches_it_at_a_point() {
    let eps = Scale::of(10.0).eps();
    for h in [5.0, 5.0 + eps / 2.0, 5.0 - eps / 2.0] {
        let slab = block([-10.0, h, -10.0], [10.0, 10.0, 10.0]);
        let cut = turned(&ball(), 360.0)
            .cut_by(&slab)
            .expect("the ball is cut");
        held(
            &cut,
            BALL,
            &format!("a ball cut by a slab resting on its pole at {h}"),
        );
        let joined = turned(&ball(), 360.0)
            .joined(&slab)
            .expect("the ball is joined");
        held(
            &joined,
            BALL + 400.0 * (10.0 - h),
            &format!("a ball joined to a slab resting on its pole at {h}"),
        );
    }
}

#[test]
fn a_ball_cut_by_a_block_through_its_axis_is_a_half_ball_with_both_poles_as_vertices() {
    let half = block([0.0, -10.0, -10.0], [10.0, 10.0, 10.0]);
    let cut = turned(&ball(), 360.0)
        .cut_by(&half)
        .expect("the ball is cut");
    held(&cut, BALL / 2.0, "a half ball");
    let [round] = rounds(&cut)[..] else {
        panic!("one round face");
    };
    assert!(round.poles.is_empty(), "{:?}", round.poles);
    assert_eq!(round.loops.len(), 1);
    assert_eq!(round.loops[0].len(), 2);
    for pole in [DVec3::Y * 5.0, DVec3::Y * -5.0] {
        assert!(
            cut.vertices
                .iter()
                .any(|vertex| vertex.point.distance(pole) < 1e-12),
            "{pole} among {:?}",
            cut.vertices
        );
    }
    let joined = turned(&ball(), 360.0)
        .joined(&half)
        .expect("the ball is joined");
    held(
        &joined,
        BALL / 2.0 + 4000.0,
        "a half ball joined to a block",
    );
}

#[test]
fn a_coaxial_bore_through_a_ball_leaves_a_napkin_ring_between_two_rims() {
    let bore = shaft(2.0, -10.0, 10.0, 0.0);
    let cut = turned(&ball(), 360.0)
        .cut_by(&bore)
        .expect("the ball is bored");
    held(&cut, 4.0 * PI / 3.0 * 21f64.powf(1.5), "a bored ball");
    let [round] = rounds(&cut)[..] else {
        panic!("one round face");
    };
    assert_eq!(round.loops.len(), 2);
    assert!(round.poles.is_empty());
    let pin = shaft(2.0, -10.0, 10.0, 0.0);
    let joined = turned(&ball(), 360.0)
        .joined(&pin)
        .expect("the ball is pinned");
    held(
        &joined,
        BALL + PI * 4.0 * 20.0 - 4.0 * PI / 3.0 * (125.0 - 21f64.powf(1.5)),
        "a ball on a pin",
    );
}

#[test]
fn a_shaft_of_a_ball_s_radius_joined_at_its_equator_touches_it_along_one_rim() {
    let joined = turned(&ball(), 360.0)
        .joined(&shaft(5.0, 0.0, 15.0, 0.0))
        .expect("the shaft is joined");
    held(
        &joined,
        BALL / 2.0 + PI * 25.0 * 15.0,
        "a rounded shaft end",
    );
    let [round] = rounds(&joined)[..] else {
        panic!("one round face");
    };
    assert_eq!(round.poles.len(), 1);
    let cut = turned(&ball(), 360.0)
        .cut_by(&shaft(5.0, 0.0, 15.0, 0.0))
        .expect("the ball is cut");
    held(&cut, BALL / 2.0, "a half ball left by a shaft");
}

#[test]
fn a_shaft_a_hair_larger_or_smaller_than_the_ball_joined_at_its_equator_is_laid_onto_it() {
    let hair = 3.0 * Scale::of(15.0).eps();
    for radius in [5.0 + hair, 5.0 - hair] {
        let joined = turned(&ball(), 360.0)
            .joined(&shaft(radius, 0.0, 15.0, 0.0))
            .expect("the shaft is joined");
        let what = format!("a shaft of radius {radius} joined to a ball of 5");
        held_and_drawn(&joined, BALL / 2.0 + PI * 25.0 * 15.0, &what);
        let [round] = rounds(&joined)[..] else {
            panic!("{what}: one round face");
        };
        assert_eq!(round.poles.len(), 1, "{what}");
        let cut = turned(&ball(), 360.0)
            .cut_by(&shaft(radius, 0.0, 15.0, 0.0))
            .expect("the ball is cut");
        held_and_drawn(
            &cut,
            BALL / 2.0,
            &format!("a ball cut by a shaft of radius {radius}"),
        );
    }
}

#[test]
fn a_shaft_further_than_the_hair_from_the_ball_s_equator_still_crosses_it() {
    let hair = 25.0 * Scale::of(15.0).eps();
    let larger = turned(&ball(), 360.0)
        .joined(&shaft(5.0 + hair, 0.0, 15.0, 0.0))
        .expect("a shaft a hair larger is joined");
    held(
        &larger,
        BALL / 2.0 + PI * (5.0 + hair) * (5.0 + hair) * 15.0,
        "a shaft a hair larger than its ball",
    );
    let radius = 5.0 - hair;
    let rim = (25.0 - radius * radius).sqrt();
    let smaller = turned(&ball(), 360.0)
        .joined(&shaft(radius, 0.0, 15.0, 0.0))
        .expect("a shaft a hair smaller is joined");
    held(
        &smaller,
        BALL / 2.0
            + PI * (25.0 * rim - rim * rim * rim / 3.0)
            + PI * radius * radius * (15.0 - rim),
        "a shaft a hair smaller than its ball",
    );
    let cut = turned(&ball(), 360.0)
        .cut_by(&shaft(radius, 0.0, 15.0, 0.0))
        .expect("the ball is cut by a shaft a hair smaller");
    held(
        &cut,
        BALL / 2.0 + PI * (25.0 * rim - rim * rim * rim / 3.0) - PI * radius * radius * rim,
        "a ball cut by a shaft a hair smaller",
    );
}

#[test]
fn a_ring_cut_by_a_coaxial_cone_keeps_the_segment_of_its_tube_beyond_the_slope() {
    let cone = turned_whole(&[[-5.0, 0.0], [-5.0, 10.5], [10.0, 3.0], [10.0, 0.0]]);
    let cut = turned(&ring(), 360.0)
        .cut_by(&cone)
        .expect("the ring is cut");
    let normal = DVec2::new(0.5, 1.0).normalize();
    let segment = turned_disc_beside(DVec2::new(3.0, 6.0), 2.0, (DVec2::new(0.0, 8.0), normal));
    held(&cut, segment, "a ring cut by a cone");
    let [round] = rounds(&cut)[..] else {
        panic!("one round face");
    };
    assert_eq!(round.loops.len(), 2);
    let joined = turned(&ring(), 360.0)
        .joined(&cone)
        .expect("the ring is joined");
    let cone_volume = PI * (10.5 * 10.5 + 10.5 * 3.0 + 9.0) * 15.0 / 3.0;
    held(&joined, cone_volume + segment, "a ring joined to a cone");
}

#[test]
fn a_plane_square_through_a_ring_s_tube_cuts_two_rims_and_touching_its_top_one() {
    let cut = turned(&ring(), 360.0)
        .cut_by(&block([-10.0, 4.0, -10.0], [10.0, 10.0, 10.0]))
        .expect("the ring is cut");
    let kept = turned_disc_beside(DVec2::new(3.0, 6.0), 2.0, (DVec2::new(4.0, 0.0), -DVec2::X));
    held(&cut, kept, "a ring cut above its tube's centre");
    let eps = Scale::of(10.0).eps();
    for h in [5.0, 5.0 + eps / 2.0, 5.0 - eps / 2.0] {
        let slab = block([-10.0, h, -10.0], [10.0, 10.0, 10.0]);
        let touched = turned(&ring(), 360.0)
            .cut_by(&slab)
            .expect("the ring is cut");
        held(
            &touched,
            RING,
            &format!("a ring under a slab resting on it at {h}"),
        );
        let joined = turned(&ring(), 360.0)
            .joined(&slab)
            .expect("the ring is joined");
        held(
            &joined,
            RING + 400.0 * (10.0 - h),
            &format!("a ring joined to a slab resting on it at {h}"),
        );
    }
}

#[test]
fn a_bore_of_the_root_s_radius_through_a_grooved_shaft_touches_the_groove_along_its_root() {
    let grooved = shaft(10.0, 0.0, 10.0, 0.0)
        .cut_by(&ring_of(5.0, 12.0, 4.0))
        .expect("the shaft is grooved");
    let removed = turned_disc_beside(
        DVec2::new(5.0, 12.0),
        4.0,
        (DVec2::new(5.0, 10.0), -DVec2::Y),
    );
    held(&grooved, PI * 100.0 * 10.0 - removed, "a grooved shaft");
    let bored = grooved
        .cut_by(&shaft(8.0, -5.0, 15.0, 0.0))
        .expect("the grooved shaft is bored");
    held(
        &bored,
        PI * 100.0 * 10.0 - removed - PI * 64.0 * 10.0,
        "a grooved shaft bored to the groove's root",
    );
}

#[test]
fn two_balls_of_one_centre_and_radius_are_one_surface_and_a_hair_apart_in_radius_two() {
    let eps = Scale::of(5.0).eps();
    for radius in [5.0, 5.0 + eps / 2.0] {
        let joined = turned(&ball(), 360.0)
            .joined(&ball_of(0.0, radius))
            .expect("the balls are joined");
        held(
            &joined,
            BALL,
            &format!("a ball joined to one of radius {radius}"),
        );
        assert_eq!(rounds(&joined).len(), 1);
    }
    let radius = 5.0 + 3.0 * eps;
    let joined = turned(&ball(), 360.0)
        .joined(&ball_of(0.0, radius))
        .expect("the balls are joined");
    held(
        &joined,
        4.0 * PI / 3.0 * radius * radius * radius,
        "a ball joined to one a hair larger",
    );
}

#[test]
fn two_balls_on_one_axis_touching_meet_at_a_point() {
    let eps = Scale::of(11.0).eps();
    for h in [8.0, 8.0 + eps / 2.0] {
        let joined = turned(&ball(), 360.0)
            .joined(&ball_of(h, 3.0))
            .expect("the balls are joined");
        held(
            &joined,
            BALL + 36.0 * PI,
            &format!("two balls touching at {h}"),
        );
        assert_eq!(rounds(&joined).len(), 2);
        assert!(
            joined
                .vertices
                .iter()
                .any(|vertex| vertex.point.distance(DVec3::Y * 5.0) <= eps),
            "{:?}",
            joined.vertices
        );
    }
    let apart = turned(&ball(), 360.0)
        .joined(&ball_of(8.0 + 2.0 * eps, 3.0))
        .expect("the balls are joined");
    held(&apart, BALL + 36.0 * PI, "two balls a hair apart");
}

#[test]
fn a_torus_sharing_the_tube_circle_is_one_surface() {
    let joined = turned(&ring(), 360.0)
        .joined(&ring_of(3.0, 6.0, 2.0))
        .expect("the rings are joined");
    held(&joined, RING, "a ring joined to itself");
    assert_eq!(rounds(&joined).len(), 1);
}

#[test]
fn a_half_ball_cut_out_of_a_block_is_a_flipped_round_face_through_a_pole() {
    let hollowed = block([-10.0, 0.0, -10.0], [10.0, 10.0, 10.0])
        .cut_by(&turned(&ball(), 360.0))
        .expect("the block is hollowed");
    held(
        &hollowed,
        4000.0 - BALL / 2.0,
        "a block with a half ball cut out",
    );
    let [round] = rounds(&hollowed)[..] else {
        panic!("one round face");
    };
    assert!(round.flipped);
    assert_eq!(round.poles.len(), 1);
    let [pole] = round.poles[..] else {
        unreachable!()
    };
    assert!(hollowed.vertex(pole).point.distance(DVec3::Y * 5.0) < 1e-12);
}

/// A slab at a slant to the ball's axis: 10 thick along the normal
/// (1, 1, 0) from the plane `offset` along it, its sides 20 from the
/// normal's line either way, so that it holds all of the ball past the plane.
fn slab_at_a_slant(offset: f64) -> Body {
    let normal = DVec3::new(1.0, 1.0, 0.0).normalize();
    let u = normal.any_orthonormal_vector();
    let frame = Frame {
        origin: normal * offset,
        u,
        v: normal.cross(u),
    };
    Body::raised(
        &Contour::rectangle(DVec2::splat(-20.0), DVec2::splat(20.0)),
        &[],
        frame,
        normal * 10.0,
    )
    .expect("a slab raises")
}

/// The slab at a slant through the ball's centre, which holds the half of
/// the ball the normal points to.
#[test]
fn a_plane_at_a_slant_through_a_ball_cuts_it_in_half_along_a_great_circle() {
    let slab = slab_at_a_slant(0.0);

    let cut = turned(&ball(), 360.0).cut_by(&slab);
    let joined = turned(&ball(), 360.0).joined(&slab);

    let what = "a ball cut by a slab at a slant through its centre";
    let cut = cut.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    held(&cut, BALL / 2.0, what);
    let [round] = rounds(&cut)[..] else {
        panic!("{what}: one round face");
    };
    assert!(!round.flipped, "{what}");
    assert_eq!(
        cut.face_ids().count(),
        2,
        "{what}: the half ball and its disc"
    );
    let what = "a ball joined to a slab at a slant through its centre";
    let joined = joined.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    held(&joined, 16_000.0 + BALL / 2.0, what);
}

/// What is left of the ball of radius 5 behind the slab at a slant through
/// its centre, less the ball of radius 3 about the same centre: half a
/// shell.
const HALF_SHELL: f64 = (BALL - 36.0 * PI) / 2.0;

/// Holds that a body is half the shell between the balls of radius 5 and 3
/// about the origin, behind the plane at a slant through it: sound, holding
/// [`HALF_SHELL`], its rounds the outer sphere and the inner one, flipped,
/// and its one other face the half ring between them on the plane.
fn a_half_shell(body: &Body, what: &str) {
    held(body, HALF_SHELL, what);
    let mut flipped: Vec<bool> = rounds(body).iter().map(|round| round.flipped).collect();
    flipped.sort();
    assert_eq!(
        flipped,
        [false, true],
        "{what}: the outer sphere, and the inner one flipped"
    );
    assert_eq!(
        body.face_ids().count(),
        3,
        "{what}: the two spheres and the half ring between them"
    );
}

/// The great circle the slanted cut leaves on the ball is no latitude and
/// no meridian of the inner ball's sphere, about the same axis: a later step
/// of that axis has to cross it all the same.
#[test]
fn a_ball_cut_at_a_slant_and_hollowed_by_a_coaxial_ball_is_one_half_shell_in_either_order() {
    let inner = ball_of(0.0, 3.0);

    let slanted = turned(&ball(), 360.0)
        .cut_by(&slab_at_a_slant(0.0))
        .expect("the ball is cut at a slant");
    let slanted_first = slanted.cut_by(&inner);
    let hollowed = turned(&ball(), 360.0)
        .cut_by(&inner)
        .expect("the ball is hollowed");
    let hollowed_first = hollowed.cut_by(&slab_at_a_slant(0.0));

    for (what, built) in [
        (
            "a ball cut at a slant, then hollowed by a coaxial ball",
            slanted_first,
        ),
        (
            "a ball hollowed by a coaxial ball, then cut at a slant",
            hollowed_first,
        ),
    ] {
        let body = built.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
        a_half_shell(&body, what);
    }
}

/// A knob of radius 1 about the ball's pole at h = −5, on the side the slab
/// leaves: it adds its own matter less the lens it shares with the ball, the
/// caps of heights 0.1 and 0.9 either side of the plane h = −4.9 the two
/// spheres meet in.
#[test]
fn a_ball_cut_at_a_slant_and_joined_to_a_coaxial_knob_at_its_pole_holds_both_in_either_order() {
    let knob = ball_of(-5.0, 1.0);
    let cap = |radius: f64, height: f64| PI * height * height * (3.0 * radius - height) / 3.0;
    let volume = BALL / 2.0 + 4.0 * PI / 3.0 - cap(5.0, 0.1) - cap(1.0, 0.9);

    let slanted = turned(&ball(), 360.0)
        .cut_by(&slab_at_a_slant(0.0))
        .expect("the ball is cut at a slant");
    let slanted_first = slanted.joined(&knob);
    let knobbed = turned(&ball(), 360.0)
        .joined(&knob)
        .expect("the knob is joined");
    let knobbed_first = knobbed.cut_by(&slab_at_a_slant(0.0));

    for (what, built) in [
        (
            "a ball cut at a slant, then joined to a coaxial knob",
            slanted_first,
        ),
        (
            "a ball joined to a coaxial knob, then cut at a slant",
            knobbed_first,
        ),
    ] {
        let body = built.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
        held(&body, volume, what);
        assert_eq!(rounds(&body).len(), 2, "{what}: the half ball and the knob");
    }
}

/// A slab at a slant whose face runs through the ball's pole at h = 5: its
/// circle passes through a pole of the sphere's chart, where θ has no value,
/// and the cut and the join are built along it all the same (#938), the
/// sphere having no pole but its chart's — the cut the ball less the cap past
/// the plane, the join the slab and that ball, each listed sound, a disc and
/// a piece of sphere about it. The slab half a unit further out cuts the cap
/// past it.
#[test]
fn a_slab_at_a_slant_through_a_pole_of_a_ball_s_chart_cuts_and_joins_it_along_its_circle() {
    let through = 5.0 / 2f64.sqrt();

    let cut = turned(&ball(), 360.0).cut_by(&slab_at_a_slant(through));
    let joined = turned(&ball(), 360.0).joined(&slab_at_a_slant(through));

    for (what, built, volume) in [
        ("cut", cut, BALL - cap_above(through)),
        ("join", joined, 16_000.0 + BALL - cap_above(through)),
    ] {
        let what = format!("the {what} of a ball by a slab at a slant through its pole");
        let body = built.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
        held_and_drawn(&body, volume, &what);
        assert_eq!(rounds(&body).len(), 1, "{what}: one piece of sphere");
    }
    let past = through + 0.5;
    let cut = turned(&ball(), 360.0)
        .cut_by(&slab_at_a_slant(past))
        .expect("a slab half a unit past the pole cuts the ball");
    held(
        &cut,
        BALL - cap_above(past),
        "a ball cut by a slab at a slant half a unit past its pole",
    );
}

/// A slab 10 thick along `towards` from the plane `offset` along it, its
/// sides 20 from the line of `towards` either way: [`SLAB`] of matter,
/// touching the ball at `towards` × 5 where `offset` is 5.
fn slab_towards(towards: DVec3, offset: f64) -> Body {
    let u = towards.any_orthonormal_vector();
    let frame = Frame {
        origin: towards * offset,
        u,
        v: towards.cross(u),
    };
    Body::raised(
        &Contour::rectangle(DVec2::splat(-20.0), DVec2::splat(20.0)),
        &[],
        frame,
        towards * 10.0,
    )
    .expect("a slab raises")
}

/// What [`slab_towards`] holds.
const SLAB: f64 = 40.0 * 40.0 * 10.0;

/// Directions from the ball's centre to points of its sphere off its poles,
/// at y = ±5: two on its equator, one 30° from it towards one pole, one 60°
/// towards the other.
fn off_the_poles() -> [DVec3; 4] {
    let at = |latitude: f64, longitude: f64| {
        let (up, out) = latitude.to_radians().sin_cos();
        let (z, x) = longitude.to_radians().sin_cos();
        DVec3::new(out * x, up, out * z)
    };
    [
        at(0.0, 0.0),
        at(0.0, 225.0),
        at(30.0, 0.0),
        at(-60.0, 110.0),
    ]
}

/// A slab twice the tolerance clear of the ball, at a point off its poles,
/// and the slab within half the tolerance of touching it there: the join
/// holds both, the ball cut by the slab is whole and so is the slab cut by
/// the ball, each listed sound — the bodies the slab clear of it gives, as a
/// slab resting on a pole gives them.
#[test]
fn a_slab_touching_a_ball_off_its_poles_is_joined_and_cut_as_the_slab_a_hair_clear_of_it_is() {
    let ball = turned(&ball(), 360.0);
    for towards in off_the_poles() {
        let eps = ball
            .scale()
            .joined(slab_towards(towards, 5.0).scale())
            .eps();
        for offset in [5.0 + 2.0 * eps, 5.0 + eps / 2.0, 5.0, 5.0 - eps / 2.0] {
            let slab = slab_towards(towards, offset);
            let what = |how: &str| {
                format!("{how}, the slab's face {offset} from the centre towards {towards}")
            };

            let joined = ball.joined(&slab);
            let cut = ball.cut_by(&slab);
            let carved = slab.cut_by(&ball);

            for (result, holds, what) in [
                (joined, BALL + SLAB, what("the ball joined to the slab")),
                (cut, BALL, what("the ball cut by the slab")),
                (carved, SLAB, what("the slab cut by the ball")),
            ] {
                let body =
                    result.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
                held(&body, holds, &what);
            }
        }
    }
}

#[test]
fn a_block_beside_a_ball_knob_is_passed_over_by_clear() {
    let knob = turned(&ball(), 360.0);
    let joined = knob
        .joined(&block([4.0, -1.0, 4.0], [8.0, 1.0, 8.0]))
        .expect("a block beside the knob joins it");
    held(&joined, BALL + 32.0, "a block beside a ball knob");
}

#[test]
fn a_hole_drilled_off_axis_through_a_ball_is_declined_as_unsupported() {
    let hole = shaft(1.0, -10.0, 10.0, 2.0);
    assert_eq!(
        turned(&ball(), 360.0).cut_by(&hole).err(),
        Some(Declined::Unsupported)
    );
}

/// Lane T's campaign found a quarter ball or a ring joined with a body
/// standing apart from it whose rays found nothing, or the wrong sign, while
/// the leaf alone passed every rule (seeds 5576500038, 5576400016).
#[test]
fn a_rounded_leaf_joined_with_a_block_apart_from_it_holds_both_volumes() {
    let far = block([20.0, 20.0, 20.0], [30.0, 30.0, 30.0]);
    for (what, leaf, volume) in [
        ("a quarter ball", turned(&ball(), 90.0), BALL / 4.0),
        ("a half ball", turned(&ball(), 180.0), BALL / 2.0),
        ("a ball", turned(&ball(), 360.0), BALL),
        ("a ring", turned(&ring(), 360.0), RING),
        ("a quarter ring", turned(&ring(), 90.0), RING / 4.0),
    ] {
        let joined = leaf.joined(&far).expect(what);
        held(
            &joined,
            volume + 1000.0,
            &format!("{what} joined with a block apart"),
        );
        let joined = far.joined(&leaf).expect(what);
        held(
            &joined,
            volume + 1000.0,
            &format!("a block joined with {what} apart"),
        );
        let cut = leaf.cut_by(&far).expect(what);
        held(&cut, volume, &format!("{what} cut by a block apart"));
        let cut = far.cut_by(&leaf).expect(what);
        held(&cut, 1000.0, &format!("a block cut by {what} apart"));
    }
}

/// The ring's quarter, turned 90° from +X toward −Z: what its own points span,
/// and what its whole torus does.
const QUARTER: [DVec3; 2] = [DVec3::new(0.0, 1.0, -8.0), DVec3::new(8.0, 5.0, 0.0)];
const WHOLE: [DVec3; 2] = [DVec3::new(-8.0, 1.0, -8.0), DVec3::new(8.0, 5.0, 8.0)];

/// A slab 2 thick across the whole torus's tube where it stands opposite the
/// quarter, about `(−6, 3, 0)`: a rectangle 6 by 12, its normal along
/// `(3, 2, 2)` and its sides along `(2, −3, 0)` and `(6, 4, −13)`, so that no
/// face of it is square to the torus's axis or holds it.
fn slanted_wall_past_the_quarter() -> Body {
    let normal = DVec3::new(3.0, 2.0, 2.0).normalize();
    let u = normal.cross(DVec3::Z).normalize();
    let frame = Frame {
        origin: DVec3::new(-6.0, 3.0, 0.0) - normal,
        u,
        v: normal.cross(u),
    };
    let outline = Contour::rectangle(DVec2::new(-3.0, -6.0), DVec2::new(3.0, 6.0));
    Body::raised(&outline, &[], frame, normal * 2.0).expect("a slab raises")
}

/// What the slab holds.
const WALL: f64 = 6.0 * 12.0 * 2.0;

fn boxes_meet([low, high]: [DVec3; 2], [from, to]: [DVec3; 2], eps: f64) -> bool {
    (low - eps).cmple(to + eps).all() && (from - eps).cmple(high + eps).all()
}

/// The quarter and the slab, held to be a pair the whole torus's box would
/// relate and the kernel could not: every face of the slab at a slant to the
/// torus's axis, its matter crossing the whole torus's tube, its box meeting
/// the whole torus's, and standing clear of the quarter's own grown by the
/// tolerance the boolean grows every box by.
fn quarter_and_wall_past_it() -> (Body, Body) {
    let quarter = turned(&ring(), 90.0);
    let wall = slanted_wall_past_the_quarter();
    for face in wall.face_ids() {
        let Surface::Plane(plane) = wall.surface(wall.face(face).surface) else {
            panic!("a slab is bounded by planes");
        };
        let leaning = plane.normal.dot(DVec3::Y).abs();
        assert!(
            0.1 < leaning && leaning < 0.9,
            "the slab's face of normal {} stands at a slant to the axis",
            plane.normal
        );
    }
    let crossed = DVec3::new(-6.0, 3.0, 0.0);
    let eps = quarter.scale().joined(wall.scale()).eps();
    assert_eq!(
        turned(&ring(), 360.0).winding(crossed, eps),
        Ok(1),
        "{crossed} lies in the whole torus's tube"
    );
    assert_eq!(
        wall.winding(crossed, eps),
        Ok(1),
        "{crossed} lies in the slab"
    );
    let (low, high) = wall.bounds().expect("a slab has a box");
    assert!(
        boxes_meet([low, high], WHOLE, 0.0),
        "the slab's box {low} .. {high} meets the whole torus's"
    );
    assert!(
        !boxes_meet([low, high], QUARTER, eps),
        "the slab's box {low} .. {high} stands clear of the quarter's"
    );
    (quarter, wall)
}

#[test]
fn a_quarter_torus_joined_with_a_slanted_wall_past_its_quarter_holds_both_apart() {
    let (quarter, wall) = quarter_and_wall_past_it();
    let joined = quarter
        .joined(&wall)
        .unwrap_or_else(|declined| panic!("the quarter joined with the wall: {declined:?}"));
    held(
        &joined,
        RING / 4.0 + WALL,
        "the quarter joined with the wall",
    );
    let joined = wall
        .joined(&quarter)
        .unwrap_or_else(|declined| panic!("the wall joined with the quarter: {declined:?}"));
    held(
        &joined,
        RING / 4.0 + WALL,
        "the wall joined with the quarter",
    );
}

#[test]
fn a_slanted_wall_cut_from_a_quarter_torus_past_its_quarter_leaves_the_quarter_whole() {
    let (quarter, wall) = quarter_and_wall_past_it();
    let cut = quarter
        .cut_by(&wall)
        .unwrap_or_else(|declined| panic!("the quarter cut by the wall: {declined:?}"));
    held(&cut, RING / 4.0, "the quarter cut by the wall");
}

/// Lane T's campaign found a plane crossing a ball, or standing a hair inside
/// its crest, that `clear` passed as apart, so that the body's faces crossed
/// (seeds 5576502870, 5576400540): a whole ball's face has no edge to read
/// its reach off. Once #805 meets the two along the circle about the foot of
/// the centre, the cut keeps the ball beside the plane and the join adds the
/// block to it; a hair inside the crest, the cut and the join are built
/// sound or declined, and never a body whose faces cross. Two hairs inside
/// and more, the circle is built however small: two hairs inside, its radius
/// is √(2r × 2 eps), under half a thousandth.
#[test]
fn a_block_whose_side_crosses_a_ball_off_its_axis_cuts_and_joins_it_along_a_circle() {
    let eps = Scale::of(10.0).eps();
    for low in [
        2.0,
        4.9,
        -3.0,
        5.0 - eps,
        5.0 - 2.0 * eps,
        5.0 - 20.0 * eps,
        5.0 - 1e-6,
    ] {
        let slab = block([low, -10.0, -10.0], [10.0, 10.0, 10.0]);

        let cut = turned(&ball(), 360.0).cut_by(&slab);
        let joined = turned(&ball(), 360.0).joined(&slab);

        let kept = BALL - cap_above(low);
        for (what, built, volume) in [
            (format!("a ball cut by a block from x = {low}"), cut, kept),
            (
                format!("a ball joined to a block from x = {low}"),
                joined,
                400.0 * (10.0 - low) + kept,
            ),
        ] {
            match built {
                Ok(body) => held(&body, volume, &what),
                Err(declined) if low == 5.0 - eps => {
                    assert_eq!(declined, Declined::Unsupported, "{what}");
                }
                Err(declined) => panic!("{what}: declined, {declined:?}"),
            }
        }
    }
}

#[test]
fn a_block_in_a_ring_s_hole_is_passed_over_by_clear() {
    let post = block([-2.0, 0.0, -2.0], [2.0, 6.0, 2.0]);
    let joined = turned(&ring(), 360.0)
        .joined(&post)
        .expect("a post in the ring's hole joins it");
    held(&joined, RING + 96.0, "a post in a ring's hole");
    let cut = turned(&ring(), 360.0)
        .cut_by(&post)
        .expect("a post in the ring's hole cuts nothing");
    held(&cut, RING, "a ring cut by a post in its hole");
}

/// Seed 5576600046 of the rounded campaign: a ring turned three quarters of
/// the way, cut by a coaxial shaft running through its tube.
#[test]
fn a_ring_turned_three_quarters_cut_by_a_coaxial_shaft_keeps_pappus_s_volume() {
    let ring = turned(
        &bent(&[&[
            ([5.0, 48.0], Some(([5.0, 28.0], -PI))),
            ([5.0, 8.0], Some(([5.0, 28.0], -PI))),
        ]]),
        270.0,
    );
    let cut = ring
        .cut_by(&shaft(13.0, -15.0, 25.0, 0.0))
        .expect("the ring is cut");
    let kept = turned_disc_beside(
        DVec2::new(5.0, 28.0),
        20.0,
        (DVec2::new(0.0, 13.0), DVec2::Y),
    );
    held(
        &cut,
        kept * 0.75,
        "three quarters of a ring cut by a coaxial shaft",
    );
}

/// Seed 5576600456: a ring whose tube a cylinder of another axis runs
/// through meets it along no curve the kernel builds.
#[test]
fn a_ring_a_cylinder_of_another_axis_runs_through_is_declined_as_unsupported() {
    let ring = ring_of(-3.0, 3.5, 2.5);
    let post = standing([2.0, 0.0], 2.0, 5.0, 11.0);
    assert_eq!(ring.joined(&post).err(), Some(Declined::Unsupported));
    assert_eq!(post.joined(&ring).err(), Some(Declined::Unsupported));
}

/// Seed 5576600742: a ring torus cut from a turned body whose bore slopes,
/// the tube reaching below the body's bottom.
#[test]
fn a_ring_cut_from_a_body_with_a_sloping_bore_is_listed_sound() {
    let body = turned_part(&[[-1.0, 0.5], [-1.0, 6.0], [1.5, 6.0], [1.5, 3.0]], 360.0);
    let cut = body
        .cut_by(&ring_of(1.0, 3.0, 2.4999998))
        .expect("the ring is cut from the body");
    assert_sound(&cut, "a ring cut from a body with a sloping bore");
    assert!(
        cut.volume() < body.volume() && cut.volume() > 0.0,
        "{}",
        cut.volume()
    );
}

/// The volume of the rounded rectangle of `the_exact_kernel_turns`' profile
/// turned whole, by Pappus: the rectangle [2,6]×[0,4] (area 16, centroid at
/// radius 4) plus the half disc of radius 2 centred at radius 6 (area 2π,
/// centroid at radius 6 + 8/(3π)).
const ROUNDED_RECT: f64 = 128.0 * PI + 24.0 * PI * PI + 32.0 * PI / 3.0;

/// A rectangle [2,6]×[0,4] whose side at radius 6 bulges out as a half disc
/// of radius 2, laid as `(h, r)`.
fn rounded_rect() -> Straight {
    bent(&[&[
        ([0.0, 2.0], None),
        ([0.0, 6.0], Some(([2.0, 6.0], -PI))),
        ([4.0, 6.0], None),
        ([4.0, 2.0], None),
    ]])
}

/// A block standing far apart from the ring is joined with it, or cut from
/// it: each body keeps its own matter, and the ring's volume holds by Pappus.
#[test]
fn a_rounded_rectangle_turned_whole_or_a_quarter_joins_and_cuts_a_block_apart() {
    let far = block([20.0, 20.0, 0.0], [30.0, 30.0, 5.0]);
    for (what, ring, volume) in [
        (
            "a rounded rectangle",
            turned(&rounded_rect(), 360.0),
            ROUNDED_RECT,
        ),
        (
            "a quarter of it",
            turned(&rounded_rect(), 90.0),
            ROUNDED_RECT / 4.0,
        ),
    ] {
        held(&ring, volume, &format!("{what} alone"));
        let joined = ring.joined(&far).expect(what);
        held(
            &joined,
            volume + 500.0,
            &format!("{what} joined with a block apart"),
        );
        let joined = far.joined(&ring).expect(what);
        held(
            &joined,
            volume + 500.0,
            &format!("a block joined with {what} apart"),
        );
        let cut = ring.cut_by(&far).expect(what);
        held(&cut, volume, &format!("{what} cut by a block apart"));
        let cut = far.cut_by(&ring).expect(what);
        held(&cut, 500.0, &format!("a block cut by {what} apart"));
    }
}

// Decision 12 (#626): a coaxial surface a hair from touching a round, or
// from a rim on it, is moved onto it with its operand.

/// The body held as `held` holds it, and drawn: its triangles, at the
/// campaigns' fineness, closed and crossing nowhere.
fn held_and_drawn(body: &Body, volume: f64, what: &str) {
    held(body, volume, what);
    let triangles = body.triangles(1e-3 * body.scale().reach());
    assert_eq!(closed(&triangles).map_err(|_| ()), Ok(()), "{what}: open");
    assert_eq!(
        uncrossed(&triangles).map_err(|_| ()),
        Ok(()),
        "{what}: crossed"
    );
}

/// The volume of the outer half of the ring of tube radius 2 about the
/// circle of radius 6, beyond the cylinder of radius 6, by Pappus.
fn outer_half_of_the_ring() -> f64 {
    TAU * (6.0 + 8.0 / (3.0 * PI)) * 2.0 * PI
}

#[test]
fn a_ring_is_grown_onto_a_shaft_a_hair_off_its_inner_equator() {
    let hair = 3.0 * Scale::of(15.0).eps();
    let post = shaft(4.0 + hair, -5.0, 10.0, 0.0);
    let joined = post
        .joined(&ring_of(3.0, 6.0, 2.0))
        .expect("the ring is joined");
    held_and_drawn(
        &joined,
        PI * (4.0 + hair) * (4.0 + hair) * 15.0 + 2.0 * PI * PI * (6.0 + hair) * 4.0,
        "a ring a hair loose on a post, joined",
    );
    let scale = post.scale().joined(ring_of(3.0, 6.0, 2.0).scale());
    let moved = tangent(&post, &ring_of(3.0, 6.0, 2.0), scale).expect("the ring is moved");
    let Some(Surface::Torus(torus)) = moved.surfaces.first() else {
        panic!("a torus: {:?}", moved.surfaces);
    };
    assert!((torus.center.y - torus.radius - (4.0 + hair)).abs() < 1e-12);
}

#[test]
fn half_a_ring_is_grown_onto_a_shaft_a_hair_off_its_inner_equator_with_its_meridian_ends() {
    let hair = 3.0 * Scale::of(15.0).eps();
    let half = || {
        turned(
            &bent(&[&[
                ([3.0, 8.0], Some(([3.0, 6.0], -PI))),
                ([3.0, 4.0], Some(([3.0, 6.0], -PI))),
            ]]),
            180.0,
        )
    };
    let joined = shaft(4.0 + hair, -5.0, 10.0, 0.0)
        .joined(&half())
        .expect("the half ring is joined");
    held_and_drawn(
        &joined,
        PI * (4.0 + hair) * (4.0 + hair) * 15.0 + PI * PI * (6.0 + hair) * 4.0,
        "half a ring a hair loose on a post, joined",
    );
}

#[test]
fn a_disc_a_hair_off_a_ring_s_crests_rests_on_them_with_its_wall_through_them() {
    let hair = 3.0 * Scale::of(15.0).eps();
    for (from, what) in [
        (1.0, "on"),
        (1.0 - hair, "a hair below"),
        (1.0 + hair, "a hair above"),
    ] {
        let disc = || {
            turned_whole(&[
                [from, 0.0],
                [from, 6.0],
                [from + 4.0, 6.0],
                [from + 4.0, 0.0],
            ])
        };
        let joined = ring_of(3.0, 6.0, 2.0)
            .joined(&disc())
            .expect("the disc is joined");
        held_and_drawn(
            &joined,
            PI * 36.0 * 4.0 + outer_half_of_the_ring(),
            &format!("a ring joined to a disc {what} its crests"),
        );
        let cut = ring_of(3.0, 6.0, 2.0)
            .cut_by(&disc())
            .expect("the ring is cut");
        held_and_drawn(
            &cut,
            outer_half_of_the_ring(),
            &format!("a ring cut by a disc {what} its crests"),
        );
    }
}

#[test]
fn a_zone_cut_by_a_bore_of_its_rims_ending_a_hair_off_them_is_a_napkin_ring() {
    let hair = 3.0 * Scale::of(15.0).eps();
    for from in [-3.0, -3.0 + hair, -3.0 - hair] {
        let cut = turned(&zone(), 360.0)
            .cut_by(&shaft(4.0, from, 10.0, 0.0))
            .expect("the zone is bored");
        held_and_drawn(
            &cut,
            132.0 * PI - 96.0 * PI,
            &format!("a zone bored from {from}"),
        );
    }
}

#[test]
fn a_ball_a_hair_inside_a_shaft_is_left_where_it_is() {
    let hair = 3.0 * Scale::of(15.0).eps();
    let post = shaft(5.0 + hair, -10.0, 10.0, 0.0);
    let ball = turned(&ball(), 360.0);
    let scale = post.scale().joined(ball.scale());
    assert!(tangent(&post, &ball, scale).is_none());
}

#[test]
fn a_shaft_further_than_the_hair_from_a_ball_is_left_where_it_is() {
    let hair = 25.0 * Scale::of(15.0).eps();
    let ball = turned(&ball(), 360.0);
    let post = shaft(5.0 - hair, 0.0, 15.0, 0.0);
    let scale = ball.scale().joined(post.scale());
    assert!(tangent(&ball, &post, scale).is_none());
}
