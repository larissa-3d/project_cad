//! A rule, a merge or a drag never leaves a circle or an arc with its radius
//! shrunk to nothing: the rule or the merge is refused with the drawing as it
//! was, and the drag stops short.
//!
//! Closes #1072.
//! - The decision is written here — no test: it is the issue's comment of
//!   2026-11-20, one guard after every solve
//! - The five cases above hold in #501's campaign: the four rules and merges
//!   are refused with the drawing as it was — no test: not in this file; the
//!   campaign holds them in `what_random_sketches_found.rs`, and `cao_part`
//!   refuses the tangencies in `crates/part/tests/an_impossible_rule_is_refused.rs`.
//!   Here the merge of seed 30005683 is held at the sketch's level, and the
//!   solve's tail the guard joins is pinned as it stands —
//!   `an_arc_s_end_made_one_with_its_mirror_s_is_refused_and_keeps_its_radius`,
//!   `a_trait_a_value_pulls_under_the_use_precision_leaves_the_solve_residual`,
//!   `a_trait_a_value_pulls_just_above_the_use_precision_is_solved`
//! - The five cases above hold in #501's campaign: the drag of seed 31000310
//!   stops short, the arcs keeping a radius — no test yet: the next round
//!   writes it
//! - The 88 seeds no longer shrink a curve to nothing — no test: it is a
//!   measurement over #501's campaign once the guard is in, not a test the
//!   gate can hold

#[allow(dead_code, unused_imports)]
mod random_sketches;

use cao_sketch::{
    DimensionTarget, Element, Rule, RuleIntent, RulePick, SegmentId, SolveOutcome, WorkPlane,
    rule_intent,
};
use cao_sketch::{PointId, Sketch};
use glam::DVec2;
use random_sketches::{Axis, Gesture, laid};

fn standing_at(sketch: &Sketch, place: [f64; 2]) -> PointId {
    sketch
        .live_points()
        .find(|(_, at)| at.distance(DVec2::from(place)) < 1e-6)
        .map(|(point, _)| point)
        .unwrap_or_else(|| panic!("no point stands at {place:?}"))
}

/// Seed 30005683 up to its last gesture: an arc and its mirror across U, an
/// end of each made one with the other's centre and curve.
fn an_arc_tied_to_its_mirror() -> Vec<Gesture> {
    vec![
        Gesture::Arc {
            centre: [45.0, 0.0],
            start: [45.0, -135.0],
            degrees: 90.0,
            construction: false,
        },
        Gesture::Trim {
            at: [140.45941546018392, -95.4594154601839],
        },
        Gesture::Arc {
            centre: [60.0, 90.0],
            start: [60.0, 75.0],
            degrees: 300.0,
            construction: false,
        },
        Gesture::Mirror {
            of: vec![[67.5, 102.99038105676658]],
            axis: Axis::U,
        },
        Gesture::Rule {
            rule: Rule::Coincident,
            clicks: vec![[47.00961894323342, 82.5], [45.0, 0.0]],
        },
        Gesture::Rule {
            rule: Rule::Coincident,
            clicks: vec![
                [44.09207696395175, 89.23179435564785],
                [71.9589620358558, 4.185344086103531],
            ],
        },
    ]
}

#[test]
fn an_arc_s_end_made_one_with_its_mirror_s_is_refused_and_keeps_its_radius() {
    let mut sketch = laid(&an_arc_tied_to_its_mirror());
    let (arc, _) = sketch.live_arcs().next().expect("an arc is left standing");
    let radius = sketch.arc_radius(arc);
    let picks = [
        [44.09207696399786, -89.23179435565704],
        [44.09207696395175, 89.23179435564785],
    ]
    .map(|place| RulePick::Element(Element::Point(standing_at(&sketch, place))));
    let Some(RuleIntent::Merge { kept, dropped }) = rule_intent(Rule::Coincident, &picks, &sketch)
    else {
        panic!("the tool does not make the two ends one");
    };
    let before = sketch.clone();

    let joined = sketch.join_points(kept, dropped, 1.0);

    assert!(
        !joined,
        "the two ends were made one, the arc's radius {radius} now {}",
        sketch.arc_radius(arc)
    );
    assert!(
        (sketch.arc_radius(arc) - radius).abs() < 1e-9,
        "the arc's radius stays {radius}, got {}",
        sketch.arc_radius(arc)
    );
    assert!(
        sketch
            .points()
            .iter()
            .zip(before.points())
            .all(|(now, was)| now.distance(*was) < 1e-9),
        "nothing about the drawing moved"
    );
}

/// A trait from (10, 10) to (60, 10) given a length of `value`, beside a trait
/// from (10, 40) to (90, 40) that keeps the drawing about 85 across: a use
/// precision of some 8.5e-6.
fn a_trait_given_a_length(value: f64) -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    for (start, end) in [((10.0, 10.0), (60.0, 10.0)), ((10.0, 40.0), (90.0, 40.0))] {
        let start = sketch.add_point(DVec2::from(start));
        let end = sketch.add_point(DVec2::from(end));
        sketch.add_segment(start, end);
    }
    sketch.set_dimension(DimensionTarget::Length(SegmentId(0)), value, false);
    sketch
}

#[test]
fn a_trait_a_value_pulls_under_the_use_precision_leaves_the_solve_residual() {
    let mut sketch = a_trait_given_a_length(1e-6);

    assert_eq!(sketch.solve(1.0), SolveOutcome::Residual);
    let (start, end) = sketch.endpoints(SegmentId(0));
    assert!(
        (start.distance(end) - 1e-6).abs() < 1e-7,
        "the value was reached, the trait 1e-6 long, got {}",
        start.distance(end)
    );
}

#[test]
fn a_trait_a_value_pulls_just_above_the_use_precision_is_solved() {
    let mut sketch = a_trait_given_a_length(1e-4);

    assert_eq!(sketch.solve(1.0), SolveOutcome::Solved);
    let (start, end) = sketch.endpoints(SegmentId(0));
    assert!(
        (start.distance(end) - 1e-4).abs() < 1e-7,
        "the trait is 1e-4 long, got {}",
        start.distance(end)
    );
}
