//! A ball already bored or knobbed along the axis it was turned about is cut
//! by a plane at a slant through a pole of that axis as the ball alone is
//! (#938), and as the cut ball is bored after (#956): the order of the two
//! steps no longer decides whether the part builds.
//!
//! Closes #1107.
//! - A ball about z bored along z through its centre, then cut through its
//!   pole by a plane leaning 20°, 45° and 70°, either side kept: built,
//!   sound, the volume the arithmetic gives — the same body as the cut then
//!   the bore (#956's test) —
//!   `a_ball_turned_about_z_bored_along_z_through_its_centre_then_cut_at_a_slant_through_its_pole_is_built_sound_of_the_arithmetic_s_volume_and_the_body_the_other_order_leaves`.
//!   Read as #956 reads "built, sound": the step not declined, the body
//!   exact and holding the arithmetic's matter to a billionth, its triangles
//!   closed, uncrossed and enclosing that matter within the drawing's
//!   tolerance over its curved faces. "The same body" read as: the cut then
//!   the bore, #956's order, built too, holding the same matter to a
//!   billionth and fitting the same box within the kernel's tolerance — not
//!   as many faces, which two orders may part differently. Answered as
//!   #956's test is, at either pole and towards three ways, whose arithmetic
//!   is one by the ball's and the bore's symmetry
//! - The same with a knob along z:
//!   `a_ball_with_a_coaxial_knob_cut_at_a_slant_through_its_other_pole_is_declined_as_unsupported`
//!   turns into a built body —
//!   `a_ball_turned_about_z_with_a_coaxial_knob_at_one_pole_cut_at_a_slant_through_the_other_is_built_sound_of_the_arithmetic_s_volume_and_the_body_the_other_order_leaves`.
//!   Read with that test's knob, a ball of radius 1 about z centred on the
//!   pole at (0, 0, −5), and its cut through the other pole, at the first
//!   criterion's tilts, towards three ways, either side kept. The knob
//!   stands clear of the plane at every tilt: keeping the rest, the body is
//!   the one the cut then the knob leave; keeping the cap, the knob falls
//!   with the rest and the body is the cut ball's, #938's. The test that
//!   pinned the decline, at 45° alone, is removed from
//!   `src/body/tests/through_a_pole.rs`: this one holds its case at every
//!   tilt
//! - #938's, #956's and #805's tests keep their bodies — no test: they are
//!   #938's in `src/body/tests/through_a_pole.rs`, less the knob's decline
//!   the second criterion removes; #956's in
//!   `spheres_of_one_centre_and_radius_are_one_surface_whatever_their_axes.rs`;
//!   and #805's in
//!   `a_chamfer_and_a_fillet_fill_a_concave_edge_up_to_a_curved_end.rs`,
//!   `src/body/tests.rs`, `src/brep/relation/tests/round.rs` and
//!   `src/brep/relation/tests/crossing/pierced.rs`. Running them unedited is
//!   the answer

use std::f64::consts::{PI, TAU};

use cao_solid::brep::Scale;
use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Loop};
use glam::{DVec2, DVec3};

/// How far the matter a body holds may stand from what the arithmetic
/// leaves, as a share of it: the kernel's billionth.
const EXACT: f64 = 1e-9;

/// The drawing's tolerance, as a share of the reach of a body's own box,
/// over the area of its curved faces: `body/drawn.rs`'s, which no public
/// constant gives. #956's.
const DRAWN: f64 = 2e-4;

/// The radius of the ball, about the origin.
const RADIUS: f64 = 5.0;

/// The radius of the bore along z.
const BORE: f64 = 1.0;

/// The radius of the knob, a ball about z centred on the ball's pole at
/// (0, 0, −5). #938's.
const KNOB: f64 = 1.0;

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

/// A ball of `radius` about `center`, a tool of `part`: a half disc turned
/// whole about the line through `center` along `axis`, its sphere's chart
/// laid about that line, its poles `radius` along it either way. #938's.
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
    let along_v = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    part.tool_turned(&profile, frame, &Turn::of(along_v, TAU, 0.0, &profile))
        .expect("the exact kernel turns the ball")
}

/// The ball of [`RADIUS`] about the origin, turned about z: its poles at
/// (0, 0, ±5).
fn ball_about_z() -> Body {
    ball_of(&Body::default(), DVec3::ZERO, DVec3::Z, RADIUS)
}

/// The knob of [`KNOB`] about z centred on the ball's pole at (0, 0, −5), a
/// tool of `part`. #938's.
fn knob_of(part: &Body) -> Body {
    ball_of(part, DVec3::Z * -RADIUS, DVec3::Z, KNOB)
}

/// A slab 20 thick along `normal` from the plane `offset` along it, its
/// sides 20 from the normal's line either way, a tool of `part`: past the
/// plane, it holds all of the ball and of its knob, which reach 11 from
/// the ball's pole at the most.
fn slab_along(part: &Body, normal: DVec3, offset: f64) -> Body {
    let normal = normal.normalize();
    let u = normal.any_orthonormal_vector();
    part.tool_raised(
        &exact(Contour::rectangle(DVec2::splat(-20.0), DVec2::splat(20.0))),
        Frame {
            origin: normal * offset,
            u,
            v: normal.cross(u),
        },
        normal * 20.0,
    )
    .expect("the slab raises")
}

/// A bore of [`BORE`] along z through the origin, from z = −10 to 10, a tool
/// of `part`. #956's.
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

/// What a cap `height` high of a ball of `radius` holds.
fn cap_of(radius: f64, height: f64) -> f64 {
    PI * height * height * (3.0 * radius - height) / 3.0
}

/// What a cap of the ball `height` high holds.
fn cap(height: f64) -> f64 {
    cap_of(RADIUS, height)
}

/// How far up the bore's wall the ball reaches, either side of its centre.
fn bore_reach() -> f64 {
    (RADIUS * RADIUS - BORE * BORE).sqrt()
}

/// What the bore takes from the whole ball: the ball less the napkin ring it
/// leaves, 4π/3 (r³ − (r² − b²)^{3/2}). #956's.
fn bore_in_the_ball() -> f64 {
    4.0 * PI / 3.0 * (RADIUS.powi(3) - bore_reach().powi(3))
}

/// What the bore takes from the cap past the plane through the pole at
/// (0, 0, 5) leaning `tilt` degrees from z towards x, z > 5 − x tan(tilt).
/// #956's, which says how.
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

/// How far from the ball's centre, along z, the plane stands that holds the
/// circle the knob's sphere and the ball's meet on: (r² + r² − k²) / 2r,
/// 4.9.
fn knob_meets_the_ball() -> f64 {
    (2.0 * RADIUS * RADIUS - KNOB * KNOB) / (2.0 * RADIUS)
}

/// What the knob adds to the ball: the knob less the lens the two share,
/// the knob's cap past the plane they meet on, k − (r − 4.9) high, and the
/// ball's cap past it, r − 4.9 high. 43π/60.
fn knob_outside() -> f64 {
    let below_the_ball = RADIUS - knob_meets_the_ball();
    4.0 * PI / 3.0 * KNOB.powi(3)
        - cap_of(KNOB, KNOB - below_the_ball)
        - cap_of(RADIUS, below_the_ball)
}

/// The area of the knob's sphere standing out of the ball: the knob's sphere
/// less its cap within the ball, 2πk times its height.
fn knob_s_sphere_outside() -> f64 {
    let below_the_ball = RADIUS - knob_meets_the_ball();
    4.0 * PI * KNOB * KNOB - TAU * KNOB * (KNOB - below_the_ball)
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
    /// The slab, a tool of `part`, whose cut from the ball leaves this side.
    fn slab(self, part: &Body, normal: DVec3, offset: f64) -> Body {
        match self {
            Side::Cap => slab_along(part, -normal, -offset),
            Side::Rest => slab_along(part, normal, offset),
        }
    }

    /// The height of the cap the plane leaning `tilt` degrees from the pole
    /// leaves, 5 (1 − cos(tilt)).
    fn height(tilt: f64) -> f64 {
        RADIUS * (1.0 - tilt.to_radians().cos())
    }

    /// What the arithmetic says this side holds once bored along z, the
    /// plane leaning `tilt` degrees from the pole: the cap less what the bore
    /// takes of it, or the ball less the cap and less what the bore takes of
    /// the rest. By the ball's and the bore's symmetry, whichever pole and
    /// whichever way the plane leans towards. #956's.
    fn bored(self, tilt: f64) -> f64 {
        let height = Side::height(tilt);
        match self {
            Side::Cap => cap(height) - bore_in_the_cap(tilt),
            Side::Rest => BALL - cap(height) - (bore_in_the_ball() - bore_in_the_cap(tilt)),
        }
    }

    /// The area this side's curved faces have at most once bored: its piece
    /// of sphere before the bore, and the bore's wall as high as it can
    /// stand on this side. #956's.
    fn bored_curved(self, tilt: f64) -> f64 {
        let height = Side::height(tilt);
        match self {
            Side::Cap => {
                TAU * RADIUS * height
                    + TAU * BORE * (bore_reach() - RADIUS + BORE * tilt.to_radians().tan())
            }
            Side::Rest => TAU * RADIUS * (2.0 * RADIUS - height) + TAU * BORE * 2.0 * bore_reach(),
        }
    }

    /// What the arithmetic says this side holds with the knob at the other
    /// pole, the plane leaning `tilt` degrees from the pole it runs through:
    /// the cap alone, the knob standing clear of it, or the ball less the
    /// cap and with all the knob adds.
    fn knobbed(self, tilt: f64) -> f64 {
        let height = Side::height(tilt);
        match self {
            Side::Cap => cap(height),
            Side::Rest => BALL - cap(height) + knob_outside(),
        }
    }

    /// The area this side's curved faces have at most with the knob: its
    /// piece of sphere, and on the rest the knob's sphere out of the ball.
    fn knobbed_curved(self, tilt: f64) -> f64 {
        let height = Side::height(tilt);
        match self {
            Side::Cap => TAU * RADIUS * height,
            Side::Rest => TAU * RADIUS * (2.0 * RADIUS - height) + knob_s_sphere_outside(),
        }
    }
}

/// Holds that a body is exact, holds `kept`, and that its triangles are
/// closed, uncrossed and enclose that within what the drawing promises over
/// its curved faces, `curved` at most: each triangle within the drawing's
/// tolerance of its face, at the reach of the body's own box. #956's.
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

/// Holds that `body` is the body `reference`, built from the same steps in
/// the other order, is: `reference` exact and holding the matter `body`
/// holds to a billionth, and both fitting one box within the kernel's
/// tolerance at its reach.
fn the_same_body(body: &Body, reference: &Body, what: &str) {
    assert!(
        reference.is_exact() && (reference.volume() - body.volume()).abs() <= EXACT * body.volume(),
        "{what}: holds {}, where the other order leaves {}",
        body.volume(),
        reference.volume()
    );
    let (low, high) = body.bounds().expect("a body has a box");
    let (other_low, other_high) = reference.bounds().expect("a body has a box");
    let eps = Scale::of(low.abs().max(high.abs()).max_element()).eps();
    assert!(
        low.abs_diff_eq(other_low, eps) && high.abs_diff_eq(other_high, eps),
        "{what}: fits the box from {low} to {high}, where the other order leaves one from \
         {other_low} to {other_high}"
    );
}

/// The ball about z bored along z through its centre, the axis it was turned
/// about, then cut through either pole by a plane leaning 20°, 45° and 70°
/// from z towards each of three ways, either side kept: the bore's wall
/// meets the sphere along circles that axis decides before the cut reads it
/// about another. Each side is built, sound, holds what the arithmetic
/// leaves, and is the body #956's order — the cut, then the bore — leaves.
#[test]
fn a_ball_turned_about_z_bored_along_z_through_its_centre_then_cut_at_a_slant_through_its_pole_is_built_sound_of_the_arithmetic_s_volume_and_the_body_the_other_order_leaves()
 {
    let ball = ball_about_z();
    let bored = ball
        .difference(&bore_along_z(&ball))
        .expect("a ball bored along the axis it was turned about");
    for pole in [DVec3::Z, -DVec3::Z] {
        for tilt in TILTS {
            for towards in ways_square_to(DVec3::Z) {
                let normal = leaning(pole, tilt, towards);
                let offset = RADIUS * normal.dot(pole);
                for side in SIDES {
                    let what = format!(
                        "the ball about z bored along z, then cut through its pole at {} by a \
                         plane leaning {tilt}° towards {towards}, keeping its {side:?}",
                        pole * RADIUS
                    );

                    let cut = bored
                        .difference(&side.slab(&bored, normal, offset))
                        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

                    built_and_drawn(&cut, side.bored(tilt), side.bored_curved(tilt), &what);
                    let first_cut = ball
                        .difference(&side.slab(&ball, normal, offset))
                        .unwrap_or_else(|declined| {
                            panic!("{what}: the cut, which #938 builds, declined, {declined:?}")
                        });
                    let reference = first_cut
                        .difference(&bore_along_z(&first_cut))
                        .unwrap_or_else(|declined| {
                            panic!(
                                "{what}: the cut then the bore, which #956 builds, declined, \
                                 {declined:?}"
                            )
                        });
                    the_same_body(&cut, &reference, &what);
                }
            }
        }
    }
}

/// #938's knobbed ball — the ball about z joined to the knob about z on its
/// pole at (0, 0, −5) — cut through its other pole, at (0, 0, 5), by a
/// plane leaning 20°, 45° and 70° from z towards each of three ways, either
/// side kept: the knob's sphere meets the ball's along a circle z decides
/// before the cut reads it about another axis. Each side is built, sound,
/// holds what the arithmetic leaves, and is the body the same steps leave
/// the other way round: the cut, then the knob, where the rest is kept and
/// the knob stands clear of the plane; the cut alone, #938's, where the cap
/// is kept and the knob falls with the rest.
#[test]
fn a_ball_turned_about_z_with_a_coaxial_knob_at_one_pole_cut_at_a_slant_through_the_other_is_built_sound_of_the_arithmetic_s_volume_and_the_body_the_other_order_leaves()
 {
    let ball = ball_about_z();
    let knobbed = ball
        .union(&knob_of(&ball))
        .expect("a coaxial knob joins the ball");
    for tilt in TILTS {
        for towards in ways_square_to(DVec3::Z) {
            let normal = leaning(DVec3::Z, tilt, towards);
            let offset = RADIUS * normal.dot(DVec3::Z);
            for side in SIDES {
                let what = format!(
                    "the ball about z with a knob at (0, 0, -5), cut through its pole at \
                     (0, 0, 5) by a plane leaning {tilt}° towards {towards}, keeping its {side:?}"
                );

                let cut = knobbed
                    .difference(&side.slab(&knobbed, normal, offset))
                    .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

                built_and_drawn(&cut, side.knobbed(tilt), side.knobbed_curved(tilt), &what);
                let first_cut = ball
                    .difference(&side.slab(&ball, normal, offset))
                    .unwrap_or_else(|declined| {
                        panic!("{what}: the cut, which #938 builds, declined, {declined:?}")
                    });
                let reference = match side {
                    Side::Cap => first_cut,
                    Side::Rest => {
                        first_cut
                            .union(&knob_of(&first_cut))
                            .unwrap_or_else(|declined| {
                                panic!("{what}: the cut then the knob declined, {declined:?}")
                            })
                    }
                };
                the_same_body(&cut, &reference, &what);
            }
        }
    }
}
