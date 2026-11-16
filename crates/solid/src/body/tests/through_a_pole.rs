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
//!   `the_side_a_cut_at_a_slant_through_one_pole_leaves_holding_the_other_holds_all_of_its_matter_at_any_tilt_about_either_axis`,
//!   `a_piece_of_sphere_laid_again_on_a_chart_whose_pole_its_loop_runs_through_is_declined_never_short_of_the_other_pole_s_cap`
//!
//! Reading a sphere about another axis is a step's own business: a step
//! whose plane through a pole cuts nothing of the sphere leaves its chart as
//! it was, for the steps after it that turn about that axis, and a face of
//! the tool far from the sphere chooses no axis for it; the sphere as the
//! second operand is read so as well as the first; and where no other axis
//! lays every circle off its poles, or another surface of the part meets the
//! sphere along circles its own axis decides, the step declines as it did
//! before #938.

use super::*;
use crate::brep::Surface;
use crate::soundness::listed;

/// A ball of radius 5 about the origin: a half disc turned whole about
/// `axis`, its sphere's chart laid about that axis, its poles 5 along it
/// either way.
fn ball_about(axis: DVec3) -> Body {
    ball_of(&Body::default(), DVec3::ZERO, axis, 5.0)
}

/// A ball of `radius` about `center`, a tool of `part`: a half disc turned
/// whole about the line through `center` along `axis`, its sphere's chart
/// laid about that line.
fn ball_of(part: &Body, center: DVec3, axis: DVec3, radius: f64) -> Body {
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
    let profile = Profile {
        exact: Some((half, Vec::new())),
        sampled: Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let frame = Frame {
        origin: center,
        u: axis.any_orthonormal_vector(),
        v: axis,
    };
    part.tool_turned(&profile, frame, &about_v(&profile, TAU))
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

/// Holds that a body is listed sound on its geometry.
fn listed_sound(body: &Body, what: &str) {
    let exact = body.brep().expect("the exact kernel computed the step");
    assert_eq!(
        listed(&exact.listing(), exact.scale().reach()),
        Ok(()),
        "{what}: listed sound"
    );
}

/// Holds that a body cut from the ball by one plane is listed sound on its
/// geometry, and is bounded by a disc on the plane and the piece of sphere
/// round it, as a cut by a plane anywhere else leaves it.
fn a_disc_and_a_piece_of_sphere(body: &Body, what: &str) {
    listed_sound(body, what);
    let exact = body.brep().expect("the exact kernel computed the cut");
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

/// How far from the closed form a cut whose circle passes a few tolerances
/// to a millionth off a pole of the sphere's chart may hold, against a ball
/// of radius 5: what the flux of a tilted trace loses as its circle closes on
/// the pole, where `θ` swings round faster the nearer it passes (#947) —
/// measured up to some 5.5e-7 on these cuts — until #958 brings it to the
/// kernel's billionth of the volume. Within the tolerance of the pole the
/// circle is through it, and the sphere read about the circle's axis; from a
/// ten-thousandth off, the tilted trace holds the billionth.
const NEAR_A_POLE: f64 = 1e-6;

/// The planes a hair off the pole along their normal, leaning `tilt` from
/// the axis, `eps` the cut's tolerance, each with the bar its volume is held
/// to against the closed form at its offset: those whose circle passes half
/// a tolerance from the pole, which is through it, and those a
/// ten-thousandth off, to the billionth of what they hold; those a few
/// tolerances, a hair ([`crate::brep::Scale::HAIR`]) and a millionth off, to
/// [`NEAR_A_POLE`].
fn hairs(eps: f64, tilt: f64) -> Vec<(f64, Option<f64>)> {
    let within = eps * tilt.to_radians().sin() / 2.0;
    let hair = crate::brep::Scale::HAIR * eps;
    [within, 1e-4]
        .into_iter()
        .map(|off| (off, None))
        .chain([3.0 * eps, hair, 1e-6].map(|off| (off, Some(NEAR_A_POLE))))
        .flat_map(|(off, bar)| [(off, bar), (-off, bar)])
        .collect()
}

/// The ball about z cut through either pole by a plane leaning 45° towards X,
/// 20° towards Y and 70° towards X, then by the same plane moved a hair off
/// the pole along its normal, either side kept. Each is built, a disc and a
/// piece of sphere, drawn within the drawing's tolerance of what the closed
/// form says it holds. The pole's cut holds that to the bit, and each moved
/// one within its bar of [`hairs`]: no further from the pole's than what the
/// closed form moves between the two, and that bar besides.
#[test]
fn a_ball_cut_at_a_slant_a_hair_off_a_pole_of_its_chart_is_the_body_the_cut_through_the_pole_leaves_within_the_tolerance()
 {
    let ball = ball_about(DVec3::Z);
    for (pole, tilt, towards) in [
        (DVec3::Z, 45.0, DVec3::X),
        (-DVec3::Z, 20.0, DVec3::Y),
        (DVec3::Z, 70.0, DVec3::X),
    ] {
        let normal = leaning(pole, tilt, towards);
        let through = 5.0 * normal.dot(pole);
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
            for (off, bar) in hairs(eps, tilt) {
                let offset = through + off;
                let what = format!("{at_the_pole}, moved {off:e} along its normal");
                let kept = side.holds(offset);

                let moved = ball
                    .difference(&side.slab(normal, offset))
                    .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

                a_disc_and_a_piece_of_sphere(&moved, &what);
                drawn_within(&moved, kept, side.curved(offset), &what);
                let bar = bar.unwrap_or(1e-9 * kept);
                assert!(
                    moved.is_exact() && (moved.volume() - kept).abs() <= bar,
                    "{what}: holds {}, where the arithmetic leaves {kept}, off by {:e}, more \
                     than the {bar:e} it is held to",
                    moved.volume(),
                    moved.volume() - kept
                );
                let apart = (moved.volume() - cut.volume()).abs();
                let within = (kept - side.holds(through)).abs() + bar + 1e-9 * side.holds(through);
                assert!(
                    apart <= within,
                    "{what}: holds {}, {apart:e} from the {} the cut through the pole holds, \
                     more than the {within:e} the hair and the bar move",
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

/// The wedge standing beside the top of the ball of radius 5 about z, a tool
/// of `part`: a prism over the triangle (x, z) = (1, 4), (6, 6), (5, 0), from
/// y = 4 to 6, its slanted face on x + z = 5 — the plane through the ball's
/// top, (0, 0, 5), leaning 45° from z towards x — and its other two faces
/// through no pole of the ball. From y = 4 on the ball reaches no further
/// than 3 from its axis, and x + z = 5 stands 5/√2 from it: the wedge stands
/// clear of the ball, inside its box. It holds 28.
fn wedge_beside_the_top(part: &Body) -> Body {
    part.tool_raised(
        &exact(
            Contour::straight(vec![
                DVec2::new(4.0, 1.0),
                DVec2::new(6.0, 6.0),
                DVec2::new(0.0, 5.0),
            ]),
            vec![],
        ),
        Frame {
            origin: DVec3::Y * 4.0,
            u: DVec3::Z,
            v: DVec3::X,
        },
        DVec3::Y * 2.0,
    )
    .expect("the wedge raises")
}

/// A bore of radius 1 along z through the origin, from z = −10 to 10, a tool
/// of `part`.
fn bore_along_z(part: &Body) -> Body {
    part.tool_raised(
        &exact(disc(DVec2::ZERO, 1.0), vec![]),
        Frame {
            origin: DVec3::Z * -10.0,
            ..FLAT
        },
        DVec3::Z * 20.0,
    )
    .expect("the bore raises")
}

/// The axes of the spheres a body's faces lie on.
fn sphere_axes(body: &Body) -> Vec<DVec3> {
    let exact = body.brep().expect("the exact kernel computed the step");
    exact
        .face_ids()
        .filter_map(|face| match exact.surface(exact.face(face).surface) {
            Surface::Sphere(sphere) => Some(sphere.axis),
            _ => None,
        })
        .collect()
}

/// The ball about z, joined to a wedge beside its top whose slanted face lies
/// on a plane through its pole and stands clear of it, then bored along z
/// through its centre: three steps, each built, sound and holding the
/// arithmetic's matter — the ball, the ball and the wedge, then less what a
/// bore of radius 1 takes from a ball of 5, 4π/3 (5³ − 24^{3/2}). The wedge
/// cuts nothing of the ball, and leaves the ball's sphere read about z, the
/// axis the bore turns about.
#[test]
fn a_wedge_beside_a_ball_s_top_on_a_plane_through_its_pole_leaves_its_chart_as_it_was_and_a_bore_along_its_axis_still_goes_through()
 {
    let ball = ball_about(DVec3::Z);
    built_and_drawn(&ball, BALL_OF_5, 100.0 * PI, "the ball about z");

    let what = "the ball about z joined to a wedge beside its top";
    let joined = ball
        .union(&wedge_beside_the_top(&ball))
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    built_and_drawn(&joined, BALL_OF_5 + 28.0, 100.0 * PI, what);
    listed_sound(&joined, what);
    let axes = sphere_axes(&joined);
    assert!(
        matches!(axes[..], [axis] if axis.cross(DVec3::Z).length() <= 1e-12),
        "{what}: its sphere read about z as the ball's was, where it is read about {axes:?}"
    );

    let what = "the ball about z joined to a wedge beside its top, then bored along z";
    let bored = joined
        .difference(&bore_along_z(&joined))
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    let root = 24f64.sqrt();
    let taken = 4.0 * PI / 3.0 * (125.0 - 24.0 * root);
    let curved = 100.0 * PI - 2.0 * TAU * 5.0 * (5.0 - root) + TAU * 2.0 * root;
    built_and_drawn(&bored, BALL_OF_5 + 28.0 - taken, curved, what);
    listed_sound(&bored, what);
}

/// The ball about z as the second operand of a slab at a slant whose face
/// runs through a pole of its chart, from either pole at 20° and 70° towards
/// X: joined to it, the slab and the ball less the cap past the face; cut
/// from it, a pocket the shape of that cap in the slanted face. Each built,
/// sound, drawn within the drawing's tolerance and holding the arithmetic's
/// matter.
#[test]
fn a_ball_after_a_slab_at_a_slant_through_its_pole_is_joined_to_it_or_cut_from_it_as_a_pocket_in_its_face()
 {
    for pole in [DVec3::Z, -DVec3::Z] {
        for tilt in [20.0, 70.0] {
            let normal = leaning(pole, tilt, DVec3::X);
            let offset = 5.0 * normal.dot(pole);
            let slab = Side::Rest.slab(normal, offset);
            let ball = ball_of(&slab, DVec3::ZERO, DVec3::Z, 5.0);
            let cap = cap_of_5(5.0 - offset);
            for (way, built, kept, curved) in [
                (
                    "joined to",
                    slab.union(&ball),
                    16_000.0 + BALL_OF_5 - cap,
                    Side::Rest.curved(offset),
                ),
                (
                    "cut from",
                    slab.difference(&ball),
                    16_000.0 - cap,
                    Side::Cap.curved(offset),
                ),
            ] {
                let what = format!(
                    "the ball about z {way} a slab whose face runs through its pole at {}, \
                     leaning {tilt}° towards X",
                    pole * 5.0
                );
                let body =
                    built.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
                built_and_drawn(&body, kept, curved, &what);
                listed_sound(&body, &what);
            }
        }
    }
}

/// The ball turned about the normal of a plane through (0, 0, 5) leaning
/// 45° from z towards X, cut by that plane, keeping the side holding
/// (0, 0, −5): its circle a latitude of its chart. Then its sphere replaced
/// by `surface`, holding the same points, and the body laid again on it.
fn the_cut_laid_again_on(surface: Surface) -> Result<crate::brep::Body, Declined> {
    let normal = leaning(DVec3::Z, 45.0, DVec3::X);
    let cut = ball_about(normal)
        .difference(&Side::Rest.slab(normal, 5.0 * normal.dot(DVec3::Z)))
        .expect("a plane square to the ball's axis cuts it");
    let mut laid = cut.brep().expect("an exact cut").clone();
    let spheres: Vec<usize> = (laid.surfaces.iter().enumerate())
        .filter(|(_, surface)| matches!(surface, Surface::Sphere(_)))
        .map(|(rank, _)| rank)
        .collect();
    let [sphere] = spheres[..] else {
        panic!("the cut lies on one sphere: {spheres:?}");
    };
    laid.surfaces[sphere] = surface;
    laid.rejoined_at(&[])
}

/// The cut of [`the_cut_laid_again_on`], its sphere read about z instead, as
/// a ball turned about z is: its piece of sphere's loop runs through a pole
/// of that chart, where no trace reads it, and the poles a face holds are
/// read off its loops. The body is declined as the trace declines, never
/// handed back holding no pole, short of the cap of the one at (0, 0, −5).
#[test]
fn a_piece_of_sphere_laid_again_on_a_chart_whose_pole_its_loop_runs_through_is_declined_never_short_of_the_other_pole_s_cap()
 {
    let about_z = *ball_about(DVec3::Z)
        .brep()
        .expect("an exact ball")
        .surfaces
        .iter()
        .find(|surface| matches!(surface, Surface::Sphere(_)))
        .expect("the ball lies on a sphere");

    let laid = the_cut_laid_again_on(about_z);

    assert_eq!(
        laid.map(|body| body.volume()),
        Err(Declined::Unsupported),
        "the cut laid again on its sphere read about z, where the side kept holds {}",
        BALL_OF_5 - cap_of_5(5.0 - 5.0 / 2f64.sqrt())
    );
}

/// The cut of [`the_cut_laid_again_on`], its sphere read as the torus about z
/// whose ring has no radius — the same points, its poles the sphere's at
/// (0, 0, ±5): its loop is no latitude and no meridian of the torus, which no
/// trace reads, and the body is declined as the trace declines.
#[test]
fn a_piece_of_sphere_laid_again_on_a_torus_of_no_ring_through_whose_pole_its_loop_runs_is_declined()
{
    let ringless = crate::brep::Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::ZERO,
        5.0,
        DVec2::new(0.0, 5.0),
    );

    let laid = the_cut_laid_again_on(Surface::Torus(ringless));

    assert_eq!(
        laid.map(|body| body.volume()),
        Err(Declined::Unsupported),
        "the cut laid again on the torus about z of no ring, where the side kept holds {}",
        BALL_OF_5 - cap_of_5(5.0 - 5.0 / 2f64.sqrt())
    );
}

/// A ball about z joined to a coaxial knob of radius 1 at its pole at
/// (0, 0, −5), then cut through its pole at (0, 0, 5) by a plane leaning 45°
/// towards X, either side kept: the knob's sphere meets the ball's along
/// circles its axis decides, the ball's sphere is not read about another
/// axis, and the cut is declined as it was before #938.
#[test]
fn a_ball_with_a_coaxial_knob_cut_at_a_slant_through_its_other_pole_is_declined_as_unsupported() {
    let ball = ball_about(DVec3::Z);
    let knobbed = ball
        .union(&ball_of(&ball, DVec3::Z * -5.0, DVec3::Z, 1.0))
        .expect("a coaxial knob joins the ball");
    let normal = leaning(DVec3::Z, 45.0, DVec3::X);
    let offset = 5.0 * normal.dot(DVec3::Z);
    for side in SIDES {
        let declined = knobbed.difference(&side.slab(normal, offset)).err();

        assert_eq!(
            declined,
            Some(Declined::Unsupported),
            "a ball with a coaxial knob at one pole, cut at a slant through the other, keeping \
             its {side:?}"
        );
    }
}

/// A prism along y over the triangle (x, z) = (2.5, 2.5), (20, 20),
/// (20, −15), a tool of `part`: its faces on x + z = 5, through the ball's
/// pole at (0, 0, 5) at 45° from z, and on x = z, through the centre and
/// through the poles (±5/√2, 0, ±5/√2) of the chart read about the first
/// plane's circle; its third face and its ends clear of the ball.
fn prism_through_a_pole_and_the_poles_of_its_circle(part: &Body) -> Body {
    part.tool_raised(
        &exact(
            Contour::straight(vec![
                DVec2::new(2.5, 2.5),
                DVec2::new(20.0, 20.0),
                DVec2::new(-15.0, 20.0),
            ]),
            vec![],
        ),
        Frame {
            origin: DVec3::Y * -20.0,
            u: DVec3::Z,
            v: DVec3::X,
        },
        DVec3::Y * 40.0,
    )
    .expect("the prism raises")
}

/// The ball about z cut by a prism one face of which runs through its pole
/// at (0, 0, 5), at a slant, and another through the poles of the chart
/// read about the first face's circle: no axis lays both circles off its
/// poles, and the cut is declined as it was before #938.
#[test]
fn a_ball_cut_by_a_tool_whose_second_face_runs_through_the_poles_its_first_face_s_circle_lays_is_declined_as_unsupported()
 {
    let ball = ball_about(DVec3::Z);

    let declined = ball
        .difference(&prism_through_a_pole_and_the_poles_of_its_circle(&ball))
        .err();

    assert_eq!(
        declined,
        Some(Declined::Unsupported),
        "a ball cut through its pole by one face of a prism, and through the poles that face's \
         circle lays by another"
    );
}

/// The normal of a plane through the pole at (0, 0, 5) of the ball of
/// radius 5 about z, leaning 60° from z: (1/2, 1/√2, 1/2), 5 times which is a
/// point of the circle x + z = 5 leaves on the ball.
fn towards_a_point_of_the_slab_s_circle() -> DVec3 {
    DVec3::new(0.5, 0.5f64.sqrt(), 0.5)
}

/// A block 40 by 10 and 10 thick far below the ball of radius 5 about z, a
/// tool of `part`: its face on the plane through the ball's pole at
/// (0, 0, 5) square to [`towards_a_point_of_the_slab_s_circle`], 36 down that
/// plane from the foot of the ball's centre on it, below z = −25, where its
/// box reaches no corner of the ball's box but the one it stands under; the
/// block raised away from the ball, its other faces clear of it.
fn block_far_on_a_plane_through_the_pole(part: &Body) -> Body {
    let normal = towards_a_point_of_the_slab_s_circle();
    let across = normal.cross(DVec3::Z).normalize();
    let down = normal.cross(across);
    part.tool_raised(
        &exact(
            Contour::rectangle(DVec2::new(-20.0, -5.0), DVec2::new(20.0, 5.0)),
            vec![],
        ),
        Frame {
            origin: normal * 2.5 + down * 36.0,
            u: across,
            v: down,
        },
        normal * 10.0,
    )
    .expect("the block raises")
}

/// The ball about z cut by a tool of two pieces, in either order: a slab
/// whose face runs through the pole at (0, 0, 5) on x + z = 5, and a block far
/// from the ball whose face lies on a plane through that pole too, leaning
/// 60° from z. Only the slab's face meets the ball: the sphere is read about
/// its circle's axis, and the step is built, sound and holding the ball less
/// the cap past the slab's face — never read about the far face's circle,
/// whose poles the slab's circle runs through, and declined so.
#[test]
fn a_ball_cut_through_a_pole_by_a_tool_with_another_face_far_off_on_a_plane_through_that_pole_is_read_about_the_circle_the_cut_leaves()
 {
    let ball = ball_about(DVec3::Z);
    let normal = leaning(DVec3::Z, 45.0, DVec3::X);
    let offset = 5.0 * normal.dot(DVec3::Z);
    let slab = Side::Rest.slab(normal, offset);
    let block = block_far_on_a_plane_through_the_pole(&slab);
    for (order, tool) in [
        ("the slab, then the block", slab.union(&block)),
        ("the block, then the slab", block.union(&slab)),
    ] {
        let what = format!(
            "the ball about z cut through its pole by a slab and a block far off on another \
             plane through it, joined as {order}"
        );
        let tool =
            tool.unwrap_or_else(|declined| panic!("{what}: the tool declined, {declined:?}"));

        let body = ball
            .difference(&tool)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        built_and_drawn(
            &body,
            Side::Rest.holds(offset),
            Side::Rest.curved(offset),
            &what,
        );
        a_disc_and_a_piece_of_sphere(&body, &what);
    }
}
