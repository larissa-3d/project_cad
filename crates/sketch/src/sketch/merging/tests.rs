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

/// Half an ellipse from (0, 0) round to (40, 0) about (20, 0), rising to
/// (20, 15): the curve, its centre and its two ends.
fn half_an_ellipse() -> (Sketch, crate::EllipseId, PointId, [PointId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(20.0, 0.0));
    let ends = [0.0, 40.0].map(|x| sketch.add_point(DVec2::new(x, 0.0)));
    let top = sketch.add_point(DVec2::new(20.0, 15.0));
    let curve = sketch.add_ellipse(centre, ends, [centre, top]);
    sketch.draw_the_stretch(curve, ends[1], ends[0]);
    (sketch, curve, centre, ends)
}

/// Each end of the half-ellipse with its centre, the end clicked first and then
/// the centre: the point kept and the point dropped.
fn each_end_and_its_centre_both_ways(
    centre: PointId,
    ends: [PointId; 2],
) -> Vec<(PointId, PointId)> {
    ends.into_iter()
        .flat_map(|end| [(end, centre), (centre, end)])
        .collect()
}

#[test]
fn half_an_ellipse_whose_end_is_made_one_with_its_centre_goes_while_no_value_holds_its_size() {
    let (_, _, centre, ends) = half_an_ellipse();
    for (kept, dropped) in each_end_and_its_centre_both_ways(centre, ends) {
        let (mut sketch, curve, _, _) = half_an_ellipse();

        let joined = sketch.join_points(kept, dropped, 1.0);

        assert!(joined, "{dropped:?} onto {kept:?}: the merge was refused");
        assert!(
            sketch.is_erased_ellipse(curve),
            "{dropped:?} onto {kept:?}: a half-ellipse of no width is still drawn"
        );
        assert!(
            sketch.regions().is_empty(),
            "{dropped:?} onto {kept:?}: an area appeared"
        );
        assert!(
            !sketch.is_erased_point(kept) && sketch.is_erased_point(dropped),
            "{dropped:?} onto {kept:?}: the merged point went with the curve, or both still stand"
        );
    }
}

#[test]
fn half_an_ellipse_s_end_is_still_made_one_with_a_point_apart() {
    let (mut sketch, _, _, ends) = half_an_ellipse();
    let apart = sketch.add_point(DVec2::new(60.0, 30.0));

    joined(sketch, ends[0], apart);
}

#[test]
fn an_arc_end_is_not_made_one_with_its_centre_while_its_radius_is_typed() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(20.0, 0.0));
    let start = sketch.add_point(DVec2::new(20.0, -5.0));
    let end = sketch.add_point(DVec2::new(25.0, 0.0));
    let arc = sketch.add_arc(centre, start, end);
    typed(&mut sketch, DimensionTarget::ArcRadius(arc), 5.0);

    joined_refused_with_the_drawing_unchanged(sketch, end, centre);
}

#[test]
fn an_arc_end_made_one_with_its_centre_takes_the_arc_while_no_value_holds_its_radius() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(20.0, 0.0));
    let start = sketch.add_point(DVec2::new(20.0, -5.0));
    let end = sketch.add_point(DVec2::new(25.0, 0.0));
    let arc = sketch.add_arc(centre, start, end);

    let joined = sketch.join_points(centre, end, 1.0);

    assert!(joined, "the merge was refused");
    assert!(
        sketch.is_erased_arc(arc),
        "an arc of no radius is still drawn"
    );
}

/// A whole ellipse about (0, 50), 60 wide from west to east and 40 high from
/// south to north: the curve, its centre, and the ends of its two axes.
fn an_ellipse_about_its_centre() -> (
    Sketch,
    crate::EllipseId,
    PointId,
    [PointId; 2],
    [PointId; 2],
) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(0.0, 50.0));
    let [west, east] = [-30.0, 30.0].map(|x| sketch.add_point(DVec2::new(x, 50.0)));
    let [south, north] = [30.0, 70.0].map(|y| sketch.add_point(DVec2::new(0.0, y)));
    let oval = sketch.add_ellipse(centre, [west, east], [south, north]);
    (sketch, oval, centre, [west, east], [south, north])
}

/// Half an ellipse running between the two ends of its second axis, as a trim
/// can leave one: the curve, its centre and its two ends.
fn half_an_ellipse_ended_on_its_second_axis() -> (Sketch, crate::EllipseId, PointId, [PointId; 2]) {
    let (mut sketch, oval, centre, _, [south, north]) = an_ellipse_about_its_centre();
    sketch.draw_the_stretch(oval, south, north);
    (sketch, oval, centre, [south, north])
}

/// An arc of ellipse a trim left: its two ends held on the curve by rules
/// rather than being ends of an axis.
fn an_arc_of_ellipse_ended_by_rules() -> (Sketch, crate::EllipseId, PointId, [PointId; 2]) {
    let (mut sketch, oval, centre, _, _) = an_ellipse_about_its_centre();
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
    (sketch, oval, centre, [from, to])
}

fn refused_with_the_curve_kept(
    sketch: &Sketch,
    curve: crate::EllipseId,
    kept: PointId,
    dropped: PointId,
) {
    let mut tried = sketch.clone();

    joined_refused_with_the_drawing_unchanged(sketch.clone(), kept, dropped);
    tried.join_points(kept, dropped, 1.0);

    assert!(
        !tried.is_erased_ellipse(curve) && tried.ellipse_ends(curve) == sketch.ellipse_ends(curve),
        "{dropped:?} onto {kept:?}: the curve went or its stretch changed"
    );
}

fn the_curve_goes(mut sketch: Sketch, curve: crate::EllipseId, kept: PointId, dropped: PointId) {
    let joined = sketch.join_points(kept, dropped, 1.0);

    assert!(joined, "{dropped:?} onto {kept:?}: the merge was refused");
    assert!(
        sketch.is_erased_ellipse(curve),
        "{dropped:?} onto {kept:?}: a curve of no width is still drawn"
    );
    assert!(
        sketch.regions().is_empty(),
        "{dropped:?} onto {kept:?}: an area appeared"
    );
}

#[test]
fn half_an_ellipse_s_end_is_not_made_one_with_its_centre_while_a_length_is_typed_on_either_axis() {
    let (_, _, centre, ends) = half_an_ellipse();
    for (kept, dropped) in each_end_and_its_centre_both_ways(centre, ends) {
        for second_typed in [false, true] {
            let (mut sketch, curve, _, _) = half_an_ellipse();
            let ellipse = sketch.ellipses()[curve.0];
            let (axis, length) = match second_typed {
                true => (ellipse.second, 15.0),
                false => (ellipse.first, 40.0),
            };
            typed(&mut sketch, DimensionTarget::Length(axis), length);

            refused_with_the_curve_kept(&sketch, curve, kept, dropped);
        }
    }
}

#[test]
fn half_an_ellipse_on_its_second_axis_is_not_made_one_with_its_centre_while_a_length_is_typed() {
    let (_, _, centre, ends) = half_an_ellipse_ended_on_its_second_axis();
    for (kept, dropped) in each_end_and_its_centre_both_ways(centre, ends) {
        for second_typed in [true, false] {
            let (mut sketch, curve, _, _) = half_an_ellipse_ended_on_its_second_axis();
            let ellipse = sketch.ellipses()[curve.0];
            let (axis, length) = match second_typed {
                true => (ellipse.second, 40.0),
                false => (ellipse.first, 60.0),
            };
            typed(&mut sketch, DimensionTarget::Length(axis), length);

            refused_with_the_curve_kept(&sketch, curve, kept, dropped);
        }
    }
}

#[test]
fn half_an_ellipse_on_its_second_axis_goes_when_an_end_is_made_one_with_its_centre() {
    let (_, _, centre, ends) = half_an_ellipse_ended_on_its_second_axis();
    for (kept, dropped) in each_end_and_its_centre_both_ways(centre, ends) {
        let (sketch, curve, _, _) = half_an_ellipse_ended_on_its_second_axis();

        the_curve_goes(sketch, curve, kept, dropped);
    }
}

#[test]
fn an_arc_of_ellipse_ended_by_rules_goes_when_an_end_is_made_one_with_its_centre() {
    let (_, _, centre, ends) = an_arc_of_ellipse_ended_by_rules();
    for (kept, dropped) in each_end_and_its_centre_both_ways(centre, ends) {
        let (sketch, curve, _, _) = an_arc_of_ellipse_ended_by_rules();

        the_curve_goes(sketch, curve, kept, dropped);
    }
}

#[test]
fn half_an_ellipse_that_goes_leaves_its_other_end_and_the_value_measuring_it() {
    let (_, _, centre, [other, end]) = half_an_ellipse();
    for (kept, dropped) in [(end, centre), (centre, end)] {
        let (mut sketch, curve, _, _) = half_an_ellipse();
        let apart = sketch.add_point(DVec2::new(0.0, -30.0));
        let target = DimensionTarget::Distance {
            from: other,
            to: apart,
        };
        typed(&mut sketch, target, 30.0);

        sketch.join_points(kept, dropped, 1.0);

        assert!(
            sketch.is_erased_ellipse(curve),
            "{dropped:?} onto {kept:?}: the curve is still drawn"
        );
        assert!(
            !sketch.is_erased_point(other),
            "{dropped:?} onto {kept:?}: the other end went with the curve"
        );
        assert!(
            sketch
                .dimensions()
                .iter()
                .any(|value| value.target == target && (value.value - 30.0).abs() < 1e-9),
            "{dropped:?} onto {kept:?}: the value on the other end went, the values read {:?}",
            sketch.dimensions()
        );
    }
}

#[test]
fn half_an_ellipse_that_goes_leaves_its_top_handle_and_the_value_measuring_it() {
    let (sketch, curve, centre, [_, end]) = half_an_ellipse();
    let top = sketch.segments()[sketch.ellipses()[curve.0].second.0].end;
    for (kept, dropped) in [(end, centre), (centre, end)] {
        let (mut sketch, curve, _, _) = half_an_ellipse();
        let apart = sketch.add_point(DVec2::new(20.0, 45.0));
        let target = DimensionTarget::Distance {
            from: top,
            to: apart,
        };
        typed(&mut sketch, target, 30.0);

        sketch.join_points(kept, dropped, 1.0);

        assert!(
            sketch.is_erased_ellipse(curve),
            "{dropped:?} onto {kept:?}: the curve is still drawn"
        );
        assert!(
            !sketch.is_erased_point(top),
            "{dropped:?} onto {kept:?}: the top handle a value measures went with the curve"
        );
        assert!(
            sketch
                .dimensions()
                .iter()
                .any(|value| value.target == target),
            "{dropped:?} onto {kept:?}: the value on the top handle went, the values read {:?}",
            sketch.dimensions()
        );
    }
}

#[test]
fn half_an_ellipse_that_goes_leaves_a_trait_on_its_other_end_where_it_was() {
    let (_, _, centre, [other, end]) = half_an_ellipse();
    for (kept, dropped) in [(end, centre), (centre, end)] {
        let (mut sketch, curve, _, _) = half_an_ellipse();
        let tail = sketch.add_point(DVec2::new(-10.0, -10.0));
        sketch.add_segment(other, tail);

        sketch.join_points(kept, dropped, 1.0);

        assert!(
            sketch.is_erased_ellipse(curve),
            "{dropped:?} onto {kept:?}: the curve is still drawn"
        );
        for (point, was) in [(other, DVec2::ZERO), (tail, DVec2::new(-10.0, -10.0))] {
            assert!(
                sketch.point(point).distance(was) < 1e-9,
                "{dropped:?} onto {kept:?}: {point:?} moved from {was} to {}",
                sketch.point(point)
            );
        }
    }
}

/// The other end and the top handle of [`half_an_ellipse`], when its right end
/// is the one made one with its centre.
fn the_other_end_and_the_top(sketch: &Sketch, curve: crate::EllipseId) -> [PointId; 2] {
    let (_, _, _, [other, _]) = half_an_ellipse();
    [
        other,
        sketch.segments()[sketch.ellipses()[curve.0].second.0].end,
    ]
}

#[test]
fn half_an_ellipse_that_goes_takes_with_it_the_points_nothing_names() {
    let (_, _, centre, [_, end]) = half_an_ellipse();
    for (kept, dropped) in [(end, centre), (centre, end)] {
        let (mut sketch, curve, _, _) = half_an_ellipse();
        let points = the_other_end_and_the_top(&sketch, curve);

        sketch.join_points(kept, dropped, 1.0);

        assert!(
            sketch.is_erased_ellipse(curve),
            "{dropped:?} onto {kept:?}: the curve is still drawn"
        );
        for point in points {
            assert!(
                sketch.is_erased_point(point),
                "{dropped:?} onto {kept:?}: {point:?}, named by nothing, outlived the curve"
            );
        }
    }
}

#[test]
fn a_rule_elsewhere_in_the_drawing_keeps_none_of_the_points_of_half_an_ellipse_that_goes() {
    let (_, _, centre, [_, end]) = half_an_ellipse();
    for (kept, dropped) in [(end, centre), (centre, end)] {
        let (mut sketch, curve, _, _) = half_an_ellipse();
        let points = the_other_end_and_the_top(&sketch, curve);
        let elsewhere = sketch.add_point(DVec2::new(-40.0, -40.0));
        sketch.add_constraint(Constraint::Fixed {
            element: crate::Element::Point(elsewhere),
        });

        sketch.join_points(kept, dropped, 1.0);

        assert!(
            sketch.is_erased_ellipse(curve),
            "{dropped:?} onto {kept:?}: the curve is still drawn"
        );
        for point in points {
            assert!(
                sketch.is_erased_point(point),
                "{dropped:?} onto {kept:?}: {point:?}, named by nothing, outlived the curve"
            );
        }
    }
}

#[test]
fn half_an_ellipse_that_goes_leaves_its_other_end_a_rule_names_and_the_rule() {
    let (_, _, centre, [other, end]) = half_an_ellipse();
    for (kept, dropped) in [(end, centre), (centre, end)] {
        let (mut sketch, curve, _, _) = half_an_ellipse();
        let rule = Constraint::Fixed {
            element: crate::Element::Point(other),
        };
        sketch.add_constraint(rule);

        sketch.join_points(kept, dropped, 1.0);

        assert!(
            sketch.is_erased_ellipse(curve),
            "{dropped:?} onto {kept:?}: the curve is still drawn"
        );
        assert!(
            !sketch.is_erased_point(other) && sketch.point(other).distance(DVec2::ZERO) < 1e-9,
            "{dropped:?} onto {kept:?}: the other end a rule fixes went with the curve, or moved"
        );
        assert!(
            sketch.carries(rule),
            "{dropped:?} onto {kept:?}: the rule on the other end went, the rules read {:?}",
            sketch.constraints()
        );
    }
}

#[test]
fn half_an_ellipse_whose_end_is_made_one_with_a_point_apart_is_still_drawn() {
    let (mut sketch, curve, _, [other, end]) = half_an_ellipse();
    let apart = sketch.add_point(DVec2::new(60.0, 30.0));

    let joined = sketch.join_points(end, apart, 1.0);

    assert!(joined, "the merge was refused");
    assert!(
        !sketch.is_erased_ellipse(curve),
        "a half-ellipse whose end did not land on its centre was taken away"
    );
    assert_eq!(
        sketch.ellipse_ends(curve),
        Some((end, other)),
        "the stretch drawn changed"
    );
}
