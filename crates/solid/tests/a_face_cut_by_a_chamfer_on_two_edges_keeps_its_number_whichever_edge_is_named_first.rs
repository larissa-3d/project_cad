//! A face a chamfer on two edges cuts back keeps its number whichever edge is
//! named first. The block 30 by 30 and 10 high stands on the ground's origin,
//! raised as `cao_part` builds a step: its floor numbered nought, its top one,
//! then its front, along y = 0, two; its end, along x = 30, three; its back
//! four; and its left end five. The column 3 by 3 under the upright where the
//! front meets the end is notched away from the floor up to a ceiling through
//! (30, 0, 6), (27, 0, 7) and (30, 3, 7), the upright now ending on it, as
//! #912 builds it, the notch's faces taking numbers of their own. Chamfered
//! at 3 on the top's edge across the end and that upright, the value bounded
//! a hair short by the notch's corner, the step lays #912's mitre in both
//! orders, each flat numbered past where the notched block's numbers end by
//! the rank its edge is named at.
//!
//! The bridge is the same block 10 wide, an arch cut through it from front
//! to back between x = 10 and x = 20, open at the floor and 7 high: its front
//! and its back each one face standing on two legs, joined by a lintel 3 high,
//! and its floor left in two pieces by the arch, the second numbered
//! 2147483648, the first number a piece takes. Chamfered at 3 on the top's
//! front and back edges, each flat's set-back on its wall lands on the arch's
//! crown and takes the lintel there whole, so the step leaves its front and
//! its back each in two pieces too.
//!
//! Closes #954.
//! - That pair at 3, in both orders: `has_face(3)` true, the same face numbers
//!   on every face, and the edges' names agreeing with the faces' —
//!   `a_chamfer_on_the_top_s_edge_across_the_end_and_the_upright_ending_on_the_notch_leaves_the_end_its_number_and_numbers_every_face_alike_whichever_is_named_first`.
//!   Read as: in each order, some face answering to 3, the end's number;
//!   every number the edges' names read answered to by some face, and every
//!   number a face answers to, a piece's read back to the face it was parted
//!   from, read by some edge's name; and every number one order's faces
//!   answer to answered to in the other by a face on the same plane with the
//!   same corners, each flat read by the edge it was laid on
//! - Not a criterion of the issue — what the fix's rule promises: a step
//!   laid in several cuts parts its faces once, off its final body, as one
//!   cut from the part would, so a step that leaves faces in pieces numbers
//!   them alike in every order —
//!   `a_chamfer_on_both_top_edges_of_a_bridge_cutting_through_its_lintel_numbers_the_pieces_of_its_front_and_back_as_one_cut_would_whichever_is_named_first`.
//!   Read as: in both orders, the value taken whole; the pieces, each with
//!   the face it was parted from, the floor's 2147483648 as before the step,
//!   the front's 2147483649 and the back's 2147483650, as one parting
//!   numbers them, ascending by the face parted; no other number up to
//!   2147483655 parted from anything; and, across the two orders, the same
//!   face parted from for each of those numbers, and every number one
//!   order's faces answer to answered to in the other by a face on the same
//!   plane with the same corners, each flat read by the edge it was laid on
//! - The order tests of #822 and #912 still pass — no test: they run beside
//!   this one, unchanged. #822's, in
//!   `rounds_meeting_through_an_end_face_or_along_a_slant_reduce_the_value.rs`:
//!   `a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_of_a_hexagonal_prism_asked_past_where_the_outer_two_would_cross_take_the_value_asked_in_every_order`,
//!   `a_fillet_and_a_chamfer_on_the_six_top_edges_of_a_hexagonal_prism_asked_past_where_alternate_edges_would_cross_take_the_value_asked_whatever_the_order`
//!   and
//!   `a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_of_a_hexagonal_prism_whose_middle_wall_overhangs_take_the_value_asked_in_every_order_its_set_back_outrunning_where_the_outer_two_would_cross`.
//!   #912's, in
//!   `a_full_round_meets_the_round_of_an_upright_at_its_end_on_an_ellipse.rs`:
//!   `a_chamfer_on_the_top_s_front_edge_and_the_upright_ending_on_the_notch_lays_one_mitred_body_whichever_is_named_first`,
//!   `a_chamfer_on_the_top_s_edge_across_the_end_and_the_upright_ending_on_the_notch_lays_one_mitred_body_whichever_is_named_first`
//!   and
//!   `a_chamfer_whose_corner_triangle_takes_whole_the_flat_of_an_upright_running_to_no_other_closed_corner_is_declined_as_unsupported_whatever_the_order`
//!
//! On `main`, the upright named first, the end answered to 3 and no face was
//! parted. The top's edge across the end named first, the end answered to a
//! piece's number, 2147483648, parted from 3: `has_face(3)` was false while
//! the edges' names still read 3. The mirror pair, the top's front edge and
//! the upright, and both pairs a tenth under, numbered their faces alike in
//! both orders.

use std::collections::{BTreeMap, BTreeSet};

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::{Body, EdgeName, FacePlane, Loop};
use glam::{DVec2, DVec3};

/// How far a point may stand from where the other order puts it: both are
/// placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// The numbers the block's faces answer to, raised as `cao_part` builds a
/// step: its floor, its top, then the walls along y = 0, along x = [`LENGTH`]
/// and along the far side.
const FLOOR: usize = 0;
const TOP: usize = 1;
const FRONT: usize = 2;
const END: usize = 3;
const BACK: usize = 4;

/// The block: [`LENGTH`] long, [`HEIGHT`] high, and as wide as it is long
/// under the notch, [`BRIDGE_WIDTH`] under the arch.
const LENGTH: f64 = 30.0;
const HEIGHT: f64 = 10.0;

/// The bridge: [`BRIDGE_WIDTH`] from front to back, the arch through it
/// between x = [`ARCH_FROM`] and x = [`ARCH_TO`], [`ARCH_HEIGHT`] high under
/// the lintel.
const BRIDGE_WIDTH: f64 = 10.0;
const ARCH_FROM: f64 = 10.0;
const ARCH_TO: f64 = 20.0;
const ARCH_HEIGHT: f64 = 7.0;

/// The value asked on the bridge: the lintel's height, at which the set-back
/// of a top edge on the front or the back lands on the arch's crown, its flat
/// taking the lintel there whole.
const LINTEL: f64 = HEIGHT - ARCH_HEIGHT;

/// The first number a piece takes, past every number a step gives.
const FIRST_PIECE: usize = 1 << 31;

/// How many numbers from [`FIRST_PIECE`] on are read for what each was parted
/// from: more than the bridge's step could hand out in its cuts and its
/// parting together.
const PIECES_READ: usize = 8;

/// The value asked, at which the notch's corner bounds it a hair short.
const VALUE: f64 = 3.0;

/// The edges the pair names, each as the two faces it lies between: the top's
/// edge across the end, and the upright where the front meets the end.
const TOP_END: [usize; 2] = [TOP, END];
const FRONT_UPRIGHT: [usize; 2] = [FRONT, END];

/// The edges the bridge's step names: the top's front and back edges.
const TOP_FRONT: [usize; 2] = [TOP, FRONT];
const TOP_BACK: [usize; 2] = [TOP, BACK];

/// The ground's plane, which a part is drawn on.
fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

/// An outline with its runs, as the drawing hands it to the exact kernel.
fn profile(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

/// The slab `size` thick beyond the plane through `through` square to `away`,
/// reaching `size` along it every way.
fn beyond(part: &Body, through: DVec3, away: DVec3, size: f64) -> Body {
    let away = away.normalize();
    let u = away.any_orthonormal_vector();
    part.tool_raised(
        &profile(Contour::rectangle(DVec2::splat(-size), DVec2::splat(size))),
        Frame {
            origin: through,
            u,
            v: away.cross(u),
        },
        away * size,
    )
    .expect("the slab raises")
}

/// The block [`LENGTH`] long, `width` wide and [`HEIGHT`] high, drawn on the
/// ground from its origin and raised as `cao_part` builds a step.
fn block(width: f64) -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(
            &profile(Contour::rectangle(DVec2::ZERO, DVec2::new(LENGTH, width))),
            ground(),
            DVec3::Z * HEIGHT,
        )
        .expect("the block raises");
    part.union(&tool).expect("the block is built")
}

/// The block, its corner under the upright where the front meets the end
/// notched from the floor: the column 3 by 3 there taken away up to the
/// ceiling through (30, 0, 6), where the upright now ends, (27, 0, 7),
/// (30, 3, 7) and (27, 3, 8).
fn notched_under_its_front_upright() -> Body {
    let part = block(LENGTH);
    let tool = part
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::new(LENGTH - 3.0, -1.0),
                DVec2::new(LENGTH + 1.0, 3.0),
            )),
            Frame {
                origin: -DVec3::Z,
                ..ground()
            },
            DVec3::Z * (HEIGHT + 2.0),
        )
        .expect("the column raises");
    let column = Body::default().union(&tool).expect("the column is built");
    let ceiling = DVec3::new(LENGTH, 0.0, 6.0);
    let notch = column
        .difference(&beyond(&column, ceiling, DVec3::new(1.0, -1.0, 3.0), 40.0))
        .expect("the notch is built");
    part.difference(&notch).expect("the notched block is built")
}

/// The block [`BRIDGE_WIDTH`] wide, an arch cut through it from front to back
/// between x = [`ARCH_FROM`] and x = [`ARCH_TO`], open at the floor and
/// [`ARCH_HEIGHT`] high: its front and its back each one face standing on two
/// legs, joined over the arch by a lintel [`LINTEL`] high, and its floor in
/// two pieces, one each side of the arch.
fn bridge() -> Body {
    let part = block(BRIDGE_WIDTH);
    let arch = part
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::new(ARCH_FROM, -1.0),
                DVec2::new(ARCH_TO, BRIDGE_WIDTH + 1.0),
            )),
            Frame {
                origin: -DVec3::Z,
                ..ground()
            },
            DVec3::Z * (ARCH_HEIGHT + 1.0),
        )
        .expect("the arch raises");
    part.difference(&arch).expect("the bridge is built")
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides || name.between == [other.clone(), one.clone()])
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

/// Every number the edges' names read on either side.
fn read_by_the_edges(body: &Body) -> BTreeSet<usize> {
    body.edge_names()
        .iter()
        .flat_map(|name| name.between.iter().flatten())
        .map(|&number| number as usize)
        .collect()
}

/// The number of the face `number` was first parted from, or `number` itself
/// when no cut parted it: what an edge's name reads for it.
fn rooted(body: &Body, number: usize) -> usize {
    std::iter::successors(Some(number), |&piece| body.parted_from(piece))
        .last()
        .unwrap_or(number)
}

/// The number a face laid on the pair answers to, named in `order` from
/// `start`, read as the rank the listed order gives its edge; any other
/// number as itself.
fn as_listed(start: usize, order: &[usize], number: usize) -> usize {
    match number.checked_sub(start) {
        Some(rank) if rank < order.len() => start + order[rank],
        _ => number,
    }
}

/// Whether every point of `some` stands at a point of `others`, and every
/// point of `others` at one of `some`.
fn same_points(some: &[DVec3], others: &[DVec3]) -> bool {
    let within =
        |points: &[DVec3], point: &DVec3| points.iter().any(|other| other.distance(*point) < PLACE);
    some.iter().all(|point| within(others, point)) && others.iter().all(|point| within(some, point))
}

/// Whether two faces lie on one plane, facing one way, with the same corners.
fn same_face(one: &Option<FacePlane>, other: &Option<FacePlane>) -> bool {
    match (one, other) {
        (Some(one), Some(other)) => {
            one.normal.distance(other.normal) < PLACE && same_points(&one.corners, &other.corners)
        }
        (None, None) => true,
        _ => false,
    }
}

/// What tells the faces `one` laid from those `other` laid, each given by the
/// number it answers to, read as the listed order reads it, and its plane: a
/// line for each number one answers to and the other does not, or answers to
/// on another plane or with other corners.
fn numbered_otherwise(
    (one, one_faces): &(String, BTreeMap<usize, Option<FacePlane>>),
    (other, other_faces): &(String, BTreeMap<usize, Option<FacePlane>>),
) -> Vec<String> {
    let every: BTreeSet<&usize> = one_faces.keys().chain(other_faces.keys()).collect();
    every
        .into_iter()
        .filter_map(
            |number| match (one_faces.get(number), other_faces.get(number)) {
                (Some(own), Some(theirs)) if same_face(own, theirs) => None,
                (Some(own), Some(theirs)) => Some(format!(
                    "face {number} is {own:?} after {one}, and {theirs:?} after {other}"
                )),
                (Some(_), None) => Some(format!(
                    "face {number} is answered to after {one}, and not after {other}"
                )),
                (None, _) => Some(format!(
                    "face {number} is answered to after {other}, and not after {one}"
                )),
            },
        )
        .collect()
}

#[test]
fn a_chamfer_on_the_top_s_edge_across_the_end_and_the_upright_ending_on_the_notch_leaves_the_end_its_number_and_numbers_every_face_alike_whichever_is_named_first()
 {
    let part = notched_under_its_front_upright();
    let start = part.faces_end();
    let listed = [
        (TOP_END, "the top's edge across the end"),
        (FRONT_UPRIGHT, "the upright ending on the notch"),
    ];
    let named: Vec<EdgeName> = listed
        .iter()
        .map(|&([one, other], _)| edge_between(&part, one, other))
        .collect();

    let mut not_so: Vec<String> = Vec::new();
    let mut laid: Vec<(String, BTreeMap<usize, Option<FacePlane>>)> = Vec::new();
    for order in [[0, 1], [1, 0]] {
        let names: Vec<EdgeName> = order.iter().map(|&edge| named[edge].clone()).collect();
        let called: Vec<&str> = order.iter().map(|&edge| listed[edge].1).collect();
        let what = format!("the chamfer on {}", called.join(", then "));
        let chamfered = match part.chamfer_edges(&names, VALUE) {
            Ok(chamfered) => chamfered,
            Err(declined) => {
                not_so.push(format!("{what}: declined, {declined:?}"));
                continue;
            }
        };
        let body = &chamfered.body;
        let answered = numbers(body);
        let read = read_by_the_edges(body);
        if !body.has_face(END) {
            not_so.push(format!(
                "{what}: no face answers to {END}, the end's number; the faces answer to \
                 {answered:?}"
            ));
        }
        let unanswered: Vec<usize> = read
            .iter()
            .copied()
            .filter(|&number| !body.has_face(number))
            .collect();
        if !unanswered.is_empty() {
            not_so.push(format!(
                "{what}: the edges' names read {unanswered:?}, which no face answers to"
            ));
        }
        let unread: Vec<usize> = answered
            .iter()
            .copied()
            .filter(|&number| !read.contains(&rooted(body, number)))
            .collect();
        if !unread.is_empty() {
            not_so.push(format!(
                "{what}: faces answer to {unread:?}, which no edge's name reads"
            ));
        }
        let faces = answered
            .iter()
            .map(|&number| (as_listed(start, &order, number), body.plane_of(number)))
            .collect();
        laid.push((what, faces));
    }

    if let [one, other] = &laid[..] {
        not_so.extend(numbered_otherwise(one, other));
    }

    assert!(
        not_so.is_empty(),
        "on the block notched under its front upright, the chamfer at {VALUE} on the top's edge \
         across the end and the upright ending on the notch leaves the end answering to {END} \
         whichever is named first, the edges' names reading the numbers the faces answer to, \
         and both orders numbering every face alike, each flat read by the edge it was laid on; \
         not so:\n{}",
        not_so.join("\n")
    );
}

#[test]
fn a_chamfer_on_both_top_edges_of_a_bridge_cutting_through_its_lintel_numbers_the_pieces_of_its_front_and_back_as_one_cut_would_whichever_is_named_first()
 {
    let part = bridge();
    let floor_parted: Vec<(usize, Option<usize>)> = part
        .pieces()
        .into_iter()
        .map(|piece| (piece, part.parted_from(piece)))
        .collect();
    assert_eq!(
        floor_parted,
        [(FIRST_PIECE, Some(FLOOR))],
        "the arch leaves the bridge's floor in two pieces, its front and its back each one face \
         joined over the arch"
    );
    let start = part.faces_end();
    let listed = [
        (TOP_FRONT, "the top's front edge"),
        (TOP_BACK, "the top's back edge"),
    ];
    let named: Vec<EdgeName> = listed
        .iter()
        .map(|&([one, other], _)| edge_between(&part, one, other))
        .collect();
    let as_one_cut: Vec<(usize, Option<usize>)> = vec![
        (FIRST_PIECE, Some(FLOOR)),
        (FIRST_PIECE + 1, Some(FRONT)),
        (FIRST_PIECE + 2, Some(BACK)),
    ];
    let read = FIRST_PIECE..FIRST_PIECE + PIECES_READ;

    let mut not_so: Vec<String> = Vec::new();
    let mut laid: Vec<(String, BTreeMap<usize, Option<FacePlane>>)> = Vec::new();
    let mut parted: Vec<(String, Vec<Option<usize>>)> = Vec::new();
    for order in [[0, 1], [1, 0]] {
        let names: Vec<EdgeName> = order.iter().map(|&edge| named[edge].clone()).collect();
        let called: Vec<&str> = order.iter().map(|&edge| listed[edge].1).collect();
        let what = format!("the chamfer on {}", called.join(", then "));
        let chamfered = match part.chamfer_edges(&names, LINTEL) {
            Ok(chamfered) => chamfered,
            Err(declined) => {
                not_so.push(format!("{what}: declined, {declined:?}"));
                continue;
            }
        };
        let body = &chamfered.body;
        if (chamfered.taken - LINTEL).abs() >= PLACE {
            not_so.push(format!(
                "{what}: took {}, where {LINTEL} was asked",
                chamfered.taken
            ));
        }
        let pieces: Vec<(usize, Option<usize>)> = body
            .pieces()
            .into_iter()
            .map(|piece| (piece, body.parted_from(piece)))
            .collect();
        if pieces != as_one_cut {
            not_so.push(format!(
                "{what}: its pieces, each with the face it was parted from, are {pieces:?}, \
                 where one cut from the bridge numbers them {as_one_cut:?}"
            ));
        }
        let unanswered: Vec<(usize, usize)> = read
            .clone()
            .filter(|&number| !body.has_face(number))
            .filter_map(|number| body.parted_from(number).map(|from| (number, from)))
            .collect();
        if !unanswered.is_empty() {
            not_so.push(format!(
                "{what}: numbers no face answers to are each parted from a face, {unanswered:?}"
            ));
        }
        let faces = numbers(body)
            .into_iter()
            .map(|number| (as_listed(start, &order, number), body.plane_of(number)))
            .collect();
        laid.push((what.clone(), faces));
        let from: Vec<Option<usize>> = read
            .clone()
            .map(|number| body.parted_from(number))
            .collect();
        parted.push((what, from));
    }

    if let [one, other] = &laid[..] {
        not_so.extend(numbered_otherwise(one, other));
    }
    if let [(one, one_parted), (other, other_parted)] = &parted[..] {
        for (number, (own, theirs)) in read.clone().zip(one_parted.iter().zip(other_parted)) {
            if own != theirs {
                not_so.push(format!(
                    "{number} is parted from {own:?} after {one}, and from {theirs:?} after \
                     {other}"
                ));
            }
        }
    }

    assert!(
        not_so.is_empty(),
        "on the bridge, the chamfer at {LINTEL} on the top's front and back edges takes the \
         lintel whole on each side, leaving the front and the back each in two pieces, numbered \
         as one cut from the bridge numbers them — the floor's second piece kept at \
         {FIRST_PIECE}, the front's the next, the back's after it, no other number parted from \
         anything — whichever is named first, both orders numbering every face alike, each flat \
         read by the edge it was laid on; not so:\n{}",
        not_so.join("\n")
    );
}
