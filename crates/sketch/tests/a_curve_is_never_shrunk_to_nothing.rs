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
//!   so is the merge of seed 30002414 at its fifth gesture, two corners of a
//!   rectangle made one whose settling lays the start of a half-turn held on
//!   its side onto the half-turn's centre, the arc touched by neither corner —
//!   `a_merge_whose_settling_lays_an_arc_s_start_on_its_centre_is_refused`;
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
//! - Also, decided by the user on 2026-11-21: "when a « Coïncidence » or a
//!   drop brings an arc's end within the use precision of its centre —
//!   through the centre point itself or through another point standing
//!   there — the arc goes, as #1043 decided for the centre point; and, as
//!   #1064/#1068, it is refused while a value holds its size. Same gesture,
//!   same result whichever point is clicked." An end made one with another
//!   point on the centre, or a hair off it, takes the arc and keeps the rest
//!   of the drawing —
//!   `an_arc_s_end_made_one_with_a_point_a_hair_off_its_centre_takes_the_arc`,
//!   `an_arc_s_end_made_one_with_another_point_standing_on_its_centre_takes_the_arc`;
//!   is refused while the arc's radius is typed —
//!   `an_arc_s_end_made_one_with_another_point_on_its_centre_is_refused_while_its_radius_is_typed`;
//!   and an end dropped on such a point does the same —
//!   `an_arc_s_end_dropped_on_another_point_standing_on_its_centre_takes_the_arc`,
//!   `an_arc_s_end_dropped_on_a_point_a_hair_off_its_centre_takes_the_arc`
//! - Also, surfaced by review: an end dragged within the use precision of its
//!   centre and dropped on no point leaves the arc a radius at both ends —
//!   `an_arc_s_end_dragged_within_the_use_precision_of_its_centre_leaves_the_arc_a_radius`
//! - Also, surfaced by the final review: an arc drawn tiny by its own step is
//!   the user's, and a merge that only carries it — its centre made one with
//!   a point elsewhere, either clicked first — keeps it, as a rule laid
//!   elsewhere does beside it —
//!   `an_arc_drawn_tiny_keeps_itself_when_its_centre_is_made_one_with_a_point_elsewhere`

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

/// A trait to (45, 25) from a point of its own `off` (20, 20), drawn first;
/// then an arc about (20, 20), 10 round, from (30, 20) to (20, 30); and
/// another about (60, 50) that keeps the drawing some 57 across, so a use
/// precision of some 5.7e-6. The arc's centre, start and end, and that point.
fn an_arc_beside_a_point_off_its_centre(off: DVec2) -> (Sketch, [PointId; 3], PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let beside = sketch.add_point(DVec2::new(20.0, 20.0) + off);
    let far = sketch.add_point(DVec2::new(45.0, 25.0));
    sketch.add_segment(beside, far);
    let [centre, start, end] =
        [(20.0, 20.0), (30.0, 20.0), (20.0, 30.0)].map(|at| sketch.add_point(DVec2::from(at)));
    sketch.add_arc(centre, start, end);
    let [other_centre, other_start, other_end] =
        [(60.0, 50.0), (65.0, 50.0), (60.0, 55.0)].map(|at| sketch.add_point(DVec2::from(at)));
    sketch.add_arc(other_centre, other_start, other_end);
    (sketch, [centre, start, end], beside)
}

fn use_precision(sketch: &Sketch) -> f64 {
    cao_sketch::precision::drawing_size(sketch) * cao_sketch::precision::USE
}

/// Whether the first arc of [`an_arc_beside_a_point_off_its_centre`] is
/// gone, and the rest of the drawing — the other arc, the trait and the
/// point the end was brought to — kept where it stood.
fn the_arc_is_taken_and_the_rest_kept(
    sketch: &Sketch,
    was: &Sketch,
    centre: PointId,
    kept: PointId,
) {
    assert!(
        !sketch.live_arcs().any(|(_, arc)| arc.center == centre),
        "the arc is still drawn, its ends {:?} from its centre",
        ends_from_centres(sketch)
    );
    assert_eq!(
        sketch.live_arcs().count(),
        1,
        "the other arc is still drawn"
    );
    assert_eq!(
        sketch.live_segments().count(),
        1,
        "the trait is still drawn"
    );
    for point in sketch
        .live_arcs()
        .flat_map(|(_, arc)| [arc.center, arc.start, arc.end])
        .chain(
            sketch
                .live_segments()
                .flat_map(|(_, side)| [side.start, side.end]),
        )
        .chain([kept])
    {
        assert!(
            sketch.point(point).distance(was.point(point)) < 1e-9,
            "{point:?} moved from {} to {}",
            was.point(point),
            sketch.point(point)
        );
    }
}

fn the_end_made_one_with_a_point_off_its_centre_takes_the_arc(off: DVec2) {
    let (mut sketch, [centre, _, end], beside) = an_arc_beside_a_point_off_its_centre(off);
    assert!(
        sketch.point(beside).distance(sketch.point(centre)) < use_precision(&sketch),
        "the point stands within the use precision of the centre"
    );
    let was = sketch.clone();

    let joined = sketch.join_points(beside, end, 1.0);

    assert!(joined, "the merge was refused");
    the_arc_is_taken_and_the_rest_kept(&sketch, &was, centre, beside);
}

#[test]
fn an_arc_s_end_made_one_with_a_point_a_hair_off_its_centre_takes_the_arc() {
    the_end_made_one_with_a_point_off_its_centre_takes_the_arc(DVec2::new(3e-7, 0.0));
}

#[test]
fn an_arc_s_end_made_one_with_another_point_standing_on_its_centre_takes_the_arc() {
    the_end_made_one_with_a_point_off_its_centre_takes_the_arc(DVec2::ZERO);
}

#[test]
fn an_arc_s_end_made_one_with_another_point_on_its_centre_is_refused_while_its_radius_is_typed() {
    let (mut sketch, [centre, start, end], beside) =
        an_arc_beside_a_point_off_its_centre(DVec2::ZERO);
    let (arc, _) = sketch
        .live_arcs()
        .find(|(_, arc)| arc.center == centre)
        .expect("the arc is drawn");
    sketch.set_dimension(DimensionTarget::ArcRadius(arc), 10.0, false);
    let was = sketch.clone();

    let joined = sketch.join_points(beside, end, 1.0);

    assert!(
        !joined,
        "the end was made one with the point: the start now {} from the centre, the end {}",
        sketch.point(start).distance(sketch.point(centre)),
        sketch.point(end).distance(sketch.point(centre))
    );
    assert!(!sketch.is_erased_arc(arc), "the arc is still drawn");
    assert!(
        sketch
            .points()
            .iter()
            .zip(was.points())
            .all(|(now, then)| now.distance(*then) < 1e-9),
        "nothing about the drawing moved"
    );
}

/// The arc's end pressed, carried onto the point `off` its centre and let go
/// of, as the drop decides and `cao_part` replays it: settled there, then
/// joined to the point when the drop joins it.
fn the_end_dropped_on_a_point_off_its_centre_takes_the_arc(off: DVec2) {
    let (mut sketch, [centre, _, end], beside) = an_arc_beside_a_point_off_its_centre(off);
    let was = sketch.clone();
    let landing = sketch.point(beside);
    let pull = sketch.pull(end, 1.0);
    let mut settling = sketch.clone();
    settling.settle_pulled(&pull, landing, 1.0);

    let merged_into = pull.joins(&settling, landing, 1e-6, 1.0);
    sketch.settle_pulled(&pull, landing, 1.0);
    if let Some(kept) = merged_into {
        sketch.join_points(kept, end, 1.0);
    }

    assert!(
        merged_into.is_some(),
        "the drop joins nothing, the end left {} from the centre",
        settling.point(end).distance(settling.point(centre))
    );
    the_arc_is_taken_and_the_rest_kept(&sketch, &was, centre, beside);
}

#[test]
fn an_arc_s_end_dropped_on_another_point_standing_on_its_centre_takes_the_arc() {
    the_end_dropped_on_a_point_off_its_centre_takes_the_arc(DVec2::ZERO);
}

#[test]
fn an_arc_s_end_dropped_on_a_point_a_hair_off_its_centre_takes_the_arc() {
    the_end_dropped_on_a_point_off_its_centre_takes_the_arc(DVec2::new(1e-11, 0.0));
}

#[test]
fn an_arc_s_end_dragged_within_the_use_precision_of_its_centre_leaves_the_arc_a_radius() {
    let (mut sketch, [centre, start, end], _) = an_arc_beside_a_point_off_its_centre(DVec2::ZERO);
    let landing = sketch.point(centre) + DVec2::new(0.0, 3e-7);
    let pull = sketch.pull(end, 1.0);

    sketch.settle_pulled(&pull, landing, 1.0);

    let reach = |point: PointId| sketch.point(point).distance(sketch.point(centre));
    assert!(
        reach(start) > use_precision(&sketch) && reach(end) > use_precision(&sketch),
        "the arc is brought under the use precision {}: its start {} from the centre, its end {}",
        use_precision(&sketch),
        reach(start),
        reach(end)
    );
}

/// A trait 1000 long, from (10, 10) to (810, 610), so a use precision of some
/// 1e-4; an arc of radius 1e-5 about (500, 100), drawn so by its own step;
/// and a short trait from (600, 100). The arc, its centre, and that trait's
/// end at (600, 100).
fn an_arc_drawn_tiny_beside_a_trait_end() -> (Sketch, cao_sketch::ArcId, PointId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    for (from, to) in [
        ((10.0, 10.0), (810.0, 610.0)),
        ((600.0, 100.0), (650.0, 150.0)),
    ] {
        let from = sketch.add_point(DVec2::from(from));
        let to = sketch.add_point(DVec2::from(to));
        sketch.add_segment(from, to);
    }
    let end = sketch
        .nearest_point(DVec2::new(600.0, 100.0), 1e-9)
        .expect("the trait's end");
    let [centre, start, finish] = [(500.0, 100.0), (500.00001, 100.0), (500.0, 100.00001)]
        .map(|at| sketch.add_point(DVec2::from(at)));
    let arc = sketch.add_arc(centre, start, finish);
    (sketch, arc, centre, end)
}

#[test]
fn an_arc_drawn_tiny_keeps_itself_when_its_centre_is_made_one_with_a_point_elsewhere() {
    for centre_kept in [true, false] {
        let (mut sketch, arc, centre, end) = an_arc_drawn_tiny_beside_a_trait_end();
        assert!(
            sketch.arc_radius(arc) < use_precision(&sketch),
            "the arc is drawn under the use precision"
        );
        let (kept, dropped) = match centre_kept {
            true => (centre, end),
            false => (end, centre),
        };

        let joined = sketch.join_points(kept, dropped, 1.0);

        let when = format!(
            "the centre clicked {}",
            if centre_kept { "first" } else { "second" }
        );
        assert!(joined, "{when}: the merge was refused");
        assert!(!sketch.is_erased_arc(arc), "{when}: the arc is gone");
        let ends = ends_from_centres(&sketch);
        assert!(
            ends.iter()
                .any(|(start, end)| (start - 1e-5).abs() < 1e-9 && (end - 1e-5).abs() < 1e-9),
            "{when}: the arc keeps its radius of 1e-5 at both ends, got {ends:?}"
        );
    }
}

/// Seed 30002414, mode `shares` 0,80,0, up to its fifth gesture: three
/// rectangles, a construction half-turn about the first one's corner at
/// (7, 5) whose start is held on that rectangle's left side, and the
/// « Coïncidence » of the first rectangle's two diagonal corners at (10, 5)
/// and (7, 9).
fn a_half_turn_on_a_rectangle_folded_corner_to_corner() -> Vec<Gesture> {
    vec![
        Gesture::Rectangle {
            corner: [7.0, 5.0],
            opposite: [10.0, 9.0],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [2.0, 0.0],
            opposite: [1.0, -4.5],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [3.0, 8.0],
            opposite: [-1.5, 11.5],
            construction: false,
        },
        Gesture::Arc {
            centre: [7.0, 5.0],
            start: [7.0, 6.0],
            degrees: 180.0,
            construction: true,
        },
        Gesture::Rule {
            rule: Rule::Coincident,
            clicks: vec![[10.0, 5.0], [7.0, 9.0]],
        },
    ]
}

#[test]
fn a_merge_whose_settling_lays_an_arc_s_start_on_its_centre_is_refused() {
    let gestures = a_half_turn_on_a_rectangle_folded_corner_to_corner();
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
        panic!("the tool does not make the two corners one");
    };
    let was = sketch.clone();

    let joined = sketch.join_points(kept, dropped, 1.0);

    assert!(
        !joined,
        "the two corners were made one, the arc's ends now {:?} from its centre, {:?} before",
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
}
