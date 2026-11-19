//! Shapes that meet only at the origin each keep their own way round.
//!
//! Closes #1039.
//! - The rectangle stays settled when another shape is attached at the origin,
//!   and the freedom count is 3 — reworded: the count is the chain's own, 4,
//!   since a two-trait chain hinged at the origin with nothing on it leaves
//!   both its free points free —
//!   `a_rectangle_settled_from_the_origin_stays_settled_when_a_chain_is_joined_there`,
//!   and the comment's two cases,
//!   `a_square_settled_from_the_origin_stays_settled_when_a_trait_is_joined_there`,
//!   `a_straight_chain_settled_from_the_origin_stays_settled_beside_a_free_trait_drawn_from_there`;
//!   what attaching may still loosen, away from the origin, reads loose —
//!   `a_way_round_given_to_a_trait_hung_on_another_s_end_lets_that_other_turn`
//! - The cases above hold in #501's campaign — no test: held by
//!   `crates/sketch/tests/what_random_sketches_found.rs`, under the same issue

use cao_sketch::{
    AxisToward, Constraint, DimensionTarget, PointId, SegmentId, Sketch, SketchAxis, Toward,
    WorkPlane,
};
use glam::DVec2;

const SCALE: f64 = 1.0;

fn trait_from(sketch: &mut Sketch, from: PointId, to: [f64; 2]) -> (PointId, SegmentId) {
    let end = sketch.add_point(DVec2::from(to));
    (end, sketch.add_segment(from, end))
}

fn assert_settled(sketch: &Sketch, points: &[PointId], case: &str) {
    let settled = sketch.settled_points(SCALE);
    for point in points {
        assert!(
            settled[point.0],
            "{case}: the point at {} reads unsettled",
            sketch.point(*point),
        );
    }
}

fn assert_loose(sketch: &Sketch, points: &[PointId], case: &str) {
    let settled = sketch.settled_points(SCALE);
    for point in points {
        assert!(
            !settled[point.0],
            "{case}: the point at {} reads settled",
            sketch.point(*point),
        );
    }
}

#[test]
fn a_rectangle_settled_from_the_origin_stays_settled_when_a_chain_is_joined_there() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (b, bottom) = trait_from(&mut sketch, Sketch::ORIGIN, [4.0, 0.0]);
    let (c, right) = trait_from(&mut sketch, b, [4.0, 2.0]);
    let (d, top) = trait_from(&mut sketch, c, [0.0, 2.0]);
    let left = sketch.add_segment(d, Sketch::ORIGIN);
    sketch.set_dimension(DimensionTarget::Length(bottom), 4.0, false);
    sketch.set_dimension(DimensionTarget::Length(right), 2.0, false);
    for (first, second) in [(bottom, right), (right, top), (top, left)] {
        sketch.lay_rule(Constraint::Perpendicular { first, second }, SCALE);
    }
    assert_settled(&sketch, &[b, c, d], "the rectangle alone");

    let start = sketch.add_point(DVec2::new(6.0, 3.0));
    let (bend, _) = trait_from(&mut sketch, start, [9.0, 5.0]);
    let (end, _) = trait_from(&mut sketch, bend, [12.0, 3.0]);
    assert!(sketch.join_points(start, Sketch::ORIGIN, SCALE));

    assert_settled(&sketch, &[b, c, d], "the chain joined at the origin");
    assert_loose(&sketch, &[bend, end], "the chain joined at the origin");
    assert_eq!(
        sketch.freedom(SCALE).degrees_of_freedom,
        4,
        "the freedom left is the chain's own",
    );
}

#[test]
fn a_square_settled_from_the_origin_stays_settled_when_a_trait_is_joined_there() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [[3.5, 0.0], [3.5, 3.5], [0.0, 3.5]].map(|at| sketch.add_point(DVec2::from(at)));
    let ring = [Sketch::ORIGIN, corners[0], corners[1], corners[2]];
    let sides: Vec<SegmentId> = (0..4)
        .map(|index| sketch.add_construction_segment(ring[index], ring[(index + 1) % 4]))
        .collect();
    for side in &sides {
        sketch.set_dimension(DimensionTarget::Length(*side), 3.5, false);
    }
    sketch.lay_rule(
        Constraint::Perpendicular {
            first: sides[0],
            second: sides[3],
        },
        SCALE,
    );
    assert_settled(&sketch, &corners, "the square alone");

    let start = sketch.add_point(DVec2::new(4.0, 3.0));
    let (end, _) = trait_from(&mut sketch, start, [4.0, 8.0]);
    assert!(sketch.join_points(start, Sketch::ORIGIN, SCALE));

    assert_settled(&sketch, &corners, "the trait joined at the origin");
    assert_loose(&sketch, &[end], "the trait joined at the origin");
    assert_eq!(
        sketch.freedom(SCALE).degrees_of_freedom,
        2,
        "the freedom left is the trait's own",
    );
}

fn straight_chain_from_the_origin(sketch: &mut Sketch) -> [PointId; 2] {
    let (bend, first) = trait_from(sketch, Sketch::ORIGIN, [0.0, -10.0]);
    let (end, second) = trait_from(sketch, bend, [0.0, -25.0]);
    sketch.set_dimension(DimensionTarget::Length(first), 10.0, false);
    sketch.set_dimension(DimensionTarget::Length(second), 15.0, false);
    sketch.lay_rule(Constraint::Parallel { first, second }, SCALE);
    [bend, end]
}

#[test]
fn a_straight_chain_settled_from_the_origin_stays_settled_beside_a_free_trait_drawn_from_there() {
    for trait_first in [false, true] {
        let case = match trait_first {
            true => "the free trait drawn first",
            false => "the free trait drawn after",
        };
        let mut sketch = Sketch::new(WorkPlane::XY);
        let mut free = None;
        if trait_first {
            free = Some(trait_from(&mut sketch, Sketch::ORIGIN, [-3.0, 7.0]).0);
        }
        let chain = straight_chain_from_the_origin(&mut sketch);
        if !trait_first {
            assert_settled(&sketch, &chain, "the chain alone");
            free = Some(trait_from(&mut sketch, Sketch::ORIGIN, [-3.0, 7.0]).0);
        }
        let free = free.expect("the free trait is drawn in both cases");

        assert_settled(&sketch, &chain, case);
        assert_loose(&sketch, &[free], case);
        assert_eq!(
            sketch.freedom(SCALE).degrees_of_freedom,
            2,
            "{case}: the freedom left is the free trait's own",
        );
    }
}

#[test]
fn a_way_round_given_to_a_trait_hung_on_another_s_end_lets_that_other_turn() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (hinge, held) = trait_from(&mut sketch, Sketch::ORIGIN, [0.0, -10.0]);
    sketch.set_dimension(DimensionTarget::Length(held), 10.0, false);
    assert_settled(&sketch, &[hinge], "the square trait alone");

    let (end, hung) = trait_from(&mut sketch, hinge, [5.0, -15.0]);
    let way_round = DimensionTarget::AxisAngle {
        segment: hung,
        segment_toward: Toward::End,
        axis: SketchAxis::U,
        axis_toward: AxisToward::Positive,
    };
    let angle = sketch
        .opening(way_round)
        .expect("the hung trait has an angle against the axis");
    sketch.set_dimension(way_round, angle, false);

    assert_loose(
        &sketch,
        &[hinge, end],
        "the hung trait's angle frees the square one to swing about the origin",
    );
}
