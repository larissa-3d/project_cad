//! A shaft along z through a ball's centre, a little narrower than the ball,
//! meets its sphere along two circles either side of its equator about z.
//! Cut by the ball, the shaft keeps what stands outside the sphere, and that
//! matter thins to nothing along those circles: a knife-edge rim, where the
//! sphere and the shaft's wall meet at a shallow angle, cos⁻¹ 0.98 for a
//! shaft of 4.9 in a ball of 5. The boolean is drawn uncrossed along that rim
//! whatever axis the ball was turned about.
//!
//! Closes #1138.
//! - The shaft of 4.9 from −10 to 0 (and to 2) less the ball of 5 turned
//!   about z and at the three slants: built, exact, closed, uncrossed, the
//!   arithmetic's volume —
//!   `a_shaft_less_a_ball_a_little_wider_than_it_turned_about_z_or_at_a_slant_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume`.
//!   Read as: the shaft a round of radius 4.9 about the z axis, raised from
//!   the plane z = −10 to the plane z = 0, or z = 2; the ball of radius 5
//!   about the origin, a half disc turned whole about the line through the
//!   origin along z, (1, 0, 1)/√2, (−1, 0, 1)/√2 or (0, 1, 1)/√2, its
//!   sphere's chart laid about that line; less, the shaft cut by the ball;
//!   built, never declined; exact, the exact kernel computed the step; closed
//!   and uncrossed, its triangles so by `soundness`; the arithmetic's volume,
//!   within a billionth of the matter in play, the ball and the shaft, of
//!   the shaft's π 4.9² by its length less the slices of it inside the ball —
//!   the shaft's whole disc within a = √(5² − 4.9²), about 0.995, of the
//!   equator, the ball's π (5² − z²) beyond.
//! - #956's and #1109's tests keep their bodies — no test: they are the
//!   tests of `spheres_of_one_centre_and_radius_are_one_surface_whatever_their_axes.rs`
//!   and of
//!   `a_shaft_a_hair_under_a_ball_s_radius_its_end_inside_the_band_is_laid_onto_the_touch_whatever_axis_the_ball_was_turned_about.rs`,
//!   and those #956 added to `src/brep/relation/tests/round.rs`,
//!   `src/brep/selection/tests/round.rs`,
//!   `src/brep/tessellation/sampling/tests/rows.rs` and
//!   `crates/part/tests/the_exact_kernel_turns_the_part.rs`; running them
//!   unchanged is the answer
//!
//! Beyond the criteria: the same steps, held to the same checks, for shafts
//! of 4.6, 4.8, 4.95, 4.97 and 4.98 — the nearer the ball's radius, the
//! nearer the rim stands to the equator and the shallower the angle the
//! sphere and the wall meet at, so the thinner the band about the rim where
//! the sphere stands within a chord's sag of the wall —
//! `a_shaft_a_few_hundredths_to_a_few_tenths_narrower_than_a_ball_less_the_ball_turned_about_z_or_at_a_slant_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume`.
//! And the drawing those steps are drawn with, held to its tolerance: a row
//! of the sphere's grid kept out of the band about a knife-edge rim must not
//! leave the cap it bounds drawn coarser than the tolerance, however thin the
//! band grows. Shafts of 4.9, 4.97, 4.9995, 4.9999, 4.99999, 4.999999 and
//! 5 − 1e-7, less the ball turned at each slant, to 0 and to 2 —
//! `a_shaft_a_hair_to_a_few_hundredths_narrower_than_a_ball_less_the_ball_turned_at_a_slant_is_drawn_within_the_drawing_s_tolerance_of_its_surfaces`;
//! and #1109's own bodies, replayed here so that its file keeps its body,
//! the ball turned about (1, 0, 1)/√2 joined with or cut by each of its
//! shafts a hair under the ball's radius, ending inside the band —
//! `the_ball_turned_at_a_slant_joined_with_or_cut_by_a_shaft_a_hair_under_its_radius_ending_inside_the_band_is_drawn_within_the_drawing_s_tolerance_of_its_surfaces`.
//! Each triangle of a curved face within [`SAG`] of the drawing's tolerance
//! of its surface, and the matter the triangles enclose within [`SPREAD`] of
//! it over their area.

use std::f64::consts::{PI, TAU};

use cao_solid::brep::Scale;
use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Declined, Loop};
use glam::{DVec2, DVec3};

/// How far the matter a body holds may stand from the arithmetic's, as a
/// share of the matter in play: the kernel's billionth.
const EXACT: f64 = 1e-9;

/// The radius of the ball, about the origin.
const RADIUS: f64 = 5.0;

/// The ball of [`RADIUS`].
const BALL: f64 = 4.0 * PI / 3.0 * RADIUS * RADIUS * RADIUS;

/// The radius of the criterion's shaft, a little under the ball's.
const SHAFT: f64 = 4.9;

/// Where the shaft starts, under the ball.
const FROM: f64 = -10.0;

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
/// line, its poles 5 along it either way. #956's.
fn ball_about(axis: DVec3) -> Body {
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

/// A shaft of `radius` along z through the origin, from z = [`FROM`] to
/// `to`, a tool of `part`.
fn shaft_along_z(part: &Body, radius: f64, to: f64) -> Body {
    shaft_between(part, radius, FROM, to)
}

/// A shaft of `radius` along z through the origin, from z = `from` to `to`,
/// a tool of `part`.
fn shaft_between(part: &Body, radius: f64, from: f64, to: f64) -> Body {
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
        DVec3::Z * (to - from),
    )
    .expect("the shaft raises")
}

/// What the shaft of `radius` from z = [`FROM`] to `to` holds, and what the
/// ball takes of it: π b² (`to` − [`FROM`]), and its slices inside the ball,
/// each the shaft's disc or the ball's where that is the smaller — the
/// shaft's within a of the equator, a = √(r² − b²), the ball's π (r² − z²)
/// beyond. #1109's.
fn shaft_and_its_share_of_the_ball(radius: f64, to: f64) -> [f64; 2] {
    let reach = (RADIUS * RADIUS - radius * radius).sqrt();
    let round = |low: f64, high: f64| {
        PI * (RADIUS * RADIUS * (high - low) - (high.powi(3) - low.powi(3)) / 3.0)
    };
    let below = |height: f64| {
        let height = height.clamp(-RADIUS, RADIUS);
        round(-RADIUS, height.min(-reach))
            + PI * radius * radius * (height.clamp(-reach, reach) + reach)
            + round(reach, height.max(reach))
    };
    [PI * radius * radius * (to - FROM), below(to) - below(FROM)]
}

/// Why `body` is not sound, if it is not: not exact, or its triangles open
/// or crossing. #1109's.
fn unsound(body: &Body) -> Option<String> {
    if !body.is_exact() {
        return Some("not exact".to_string());
    }
    let triangles = body.triangles();
    if let Err(open) = closed(&triangles) {
        return Some(format!("its triangles open, {open:?}"));
    }
    if let Err(crossing) = uncrossed(&triangles) {
        return Some(format!("its triangles crossing, {crossing:?}"));
    }
    None
}

/// The axes the ball is turned about: z, the shaft's own, and the three
/// slants, each named as a message reads it.
fn axes() -> [(&'static str, DVec3); 4] {
    [
        ("about z", DVec3::Z),
        ("about (1, 0, 1)/√2", DVec3::new(1.0, 0.0, 1.0).normalize()),
        (
            "about (−1, 0, 1)/√2",
            DVec3::new(-1.0, 0.0, 1.0).normalize(),
        ),
        ("about (0, 1, 1)/√2", DVec3::new(0.0, 1.0, 1.0).normalize()),
    ]
}

/// Every way the shaft of `radius` from z = [`FROM`] to `to`, less `ball`,
/// turned `about`, is not what it should be: declined, not exact, its
/// triangles open or crossing, or holding more than a billionth of the ball
/// and the shaft off what the arithmetic leaves.
fn ways_wrong(ball: &Body, about: &str, radius: f64, to: f64) -> Vec<String> {
    let what = format!(
        "a shaft of {radius} along z from z = {FROM} to {to} less the ball of {RADIUS} turned \
         {about}"
    );
    let [shaft, shared] = shaft_and_its_share_of_the_ball(radius, to);
    let left = shaft - shared;
    let within = EXACT * (BALL + shaft);
    let mut wrong = Vec::new();
    match shaft_along_z(ball, radius, to).difference(ball) {
        Err(declined) => wrong.push(format!("{what}: declined, {declined:?}")),
        Ok(body) => {
            if let Some(why) = unsound(&body) {
                wrong.push(format!("{what}: {why}"));
            }
            if (body.volume() - left).abs() > within {
                wrong.push(format!(
                    "{what}: holds {}, where the arithmetic leaves {left}",
                    body.volume()
                ));
            }
        }
    }
    wrong
}

/// The shaft from z = −10 to the ball's centre, or past it to z = 2, cut by
/// the ball turned about z, the shaft's own axis, and about each of the
/// three slants: where the shaft's wall meets the sphere, the circle z =
/// −a, and z = a for the shaft to 2, is a parallel of the sphere's chart
/// beside the first and runs across its parallels beside the others. Each
/// built, exact, its triangles closed and uncrossed, holding what the
/// arithmetic leaves within a billionth of the ball and the shaft. Every
/// step that is not is named, with each way it is not.
#[test]
fn a_shaft_less_a_ball_a_little_wider_than_it_turned_about_z_or_at_a_slant_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let mut wrong = Vec::new();
    for (about, axis) in axes() {
        let ball = ball_about(axis);
        for to in [0.0, 2.0] {
            wrong.extend(ways_wrong(&ball, about, SHAFT, to));
        }
    }
    assert!(
        wrong.is_empty(),
        "a shaft less a ball a little wider than it is built exact, closed, uncrossed and of \
         the arithmetic's volume whatever axis the ball was turned about; not so:\n{}",
        wrong.join("\n")
    );
}

/// The same steps for shafts of 4.6, 4.8, 4.95, 4.97 and 4.98, from z = −10
/// to the ball's centre or past it to z = 2, less the ball turned about z
/// and about each of the three slants: the nearer the shaft's radius to the
/// ball's, the nearer its rims stand to the equator, the shallower the angle
/// the sphere meets its wall at, and the thinner the band about each rim
/// where the sphere stands within a chord's sag of the wall. Each built,
/// exact, its triangles closed and uncrossed, holding what the arithmetic
/// leaves within a billionth of the ball and the shaft. Every step that is
/// not is named, with each way it is not.
#[test]
fn a_shaft_a_few_hundredths_to_a_few_tenths_narrower_than_a_ball_less_the_ball_turned_about_z_or_at_a_slant_is_built_exact_closed_uncrossed_and_of_the_arithmetic_s_volume()
 {
    let mut wrong = Vec::new();
    for (about, axis) in axes() {
        let ball = ball_about(axis);
        for radius in [4.6, 4.8, 4.95, 4.97, 4.98] {
            for to in [0.0, 2.0] {
                wrong.extend(ways_wrong(&ball, about, radius, to));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a shaft a few hundredths to a few tenths narrower than a ball, less the ball, is built \
         exact, closed, uncrossed and of the arithmetic's volume whatever axis the ball was \
         turned about; not so:\n{}",
        wrong.join("\n")
    );
}

/// How far the triangles of an exact body may stand from its surfaces, as a
/// share of its reach: `body/drawn.rs`'s `DRAWN`, which no public constant
/// gives.
const DRAWN: f64 = 2e-4;

/// How many of the drawing's tolerances a triangle drawn on a curved face
/// may stand off that face's surface, at its centre or the middle of a side.
/// The drawing holds its chords to the tolerance, not the middles of its
/// triangles: the ball turned at a slant, alone, is drawn some 1.4 of it
/// off there, and every body below was drawn within 1.43 before the knife
/// edge's band (#1138). Half a tolerance past one leaves that room and no
/// more.
const SAG: f64 = 1.5;

/// How many of the drawing's tolerances, spread over every triangle's area,
/// the matter the triangles enclose may stand off the matter the body holds:
/// one, as though every triangle stood a whole tolerance off its surface on
/// the same side. A cap of the sphere drawn as one fan of chords encloses a
/// hundred times that.
const SPREAD: f64 = 1.0;

/// Closer than this to a surface, a corner of a triangle stands on it: a
/// two-thousandth of the drawing's tolerance at a reach of 10, and a
/// thousand times the kernel's own.
const ON: f64 = 1e-6;

/// How far the triangles of `body` may stand from its surfaces: [`DRAWN`] of
/// the reach its bounds stand at, as [`Scale`] reads a reach.
fn tolerance(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a body holding matter has bounds");
    DRAWN * Scale::of(low.abs().max(high.abs()).max_element()).reach()
}

/// Where `body`'s drawing stands further from its surfaces than [`SAG`] of
/// the drawing's tolerance allows, or encloses further from what it holds
/// than [`SPREAD`] allows. Each curved face is read as drawn on the sphere
/// of the ball when every corner of its triangles stands on that sphere, and
/// as drawn on a wall about z when every corner stands as far from z as the
/// furthest does; a curved face drawn on neither is named. An empty body is
/// drawn with nothing, and nothing of it stands off.
fn drawn_off(body: &Body, what: &str) -> Vec<String> {
    if body.is_empty() {
        return Vec::new();
    }
    let tolerance = tolerance(body);
    let mut wrong = Vec::new();
    for face in (0..body.faces_end()).chain(body.pieces()) {
        if !body.has_face(face) || body.is_flat(face) {
            continue;
        }
        let triangles: Vec<[DVec3; 3]> = body.triangles_of(face).collect();
        let corners = || triangles.iter().flatten();
        let wall = corners()
            .map(|corner| corner.truncate().length())
            .fold(0.0, f64::max);
        let (surface, off): (String, Box<dyn Fn(DVec3) -> f64>) =
            if corners().all(|corner| (corner.length() - RADIUS).abs() < ON) {
                (
                    "the ball's sphere".to_string(),
                    Box::new(|point: DVec3| (point.length() - RADIUS).abs()),
                )
            } else if corners().all(|corner| wall - corner.truncate().length() < ON) {
                (
                    format!("a wall of radius {wall} about z"),
                    Box::new(move |point: DVec3| (point.truncate().length() - wall).abs()),
                )
            } else {
                wrong.push(format!(
                    "{what}: its face {face} is curved and drawn on neither the ball's sphere \
                     nor a wall about z"
                ));
                continue;
            };
        let worst = triangles
            .iter()
            .flat_map(|&[a, b, c]| {
                [
                    (a + b + c) / 3.0,
                    (a + b) / 2.0,
                    (b + c) / 2.0,
                    (c + a) / 2.0,
                ]
            })
            .map(|point| (off(point), point))
            .max_by(|(first, _), (second, _)| first.total_cmp(second));
        if let Some((off, point)) = worst
            && off > SAG * tolerance
        {
            wrong.push(format!(
                "{what}: its face {face}, drawn on {surface}, stands {off:.5} off it at \
                 {point:.4}, {:.2} of the drawing's tolerance {tolerance}",
                off / tolerance
            ));
        }
    }
    let triangles = body.triangles();
    let area: f64 = triangles
        .iter()
        .map(|&[a, b, c]| (b - a).cross(c - a).length() / 2.0)
        .sum();
    let drawn = enclosed(&triangles);
    if (drawn - body.volume()).abs() > SPREAD * tolerance * area {
        wrong.push(format!(
            "{what}: its triangles enclose {drawn:.6} where it holds {:.6}, {:.3} of the \
             drawing's tolerance {tolerance} over their area {area:.2}",
            body.volume(),
            (drawn - body.volume()).abs() / (tolerance * area)
        ));
    }
    wrong
}

/// Shafts a hair to a few hundredths under the ball's radius, from z = −10
/// to the ball's centre or past it to z = 2, less the ball turned about each
/// of the three slants: the nearer the shaft's radius to the ball's, the
/// nearer the rim to the equator and the shallower the angle the sphere
/// meets the wall at, down to a shaft a ten-millionth under. Each drawn with
/// every triangle of a curved face within [`SAG`] of the drawing's tolerance
/// of its surface, and enclosing what the body holds within [`SPREAD`] of
/// that tolerance over the triangles' area — never a cap of the sphere drawn
/// as one fan of chords. Every body that is not is named, with each way it
/// is not.
#[test]
fn a_shaft_a_hair_to_a_few_hundredths_narrower_than_a_ball_less_the_ball_turned_at_a_slant_is_drawn_within_the_drawing_s_tolerance_of_its_surfaces()
 {
    let mut wrong = Vec::new();
    for (about, axis) in axes().into_iter().skip(1) {
        let ball = ball_about(axis);
        for radius in [4.9, 4.97, 4.9995, 4.9999, 4.99999, 4.999999, RADIUS - 1e-7] {
            for to in [0.0, 2.0] {
                let what = format!(
                    "a shaft of {radius} along z from z = {FROM} to {to} less the ball of \
                     {RADIUS} turned {about}"
                );
                match shaft_along_z(&ball, radius, to).difference(&ball) {
                    Err(declined) => wrong.push(format!("{what}: declined, {declined:?}")),
                    Ok(body) => wrong.extend(drawn_off(&body, &what)),
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a shaft a hair to a few hundredths narrower than a ball, less the ball turned at a \
         slant, is drawn within the drawing's tolerance of its surfaces; not so:\n{}",
        wrong.join("\n")
    );
}

/// #1109's bodies: the ball turned about (1, 0, 1)/√2 and a shaft along z
/// through its centre of the ball's radius less 2, 5 and 15 of the
/// tolerance the two are read at, standing from the plane a
/// hundred-thousandth inside the band its wall meets the sphere along up
/// past the ball, from past the ball up to the other such plane, or between
/// the two — joined either way, the ball cut by the shaft and the shaft by
/// the ball. Each drawn as the shafts above are: within [`SAG`] of the
/// drawing's tolerance of its surfaces, and enclosing what it holds within
/// [`SPREAD`] of it over the triangles' area.
#[test]
fn the_ball_turned_at_a_slant_joined_with_or_cut_by_a_shaft_a_hair_under_its_radius_ending_inside_the_band_is_drawn_within_the_drawing_s_tolerance_of_its_surfaces()
 {
    let ball = ball_about(DVec3::new(1.0, 0.0, 1.0).normalize());
    let under = |reach: f64| [2.0, 5.0, 15.0].map(|hair| RADIUS - hair * Scale::of(reach).eps());
    let inside = |radius: f64| (RADIUS * RADIUS - radius * radius).sqrt() - 1e-5;
    let shafts = under(10.0)
        .into_iter()
        .flat_map(|radius| {
            [
                (radius, -inside(radius), 10.0),
                (radius, -10.0, inside(radius)),
            ]
        })
        .chain(under(RADIUS).map(|radius| (radius, -inside(radius), inside(radius))));
    let mut wrong = Vec::new();
    for (radius, from, to) in shafts {
        let shaft = shaft_between(&ball, radius, from, to);
        let steps: [(&str, Result<Body, Declined>); 4] = [
            ("joined with", ball.union(&shaft)),
            ("joined to, the shaft first,", shaft.union(&ball)),
            ("cut by", ball.difference(&shaft)),
            ("cut from", shaft.difference(&ball)),
        ];
        for (way, built) in steps {
            let what = format!(
                "the ball turned about (1, 0, 1)/√2 {way} a shaft along z through its centre of \
                 radius {radius} from z = {from} to {to}"
            );
            match built {
                Err(declined) => wrong.push(format!("{what}: declined, {declined:?}")),
                Ok(body) => wrong.extend(drawn_off(&body, &what)),
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "the ball turned at a slant joined with or cut by a shaft a hair under its radius, \
         ending inside the band, is drawn within the drawing's tolerance of its surfaces; not \
         so:\n{}",
        wrong.join("\n")
    );
}
