use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget, SketchAxis};
use crate::length::LengthOutcome;
use crate::plane::WorkPlane;
use crate::sketch::{PointId, Sketch};

#[test]
fn an_arc_end_made_one_with_its_centre_leaves_no_value_behind_whichever_end() {
    for start_goes in [true, false] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let centre = sketch.add_point(DVec2::new(20.0, 0.0));
        let start = sketch.add_point(DVec2::new(20.0, -5.0));
        let end = sketch.add_point(DVec2::new(25.0, 0.0));
        let arc = sketch.add_arc(centre, start, end);
        sketch.set_dimension(DimensionTarget::ArcRadius(arc), 5.0, true);

        sketch.merge_points(centre, if start_goes { start } else { end });

        assert!(
            sketch.is_erased_arc(arc),
            "an arc of no radius is still drawn"
        );
        assert!(
            sketch.dimensions().is_empty(),
            "the start going: {start_goes}, and a value was left behind: {:?}",
            sketch.dimensions()
        );
    }
}

#[test]
fn an_arc_of_ellipse_whose_two_ends_become_one_is_taken_away() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(0.0, 50.0));
    let [west, east] = [-30.0, 30.0].map(|x| sketch.add_point(DVec2::new(x, 50.0)));
    let [south, north] = [30.0, 70.0].map(|y| sketch.add_point(DVec2::new(0.0, y)));
    let oval = sketch.add_ellipse(centre, [west, east], [south, north]);
    let from = sketch.add_point(DVec2::new(21.213, 64.142));
    let to = sketch.add_point(DVec2::new(-21.213, 64.142));
    for point in [from, to] {
        sketch.add_constraint(Constraint::OnEllipse {
            point,
            ellipse: oval,
            from: crate::LaidFrom::Nowhere,
        });
    }
    sketch.draw_the_stretch(oval, to, from);

    sketch.merge_points(from, to);

    assert!(
        sketch.is_erased_ellipse(oval),
        "a stretch of no length is still drawn"
    );
}

/// A rectangle from (2, 0) to (0.5, 1), and its corner at (0.5, 0).
fn a_rectangle_half_a_unit_from_the_origin() -> (Sketch, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [[2.0, 0.0], [0.5, 0.0], [0.5, 1.0], [2.0, 1.0]]
        .map(|[x, y]| sketch.add_point(DVec2::new(x, y)));
    for index in 0..4 {
        sketch.add_segment(corners[index], corners[(index + 1) % 4]);
    }
    (sketch, corners[1])
}

fn typed(sketch: &mut Sketch, target: DimensionTarget, value: f64) {
    sketch.set_dimension(target, value, false);
    assert_eq!(
        sketch.land_value(target, 1.0),
        LengthOutcome::Exact,
        "{target:?} = {value} does not land"
    );
}

fn joined_refused_with_the_drawing_unchanged(mut sketch: Sketch, kept: PointId, dropped: PointId) {
    let before = sketch.clone();

    let joined = sketch.join_points(kept, dropped, 1.0);

    assert!(
        !joined,
        "the two points were made one, and the values read {:?}",
        sketch.dimensions()
    );
    assert_eq!(
        sketch.constraints(),
        before.constraints(),
        "the rules are as they were"
    );
    assert!(
        sketch
            .dimensions()
            .iter()
            .map(|value| (value.target, value.value, value.driven))
            .eq(before
                .dimensions()
                .iter()
                .map(|value| (value.target, value.value, value.driven))),
        "the values are as they were: {:?}",
        sketch.dimensions()
    );
    assert_eq!(sketch.points().len(), before.points().len());
    for (index, (now, was)) in sketch.points().iter().zip(before.points()).enumerate() {
        assert!(
            now.distance(*was) < 1e-9,
            "point {index} moved from {was} to {now}"
        );
        assert_eq!(
            sketch.is_erased_point(PointId(index)),
            before.is_erased_point(PointId(index)),
            "point {index} was erased or brought back"
        );
    }
}

#[test]
fn a_corner_a_distance_holds_apart_from_the_origin_is_not_made_one_with_it() {
    let (mut sketch, corner) = a_rectangle_half_a_unit_from_the_origin();
    typed(
        &mut sketch,
        DimensionTarget::Distance {
            from: corner,
            to: Sketch::ORIGIN,
        },
        0.5,
    );

    joined_refused_with_the_drawing_unchanged(sketch, Sketch::ORIGIN, corner);
}

#[test]
fn a_corner_a_horizontal_value_holds_apart_from_the_origin_is_not_made_one_with_it() {
    let (mut sketch, corner) = a_rectangle_half_a_unit_from_the_origin();
    typed(
        &mut sketch,
        DimensionTarget::Projected {
            from: corner,
            to: Sketch::ORIGIN,
            axis: SketchAxis::U,
        },
        0.5,
    );

    joined_refused_with_the_drawing_unchanged(sketch, Sketch::ORIGIN, corner);
}

#[test]
fn two_free_points_a_distance_holds_apart_are_not_made_one() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let first = sketch.add_point(DVec2::new(10.0, 10.0));
    let second = sketch.add_point(DVec2::new(30.0, 25.0));
    typed(
        &mut sketch,
        DimensionTarget::Distance {
            from: first,
            to: second,
        },
        20.0,
    );

    joined_refused_with_the_drawing_unchanged(sketch, first, second);
}

fn joined(mut sketch: Sketch, kept: PointId, dropped: PointId) {
    let place = sketch.point(kept);

    let joined = sketch.join_points(kept, dropped, 1.0);

    assert!(joined, "the merge was refused: {:?}", sketch.dimensions());
    assert!(
        sketch.is_erased_point(dropped),
        "the point dropped still stands"
    );
    assert!(
        sketch.point(kept).distance(place) < 1e-9,
        "the point kept moved from {place} to {}",
        sketch.point(kept)
    );
}

#[test]
fn two_free_points_with_nothing_between_them_are_made_one() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let first = sketch.add_point(DVec2::new(10.0, 10.0));
    let second = sketch.add_point(DVec2::new(30.0, 25.0));

    joined(sketch, first, second);
}

#[test]
fn a_read_out_distance_between_two_points_does_not_keep_them_apart() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let first = sketch.add_point(DVec2::new(10.0, 10.0));
    let second = sketch.add_point(DVec2::new(30.0, 25.0));
    sketch.set_dimension(
        DimensionTarget::Distance {
            from: first,
            to: second,
        },
        25.0,
        true,
    );

    joined(sketch, first, second);
}

#[test]
fn a_distance_elsewhere_in_the_drawing_does_not_keep_two_points_apart() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let first = sketch.add_point(DVec2::new(10.0, 10.0));
    let second = sketch.add_point(DVec2::new(30.0, 25.0));
    let [held, apart] =
        [[-20.0, 0.0], [-20.0, 20.0]].map(|[x, y]| sketch.add_point(DVec2::new(x, y)));
    typed(
        &mut sketch,
        DimensionTarget::Distance {
            from: held,
            to: apart,
        },
        20.0,
    );

    joined(sketch, first, second);
}
