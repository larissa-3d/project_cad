//! What a drag does to a fixed point: it moves it only when it grabbed it.
//!
//! Closes #1041.
//! - The fixed point stays; the dragged point stops short —
//!   `a_trait_held_whole_stops_short_rather_than_move_its_fixed_end`; a fixed
//!   point grabbed directly still follows the hand and is fixed where it is
//!   let go of — `a_fixed_end_grabbed_follows_the_hand_and_stays_where_it_is_let_go`;
//!   and the same trait with nothing fixed still travels whole —
//!   `a_trait_held_whole_with_nothing_fixed_still_travels_with_the_hand`
//! - The case above holds in #501's campaign — no test: not in this file;
//!   `what_random_sketches_found.rs` holds the campaign's drawing,
//!   `a_drag_never_moves_a_fixed_point_it_did_not_grab`, its `ignore` taken off

use cao_sketch::{
    Constraint, DimensionTarget, Element, LengthOutcome, PointId, Sketch, SketchAxis, WorkPlane,
};
use glam::DVec2;

const AFTER_THE_SOLVER: f64 = 1e-6;
const THE_HAND: DVec2 = DVec2::new(2.456, 0.0);

/// The campaign's drawing: a level trait from (2, 2) to (8, 2), held along
/// the U axis and 6 long, its end at (8, 2) fixed when `fixed` says so.
fn a_level_trait(fixed: bool) -> (Sketch, PointId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(2.0, 2.0));
    let end = sketch.add_point(DVec2::new(8.0, 2.0));
    let segment = sketch.add_segment(start, end);
    if fixed {
        sketch.add_constraint(Constraint::Fixed {
            element: Element::Point(end),
        });
    }
    sketch.add_constraint(Constraint::AxisParallel {
        segment,
        axis: SketchAxis::U,
    });
    sketch.set_dimension(DimensionTarget::Length(segment), 6.0, false);
    sketch.resolve(1.0);
    (sketch, start, end)
}

fn drag(sketch: &mut Sketch, point: PointId, by: DVec2) -> LengthOutcome {
    let pull = sketch.pull(point, 1.0);
    let to = sketch.point(point) + by;
    let landing = sketch.slide(point, to);
    sketch.settle_pulled(&pull, landing, 1.0)
}

fn assert_at(sketch: &Sketch, point: PointId, wanted: DVec2, what: &str) {
    assert!(
        sketch.point(point).distance(wanted) < AFTER_THE_SOLVER,
        "{what} is at {}, not {wanted}",
        sketch.point(point)
    );
}

#[test]
fn a_trait_held_whole_stops_short_rather_than_move_its_fixed_end() {
    let (mut sketch, start, end) = a_level_trait(true);

    drag(&mut sketch, start, THE_HAND);

    assert_at(&sketch, end, DVec2::new(8.0, 2.0), "the fixed end");
    assert_at(
        &sketch,
        start,
        DVec2::new(2.0, 2.0),
        "the dragged end, which has nowhere to go,",
    );
}

#[test]
fn a_fixed_end_grabbed_follows_the_hand_and_stays_where_it_is_let_go() {
    let (mut sketch, start, end) = a_level_trait(true);
    let let_go = DVec2::new(8.0, 2.0) + THE_HAND;

    drag(&mut sketch, end, THE_HAND);

    assert_at(&sketch, end, let_go, "the grabbed fixed end");
    assert_at(
        &sketch,
        start,
        DVec2::new(2.0, 2.0) + THE_HAND,
        "the other end",
    );

    drag(&mut sketch, start, THE_HAND);

    assert_at(
        &sketch,
        end,
        let_go,
        "after the other end was dragged, the fixed end",
    );
}

#[test]
fn a_trait_held_whole_with_nothing_fixed_still_travels_with_the_hand() {
    let (mut sketch, start, end) = a_level_trait(false);

    drag(&mut sketch, start, THE_HAND);

    assert_at(
        &sketch,
        start,
        DVec2::new(2.0, 2.0) + THE_HAND,
        "the dragged end",
    );
    assert_at(
        &sketch,
        end,
        DVec2::new(8.0, 2.0) + THE_HAND,
        "the other end",
    );
}
