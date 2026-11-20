//! A rule, a merge or a drag never leaves a circle or an arc shrunk to
//! nothing, an arc at either end: the rule or the merge is refused with the
//! drawing as it was, and the drag stops short.
//!
//! Closes #1072.
//! - The decision is written here — no test: it is the issue's comment of
//!   2026-11-20, one guard after every solve
//! - The five cases above hold in #501's campaign: the four rules and merges
//!   are refused with the drawing as it was; the campaign holds them in
//!   `what_random_sketches_found.rs`. Here, the merge seeds 30005683 and
//!   31000310 make at their sixth gesture — an arc's end made one with its
//!   own centre, which brings its mirror's end onto the mirror's centre — is
//!   refused, every arc keeping both ends off its centre —
//!   `an_arc_and_its_mirror_refuse_an_end_made_one_with_its_centre`,
//!   `two_mirrored_arcs_tied_end_to_end_refuse_an_end_made_one_with_its_centre`;
//!   the solve's tail the guard joins is pinned as it stands —
//!   `a_trait_a_value_pulls_under_the_use_precision_leaves_the_solve_residual`,
//!   `a_trait_a_value_pulls_just_above_the_use_precision_is_solved`;
//!   and a circle or an arc a value brings to a radius small but well above
//!   the use precision still solves —
//!   `a_circle_given_a_small_radius_well_above_the_use_precision_is_solved`,
//!   `an_arc_given_a_small_radius_well_above_the_use_precision_is_solved`
//! - The five cases above hold in #501's campaign: the drag of seed 31000310
//!   stops short, the arcs keeping a radius. Once the sixth gesture is
//!   refused the drag presses on nothing; what is held is that both arcs come
//!   through the whole seed with their ends off their centres —
//!   `two_mirrored_arcs_tied_end_to_end_keep_their_ends_off_their_centres_through_the_seed`;
//!   the campaign holds the seed whole in `what_random_sketches_found.rs`
//! - The 88 seeds no longer shrink a curve to nothing — no test: it is a
//!   measurement over #501's campaign once the guard is in, not a test the
//!   gate can hold
//! - Also: an arc's end made one with its own centre, with no mirror to
//!   bring down, still takes the arc as #1043 and #1063 decided — no test:
//!   not in this file; `crates/part/tests/two_points_made_one.rs` holds it as
//!   `an_arc_end_made_one_with_its_own_centre_takes_the_arc`, and
//!   `crates/sketch/src/sketch/merging/tests.rs` at the sketch's level
//! - Also: an arc's start dragged and dropped on its own centre is that same
//!   merge, and takes the arc away rather than stopping a hair short of the
//!   centre — laid here as the drop decides and `cao_part` replays it —
//!   `an_arc_s_start_dropped_on_its_own_centre_takes_the_arc_as_the_merge_does`;
//!   the application's drop holds it under the same name in
//!   `crates/app/src/screens/viewport/input/dragging/tests.rs`

#[allow(dead_code, unused_imports)]
mod random_sketches;

use cao_sketch::{
    DimensionTarget, Element, Rule, RuleIntent, RulePick, SegmentId, SnapSettings, SolveOutcome,
    WorkPlane, rule_intent,
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

/// How far each end of every arc stands from its centre.
fn ends_from_centres(sketch: &Sketch) -> Vec<(f64, f64)> {
    sketch
        .live_arcs()
        .map(|(_, arc)| {
            let centre = sketch.point(arc.center);
            (
                centre.distance(sketch.point(arc.start)),
                centre.distance(sketch.point(arc.end)),
            )
        })
        .collect()
}

fn every_arc_keeps_its_ends_off_its_centre(sketch: &Sketch) {
    for (start, end) in ends_from_centres(sketch) {
        assert!(
            start > 1.0 && end > 1.0,
            "an arc's start stands {start} from its centre and its end {end}"
        );
    }
}

/// The last gesture of `gestures`, « Coïncidence » clicked on two points, made
/// as the tool makes it after the others are laid: refused, nothing of the
/// drawing moved, and every arc's ends where they were.
fn the_last_merge_is_refused(gestures: &[Gesture]) {
    let (Gesture::Rule { clicks, .. }, before) = gestures.split_last().expect("a gesture") else {
        panic!("the last gesture is not a rule");
    };
    let mut sketch = laid(before);
    let picks: Vec<RulePick> = clicks
        .iter()
        .map(|place| RulePick::Element(Element::Point(standing_at(&sketch, *place))))
        .collect();
    let Some(RuleIntent::Merge { kept, dropped }) = rule_intent(Rule::Coincident, &picks, &sketch)
    else {
        panic!("the tool does not make the two points one");
    };
    let was = sketch.clone();

    let joined = sketch.join_points(kept, dropped, 1.0);

    assert!(
        !joined,
        "the two points were made one, each arc's ends now {:?} from its centre, {:?} before",
        ends_from_centres(&sketch),
        ends_from_centres(&was)
    );
    assert!(
        sketch
            .points()
            .iter()
            .zip(was.points())
            .all(|(now, then)| now.distance(*then) < 1e-9),
        "nothing about the drawing moved"
    );
    assert_eq!(ends_from_centres(&sketch), ends_from_centres(&was));
    every_arc_keeps_its_ends_off_its_centre(&sketch);
}

/// Seed 30005683 up to its sixth gesture: an arc and its mirror across U, one
/// end of the first made one with where the original arc's centre stood, then
/// its other end made one with its own centre.
fn an_arc_and_its_mirror_tied() -> Vec<Gesture> {
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
fn an_arc_and_its_mirror_refuse_an_end_made_one_with_its_centre() {
    the_last_merge_is_refused(&an_arc_and_its_mirror_tied());
}

/// Seed 31000310 up to its sixth gesture: an arc and its mirror across U,
/// tied end to end at both ends, then an end of the first made one with its
/// own centre.
fn two_mirrored_arcs_tied_end_to_end() -> Vec<Gesture> {
    vec![
        Gesture::Arc {
            centre: [5.0, 6.0],
            start: [8.5, 6.0],
            degrees: 135.0,
            construction: false,
        },
        Gesture::Mirror {
            of: vec![[6.3393920132778145, 9.233578363789503]; 3],
            axis: Axis::U,
        },
        Gesture::Rule {
            rule: Rule::Coincident,
            clicks: vec![[8.5, -6.0], [8.5, 6.0]],
        },
        Gesture::Drag {
            from: [2.069273733843902, 8.932493948246039],
            to: [2.069273733843902, 10.066230394236461],
        },
        Gesture::Rule {
            rule: Rule::Coincident,
            clicks: vec![[0.0, 0.0], [2.5837811533561186, -8.932493948245206]],
        },
        Gesture::Rule {
            rule: Rule::Coincident,
            clicks: vec![
                [9.572769548800967, 5.4085440256488434e-12],
                [4.786384774413328, 5.236530304488603],
            ],
        },
    ]
}

#[test]
fn two_mirrored_arcs_tied_end_to_end_refuse_an_end_made_one_with_its_centre() {
    the_last_merge_is_refused(&two_mirrored_arcs_tied_end_to_end());
}

#[test]
fn two_mirrored_arcs_tied_end_to_end_keep_their_ends_off_their_centres_through_the_seed() {
    let mut seed = two_mirrored_arcs_tied_end_to_end();
    seed.push(Gesture::Drag {
        from: [16.341739811492303, -6.768970262700572],
        to: [16.341739811492303, -20.306910788087073],
    });

    let sketch = laid(&seed);

    assert_eq!(
        ends_from_centres(&sketch).len(),
        2,
        "both arcs are still drawn: {:?}",
        ends_from_centres(&sketch)
    );
    every_arc_keeps_its_ends_off_its_centre(&sketch);
}

/// Two traits, from (10, 10) to (60, 10) and from (10, 40) to (90, 40), that
/// keep the drawing about 85 across: a use precision of some 8.5e-6.
fn two_traits() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    for (start, end) in [((10.0, 10.0), (60.0, 10.0)), ((10.0, 40.0), (90.0, 40.0))] {
        let start = sketch.add_point(DVec2::from(start));
        let end = sketch.add_point(DVec2::from(end));
        sketch.add_segment(start, end);
    }
    sketch
}

/// The first of [`two_traits`] given a length of `value`.
fn a_trait_given_a_length(value: f64) -> Sketch {
    let mut sketch = two_traits();
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

#[test]
fn a_circle_given_a_small_radius_well_above_the_use_precision_is_solved() {
    let mut sketch = two_traits();
    let centre = sketch.add_point(DVec2::new(50.0, 25.0));
    let circle = sketch.add_circle(centre, 5.0);
    sketch.set_dimension(DimensionTarget::Radius(circle), 1e-4, false);

    assert_eq!(sketch.solve(1.0), SolveOutcome::Solved);
    assert!(
        (sketch.circle(circle).radius - 1e-4).abs() < 1e-7,
        "the circle is 1e-4 round, got {}",
        sketch.circle(circle).radius
    );
}

#[test]
fn an_arc_given_a_small_radius_well_above_the_use_precision_is_solved() {
    let mut sketch = two_traits();
    let [centre, start, end] =
        [(50.0, 25.0), (55.0, 25.0), (50.0, 30.0)].map(|at| sketch.add_point(DVec2::from(at)));
    let arc = sketch.add_arc(centre, start, end);
    sketch.set_dimension(DimensionTarget::ArcRadius(arc), 1e-4, false);

    assert_eq!(sketch.solve(1.0), SolveOutcome::Solved);
    for (from_start, from_end) in ends_from_centres(&sketch) {
        assert!(
            (from_start - 1e-4).abs() < 1e-7 && (from_end - 1e-4).abs() < 1e-7,
            "the arc is 1e-4 round at both ends, got {from_start} and {from_end}"
        );
    }
}

const NO_MAGNET: SnapSettings = SnapSettings {
    point_reach: 0.0,
    curve_reach: 0.0,
    grid_step: None,
    grid_reach: 0.0,
};

#[test]
fn an_arc_s_start_dropped_on_its_own_centre_takes_the_arc_as_the_merge_does() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let [centre, start, end] =
        [(20.0, 20.0), (30.0, 20.0), (20.0, 30.0)].map(|at| sketch.add_point(DVec2::from(at)));
    sketch.add_arc(centre, start, end);
    let hand = sketch.point(centre);
    let pull = sketch.pull(start, 1.0);
    let landing = pull.landing(&sketch, hand, hand, false, &NO_MAGNET);
    let mut settling = sketch.clone();
    settling.settle_pulled(&pull, landing, 1.0);

    let merged_into = pull.joins(&settling, landing, 1e-6, 1.0);
    sketch.settle_pulled(&pull, landing, 1.0);
    if let Some(kept) = merged_into {
        sketch.join_points(kept, start, 1.0);
    }

    assert_eq!(
        merged_into,
        Some(centre),
        "the drop joins nothing, the start left {} from the centre",
        settling.point(start).distance(settling.point(centre))
    );
    assert!(
        ends_from_centres(&sketch).is_empty(),
        "the arc is still drawn, its ends {:?} from its centre",
        ends_from_centres(&sketch)
    );
}
