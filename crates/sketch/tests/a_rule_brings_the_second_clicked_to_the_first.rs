//! Closes #451.
//! - a perpendicular laid in both orders leaves the first trait clicked where
//!   it was — `a_perpendicular_leaves_the_first_trait_clicked_where_it_was`
//! - a parallel and a collinear do the same —
//!   `a_parallel_leaves_the_first_trait_clicked_where_it_was`,
//!   `a_collinear_leaves_the_first_trait_clicked_where_it_was`
//! - a tangent between a trait and a circle brings the second clicked to the
//!   first — `a_tangent_brings_the_second_clicked_to_the_first`
//! - a point laid on a trait comes onto it in both orders —
//!   `a_point_laid_on_a_trait_comes_onto_it_whichever_was_clicked_first`
//! - **Equal** keeps the first trait's length, as today —
//!   `an_equality_keeps_the_length_of_the_first_trait_clicked`
//! - an angle typed between two free traits, in both orders, turns only the
//!   second trait clicked — `an_angle_typed_turns_only_the_second_trait_clicked`
//!
//! Beside the criteria, what the issue says around them: two circles made equal
//! (`two_circles_made_equal_leave_the_first_clicked_as_it_was`), an angle where
//! two traits cross
//! (`an_angle_typed_where_two_traits_cross_turns_only_the_second_clicked`), the
//! rank of #445 deciding before the order of the clicks
//! (`a_fixed_trait_stays_although_it_was_clicked_second`), an angle retyped
//! turning the same trait
//! (`an_angle_retyped_the_other_way_round_turns_the_same_trait`), and the
//! reference held only while the rule lands
//! (`a_length_typed_later_on_the_second_trait_reaches_the_first`,
//! `an_end_of_the_second_of_two_equal_traits_dragged_stretches_the_first_with_it`).
//!
//! A curve beside the end of a trait and the trait are brought alongside each
//! other, an arc and an ellipse as a circle is: the trait clicked first, the
//! curve comes along it until it touches the trait itself; the curve clicked
//! first, the trait slides along itself to it before the solver closes the
//! gap, turning it about its far end
//! (`an_arc_beside_the_end_of_a_trait_is_brought_alongside_it`,
//! `an_ellipse_beside_the_end_of_a_trait_is_brought_alongside_it`).
//!
//! Closes #1040.
//! - In both cases the fixed trait does not move and the free curve comes to
//!   it — `a_fixed_side_stays_and_a_free_circle_made_tangent_to_it_comes_whichever_was_clicked_first`,
//!   `a_free_circle_s_centre_put_on_a_fixed_side_comes_onto_it_whichever_was_clicked_first`;
//!   the same rules with nothing fixed still land as they did —
//!   `a_tangency_between_a_side_and_a_circle_with_nothing_fixed_still_leaves_the_first_clicked_where_it_was`,
//!   `a_circle_s_centre_put_on_a_side_with_nothing_fixed_still_lands_touching_the_rectangle`;
//!   and with the curve fixed too, something fixed still gives —
//!   `a_rule_between_a_fixed_side_and_a_fixed_circle_still_lands_since_nothing_free_can_hold_it`;
//!   and the other way round, the circle fixed — its centre held, its radius
//!   free unless its diameter is typed — and the side free —
//!   `a_fixed_circle_stays_and_a_free_side_made_tangent_to_it_comes_whichever_was_clicked_first`
//! - The cases above hold in #501's campaign — no test: not in this file;
//!   `what_random_sketches_found.rs` holds the two drawings

use cao_sketch::{
    CircleId, Constraint, DimensionTarget, Element, LaidFrom, LengthOutcome, PointId, Rule,
    RuleIntent, RulePick, SegmentId, Sketch, Support, Toward, WorkPlane, rule_intent,
};
use glam::DVec2;

const SCALE: f64 = 1.0;
const CLOSE: f64 = 1e-6;
/// What the solver promises of a value it had to reach by moving both sides of
/// a rule: a hundred-thousandth of the drawing's size, which is a hundred
/// units across here.
const SETTLED: f64 = 1e-5 * 100.0;

/// Two traits hanging off nothing, leaning at an angle no rule names: one from
/// (10, 10) to (110, 30), the other from (10, 90) to (60, 180). Neither
/// touches the origin, so nothing but the rule laid decides which gives way.
fn two_free_traits() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    for place in [
        DVec2::new(10.0, 10.0),
        DVec2::new(110.0, 30.0),
        DVec2::new(10.0, 90.0),
        DVec2::new(60.0, 180.0),
    ] {
        sketch.add_point(place);
    }
    sketch.add_segment(PointId(1), PointId(2));
    sketch.add_segment(PointId(3), PointId(4));
    sketch
}

/// Two traits meeting at a corner, 60° or so apart.
fn a_corner() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let far_one = sketch.add_point(DVec2::new(110.0, 10.0));
    let far_other = sketch.add_point(DVec2::new(60.0, 100.0));
    sketch.add_segment(corner, far_one);
    sketch.add_segment(corner, far_other);
    sketch
}

fn ends(sketch: &Sketch, segment: usize) -> [DVec2; 2] {
    let drawn = sketch.segments()[segment];
    [sketch.point(drawn.start), sketch.point(drawn.end)]
}

/// How far the trait moved, taken as the worse of its two ends.
fn moved(sketch: &Sketch, segment: usize, from: [DVec2; 2]) -> f64 {
    let now = ends(sketch, segment);
    now[0].distance(from[0]).max(now[1].distance(from[1]))
}

const BOTH_ORDERS: [[usize; 2]; 2] = [[0, 1], [1, 0]];

/// Lays a rule between the two traits of `two_free_traits`, clicked in each
/// order in turn, and says how far the first and the second clicked travelled.
fn laid(rule: impl Fn(SegmentId, SegmentId) -> Constraint) -> [(f64, f64); 2] {
    BOTH_ORDERS.map(|[first, second]| {
        let mut sketch = two_free_traits();
        let before = [ends(&sketch, 0), ends(&sketch, 1)];
        let outcome = sketch.lay_rule(rule(SegmentId(first), SegmentId(second)), SCALE);
        assert_eq!(outcome, LengthOutcome::Exact, "the rule did not land");
        (
            moved(&sketch, first, before[first]),
            moved(&sketch, second, before[second]),
        )
    })
}

fn assert_only_the_second_moved(travel: [(f64, f64); 2], rule: &str) {
    for (order, (first, second)) in travel.into_iter().enumerate() {
        assert!(
            first < CLOSE,
            "{rule}, order {order}: the first trait clicked moved by {first}"
        );
        assert!(
            second > CLOSE,
            "{rule}, order {order}: the second trait clicked did not move"
        );
    }
}

#[test]
fn a_perpendicular_leaves_the_first_trait_clicked_where_it_was() {
    assert_only_the_second_moved(
        laid(|first, second| Constraint::Perpendicular { first, second }),
        "perpendicular",
    );
}

#[test]
fn a_parallel_leaves_the_first_trait_clicked_where_it_was() {
    assert_only_the_second_moved(
        laid(|first, second| Constraint::Parallel { first, second }),
        "parallel",
    );
}

#[test]
fn a_collinear_leaves_the_first_trait_clicked_where_it_was() {
    assert_only_the_second_moved(
        laid(|first, second| Constraint::Collinear { first, second }),
        "collinear",
    );
}

#[test]
fn an_equality_keeps_the_length_of_the_first_trait_clicked() {
    for [first, second] in BOTH_ORDERS {
        let mut sketch = two_free_traits();
        let wanted = sketch.segment_length(SegmentId(first));
        sketch.lay_rule(
            Constraint::Equal {
                first: SegmentId(first),
                second: SegmentId(second),
            },
            SCALE,
        );
        assert!(
            (sketch.segment_length(SegmentId(first)) - wanted).abs() < CLOSE,
            "the first trait clicked changed length"
        );
        assert!(
            (sketch.segment_length(SegmentId(second)) - wanted).abs() < CLOSE,
            "the second trait clicked did not take the first's length"
        );
    }
}

/// What a tangency laid between a trait and a circle moved: how far the
/// trait's ends went, and how far the circle's centre and rim went.
fn touched(centre: DVec2, diameter: Option<f64>, from: LaidFrom) -> (f64, f64) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(10.0, 10.0));
    let b = sketch.add_point(DVec2::new(210.0, 10.0));
    let segment = sketch.add_segment(a, b);
    let centre = sketch.add_point(centre);
    let circle = sketch.add_circle(centre, 30.0);
    if let Some(diameter) = diameter {
        sketch.set_dimension(DimensionTarget::Diameter(circle), diameter, false);
        sketch.resolve(SCALE);
    }

    let before = (
        ends(&sketch, 0),
        sketch.point(centre),
        sketch.circle(circle).radius,
    );
    let outcome = sketch.lay_rule(
        Constraint::Tangent {
            circle,
            segment,
            at: None,
            from,
        },
        SCALE,
    );
    assert_eq!(outcome, LengthOutcome::Exact, "the tangency did not land");

    (
        moved(&sketch, 0, before.0),
        sketch.point(centre).distance(before.1) + (sketch.circle(circle).radius - before.2).abs(),
    )
}

#[test]
fn a_tangent_brings_the_second_clicked_to_the_first() {
    let over_the_trait = DVec2::new(100.0, 90.0);
    let beside_its_end = DVec2::new(260.0, 90.0);
    for centre in [over_the_trait, beside_its_end] {
        for diameter in [None, Some(60.0)] {
            for from in [LaidFrom::Curve, LaidFrom::Trait] {
                let (trait_moved, circle_moved) = touched(centre, diameter, from);
                let (stayed, came) = match from {
                    LaidFrom::Trait => (trait_moved, circle_moved),
                    _ => (circle_moved, trait_moved),
                };
                let case = format!("{from:?} first, centre at {centre}, diameter {diameter:?}");
                assert!(
                    stayed < CLOSE,
                    "{case}: the first clicked moved by {stayed}"
                );
                assert!(
                    came > CLOSE,
                    "{case}: the second clicked stayed where it was"
                );
            }
        }
    }
}

#[test]
fn a_point_laid_on_a_trait_comes_onto_it_whichever_was_clicked_first() {
    for point_first in [true, false] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(10.0, 10.0));
        let b = sketch.add_point(DVec2::new(210.0, 10.0));
        let segment = sketch.add_segment(a, b);
        let loose = sketch.add_point(DVec2::new(100.0, 90.0));
        let (point, line) = (
            RulePick::Element(Element::Point(loose)),
            RulePick::Element(Element::Segment(segment)),
        );
        let picks = match point_first {
            true => [point, line],
            false => [line, point],
        };
        let Some(RuleIntent::Constrain(rule)) = rule_intent(Rule::Coincident, &picks, &sketch)
        else {
            panic!("a point and a trait make no coincidence");
        };

        let before = ends(&sketch, 0);
        sketch.lay_rule(rule, SCALE);

        assert!(
            moved(&sketch, 0, before) < CLOSE,
            "point first: {point_first}, the trait moved to meet the point"
        );
        let span = sketch.point(b) - sketch.point(a);
        let across = span.perp_dot(sketch.point(loose) - sketch.point(a)) / span.length();
        assert!(
            across.abs() < CLOSE,
            "point first: {point_first}, the point is {across} off the trait"
        );
    }
}

#[test]
fn two_circles_made_equal_leave_the_first_clicked_as_it_was() {
    for [first, second] in BOTH_ORDERS {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let circles = [
            (DVec2::new(40.0, 40.0), 20.0),
            (DVec2::new(140.0, 60.0), 35.0),
        ]
        .map(|(centre, radius)| {
            let centre = sketch.add_point(centre);
            sketch.add_circle(centre, radius)
        });
        let before = circles.map(|circle| sketch.circle(circle));

        sketch.lay_rule(
            Constraint::EqualRadius {
                first: circles[first],
                second: circles[second],
            },
            SCALE,
        );

        let now = circles.map(|circle| sketch.circle(circle));
        assert!(
            (now[first].radius - before[first].radius).abs() < CLOSE,
            "the first circle clicked changed size"
        );
        assert!(
            (now[second].radius - before[first].radius).abs() < CLOSE,
            "the second circle clicked did not take the first's size"
        );
    }
}

#[test]
fn a_fixed_trait_stays_although_it_was_clicked_second() {
    let mut sketch = two_free_traits();
    let before = [ends(&sketch, 0), ends(&sketch, 1)];
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Segment(SegmentId(1)),
    });
    let outcome = sketch.lay_rule(
        Constraint::Perpendicular {
            first: SegmentId(0),
            second: SegmentId(1),
        },
        SCALE,
    );

    assert_eq!(outcome, LengthOutcome::Exact, "the rule was refused");
    assert!(
        moved(&sketch, 1, before[1]) < CLOSE,
        "the fixed trait moved although nothing may move it"
    );
    assert!(
        moved(&sketch, 0, before[0]) > CLOSE,
        "nothing gave way, so the rule cannot hold"
    );
}

fn type_the_angle(sketch: &mut Sketch, first: usize, second: usize, degrees: f64) {
    let target = DimensionTarget::corner(SegmentId(first), SegmentId(second));
    sketch.set_dimension(target, degrees, false);
    let outcome = sketch.land_value(target, SCALE);
    assert_eq!(outcome, LengthOutcome::Exact, "the angle did not land");
}

#[test]
fn an_angle_typed_turns_only_the_second_trait_clicked() {
    for [first, second] in BOTH_ORDERS {
        let mut sketch = a_corner();
        let before = [ends(&sketch, 0), ends(&sketch, 1)];
        type_the_angle(&mut sketch, first, second, 30.0);

        assert!(
            moved(&sketch, first, before[first]) < CLOSE,
            "order {first}{second}: the first trait clicked turned"
        );
        assert!(
            moved(&sketch, second, before[second]) > CLOSE,
            "order {first}{second}: the second trait clicked did not turn"
        );
    }
}

#[test]
fn an_angle_typed_where_two_traits_cross_turns_only_the_second_clicked() {
    for [first, second] in BOTH_ORDERS {
        let mut sketch = Sketch::new(WorkPlane::XY);
        for place in [
            DVec2::new(10.0, 60.0),
            DVec2::new(110.0, 60.0),
            DVec2::new(60.0, 10.0),
            DVec2::new(80.0, 110.0),
        ] {
            sketch.add_point(place);
        }
        sketch.add_segment(PointId(1), PointId(2));
        sketch.add_segment(PointId(3), PointId(4));
        let before = [ends(&sketch, 0), ends(&sketch, 1)];

        let target = DimensionTarget::AngleBetween {
            first: SegmentId(first),
            first_toward: Toward::End,
            second: SegmentId(second),
            second_toward: Toward::End,
        };
        sketch.set_dimension(target, 60.0, false);
        assert_eq!(sketch.land_value(target, SCALE), LengthOutcome::Exact);

        assert!(
            moved(&sketch, first, before[first]) < CLOSE,
            "order {first}{second}: the first trait clicked turned"
        );
        assert!(
            moved(&sketch, second, before[second]) > CLOSE,
            "order {first}{second}: the second trait clicked did not turn"
        );
    }
}

#[test]
fn an_end_of_the_second_of_two_equal_traits_dragged_stretches_the_first_with_it() {
    let mut sketch = two_free_traits();
    sketch.lay_rule(
        Constraint::Equal {
            first: SegmentId(0),
            second: SegmentId(1),
        },
        SCALE,
    );
    let dragged = sketch.segments()[1].end;
    let cursor = sketch.point(dragged) + DVec2::new(0.0, 30.0);

    assert_eq!(
        sketch.settle_around(dragged, cursor, SCALE),
        LengthOutcome::Exact
    );

    assert!(
        sketch.point(dragged).distance(cursor) < SETTLED,
        "the end dragged stopped short of the hand"
    );
    let lengths = [0, 1].map(|segment| sketch.segment_length(SegmentId(segment)));
    assert!(
        (lengths[0] - lengths[1]).abs() < SETTLED,
        "the two traits are no longer equal: {lengths:?}"
    );
}

#[test]
fn an_angle_retyped_the_other_way_round_turns_the_same_trait() {
    let mut sketch = a_corner();
    type_the_angle(&mut sketch, 0, 1, 30.0);
    let before = [ends(&sketch, 0), ends(&sketch, 1)];

    type_the_angle(&mut sketch, 1, 0, 45.0);

    assert_eq!(sketch.dimensions().len(), 1, "the angle was laid twice");
    assert!(
        moved(&sketch, 0, before[0]) < CLOSE,
        "retyped, the angle turned the trait it was first typed from"
    );
    assert!(
        moved(&sketch, 1, before[1]) > CLOSE,
        "retyped, the angle turned nothing"
    );
}

#[test]
fn a_length_typed_later_on_the_second_trait_reaches_the_first() {
    let mut sketch = two_free_traits();
    sketch.lay_rule(
        Constraint::Equal {
            first: SegmentId(0),
            second: SegmentId(1),
        },
        SCALE,
    );
    let target = DimensionTarget::Length(SegmentId(1));
    sketch.set_dimension(target, 50.0, false);

    assert_eq!(sketch.land_value(target, SCALE), LengthOutcome::Exact);
    let reached = sketch.segment_length(SegmentId(0));
    assert!(
        (reached - 50.0).abs() < SETTLED,
        "the first trait stayed {reached} long"
    );
}

/// Lays a tangency between the trait from (10, 10) to (210, 10) and a curve
/// standing beside its end, in each order, the first clicked staying. The
/// trait clicked first, the curve touches the trait itself; the curve clicked
/// first, the trait's far end travels along it by most of what the trait
/// lacked. `curve` draws the curve and says the rule and the point it turns
/// about; `touch` reads where the curve touches the trait's line.
fn assert_brought_alongside(
    what: &str,
    curve: impl Fn(&mut Sketch, SegmentId, LaidFrom) -> (Constraint, PointId),
    touch: impl Fn(&Sketch, Constraint) -> DVec2,
) {
    for from in [LaidFrom::Curve, LaidFrom::Trait] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(10.0, 10.0));
        let b = sketch.add_point(DVec2::new(210.0, 10.0));
        let segment = sketch.add_segment(a, b);
        let (rule, centre) = curve(&mut sketch, segment, from);
        let past = |sketch: &Sketch| {
            let (start, end) = (sketch.point(a), sketch.point(b));
            let span = end - start;
            let along = (touch(sketch, rule) - start).dot(span) / span.length_squared();
            (along - along.clamp(0.0, 1.0)).abs() * span.length()
        };
        let case = format!("{what}, {from:?} first");
        let lacked = past(&sketch);
        assert!(
            lacked > 1.0,
            "{case}: the curve was to stand beyond the trait's end"
        );
        let before = (ends(&sketch, 0), sketch.point(centre));

        let outcome = sketch.lay_rule(rule, SCALE);

        assert_eq!(
            outcome,
            LengthOutcome::Exact,
            "{case}: the tangency did not land"
        );
        match from {
            LaidFrom::Trait => {
                let stayed = moved(&sketch, 0, before.0);
                assert!(stayed < CLOSE, "{case}: the trait moved by {stayed}");
                let off = past(&sketch);
                assert!(off < CLOSE, "{case}: it touches {off} past the trait's end");
            }
            _ => {
                let stayed = sketch.point(centre).distance(before.1);
                assert!(stayed < CLOSE, "{case}: the curve moved by {stayed}");
                let along = (before.0[1] - before.0[0]).normalize();
                let slid = (sketch.point(a) - before.0[0]).dot(along);
                assert!(
                    slid > lacked / 2.0,
                    "{case}: the trait slid {slid} along itself, short of the {lacked} it lacked"
                );
            }
        }
    }
}

#[test]
fn an_arc_beside_the_end_of_a_trait_is_brought_alongside_it() {
    assert_brought_alongside(
        "an arc",
        |sketch, segment, from| {
            let centre = sketch.add_point(DVec2::new(260.0, 60.0));
            let start = sketch.add_point(DVec2::new(290.0, 60.0));
            let end = sketch.add_point(DVec2::new(260.0, 90.0));
            let arc = sketch.add_arc(centre, start, end);
            let rule = Constraint::ArcTangent {
                arc,
                segment,
                at: None,
                from,
            };
            (rule, centre)
        },
        |sketch, rule| {
            let Constraint::ArcTangent { arc, segment, .. } = rule else {
                unreachable!()
            };
            sketch
                .foot_on_segment(sketch.arc(arc).center, segment)
                .expect("the trait has a length")
        },
    );
}

#[test]
fn an_ellipse_beside_the_end_of_a_trait_is_brought_alongside_it() {
    assert_brought_alongside(
        "an ellipse",
        |sketch, segment, from| {
            let centre = sketch.add_point(DVec2::new(260.0, 60.0));
            let [west, east, south, north] =
                [(230.0, 60.0), (290.0, 60.0), (260.0, 45.0), (260.0, 75.0)]
                    .map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
            let ellipse = sketch.add_ellipse(centre, [west, east], [south, north]);
            let rule = Constraint::EllipseTangent {
                ellipse,
                segment,
                at: None,
                from,
            };
            (rule, centre)
        },
        |sketch, rule| {
            let Constraint::EllipseTangent {
                ellipse, segment, ..
            } = rule
            else {
                unreachable!()
            };
            sketch
                .ellipse_touching(ellipse, segment)
                .expect("the trait has a length")
        },
    );
}

/// A rectangle from (15, 40) to (25, 55) whose left side — its fourth trait,
/// from (15, 55) down to (15, 40) — may be fixed, and a circle of radius 5
/// centred at (20, 30) below it, whose centre may be fixed too: #1040's first
/// case. The circle stands beside the side's lower end, so one of the two has
/// to travel along the other before they touch.
fn a_side_over_a_circle(side_fixed: bool, circle_fixed: bool) -> (Sketch, SegmentId, CircleId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [(15.0, 40.0), (25.0, 40.0), (25.0, 55.0), (15.0, 55.0)]
        .map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
    let sides =
        [0, 1, 2, 3].map(|index| sketch.add_segment(corners[index], corners[(index + 1) % 4]));
    let centre = sketch.add_point(DVec2::new(20.0, 30.0));
    let circle = sketch.add_circle(centre, 5.0);
    if side_fixed {
        sketch.add_constraint(Constraint::Fixed {
            element: Element::Segment(sides[3]),
        });
    }
    if circle_fixed {
        sketch.add_constraint(Constraint::Fixed {
            element: Element::Circle(circle),
        });
    }
    (sketch, sides[3], circle)
}

/// The rule the constraint tool lays for these picks, in this order.
fn intended(rule: Rule, picks: [Element; 2], sketch: &Sketch) -> Constraint {
    match rule_intent(rule, &picks.map(RulePick::Element), sketch) {
        Some(RuleIntent::Constrain(constraint)) => constraint,
        other => panic!("{rule:?} on {picks:?} lays {other:?}"),
    }
}

/// How far the circle stands off touching the trait itself: off its line, or
/// beside the trait's ends rather than over it.
fn off_touching(sketch: &Sketch, circle: CircleId, segment: SegmentId) -> f64 {
    let (start, end) = sketch.endpoints(segment);
    let drawn = sketch.circle(circle);
    let centre = sketch.point(drawn.center);
    let span = end - start;
    let across = span.perp_dot(centre - start).abs() / span.length();
    let along = (centre - start).dot(span) / span.length_squared();
    let beside = (along - along.clamp(0.0, 1.0)).abs() * span.length();
    (across - drawn.radius).abs().max(beside)
}

/// How far the centre of `circle` stands off the line of `segment`.
fn off_the_line(sketch: &Sketch, circle: CircleId, segment: SegmentId) -> f64 {
    let (start, end) = sketch.endpoints(segment);
    let centre = sketch.point(sketch.circle(circle).center);
    let span = end - start;
    span.perp_dot(centre - start).abs() / span.length()
}

/// How far `circle` stands off touching the line of `segment`.
fn off_its_line(sketch: &Sketch, circle: CircleId, segment: SegmentId) -> f64 {
    (off_the_line(sketch, circle, segment) - sketch.circle(circle).radius).abs()
}

fn where_the_circle_is(sketch: &Sketch, circle: CircleId) -> (DVec2, f64) {
    let drawn = sketch.circle(circle);
    (sketch.point(drawn.center), drawn.radius)
}

fn how_far_it_went(sketch: &Sketch, circle: CircleId, was: (DVec2, f64)) -> f64 {
    let now = where_the_circle_is(sketch, circle);
    now.0.distance(was.0) + (now.1 - was.1).abs()
}

fn side_moved(sketch: &Sketch, side: SegmentId, was: (DVec2, DVec2)) -> f64 {
    let now = sketch.endpoints(side);
    now.0.distance(was.0).max(now.1.distance(was.1))
}

const CLICK_ORDERS: [bool; 2] = [true, false];

#[test]
fn a_fixed_side_stays_and_a_free_circle_made_tangent_to_it_comes_whichever_was_clicked_first() {
    for curve_first in CLICK_ORDERS {
        let (mut sketch, side, circle) = a_side_over_a_circle(true, false);
        let picks = match curve_first {
            true => [Element::Circle(circle), Element::Segment(side)],
            false => [Element::Segment(side), Element::Circle(circle)],
        };
        let tangent = intended(Rule::Tangent, picks, &sketch);
        let before = (sketch.endpoints(side), where_the_circle_is(&sketch, circle));

        let outcome = sketch.lay_rule(tangent, SCALE);

        let case = format!("curve first: {curve_first}");
        assert_eq!(
            outcome,
            LengthOutcome::Exact,
            "{case}: the tangency was refused"
        );
        let slid = side_moved(&sketch, side, before.0);
        assert!(slid < CLOSE, "{case}: the fixed side moved by {slid}");
        let off = off_touching(&sketch, circle, side);
        assert!(
            off < SETTLED,
            "{case}: the circle stands {off} off touching the side"
        );
        assert!(
            how_far_it_went(&sketch, circle, before.1) > CLOSE,
            "{case}: the circle did not come"
        );
    }
}

#[test]
fn a_fixed_circle_stays_and_a_free_side_made_tangent_to_it_comes_whichever_was_clicked_first() {
    for diameter in [None, Some(10.0)] {
        for curve_first in CLICK_ORDERS {
            let (mut sketch, side, circle) = a_side_over_a_circle(false, true);
            if let Some(diameter) = diameter {
                sketch.set_dimension(DimensionTarget::Diameter(circle), diameter, false);
                sketch.resolve(SCALE);
            }
            let picks = match curve_first {
                true => [Element::Circle(circle), Element::Segment(side)],
                false => [Element::Segment(side), Element::Circle(circle)],
            };
            let tangent = intended(Rule::Tangent, picks, &sketch);
            let before = (sketch.endpoints(side), where_the_circle_is(&sketch, circle));

            let outcome = sketch.lay_rule(tangent, SCALE);

            let case = format!("diameter {diameter:?}, curve first: {curve_first}");
            assert_eq!(
                outcome,
                LengthOutcome::Exact,
                "{case}: the tangency was refused"
            );
            let went = where_the_circle_is(&sketch, circle).0.distance(before.1.0);
            assert!(
                went < CLOSE,
                "{case}: the fixed circle's centre moved by {went}"
            );
            let off = off_touching(&sketch, circle, side);
            assert!(
                off < SETTLED,
                "{case}: the circle stands {off} off touching the side"
            );
            assert!(
                side_moved(&sketch, side, before.0) > CLOSE,
                "{case}: the side did not come"
            );
        }
    }
}

#[test]
fn a_tangency_between_a_side_and_a_circle_with_nothing_fixed_still_leaves_the_first_clicked_where_it_was()
 {
    for curve_first in CLICK_ORDERS {
        let (mut sketch, side, circle) = a_side_over_a_circle(false, false);
        let picks = match curve_first {
            true => [Element::Circle(circle), Element::Segment(side)],
            false => [Element::Segment(side), Element::Circle(circle)],
        };
        let tangent = intended(Rule::Tangent, picks, &sketch);
        let before = (sketch.endpoints(side), where_the_circle_is(&sketch, circle));

        let outcome = sketch.lay_rule(tangent, SCALE);

        let case = format!("curve first: {curve_first}");
        assert_eq!(
            outcome,
            LengthOutcome::Exact,
            "{case}: the tangency was refused"
        );
        let (stayed, came) = match curve_first {
            true => (
                how_far_it_went(&sketch, circle, before.1),
                side_moved(&sketch, side, before.0),
            ),
            false => (
                side_moved(&sketch, side, before.0),
                how_far_it_went(&sketch, circle, before.1),
            ),
        };
        assert!(
            stayed < CLOSE,
            "{case}: the first clicked moved by {stayed}"
        );
        assert!(
            came > CLOSE,
            "{case}: the second clicked stayed where it was"
        );
    }
}

/// #1040's second case: a rectangle from (1, 3) to (6.5, 7) whose left side —
/// its fourth trait, from (1, 7) down to (1, 3) — may be fixed; a second,
/// free rectangle from (2, 4) to (3.5, 9), drawn from its top corner as the
/// seed drew it or from its bottom one, which runs its bottom the other way;
/// and a circle made tangent to that bottom, which brings it under the
/// bottom's right end. The circle's centre may be fixed too.
fn a_side_beside_a_circle_touching_a_free_rectangle(
    side_fixed: bool,
    circle_fixed: bool,
    drawn_from_the_top: bool,
) -> (Sketch, SegmentId, CircleId, SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let mut rectangle = |corner: (f64, f64), opposite: (f64, f64)| {
        let corners = [
            corner,
            (opposite.0, corner.1),
            opposite,
            (corner.0, opposite.1),
        ]
        .map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
        [0, 1, 2, 3].map(|index| sketch.add_segment(corners[index], corners[(index + 1) % 4]))
    };
    let fixed_one = rectangle((1.0, 3.0), (6.5, 7.0));
    let (free_one, bottom) = match drawn_from_the_top {
        true => (rectangle((2.0, 9.0), (3.5, 4.0)), 2),
        false => (rectangle((2.0, 4.0), (3.5, 9.0)), 0),
    };
    let centre = sketch.add_point(DVec2::new(9.0, 2.0));
    let circle = sketch.add_circle(centre, 1.0);
    let rim = sketch.add_point(DVec2::new(10.0, 2.0));
    sketch.add_constraint(Support::Circle(circle).holding(rim));
    if side_fixed {
        sketch.add_constraint(Constraint::Fixed {
            element: Element::Segment(fixed_one[3]),
        });
    }
    if circle_fixed {
        sketch.add_constraint(Constraint::Fixed {
            element: Element::Circle(circle),
        });
    }
    let touching = intended(
        Rule::Tangent,
        [Element::Segment(free_one[bottom]), Element::Circle(circle)],
        &sketch,
    );
    assert_eq!(
        sketch.lay_rule(touching, SCALE),
        LengthOutcome::Exact,
        "the circle did not come to touch the free rectangle"
    );
    (sketch, fixed_one[3], circle, free_one[bottom])
}

fn its_centre(sketch: &Sketch, circle: CircleId) -> Element {
    Element::Point(sketch.circle(circle).center)
}

const BOTH_WAYS_ROUND: [bool; 2] = [true, false];

#[test]
fn a_free_circle_s_centre_put_on_a_fixed_side_comes_onto_it_whichever_was_clicked_first() {
    for (drawn_from_the_top, centre_first) in BOTH_WAYS_ROUND
        .into_iter()
        .flat_map(|way| CLICK_ORDERS.map(|order| (way, order)))
    {
        let (mut sketch, side, circle, touched) =
            a_side_beside_a_circle_touching_a_free_rectangle(true, false, drawn_from_the_top);
        let (centre, line) = (its_centre(&sketch, circle), Element::Segment(side));
        let picks = match centre_first {
            true => [centre, line],
            false => [line, centre],
        };
        let on_the_side = intended(Rule::Coincident, picks, &sketch);
        let before = sketch.endpoints(side);

        let outcome = sketch.lay_rule(on_the_side, SCALE);

        let case = format!(
            "free rectangle drawn from the top: {drawn_from_the_top}, centre first: {centre_first}"
        );
        assert_eq!(
            outcome,
            LengthOutcome::Exact,
            "{case}: the rule was refused"
        );
        let turned = side_moved(&sketch, side, before);
        assert!(turned < CLOSE, "{case}: the fixed side moved by {turned}");
        let off = off_the_line(&sketch, circle, side);
        assert!(
            off < SETTLED,
            "{case}: the centre stands {off} off the side"
        );
        let gap = off_its_line(&sketch, circle, touched);
        assert!(
            gap < SETTLED,
            "{case}: the circle no longer touches the rectangle, {gap} off"
        );
    }
}

#[test]
fn a_circle_s_centre_put_on_a_side_with_nothing_fixed_still_lands_touching_the_rectangle() {
    for (drawn_from_the_top, centre_first) in BOTH_WAYS_ROUND
        .into_iter()
        .flat_map(|way| CLICK_ORDERS.map(|order| (way, order)))
    {
        let (mut sketch, side, circle, touched) =
            a_side_beside_a_circle_touching_a_free_rectangle(false, false, drawn_from_the_top);
        let (centre, line) = (its_centre(&sketch, circle), Element::Segment(side));
        let picks = match centre_first {
            true => [centre, line],
            false => [line, centre],
        };
        let on_the_side = intended(Rule::Coincident, picks, &sketch);

        let outcome = sketch.lay_rule(on_the_side, SCALE);

        let case = format!(
            "free rectangle drawn from the top: {drawn_from_the_top}, centre first: {centre_first}"
        );
        assert_eq!(
            outcome,
            LengthOutcome::Exact,
            "{case}: the rule was refused"
        );
        let off = off_the_line(&sketch, circle, side);
        assert!(
            off < SETTLED,
            "{case}: the centre stands {off} off the side"
        );
        let gap = off_its_line(&sketch, circle, touched);
        assert!(
            gap < SETTLED,
            "{case}: the circle no longer touches the rectangle, {gap} off"
        );
    }
}

#[test]
fn a_rule_between_a_fixed_side_and_a_fixed_circle_still_lands_since_nothing_free_can_hold_it() {
    for curve_first in CLICK_ORDERS {
        let (mut sketch, side, circle) = a_side_over_a_circle(true, true);
        let picks = match curve_first {
            true => [Element::Circle(circle), Element::Segment(side)],
            false => [Element::Segment(side), Element::Circle(circle)],
        };
        let tangent = intended(Rule::Tangent, picks, &sketch);

        let outcome = sketch.lay_rule(tangent, SCALE);

        let case = format!("a tangency, curve first: {curve_first}");
        assert_eq!(
            outcome,
            LengthOutcome::Exact,
            "{case}: the rule was refused"
        );
        let off = off_touching(&sketch, circle, side);
        assert!(
            off < SETTLED,
            "{case}: the circle stands {off} off touching the side"
        );
    }
    for (drawn_from_the_top, centre_first) in BOTH_WAYS_ROUND
        .into_iter()
        .flat_map(|way| CLICK_ORDERS.map(|order| (way, order)))
    {
        let (mut sketch, side, circle, _) =
            a_side_beside_a_circle_touching_a_free_rectangle(true, true, drawn_from_the_top);
        let (centre, line) = (its_centre(&sketch, circle), Element::Segment(side));
        let picks = match centre_first {
            true => [centre, line],
            false => [line, centre],
        };
        let on_the_side = intended(Rule::Coincident, picks, &sketch);

        let outcome = sketch.lay_rule(on_the_side, SCALE);

        let case = format!(
            "a centre on a side, free rectangle drawn from the top: {drawn_from_the_top}, centre first: {centre_first}"
        );
        assert_eq!(
            outcome,
            LengthOutcome::Exact,
            "{case}: the rule was refused"
        );
        let off = off_the_line(&sketch, circle, side);
        assert!(
            off < SETTLED,
            "{case}: the centre stands {off} off the side"
        );
    }
}
