//! A ball cut by planes of several axes: the sphere left is bounded by
//! circles turning about two axes or three, and it is drawn whatever poles of
//! the ball's chart it holds.
//!
//! Closes #940.
//! - A ball about Y cut by x ≥ 4 then z ≥ 4: built, closed, sound, the volume
//!   the arithmetic gives —
//!   `a_ball_cut_by_two_flats_whose_planes_are_not_parallel_is_built_and_drawn_within_the_drawing_s_tolerance`
//! - The same with three flats of three axes —
//!   `a_ball_cut_by_three_flats_of_three_axes_is_built_and_drawn_within_the_drawing_s_tolerance`
//!
//! Read as: every cut built, never declined, in each order tried; the body
//! left exact and holding, to 1e-9 of it, the ball less the caps the planes
//! take, what two or three of them share counted once; listed sound, what it
//! lists itself as made of standing on its geometry; and its triangles
//! closed, uncrossed and enclosing that within the drawing's tolerance over
//! the sphere left. The issue's ball is cut both ways round. Its three flats
//! are x, y and z ≥ 4, whose caps keep apart, ≥ 3, each two of which share a
//! piece and all three of which ring a triangle of the sphere apart, and ≥ 2,
//! all three of which share one: the sphere left holds the pole at y = −5,
//! the cap about y taking the other.
//!
//! Beside the criteria: the issue's two flats moved in to 2, whose caps
//! share a piece, leaving one loop of two arcs; planes at a slant square to
//! each other, two and three, the same caps turned off the axes, none along
//! the ball's own; and the band round the ball's axis two flats at a slant
//! leave between two circles of two axes. Two pins of `body/tests.rs` that
//! held such cuts declined `Undrawn` flip into this file: the issue's case,
//! and the band. A third stays there, declined:
//! `a_ball_cut_by_two_flats_whose_circle_passes_two_degrees_by_a_pole_of_its_chart_is_declined_as_undrawn`.
//! Its face holds no pole; what refuses it is a circle two degrees by a pole
//! of the ball's chart, within the reach every layout of a sphere face in
//! that chart keeps off a pole — a limit of the chart, which a chart about
//! another axis does not have, and none of #940's criteria: #962 is to lift
//! it. The fallback's
//! own test in `brep/tessellation/outline/tests.rs` now holds that a face
//! bounded by circles alone is laid out as one bounded by a cylinder's curve
//! is, and refused for the same nearness to a pole.

use super::*;
use crate::soundness::listed;

/// The ball of radius 5 about the origin cut by each tool in turn, every cut
/// built.
fn ball_cut_by(tools: &[&Body], what: &str) -> Body {
    tools
        .iter()
        .enumerate()
        .fold(ball_about_y(), |body, (index, tool)| {
            body.difference(tool).unwrap_or_else(|declined| {
                panic!(
                    "{what}: cut {} of {} declined, {declined:?}",
                    index + 1,
                    tools.len()
                )
            })
        })
}

/// Holds what [`built_and_drawn`] holds, and that the exact body lists itself
/// as made of what its geometry stands on.
fn built_listed_and_drawn(body: &Body, kept: f64, curved: f64, what: &str) {
    built_and_drawn(body, kept, curved, what);
    let brep = body.brep().expect("an exact body");
    assert_eq!(
        listed(&brep.listing(), brep.scale().reach()),
        Ok(()),
        "{what}"
    );
}

/// The block holding all of the ball past `offset` along `axis`, one of X, Y
/// and Z.
fn block_past(axis: DVec3, offset: f64) -> Body {
    boxed(
        DVec3::splat(-10.0) + axis * (offset + 10.0),
        DVec3::splat(10.0),
    )
}

/// What three caps of the ball hold in common, their planes `offset` from
/// its centre along three directions square to each other, nothing where
/// they keep apart: over every place `a` along the first from `offset` out to
/// √(25 − 2 offset²), the piece of the disc of radius c = √(25 − a²) past
/// `offset` along both others, c² (asin(w/c) − asin(offset/c))/2
/// − offset (w − offset), w = √(c² − offset²), summed by Simpson's rule. The
/// piece vanishes at the end as the square of the distance to it, which
/// leaves the sum smooth.
fn three_caps_square_to_each_other_share(offset: f64) -> f64 {
    let squared = offset * offset;
    if 3.0 * squared >= 25.0 {
        return 0.0;
    }
    let piece = |a: f64| {
        let square = 25.0 - a * a;
        if square <= 2.0 * squared {
            return 0.0;
        }
        let (c, w) = (square.sqrt(), (square - squared).sqrt());
        square * ((w / c).asin() - (offset / c).asin()) / 2.0 - offset * (w - offset)
    };
    let (steps, from, to) = (1 << 12, offset, (25.0 - 2.0 * squared).sqrt());
    let step = (to - from) / f64::from(steps);
    let inner: f64 = (1..steps)
        .map(|index| piece(from + step * f64::from(index)) * if index % 2 == 1 { 4.0 } else { 2.0 })
        .sum();
    (piece(from) + inner + piece(to)) * step / 3.0
}

/// What the ball keeps once `count` planes square to each other, two or
/// three, each `offset` from its centre, take their caps: the ball less each
/// cap, given back what each two share, less what all three share.
fn kept_past_planes_square_to_each_other(count: u32, offset: f64) -> f64 {
    let pairs = f64::from(count * (count - 1) / 2);
    let all = if count == 3 {
        three_caps_square_to_each_other_share(offset)
    } else {
        0.0
    };
    BALL_OF_5 - f64::from(count) * cap_of_5(5.0 - offset)
        + pairs * two_caps_square_to_each_other_share(offset)
        - all
}

/// What the sphere of the ball keeps at most once a cap `offset` from its
/// centre is taken: the whole sphere less one cap, 2π·5·(5 + offset).
fn sphere_past(offset: f64) -> f64 {
    TAU * 5.0 * (5.0 + offset)
}

/// The issue's ball: cut by the block x ≥ 4, then by the block z ≥ 4, and the
/// other way round. The two caps keep apart, x² + z² ≥ 32 > 25 where both
/// would be, and the sphere left, 80π, holds both poles of the ball's chart
/// and is bounded by a whole circle about X and a whole circle about Z.
#[test]
fn a_ball_cut_by_two_flats_whose_planes_are_not_parallel_is_built_and_drawn_within_the_drawing_s_tolerance()
 {
    let (across, deep) = (block_past(DVec3::X, 4.0), block_past(DVec3::Z, 4.0));
    let kept = BALL_OF_5 - 2.0 * cap_of_5(1.0);
    for (tools, what) in [
        ([&across, &deep], "the ball cut by x ≥ 4, then by z ≥ 4"),
        ([&deep, &across], "the ball cut by z ≥ 4, then by x ≥ 4"),
    ] {
        let body = ball_cut_by(&tools, what);
        built_listed_and_drawn(&body, kept, 80.0 * PI, what);
    }
}

/// The ball cut by x ≥ 2 and z ≥ 2, in either order: the caps share the
/// piece both take, and the sphere left, holding both poles, is bounded by
/// one loop of two arcs, the one about X and the other about Z, meeting at
/// (2, ±√17, 2).
#[test]
fn a_ball_cut_by_two_flats_of_two_axes_whose_caps_overlap_is_built_and_drawn_within_the_drawing_s_tolerance()
 {
    let (across, deep) = (block_past(DVec3::X, 2.0), block_past(DVec3::Z, 2.0));
    let kept = kept_past_planes_square_to_each_other(2, 2.0);
    for (tools, what) in [
        ([&across, &deep], "the ball cut by x ≥ 2, then by z ≥ 2"),
        ([&deep, &across], "the ball cut by z ≥ 2, then by x ≥ 2"),
    ] {
        let body = ball_cut_by(&tools, what);
        built_listed_and_drawn(&body, kept, sphere_past(2.0), what);
    }
}

/// The ball cut by x, y and z ≥ `offset`, the cap about y one about the
/// ball's own axis, taking its pole at y = 5: at 4 the three caps keep
/// apart; at 3 each two share a piece and all three ring a triangle of the
/// sphere they leave apart, about (1, 1, 1), which 5/√3 < 3 along each axis
/// keeps; at 2 all three share a piece. The sphere left holds the pole at
/// y = −5. Each set is cut in every order.
#[test]
fn a_ball_cut_by_three_flats_of_three_axes_is_built_and_drawn_within_the_drawing_s_tolerance() {
    for offset in [4.0, 3.0, 2.0] {
        let blocks = [DVec3::X, DVec3::Y, DVec3::Z].map(|axis| block_past(axis, offset));
        let kept = kept_past_planes_square_to_each_other(3, offset);
        for order in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            let what = format!(
                "the ball cut by {} ≥ {offset}",
                order.map(|axis| ["x", "y", "z"][axis]).join(", then ")
            );
            let body = ball_cut_by(&order.map(|axis| &blocks[axis]), &what);
            built_listed_and_drawn(&body, kept, sphere_past(offset), &what);
        }
    }
}

/// Three directions square to each other, none along an axis: (3, 1, 3)
/// and (9, 3, −10), 77° off the ball's axis, and (1, −3, 0), 18° off its
/// pole at y = −5.
const SLANTS: [DVec3; 3] = [
    DVec3::new(3.0, 1.0, 3.0),
    DVec3::new(9.0, 3.0, -10.0),
    DVec3::new(1.0, -3.0, 0.0),
];

/// The ball cut by planes at a slant square to each other, each `offset`
/// from its centre along one of [`SLANTS`]: the first two, whose caps take
/// neither pole, at 4 apart and at 2 sharing a piece, the sphere left
/// holding both poles; and all three, the third's cap taking the pole at
/// y = −5, at 4, 3 and 2, the sphere left holding the other, and at 3 a
/// triangle of it apart, as along the axes. Every circle
/// passes at least ten degrees off both poles, and the caps hold what the
/// same caps along the axes hold, a ball being one whichever way it is
/// turned.
#[test]
fn a_ball_cut_by_flats_at_a_slant_square_to_each_other_is_built_and_drawn_within_the_drawing_s_tolerance()
 {
    for (count, offset) in [(2, 4.0), (2, 2.0), (3, 4.0), (3, 3.0), (3, 2.0)] {
        let slabs: Vec<Body> = SLANTS[..count]
            .iter()
            .map(|&normal| slab_along(normal, offset))
            .collect();
        let what = format!(
            "the ball cut by {count} flats at a slant square to each other, {offset} from its centre"
        );
        let body = ball_cut_by(&slabs.iter().collect::<Vec<_>>(), &what);
        built_listed_and_drawn(
            &body,
            kept_past_planes_square_to_each_other(count as u32, offset),
            sphere_past(offset),
            &what,
        );
    }
}

/// The ball cut by two flats at a slant whose circles do not meet, 2 from its
/// centre along (1, 1, 0) and nearly against it, (−1, −1, 0.4): each cap
/// takes one pole, and the sphere left is a band round the ball's axis
/// between two circles of two axes, 2π·5·4. The caps keep apart, their
/// directions 164° from each other and each reaching 66°.
#[test]
fn a_ball_cut_by_two_flats_whose_circles_turn_about_two_axes_round_its_own_is_built_and_drawn_within_the_drawing_s_tolerance()
 {
    let what = "the ball cut by two flats into a band round its axis";
    let body = ball_cut_by(
        &[
            &slab_along(DVec3::new(1.0, 1.0, 0.0), 2.0),
            &slab_along(DVec3::new(-1.0, -1.0, 0.4), 2.0),
        ],
        what,
    );
    built_listed_and_drawn(
        &body,
        BALL_OF_5 - 2.0 * cap_of_5(3.0),
        TAU * 5.0 * 4.0,
        what,
    );
}
