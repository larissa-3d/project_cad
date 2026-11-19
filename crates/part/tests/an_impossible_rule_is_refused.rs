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
//!   `a_tangency_between_a_circle_of_free_radius_and_a_trait_through_its_centre_is_refused`
//! - The cases above hold in #501's campaign — no test: the campaign lives in
//!   `cao_sketch`, and the four drawings are held to its rules there, under
//!   the same names, in `crates/sketch/tests/what_random_sketches_found.rs`

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

fn radius_of(document: &PartDocument, curve: Element) -> f64 {
    let sketch = &document.sketches()[0];
    match curve {
        Element::Circle(circle) => sketch.circle(circle).radius,
        Element::Arc(arc) => sketch.arc_radius(arc),
        other => panic!("not a round curve: {other:?}"),
    }
}

fn tangency_through_the_centre_is_refused(mut document: PartDocument, curve: Element) {
    let sketch = &document.sketches()[0];
    let picks = [
        RulePick::Element(Element::Segment(SegmentId(0))),
        RulePick::Element(curve),
    ];
    let Some(RuleIntent::Constrain(rule)) = rule_intent(Rule::Tangent, &picks, sketch) else {
        panic!("the tool offers no tangency for a trait and a curve");
    };
    let before = sketch.clone();
    let radius = radius_of(&document, curve);
    let recorded = steps(&document);

    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });

    assert_eq!(
        outcome,
        Some(Outcome::RuleRefused),
        "{rule:?} was laid, the radius now {}",
        radius_of(&document, curve)
    );
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
    assert!(
        (radius_of(&document, curve) - radius).abs() < 1e-9,
        "the radius stays {radius}, got {}",
        radius_of(&document, curve)
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
