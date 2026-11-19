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

use cao_part::{Operation, Outcome, PartDocument, PointRef};
use cao_sketch::{
    Constraint, DimensionTarget, Element, PointId, Rule, RuleIntent, RulePick, SegmentId,
    SketchAxis, WorkPlane, rule_intent,
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

fn centre_put_on_its_own_curve_is_refused(mut document: PartDocument, curve: Element) {
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
    let Some(RuleIntent::Constrain(rule)) = rule_intent(Rule::Coincident, &picks, sketch) else {
        panic!("the tool offers no rule for a centre and its own curve");
    };
    let before = sketch.clone();
    let recorded = steps(&document);

    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });

    assert_eq!(outcome, Some(Outcome::RuleRefused), "{rule:?} was laid");
    let sketch = &document.sketches()[0];
    assert!(!sketch.carries(rule), "the rule is not kept");
    assert_eq!(sketch.points().len(), before.points().len());
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
    refused_leaving_the_curve_as_it_was(document, rule, curve);
}

fn refused_leaving_the_curve_as_it_was(
    mut document: PartDocument,
    rule: Constraint,
    curve: Element,
) {
    let before = document.sketches()[0].clone();
    let radius = size_of(&document, curve);
    let recorded = steps(&document);

    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });

    assert_eq!(
        outcome,
        Some(Outcome::RuleRefused),
        "{rule:?} was laid, the curve's size now {}",
        size_of(&document, curve)
    );
    let sketch = &document.sketches()[0];
    assert_eq!(
        sketch.constraints(),
        before.constraints(),
        "the rule is not kept"
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
    assert!(
        (size_of(&document, curve) - radius).abs() < 1e-9,
        "the curve's size stays {radius}, got {}",
        size_of(&document, curve)
    );
    assert_eq!(steps(&document), recorded, "the history has no new step");
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

    refused_leaving_the_curve_as_it_was(document, held, curve);
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
