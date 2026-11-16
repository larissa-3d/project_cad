//! A ball cut by a plane at a slant through a pole of its sphere's own chart
//! — where the axis it was turned about pierces it, and θ has no value — is
//! cut along the plane's circle as a plane at a slant anywhere else cuts it
//! (#805): the chart's axis is an artefact of how the ball was made, not of
//! its shape.
//!
//! Closes #938.
//! - A ball turned about z, cut by a plane at a slant through (0, 0, r):
//!   built, the two halves of the right volume, sound —
//!   `a_ball_turned_about_z_cut_at_a_slant_through_its_pole_at_r_leaves_either_side_built_sound_and_of_its_cap_s_volume`
//! - The same through the other pole, and a cut a hair off the pole, giving
//!   one body within tolerance of the pole cut —
//!   `a_ball_turned_about_z_cut_at_a_slant_through_its_pole_at_minus_r_leaves_either_side_built_sound_and_of_its_cap_s_volume`,
//!   `a_ball_cut_at_a_slant_a_hair_off_a_pole_of_its_chart_is_the_body_the_cut_through_the_pole_leaves_within_the_tolerance`
//! - the side holding the other pole has the exact volume, or the step
//!   declines — never a wrong volume (#945's note: a sphere face whose loop
//!   no trace reads is taken as holding no pole, and its volume loses that
//!   pole's cap) —
//!   `the_side_a_cut_at_a_slant_through_one_pole_leaves_holding_the_other_holds_all_of_its_matter_at_any_tilt_about_either_axis`

use super::*;
use crate::brep::Surface;
use crate::soundness::listed;

/// A ball of radius 5 about the origin: a half disc turned whole about
/// `axis`, its sphere's chart laid about that axis, its poles 5 along it
/// either way.
fn ball_about(axis: DVec3) -> Body {
    let sampled: &'static [DVec2] = Box::leak(
        (0..=32)
            .map(|step| DVec2::from_angle(-PI / 2.0 + PI * f64::from(step) / 32.0) * 5.0)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let half = Contour {
        corners: vec![DVec2::new(0.0, -5.0), DVec2::new(0.0, 5.0)],
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
    Body::default()
        .tool_turned(&profile, frame, &about_v(&profile, TAU))
        .expect("the exact kernel turns the ball")
}

/// The tilts, in degrees from the ball's axis, the planes through a pole
/// lean at.
const TILTS: [f64; 3] = [20.0, 45.0, 70.0];

/// Three ways square to `axis` a plane through a pole may lean towards.
fn ways_square_to(axis: DVec3) -> [DVec3; 3] {
    let u = axis.any_orthonormal_vector();
    let w = axis.cross(u);
    let (sin, cos) = 250f64.to_radians().sin_cos();
    [u, w, u * cos + w * sin]
}

/// The normal of a plane through the pole 5 along `pole` of the ball about
/// the origin, leaning `tilt` degrees from `pole` towards `towards`.
fn leaning(pole: DVec3, tilt: f64, towards: DVec3) -> DVec3 {
    let (sin, cos) = tilt.to_radians().sin_cos();
    pole * cos + towards * sin
}

/// One side of a ball of radius 5 about the origin that the plane `offset`
/// along `normal` leaves: what lies past it, the cap about the pole the plane
/// leans from, or what lies behind it, holding the other pole.
#[derive(Clone, Copy, Debug)]
enum Side {
    Cap,
    Rest,
}

const SIDES: [Side; 2] = [Side::Cap, Side::Rest];

impl Side {
    /// The slab whose cut from the ball leaves this side.
    fn slab(self, normal: DVec3, offset: f64) -> Body {
        match self {
            Side::Cap => slab_along(-normal, -offset),
            Side::Rest => slab_along(normal, offset),
        }
    }

    /// What the arithmetic says this side holds: a cap 5 − `offset` high, or
    /// the ball less it.
    fn holds(self, offset: f64) -> f64 {
        match self {
            Side::Cap => cap_of_5(5.0 - offset),
            Side::Rest => BALL_OF_5 - cap_of_5(5.0 - offset),
        }
    }

    /// The area of this side's piece of sphere, 2πr times its height.
    fn curved(self, offset: f64) -> f64 {
        match self {
            Side::Cap => TAU * 5.0 * (5.0 - offset),
            Side::Rest => TAU * 5.0 * (5.0 + offset),
        }
    }
}

/// Holds that a body cut from the ball by one plane is listed sound on its
/// geometry, and is bounded by a disc on the plane and the piece of sphere
/// round it, as a cut by a plane anywhere else leaves it.
fn a_disc_and_a_piece_of_sphere(body: &Body, what: &str) {
    let exact = body.brep().expect("the exact kernel computed the cut");
    assert_eq!(
        listed(&exact.listing(), exact.scale().reach()),
        Ok(()),
        "{what}: listed sound"
    );
    let surfaces: Vec<&Surface> = exact
        .face_ids()
        .map(|face| exact.surface(exact.face(face).surface))
        .collect();
    assert!(
        matches!(
            surfaces[..],
            [Surface::Plane(_), Surface::Sphere(_)] | [Surface::Sphere(_), Surface::Plane(_)]
        ),
        "{what}: a disc and a piece of sphere, where it has {surfaces:?}"
    );
}

/// The ball cut by the plane through its pole 5 along `pole`, leaning each of
/// [`TILTS`] from it towards each of [`ways_square_to`] its axis: either side
/// built, exact, sound and drawn within the drawing's tolerance of what the
/// arithmetic says it holds.
fn either_side_of_each_cut_through(ball: &Body, pole: DVec3, ball_is: &str) {
    for tilt in TILTS {
        for towards in ways_square_to(pole) {
            let normal = leaning(pole, tilt, towards);
            let offset = 5.0 * normal.dot(pole);
            for side in SIDES {
                let what = format!(
                    "{ball_is} cut through its pole at {} by a plane leaning {tilt}° towards \
                     {towards}, keeping its {side:?}",
                    pole * 5.0
                );
                let body = ball
                    .difference(&side.slab(normal, offset))
                    .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
                built_and_drawn(&body, side.holds(offset), side.curved(offset), &what);
                a_disc_and_a_piece_of_sphere(&body, &what);
            }
        }
    }
}

#[test]
fn a_ball_turned_about_z_cut_at_a_slant_through_its_pole_at_r_leaves_either_side_built_sound_and_of_its_cap_s_volume()
 {
    either_side_of_each_cut_through(&ball_about(DVec3::Z), DVec3::Z, "the ball about z");
}

#[test]
fn a_ball_turned_about_z_cut_at_a_slant_through_its_pole_at_minus_r_leaves_either_side_built_sound_and_of_its_cap_s_volume()
 {
    either_side_of_each_cut_through(&ball_about(DVec3::Z), -DVec3::Z, "the ball about z");
}

/// How far off the pole, as a share of the cut's tolerance or outright, the
/// planes a hair from it stand, either way along their normal: half the
/// tolerance, a few, a hair ([`crate::brep::Scale::HAIR`]), and a millionth.
fn hairs(eps: f64) -> [f64; 8] {
    let hair = crate::brep::Scale::HAIR * eps;
    [
        eps / 2.0,
        -eps / 2.0,
        3.0 * eps,
        -3.0 * eps,
        hair,
        -hair,
        1e-6,
        -1e-6,
    ]
}

/// The ball about z cut through either pole by a plane leaning 45° towards X,
/// and 20° towards Y, then by the same plane moved a hair off the pole along
/// its normal, either side kept. Each is built, a disc and a piece of sphere,
/// drawn within the drawing's tolerance of what it holds; the pole's cut holds
/// what the arithmetic says to the bit, and the hair's what it says at the
/// plane's own offset within the disc's area times the cut's tolerance — the
/// furthest two places the kernel reads as one may stand apart — and so no
/// further from the pole's than that and the hair besides.
#[test]
fn a_ball_cut_at_a_slant_a_hair_off_a_pole_of_its_chart_is_the_body_the_cut_through_the_pole_leaves_within_the_tolerance()
 {
    let ball = ball_about(DVec3::Z);
    for (pole, tilt, towards) in [(DVec3::Z, 45.0, DVec3::X), (-DVec3::Z, 20.0, DVec3::Y)] {
        let normal = leaning(pole, tilt, towards);
        let through = 5.0 * normal.dot(pole);
        let disc = PI * (25.0 - through * through);
        for side in SIDES {
            let slab = side.slab(normal, through);
            let eps = slab.brep().expect("an exact slab").scale().eps();
            let at_the_pole = format!(
                "the ball about z cut through its pole at {} leaning {tilt}° towards {towards}, \
                 keeping its {side:?}",
                pole * 5.0
            );
            let cut = ball
                .difference(&slab)
                .unwrap_or_else(|declined| panic!("{at_the_pole}: declined, {declined:?}"));
            built_and_drawn(
                &cut,
                side.holds(through),
                side.curved(through),
                &at_the_pole,
            );
            a_disc_and_a_piece_of_sphere(&cut, &at_the_pole);
            for off in hairs(eps) {
                let offset = through + off;
                let what = format!("{at_the_pole}, moved {off} along its normal");

                let moved = ball
                    .difference(&side.slab(normal, offset))
                    .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

                built_and_drawn(&moved, moved.volume(), side.curved(offset), &what);
                a_disc_and_a_piece_of_sphere(&moved, &what);
                let kept = side.holds(offset);
                let promised = disc * eps + 1e-9 * kept;
                assert!(
                    (moved.volume() - kept).abs() <= promised,
                    "{what}: holds {}, where the arithmetic leaves {kept}, off by more than the \
                     {promised} the tolerance moves",
                    moved.volume()
                );
                let apart = (moved.volume() - cut.volume()).abs();
                let within = disc * (off.abs() + eps) + 2e-9 * kept;
                assert!(
                    apart <= within,
                    "{what}: holds {}, {apart} from the {} the cut through the pole holds, \
                     more than the {within} the hair and the tolerance move",
                    moved.volume(),
                    cut.volume()
                );
            }
        }
    }
}

/// The ball turned about y or about z, cut through either of its poles by a
/// plane leaning from 10° to 80° off its axis, towards each of three ways,
/// keeping the side that holds the other pole within its piece of sphere:
/// the step is built and holds all that side's matter, the other pole's cap
/// with it — never a piece of sphere read as holding no pole because its
/// loop runs through one.
#[test]
fn the_side_a_cut_at_a_slant_through_one_pole_leaves_holding_the_other_holds_all_of_its_matter_at_any_tilt_about_either_axis()
 {
    for (axis, ball_is) in [
        (DVec3::Y, "the ball about y"),
        (DVec3::Z, "the ball about z"),
    ] {
        let ball = ball_about(axis);
        for pole in [axis, -axis] {
            for tilt in [10.0, 30.0, 50.0, 60.0, 80.0] {
                for towards in ways_square_to(axis) {
                    let normal = leaning(pole, tilt, towards);
                    let offset = 5.0 * normal.dot(pole);
                    let what = format!(
                        "{ball_is} cut through its pole at {} by a plane leaning {tilt}° towards \
                         {towards}, keeping the side holding the pole at {}",
                        pole * 5.0,
                        -pole * 5.0
                    );
                    let kept = Side::Rest.holds(offset);

                    let body = ball
                        .difference(&Side::Rest.slab(normal, offset))
                        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

                    assert!(
                        body.is_exact() && (body.volume() - kept).abs() <= 1e-9 * kept,
                        "{what}: holds {}, where the arithmetic leaves {kept}, off by {}",
                        body.volume(),
                        body.volume() - kept
                    );
                }
            }
        }
    }
}
