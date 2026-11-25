//! A chamfer and a fillet leave the tolerance the part decides at as it was
//! (#1023): every solid the step lays on the part — an edge's fill or cut, a
//! corner's closing or clearing, a closed round — stands past the part,
//! raised over its box and trimmed by boxes clear of it, and is read at the
//! part's scale rather than at its own. Held on the block of
//! `a_chamfer_and_a_fillet_fill_concave_edges_meeting_at_a_corner.rs`, 30 by
//! 20 by 10 with a pocket 10 by 10 sunk 5 into its top, which reaches 30:
//! the three concave edges at the pocket's bottom corner, the three convex
//! edges at the block's top corner, and one or two of the convex edges
//! there, each chamfered and filleted in one step; on #790's bar, 30 long and
//! 10 square, its four long edges rounded into one cylinder; and on a
//! pyramid cut by tools reaching no further than 11, its four hips at its
//! apex chamfered and rounded, the apex cleared before they are laid. Read
//! through `Body::on_a_plane`, the kernel's one tolerance, a billionth of the
//! reach the body carries, the same to the bit as the part's before the
//! step. The bodies themselves are held by the files that build them; here
//! each step is held to have done its work — taken whole, the body exact,
//! closed and uncrossed, holding more matter than the part where the edges
//! are concave and less where they are convex, and the matter arithmetic
//! gives where it gives it in closed form.

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

/// The values every step on the pocketed block is laid at: those of the
/// corner file, and one between.
const VALUES: [f64; 3] = [1.0, 2.0, 2.5];

/// Both cuts a step lays.
const BOTH: [Cut; 2] = [Cut::Chamfer, Cut::Fillet];

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
    let frame = Frame {
        origin: DVec3::Z * from,
        ..ground()
    };
    tool_raised_on(part, outline, frame, DVec3::Z * height)
}

/// The tool an outline drawn on `frame` raises along `travel`: its faces
/// numbered past the part's, its floor first, then its top, then its walls,
/// one for each run of the outline.
fn tool_raised_on(part: &Body, outline: Contour, frame: Frame, travel: DVec3) -> Body {
    let profile = Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    part.tool_raised(&profile, frame, travel)
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

/// Every step of each of `cuts` at each of `values` on the edges between
/// each pair of `between` of `part`, in one step, that leaves the body
/// deciding at another tolerance than the part did, said with the reach each
/// carries; the part, before the step, deciding at the tolerance a reach of
/// `reach` asks. Each step is held to have done its work: taken whole, the
/// body exact, closed and uncrossed, holding more matter than the part where
/// `edges` are concave and less where convex, and what `promised` gives,
/// where it gives anything, within [`EXACT`].
fn grown_by_a_step_on(
    (part, reach): (&Body, f64),
    between: &[[usize; 2]],
    (edges, promised): (Edges, &dyn Fn(Cut, f64) -> Option<f64>),
    (cuts, values): (&[Cut], &[f64]),
    what: &str,
) -> Vec<String> {
    let before = part.on_a_plane(0.0);
    let own = Scale::of(reach).eps();
    assert_eq!(
        before, own,
        "{what}, before the step: the part decides at a tolerance of {before}, where a reach of \
         {reach} asks {own}"
    );
    let names: Vec<EdgeName> = between
        .iter()
        .map(|&faces| edge_between(part, faces))
        .collect();
    let mut grown = Vec::new();
    for &cut in cuts {
        for &value in values {
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
                     {}, where before the step the part decided at {before}, a share of a \
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
        (&pocketed(), 30.0),
        &[
            [POCKET_FLOOR, POCKET_FRONT],
            [POCKET_FLOOR, POCKET_RIGHT],
            [POCKET_FRONT, POCKET_RIGHT],
        ],
        (Edges::Concave, &no_volume),
        (&BOTH, &VALUES),
        "the three edges at the pocket's front right bottom corner",
    ));
}

#[test]
fn a_chamfer_and_a_fillet_on_the_three_edges_of_a_block_s_convex_corner_leave_the_part_s_tolerance_as_it_was()
 {
    none_grown(grown_by_a_step_on(
        (&pocketed(), 30.0),
        &[[TOP, FRONT], [TOP, RIGHT], [FRONT, RIGHT]],
        (Edges::Convex, &no_volume),
        (&BOTH, &VALUES),
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
        (&part, 30.0),
        &[[TOP, FRONT]],
        (Edges::Convex, &alone),
        (&BOTH, &VALUES),
        "the block's top front edge",
    );
    let two = grown_by_a_step_on(
        (&part, 30.0),
        &[[TOP, FRONT], [TOP, RIGHT]],
        (Edges::Convex, &no_volume),
        (&BOTH, &VALUES),
        "the block's top front and right edges, meeting at its corner",
    );

    none_grown([one, two].concat());
}

/// The matter the [`pocketed`] block holds: its 30 × 20 × 10 less the
/// pocket's 10 × 10 × 5.
fn part_volume() -> f64 {
    30.0 * 20.0 * 10.0 - 10.0 * 10.0 * 5.0
}

/// #790's bar: a block 30 long along X and 10 square, standing on the ground
/// at the origin, its floor numbered nought, its top one and its sides front,
/// right, back and left two to five. Its end at x = 30 sets its reach.
fn bar() -> Body {
    let nothing = Body::default();
    nothing
        .union(&tool_raised(
            &nothing,
            Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 10.0)),
            0.0,
            10.0,
        ))
        .expect("the bar is built")
}

#[test]
fn a_fillet_on_the_four_long_edges_of_a_square_bar_closing_them_into_one_cylinder_leaves_the_part_s_tolerance_as_it_was()
 {
    let (floor, back) = (0, 4);
    let cylinder = |_: Cut, radius: f64| Some(PI * radius * radius * 30.0);
    none_grown(grown_by_a_step_on(
        (&bar(), 30.0),
        &[[TOP, FRONT], [floor, back], [TOP, back], [floor, FRONT]],
        (Edges::Convex, &cylinder),
        (&[Cut::Fillet], &[5.0]),
        "the four long edges of the bar 10 square, rounded at half its side",
    ));
}

/// A square pyramid on a base 10 by 10, its apex 5 over the middle: a block
/// a half wider than the base all round and a half higher than the apex, cut
/// by the four planes through the apex and an edge of the base, each by a
/// prism whose triangle has one side on its plane, raised along that edge's
/// line, reaching no further than 11 — so that the part carries a reach of
/// 11, where a slab as wide as the block would carry its own. Its slopes,
/// front, back, left and right, are each prism's first wall, numbered eight,
/// thirteen, eighteen and twenty-three; its base the block's floor.
fn pyramid() -> Body {
    let nothing = Body::default();
    let block = nothing
        .union(&tool_raised(
            &nothing,
            Contour::rectangle(DVec2::splat(-0.5), DVec2::splat(10.5)),
            0.0,
            5.5,
        ))
        .expect("the block is built");
    let triangle = |corners: [[f64; 2]; 3]| {
        Contour::straight(corners.iter().map(|&[u, v]| DVec2::new(u, v)).collect())
    };
    let along_x = Frame {
        origin: DVec3::X * -1.0,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    let along_y = Frame {
        origin: DVec3::Y * -1.0,
        u: DVec3::Z,
        v: DVec3::X,
    };
    let cuts = [
        (along_x, [[-1.0, -1.0], [6.5, 6.5], [-1.0, 6.5]], DVec3::X),
        (along_x, [[3.5, 6.5], [11.0, -1.0], [11.0, 6.5]], DVec3::X),
        (along_y, [[6.5, 6.5], [-1.0, -1.0], [6.5, -1.0]], DVec3::Y),
        (along_y, [[-1.0, 11.0], [6.5, 3.5], [6.5, 11.0]], DVec3::Y),
    ];
    cuts.into_iter()
        .fold(block, |part, (frame, corners, along)| {
            let tool = tool_raised_on(&part, triangle(corners), frame, along * 12.0);
            part.difference(&tool)
                .expect("a plane through the apex cuts the block")
        })
}

#[test]
fn a_chamfer_and_a_fillet_on_the_four_hips_at_a_pyramid_s_apex_leave_the_part_s_tolerance_as_it_was()
 {
    let part = pyramid();
    let (front, back, left, right) = (8, 13, 18, 23);
    let volume = part.volume();
    assert!(
        (volume - 500.0 / 3.0).abs() <= EXACT * volume,
        "the pyramid holds {volume}, where its base of 100 and height of 5 make 500 / 3"
    );
    let hips = [[front, left], [front, right], [back, right], [back, left]];
    let chamfered = grown_by_a_step_on(
        (&part, 11.0),
        &hips,
        (Edges::Convex, &no_volume),
        (&[Cut::Chamfer], &[1.0, 2.5]),
        "the four hips at the pyramid's apex",
    );
    let filleted = grown_by_a_step_on(
        (&part, 11.0),
        &hips,
        (Edges::Convex, &no_volume),
        (&[Cut::Fillet], &[0.5, 1.0]),
        "the four hips at the pyramid's apex",
    );

    none_grown([chamfered, filleted].concat());
}
