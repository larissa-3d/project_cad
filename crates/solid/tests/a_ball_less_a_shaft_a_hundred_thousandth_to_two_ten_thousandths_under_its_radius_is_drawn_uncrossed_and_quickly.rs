//! A shaft along z through a ball's centre, a hundred-thousandth to two
//! ten-thousandths under its radius, leaves a ring between its wall and the
//! sphere thicker than #1137 leaves undrawn — 400 of the kernel's tolerances
//! at the cut's scale and a hundredth of the drawing's tolerance, 4e-6 for
//! the ball of 5 cut by a shaft reaching 10 — which no decision lays onto
//! the touch. The ball less such a shaft, through it or ending anywhere in
//! the band, has that ring drawn, uncrossed, in the time its neighbours
//! take, whatever axis the ball was turned about.
//!
//! Closes #1144.
//! - The ball of 5 about z and at the slant (1, 0, 1)/√2, less a shaft along
//!   z 1e-5, 1e-4 and 2e-4 under its radius, through it and ending 1e-7 and
//!   1e-3 inside the band: built, exact, closed, uncrossed, the arithmetic's
//!   volume, drawn within the drawing's tolerance, each cut in under a second
//!   in release —
//!   `a_ball_turned_about_z_or_at_a_slant_less_a_shaft_a_hundred_thousandth_to_two_ten_thousandths_under_its_radius_through_it_or_ending_inside_the_band_is_built_exact_closed_uncrossed_of_the_arithmetic_s_volume_and_drawn_within_the_drawing_s_tolerance_each_cut_in_under_a_second`.
//!   Read as: the ball a half disc of radius 5 turned whole about the line
//!   through the origin along z, or along (1, 0, 1)/√2, its sphere's chart
//!   laid about that line, as #956's, #1109's and #1137's balls are; the
//!   shaft a round about the z axis of radius b, 5 less 1e-5, 1e-4 or 2e-4 —
//!   the lengths themselves, a thousand to twenty thousand of the kernel's
//!   tolerances at the reach of 10, not a count of them; a where the shaft's
//!   wall meets the sphere, √(5² − b²); through it, the shaft from z = −10
//!   to 10; ending 1e-7 and 1e-3 inside the band, the shaft from the plane
//!   z = −(a − 1e-7) or −(a − 1e-3) up to z = 10, as #1137's band
//!   placements, and from the other end too, from z = −10 up to the plane
//!   z = a − 1e-7 or a − 1e-3, as #1137 takes both ends: the ball at the
//!   slant is not its own mirror across its equator about z; a shaft with
//!   both ends inside the band is not taken, the criterion naming none;
//!   less, the ball cut by the shaft;
//!   built, never declined; exact, the exact kernel computed the step;
//!   closed and uncrossed, its triangles so by `soundness`; the arithmetic's
//!   volume, within a billionth of the matter in play, the ball and the
//!   shaft, of the ball less the slices of it the shaft takes — the shaft's
//!   whole disc within a of the equator, the ball's π (5² − z²) beyond — with
//!   the shaft as drawn and never laid onto the touch, which #1109 and #1137
//!   allowed for shafts a hair or a few tens of tolerances under: laid there,
//!   the ring would be gone, where the issue asks it drawn; drawn within the
//!   drawing's tolerance, as #1137 and #1138 read it — every triangle of a
//!   curved face within [`SAG`] of the drawing's tolerance of its surface at
//!   its centre and the middles of its sides, the matter the triangles
//!   enclose within [`SPREAD`] of that tolerance over their area — and the
//!   ring drawn, not left out, as #1137 reads the rings it still draws: the
//!   drawing not empty, and triangles on the ball's sphere and on the
//!   shaft's wall over the band, from where the two meet or the end plane
//!   stands up to where they meet; each cut in under a second in release,
//!   each cut timed alone on the wall clock, once, the shaft raised before
//!   and the checks run after, and held under [`QUICK`] in the profile the
//!   tests run in, as #1137's are. The cut draws the body as it builds it,
//!   so the time is that of the drawing too. That profile builds `cao_solid`
//!   and every dependency at `opt-level` 2 (the workspace's `Cargo.toml`):
//!   it is no faster than release, so a cut held under a second there is
//!   under a second in release
//! - #956's, #1109's and #1137's tests keep their bodies — no test: they are
//!   the tests of `spheres_of_one_centre_and_radius_are_one_surface_whatever_their_axes.rs`,
//!   of
//!   `a_shaft_a_hair_under_a_ball_s_radius_its_end_inside_the_band_is_laid_onto_the_touch_whatever_axis_the_ball_was_turned_about.rs`
//!   and of
//!   `a_ball_less_a_shaft_a_few_tens_of_tolerances_under_its_radius_ending_inside_the_band_is_drawn_uncrossed_and_quickly.rs`,
//!   those #956 added to `src/brep/relation/tests/round.rs`,
//!   `src/brep/selection/tests/round.rs`,
//!   `src/brep/tessellation/sampling/tests/rows.rs` and
//!   `crates/part/tests/the_exact_kernel_turns_the_part.rs`, those #1109
//!   added to `src/brep/tessellation/sampling/axis/tests.rs`,
//!   `src/brep/tessellation/sampling/tests/conic.rs` and
//!   `src/brep/tessellation/sampling/tests/crossed.rs`, and those #1137 added
//!   to `src/brep/tessellation/skins/tests.rs`; running them unchanged is the
//!   answer

use std::f64::consts::{PI, TAU};
use std::time::{Duration, Instant};

use cao_solid::brep::Scale;
use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{Triangle, closed, enclosed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Loop};
use glam::{DVec2, DVec3};

/// How far the matter a body holds may stand from the arithmetic's, as a
/// share of the matter in play: the kernel's billionth.
const EXACT: f64 = 1e-9;

/// The radius of every ball here, about the origin.
const RADIUS: f64 = 5.0;

/// The ball of [`RADIUS`].
const BALL: f64 = 4.0 * PI / 3.0 * RADIUS * RADIUS * RADIUS;

/// How far under the ball's radius each shaft stands: the criterion's
/// lengths, a thousand to twenty thousand of the kernel's tolerances at the
/// reach of [`OUT`].
const UNDERS: [f64; 3] = [1e-5, 1e-4, 2e-4];

/// How far inside the band each shaft ending there has its end plane, from
/// the circle its wall meets the sphere along: a hair, where the plane's
/// circle on the sphere and the shaft's rim are one, and a thousandth, where
/// they stand a few millionths apart.
const INSIDES: [f64; 2] = [1e-7, 1e-3];

/// How far along z, either way, a shaft standing past the ball reaches.
const OUT: f64 = 10.0;

/// How long one cut may take on the wall clock, the shaft raised before it
/// and the checks run after: the criterion's second.
const QUICK: Duration = Duration::from_secs(1);

/// How far the triangles of an exact body may stand from its surfaces, as a
/// share of its reach: `body/drawn.rs`'s `DRAWN`, which no public constant
/// gives.
const DRAWN: f64 = 2e-4;

/// How many of the drawing's tolerances a triangle drawn on a curved face
/// may stand off that face's surface, at its centre or the middle of a side:
/// #1138's. The drawing holds its chords to the tolerance, not the middles of
/// its triangles, and the ball turned at a slant, alone, is drawn some 1.4
/// of it off there.
const SAG: f64 = 1.5;

/// How many of the drawing's tolerances, spread over every triangle's area,
/// the matter the triangles enclose may stand off the matter the body holds:
/// one, as though every triangle stood a whole tolerance off its surface on
/// the same side. #1138's.
const SPREAD: f64 = 1.0;

/// Closer than this to a surface, a corner of a triangle stands on it: a
/// two-thousandth of the drawing's tolerance at a reach of 10, and a
/// hundred times the kernel's own there. #1138's.
const ON: f64 = 1e-6;

/// The balls the criterion names, each with how it was turned.
fn balls() -> [(&'static str, DVec3); 2] {
    [
        ("about z", DVec3::Z),
        (
            "at a slant, about (1, 0, 1)/√2,",
            DVec3::new(1.0, 0.0, 1.0).normalize(),
        ),
    ]
}

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

/// A shaft of `radius` along z through the origin, from z = `from` to `to`,
/// a tool of `part`.
fn shaft_along_z(part: &Body, radius: f64, from: f64, to: f64) -> Body {
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

/// How far either side of the equator about z the wall of a shaft of
/// `radius` meets the ball's sphere: a = √(r² − b²).
fn band(radius: f64) -> f64 {
    (RADIUS * RADIUS - radius * radius).sqrt()
}

/// Where the shafts of `radius` the criterion names start and end: through
/// the ball from z = −[`OUT`] to [`OUT`]; and, for each of [`INSIDES`], from
/// the plane that far above z = −a up to [`OUT`], and from −[`OUT`] up to
/// the plane that far under z = a.
fn placements(radius: f64) -> [(f64, f64); 5] {
    let a = band(radius);
    let [hair, deeper] = INSIDES;
    [
        (-OUT, OUT),
        (-(a - hair), OUT),
        (-OUT, a - hair),
        (-(a - deeper), OUT),
        (-OUT, a - deeper),
    ]
}

/// What a shaft of `radius` along z through the ball's centre, from z =
/// `from` to `to`, holds, and what it takes of the ball: π b² (`to` −
/// `from`), and the ball's slices between, each the disc of the shaft or the
/// ball's own where that is the smaller — the shaft's within a of the
/// equator, the ball's π (r² − z²) beyond. #1109's.
fn shaft_and_its_share_of_the_ball(radius: f64, from: f64, to: f64) -> [f64; 2] {
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
    [PI * radius * radius * (to - from), below(to) - below(from)]
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

/// Why `body`, the ball less a shaft of `radius` from z = `from` to `to`,
/// does not hold what the arithmetic leaves with the shaft as drawn, if it
/// does not: within a billionth of the ball and the shaft.
fn off_the_arithmetic(body: &Body, radius: f64, from: f64, to: f64) -> Option<String> {
    let [held, taken] = shaft_and_its_share_of_the_ball(radius, from, to);
    let left = BALL - taken;
    ((body.volume() - left).abs() > EXACT * (BALL + held)).then(|| {
        format!(
            "holds {}, where the arithmetic leaves {left} with the shaft as drawn",
            body.volume()
        )
    })
}

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
/// furthest does; a curved face drawn on neither is named. #1137's.
fn drawn_off(body: &Body, what: &str) -> Vec<String> {
    if body.is_empty() {
        return vec![format!(
            "{what}: empty, where the ball less the shaft holds matter"
        )];
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
            "{what}: its triangles enclose {drawn:.9} where it holds {:.9}, over their area \
             {area:.4} at the drawing's tolerance {tolerance}",
            body.volume(),
        ));
    }
    wrong
}

/// Where the ring a shaft of `radius` leaves in `body`, between z = `low`
/// and `high`, is left out of the drawing: the drawing empty, or without a
/// triangle over the band on the ball's sphere or on the shaft's wall —
/// every corner on it and between the two heights, its middle strictly
/// between them, so that neither a cap below nor a disc at an end counts.
/// #1137's.
fn ring_left_out(body: &Body, radius: f64, low: f64, high: f64, what: &str) -> Vec<String> {
    let triangles = body.triangles();
    if triangles.is_empty() {
        return vec![format!(
            "{what}: drawn with nothing, where it holds {}",
            body.volume()
        )];
    }
    let over = |triangle: &Triangle| {
        let middle = (triangle[0] + triangle[1] + triangle[2]).z / 3.0;
        triangle
            .iter()
            .all(|corner| (low - ON..=high + ON).contains(&corner.z))
            && low + ON < middle
            && middle < high - ON
    };
    let covered = |on: &dyn Fn(DVec3) -> bool| {
        triangles
            .iter()
            .any(|triangle| over(triangle) && triangle.iter().all(|&corner| on(corner)))
    };
    let on_the_sphere = |corner: DVec3| (corner.length() - RADIUS).abs() < ON;
    let on_the_wall = |corner: DVec3| (corner.truncate().length() - radius).abs() < ON;
    [
        ("the ball's sphere", covered(&on_the_sphere)),
        ("the shaft's wall", covered(&on_the_wall)),
    ]
    .into_iter()
    .filter(|(_, covered)| !covered)
    .map(|(surface, _)| {
        format!("{what}: no triangle drawn on {surface} over the band from z = {low} to {high}")
    })
    .collect()
}

/// What the cut of the ball by each shaft is called in a failure.
fn named(about: &str, radius: f64, from: f64, to: f64) -> String {
    format!(
        "the ball of {RADIUS} turned {about} less a shaft along z through its centre of radius \
         {radius} from z = {from} to {to}"
    )
}

/// The ball about z and the ball turned at a slant, each less a shaft along
/// z through its centre each of [`UNDERS`] under its radius, at each of
/// [`placements`]: every cut built, exact, its triangles closed and
/// uncrossed, holding within a billionth of the ball and the shaft what the
/// arithmetic leaves with the shaft as drawn, drawn within the drawing's
/// tolerance with its ring drawn, and made, drawing and all, within
/// [`QUICK`]. Every cut that is not is named, with how long it took and each
/// way it is not.
#[test]
fn a_ball_turned_about_z_or_at_a_slant_less_a_shaft_a_hundred_thousandth_to_two_ten_thousandths_under_its_radius_through_it_or_ending_inside_the_band_is_built_exact_closed_uncrossed_of_the_arithmetic_s_volume_and_drawn_within_the_drawing_s_tolerance_each_cut_in_under_a_second()
 {
    let mut wrong = Vec::new();
    for (about, axis) in balls() {
        let ball = ball_about(axis);
        for under in UNDERS {
            let radius = RADIUS - under;
            let a = band(radius);
            for (from, to) in placements(radius) {
                let shaft = shaft_along_z(&ball, radius, from, to);
                let started = Instant::now();
                let built = ball.difference(&shaft);
                let took = started.elapsed();
                let what = format!("{}, cut in {took:.1?}", named(about, radius, from, to));
                if took > QUICK {
                    wrong.push(format!("{what}: past {QUICK:?}"));
                }
                let body = match built {
                    Err(declined) => {
                        wrong.push(format!("{what}: declined, {declined:?}"));
                        continue;
                    }
                    Ok(body) => body,
                };
                if let Some(why) = unsound(&body) {
                    wrong.push(format!("{what}: {why}"));
                }
                if let Some(why) = off_the_arithmetic(&body, radius, from, to) {
                    wrong.push(format!("{what}: {why}"));
                }
                wrong.extend(drawn_off(&body, &what));
                wrong.extend(ring_left_out(&body, radius, from.max(-a), to.min(a), &what));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a ball less a shaft a hundred-thousandth to two ten-thousandths under its radius, \
         through it or ending inside the band, is built exact, closed, uncrossed, of the \
         arithmetic's volume and drawn within the drawing's tolerance, each cut in under a \
         second, whatever axis the ball was turned about; not so:\n{}",
        wrong.join("\n")
    );
}
