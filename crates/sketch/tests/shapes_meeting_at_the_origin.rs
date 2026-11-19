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
//!   `a_way_round_given_to_a_trait_hung_on_another_s_end_lets_that_other_turn`;
//!   and what the decision taken here implies — the origin joins no group, in
//!   the reading and in the solver alike, as « Orientation » reads in
//!   docs/sketch.md: a shape that reads settled is not turned by a value
//!   retyped beside it —
//!   `a_rectangle_retyped_beside_a_hook_at_the_origin_turns_neither`,
//!   `a_value_retyped_in_one_of_three_shapes_at_the_origin_moves_none_of_the_others`;
//!   and each trait from the origin keeps its own way round —
//!   `two_legs_along_the_axes_from_the_origin_are_settled_by_their_lengths_alone`,
//!   `a_chain_through_the_origin_along_an_axis_is_settled_by_its_two_lengths`,
//!   `a_free_square_trait_from_the_origin_beside_a_settled_rectangle_leaves_only_its_length_free`;
//!   and a value landed in one shape carries no other shape met only at the
//!   origin — `a_free_trait_from_the_origin_stays_put_when_a_rectangle_beside_it_is_retyped`
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

const LANDED: f64 = 1e-6;

/// A shape drawn from the origin, settled alone, and one of its values with
/// where each of its points stands once that value is retyped.
struct Shape {
    points: Vec<PointId>,
    retyped: (DimensionTarget, f64),
    lands_at: Vec<DVec2>,
}

fn rectangle_from_the_origin(sketch: &mut Sketch, height: f64) -> Shape {
    let (b, bottom) = trait_from(sketch, Sketch::ORIGIN, [4.0, 0.0]);
    let (c, right) = trait_from(sketch, b, [4.0, 2.0]);
    let (d, top) = trait_from(sketch, c, [0.0, 2.0]);
    let left = sketch.add_segment(d, Sketch::ORIGIN);
    sketch.set_dimension(DimensionTarget::Length(bottom), 4.0, false);
    sketch.set_dimension(DimensionTarget::Length(right), 2.0, false);
    for (first, second) in [(bottom, right), (right, top), (top, left)] {
        sketch.lay_rule(Constraint::Perpendicular { first, second }, SCALE);
    }
    Shape {
        points: vec![b, c, d],
        retyped: (DimensionTarget::Length(right), height),
        lands_at: vec![
            DVec2::new(4.0, 0.0),
            DVec2::new(4.0, height),
            DVec2::new(0.0, height),
        ],
    }
}

fn hook_from_the_origin(sketch: &mut Sketch, reach: f64) -> Shape {
    let (p, down) = trait_from(sketch, Sketch::ORIGIN, [0.0, -5.0]);
    let (q, across) = trait_from(sketch, p, [-3.0, -5.0]);
    sketch.set_dimension(DimensionTarget::Length(down), 5.0, false);
    sketch.set_dimension(DimensionTarget::Length(across), 3.0, false);
    sketch.lay_rule(
        Constraint::Perpendicular {
            first: down,
            second: across,
        },
        SCALE,
    );
    Shape {
        points: vec![p, q],
        retyped: (DimensionTarget::Length(down), reach),
        lands_at: vec![DVec2::new(0.0, -reach), DVec2::new(-3.0, -reach)],
    }
}

fn trait_leftwards_from_the_origin(sketch: &mut Sketch, length: f64) -> Shape {
    let (end, along) = trait_from(sketch, Sketch::ORIGIN, [-6.0, 0.0]);
    sketch.set_dimension(DimensionTarget::Length(along), 6.0, false);
    Shape {
        points: vec![end],
        retyped: (DimensionTarget::Length(along), length),
        lands_at: vec![DVec2::new(-length, 0.0)],
    }
}

/// Retypes the value of `shapes[retyped]`, resolves, and holds every point of
/// every shape to where it must stand: where it stood, or, for the shape
/// retyped, where its value carries it without turning it.
fn assert_a_retyped_value_turns_nothing(
    sketch: &mut Sketch,
    shapes: &[Shape],
    retyped: usize,
    case: &str,
) {
    let every_point: Vec<PointId> = shapes
        .iter()
        .flat_map(|shape| shape.points.clone())
        .collect();
    assert_settled(
        sketch,
        &every_point,
        &format!("{case}, before the value is retyped"),
    );
    assert_eq!(
        sketch.freedom(SCALE).degrees_of_freedom,
        0,
        "{case}: the drawing reads settled through and through before the value is retyped",
    );
    let stood: Vec<Vec<DVec2>> = shapes
        .iter()
        .map(|shape| {
            shape
                .points
                .iter()
                .map(|point| sketch.point(*point))
                .collect()
        })
        .collect();

    let (target, value) = shapes[retyped].retyped;
    sketch.set_dimension(target, value, false);
    sketch.resolve(SCALE);

    for (index, shape) in shapes.iter().enumerate() {
        let wanted = match index == retyped {
            true => &shape.lands_at,
            false => &stood[index],
        };
        for (point, place) in shape.points.iter().zip(wanted) {
            let found = sketch.point(*point);
            assert!(
                found.distance(*place) < LANDED,
                "{case}: a point that stood settled at {place} is at {found} after the resolve",
            );
        }
    }
}

#[test]
fn a_rectangle_retyped_beside_a_hook_at_the_origin_turns_neither() {
    for hook_first in [false, true] {
        for height in [6.0, 0.4, 11.0] {
            let case = format!("hook drawn first: {hook_first}, height retyped to {height}");
            let mut sketch = Sketch::new(WorkPlane::XY);
            let shapes = match hook_first {
                true => {
                    let hook = hook_from_the_origin(&mut sketch, 5.0);
                    [rectangle_from_the_origin(&mut sketch, height), hook]
                }
                false => {
                    let rectangle = rectangle_from_the_origin(&mut sketch, height);
                    [rectangle, hook_from_the_origin(&mut sketch, 5.0)]
                }
            };
            assert_a_retyped_value_turns_nothing(&mut sketch, &shapes, 0, &case);
        }
    }
}

#[test]
fn a_value_retyped_in_one_of_three_shapes_at_the_origin_moves_none_of_the_others() {
    type Draw = fn(&mut Sketch, f64) -> Shape;
    let draws: [(&str, Draw, f64); 3] = [
        ("rectangle", rectangle_from_the_origin, 6.0),
        ("hook", hook_from_the_origin, 7.0),
        ("leftward trait", trait_leftwards_from_the_origin, 9.0),
    ];
    for first in 0..3 {
        for retyped in 0..3 {
            let order: Vec<usize> = (0..3).map(|step| (first + step) % 3).collect();
            let names: Vec<&str> = order.iter().map(|index| draws[*index].0).collect();
            let case = format!("drawn {names:?}, the {} retyped", draws[retyped].0);
            let mut sketch = Sketch::new(WorkPlane::XY);
            let mut shapes: Vec<(usize, Shape)> = order
                .iter()
                .map(|index| (*index, (draws[*index].1)(&mut sketch, draws[*index].2)))
                .collect();
            shapes.sort_by_key(|(index, _)| *index);
            let shapes: Vec<Shape> = shapes.into_iter().map(|(_, shape)| shape).collect();
            assert_a_retyped_value_turns_nothing(&mut sketch, &shapes, retyped, &case);
        }
    }
}

#[test]
fn two_legs_along_the_axes_from_the_origin_are_settled_by_their_lengths_alone() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (across, along_u) = trait_from(&mut sketch, Sketch::ORIGIN, [5.0, 0.0]);
    let (up, along_v) = trait_from(&mut sketch, Sketch::ORIGIN, [0.0, 3.0]);
    sketch.set_dimension(DimensionTarget::Length(along_u), 5.0, false);
    sketch.set_dimension(DimensionTarget::Length(along_v), 3.0, false);

    assert_settled(&sketch, &[across, up], "an L with no right angle");
    assert_eq!(sketch.freedom(SCALE).degrees_of_freedom, 0);
}

#[test]
fn a_chain_through_the_origin_along_an_axis_is_settled_by_its_two_lengths() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(-5.0, 0.0));
    let inward = sketch.add_segment(west, Sketch::ORIGIN);
    let (east, outward) = trait_from(&mut sketch, Sketch::ORIGIN, [5.0, 0.0]);
    sketch.set_dimension(DimensionTarget::Length(inward), 5.0, false);
    sketch.set_dimension(DimensionTarget::Length(outward), 5.0, false);

    assert_settled(&sketch, &[west, east], "a chain through the origin");
    assert_eq!(sketch.freedom(SCALE).degrees_of_freedom, 0);
}

#[test]
fn a_free_square_trait_from_the_origin_beside_a_settled_rectangle_leaves_only_its_length_free() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let rectangle = rectangle_from_the_origin(&mut sketch, 2.0);
    let (end, _) = trait_from(&mut sketch, Sketch::ORIGIN, [-6.0, 0.0]);

    assert_settled(&sketch, &rectangle.points, "beside a free leftward trait");
    assert_loose(&sketch, &[end], "a leftward trait with no length");
    assert_eq!(
        sketch.freedom(SCALE).degrees_of_freedom,
        1,
        "the trait keeps its way round and is free only in length",
    );
}

#[test]
fn a_free_trait_from_the_origin_stays_put_when_a_rectangle_beside_it_is_retyped() {
    for trait_first in [false, true] {
        let case = format!("free trait drawn first: {trait_first}");
        let mut sketch = Sketch::new(WorkPlane::XY);
        let mut free = None;
        if trait_first {
            free = Some(trait_from(&mut sketch, Sketch::ORIGIN, [-3.0, 5.0]).0);
        }
        let rectangle = rectangle_from_the_origin(&mut sketch, 6.0);
        let free = free.unwrap_or_else(|| trait_from(&mut sketch, Sketch::ORIGIN, [-3.0, 5.0]).0);

        let (target, height) = rectangle.retyped;
        sketch.set_dimension(target, height, false);
        sketch.land_value(target, SCALE);

        let found = sketch.point(free);
        assert!(
            found.distance(DVec2::new(-3.0, 5.0)) < LANDED,
            "{case}: the free trait's end is at {found} after the rectangle was retyped",
        );
        for (point, place) in rectangle.points.iter().zip(&rectangle.lands_at) {
            let found = sketch.point(*point);
            assert!(
                found.distance(*place) < LANDED,
                "{case}: the rectangle's corner meant for {place} is at {found}",
            );
        }
    }
}
