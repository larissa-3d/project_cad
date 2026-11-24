//! Two spheres whose centres and radii agree within the tolerance are one
//! surface, whatever axis each one's chart was laid about. A sphere has no
//! axis of its own: the chart's is an artefact of how the ball was made, or
//! of a step that read it about another (#938's re-reading of a sphere about
//! the circle a plane at a slant cuts through a pole of its chart). A boolean
//! between two such spheres, or between one of them and a surface coaxial
//! with either chart, is built as if they shared one chart.
//!
//! Closes #956.
//! - A ball turned about z, cut at a slant through its pole (#938), then
//!   bored along z through its centre: built, sound, the volume the
//!   arithmetic gives —
//!   `a_ball_turned_about_z_cut_at_a_slant_through_its_pole_then_bored_along_z_through_its_centre_is_built_sound_and_holds_the_arithmetic_s_volume`.
//!   Read as #938 reads "built, sound": the step not declined, the body
//!   exact and holding the arithmetic's matter to a billionth, its triangles
//!   closed, uncrossed and enclosing that matter within the drawing's
//!   tolerance over its curved faces. Answered at either pole, at three
//!   tilts towards three ways, keeping either side of the plane: by the
//!   ball's and the bore's symmetry about z, and their mirror across z = 0,
//!   the arithmetic is one for every way and either pole
//! - A ball turned about x joined with, and cut by, a ball of the same centre
//!   and radius turned about y: the same body as for two balls turned about
//!   one axis —
//!   `a_ball_turned_about_x_joined_with_or_cut_by_a_ball_of_its_centre_and_radius_turned_about_y_is_the_body_two_balls_turned_about_one_axis_leave`.
//!   Read as: joined either way and each cut by the other, every step built,
//!   never declined, and its body the one the same step leaves between the
//!   same two balls turned about one axis — exact, of the same volume and
//!   the arithmetic's, empty where that one is, with as many faces of each
//!   kind, flat or on the sphere, its triangles closed and uncrossed.
//!   Answered for balls turned whole, which leave the ball or nothing, and
//!   for balls turned half a turn, which leave three quarters of the ball and
//!   a quarter: a ball turned about x is one turned part of the way too, and
//!   only there is the body more than the ball or nothing. Under it, the
//!   relation: two spheres of one centre and radius are one surface whatever
//!   axes they were turned about, and a hair apart in radius are not —
//!   `two_balls_of_one_centre_and_radius_are_one_surface_whatever_axes_they_were_turned_about_and_a_hair_apart_in_radius_are_not`,
//!   in `src/brep/relation/tests/round.rs`, which takes that pair out of the
//!   declines `every_other_pair_with_a_round_is_unsupported_and_none_is_at_a_slant`
//!   pinned; and a ring meets a sphere whose centre its axis holds, turned
//!   about another axis, as it meets that sphere turned about its own —
//!   `a_ball_whose_centre_a_ring_s_axis_holds_meets_it_whatever_axis_it_was_turned_about_as_it_does_turned_about_the_ring_s`,
//!   the other pair the pin lost: the ring against the first pair's sphere
//!   about x, which is apart from it
//! - #938's and #805's tests still pass — no test: they are #938's in
//!   `src/body/tests/through_a_pole.rs`, and #805's in
//!   `a_chamfer_and_a_fillet_fill_a_concave_edge_up_to_a_curved_end.rs`,
//!   `src/body/tests.rs`, `src/brep/relation/tests/round.rs` and
//!   `src/brep/relation/tests/crossing/pierced.rs`, left as they were but for
//!   the two pairs the second criterion takes out of the pinned declines;
//!   running them is the answer
//!
//! A boolean between a sphere a step read about another axis and a sphere
//! of its centre and radius about the first — #938's cut, then a ball about
//! the axis it was turned about — is built only by reading one about the
//! other's axis before the boolean (`aligned`, in `brep/combine/charted.rs`):
//! with the two spheres one surface and their charts apart, it declines
//! `Unverified` without it. Main built it, the two spheres not yet one
//! surface, by #938's retry, which reads the second about the cut's axis;
//! this keeps it built —
//! `a_ball_cut_at_a_slant_through_its_pole_joined_with_or_cut_by_a_ball_of_its_centre_and_radius_turned_about_its_first_axis_is_built_sound_and_of_the_arithmetic_s_volume`.
//!
//! One fixture leaned on a decline #956 lifts: the turn of
//! `a_cone_turned_at_thirty_degrees_beside_a_ball_at`, in
//! `crates/part/tests/the_exact_kernel_turns_the_part.rs`, whose axis ran
//! through the ball's centre — its inner wall through the centre is what
//! declined, and its cone stood clear of the ball. Its axis now stands 1.5
//! off the centre, so that its cone crosses the ball off its centre, which
//! the kernel still declines, and the step after it is still numbered as if
//! it had stood.
//!
//! Out of scope, and so not asked here: spheres that differ by more than the
//! tolerance.

use std::collections::BTreeSet;
use std::f64::consts::{PI, TAU};

use cao_solid::brep::Scale;
use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Declined, Loop};
use glam::{DVec2, DVec3};

/// How far the matter a body holds may stand from what the arithmetic
/// leaves, as a share of it: the kernel's billionth.
const EXACT: f64 = 1e-9;

/// The drawing's tolerance, as a share of the reach of a body's own box,
/// over the area of its curved faces: `body/drawn.rs`'s, which no public
/// constant gives.
const DRAWN: f64 = 2e-4;

/// The radius of every ball here, about the origin.
const RADIUS: f64 = 5.0;

/// The radius of the bore along z.
const BORE: f64 = 1.0;

/// The tilts, in degrees from z, the planes through a pole lean at: #938's.
const TILTS: [f64; 3] = [20.0, 45.0, 70.0];

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
/// line, its poles 5 along it either way. #938's.
fn ball_about(axis: DVec3) -> Body {
    turned_about(axis, axis.any_orthonormal_vector(), TAU)
}

/// The half disc of [`RADIUS`] about the origin standing on the side of
/// `from`, square to `axis`, turned `angle` radians about the line through
/// the origin along `axis`, from `from` towards `axis × from`: turned half a
/// turn, the half ball on the side of `axis × from` of the plane holding
/// `axis` and `from`.
fn turned_about(axis: DVec3, from: DVec3, angle: f64) -> Body {
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
        u: from,
        v: axis,
    };
    let along_v = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    Body::default()
        .tool_turned(&profile, frame, &Turn::of(along_v, angle, 0.0, &profile))
        .expect("the exact kernel turns the ball")
}

/// A slab 10 thick along `normal` from the plane `offset` along it, its
/// sides 20 from the normal's line either way: past the plane, it holds all
/// of the ball. #938's.
fn slab_along(normal: DVec3, offset: f64) -> Body {
    let normal = normal.normalize();
    let u = normal.any_orthonormal_vector();
    Body::default()
        .tool_raised(
            &exact(Contour::rectangle(DVec2::splat(-20.0), DVec2::splat(20.0))),
            Frame {
                origin: normal * offset,
                u,
                v: normal.cross(u),
            },
            normal * 10.0,
        )
        .expect("the slab raises")
}

/// A bore of [`BORE`] along z through the origin, from z = −10 to 10, a tool
/// of `part`.
fn bore_along_z(part: &Body) -> Body {
    part.tool_raised(
        &exact(Contour {
            corners: vec![DVec2::X * BORE],
            runs: vec![Run::Round {
                center: DVec2::ZERO,
                turn: TAU,
            }],
        }),
        Frame {
            origin: DVec3::Z * -10.0,
            u: DVec3::X,
            v: DVec3::Y,
        },
        DVec3::Z * 20.0,
    )
    .expect("the bore raises")
}

/// Three ways square to `axis` a plane through a pole may lean towards.
/// #938's.
fn ways_square_to(axis: DVec3) -> [DVec3; 3] {
    let u = axis.any_orthonormal_vector();
    let w = axis.cross(u);
    let (sin, cos) = 250f64.to_radians().sin_cos();
    [u, w, u * cos + w * sin]
}

/// The normal of a plane through the pole 5 along `pole`, leaning `tilt`
/// degrees from `pole` towards `towards`. #938's.
fn leaning(pole: DVec3, tilt: f64, towards: DVec3) -> DVec3 {
    let (sin, cos) = tilt.to_radians().sin_cos();
    pole * cos + towards * sin
}

/// The ball of [`RADIUS`].
const BALL: f64 = 4.0 * PI / 3.0 * RADIUS * RADIUS * RADIUS;

/// What a cap of the ball `height` high holds.
fn cap(height: f64) -> f64 {
    PI * height * height * (3.0 * RADIUS - height) / 3.0
}

/// How far up the bore's wall the ball reaches, either side of its centre.
fn bore_reach() -> f64 {
    (RADIUS * RADIUS - BORE * BORE).sqrt()
}

/// What the bore takes from the whole ball: the ball less the napkin ring it
/// leaves, 4π/3 (r³ − (r² − b²)^{3/2}).
fn bore_in_the_ball() -> f64 {
    4.0 * PI / 3.0 * (RADIUS.powi(3) - bore_reach().powi(3))
}

/// What the bore takes from the cap past the plane through the pole at
/// (0, 0, 5) leaning `tilt` degrees from z towards x, z > 5 − x tan(tilt).
///
/// Within the bore, the cap stands over 0 ≤ x ≤ b; its slice at x holds, for
/// |y| ≤ Y(x), the height from the plane up to the sphere,
/// √(a² − y²) − p with a² = r² − x² and p = 5 − x tan(tilt): over y, that is
/// Y √(a² − Y²) + a² asin(Y / a) − 2 p Y. Y is where the plane meets the
/// sphere, √(a² − p²), up to the x at which that reaches the bore's wall,
/// where p = √(r² − b²), and √(b² − x²), the wall, past it. The two pieces
/// are summed over x by Simpson's rule after x = lo + (hi − lo)(1 − cos u)/2,
/// which leaves no square root at their ends: the sum holds to the last bits
/// at a few thousand steps.
fn bore_in_the_cap(tilt: f64) -> f64 {
    let slope = tilt.to_radians().tan();
    let slice = |x: f64| {
        let a2 = RADIUS * RADIUS - x * x;
        let p = RADIUS - x * slope;
        let y = (a2 - p * p).min(BORE * BORE - x * x).max(0.0).sqrt();
        y * (a2 - y * y).max(0.0).sqrt() + a2 * (y / a2.sqrt()).min(1.0).asin() - 2.0 * p * y
    };
    let meets_the_wall = (RADIUS - bore_reach()) / slope;
    [(0.0, meets_the_wall), (meets_the_wall, BORE)]
        .into_iter()
        .map(|(lo, hi)| {
            const STEPS: u32 = 1 << 12;
            let step = PI / f64::from(STEPS);
            (0..=STEPS)
                .map(|at| {
                    let u = step * f64::from(at);
                    let weight = match at {
                        0 | STEPS => 1.0,
                        _ if at % 2 == 1 => 4.0,
                        _ => 2.0,
                    };
                    let x = lo + (hi - lo) * (1.0 - u.cos()) / 2.0;
                    weight * slice(x) * (hi - lo) / 2.0 * u.sin()
                })
                .sum::<f64>()
                * step
                / 3.0
        })
        .sum()
}

/// One side of the ball the plane through a pole leaves: what lies past it,
/// the cap about the pole the plane leans from, or what lies behind it,
/// holding the other pole. #938's.
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

    /// The side the plane leaves across from this one.
    fn other(self) -> Side {
        match self {
            Side::Cap => Side::Rest,
            Side::Rest => Side::Cap,
        }
    }

    /// What the arithmetic says this side holds, the plane leaning `tilt`
    /// degrees from the pole: the cap 5 (1 − cos(tilt)) high, or the ball
    /// less it.
    fn holds(self, tilt: f64) -> f64 {
        let height = RADIUS * (1.0 - tilt.to_radians().cos());
        match self {
            Side::Cap => cap(height),
            Side::Rest => BALL - cap(height),
        }
    }

    /// The area of this side's piece of sphere, 2πr times its height.
    fn sphere(self, tilt: f64) -> f64 {
        let height = RADIUS * (1.0 - tilt.to_radians().cos());
        match self {
            Side::Cap => TAU * RADIUS * height,
            Side::Rest => TAU * RADIUS * (2.0 * RADIUS - height),
        }
    }

    /// What the arithmetic says this side holds once bored along z, the
    /// plane leaning `tilt` degrees from the pole: the cap less what the bore
    /// takes of it, or the ball less the cap and less what the bore takes of
    /// the rest. By the ball's and the bore's symmetry, whichever pole and
    /// whichever way the plane leans towards.
    fn bored(self, tilt: f64) -> f64 {
        let height = RADIUS * (1.0 - tilt.to_radians().cos());
        match self {
            Side::Cap => cap(height) - bore_in_the_cap(tilt),
            Side::Rest => BALL - cap(height) - (bore_in_the_ball() - bore_in_the_cap(tilt)),
        }
    }

    /// The area this side's curved faces have at most once bored, which the
    /// drawing's tolerance is held over: its piece of sphere before the bore,
    /// 2πr times its height, and the bore's wall as high as it can stand on
    /// this side — through the ball behind the plane, from the plane up to
    /// the sphere within the cap.
    fn curved(self, tilt: f64) -> f64 {
        let height = RADIUS * (1.0 - tilt.to_radians().cos());
        match self {
            Side::Cap => {
                TAU * RADIUS * height
                    + TAU * BORE * (bore_reach() - RADIUS + BORE * tilt.to_radians().tan())
            }
            Side::Rest => TAU * RADIUS * (2.0 * RADIUS - height) + TAU * BORE * 2.0 * bore_reach(),
        }
    }
}

/// Holds that a body is exact, holds `kept`, and that its triangles are
/// closed, uncrossed and enclose that within what the drawing promises over
/// its curved faces, `curved` at most: each triangle within the drawing's
/// tolerance of its face, at the reach of the body's own box. #938's.
fn built_and_drawn(body: &Body, kept: f64, curved: f64, what: &str) {
    assert!(
        body.is_exact() && (body.volume() - kept).abs() <= EXACT * kept,
        "{what}: holds {}, where the arithmetic leaves {kept}",
        body.volume()
    );
    let triangles = body.triangles();
    assert_eq!(closed(&triangles), Ok(()), "{what}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
    let (low, high) = body.bounds().expect("a body has a box");
    let reach = Scale::of(low.abs().max(high.abs()).max_element()).reach();
    let promised = DRAWN * reach * curved;
    let enclosed = enclosed(&triangles);
    assert!(
        (enclosed - kept).abs() <= promised,
        "{what}: its triangles enclose {enclosed}, where it holds {kept}, more than the \
         {promised} the drawing's tolerance gives"
    );
}

/// The ball about z cut through either pole by a plane leaning 20°, 45° and
/// 70° from z towards each of three ways, either side kept, as #938 builds
/// it — its sphere then read about the cut circle's axis — and that side
/// bored along z through the ball's centre, the axis the ball was turned
/// about: the bore's wall meets the sphere along circles that axis decides.
/// Each bored side is built, sound and holds what the arithmetic leaves.
#[test]
fn a_ball_turned_about_z_cut_at_a_slant_through_its_pole_then_bored_along_z_through_its_centre_is_built_sound_and_holds_the_arithmetic_s_volume()
 {
    let ball = ball_about(DVec3::Z);
    for pole in [DVec3::Z, -DVec3::Z] {
        for tilt in TILTS {
            for towards in ways_square_to(DVec3::Z) {
                let normal = leaning(pole, tilt, towards);
                let offset = RADIUS * normal.dot(pole);
                for side in SIDES {
                    let what = format!(
                        "the ball about z cut through its pole at {} by a plane leaning {tilt}° \
                         towards {towards}, keeping its {side:?}, then bored along z",
                        pole * RADIUS
                    );
                    let cut =
                        ball.difference(&side.slab(normal, offset))
                            .unwrap_or_else(|declined| {
                                panic!("{what}: the cut, which #938 builds, declined, {declined:?}")
                            });

                    let bored = cut
                        .difference(&bore_along_z(&cut))
                        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

                    built_and_drawn(&bored, side.bored(tilt), side.curved(tilt), &what);
                }
            }
        }
    }
}

/// The four booleans of `first` and `second`: joined either way, and each
/// cut by the other.
fn booleans(first: &Body, second: &Body) -> [(&'static str, Result<Body, Declined>); 4] {
    [
        ("joined with", first.union(second)),
        ("joined to, the other first,", second.union(first)),
        ("cut by", first.difference(second)),
        ("cut from", second.difference(first)),
    ]
}

/// How many faces of `body` are flat, and how many are not, each of the
/// latter drawn on the sphere of [`RADIUS`] about the origin.
fn kinds(body: &Body, what: &str) -> [usize; 2] {
    let faces: BTreeSet<usize> = (0..body.faces_end())
        .filter(|&face| body.has_face(face))
        .chain(body.pieces())
        .collect();
    let flat = faces.iter().filter(|&&face| body.is_flat(face)).count();
    for &face in faces.iter().filter(|&&face| !body.is_flat(face)) {
        for corner in body.triangles_of(face).flatten() {
            assert!(
                (corner.length() - RADIUS).abs() <= EXACT * RADIUS,
                "{what}: face {face} is drawn through {corner}, off the sphere"
            );
        }
    }
    [flat, faces.len() - flat]
}

/// Holds that `body` is the body `reference`, the same boolean between two
/// balls turned about one axis, is, and that both are what the arithmetic
/// says, `kept`: each exact and holding `kept` to a billionth of the ball,
/// its triangles closed and uncrossed; `body` empty where `reference` is,
/// with as many flat faces and as many on the sphere.
fn the_same_body(body: &Body, reference: &Body, kept: f64, what: &str) {
    for (one, which) in [(reference, "about one axis"), (body, "about x and y")] {
        assert!(
            one.is_exact() && (one.volume() - kept).abs() <= EXACT * BALL,
            "{what}, {which}: holds {}, where the arithmetic leaves {kept}",
            one.volume()
        );
        let triangles = one.triangles();
        assert_eq!(closed(&triangles), Ok(()), "{what}, {which}");
        assert_eq!(uncrossed(&triangles), Ok(()), "{what}, {which}");
    }
    assert_eq!(
        body.is_empty(),
        reference.is_empty(),
        "{what}: empty where the same boolean about one axis is, or not"
    );
    assert_eq!(
        kinds(body, what),
        kinds(reference, what),
        "{what}: as many flat faces, and as many on the sphere, as the same boolean about one \
         axis leaves"
    );
}

/// A ball turned whole about x, from y, and one of its centre and radius
/// turned whole about y, from z, joined either way and each cut by the
/// other: the ball, the ball, nothing and nothing, as two balls turned about
/// x leave them. Then each turned half a turn, the half ball z ≥ 0 about x
/// and the half ball x ≥ 0 about y: three quarters of the ball, either way,
/// and a quarter for each cut, as the half ball z ≥ 0 turned about y leaves
/// them with the second. Each built, and the body the same boolean about one
/// axis leaves.
#[test]
fn a_ball_turned_about_x_joined_with_or_cut_by_a_ball_of_its_centre_and_radius_turned_about_y_is_the_body_two_balls_turned_about_one_axis_leave()
 {
    for (turned, about_x_and_y, about_one_axis, [joined, x_less_y, y_less_x]) in [
        (
            "whole",
            [
                turned_about(DVec3::X, DVec3::Y, TAU),
                turned_about(DVec3::Y, DVec3::Z, TAU),
            ],
            [
                turned_about(DVec3::X, DVec3::Y, TAU),
                turned_about(DVec3::X, DVec3::Z, TAU),
            ],
            [BALL, 0.0, 0.0],
        ),
        (
            "half a turn",
            [
                turned_about(DVec3::X, DVec3::Y, PI),
                turned_about(DVec3::Y, DVec3::Z, PI),
            ],
            [
                turned_about(DVec3::Y, -DVec3::X, PI),
                turned_about(DVec3::Y, DVec3::Z, PI),
            ],
            [0.75 * BALL, 0.25 * BALL, 0.25 * BALL],
        ),
    ] {
        let ours = booleans(&about_x_and_y[0], &about_x_and_y[1]);
        let theirs = booleans(&about_one_axis[0], &about_one_axis[1]);
        let kept = [joined, joined, x_less_y, y_less_x];
        for (((way, ours), (_, theirs)), kept) in ours.into_iter().zip(theirs).zip(kept) {
            let what = format!(
                "a ball turned {turned} about x {way} a ball of its centre and radius turned \
                 {turned} about y"
            );
            let reference = theirs.unwrap_or_else(|declined| {
                panic!("{what}, the two turned about one axis: declined, {declined:?}")
            });

            let body = ours.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

            the_same_body(&body, &reference, kept, &what);
        }
    }
}

/// The ball about z cut through its pole at (0, 0, 5) by a plane leaning
/// 20°, 45° and 70° towards x, either side kept, as #938 builds it — its
/// sphere then read about the cut circle's axis — then joined with a ball of
/// its centre and radius turned about z, cut by it, and each the other way:
/// the ball either way, nothing, and the side across the plane. Each built,
/// sound and holding what the arithmetic leaves.
#[test]
fn a_ball_cut_at_a_slant_through_its_pole_joined_with_or_cut_by_a_ball_of_its_centre_and_radius_turned_about_its_first_axis_is_built_sound_and_of_the_arithmetic_s_volume()
 {
    let ball = ball_about(DVec3::Z);
    for tilt in TILTS {
        let normal = leaning(DVec3::Z, tilt, DVec3::X);
        let offset = RADIUS * normal.dot(DVec3::Z);
        for side in SIDES {
            let cut = ball
                .difference(&side.slab(normal, offset))
                .expect("the cut, which #938 builds");
            let other = side.other();
            for ((way, built), kept) in booleans(&cut, &ball_about(DVec3::Z)).into_iter().zip([
                Some((BALL, 100.0 * PI)),
                Some((BALL, 100.0 * PI)),
                None,
                Some((other.holds(tilt), other.sphere(tilt))),
            ]) {
                let what = format!(
                    "the ball about z cut through its pole by a plane leaning {tilt}° towards x, \
                     keeping its {side:?}, {way} a ball of its centre and radius turned about z"
                );

                let body =
                    built.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

                match kept {
                    Some((kept, curved)) => built_and_drawn(&body, kept, curved, &what),
                    None => assert!(
                        body.is_exact() && body.is_empty(),
                        "{what}: nothing left, where it holds {}",
                        body.volume()
                    ),
                }
            }
        }
    }
}
