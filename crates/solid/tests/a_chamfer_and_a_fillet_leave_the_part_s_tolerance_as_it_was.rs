//! A chamfer and a fillet leave the tolerance the part decides at as it was
//! (#1023): every solid the step lays on the part — an edge's fill or cut, a
//! corner's closing or clearing — stands past the part, raised over its box
//! and trimmed by boxes clear of it, and is read at the part's scale rather
//! than at its own. Held on the block of
//! `a_chamfer_and_a_fillet_fill_concave_edges_meeting_at_a_corner.rs`, 30 by
//! 20 by 10 with a pocket 10 by 10 sunk 5 into its top, which reaches 30:
//! the three concave edges at the pocket's bottom corner, the three convex
//! edges at the block's top corner, and one or two of the convex edges
//! there, each chamfered and filleted in one step. Read through
//! `Body::on_a_plane`, the kernel's one tolerance, a billionth of the reach
//! the body carries, the same to the bit as the part's before the step. The
//! bodies themselves are held by the files that build them; here each step
//! is held to have done its work — taken whole, the body exact, closed and
//! uncrossed, holding more matter than the part where the edges are concave
//! and less where they are convex, a lone convex edge exactly what its
//! section along its length takes.

use std::f64::consts::PI;

use cao_solid::brep::Scale;
use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, uncrossed};
use cao_solid::{Body, EdgeName, Loop};
use glam::{DVec2, DVec3};

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far a value taken may stand from the one asked.
const PLACE: f64 = 1e-9;

/// The values every step is laid at: those of the corner file, and one
/// between.
const VALUES: [f64; 3] = [1.0, 2.0, 2.5];

/// The numbers the block's faces answer to: its top, its front and its
/// right side; the pocket's floor, its front wall and its right wall.
const TOP: usize = 1;
const FRONT: usize = 2;
const RIGHT: usize = 3;
const POCKET_FLOOR: usize = 6;
const POCKET_FRONT: usize = 8;
const POCKET_RIGHT: usize = 9;

/// The ground's plane, which a part is drawn on.
fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

/// The tool an outline drawn on the ground raises from `from` by `height`,
/// as `cao_part` raises a step.
fn tool_raised(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
    let profile = Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let frame = Frame {
        origin: DVec3::Z * from,
        ..ground()
    };
    part.tool_raised(&profile, frame, DVec3::Z * height)
        .expect("the tool raises")
}

/// The block 30 by 20 by 10 with a pocket 10 by 10 sunk 5 into the middle of
/// its top, as the corner file builds it: the block's floor numbered nought,
/// its top one, its walls front, right, back and left two to five; the
/// pocket's floor six, its walls eight to eleven, the tool's top, standing
/// above the block, gone.
fn pocketed() -> Body {
    let nothing = Body::default();
    let block = nothing
        .union(&tool_raised(
            &nothing,
            Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 20.0)),
            0.0,
            10.0,
        ))
        .expect("the block is built");
    let pocket = tool_raised(
        &block,
        Contour::rectangle(DVec2::new(10.0, 5.0), DVec2::new(20.0, 15.0)),
        5.0,
        6.0,
    );
    block.difference(&pocket).expect("the pocket is cut")
}

/// The name of the one edge between the faces numbered `first` and `second`.
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

#[derive(Clone, Copy, Debug)]
enum Cut {
    Chamfer,
    Fillet,
}

/// Which way the matter goes: a fill of concave edges adds it, a cut of
/// convex ones takes it away.
#[derive(Clone, Copy, PartialEq)]
enum Edges {
    Concave,
    Convex,
}

/// Every step of a chamfer and a fillet of each of [`VALUES`] on the edges
/// between each pair of `between` of `part`, in one step, that leaves the
/// body deciding at another tolerance than the part did, said with the reach
/// each carries. Each step is held to have done its work: taken whole, the
/// body exact, closed and uncrossed, holding more matter than the part where
/// `edges` are concave and less where convex, and what `promised` gives,
/// where it gives anything, within [`EXACT`].
fn grown_by_a_step_on(
    part: &Body,
    between: &[[usize; 2]],
    (edges, promised): (Edges, &dyn Fn(Cut, f64) -> Option<f64>),
    what: &str,
) -> Vec<String> {
    let before = part.on_a_plane(0.0);
    let own = Scale::of(30.0).eps();
    assert_eq!(
        before, own,
        "{what}, before the step: the part decides at a tolerance of {before}, where its box of \
         30 asks {own}"
    );
    let names: Vec<EdgeName> = between
        .iter()
        .map(|&faces| edge_between(part, faces))
        .collect();
    let mut grown = Vec::new();
    for cut in [Cut::Chamfer, Cut::Fillet] {
        for value in VALUES {
            let what = format!("{what}, the {cut:?} of {value}");
            let laid = match cut {
                Cut::Chamfer => part
                    .chamfer_edges(&names, value)
                    .map(|step| (step.body, step.asked, step.taken)),
                Cut::Fillet => part
                    .fillet_edges(&names, value)
                    .map(|step| (step.body, step.asked, step.taken)),
            };
            let (body, asked, taken) =
                laid.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

            assert!(
                (asked - value).abs() < PLACE && (taken - value).abs() < PLACE,
                "{what}: asked {asked} and took {taken}, where {value} fits"
            );
            assert!(
                body.is_exact(),
                "{what}: the exact kernel computed the body"
            );
            let triangles = body.triangles();
            assert_eq!(closed(&triangles), Ok(()), "{what}");
            assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
            let (was, is) = (part.volume(), body.volume());
            assert!(
                (is > was) == (edges == Edges::Concave) && is != was,
                "{what}: the body holds {is}, where the part held {was}"
            );
            if let Some(promised) = promised(cut, value) {
                assert!(
                    (is - promised).abs() <= EXACT * promised,
                    "{what}: the body holds {is}, where arithmetic promised {promised}"
                );
            }
            let after = body.on_a_plane(0.0);
            if after != before {
                grown.push(format!(
                    "{what}: the body decides at a tolerance of {after}, a share of a reach of \
                     {}, where before the step the part decided at {before}, its own box's \
                     reach of {}",
                    after / Scale::RELATIVE,
                    before / Scale::RELATIVE
                ));
            }
        }
    }
    grown
}

/// Holds that no step of `grown` left the body deciding at another
/// tolerance than its part did.
fn none_grown(grown: Vec<String>) {
    assert!(
        grown.is_empty(),
        "a step left the part deciding at a coarser tolerance than before it, and every later \
         step would be decided and drawn coarser than the part's size asks:\n{}",
        grown.join("\n")
    );
}

/// Nothing promised but which way the matter goes.
fn no_volume(_: Cut, _: f64) -> Option<f64> {
    None
}

#[test]
fn a_chamfer_and_a_fillet_on_the_three_edges_of_a_pocket_s_bottom_corner_leave_the_part_s_tolerance_as_it_was()
 {
    none_grown(grown_by_a_step_on(
        &pocketed(),
        &[
            [POCKET_FLOOR, POCKET_FRONT],
            [POCKET_FLOOR, POCKET_RIGHT],
            [POCKET_FRONT, POCKET_RIGHT],
        ],
        (Edges::Concave, &no_volume),
        "the three edges at the pocket's front right bottom corner",
    ));
}

#[test]
fn a_chamfer_and_a_fillet_on_the_three_edges_of_a_block_s_convex_corner_leave_the_part_s_tolerance_as_it_was()
 {
    none_grown(grown_by_a_step_on(
        &pocketed(),
        &[[TOP, FRONT], [TOP, RIGHT], [FRONT, RIGHT]],
        (Edges::Convex, &no_volume),
        "the three edges at the block's top front right corner",
    ));
}

#[test]
fn a_chamfer_and_a_fillet_on_one_convex_edge_or_two_meeting_at_a_corner_leave_the_part_s_tolerance_as_it_was()
 {
    let part = pocketed();
    let length = 30.0;
    let alone = move |cut: Cut, value: f64| {
        let section = match cut {
            Cut::Chamfer => value * value / 2.0,
            Cut::Fillet => value * value * (1.0 - PI / 4.0),
        };
        Some(part_volume() - section * length)
    };
    let one = grown_by_a_step_on(
        &part,
        &[[TOP, FRONT]],
        (Edges::Convex, &alone),
        "the block's top front edge",
    );
    let two = grown_by_a_step_on(
        &part,
        &[[TOP, FRONT], [TOP, RIGHT]],
        (Edges::Convex, &no_volume),
        "the block's top front and right edges, meeting at its corner",
    );

    none_grown([one, two].concat());
}

/// The matter the [`pocketed`] block holds: its 30 × 20 × 10 less the
/// pocket's 10 × 10 × 5.
fn part_volume() -> f64 {
    30.0 * 20.0 * 10.0 - 10.0 * 10.0 * 5.0
}
