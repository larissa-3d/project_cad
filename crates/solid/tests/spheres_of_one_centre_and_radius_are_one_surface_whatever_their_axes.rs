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
//!   in `src/brep/relation/tests/round.rs`; and a ring meets a sphere whose
//!   centre its axis holds, turned about another axis, as it meets that
//!   sphere turned about its own —
//!   `a_ball_whose_centre_a_ring_s_axis_holds_meets_it_whatever_axis_it_was_turned_about_as_it_does_turned_about_the_ring_s`.
//!   Neither pair is among the declines
//!   `every_other_pair_with_a_round_is_unsupported_and_none_is_at_a_slant`
//!   holds
//! - #938's and #805's tests still pass — no test: they are #938's in
//!   `src/body/tests/through_a_pole.rs`, and #805's in
//!   `a_chamfer_and_a_fillet_fill_a_concave_edge_up_to_a_curved_end.rs`,
//!   `src/body/tests.rs`, `src/brep/relation/tests/round.rs` and
//!   `src/brep/relation/tests/crossing/pierced.rs`; running them is the
//!   answer
//!
//! A boolean between a sphere a step read about another axis and a sphere
//! of its centre and radius about the first — #938's cut, then a ball about
//! the axis it was turned about — is built only by reading one about the
//! other's axis before the boolean (`aligned`, in `brep/combine/charted.rs`):
//! with the two spheres one surface and their charts apart, it declines
//! `Unverified` without it —
//! `a_ball_cut_at_a_slant_through_its_pole_joined_with_or_cut_by_a_ball_of_its_centre_and_radius_turned_about_its_first_axis_is_built_sound_and_of_the_arithmetic_s_volume`.
//!
//! The turn of `a_cone_turned_at_thirty_degrees_beside_a_ball_at`, in
//! `crates/part/tests/the_exact_kernel_turns_the_part.rs`, stands about a
//! line 1.5 off the ball's centre, so that its cone crosses the ball off its
//! centre, which the kernel declines: a wall whose axis runs through the
//! centre is no decline.
//!
//! Out of scope, and so not asked here: spheres that differ by more than the
//! tolerance.
//!
//! Beyond the criteria — pins, not criteria of #956 — what reading a sphere
//! about another axis reaches downstream.
//!
//! Decision 12 lays a wall a hair inside a ball onto the touch where the two
//! are coaxial about the wall's axis, whatever the ball's chart's: a shaft
//! through the ball's centre along another axis a hair under its radius is
//! laid onto the touch, rather than leaving two circles a few
//! ten-thousandths apart and a ring of crossed triangles:
//! `a_ball_turned_at_a_slant_joined_with_or_cut_by_a_shaft_along_z_through_its_centre_even_a_hair_under_its_radius_is_built_sound_and_of_the_arithmetic_s_volume`.
//! Where those circles run within the drawing's reach of a pole of the
//! ball's own chart (#962), the boolean is declined:
//! `a_ball_turned_about_x_joined_with_or_cut_by_a_shaft_along_z_through_its_centre_is_declined_where_the_rims_pass_near_its_chart_s_poles_and_built_further_off`.
//! The same with a plane a hair above a rim the first criterion's bore
//! leaves is built sound, the plane laid onto the rim as it is beside a ball
//! turned about z:
//! `a_ball_cut_at_a_slant_through_its_pole_and_bored_along_z_joined_with_or_cut_by_a_slab_whose_top_stands_a_hair_above_the_bore_s_rim_is_built_sound_and_of_the_arithmetic_s_volume_or_declined`.
//!
//! So does every circle a sphere's faces are rimmed by, each read about its
//! own axis through the centre: a slab parallel to a cut at a slant, or to
//! the flat face of a ball turned part of the way, a hair off it, is laid
//! onto it or left as drawn, and built sound either way:
//! `a_ball_cut_at_a_slant_joined_with_or_cut_by_a_slab_parallel_to_the_cut_a_hair_off_it_is_built_sound_and_holds_the_matter_as_drawn_or_laid_onto_the_cut`,
//! `a_ball_turned_part_way_joined_with_or_cut_by_a_slab_parallel_to_its_flat_face_a_hair_off_it_is_built_sound_and_holds_the_matter_as_drawn_or_laid_onto_the_face`.
//!
//! #938's re-reading is refused where another surface meets the sphere along
//! circles its chart's axis decides, and only there: a surface whose axis
//! runs through the centre along another axis, standing clear of the ball,
//! leaves the sphere free to be read about a cut circle's axis:
//! `a_ball_beside_a_rod_whose_axis_runs_through_its_centre_cut_at_a_slant_through_its_pole_is_built_sound_and_of_the_arithmetic_s_volume`.
//!
//! Where neither sphere can be read about the other's axis, each one's
//! circles running through the other's poles, the boolean is built sound:
//! `a_half_ball_turned_about_x_joined_with_or_cut_by_itself_turned_about_y_is_built_sound_and_of_the_arithmetic_s_volume_or_declined`.
//!
//! Two spheres are read about one axis before the boolean only where they
//! are one surface on charts of two axes: a ball about z and a smaller ball
//! about x whose axis runs through the first's centre are read each in its
//! own chart, and join and cut along the circle they cross on —
//! `a_ball_and_a_ball_about_an_axis_through_its_centre_join_and_one_is_cut_by_the_other_along_the_circle_they_cross_on`,
//! the first cut by the second built sound or declined, never crossed —
//! `a_ball_cut_by_a_ball_about_an_axis_through_its_centre_is_built_sound_and_of_the_arithmetic_s_volume_or_declined`,
//! and the same two balls turned about one axis built sound, the drawing
//! leaving out a row of the smaller's grid a hair from the rim they cross
//! on rather than a sliver of strip beside it —
//! `a_ball_cut_by_a_smaller_ball_on_its_own_axis_whose_rim_falls_a_hair_from_a_row_of_its_grid_is_built_sound_and_of_the_arithmetic_s_volume`,
//! the rule itself held in `src/brep/tessellation/sampling/tests/rows.rs`;
//! a meridian arc keeping every row all the same, where the faces either
//! side of it are cut —
//! `a_half_ball_on_a_shaft_s_top_a_hair_wider_than_it_whose_meridian_arcs_end_a_hair_from_a_row_of_its_grid_is_drawn_closed`;
//! and two charts whose axes part by a few hundred-millionths of a radian,
//! far less than the drawing sees but more than the tolerance at the ball's
//! radius, are read about one —
//! `a_ball_cut_at_a_slant_through_its_pole_joined_with_or_cut_by_a_ball_of_its_centre_and_radius_turned_about_an_axis_a_hair_off_the_cut_s_is_built_sound_and_of_the_arithmetic_s_volume`.
//!
//! The rest is held beside the code it reaches: a cone and a cylinder whose
//! axis runs through a sphere's centre along another, and how far a plane
//! square to a ball's axis or a cone about it stands from touching it, at
//! the relation, in `src/brep/relation/tests/round.rs`; which side of each other a sphere and
//! such a cylinder stand on, read in the meridian half-plane of their rims'
//! axis rather than the sphere's chart's, in
//! `src/brep/selection/tests/round.rs`; and a turn whose inner wall runs
//! through a ball's centre, built, in
//! `crates/part/tests/the_exact_kernel_turns_the_part.rs`.

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
    shaft_along_z(part, BORE)
}

/// A shaft of `radius` along z through the origin, from z = −10 to 10, a
/// tool of `part`.
fn shaft_along_z(part: &Body, radius: f64) -> Body {
    shaft_along_z_from(part, radius, -10.0)
}

/// A shaft of `radius` along z through the origin, from z = `from` to 10, a
/// tool of `part`.
fn shaft_along_z_from(part: &Body, radius: f64, from: f64) -> Body {
    part.tool_raised(
        &exact(Contour {
            corners: vec![DVec2::X * radius],
            runs: vec![Run::Round {
                center: DVec2::ZERO,
                turn: TAU,
            }],
        }),
        Frame {
            origin: DVec3::Z * from,
            u: DVec3::X,
            v: DVec3::Y,
        },
        DVec3::Z * (10.0 - from),
    )
    .expect("the shaft raises")
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

/// Holds that a boolean either declined, or built a body that is exact, its
/// triangles closed and uncrossed, holding within `within` of one of
/// `volumes` — never a crossed body, nor one short of a cap or holding a
/// face it should not.
fn sound_or_declined(built: Result<Body, Declined>, volumes: &[f64], within: f64, what: &str) {
    let Ok(body) = built else {
        return;
    };
    assert!(
        body.is_exact()
            && volumes
                .iter()
                .any(|volume| (body.volume() - volume).abs() <= within),
        "{what}: holds {}, where the arithmetic leaves one of {volumes:?}",
        body.volume()
    );
    let triangles = body.triangles();
    assert_eq!(closed(&triangles), Ok(()), "{what}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
}

/// What a shaft of `radius` along z through the ball's centre, from z =
/// `from` to 10, holds, and what it takes of the ball: π b² (10 − `from`),
/// and the ball's slices above `from`, each the disc of the shaft or the
/// ball's own where that is the smaller — the shaft's within a of the
/// equator, a = √(r² − b²), the ball's π (r² − z²) beyond. From z = −10, the
/// ball less the napkin ring it leaves, 4π/3 (r³ − a³).
fn shaft_and_its_share_of_the_ball(radius: f64, from: f64) -> [f64; 2] {
    let reach = (RADIUS * RADIUS - radius * radius).max(0.0).sqrt();
    let round = |low: f64, high: f64| {
        PI * (RADIUS * RADIUS * (high - low) - (high.powi(3) - low.powi(3)) / 3.0)
    };
    let below = |height: f64| {
        let height = height.clamp(-RADIUS, RADIUS);
        round(-RADIUS, height.min(-reach))
            + PI * radius * radius * (height.clamp(-reach, reach) + reach)
            + round(reach, height.max(reach))
    };
    [
        PI * radius * radius * (10.0 - from),
        below(RADIUS) - below(from),
    ]
}

/// What a ball and a shaft of `radius` through its centre from z = `from`
/// leave, joined either way, the ball cut by the shaft and the shaft by the
/// ball.
fn ball_and_shaft(radius: f64, from: f64) -> [f64; 4] {
    let [shaft, shared] = shaft_and_its_share_of_the_ball(radius, from);
    [
        BALL + shaft - shared,
        BALL + shaft - shared,
        BALL - shared,
        shaft - shared,
    ]
}

/// The cosine of the angle off a sphere's chart axis the drawing lays a
/// circle of another axis at, `tessellation/outline/central.rs`'s, which no
/// public constant gives: a rim nearer a pole than that, about 2.9°, is no
/// circle the drawing closes a face along (#962).
const FRONT: f64 = 0.05;

/// The shafts along z through the ball's centre the two tests below hold,
/// each with where it starts and whether it stands a hair under the ball's
/// radius: of radius 1 and 3, and of the ball's radius, touching it along its
/// equator about z, from z = −10; a hair under the ball's radius — 2, 5 and
/// 15 of the tolerance the two are read at, under [`Scale::HAIR`] — where
/// the shaft's wall meets the sphere along two circles a few
/// ten-thousandths either side of that equator, so near it that decision 12
/// lays such a wall onto the touch, from z = −10 and from within the band
/// between those circles, a hundred-thousandth above the lower.
fn shafts() -> Vec<(f64, f64, bool)> {
    let eps = Scale::of(10.0).eps();
    let band = |radius: f64| -(RADIUS * RADIUS - radius * radius).sqrt() + 1e-5;
    let hairs = [2.0, 5.0, 15.0].map(|hair| RADIUS - hair * eps);
    [
        (1.0, -10.0, false),
        (3.0, -10.0, false),
        (RADIUS, -10.0, true),
    ]
    .into_iter()
    .chain(hairs.map(|radius| (radius, -10.0, true)))
    .chain(hairs.map(|radius| (radius, band(radius), true)))
    .collect()
}

/// The ball turned about a slant, (1, 0, 1), and each of [`shafts`]: their
/// rims stand 45° or more from its chart's poles. Joined either way, the
/// ball cut by the shaft and the shaft by the ball: each built, exact, its
/// triangles closed and uncrossed, holding what the arithmetic leaves — at
/// the shaft's radius, or at the ball's for a shaft laid onto the touch;
/// never a crossed body, nor the matter of a face kept on the wrong side. A
/// shaft from within the band, the shaft second, may decline: #1109 carries
/// it, and the ball joined with it declines `Undrawn` today.
#[test]
fn a_ball_turned_at_a_slant_joined_with_or_cut_by_a_shaft_along_z_through_its_centre_even_a_hair_under_its_radius_is_built_sound_and_of_the_arithmetic_s_volume()
 {
    let ball = ball_about(DVec3::new(1.0, 0.0, 1.0).normalize());
    for (radius, from, hair) in shafts() {
        let shaft = shaft_along_z_from(&ball, radius, from);
        let exactly = ball_and_shaft(radius, from);
        let laid = ball_and_shaft(RADIUS, from);
        let within = EXACT * (BALL + laid[0]);
        for (index, (way, built)) in booleans(&ball, &shaft).into_iter().enumerate() {
            let what = format!(
                "the ball turned at a slant {way} a shaft along z through its centre of radius \
                 {radius} from z = {from}"
            );
            let volumes = if hair {
                vec![exactly[index], laid[index]]
            } else {
                vec![exactly[index]]
            };
            if from > -RADIUS && matches!(index, 0 | 2) {
                sound_or_declined(built, &volumes, within, &what);
            } else {
                built_sound_holding_one_of(built, &volumes, within, &what);
            }
        }
    }
}

/// The ball turned about x, and each of [`shafts`]. Those of 1 and 3 meet
/// the sphere along circles far from its chart's poles, and are built as
/// beside the ball turned at a slant. The rest meet it along its equator
/// about z, or a few ten-thousandths either side of it, which runs through
/// the poles of its chart about x: nearer them than [`FRONT`] lays a circle,
/// no trace of the sphere reads the circle and the drawing leaves the face
/// open, and each boolean is declined `Unsupported` rather than built so.
#[test]
fn a_ball_turned_about_x_joined_with_or_cut_by_a_shaft_along_z_through_its_centre_is_declined_where_the_rims_pass_near_its_chart_s_poles_and_built_further_off()
 {
    let ball = ball_about(DVec3::X);
    let beyond = (1.0 - FRONT * FRONT).sqrt();
    for (radius, from, _) in shafts() {
        let shaft = shaft_along_z_from(&ball, radius, from);
        let exactly = ball_and_shaft(radius, from);
        for (index, (way, built)) in booleans(&ball, &shaft).into_iter().enumerate() {
            let what = format!(
                "the ball turned about x {way} a shaft along z through its centre of radius \
                 {radius} from z = {from}"
            );
            if radius / RADIUS > beyond {
                assert_eq!(
                    built.map(|body| body.volume()),
                    Err(Declined::Unsupported),
                    "{what}: its rims within {}° of the chart's poles",
                    FRONT.asin().to_degrees()
                );
            } else {
                built_sound_holding_one_of(
                    built,
                    &[exactly[index]],
                    EXACT * (BALL + exactly[0]),
                    &what,
                );
            }
        }
    }
}

/// The ball about z cut through its pole at (0, 0, 5) by a plane leaning 45°
/// towards x, keeping the side holding (0, 0, −5), and bored along z, as the
/// first criterion builds it: the bore's wall meets the sphere along the
/// circle z = −√24, a rim of the sphere about z. Then a slab 40 by 40 and 20
/// thick whose top stands on the plane square to z 5 and 15 hundred-millionths
/// above that rim — a hair, under [`Scale::HAIR`] of the tolerance the two
/// are read at, which decision 12 lays onto a rim of a sphere's own axis —
/// joined either way, the bored ball cut by it and it by the bored ball. Each
/// built, exact, its triangles closed and uncrossed, holding what the
/// arithmetic leaves, within the ring a few hairs wide the slab's top takes
/// off the ball's foot — or declined; never a crossed body. What it leaves
/// is read with the slab as drawn, or with its top laid down the hair onto
/// the rim, as decision 12 lays it beside a ball turned about z and bored
/// so: the rim is about z whatever axis the sphere's chart was laid about,
/// and the slab is then 20 less the hair thick.
#[test]
fn a_ball_cut_at_a_slant_through_its_pole_and_bored_along_z_joined_with_or_cut_by_a_slab_whose_top_stands_a_hair_above_the_bore_s_rim_is_built_sound_and_of_the_arithmetic_s_volume_or_declined()
 {
    let normal = leaning(DVec3::Z, 45.0, DVec3::X);
    let cut = ball_about(DVec3::Z)
        .difference(&Side::Rest.slab(normal, RADIUS * normal.dot(DVec3::Z)))
        .expect("the cut, which #938 builds");
    let bored = cut
        .difference(&bore_along_z(&cut))
        .expect("the bore, which the first criterion builds");
    let kept = Side::Rest.bored(45.0);
    let slab = 40.0 * 40.0 * 20.0;
    for hair in [5e-8, 15e-8] {
        let top = -bore_reach() + hair;
        let below = bored
            .tool_raised(
                &exact(Contour::rectangle(DVec2::splat(-20.0), DVec2::splat(20.0))),
                Frame {
                    origin: DVec3::Z * (top - 20.0),
                    u: DVec3::X,
                    v: DVec3::Y,
                },
                DVec3::Z * 20.0,
            )
            .expect("the slab raises");
        let laid = 40.0 * 40.0 * (20.0 - hair);
        for ((way, built), volumes) in booleans(&bored, &below).into_iter().zip([
            [kept + slab, kept + laid],
            [kept + slab, kept + laid],
            [kept, kept],
            [slab, laid],
        ]) {
            let what = format!(
                "the ball cut at a slant through its pole and bored along z {way} a slab whose \
                 top stands {hair:e} above the bore's rim"
            );
            sound_or_declined(built, &volumes, EXACT * volumes[0].max(BALL), &what);
        }
    }
}

/// A rod of radius 1 along x, from x = −15 to −8, a tool of `part`: its axis
/// the line through the ball's centre along x, its wall clear of the ball.
fn rod_along_x(part: &Body) -> Body {
    part.tool_raised(
        &exact(Contour {
            corners: vec![DVec2::X],
            runs: vec![Run::Round {
                center: DVec2::ZERO,
                turn: TAU,
            }],
        }),
        Frame {
            origin: DVec3::X * -15.0,
            u: DVec3::Y,
            v: DVec3::Z,
        },
        DVec3::X * 7.0,
    )
    .expect("the rod raises")
}

/// The ball about z joined to a rod along x standing clear of it, whose
/// axis runs through the ball's centre, then cut through the ball's pole at
/// (0, 0, 5) by a plane leaning 20°, 45° and 70° towards y, either side
/// kept, as #938 cuts a ball alone: the rod meets nothing of the ball, and
/// leaves the ball's sphere free to be read about the cut circle's axis.
/// Each cut built, sound and holding what the arithmetic leaves: the side
/// holding (0, 0, −5) and the rod, which the plane leaves whole, or the cap,
/// the slab taking the rod with the rest.
#[test]
fn a_ball_beside_a_rod_whose_axis_runs_through_its_centre_cut_at_a_slant_through_its_pole_is_built_sound_and_of_the_arithmetic_s_volume()
 {
    let ball = ball_about(DVec3::Z);
    let part = ball
        .union(&rod_along_x(&ball))
        .expect("a rod clear of the ball joins it");
    let rod = 7.0 * PI;
    for tilt in TILTS {
        let normal = leaning(DVec3::Z, tilt, DVec3::Y);
        let offset = RADIUS * normal.dot(DVec3::Z);
        for side in SIDES {
            let what = format!(
                "the ball about z beside a rod along x through its centre's line, cut through \
                 its pole by a plane leaning {tilt}° towards y, keeping its {side:?}"
            );

            let body = part
                .difference(&side.slab(normal, offset))
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

            let (kept, curved) = match side {
                Side::Cap => (side.holds(tilt), side.sphere(tilt)),
                Side::Rest => (side.holds(tilt) + rod, side.sphere(tilt) + 14.0 * PI),
            };
            built_and_drawn(&body, kept, curved, &what);
        }
    }
}

/// The half ball z ≥ 0 turned half a turn about x, and the same half ball
/// turned half a turn about y: each one's flat circle on z = 0 runs through
/// the poles of the other's chart, so neither sphere can be read about the
/// other's axis. Joined either way, and each cut by the other: the half
/// ball, or nothing — built, exact, its triangles closed and uncrossed and
/// holding that, or declined; never short of a cap.
#[test]
fn a_half_ball_turned_about_x_joined_with_or_cut_by_itself_turned_about_y_is_built_sound_and_of_the_arithmetic_s_volume_or_declined()
 {
    let about_x = turned_about(DVec3::X, DVec3::Y, PI);
    let about_y = turned_about(DVec3::Y, -DVec3::X, PI);
    for ((way, built), volume) in
        booleans(&about_x, &about_y)
            .into_iter()
            .zip([BALL / 2.0, BALL / 2.0, 0.0, 0.0])
    {
        let what =
            format!("the half ball z ≥ 0 turned about x {way} the same half ball turned about y");
        sound_or_declined(built, &[volume], EXACT * BALL, &what);
    }
}

/// Holds that a boolean built a body that is exact, its triangles closed
/// and uncrossed, holding within `within` of one of `volumes`.
fn built_sound_holding_one_of(
    built: Result<Body, Declined>,
    volumes: &[f64],
    within: f64,
    what: &str,
) {
    let body = built.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    sound_or_declined(Ok(body), volumes, within, what);
}

/// `part`, holding `held`, with a flat face on the plane `normal` · p =
/// `offset` and its matter behind it, against a slab 40 by 40 and 10 thick
/// parallel to that face, beyond it: standing 2, 5 and 15 of the tolerance
/// the two are read at off it, under [`Scale::HAIR`], clear of `part` or
/// taking in the layer that deep behind the face, `layer` of it. Joined
/// either way, `part` cut by the slab and the slab by `part`: each built,
/// exact, its triangles closed and uncrossed, holding what the arithmetic
/// leaves with the slab as drawn, or laid onto the face's rim on the sphere
/// by decision 12 — which reads that circle about its own axis, through the
/// sphere's centre, as it reads a latitude — the slab's near face then on
/// the part's, the slab the hair thicker or thinner, or the part moved the
/// hair onto the slab whole.
fn beside_a_slab_a_hair_off(
    part: &Body,
    held: f64,
    (normal, offset): (DVec3, f64),
    layer: impl Fn(f64) -> f64,
    part_is: &str,
) {
    let slab = 40.0 * 40.0 * 10.0;
    let reach = |body: &Body| {
        let (low, high) = body.bounds().expect("a body has a box");
        low.abs().max(high.abs()).max_element()
    };
    for hair in [2.0, 5.0, 15.0] {
        for (stands, off) in [("clear of it", 1.0), ("into it", -1.0)] {
            let tool = slab_along(normal, offset);
            let eps = Scale::of(reach(part).max(reach(&tool))).eps();
            let off = off * hair * eps;
            let tool = slab_along(normal, offset + off);
            let taken = if off < 0.0 { layer(-off) } else { 0.0 };
            let laid = 40.0 * 40.0 * (10.0 + off);
            for ((way, built), volumes) in booleans(part, &tool).into_iter().zip([
                [held + slab - taken, held + laid, held + slab],
                [held + slab - taken, held + laid, held + slab],
                [held - taken, held, held],
                [slab - taken, laid, slab],
            ]) {
                let what = format!(
                    "{part_is} {way} a slab parallel to its face {hair} tolerances off it, {stands}"
                );
                built_sound_holding_one_of(built, &volumes, EXACT * volumes[2].max(BALL), &what);
            }
        }
    }
}

/// The ball about z cut by a plane at a slant off its poles, 2 from its
/// centre along a normal leaning 45° from z towards x, keeping the side
/// holding its centre: its flat face rimmed by the circle of radius √21 the
/// plane cuts the sphere along (#805), of another axis than the sphere's
/// chart. A slab parallel to the cut a hair off it, joined and cut either
/// way, holds what [`beside_a_slab_a_hair_off`] says.
#[test]
fn a_ball_cut_at_a_slant_joined_with_or_cut_by_a_slab_parallel_to_the_cut_a_hair_off_it_is_built_sound_and_holds_the_matter_as_drawn_or_laid_onto_the_cut()
 {
    let normal = leaning(DVec3::Z, 45.0, DVec3::X);
    let cut = ball_about(DVec3::Z)
        .difference(&slab_along(normal, 2.0))
        .expect("a plane at a slant cuts the ball");
    beside_a_slab_a_hair_off(
        &cut,
        BALL - cap(3.0),
        (normal, 2.0),
        |deep| cap(3.0 + deep) - cap(3.0),
        "the ball cut at a slant",
    );
}

/// The ball turned half a turn about z from x, the half y ≥ 0, and a
/// quarter of a turn, x ≥ 0 and y ≥ 0: their flat face on y = 0 is rimmed by
/// a meridian of the sphere's chart, a circle of the axis y through its
/// centre. A slab parallel to that face a hair off it, joined and cut either
/// way, holds what [`beside_a_slab_a_hair_off`] says, the layer behind the
/// face the disc or the half disc of radius √(r² − y²) summed over its
/// depth.
#[test]
fn a_ball_turned_part_way_joined_with_or_cut_by_a_slab_parallel_to_its_flat_face_a_hair_off_it_is_built_sound_and_holds_the_matter_as_drawn_or_laid_onto_the_face()
 {
    for (turn, share, part_is) in [
        (PI, 0.5, "the half ball turned about z"),
        (PI / 2.0, 0.25, "the quarter ball turned about z"),
    ] {
        let part = turned_about(DVec3::Z, DVec3::X, turn);
        beside_a_slab_a_hair_off(
            &part,
            BALL * share,
            (-DVec3::Y, 0.0),
            |deep| 2.0 * share * PI * (RADIUS * RADIUS * deep - deep.powi(3) / 3.0),
            part_is,
        );
    }
}

/// A ball of `radius` about `centre`, turned whole about the line through
/// `centre` along `axis`, its sphere's chart laid about that line.
fn ball_at(centre: DVec3, axis: DVec3, radius: f64) -> Body {
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
        origin: centre,
        u: axis.any_orthonormal_vector(),
        v: axis,
    };
    let along_v = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    Body::default()
        .tool_turned(&profile, frame, &Turn::of(along_v, TAU, 0.0, &profile))
        .expect("the exact kernel turns the ball")
}

/// The ball about z, and a ball of radius 3 about (3, 0, 0) turned about x:
/// the line through the second's centre along its axis runs through the
/// first's centre, so the two meet along the circle x = 25/6 of radius
/// √(25 − (25/6)²) about that line, far from the poles of either chart. The
/// lens they share holds π (5 + 3 − 3)² (3² + 2·3·3 − 3·3² + 2·3·5 + 6·3·5 −
/// 3·5²) / (12·3) = 125π/4.
fn ball_and_a_ball_about_an_axis_through_its_centre() -> (Body, Body, f64) {
    let first = ball_about(DVec3::Z);
    let second = ball_at(DVec3::X * 3.0, DVec3::X, 3.0);
    (first, second, 125.0 * PI / 4.0)
}

/// [`ball_and_a_ball_about_an_axis_through_its_centre`], joined either way,
/// and the second cut by the first: each built, exact, its triangles closed
/// and uncrossed, holding the two balls less the lens, and the smaller ball
/// less it — the second read about neither the first's axis nor any but
/// its own, whose line holds both centres.
#[test]
fn a_ball_and_a_ball_about_an_axis_through_its_centre_join_and_one_is_cut_by_the_other_along_the_circle_they_cross_on()
 {
    let (first, second, lens) = ball_and_a_ball_about_an_axis_through_its_centre();
    let smaller = 36.0 * PI;
    for (way, built, volume) in [
        ("joined with", first.union(&second), BALL + smaller - lens),
        (
            "joined to, the other first,",
            second.union(&first),
            BALL + smaller - lens,
        ),
        ("cut from", second.difference(&first), smaller - lens),
    ] {
        let what = format!("the ball about z {way} a ball of 3 about (3, 0, 0) turned about x");
        built_sound_holding_one_of(built, &[volume], EXACT * BALL, &what);
    }
}

/// [`ball_and_a_ball_about_an_axis_through_its_centre`], the first cut by
/// the second: built, exact, its triangles closed and uncrossed, holding the
/// ball less the lens — or declined; never a crossed body.
#[test]
fn a_ball_cut_by_a_ball_about_an_axis_through_its_centre_is_built_sound_and_of_the_arithmetic_s_volume_or_declined()
 {
    let (first, second, lens) = ball_and_a_ball_about_an_axis_through_its_centre();

    let built = first.difference(&second);

    sound_or_declined(
        built,
        &[BALL - lens],
        EXACT * BALL,
        "the ball about z cut by a ball of 3 about (3, 0, 0) turned about x",
    );
}

/// The ball about z cut through its pole at (0, 0, 5) by a plane leaning 45°
/// towards x, keeping the cap, its sphere read about the cut circle's axis
/// (#938), and a ball of its centre and radius turned about that axis tilted
/// 10, 30 and 100 billionths of a radian towards y: a chart a hair off the
/// cut's, whose sphere the boolean reads about the cut's axis. Joined either
/// way and each cut by the other: built, sound and holding the ball, the
/// ball, nothing, and the ball less the cap.
#[test]
fn a_ball_cut_at_a_slant_through_its_pole_joined_with_or_cut_by_a_ball_of_its_centre_and_radius_turned_about_an_axis_a_hair_off_the_cut_s_is_built_sound_and_of_the_arithmetic_s_volume()
 {
    let normal = leaning(DVec3::Z, 45.0, DVec3::X);
    let cut = ball_about(DVec3::Z)
        .difference(&Side::Cap.slab(normal, RADIUS * normal.dot(DVec3::Z)))
        .expect("the cut, which #938 builds");
    for tilt in [1e-8, 3e-8, 1e-7] {
        let other = ball_about((normal + DVec3::Y * tilt).normalize());
        for ((way, built), kept) in booleans(&cut, &other).into_iter().zip([
            Some((BALL, 100.0 * PI)),
            Some((BALL, 100.0 * PI)),
            None,
            Some((Side::Rest.holds(45.0), Side::Rest.sphere(45.0))),
        ]) {
            let what = format!(
                "the ball cut at a slant through its pole, keeping its cap, {way} a ball turned \
                 about the cut's axis tilted {tilt:e}"
            );
            let body = built.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
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

/// The ball turned about x, and the ball of 3 about (3, 0, 0) turned about x
/// too: one axis, the rim they cross on, x = 25/6, a latitude of both,
/// falling a hair from a row of the smaller's grid. Joined either way and
/// each cut by the other: built, exact, its triangles closed and uncrossed,
/// holding the two balls less their lens, the larger less it, and the
/// smaller less it — the drawing leaving out the row a hair from the rim
/// rather than a sliver of strip beside it.
#[test]
fn a_ball_cut_by_a_smaller_ball_on_its_own_axis_whose_rim_falls_a_hair_from_a_row_of_its_grid_is_built_sound_and_of_the_arithmetic_s_volume()
 {
    let (_, _, lens) = ball_and_a_ball_about_an_axis_through_its_centre();
    let first = ball_about(DVec3::X);
    let second = ball_at(DVec3::X * 3.0, DVec3::X, 3.0);
    let smaller = 36.0 * PI;
    for ((way, built), volume) in booleans(&first, &second).into_iter().zip([
        BALL + smaller - lens,
        BALL + smaller - lens,
        BALL - lens,
        smaller - lens,
    ]) {
        let what = format!("the ball about x {way} a ball of 3 about (3, 0, 0) turned about x");
        built_sound_holding_one_of(built, &[volume], EXACT * BALL, &what);
    }
}

/// A shaft of `radius` along z from z = −10 up to the ball's centre, a tool
/// of `part`.
fn shaft_up_to_the_centre(part: &Body, radius: f64) -> Body {
    part.tool_raised(
        &exact(Contour {
            corners: vec![DVec2::X * radius],
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
        DVec3::Z * 10.0,
    )
    .expect("the shaft raises")
}

/// A shaft up to the ball's centre, its top on the ball's equator plane,
/// 5, 10 and 20 hundred-millionths under the ball's radius, and the ball
/// turned half a turn about z, the half y ≥ 0: the sphere crosses the
/// shaft's wall a thousandth or so under its equator, and the half ball's
/// meridian arcs end there, a ten-thousandth of a radian or two from the
/// equator's row of the sphere's grid. The arcs keep that row, which the
/// faces either side of them are cut at. Joined either way, and each cut by
/// the other: built, exact, its triangles closed and uncrossed, holding what
/// the arithmetic leaves — at the shaft's radius, or at the ball's for a
/// shaft laid onto the touch.
#[test]
fn a_half_ball_on_a_shaft_s_top_a_hair_wider_than_it_whose_meridian_arcs_end_a_hair_from_a_row_of_its_grid_is_drawn_closed()
 {
    let half = turned_about(DVec3::Z, DVec3::X, PI);
    let leaves = |radius: f64| {
        let [shaft, shared] = shaft_and_its_share_of_the_ball(radius, 0.0);
        let shared = shared / 2.0;
        [
            shaft + BALL / 2.0 - shared,
            shaft + BALL / 2.0 - shared,
            shaft - shared,
            BALL / 2.0 - shared,
        ]
    };
    for hair in [5e-8, 1e-7, 2e-7] {
        let radius = RADIUS - hair;
        let shaft = shaft_up_to_the_centre(&half, radius);
        let [exactly, laid] = [leaves(radius), leaves(RADIUS)];
        for (index, (way, built)) in booleans(&shaft, &half).into_iter().enumerate() {
            let what = format!(
                "a shaft {hair:e} under the ball's radius up to its centre {way} the half ball"
            );
            built_sound_holding_one_of(
                built,
                &[exactly[index], laid[index]],
                EXACT * (BALL + laid[0]),
                &what,
            );
        }
    }
}
