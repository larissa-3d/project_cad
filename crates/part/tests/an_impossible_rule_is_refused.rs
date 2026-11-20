//! A rule the drawing cannot hold is refused, as a value is, rather than kept
//! untrue.
//!
//! Closes #467.
//! - a rule contradicting a value is refused: the drawing is as it was, and the
//!   history has no new step —
//!   `a_rule_contradicting_a_value_is_refused_and_leaves_no_step`
//! - the message reads « Règle impossible : elle contredit le dessin » — no
//!   test: the sentence is the interface's, held beside it in
//!   `crates/app/src/wording/outcome/tests.rs`
//! - a rule only fixed points stand in the way of still lands —
//!   `a_rule_only_fixed_points_stand_in_the_way_of_still_lands`
//! - a rule the drawing can take still lands, and is recorded —
//!   `a_rule_the_drawing_can_take_lands_and_is_recorded`
//!
//! Closes #1038.
//! - The circle, arc and half-ellipse cases are refused with the drawing
//!   unchanged — `a_circle_s_centre_put_on_its_own_circle_is_refused`,
//!   `an_arc_s_centre_put_on_its_own_arc_is_refused`,
//!   `half_an_ellipse_s_centre_put_on_its_own_curve_is_refused`
//! - The cases above hold in #501's campaign — no test: the campaign lives in
//!   `cao_sketch`, and the three drawings are held to its rules there, under
//!   the same names, in `crates/sketch/tests/what_random_sketches_found.rs`
//!
//! Closes #1037.
//! - Both kinds of curve refuse the tangency with the drawing unchanged,
//!   whether the radius is typed or not —
//!   `a_tangency_between_an_arc_and_a_trait_through_its_centre_is_refused`,
//!   `a_tangency_between_an_arc_of_free_radius_and_a_trait_through_its_centre_is_refused`,
//!   `a_tangency_between_a_circle_and_a_trait_through_its_centre_is_refused`,
//!   `a_tangency_between_a_circle_of_free_radius_and_a_trait_through_its_centre_is_refused`;
//!   so are the traits the centre is held on or is the middle of —
//!   `a_tangency_between_a_circle_of_free_radius_and_a_trait_its_centre_is_held_on_is_refused`,
//!   `a_tangency_between_a_circle_of_free_radius_and_a_trait_its_centre_is_the_middle_of_is_refused`,
//!   `a_tangency_between_an_arc_of_free_radius_and_a_trait_its_centre_is_held_on_is_refused`,
//!   `a_tangency_between_an_arc_of_free_radius_and_a_trait_its_centre_is_the_middle_of_is_refused`;
//!   and a centre held on a trait its curve was made tangent to first is
//!   refused alike —
//!   `a_circle_s_centre_held_on_a_trait_it_is_tangent_to_is_refused`,
//!   `a_circle_s_centre_made_the_middle_of_a_trait_it_is_tangent_to_is_refused`,
//!   `an_arc_s_centre_held_on_a_trait_it_is_tangent_to_is_refused`,
//!   `an_arc_s_centre_made_the_middle_of_a_trait_it_is_tangent_to_is_refused`;
//!   half an ellipse is refused alike —
//!   `a_tangency_between_half_an_ellipse_and_a_trait_through_its_centre_is_refused`;
//!   and a trait merely crossing the centre, nothing holding the centre on it,
//!   still takes the tangency —
//!   `a_tangency_between_a_circle_and_a_trait_merely_crossing_its_centre_lands`,
//!   `a_tangency_between_an_arc_and_a_trait_merely_crossing_its_centre_lands`
//! - The cases above hold in #501's campaign — no test: the campaign lives in
//!   `cao_sketch`, and the drawings are held to its rules there, under the
//!   same names, in `crates/sketch/tests/what_random_sketches_found.rs`
//!
//! Closes #1042.
//! - The three cases are refused with the drawing unchanged —
//!   `a_trait_given_a_horizontal_value_is_not_turned_vertical`,
//!   `an_ellipse_given_a_horizontal_value_across_its_axis_is_not_turned`,
//!   `a_trait_given_a_vertical_value_is_not_turned_square_to_a_vertical_one`;
//!   the same rules with no value against them still land —
//!   `a_trait_with_no_value_is_turned_vertical`,
//!   `a_trait_given_a_vertical_value_is_still_made_vertical`,
//!   `an_ellipse_with_no_value_is_turned`,
//!   `a_trait_with_no_value_is_turned_square_to_a_vertical_one`;
//!   and two points a horizontal value holds apart are not made one —
//!   `two_points_a_horizontal_value_holds_apart_are_not_made_one`
//! - The cases above hold in #501's campaign — no test: the campaign lives in
//!   `cao_sketch`, and the trait, the half-ellipse and the perpendicular
//!   drawings it found are held to its rules there, in
//!   `crates/sketch/tests/what_random_sketches_found.rs`
//!
//! Closes #1036.
//! - That join returns false and the drawing is as it was —
//!   `a_corner_a_distance_holds_apart_from_the_origin_is_not_made_one_with_it`;
//!   so it does for any two points a distance holds apart —
//!   `two_free_points_a_distance_holds_apart_are_not_made_one`; the join
//!   itself is held at the sketch's level under the same names, with the
//!   horizontal value beside them, in
//!   `crates/sketch/src/sketch/merging/tests.rs`; so is what the refusal
//!   leaves alone — two points with nothing between them, a read-out distance
//!   between them, and a distance elsewhere in the drawing do not keep them
//!   apart
//! - The case above holds in #501's campaign — no test: the campaign lives in
//!   `cao_sketch`, and the drawing is held to its rules there, in
//!   `crates/sketch/tests/what_random_sketches_found.rs`
//!
//! Closes #1044.
//! - The rule is refused with the drawing unchanged —
//!   `collinear_between_two_equal_sides_an_angle_holds_apart_is_refused`;
//!   so is the same rule with no « Égalité » between the two sides —
//!   `collinear_between_two_sides_an_angle_holds_apart_is_refused`; and two
//!   equal sides no angle holds apart still become collinear, both keeping
//!   their length — `collinear_between_two_equal_sides_no_angle_holds_apart_lands`
//! - The case above holds in #501's campaign — no test: the campaign lives in
//!   `cao_sketch`, and the drawing is held to its rules there, under the same
//!   name, in `crates/sketch/tests/what_random_sketches_found.rs`
//!
//! Closes #1054.
//! - The three cases are refused with the drawing unchanged —
//!   `a_point_a_value_holds_off_a_trait_is_not_put_on_it`,
//!   `a_point_a_value_holds_off_a_trait_is_not_made_its_middle`,
//!   `a_trait_a_value_holds_off_a_point_is_not_made_collinear_through_it`
//! - The same rules without a value standing against them still land —
//!   `a_free_point_is_put_on_a_trait`,
//!   `a_free_point_is_made_a_trait_s_middle`,
//!   `a_trait_is_made_collinear_through_a_free_point`
//! - Also: the merge above is refused with the drawing unchanged —
//!   `a_point_a_value_holds_off_a_trait_is_not_made_one_with_its_end`
//!
//! Closes #1043.
//! - The merge is refused or the half-ellipse goes; no area appears — as for an
//!   arc: refused while a length typed on its first axis holds its size, for
//!   either end and either click order —
//!   `half_an_ellipse_s_end_made_one_with_its_centre_is_refused_while_its_axis_length_is_typed`;
//!   with no value, the issue's case, it goes, held in `two_points_made_one.rs`
//! - The cases above hold in #501's campaign — no test: the campaign lives in
//!   `cao_sketch`, and the drawing is held to its rules there, in
//!   `crates/sketch/tests/what_random_sketches_found.rs`

use cao_part::{DimensionOutcome, Operation, Outcome, PartDocument, PointRef};
use cao_sketch::{
    Along, AxisToward, Constraint, DimensionTarget, Element, EllipseId, PointId, Rule, RuleIntent,
    RulePick, SegmentId, Sketch, SketchAxis, Support, Toward, WorkPlane, rule_intent,
};
use glam::DVec2;

/// Two traits apart, 50 and 80 long, both typed.
fn two_typed_traits() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (start, end) in [
        (DVec2::new(10.0, 10.0), DVec2::new(60.0, 10.0)),
        (DVec2::new(10.0, 40.0), DVec2::new(90.0, 40.0)),
    ] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(start),
            end: PointRef::New(end),
            construction: false,
        });
    }
    for (side, value) in [(SegmentId(0), 50.0), (SegmentId(1), 80.0)] {
        document.apply(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(side),
            value: value.into(),
            placement: None,
        });
    }
    document
}

fn steps(document: &PartDocument) -> usize {
    document.history.applied_operations().len()
}

#[test]
fn a_rule_contradicting_a_value_is_refused_and_leaves_no_step() {
    let mut document = two_typed_traits();
    let before = document.sketches()[0].clone();
    let recorded = steps(&document);
    let equal = Constraint::Equal {
        first: SegmentId(0),
        second: SegmentId(1),
    };

    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: equal,
    });

    assert_eq!(outcome, Some(Outcome::RuleRefused));
    let sketch = &document.sketches()[0];
    assert!(!sketch.carries(equal), "the rule is not kept");
    assert!(
        sketch
            .points()
            .iter()
            .zip(before.points())
            .all(|(now, was)| now.distance(*was) < 1e-9),
        "nothing about the drawing moved"
    );
    assert_eq!(steps(&document), recorded, "the history has no new step");
}

#[test]
fn a_rule_only_fixed_points_stand_in_the_way_of_still_lands() {
    let mut document = two_typed_traits();
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Fixed {
            element: Element::Segment(SegmentId(0)),
        },
    });
    let along = Constraint::AxisCollinear {
        segment: SegmentId(0),
        axis: SketchAxis::U,
    };

    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: along,
    });

    assert_eq!(outcome, None, "nothing to say: the rule holds");
    let sketch = &document.sketches()[0];
    assert!(sketch.carries(along));
    let (start, end) = sketch.endpoints(SegmentId(0));
    assert!(
        start.y.abs() < 1e-3 && end.y.abs() < 1e-3,
        "the fixed trait came onto the axis: {start} to {end}"
    );
}

#[test]
fn a_rule_the_drawing_can_take_lands_and_is_recorded() {
    let mut document = two_typed_traits();
    let recorded = steps(&document);
    let parallel = Constraint::Parallel {
        first: SegmentId(0),
        second: SegmentId(1),
    };

    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: parallel,
    });

    assert_eq!(outcome, None);
    assert!(document.sketches()[0].carries(parallel));
    assert_eq!(steps(&document), recorded + 1, "the rule is a step");
}

fn a_blank_sketch() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document
}

fn centre_put_on_its_own_curve_is_refused(document: PartDocument, curve: Element) {
    let sketch = &document.sketches()[0];
    let centre: PointId = match curve {
        Element::Circle(circle) => sketch.circles()[circle.0].center,
        Element::Arc(arc) => sketch.arcs()[arc.0].center,
        Element::Ellipse(ellipse) => sketch.ellipses()[ellipse.0].center,
        other => panic!("not a curve: {other:?}"),
    };
    let picks = [
        RulePick::Element(Element::Point(centre)),
        RulePick::Element(curve),
    ];
    let rule = the_rule(&document, Rule::Coincident, &picks);
    refused_with_the_drawing_unchanged(document, rule, Some(curve));
}

fn the_rule(document: &PartDocument, rule: Rule, picks: &[RulePick]) -> Constraint {
    match rule_intent(rule, picks, &document.sketches()[0]) {
        Some(RuleIntent::Constrain(rule)) => rule,
        other => panic!("the tool offers no rule for {picks:?}: {other:?}"),
    }
}

fn landed(document: &mut PartDocument, rule: Rule, picks: &[RulePick]) {
    let rule = the_rule(document, rule, picks);
    let recorded = steps(document);

    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });

    assert_eq!(outcome, None, "{rule:?} was refused");
    assert!(document.sketches()[0].carries(rule), "the rule is kept");
    assert_eq!(steps(document), recorded + 1, "the rule is a step");
}

#[test]
fn a_circle_s_centre_put_on_its_own_circle_is_refused() {
    let mut document = a_blank_sketch();
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(20.0, 15.0)),
        radius: 25.0,
        rim: Vec::new(),
        construction: false,
    });

    centre_put_on_its_own_curve_is_refused(document, Element::Circle(cao_sketch::CircleId(0)));
}

#[test]
fn an_arc_s_centre_put_on_its_own_arc_is_refused() {
    let mut document = a_blank_sketch();
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(20.0, 15.0)),
        start: PointRef::New(DVec2::new(45.0, 15.0)),
        end: PointRef::New(DVec2::new(20.0, 40.0)),
        construction: false,
    });

    centre_put_on_its_own_curve_is_refused(document, Element::Arc(cao_sketch::ArcId(0)));
}

#[test]
fn half_an_ellipse_s_centre_put_on_its_own_curve_is_refused() {
    let mut document = a_blank_sketch();
    let centre = PointRef::New(DVec2::new(20.0, 0.0));
    let (left, right) = (
        PointRef::New(DVec2::new(0.0, 0.0)),
        PointRef::New(DVec2::new(40.0, 0.0)),
    );
    document.apply(Operation::AddEllipse {
        sketch: 0,
        center: centre.clone(),
        first: [left.clone(), right.clone()],
        second: [centre, PointRef::New(DVec2::new(20.0, 15.0))],
        construction: false,
        drawn: Some([right, left]),
    });

    centre_put_on_its_own_curve_is_refused(document, Element::Ellipse(cao_sketch::EllipseId(0)));
}

/// A circle of radius 25 about (20, 15), and a trait starting on its centre,
/// the radius typed when `typed`.
fn a_circle_and_a_trait_from_its_centre(typed: bool) -> PartDocument {
    let mut document = a_blank_sketch();
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(20.0, 15.0)),
        radius: 25.0,
        rim: Vec::new(),
        construction: false,
    });
    let centre = document.sketches()[0].circles()[0].center;
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(centre),
        end: PointRef::New(DVec2::new(60.0, 45.0)),
        construction: false,
    });
    if typed {
        document.apply(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Radius(cao_sketch::CircleId(0)),
            value: 25.0.into(),
            placement: None,
        });
    }
    document
}

/// An arc of radius 25 about (20, 15), a quarter turn from (45, 15), and a
/// trait starting on its centre, the radius typed when `typed`.
fn an_arc_and_a_trait_from_its_centre(typed: bool) -> PartDocument {
    let mut document = a_blank_sketch();
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(20.0, 15.0)),
        start: PointRef::New(DVec2::new(45.0, 15.0)),
        end: PointRef::New(DVec2::new(20.0, 40.0)),
        construction: false,
    });
    let centre = document.sketches()[0].arcs()[0].center;
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(centre),
        end: PointRef::New(DVec2::new(60.0, 45.0)),
        construction: false,
    });
    if typed {
        document.apply(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::ArcRadius(cao_sketch::ArcId(0)),
            value: 25.0.into(),
            placement: None,
        });
    }
    document
}

fn size_of(document: &PartDocument, curve: Element) -> f64 {
    let sketch = &document.sketches()[0];
    match curve {
        Element::Circle(circle) => sketch.circle(circle).radius,
        Element::Arc(arc) => sketch.arc_radius(arc),
        Element::Ellipse(ellipse) => {
            let (from, to) = sketch.endpoints(sketch.ellipses()[ellipse.0].first);
            from.distance(to)
        }
        other => panic!("not a curve: {other:?}"),
    }
}

fn tangency_through_the_centre_is_refused(document: PartDocument, curve: Element) {
    let sketch = &document.sketches()[0];
    let picks = [
        RulePick::Element(Element::Segment(last_trait(sketch))),
        RulePick::Element(curve),
    ];
    let Some(RuleIntent::Constrain(rule)) = rule_intent(Rule::Tangent, &picks, sketch) else {
        panic!("the tool offers no tangency for a trait and a curve");
    };
    refused_with_the_drawing_unchanged(document, rule, Some(curve));
}

/// `rule` refused, and nothing of the drawing touched: its rules, its values,
/// its points, the size of `curve` when one is named, and the history.
fn refused_with_the_drawing_unchanged(
    document: PartDocument,
    rule: Constraint,
    curve: Option<Element>,
) {
    let constrain = Operation::Constrain {
        sketch: 0,
        constraint: rule,
    };
    let document = step_refused_with_the_drawing_unchanged(document, constrain, curve);
    assert!(
        !document.sketches()[0].carries(rule),
        "the rule is not kept"
    );
}

fn step_refused_with_the_drawing_unchanged(
    mut document: PartDocument,
    step: Operation,
    curve: Option<Element>,
) -> PartDocument {
    let before = document.sketches()[0].clone();
    let size = curve.map(|curve| size_of(&document, curve));
    let recorded = steps(&document);
    let tried = format!("{step:?}");

    let outcome = document.apply(step);

    assert_eq!(
        outcome,
        Some(Outcome::RuleRefused),
        "{tried} was laid, the curve's size now {:?}",
        curve.map(|curve| size_of(&document, curve))
    );
    let sketch = &document.sketches()[0];
    assert_eq!(
        sketch.constraints(),
        before.constraints(),
        "the rules are as they were"
    );
    assert!(
        sketch
            .dimensions()
            .iter()
            .map(|dimension| (dimension.target, dimension.value))
            .eq(before
                .dimensions()
                .iter()
                .map(|dimension| (dimension.target, dimension.value))),
        "the values are as they were"
    );
    assert_eq!(sketch.points().len(), before.points().len());
    assert!(
        sketch
            .points()
            .iter()
            .zip(before.points())
            .all(|(now, was)| now.distance(*was) < 1e-9),
        "nothing about the drawing moved"
    );
    if let (Some(curve), Some(size)) = (curve, size) {
        assert!(
            (size_of(&document, curve) - size).abs() < 1e-9,
            "the curve's size stays {size}, got {}",
            size_of(&document, curve)
        );
    }
    assert_eq!(steps(&document), recorded, "the history has no new step");
    document
}

#[test]
fn a_tangency_between_an_arc_and_a_trait_through_its_centre_is_refused() {
    tangency_through_the_centre_is_refused(
        an_arc_and_a_trait_from_its_centre(true),
        Element::Arc(cao_sketch::ArcId(0)),
    );
}

#[test]
fn a_tangency_between_an_arc_of_free_radius_and_a_trait_through_its_centre_is_refused() {
    tangency_through_the_centre_is_refused(
        an_arc_and_a_trait_from_its_centre(false),
        Element::Arc(cao_sketch::ArcId(0)),
    );
}

#[test]
fn a_tangency_between_a_circle_and_a_trait_through_its_centre_is_refused() {
    tangency_through_the_centre_is_refused(
        a_circle_and_a_trait_from_its_centre(true),
        Element::Circle(cao_sketch::CircleId(0)),
    );
}

#[test]
fn a_tangency_between_a_circle_of_free_radius_and_a_trait_through_its_centre_is_refused() {
    tangency_through_the_centre_is_refused(
        a_circle_and_a_trait_from_its_centre(false),
        Element::Circle(cao_sketch::CircleId(0)),
    );
}

fn last_trait(sketch: &cao_sketch::Sketch) -> SegmentId {
    SegmentId(sketch.segments().len() - 1)
}

#[test]
fn a_tangency_between_half_an_ellipse_and_a_trait_through_its_centre_is_refused() {
    let mut document = a_blank_sketch();
    let centre = PointRef::New(DVec2::new(20.0, 0.0));
    let (left, right) = (
        PointRef::New(DVec2::new(0.0, 0.0)),
        PointRef::New(DVec2::new(40.0, 0.0)),
    );
    document.apply(Operation::AddEllipse {
        sketch: 0,
        center: centre.clone(),
        first: [left.clone(), right.clone()],
        second: [centre, PointRef::New(DVec2::new(20.0, 15.0))],
        construction: false,
        drawn: Some([right, left]),
    });
    let centre = document.sketches()[0].ellipses()[0].center;
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(centre),
        end: PointRef::New(DVec2::new(40.0, 30.0)),
        construction: false,
    });

    tangency_through_the_centre_is_refused(document, Element::Ellipse(cao_sketch::EllipseId(0)));
}

/// `curve`'s centre held on a trait from (0, 0) to (40, 30) — whose middle is
/// (20, 15), where the centre stands — by `rule` laid as the tool lays it.
fn the_centre_held_on_a_trait(
    mut document: PartDocument,
    curve: Element,
    rule: Rule,
) -> PartDocument {
    let sketch = &document.sketches()[0];
    let centre = match curve {
        Element::Circle(circle) => sketch.circles()[circle.0].center,
        Element::Arc(arc) => sketch.arcs()[arc.0].center,
        other => panic!("not a round curve: {other:?}"),
    };
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 0.0)),
        end: PointRef::New(DVec2::new(40.0, 30.0)),
        construction: false,
    });
    let sketch = &document.sketches()[0];
    let picks = [
        RulePick::Element(Element::Point(centre)),
        RulePick::Element(Element::Segment(last_trait(sketch))),
    ];
    let Some(RuleIntent::Constrain(held)) = rule_intent(rule, &picks, sketch) else {
        panic!("the tool offers no {rule:?} for a centre and a trait");
    };
    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: held,
    });
    assert_eq!(outcome, None, "{held:?} could not be laid");
    assert!(document.sketches()[0].carries(held));
    document
}

fn a_free_circle() -> PartDocument {
    let mut document = a_blank_sketch();
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(20.0, 15.0)),
        radius: 25.0,
        rim: Vec::new(),
        construction: false,
    });
    document
}

fn a_free_arc() -> PartDocument {
    let mut document = a_blank_sketch();
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(20.0, 15.0)),
        start: PointRef::New(DVec2::new(45.0, 15.0)),
        end: PointRef::New(DVec2::new(20.0, 40.0)),
        construction: false,
    });
    document
}

#[test]
fn a_tangency_between_a_circle_of_free_radius_and_a_trait_its_centre_is_held_on_is_refused() {
    let circle = Element::Circle(cao_sketch::CircleId(0));
    tangency_through_the_centre_is_refused(
        the_centre_held_on_a_trait(a_free_circle(), circle, Rule::Coincident),
        circle,
    );
}

#[test]
fn a_tangency_between_an_arc_of_free_radius_and_a_trait_its_centre_is_held_on_is_refused() {
    let arc = Element::Arc(cao_sketch::ArcId(0));
    tangency_through_the_centre_is_refused(
        the_centre_held_on_a_trait(a_free_arc(), arc, Rule::Coincident),
        arc,
    );
}

#[test]
fn a_tangency_between_a_circle_of_free_radius_and_a_trait_its_centre_is_the_middle_of_is_refused() {
    let circle = Element::Circle(cao_sketch::CircleId(0));
    tangency_through_the_centre_is_refused(
        the_centre_held_on_a_trait(a_free_circle(), circle, Rule::Midpoint),
        circle,
    );
}

#[test]
fn a_tangency_between_an_arc_of_free_radius_and_a_trait_its_centre_is_the_middle_of_is_refused() {
    let arc = Element::Arc(cao_sketch::ArcId(0));
    tangency_through_the_centre_is_refused(
        the_centre_held_on_a_trait(a_free_arc(), arc, Rule::Midpoint),
        arc,
    );
}

fn a_tangency_with_a_trait_merely_crossing_the_centre_lands(
    mut document: PartDocument,
    curve: Element,
) -> PartDocument {
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 0.0)),
        end: PointRef::New(DVec2::new(40.0, 30.0)),
        construction: false,
    });
    let sketch = &document.sketches()[0];
    let picks = [
        RulePick::Element(Element::Segment(last_trait(sketch))),
        RulePick::Element(curve),
    ];
    let Some(RuleIntent::Constrain(rule)) = rule_intent(Rule::Tangent, &picks, sketch) else {
        panic!("the tool offers no tangency for a trait and a curve");
    };
    let radius = size_of(&document, curve);

    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });

    assert_eq!(outcome, None, "{rule:?} was refused");
    let sketch = &document.sketches()[0];
    assert!(
        sketch.constraints().iter().any(|held| match (*held, rule) {
            (
                Constraint::Tangent {
                    circle, segment, ..
                },
                Constraint::Tangent {
                    circle: c,
                    segment: s,
                    ..
                },
            ) => circle == c && segment == s,
            (
                Constraint::ArcTangent { arc, segment, .. },
                Constraint::ArcTangent {
                    arc: a, segment: s, ..
                },
            ) => arc == a && segment == s,
            _ => false,
        }),
        "{rule:?} is not kept: {:?}",
        sketch.constraints()
    );
    assert!(
        size_of(&document, curve) > radius * 1e-3,
        "the curve keeps a size: radius {radius} became {}",
        size_of(&document, curve)
    );
    document
}

#[test]
fn a_tangency_between_a_circle_and_a_trait_merely_crossing_its_centre_lands() {
    a_tangency_with_a_trait_merely_crossing_the_centre_lands(
        a_free_circle(),
        Element::Circle(cao_sketch::CircleId(0)),
    );
}

#[test]
fn a_tangency_between_an_arc_and_a_trait_merely_crossing_its_centre_lands() {
    a_tangency_with_a_trait_merely_crossing_the_centre_lands(
        a_free_arc(),
        Element::Arc(cao_sketch::ArcId(0)),
    );
}

fn the_centre_then_held_on_the_tangent_trait_is_refused(
    curve: Element,
    drawn: PartDocument,
    rule: Rule,
) {
    let document = a_tangency_with_a_trait_merely_crossing_the_centre_lands(drawn, curve);
    let sketch = &document.sketches()[0];
    let centre = match curve {
        Element::Circle(circle) => sketch.circles()[circle.0].center,
        Element::Arc(arc) => sketch.arcs()[arc.0].center,
        other => panic!("not a round curve: {other:?}"),
    };
    let picks = [
        RulePick::Element(Element::Point(centre)),
        RulePick::Element(Element::Segment(last_trait(sketch))),
    ];
    let Some(RuleIntent::Constrain(held)) = rule_intent(rule, &picks, sketch) else {
        panic!("the tool offers no {rule:?} for a centre and a trait");
    };

    refused_with_the_drawing_unchanged(document, held, Some(curve));
}

#[test]
fn a_circle_s_centre_held_on_a_trait_it_is_tangent_to_is_refused() {
    the_centre_then_held_on_the_tangent_trait_is_refused(
        Element::Circle(cao_sketch::CircleId(0)),
        a_free_circle(),
        Rule::Coincident,
    );
}

#[test]
fn a_circle_s_centre_made_the_middle_of_a_trait_it_is_tangent_to_is_refused() {
    the_centre_then_held_on_the_tangent_trait_is_refused(
        Element::Circle(cao_sketch::CircleId(0)),
        a_free_circle(),
        Rule::Midpoint,
    );
}

#[test]
fn an_arc_s_centre_held_on_a_trait_it_is_tangent_to_is_refused() {
    the_centre_then_held_on_the_tangent_trait_is_refused(
        Element::Arc(cao_sketch::ArcId(0)),
        a_free_arc(),
        Rule::Coincident,
    );
}

#[test]
fn an_arc_s_centre_made_the_middle_of_a_trait_it_is_tangent_to_is_refused() {
    the_centre_then_held_on_the_tangent_trait_is_refused(
        Element::Arc(cao_sketch::ArcId(0)),
        a_free_arc(),
        Rule::Midpoint,
    );
}

/// A slanted trait from (10, 10) to (30, 25): 20 across and 15 up, given the
/// values named.
fn a_slanted_trait_given(values: &[(SketchAxis, f64)]) -> PartDocument {
    let mut document = a_blank_sketch();
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 10.0)),
        end: PointRef::New(DVec2::new(30.0, 25.0)),
        construction: false,
    });
    let trait_ = document.sketches()[0].segments()[0];
    for &(axis, value) in values {
        document.apply(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Projected {
                from: trait_.start,
                to: trait_.end,
                axis,
            },
            value: value.into(),
            placement: None,
        });
    }
    document
}

fn the_trait_and_the_axis(axis: SketchAxis) -> [RulePick; 2] {
    [
        RulePick::Element(Element::Segment(SegmentId(0))),
        RulePick::Axis(axis),
    ]
}

#[test]
fn a_trait_given_a_horizontal_value_is_not_turned_vertical() {
    let document = a_slanted_trait_given(&[(SketchAxis::U, 20.0)]);

    let rule = the_rule(
        &document,
        Rule::Parallel,
        &the_trait_and_the_axis(SketchAxis::V),
    );
    refused_with_the_drawing_unchanged(document, rule, None);
}

#[test]
fn a_trait_with_no_value_is_turned_vertical() {
    let mut document = a_slanted_trait_given(&[]);

    landed(
        &mut document,
        Rule::Parallel,
        &the_trait_and_the_axis(SketchAxis::V),
    );

    let (start, end) = document.sketches()[0].endpoints(SegmentId(0));
    assert!(
        (start.x - end.x).abs() < 1e-6,
        "the trait stands vertical: {start} to {end}"
    );
}

#[test]
fn a_trait_given_a_vertical_value_is_still_made_vertical() {
    let mut document = a_slanted_trait_given(&[(SketchAxis::V, 15.0)]);

    landed(
        &mut document,
        Rule::Parallel,
        &the_trait_and_the_axis(SketchAxis::V),
    );

    let (start, end) = document.sketches()[0].endpoints(SegmentId(0));
    assert!(
        (start.x - end.x).abs() < 1e-6 && ((end.y - start.y).abs() - 15.0).abs() < 1e-6,
        "the trait stands vertical, 15 up: {start} to {end}"
    );
}

/// An ellipse about (50, 50), its first axis 20 across along U, its second 10
/// up along V, given the values named across its first axis.
fn an_ellipse_given(values: &[(SketchAxis, f64)]) -> PartDocument {
    let mut document = a_blank_sketch();
    document.apply(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(DVec2::new(50.0, 50.0)),
        first: [
            PointRef::New(DVec2::new(40.0, 50.0)),
            PointRef::New(DVec2::new(60.0, 50.0)),
        ],
        second: [
            PointRef::New(DVec2::new(50.0, 45.0)),
            PointRef::New(DVec2::new(50.0, 55.0)),
        ],
        construction: false,
        drawn: None,
    });
    let sketch = &document.sketches()[0];
    let first = sketch.segments()[sketch.ellipses()[0].first.0];
    for &(axis, value) in values {
        document.apply(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Projected {
                from: first.start,
                to: first.end,
                axis,
            },
            value: value.into(),
            placement: None,
        });
    }
    document
}

fn the_second_axis_and_u(document: &PartDocument) -> [RulePick; 2] {
    let second = document.sketches()[0].ellipses()[EllipseId(0).0].second;
    [
        RulePick::Element(Element::Segment(second)),
        RulePick::Axis(SketchAxis::U),
    ]
}

#[test]
fn an_ellipse_given_a_horizontal_value_across_its_axis_is_not_turned() {
    let document = an_ellipse_given(&[(SketchAxis::U, 20.0)]);
    let picks = the_second_axis_and_u(&document);

    let rule = the_rule(&document, Rule::Parallel, &picks);
    refused_with_the_drawing_unchanged(document, rule, Some(Element::Ellipse(EllipseId(0))));
}

#[test]
fn an_ellipse_with_no_value_is_turned() {
    let mut document = an_ellipse_given(&[]);
    let picks = the_second_axis_and_u(&document);

    landed(&mut document, Rule::Parallel, &picks);

    let sketch = &document.sketches()[0];
    let ellipse = sketch.ellipses()[0];
    let (start, end) = sketch.endpoints(ellipse.second);
    assert!(
        (start.y - end.y).abs() < 1e-6,
        "the second axis lies along U: {start} to {end}"
    );
    let (start, end) = sketch.endpoints(ellipse.first);
    assert!(
        (start.x - end.x).abs() < 1e-6,
        "the first axis turned with it, along V: {start} to {end}"
    );
}

/// A trait held vertical from (10, 10) to (10, 40), and a slanted one from
/// (30, 10) to (50, 25) given the values named.
fn a_vertical_trait_and_a_slanted_one_given(values: &[(SketchAxis, f64)]) -> PartDocument {
    let mut document = a_blank_sketch();
    for (start, end) in [
        (DVec2::new(10.0, 10.0), DVec2::new(10.0, 40.0)),
        (DVec2::new(30.0, 10.0), DVec2::new(50.0, 25.0)),
    ] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(start),
            end: PointRef::New(end),
            construction: false,
        });
    }
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::AxisParallel {
            segment: SegmentId(0),
            axis: SketchAxis::V,
        },
    });
    let slanted = document.sketches()[0].segments()[1];
    for &(axis, value) in values {
        document.apply(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Projected {
                from: slanted.start,
                to: slanted.end,
                axis,
            },
            value: value.into(),
            placement: None,
        });
    }
    document
}

const THE_TWO_TRAITS: [RulePick; 2] = [
    RulePick::Element(Element::Segment(SegmentId(0))),
    RulePick::Element(Element::Segment(SegmentId(1))),
];

#[test]
fn a_trait_given_a_vertical_value_is_not_turned_square_to_a_vertical_one() {
    let document = a_vertical_trait_and_a_slanted_one_given(&[(SketchAxis::V, 15.0)]);

    let rule = the_rule(&document, Rule::Perpendicular, &THE_TWO_TRAITS);
    refused_with_the_drawing_unchanged(document, rule, None);
}

#[test]
fn a_trait_with_no_value_is_turned_square_to_a_vertical_one() {
    let mut document = a_vertical_trait_and_a_slanted_one_given(&[]);

    landed(&mut document, Rule::Perpendicular, &THE_TWO_TRAITS);

    let (start, end) = document.sketches()[0].endpoints(SegmentId(1));
    assert!(
        (start.y - end.y).abs() < 1e-6,
        "the slanted trait lies horizontal: {start} to {end}"
    );
}

#[test]
fn two_points_a_horizontal_value_holds_apart_are_not_made_one() {
    let mut document = a_blank_sketch();
    for position in [DVec2::new(10.0, 10.0), DVec2::new(30.0, 25.0)] {
        document.apply(Operation::AddPoint {
            sketch: 0,
            position,
            on: Vec::new(),
        });
    }
    let count = document.sketches()[0].points().len();
    let (first, second) = (PointId(count - 2), PointId(count - 1));
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Projected {
            from: first,
            to: second,
            axis: SketchAxis::U,
        },
        value: 20.0.into(),
        placement: None,
    });
    let picks = [
        RulePick::Element(Element::Point(first)),
        RulePick::Element(Element::Point(second)),
    ];
    let Some(RuleIntent::Merge { kept, dropped }) =
        rule_intent(Rule::Coincident, &picks, &document.sketches()[0])
    else {
        panic!("the tool does not make two points one");
    };

    step_refused_with_the_drawing_unchanged(
        document,
        Operation::MergePoints {
            sketch: 0,
            kept,
            dropped,
        },
        None,
    );
}

fn the_merge(document: &PartDocument, first: PointId, second: PointId) -> Operation {
    let picks = [
        RulePick::Element(Element::Point(first)),
        RulePick::Element(Element::Point(second)),
    ];
    let Some(RuleIntent::Merge { kept, dropped }) =
        rule_intent(Rule::Coincident, &picks, &document.sketches()[0])
    else {
        panic!("the tool does not make two points one");
    };
    Operation::MergePoints {
        sketch: 0,
        kept,
        dropped,
    }
}

fn a_distance_typed(document: &mut PartDocument, from: PointId, to: PointId, value: f64) {
    a_value_typed(document, DimensionTarget::Distance { from, to }, value);
}

fn a_value_typed(document: &mut PartDocument, target: DimensionTarget, value: f64) {
    let outcome = document.apply(Operation::SetDimension {
        sketch: 0,
        target,
        value: value.into(),
        placement: None,
    });
    assert!(
        matches!(
            outcome,
            None | Some(Outcome::Dimension(DimensionOutcome::ScaleDefined { .. }))
        ),
        "{target:?} = {value} does not land: {outcome:?}"
    );
    assert!(
        document.sketches()[0]
            .dimensions()
            .iter()
            .any(|laid| laid.target == target && !laid.driven),
        "{target:?} does not drive the drawing"
    );
}

#[test]
fn a_corner_a_distance_holds_apart_from_the_origin_is_not_made_one_with_it() {
    let mut document = a_blank_sketch();
    let corners = [[2.0, 0.0], [0.5, 0.0], [0.5, 1.0], [2.0, 1.0]].map(|[x, y]| {
        document.apply(Operation::AddPoint {
            sketch: 0,
            position: DVec2::new(x, y),
            on: Vec::new(),
        });
        PointId(document.sketches()[0].points().len() - 1)
    });
    for index in 0..4 {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(corners[index]),
            end: PointRef::Existing(corners[(index + 1) % 4]),
            construction: false,
        });
    }
    let corner = corners[1];
    assert!(
        document.sketches()[0]
            .point(corner)
            .distance(DVec2::new(0.5, 0.0))
            < 1e-9,
        "the corner is not at (0.5, 0)"
    );
    a_distance_typed(&mut document, corner, Sketch::ORIGIN, 0.5);
    let merge = the_merge(&document, corner, Sketch::ORIGIN);

    step_refused_with_the_drawing_unchanged(document, merge, None);
}

#[test]
fn two_free_points_a_distance_holds_apart_are_not_made_one() {
    let mut document = a_blank_sketch();
    for position in [DVec2::new(10.0, 10.0), DVec2::new(30.0, 25.0)] {
        document.apply(Operation::AddPoint {
            sketch: 0,
            position,
            on: Vec::new(),
        });
    }
    let count = document.sketches()[0].points().len();
    let (first, second) = (PointId(count - 2), PointId(count - 1));
    a_distance_typed(&mut document, first, second, 20.0);
    let merge = the_merge(&document, first, second);

    step_refused_with_the_drawing_unchanged(document, merge, None);
}

/// The rectangle from (0, 1) to (1, -2.5), its first corner held on V: the
/// top, the right side, the bottom, the left. The left side is held at 0° to
/// V, the right side typed 3.535; the bottom and the left are made equal when
/// `equal`, and given 81.258° at both their ends when `apart`.
fn a_quadrilateral(equal: bool, apart: bool) -> PartDocument {
    let mut document = a_blank_sketch();
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::Held {
            at: DVec2::new(0.0, 1.0),
            on: vec![Support::Axis(SketchAxis::V)],
        },
        opposite: PointRef::New(DVec2::new(1.0, -2.5)),
        construction: false,
    });
    let [right, bottom, left] = [SegmentId(1), SegmentId(2), SegmentId(3)];
    if equal {
        let rule = the_rule(
            &document,
            Rule::Equal,
            &[
                RulePick::Element(Element::Segment(bottom)),
                RulePick::Element(Element::Segment(left)),
            ],
        );
        document.apply(Operation::Constrain {
            sketch: 0,
            constraint: rule,
        });
        assert!(document.sketches()[0].carries(rule), "the sides are equal");
    }
    let mut values = vec![
        (
            DimensionTarget::AxisAngle {
                segment: left,
                segment_toward: Toward::End,
                axis: SketchAxis::V,
                axis_toward: AxisToward::Positive,
            },
            0.0,
        ),
        (DimensionTarget::Length(right), 3.535),
    ];
    if apart {
        values.extend([
            (
                DimensionTarget::Angle {
                    first: right,
                    first_along: Along::Trait,
                    second: bottom,
                    second_along: Along::Prolongation,
                },
                81.258,
            ),
            (
                DimensionTarget::Angle {
                    first: bottom,
                    first_along: Along::Trait,
                    second: left,
                    second_along: Along::Trait,
                },
                81.258,
            ),
        ]);
    }
    for (target, value) in values {
        document.apply(Operation::SetDimension {
            sketch: 0,
            target,
            value: value.into(),
            placement: None,
        });
        assert!(
            document.sketches()[0]
                .dimensions()
                .iter()
                .any(|laid| laid.target == target && !laid.driven),
            "{target:?} drives the drawing"
        );
    }
    document
}

fn the_bottom_and_the_left() -> [RulePick; 2] {
    [
        RulePick::Element(Element::Segment(SegmentId(3))),
        RulePick::Element(Element::Segment(SegmentId(2))),
    ]
}

#[test]
fn collinear_between_two_equal_sides_an_angle_holds_apart_is_refused() {
    let document = a_quadrilateral(true, true);

    let rule = the_rule(&document, Rule::Collinear, &the_bottom_and_the_left());
    refused_with_the_drawing_unchanged(document, rule, None);
}

#[test]
fn collinear_between_two_sides_an_angle_holds_apart_is_refused() {
    let document = a_quadrilateral(false, true);

    let rule = the_rule(&document, Rule::Collinear, &the_bottom_and_the_left());
    refused_with_the_drawing_unchanged(document, rule, None);
}

#[test]
fn collinear_between_two_equal_sides_no_angle_holds_apart_lands() {
    let mut document = a_quadrilateral(true, false);

    landed(&mut document, Rule::Collinear, &the_bottom_and_the_left());

    let sketch = &document.sketches()[0];
    let (bottom, left) = (
        sketch.endpoints(SegmentId(2)),
        sketch.endpoints(SegmentId(3)),
    );
    let lengths = [bottom.0.distance(bottom.1), left.0.distance(left.1)];
    assert!(
        lengths.iter().all(|length| (length - 1.0).abs() < 1e-6),
        "both sides keep their length of 1: {lengths:?}"
    );
    let (along, across) = (bottom.0 - bottom.1, left.1 - left.0);
    assert!(
        along.perp_dot(across).abs() < 1e-6 * along.length() * across.length(),
        "the two sides lie on one line: {bottom:?} and {left:?}"
    );
}

/// A trait from (10, 10) to (60, 10), and a free point at (35, 20) beside it,
/// held 10 off its line when `held` says so.
fn a_trait_and_a_point_beside_it(held: bool) -> (PartDocument, PointId) {
    let mut document = a_blank_sketch();
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 10.0)),
        end: PointRef::New(DVec2::new(60.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(35.0, 20.0),
        on: Vec::new(),
    });
    let point = PointId(document.sketches()[0].points().len() - 1);
    if held {
        a_value_typed(
            &mut document,
            DimensionTarget::PointToSegment {
                point,
                segment: SegmentId(0),
            },
            10.0,
        );
    }
    (document, point)
}

/// The same, with a second trait running from the point to (55, 30).
fn a_trait_and_one_from_a_point_beside_it(held: bool) -> (PartDocument, PointId) {
    let (mut document, point) = a_trait_and_a_point_beside_it(held);
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(point),
        end: PointRef::New(DVec2::new(55.0, 30.0)),
        construction: false,
    });
    (document, point)
}

fn the_point_and_the_trait(point: PointId) -> [RulePick; 2] {
    [
        RulePick::Element(Element::Point(point)),
        RulePick::Element(Element::Segment(SegmentId(0))),
    ]
}

fn off_the_first_trait_s_line(document: &PartDocument, point: PointId) -> f64 {
    let sketch = &document.sketches()[0];
    let (start, end) = sketch.endpoints(SegmentId(0));
    let along = (end - start).normalize();
    along.perp_dot(sketch.point(point) - start).abs()
}

#[test]
fn a_point_a_value_holds_off_a_trait_is_not_put_on_it() {
    let (document, point) = a_trait_and_a_point_beside_it(true);

    let rule = the_rule(&document, Rule::Coincident, &the_point_and_the_trait(point));
    refused_with_the_drawing_unchanged(document, rule, None);
}

#[test]
fn a_free_point_is_put_on_a_trait() {
    let (mut document, point) = a_trait_and_a_point_beside_it(false);

    landed(
        &mut document,
        Rule::Coincident,
        &the_point_and_the_trait(point),
    );

    let off = off_the_first_trait_s_line(&document, point);
    assert!(off < 1e-6, "the point lies on the trait, {off} off it");
}

#[test]
fn a_point_a_value_holds_off_a_trait_is_not_made_its_middle() {
    let (document, point) = a_trait_and_a_point_beside_it(true);

    let rule = the_rule(&document, Rule::Midpoint, &the_point_and_the_trait(point));
    refused_with_the_drawing_unchanged(document, rule, None);
}

#[test]
fn a_free_point_is_made_a_trait_s_middle() {
    let (mut document, point) = a_trait_and_a_point_beside_it(false);

    landed(
        &mut document,
        Rule::Midpoint,
        &the_point_and_the_trait(point),
    );

    let sketch = &document.sketches()[0];
    let (start, end) = sketch.endpoints(SegmentId(0));
    let middle = (start + end) / 2.0;
    assert!(
        sketch.point(point).distance(middle) < 1e-6,
        "the point stands at the trait's middle {middle}, not at {}",
        sketch.point(point)
    );
}

#[test]
fn a_trait_a_value_holds_off_a_point_is_not_made_collinear_through_it() {
    let (document, _) = a_trait_and_one_from_a_point_beside_it(true);

    let rule = the_rule(&document, Rule::Collinear, &THE_TWO_TRAITS);
    refused_with_the_drawing_unchanged(document, rule, None);
}

#[test]
fn a_trait_is_made_collinear_through_a_free_point() {
    let (mut document, point) = a_trait_and_one_from_a_point_beside_it(false);

    landed(&mut document, Rule::Collinear, &THE_TWO_TRAITS);

    let off = off_the_first_trait_s_line(&document, point);
    assert!(
        off < 1e-6,
        "the trait's line runs through the point, {off} off it"
    );
}

#[test]
fn a_point_a_value_holds_off_a_trait_is_not_made_one_with_its_end() {
    let mut document = a_blank_sketch();
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 10.0)),
        end: PointRef::New(DVec2::new(20.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(15.0, 15.0),
        on: Vec::new(),
    });
    let point = PointId(document.sketches()[0].points().len() - 1);
    a_value_typed(
        &mut document,
        DimensionTarget::PointToSegment {
            point,
            segment: SegmentId(0),
        },
        5.0,
    );
    let end = document.sketches()[0].segments()[0].start;
    let merge = the_merge(&document, end, point);

    step_refused_with_the_drawing_unchanged(document, merge, None);
}

#[test]
fn half_an_ellipse_s_end_made_one_with_its_centre_is_refused_while_its_axis_length_is_typed() {
    for end_first in [true, false] {
        for which in [0, 1] {
            let mut document = a_blank_sketch();
            let centre = PointRef::New(DVec2::new(20.0, 0.0));
            let (left, right) = (
                PointRef::New(DVec2::new(0.0, 0.0)),
                PointRef::New(DVec2::new(40.0, 0.0)),
            );
            document.apply(Operation::AddEllipse {
                sketch: 0,
                center: centre.clone(),
                first: [left.clone(), right.clone()],
                second: [centre, PointRef::New(DVec2::new(20.0, 15.0))],
                construction: false,
                drawn: Some([right, left]),
            });
            let curve = EllipseId(0);
            let axis = document.sketches()[0].ellipses()[curve.0].first;
            let outcome = document.apply(Operation::SetDimension {
                sketch: 0,
                target: DimensionTarget::Length(axis),
                value: 40.0.into(),
                placement: None,
            });
            assert!(
                matches!(
                    outcome,
                    None | Some(Outcome::Dimension(DimensionOutcome::ScaleDefined { .. }))
                ),
                "the axis length does not land: {outcome:?}"
            );
            let sketch = &document.sketches()[0];
            let centre = sketch.ellipses()[curve.0].center;
            let (from, to) = sketch
                .ellipse_ends(curve)
                .expect("half an ellipse has two ends");
            let end = [from, to][which];
            let merge = match end_first {
                true => the_merge(&document, end, centre),
                false => the_merge(&document, centre, end),
            };

            let document = step_refused_with_the_drawing_unchanged(
                document,
                merge,
                Some(Element::Ellipse(curve)),
            );

            let sketch = &document.sketches()[0];
            assert!(
                !sketch.is_erased_ellipse(curve) && sketch.ellipse_ends(curve) == Some((from, to)),
                "the end clicked first: {end_first}, and the curve went or its stretch changed"
            );
            assert!(
                sketch.regions().is_empty(),
                "the end clicked first: {end_first}, and an area appeared"
            );
        }
    }
}
