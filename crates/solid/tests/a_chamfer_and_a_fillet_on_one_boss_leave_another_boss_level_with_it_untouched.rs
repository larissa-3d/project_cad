//! Two bosses stand on one block, as high as each other, their tops in one
//! plane and apart. A chamfer or a fillet on a top edge of one of them is laid
//! on that edge as on a boss standing alone, and the other boss keeps every
//! face and every edge it had. Two tops that touch or overlap are one face, a
//! different body, and out of this issue's reach.
//!
//! Closes #924.
//! - A chamfer and a fillet on a top edge of one of two bosses whose tops share
//!   a plane are built, the other boss untouched, the body closed and sound,
//!   the volume the closed form —
//!   `a_chamfer_on_a_top_edge_of_one_of_two_bosses_level_with_each_other_is_cut_leaving_the_other_untouched`,
//!   `a_fillet_on_a_top_edge_of_one_of_two_bosses_level_with_each_other_is_laid_leaving_the_other_untouched`,
//!   each on the first boss's top front edge with the second boss's front in
//!   line with it and set back by 1, the issue's table; on its top back edge;
//!   on the second boss's top front edge, the first boss the one left
//!   untouched; on the first boss's top edge facing the second; and on two
//!   blocks apart in line with nothing under them. At the part's level, the
//!   issue's own probe, a boss put in before the chamfered one as high as
//!   it —
//!   `a_step_put_in_before_a_chamfer_while_the_part_is_read_earlier_leaves_it_on_its_edge`
//!   in `crates/part/tests/a_chamfer_moves_nothing_raised_or_laid_after_it.rs`
//! - Existing chamfer and fillet tests still pass — no test: their own suites,
//!   `a_chamfer_cuts_one_straight_edge.rs`, `a_fillet_rounds_one_straight_edge.rs`,
//!   the other `a_chamfer_and_a_fillet_` files and `src/body/tests/`, are the
//!   answer, left as they were but for one test: the chamfer's suite held
//!   `a_chamfer_whose_cut_carried_along_its_edge_would_reach_a_second_block_in_line_is_declined_as_unsupported`,
//!   the very limit this issue lifts, two blocks in line declined; it went,
//!   and its two blocks are the last case of the two tests above, built

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Chamfered, EdgeName, Filleted, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it, on a plane
/// or on the round: each is placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// The block both bosses stand on: 30 along X, 10 along Y, 20 high.
const BLOCK: [f64; 3] = [30.0, 10.0, 20.0];

/// How high both bosses stand, raised from the ground through the block.
const HIGH: f64 = 30.0;

/// Where each boss stands along X, and where both end at the back.
const FIRST: [f64; 2] = [5.0, 10.0];
const SECOND: [f64; 2] = [20.0, 25.0];
const BACK: f64 = 8.0;

/// Where the first boss's front stands along Y.
const FRONT: f64 = 2.0;

/// The value every chamfer and fillet here is asked: a distance or a radius
/// both of an edge's faces give whole.
const VALUE: f64 = 2.0;

/// The numbers each boss's faces answer to, joined after the block's six:
/// the first's floor, which the join takes in, six, then its top, and its
/// walls from the front round to the left; the second's six on from there.
const FIRST_FACES: [usize; 5] = [7, 8, 9, 10, 11];
const SECOND_FACES: [usize; 5] = [13, 14, 15, 16, 17];
const FIRST_TOP: usize = 7;
const FIRST_FRONT: usize = 8;
const FIRST_RIGHT: usize = 9;
const FIRST_BACK: usize = 10;
const SECOND_TOP: usize = 13;
const SECOND_FRONT: usize = 14;

/// An outline with its runs, as the drawing hands it to the exact kernel.
fn profile(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

/// The part with an outline drawn on the ground raised up by `height`,
/// joined to it as `cao_part` joins a step: the tool's faces numbered past
/// the part's.
fn joined_to(part: &Body, outline: Contour, height: f64) -> Body {
    let ground = Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    };
    let tool = part
        .tool_raised(&profile(outline), ground, DVec3::Z * height)
        .expect("the tool raises");
    part.union(&tool).expect("the join is built")
}

/// The [`BLOCK`], and on it two bosses [`HIGH`] raised from the ground as a
/// sketch on the ground raises them: the first across [`FIRST`], its front at
/// [`FRONT`], the second across [`SECOND`], its front at `second_front`, both
/// back to [`BACK`]. Their tops lie in one plane, apart.
fn two_bosses(second_front: f64) -> Body {
    let [length, breadth, height] = BLOCK;
    let block = joined_to(
        &Body::default(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth)),
        height,
    );
    let first = joined_to(
        &block,
        Contour::rectangle(DVec2::new(FIRST[0], FRONT), DVec2::new(FIRST[1], BACK)),
        HIGH,
    );
    joined_to(
        &first,
        Contour::rectangle(
            DVec2::new(SECOND[0], second_front),
            DVec2::new(SECOND[1], BACK),
        ),
        HIGH,
    )
}

/// The matter of [`two_bosses`], before any edge is taken off.
fn two_bosses_volume(second_front: f64) -> f64 {
    let [length, breadth, height] = BLOCK;
    let over_the_block = HIGH - height;
    length * breadth * height
        + (FIRST[1] - FIRST[0]) * (BACK - FRONT) * over_the_block
        + (SECOND[1] - SECOND[0]) * (BACK - second_front) * over_the_block
}

/// Two blocks 15 by 10 by 10 on the ground with nothing under them, the
/// second from x = 20, its front and its top in line with the first's: the
/// first's faces numbered nought to five, its top one and its front two, the
/// second's six to eleven. The chamfer's own suite declined a chamfer on the
/// first's top front edge here, since its cut carried along the edge would
/// chamfer the second's as well.
fn two_blocks_in_line() -> Body {
    let first = joined_to(
        &Body::default(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(15.0, 10.0)),
        10.0,
    );
    joined_to(
        &first,
        Contour::rectangle(DVec2::new(20.0, 0.0), DVec2::new(35.0, 10.0)),
        10.0,
    )
}

/// A top edge of one boss: the part it stands on and the matter that part
/// holds, the boss's top and the wall the edge lies between, the edge's two
/// ends, the way the wall faces, and the faces of the other boss, which
/// nothing may touch.
struct TopEdge {
    what: &'static str,
    part: Body,
    volume: f64,
    sides: [usize; 2],
    ends: [DVec3; 2],
    outward: DVec3,
    other: Vec<usize>,
}

impl TopEdge {
    fn length(&self) -> f64 {
        self.ends[0].distance(self.ends[1])
    }

    fn along(&self) -> DVec3 {
        (self.ends[1] - self.ends[0]).normalize()
    }
}

/// The edges each test takes off: the issue's two cases first, the first
/// boss's top front edge with the second boss's front in line with it and
/// set back by 1; then its top back edge, the backs in line; the second
/// boss's top front edge, the first boss the one beside it; the first boss's
/// top edge facing the second, which runs across the other's line rather
/// than along it; and the first of [`two_blocks_in_line`]'s top front edge,
/// two bosses with no block under them.
fn top_edges() -> Vec<TopEdge> {
    let at = |x: f64, y: f64| DVec3::new(x, y, HIGH);
    let set_back = FRONT + 1.0;
    vec![
        TopEdge {
            what: "the first boss's top front edge, the second's front in line with it",
            part: two_bosses(FRONT),
            volume: two_bosses_volume(FRONT),
            sides: [FIRST_TOP, FIRST_FRONT],
            ends: [at(FIRST[0], FRONT), at(FIRST[1], FRONT)],
            outward: -DVec3::Y,
            other: SECOND_FACES.to_vec(),
        },
        TopEdge {
            what: "the first boss's top front edge, the second's front set back by 1",
            part: two_bosses(set_back),
            volume: two_bosses_volume(set_back),
            sides: [FIRST_TOP, FIRST_FRONT],
            ends: [at(FIRST[0], FRONT), at(FIRST[1], FRONT)],
            outward: -DVec3::Y,
            other: SECOND_FACES.to_vec(),
        },
        TopEdge {
            what: "the first boss's top back edge, the backs in line",
            part: two_bosses(FRONT),
            volume: two_bosses_volume(FRONT),
            sides: [FIRST_TOP, FIRST_BACK],
            ends: [at(FIRST[0], BACK), at(FIRST[1], BACK)],
            outward: DVec3::Y,
            other: SECOND_FACES.to_vec(),
        },
        TopEdge {
            what: "the second boss's top front edge, the first's front in line with it",
            part: two_bosses(FRONT),
            volume: two_bosses_volume(FRONT),
            sides: [SECOND_TOP, SECOND_FRONT],
            ends: [at(SECOND[0], FRONT), at(SECOND[1], FRONT)],
            outward: -DVec3::Y,
            other: FIRST_FACES.to_vec(),
        },
        TopEdge {
            what: "the first boss's top edge facing the second",
            part: two_bosses(FRONT),
            volume: two_bosses_volume(FRONT),
            sides: [FIRST_TOP, FIRST_RIGHT],
            ends: [at(FIRST[1], FRONT), at(FIRST[1], BACK)],
            outward: DVec3::X,
            other: SECOND_FACES.to_vec(),
        },
        TopEdge {
            what: "the top front edge of a block standing apart from a second in line with it, \
                   nothing under either",
            part: two_blocks_in_line(),
            volume: 2.0 * 15.0 * 10.0 * 10.0,
            sides: [1, 2],
            ends: [DVec3::new(0.0, 0.0, 10.0), DVec3::new(15.0, 0.0, 10.0)],
            outward: -DVec3::Y,
            other: (6..12).collect(),
        },
    ]
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, [first, second]: [usize; 2]) -> EdgeName {
    let sides = [vec![first as u32], vec![second as u32]];
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides)
        .collect();
    assert_eq!(
        named.len(),
        1,
        "one edge between {first} and {second}: {named:?}"
    );
    named[0].clone()
}

/// Every number some face of the body answers to: a step's, below where the
/// steps' numbers end, and a piece's, past them.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&number| body.has_face(number))
        .collect()
}

/// Holds that a step left every face of the part it was given and made one
/// face more, and gives back the number that face answers to.
fn one_face_more(part: &Body, body: &Body, what: &str) -> usize {
    let (before, after) = (numbers(part), numbers(body));
    assert!(
        before.is_subset(&after),
        "{what}: a face went: {before:?} became {after:?}"
    );
    let made: Vec<usize> = after.difference(&before).copied().collect();
    assert_eq!(
        made.len(),
        1,
        "{what}: one face more: {before:?} became {after:?}"
    );
    made[0]
}

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    for corner in &plane.corners {
        assert!(
            expected.iter().any(|point| point.distance(*corner) < PLACE),
            "{what}: a corner at {corner}, where the corners are {expected:?}"
        );
    }
    for point in expected {
        assert!(
            plane
                .corners
                .iter()
                .any(|corner| corner.distance(*point) < PLACE),
            "{what}: no corner at {point}, among {:?}",
            plane.corners
        );
    }
}

/// Holds that a body is exact and keeps the rules every solid must: its
/// surface closed, no face through another, the matter it holds what
/// arithmetic promised, and the matter its triangles enclose within `drawn`
/// of it.
fn sound_drawn_within(body: &Body, promised: f64, drawn: f64, what: &str) {
    assert!(
        body.is_exact(),
        "{what}: the exact kernel computed the body"
    );
    let triangles = body.triangles();
    assert_eq!(closed(&triangles), Ok(()), "{what}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
    assert!(
        (body.volume() - promised).abs() <= EXACT * promised,
        "{what}: the body holds {}, where arithmetic promised {promised}",
        body.volume()
    );
    let enclosed = enclosed(&triangles);
    assert!(
        (enclosed - promised).abs() <= drawn,
        "{what}: its triangles enclose {enclosed}, where arithmetic promised {promised}"
    );
}

/// Holds that the faces numbered `faces`, the other boss's, are each the
/// plane they were before the step, with the very corners they had, and that
/// every edge one of them met before is found by its name after.
fn untouched(part: &Body, body: &Body, faces: &[usize], what: &str) {
    for &face in faces {
        let was = part
            .plane_of(face)
            .unwrap_or_else(|| panic!("{what}: face {face} of the other boss offers a plane"));
        let is = body.plane_of(face).unwrap_or_else(|| {
            panic!("{what}: face {face} of the other boss offers no plane any more")
        });
        assert!(
            is.normal.distance(was.normal) < PLACE,
            "{what}: face {face} of the other boss faces {}, where it faced {}",
            is.normal,
            was.normal
        );
        corners_are(
            body,
            face,
            &was.corners,
            &format!("{what}: face {face} of the other boss"),
        );
    }
    let its_own = |name: &EdgeName| {
        name.between
            .iter()
            .flatten()
            .any(|side| faces.contains(&(*side as usize)))
    };
    let edges: Vec<EdgeName> = part.edge_names().into_iter().filter(its_own).collect();
    assert!(!edges.is_empty(), "{what}: the other boss has edges");
    for name in edges {
        assert_eq!(
            body.lost(&name),
            None,
            "{what}: the other boss's edge between {:?} is no longer found",
            name.between
        );
    }
}

/// Holds that a chamfer was given the distance it was asked.
fn chamfer_taken_whole(chamfered: &Chamfered, what: &str) {
    assert!(
        (chamfered.asked - VALUE).abs() < PLACE && (chamfered.taken - VALUE).abs() < PLACE,
        "{what}: asked {} and took {}, where {VALUE} fits",
        chamfered.asked,
        chamfered.taken
    );
}

/// Holds that a fillet was given the radius it was asked.
fn fillet_taken_whole(filleted: &Filleted, what: &str) {
    assert!(
        (filleted.asked - VALUE).abs() < PLACE && (filleted.taken - VALUE).abs() < PLACE,
        "{what}: asked {} and took {}, where {VALUE} fits",
        filleted.asked,
        filleted.taken
    );
}

/// Holds that the face numbered `face` lies on the quarter of a cylinder of
/// [`VALUE`] tangent to the edge's top and wall: every corner of its
/// triangles at that radius from the line set back by it under the top and
/// behind the wall, between the edge's two ends, and on the side of the line
/// facing out of both.
fn on_the_round(body: &Body, face: usize, edge: &TopEdge, what: &str) {
    let axis = edge.ends[0] - edge.outward * VALUE - DVec3::Z * VALUE;
    let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the round is drawn");
    for corner in corners {
        let off = corner - axis;
        let along = off.dot(edge.along());
        let across = off - edge.along() * along;
        assert!(
            (across.length() - VALUE).abs() < PLACE
                && (-PLACE..=edge.length() + PLACE).contains(&along)
                && across.dot(edge.outward) >= -PLACE
                && across.dot(DVec3::Z) >= -PLACE,
            "{what}: the round is drawn through {corner}, off the quarter cylinder of {VALUE} \
             about the line from {axis} along {}",
            edge.along()
        );
    }
}

#[test]
fn a_chamfer_on_a_top_edge_of_one_of_two_bosses_level_with_each_other_is_cut_leaving_the_other_untouched()
 {
    for edge in top_edges() {
        let (what, part) = (edge.what, &edge.part);
        let name = edge_between(part, edge.sides);

        let chamfered = part
            .chamfer(&name, VALUE)
            .unwrap_or_else(|declined| panic!("{what}: the chamfer is declined, {declined:?}"));

        chamfer_taken_whole(&chamfered, what);
        let body = &chamfered.body;
        let promised = edge.volume - VALUE * VALUE / 2.0 * edge.length();
        sound_drawn_within(body, promised, EXACT * promised, what);
        untouched(part, body, &edge.other, what);
        let chamfer = one_face_more(part, body, what);
        assert!(
            body.is_flat(chamfer),
            "{what}: the face the chamfer lays is flat"
        );
        let [start, end] = edge.ends;
        let (on_top, on_wall) = (-edge.outward * VALUE, -DVec3::Z * VALUE);
        corners_are(
            body,
            chamfer,
            &[start + on_top, end + on_top, end + on_wall, start + on_wall],
            &format!("{what}: the chamfer, set back by {VALUE} along the top and the wall"),
        );
    }
}

#[test]
fn a_fillet_on_a_top_edge_of_one_of_two_bosses_level_with_each_other_is_laid_leaving_the_other_untouched()
 {
    for edge in top_edges() {
        let (what, part) = (edge.what, &edge.part);
        let name = edge_between(part, edge.sides);

        let filleted = part
            .fillet(&name, VALUE)
            .unwrap_or_else(|declined| panic!("{what}: the fillet is declined, {declined:?}"));

        fillet_taken_whole(&filleted, what);
        let body = &filleted.body;
        let promised = edge.volume - (1.0 - PI / 4.0) * VALUE * VALUE * edge.length();
        let (low, high) = part.bounds().expect("a part has a box");
        let reach = low.abs().max(high.abs()).max_element();
        let area = PI / 2.0 * VALUE * edge.length();
        sound_drawn_within(body, promised, DRAWN * reach * area, what);
        untouched(part, body, &edge.other, what);
        let fillet = one_face_more(part, body, what);
        assert!(
            !body.is_flat(fillet) && body.plane_of(fillet).is_none(),
            "{what}: the face the fillet lays is curved, and offers no plane to draw on"
        );
        on_the_round(body, fillet, &edge, what);
    }
}
