//! What every rule the solver writes an equation for does today, held while a
//! drawing is dragged: the net `docs/code-map.md` asks for before a rule is
//! added to `rule_equations`, which carries no test of its own.

use cao_sketch::{
    Constraint, DimensionTarget, Element, LaidFrom, LengthOutcome, PointId, SegmentId, Sketch,
    SketchAxis, WorkPlane,
};
use glam::DVec2;

const AFTER_THE_SOLVER: f64 = 1e-6;

fn drag(sketch: &mut Sketch, point: PointId, by: DVec2) -> LengthOutcome {
    let pull = sketch.pull(point, 1.0);
    let to = sketch.point(point) + by;
    let landing = sketch.slide(point, to);
    sketch.settle_pulled(&pull, landing, 1.0)
}

fn direction(sketch: &Sketch, segment: SegmentId) -> DVec2 {
    let (from, to) = sketch.endpoints(segment);
    (to - from).normalize()
}

fn distance_to_line(sketch: &Sketch, place: DVec2, segment: SegmentId) -> f64 {
    let (from, to) = sketch.endpoints(segment);
    (to - from).normalize().perp_dot(place - from)
}

fn a_trait(sketch: &mut Sketch, from: DVec2, to: DVec2) -> (SegmentId, PointId, PointId) {
    let start = sketch.add_point(from);
    let end = sketch.add_point(to);
    (sketch.add_segment(start, end), start, end)
}

fn assert_landed(sketch: &Sketch, point: PointId, wanted: DVec2) {
    assert!(
        sketch.point(point).distance(wanted) < 1e-3,
        "the dragged point is at {}, not where the hand took it, {wanted}",
        sketch.point(point)
    );
}

fn assert_along_its_own_line(sketch: &Sketch, point: PointId, wanted: DVec2) {
    assert!(
        sketch.point(point).distance(wanted) < 1e-3,
        "the dragged end is at {}, not slid along its trait to {wanted}",
        sketch.point(point)
    );
}

#[test]
fn two_traits_held_square_stay_square_when_an_end_is_dragged() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(1.0, 1.0));
    let east = sketch.add_point(DVec2::new(5.0, 1.0));
    let north = sketch.add_point(DVec2::new(1.0, 4.0));
    let first = sketch.add_segment(corner, east);
    let second = sketch.add_segment(corner, north);
    sketch.add_constraint(Constraint::Perpendicular { first, second });
    let to = sketch.point(east) + DVec2::new(1.0, 2.0);

    drag(&mut sketch, east, DVec2::new(1.0, 2.0));

    assert_landed(&sketch, east, to);
    let square = direction(&sketch, first).dot(direction(&sketch, second));
    assert!(
        square.abs() < AFTER_THE_SOLVER,
        "no longer square: {square}"
    );
}

#[test]
fn a_trait_held_parallel_slides_along_its_direction_when_an_end_is_dragged() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (first, _, _) = a_trait(&mut sketch, DVec2::new(1.0, 1.0), DVec2::new(5.0, 1.0));
    let (second, _, end) = a_trait(&mut sketch, DVec2::new(1.0, 3.0), DVec2::new(5.0, 3.0));
    sketch.add_constraint(Constraint::Parallel { first, second });

    drag(&mut sketch, end, DVec2::new(1.0, 1.0));

    assert_along_its_own_line(&sketch, end, DVec2::new(6.0, 3.0));
    let across = direction(&sketch, first).perp_dot(direction(&sketch, second));
    assert!(
        across.abs() < AFTER_THE_SOLVER,
        "no longer parallel: {across}"
    );
}

#[test]
fn two_traits_held_equal_stay_equal_when_an_end_is_dragged() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (first, _, _) = a_trait(&mut sketch, DVec2::new(1.0, 1.0), DVec2::new(5.0, 1.0));
    let (second, _, end) = a_trait(&mut sketch, DVec2::new(1.0, 3.0), DVec2::new(5.0, 3.0));
    sketch.add_constraint(Constraint::Equal { first, second });

    drag(&mut sketch, end, DVec2::new(2.0, 0.0));

    let (a, b) = sketch.endpoints(first);
    let (c, d) = sketch.endpoints(second);
    assert!(
        d.distance(c) > 4.0 + 1.0,
        "the drag took nothing: the second trait is {} long",
        d.distance(c)
    );
    assert!(
        (a.distance(b) - c.distance(d)).abs() < AFTER_THE_SOLVER,
        "no longer equal: {} against {}",
        a.distance(b),
        c.distance(d)
    );
}

#[test]
fn a_trait_held_on_another_s_line_slides_along_it_when_an_end_is_dragged() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (first, _, _) = a_trait(&mut sketch, DVec2::new(1.0, 1.0), DVec2::new(3.0, 1.0));
    let (second, start, end) = a_trait(&mut sketch, DVec2::new(5.0, 1.0), DVec2::new(7.0, 1.0));
    sketch.add_constraint(Constraint::Collinear { first, second });

    drag(&mut sketch, end, DVec2::new(1.0, 1.0));

    assert_along_its_own_line(&sketch, end, DVec2::new(8.0, 1.0));
    for point in [start, end] {
        let off = distance_to_line(&sketch, sketch.point(point), first);
        assert!(off.abs() < AFTER_THE_SOLVER, "off the line by {off}");
    }
}

#[test]
fn a_point_held_on_a_trait_stays_on_its_line_when_the_trait_turns() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (segment, _, end) = a_trait(&mut sketch, DVec2::new(1.0, 1.0), DVec2::new(5.0, 1.0));
    let point = sketch.add_point(DVec2::new(3.0, 1.0));
    sketch.add_constraint(Constraint::OnSegment {
        point,
        segment,
        from: LaidFrom::Nowhere,
    });
    let to = sketch.point(end) + DVec2::new(0.0, 2.0);

    drag(&mut sketch, end, DVec2::new(0.0, 2.0));

    assert_landed(&sketch, end, to);
    let off = distance_to_line(&sketch, sketch.point(point), segment);
    assert!(off.abs() < AFTER_THE_SOLVER, "off the trait by {off}");
}

#[test]
fn a_point_held_on_an_axis_slides_along_it_and_stays_on_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let point = sketch.add_point(DVec2::new(0.0, 2.0));
    let other = sketch.add_point(DVec2::new(3.0, 1.0));
    sketch.add_segment(point, other);
    sketch.add_constraint(Constraint::OnAxis {
        point,
        axis: SketchAxis::V,
    });

    drag(&mut sketch, point, DVec2::new(1.0, 1.0));

    let at = sketch.point(point);
    assert!(at.x.abs() < AFTER_THE_SOLVER, "left the axis: {at}");
    assert!(
        (at.y - 3.0).abs() < 1e-3,
        "did not slide along the axis with the hand: {at}"
    );
}

#[test]
fn a_trait_held_tangent_to_a_circle_keeps_grazing_it_when_an_end_is_dragged() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(3.0, 3.0));
    let circle = sketch.add_circle(centre, 2.0);
    let (segment, _, end) = a_trait(&mut sketch, DVec2::new(0.0, 1.0), DVec2::new(6.0, 1.0));
    sketch.add_constraint(Constraint::Tangent {
        circle,
        segment,
        at: None,
        from: LaidFrom::Nowhere,
    });
    let before = sketch.point(end);

    drag(&mut sketch, end, DVec2::new(0.0, 1.0));

    assert!(
        sketch.point(end).distance(before) > 0.5,
        "the drag took nothing: the end is still at {before}"
    );
    let round = sketch.circles()[circle.0];
    let gap = distance_to_line(&sketch, sketch.point(round.center), segment).abs() - round.radius;
    assert!(gap.abs() < AFTER_THE_SOLVER, "no longer grazing: {gap}");
}

#[test]
fn two_circles_held_one_radius_share_a_radius_typed_on_either() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let first_centre = sketch.add_point(DVec2::new(2.0, 2.0));
    let second_centre = sketch.add_point(DVec2::new(6.0, 2.0));
    let first = sketch.add_circle(first_centre, 1.0);
    let second = sketch.add_circle(second_centre, 1.0);
    sketch.add_constraint(Constraint::EqualRadius { first, second });

    sketch.set_dimension(DimensionTarget::Radius(first), 1.5, false);
    sketch.resolve(1.0);

    let radius = sketch.circles()[second.0].radius;
    assert!(
        (radius - 1.5).abs() < AFTER_THE_SOLVER,
        "the second circle kept a radius of {radius}"
    );
}

#[test]
fn a_point_held_at_the_middle_stays_there_when_an_end_is_dragged() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (segment, _, end) = a_trait(&mut sketch, DVec2::new(1.0, 1.0), DVec2::new(5.0, 1.0));
    let middle = sketch.add_point(DVec2::new(3.0, 1.0));
    sketch.add_constraint(Constraint::Midpoint {
        point: middle,
        segment,
    });
    let to = sketch.point(end) + DVec2::new(2.0, 2.0);

    drag(&mut sketch, end, DVec2::new(2.0, 2.0));

    assert_landed(&sketch, end, to);
    let (a, b) = sketch.endpoints(segment);
    let off = sketch.point(middle).distance((a + b) / 2.0);
    assert!(off < AFTER_THE_SOLVER, "off the middle by {off}");
}

#[test]
fn a_trait_held_on_an_axis_stays_on_it_when_an_end_is_dragged() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (segment, _, end) = a_trait(&mut sketch, DVec2::new(1.0, 0.0), DVec2::new(4.0, 0.0));
    sketch.add_constraint(Constraint::AxisCollinear {
        segment,
        axis: SketchAxis::U,
    });

    drag(&mut sketch, end, DVec2::new(1.0, 1.0));

    let (a, b) = sketch.endpoints(segment);
    assert!(
        (b.x - 5.0).abs() < 1e-3,
        "the end did not follow the hand along the axis: {b}"
    );
    assert!(
        a.y.abs() < AFTER_THE_SOLVER && b.y.abs() < AFTER_THE_SOLVER,
        "left the axis: {a} {b}"
    );
}

#[test]
fn a_trait_held_along_an_axis_keeps_its_direction_when_an_end_is_dragged() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (segment, _, end) = a_trait(&mut sketch, DVec2::new(1.0, 1.0), DVec2::new(4.0, 1.0));
    sketch.add_constraint(Constraint::AxisParallel {
        segment,
        axis: SketchAxis::U,
    });
    let before = sketch.point(end);

    drag(&mut sketch, end, DVec2::new(1.0, 2.0));

    assert!(
        sketch.point(end).distance(before) > 0.5,
        "the drag took nothing: the end is still at {before}"
    );
    let (a, b) = sketch.endpoints(segment);
    assert!(
        (a.y - b.y).abs() < AFTER_THE_SOLVER,
        "no longer along the axis: {a} {b}"
    );
}

#[test]
fn a_fixed_point_stays_put_when_the_trait_it_ends_is_dragged() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (_, start, end) = a_trait(&mut sketch, DVec2::new(1.0, 1.0), DVec2::new(5.0, 1.0));
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(start),
    });
    let held = sketch.point(start);
    let to = sketch.point(end) + DVec2::new(1.0, 2.0);

    drag(&mut sketch, end, DVec2::new(1.0, 2.0));

    assert_landed(&sketch, end, to);
    assert!(
        sketch.point(start).distance(held) < AFTER_THE_SOLVER,
        "the fixed point moved to {}",
        sketch.point(start)
    );
}
