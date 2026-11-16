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
//! it. The fallback's own test in `brep/tessellation/outline/tests.rs` now
//! holds that a face bounded by circles alone is laid out as one bounded by a
//! cylinder's curve is, and refused for the same nearness to a pole.
//!
//! The review's two risks, each held at four finenesses of the drawing: a
//! half ball cut by a flat across its rim, whose circle ends at corners of
//! the half ball's floor, where it touches or crosses the half-plane of its
//! sphere's grid through each corner — declined `Undrawn` before #940, now
//! the fallback's; and bodies drawn before #940 whose sphere carries a
//! circle of another axis beside a round or a flat — a block's corner
//! filleted on its three edges, and the rib cut from a ball chamfered or
//! filleted at its foot — which now also take the angles of the sphere's
//! grid their circles cross.

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

/// How fine the drawings [`drawn_at_each_fineness`] cuts are, as shares of
/// the drawing's own tolerance: four times finer, the drawing's, and four
/// and sixteen times coarser. A circle's grid, the angles of its sphere it
/// is sampled at and the vertices among them move with the fineness.
const FINENESS: [f64; 4] = [0.25, 1.0, 4.0, 16.0];

/// Holds that `body` is drawn whole, closed and uncrossed at each of
/// [`FINENESS`].
fn drawn_at_each_fineness(body: &Body, what: &str) {
    let reach = body.brep().expect("an exact body").scale().reach();
    for share in FINENESS {
        let tolerance = super::super::drawn::DRAWN * reach * share;
        let triangles = body
            .triangles_within(tolerance)
            .unwrap_or_else(|| panic!("{what}: a face left uncut at {tolerance}"));
        assert_eq!(
            crate::soundness::closed(&triangles),
            Ok(()),
            "{what}, at {tolerance}"
        );
        assert_eq!(
            crate::soundness::uncrossed(&triangles),
            Ok(()),
            "{what}, at {tolerance}"
        );
    }
}

/// What the cap of the ball past the plane `offset` from its centre along a
/// normal leaning `tilt` below square to its axis holds above its equator:
/// over every place `t` along the normal from `offset` out to 5 cos(tilt),
/// the segment of the disc of radius ρ = √(25 − t²) past the chord t tan(tilt)
/// across it, ρ² acos(c/ρ) − c √(ρ² − c²), summed by Simpson's rule with
/// t = 5 cos(tilt) − s², under which the segment's vanishing there is smooth.
/// Square to the axis, half the cap.
fn cap_above_the_equator(tilt: f64, offset: f64) -> f64 {
    let end = 5.0 * tilt.cos();
    if end <= offset {
        return 0.0;
    }
    let slice = |s: f64| {
        let t = end - s * s;
        let (square, chord) = ((25.0 - t * t).max(0.0), t * tilt.tan());
        let segment = if chord * chord >= square {
            0.0
        } else {
            square * (chord / square.sqrt()).acos() - chord * (square - chord * chord).sqrt()
        };
        segment * 2.0 * s
    };
    let (steps, last) = (1 << 12, (end - offset).sqrt());
    let step = last / f64::from(steps);
    let inner: f64 = (1..steps)
        .map(|index| slice(step * f64::from(index)) * if index % 2 == 1 { 4.0 } else { 2.0 })
        .sum();
    (slice(0.0) + inner + slice(last)) * step / 3.0
}

/// The ball cut in half at y = 0, keeping the dome above it or the bowl
/// below it.
fn dome(above: bool) -> Body {
    let (low, high) = if above { (-10.0, 0.0) } else { (0.0, 10.0) };
    ball_about_y()
        .difference(&boxed(
            DVec3::new(-10.0, low, -10.0),
            DVec3::new(10.0, high, 10.0),
        ))
        .expect("a plane square to its axis cuts the ball in half")
}

/// A half ball, then a flat across its rim, whose circle on the sphere ends
/// on the rim at two corners shared by the flat, the half ball's floor and
/// its sphere: on main, declined `Undrawn`; since #940 the fallback's. Square
/// to the floor, x ≥ 3 on the dome and on the bowl, x ≤ −3, and a flat 3
/// along (1, 0, 1): the circle stands square to the rim at each corner, and
/// touches the half-plane of its sphere's grid through that corner there.
/// At a slant, 2 along a normal leaning thirty degrees below the floor's
/// plane, turned 35° about the axis, on the dome, and its mirror on the bowl:
/// the circle crosses that half-plane at the corner. Each holds the half ball
/// less the part of the cap on its side of the floor, and is drawn closed and
/// uncrossed at every fineness.
#[test]
fn a_half_ball_cut_by_a_flat_across_its_rim_is_built_and_drawn_closed_and_uncrossed_at_every_fineness()
 {
    let half = BALL_OF_5 / 2.0;
    let square = half - cap_above_the_equator(0.0, 3.0);
    let tilt = 30f64.to_radians();
    let leaning = half - cap_above_the_equator(tilt, 2.0);
    let turned = |y: f64| {
        let (sin, cos) = 35f64.to_radians().sin_cos();
        DVec3::new(tilt.cos() * cos, y, tilt.cos() * sin)
    };
    let cases = [
        (
            true,
            block_past(DVec3::X, 3.0),
            square,
            "the dome cut by x ≥ 3",
        ),
        (
            false,
            block_past(DVec3::X, 3.0),
            square,
            "the bowl cut by x ≥ 3",
        ),
        (
            true,
            boxed(DVec3::splat(-10.0), DVec3::new(-3.0, 10.0, 10.0)),
            square,
            "the dome cut by x ≤ −3",
        ),
        (
            true,
            slab_along(DVec3::new(1.0, 0.0, 1.0), 3.0),
            square,
            "the dome cut by a flat 3 along (1, 0, 1)",
        ),
        (
            true,
            slab_along(turned(-tilt.sin()), 2.0),
            leaning,
            "the dome cut by a flat leaning down",
        ),
        (
            false,
            slab_along(turned(tilt.sin()), 2.0),
            leaning,
            "the bowl cut by a flat leaning up",
        ),
    ];
    for (above, flat, kept, what) in &cases {
        let body = dome(*above)
            .difference(flat)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
        built_listed_and_drawn(&body, *kept, 50.0 * PI, what);
        drawn_at_each_fineness(&body, what);
    }
}

/// The face numbered `first` and the face numbered `second` meet along one
/// edge: its name.
fn the_edge_between(body: &Body, first: usize, second: usize) -> crate::EdgeName {
    let sides = [vec![first as u32], vec![second as u32]];
    let named: Vec<crate::EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| {
            name.between == sides || name.between == [sides[1].clone(), sides[0].clone()]
        })
        .collect();
    assert_eq!(
        named.len(),
        1,
        "one edge between {first} and {second}: {named:?}"
    );
    named[0].clone()
}

/// The face of `body` whose plane faces `normal` and holds `point`.
fn flat_facing(body: &Body, normal: DVec3, point: DVec3) -> usize {
    (0..body.faces_end())
        .find(|&face| {
            body.plane_of(face).is_some_and(|plane| {
                plane.normal.dot(normal) > 1.0 - 1e-9
                    && plane
                        .corners
                        .iter()
                        .any(|corner| (*corner - point).dot(normal).abs() < 1e-9)
            })
        })
        .unwrap_or_else(|| panic!("a face facing {normal} through {point}"))
}

/// The plate 40 × 30 × 4 with a rib on its top cut from the ball of radius 8
/// about (20, 15, 4), between x = 18 and x = 22, as #805's first slice
/// chamfers and its second fillets at the foot of either side: a half disc
/// turned half a turn about the line along X through the centre.
fn rib_cut_from_a_ball() -> Body {
    let plate = block(DVec2::ZERO, DVec2::new(40.0, 30.0), 4.0);
    let reach = 60f64.sqrt();
    let slice = Contour {
        corners: vec![
            DVec2::new(18.0, 0.0),
            DVec2::new(22.0, 0.0),
            DVec2::new(22.0, reach),
            DVec2::new(18.0, reach),
        ],
        runs: vec![
            Run::Straight,
            Run::Straight,
            Run::Round {
                center: DVec2::new(20.0, 0.0),
                turn: PI - 2.0 * reach.atan2(2.0),
            },
            Run::Straight,
        ],
    };
    let sampled: &'static [DVec2] = Box::leak(slice.corners.clone().into_boxed_slice());
    let outline = Profile {
        exact: Some((slice, Vec::new())),
        sampled: Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let on_the_top = Frame {
        origin: DVec3::new(0.0, 15.0, 4.0),
        ..FLAT
    };
    let along_x = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::X,
    };
    let rib = plate
        .tool_turned(&outline, on_the_top, &Turn::of(along_x, PI, 1e-4, &outline))
        .expect("the slice turns");
    plate.union(&rib).expect("the rib is joined")
}

/// Bodies drawn before #940 whose sphere carries a circle of another axis
/// beside a round or a flat, where its samples are shared or held back:
/// a block's corner filleted on its three edges, closed by a piece of sphere
/// whose three arcs each run along a fillet's round (#699), and the rib cut
/// from a ball chamfered or filleted at the foot of either side, the flat's
/// circle, or the round's curve, on the sphere at both ends (#805). Every
/// circle on a sphere now also takes the angles of the sphere's grid it
/// crosses; each is drawn closed and uncrossed at every fineness all the
/// same.
#[test]
fn the_spheres_of_a_corner_fillet_and_of_a_rib_cut_from_a_ball_and_blended_at_its_foot_are_drawn_closed_and_uncrossed_at_every_fineness()
 {
    let corner = block(DVec2::ZERO, DVec2::new(30.0, 20.0), 10.0);
    let at = DVec3::new(30.0, 20.0, 10.0);
    let [right, back, top] =
        [DVec3::X, DVec3::Y, DVec3::Z].map(|normal| flat_facing(&corner, normal, at));
    let names = [(right, back), (back, top), (top, right)]
        .map(|(one, other)| the_edge_between(&corner, one, other));
    let filleted = corner
        .fillet_edges(&names, 3.0)
        .unwrap_or_else(|declined| panic!("the corner's fillet: declined, {declined:?}"));
    drawn_at_each_fineness(
        &filleted.body,
        "the block's corner filleted on its three edges",
    );

    let rib = rib_cut_from_a_ball();
    let plate_top = flat_facing(&rib, DVec3::Z, DVec3::new(0.0, 0.0, 4.0));
    for (normal, x, side) in [(DVec3::X, 22.0, "right"), (DVec3::NEG_X, 18.0, "left")] {
        let face = flat_facing(&rib, normal, DVec3::new(x, 15.0, 4.0));
        let foot = the_edge_between(&rib, plate_top, face);
        let chamfered = rib
            .chamfer(&foot, 2.0)
            .unwrap_or_else(|declined| panic!("the {side} foot's chamfer: declined, {declined:?}"));
        drawn_at_each_fineness(
            &chamfered.body,
            &format!("the rib cut from a ball chamfered at the foot of its {side} side"),
        );
        let filleted = rib
            .fillet(&foot, 2.0)
            .unwrap_or_else(|declined| panic!("the {side} foot's fillet: declined, {declined:?}"));
        drawn_at_each_fineness(
            &filleted.body,
            &format!("the rib cut from a ball filleted at the foot of its {side} side"),
        );
    }
}
