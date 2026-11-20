//! Whether something closes around a place, reckoned from the curves of a
//! drawing alone — what the campaign holds the areas against, and so what
//! must never be read from the walk it is checking.
//!
//! Closes #500.
//! - whether a place is enclosed is never read from the walk under test: it
//!   comes from a reckoning of its own, which cuts the curves where they cross,
//!   touch or end on one another and asks whether a loop of them winds round
//!   the place — `four_traits_crossing_like_a_grid_enclose_the_square_between_them`,
//!   `three_circles_each_touching_the_other_two_enclose_the_gap_between_them`,
//!   `two_points_standing_in_one_place_are_one_place_for_what_closes`
//! - an area is closed, or it is no area: a shape missing its last stretch by
//!   a hair encloses nothing — `a_square_a_hair_short_of_closing_encloses_nothing`,
//!   `three_circles_a_hair_apart_leave_the_gap_between_them_open`
//!
//! Closes #509.
//! - a campaign has run with it, and what it found is written down — the
//!   reckoning's own false alarm, a side through an ellipse's centre read as
//!   a touch and cut once, is held here —
//!   `a_side_through_an_ellipse_s_centre_crosses_it_twice`,
//!   `a_side_through_a_turned_ellipse_s_centre_crosses_it_twice`
//!
//! Closes #1070.
//! - two traits lying along each other within one place meet at the ends of
//!   each that lie on the other, as they do when they lie along each other
//!   exactly — the chain seed 31003453 settles into, its three traits a few
//!   1e-10 apart, closes nothing, as it does laid flat —
//!   `a_chain_folded_back_along_itself_closes_nothing_when_its_traits_lie_within_one_place_of_each_other`,
//!   `a_chain_folded_back_along_itself_closes_nothing_when_its_traits_lie_along_each_other_exactly`;
//!   and a trait only beside another, its ends within one place of the
//!   other's line far out but the other standing farther than one place from
//!   it, meets it nowhere —
//!   `a_long_trait_beside_a_short_one_farther_than_one_place_from_it_closes_nothing`
//! - seeds 31003453 and 30000037 hold — no test: not in this file;
//!   `what_random_sketches_found.rs` holds the two shrunk cases
//! - the reckoning's own tests in `every_sketch_keeps_its_areas.rs` and
//!   `curves_lying_along_each_other.rs` do not break — no test: they are those
//!   files' own, run in the same gate. The issue also names
//!   `two_shapes_lying_along_each_other`, which no test or file of the
//!   repository is called
//! - a crossing between two traits only nearly parallel, which do not lie
//!   along each other, is still found once —
//!   `two_traits_nearly_parallel_crossing_in_their_middle_are_cut_once_where_they_cross`,
//!   down to the finest lean the reckoning can tell from lying along —
//!   `two_traits_whose_ends_stand_just_beyond_the_doubt_of_lying_along_each_other_are_cut_once_where_they_cross`
//!
//! Closes #1071.
//! - two closed curves that are the same curve within one place close one
//!   loop between them, as two edges laid along each other already do in
//!   `areas_closed` — a circle and a round ellipse of the same centre and
//!   radius, as seed 30002413 leaves them, and an ellipse and its mirror that
//!   is the same ellipse, as seed 30001705 leaves them —
//!   `a_circle_and_a_round_ellipse_lying_on_it_within_one_place_close_one_area`,
//!   `an_ellipse_and_its_mirror_lying_on_it_within_one_place_close_one_area`;
//!   and two closed curves a hair apart beyond one place are a real sliver,
//!   which keeps its two loops —
//!   `a_circle_and_a_round_ellipse_a_hair_apart_beyond_one_place_close_two_areas`
//! - seeds 30002413 and 30001705 hold — no test: not in this file;
//!   `what_random_sketches_found.rs` holds the two shrunk cases

// The drawing, the rules and the campaign are the campaign file's; this one
// only holds the reckoning to drawings whose answer is known.
#[allow(dead_code, unused_imports)]
mod random_sketches;

use cao_sketch::{EllipseDraft, EllipseMode, Sketch, WorkPlane, ellipse_from, rise_of};
use glam::DVec2;
use random_sketches::areas::areas_of;
use random_sketches::checking::carried;
use random_sketches::enclosing::{Curve, Enclosure, curves_of};
use random_sketches::{Gesture, Relaying, as_many_as_closed, laid};

fn straight(from: [f64; 2], to: [f64; 2]) -> Curve {
    Curve::Straight {
        from: DVec2::from(from),
        to: DVec2::from(to),
    }
}

fn chain(through: &[[f64; 2]]) -> Vec<Curve> {
    through
        .windows(2)
        .map(|pair| straight(pair[0], pair[1]))
        .collect()
}

fn circle(centre: [f64; 2], radius: f64) -> Curve {
    Curve::Round {
        centre: DVec2::from(centre),
        radius,
        from: 0.0,
        sweep: std::f64::consts::TAU,
    }
}

fn ellipse(centre: [f64; 2], first: [f64; 2], second: f64) -> Curve {
    Curve::Oval {
        drawn: EllipseDraft {
            centre: DVec2::from(centre),
            first: DVec2::from(first),
            second,
        },
        from: 0.0,
        sweep: std::f64::consts::TAU,
    }
}

fn enclosed(curves: &[Curve], at: [f64; 2]) -> bool {
    Enclosure::of(curves)
        .encloses(DVec2::from(at))
        .expect("a place the reckoning can tell")
}

/// Five times what the drawing calls one place, at a drawing reaching `reach`
/// from the origin: the finest miss a campaign draws.
fn hair(reach: f64) -> f64 {
    5e-7 * (1.0 + reach)
}

#[test]
fn a_square_encloses_its_middle_and_nothing_outside_it() {
    let square = chain(&[[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0], [0.0, 0.0]]);
    assert!(enclosed(&square, [1.0, 1.0]));
    assert!(enclosed(&square, [0.1, 1.9]));
    assert!(!enclosed(&square, [3.0, 1.0]));
    assert!(!enclosed(&square, [-0.5, -0.5]));
}

#[test]
fn a_square_a_hair_short_of_closing_encloses_nothing() {
    let short = chain(&[
        [0.0, 0.0],
        [2.0, 0.0],
        [2.0, 2.0],
        [0.0, 2.0],
        [0.0, hair(2.0)],
    ]);
    assert!(!enclosed(&short, [1.0, 1.0]));
}

#[test]
fn a_square_closed_closer_than_one_place_encloses_its_middle() {
    let shut = chain(&[[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0], [0.0, 1e-8]]);
    assert!(enclosed(&shut, [1.0, 1.0]));
}

#[test]
fn a_trait_ending_between_one_and_five_places_from_a_curve_cannot_be_told() {
    let mut drawn = chain(&[[1.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]]);
    let near = 1.5e-7;
    drawn.push(straight([0.0, 3.0], [0.0, near]));
    drawn.push(straight([0.0, 0.0], [1.0, 0.0]));
    assert_eq!(
        Enclosure::of(&drawn).encloses(DVec2::new(2.0, 1.5)),
        None,
        "an end standing where the walk and the reckoning may each read a touch"
    );
}

#[test]
fn a_trait_poking_into_a_square_encloses_nothing_more() {
    let mut drawn = chain(&[[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0], [0.0, 0.0]]);
    drawn.push(straight([3.0, 1.0], [1.0, 1.0]));
    assert!(enclosed(&drawn, [1.5, 1.5]));
    assert!(enclosed(&drawn, [0.5, 1.0]));
    assert!(!enclosed(&drawn, [2.5, 1.5]));
}

#[test]
fn four_traits_crossing_like_a_grid_enclose_the_square_between_them() {
    let grid = [
        straight([0.0, 1.0], [3.0, 1.0]),
        straight([0.0, 2.0], [3.0, 2.0]),
        straight([1.0, 0.0], [1.0, 3.0]),
        straight([2.0, 0.0], [2.0, 3.0]),
    ];
    assert!(enclosed(&grid, [1.5, 1.5]));
    assert!(!enclosed(&grid, [0.5, 0.5]));
    assert!(!enclosed(&grid, [1.5, 0.5]));
}

#[test]
fn a_trait_ending_on_the_side_of_another_closes_what_they_bound() {
    let drawn = [
        straight([0.0, 0.0], [4.0, 0.0]),
        straight([4.0, 0.0], [4.0, 2.0]),
        straight([4.0, 2.0], [2.0, 2.0]),
        straight([2.0, 2.0], [2.0, 0.0]),
    ];
    assert!(enclosed(&drawn, [3.0, 1.0]));
    assert!(!enclosed(&drawn, [1.0, 1.0]));
}

#[test]
fn a_trait_laid_along_the_end_of_another_closes_what_they_bound() {
    let mut drawn = chain(&[[2.0, 2.0], [0.0, 2.0], [0.0, 0.0], [2.0, 0.0], [2.0, 1.0]]);
    drawn.push(straight([2.0, 0.5], [2.0, 2.5]));
    assert!(enclosed(&drawn, [1.0, 1.0]));

    drawn.pop();
    drawn.push(straight([2.0, 1.0 + hair(2.5)], [2.0, 2.5]));
    assert!(!enclosed(&drawn, [1.0, 1.0]));
}

#[test]
fn a_circle_encloses_its_disc_and_an_arc_alone_encloses_nothing() {
    assert!(enclosed(&[circle([0.0, 0.0], 2.0)], [0.5, -1.0]));
    assert!(!enclosed(&[circle([0.0, 0.0], 2.0)], [2.5, 0.0]));
    let arc = Curve::Round {
        centre: DVec2::ZERO,
        radius: 2.0,
        from: 0.0,
        sweep: 5.0,
    };
    assert!(!enclosed(&[arc], [0.5, 0.0]));
}

#[test]
fn an_arc_closed_by_its_chord_encloses_what_lies_between_them() {
    let arc = Curve::Round {
        centre: DVec2::ZERO,
        radius: 1.0,
        from: 0.0,
        sweep: 1.5 * std::f64::consts::PI,
    };
    let drawn = [arc, straight([0.0, -1.0], [1.0, 0.0])];
    assert!(enclosed(&drawn, [-0.5, 0.0]));
    assert!(enclosed(&drawn, [0.1, 0.1]));
    assert!(!enclosed(&drawn, [0.6, -0.6]));
}

#[test]
fn a_trait_touching_a_circle_closes_the_corner_between_them() {
    for radius in [1.0, 1.0 - 1e-9] {
        let mut drawn = vec![circle([0.0, 0.0], radius)];
        drawn.extend(chain(&[[-2.0, 1.0], [2.0, 1.0], [2.0, 0.0], [radius, 0.0]]));
        assert!(enclosed(&drawn, [1.5, 0.5]), "radius {radius}");
    }

    let mut apart = vec![circle([0.0, 0.0], 1.0 - hair(2.0))];
    apart.extend(chain(&[
        [-2.0, 1.0],
        [2.0, 1.0],
        [2.0, 0.0],
        [1.0 - hair(2.0), 0.0],
    ]));
    assert!(!enclosed(&apart, [1.5, 0.5]));
}

#[test]
fn a_trait_touching_an_ellipse_closes_the_corner_between_them() {
    let mut drawn = vec![ellipse([0.0, 0.0], [2.0, 0.0], 1.0)];
    drawn.extend(chain(&[[-3.0, 1.0], [3.0, 1.0], [3.0, 0.0], [2.0, 0.0]]));
    assert!(enclosed(&drawn, [2.5, 0.5]));
    assert!(!enclosed(&drawn, [-2.5, 0.5]));
}

#[test]
fn a_circle_touching_an_ellipse_closes_the_gap_a_trait_bounds_above_them() {
    let turned = Curve::Oval {
        drawn: EllipseDraft {
            centre: DVec2::ZERO,
            first: DVec2::new(2.0, 0.0),
            second: 1.0,
        },
        from: 0.3,
        sweep: std::f64::consts::TAU,
    };
    for radius in [1.0, 1.0 - 1e-9] {
        let drawn = [
            turned,
            circle([3.0, 0.0], radius),
            straight([0.0, 0.8], [4.0, 0.8]),
        ];
        assert!(enclosed(&drawn, [2.0, 0.6]), "radius {radius}");
        assert!(!enclosed(&drawn, [2.0, -0.6]), "radius {radius}");
    }

    let apart = [
        ellipse([0.0, 0.0], [2.0, 0.0], 1.0),
        circle([3.0, 0.0], 1.0 - hair(4.0)),
        straight([0.0, 0.8], [4.0, 0.8]),
    ];
    assert!(!enclosed(&apart, [2.0, 0.6]));
}

#[test]
fn an_arc_crossing_half_an_ellipse_encloses_the_lens_between_them() {
    let half = Curve::Oval {
        drawn: EllipseDraft {
            centre: DVec2::ZERO,
            first: DVec2::new(2.0, 0.0),
            second: 1.0,
        },
        from: 0.0,
        sweep: std::f64::consts::PI,
    };
    let dipping = Curve::Round {
        centre: DVec2::new(0.0, 1.5),
        radius: 1.0,
        from: std::f64::consts::PI,
        sweep: std::f64::consts::PI,
    };
    assert!(enclosed(&[half, dipping], [0.0, 0.75]));
    assert!(!enclosed(&[half, dipping], [0.0, 0.3]));
    assert!(!enclosed(&[dipping, half], [0.0, 1.2]));
}

#[test]
fn a_touch_where_a_curve_starts_its_turn_closes_what_it_pinches() {
    let touching = |first: Curve| {
        [
            first,
            ellipse([1.3, 0.0], [0.7, 0.0], 0.9),
            straight([-1.2, 0.3], [3.0, 0.3]),
        ]
    };
    let pocket = [0.5925123921954916, 0.26738742834040424];
    assert!(enclosed(&touching(circle([0.0, 0.0], 0.6)), pocket));
    let mut reversed = touching(circle([0.0, 0.0], 0.6));
    reversed.reverse();
    assert!(enclosed(&reversed, pocket));
}

#[test]
fn an_arc_shorter_than_one_place_encloses_nothing() {
    let sliver = Curve::Round {
        centre: DVec2::ZERO,
        radius: 5.0,
        from: 0.3,
        sweep: 1e-8,
    };
    assert!(!enclosed(&[sliver], [0.0, 0.0]));
    let nearly_whole = Curve::Round {
        centre: DVec2::ZERO,
        radius: 5.0,
        from: 0.3,
        sweep: std::f64::consts::TAU - 1e-8,
    };
    assert!(enclosed(&[nearly_whole], [0.0, 0.0]));
}

#[test]
fn three_circles_each_touching_the_other_two_enclose_the_gap_between_them() {
    let height = 3.0f64.sqrt();
    for radius in [1.0, 1.0 - 1e-9] {
        let touching = [
            circle([0.0, 0.0], radius),
            circle([2.0, 0.0], radius),
            circle([1.0, height], radius),
        ];
        assert!(enclosed(&touching, [1.0, height / 3.0]), "radius {radius}");
        assert!(!enclosed(&touching, [1.0, -1.5]), "radius {radius}");
    }
}

#[test]
fn three_circles_a_hair_apart_leave_the_gap_between_them_open() {
    let radius = 1.0 - hair(3.0);
    let apart = [
        circle([0.0, 0.0], radius),
        circle([2.0, 0.0], radius),
        circle([1.0, 3.0f64.sqrt()], radius),
    ];
    assert!(!enclosed(&apart, [1.0, 3.0f64.sqrt() / 3.0]));
    assert!(enclosed(&apart, [0.0, 0.0]));
}

#[test]
fn two_points_standing_in_one_place_are_one_place_for_what_closes() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners: Vec<_> = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0]]
        .iter()
        .map(|place| sketch.add_point(DVec2::from(*place)))
        .collect();
    sketch.add_segment(corners[0], corners[1]);
    sketch.add_segment(corners[1], corners[2]);
    let again = sketch.add_point(DVec2::new(2.0, 2.0));
    let side = sketch.add_point(DVec2::new(0.0, 2.0));
    let back = sketch.add_point(DVec2::ZERO);
    sketch.add_segment(again, side);
    sketch.add_segment(side, back);

    assert!(enclosed(&curves_of(&sketch), [1.0, 1.0]));
}

#[test]
fn construction_geometry_closes_nothing() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners: Vec<_> = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]]
        .iter()
        .map(|place| sketch.add_point(DVec2::from(*place)))
        .collect();
    for index in 0..3 {
        sketch.add_segment(corners[index], corners[index + 1]);
    }
    sketch.add_construction_segment(corners[3], corners[0]);
    let centre = sketch.add_point(DVec2::new(5.0, 5.0));
    sketch.add_construction_circle(centre, 1.0);

    let curves = curves_of(&sketch);
    assert!(!enclosed(&curves, [1.0, 1.0]));
    assert!(!enclosed(&curves, [5.0, 5.0]));
}

#[test]
fn half_an_ellipse_closed_by_a_trait_encloses_what_lies_inside() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (left, right) = (DVec2::new(-2.0, 0.0), DVec2::new(2.0, 0.0));
    let above = DVec2::new(0.0, 1.0);
    let drawn = ellipse_from(EllipseMode::ByEnds, &[left, right], above).expect("an ellipse");
    let rise = rise_of(drawn, above);
    let centre = sketch.add_point(drawn.centre);
    let first = [sketch.add_point(left), sketch.add_point(right)];
    let second = [centre, sketch.add_point(drawn.centre + rise.reach(drawn))];
    let id = sketch.add_ellipse(centre, first, second);
    let [from, to] = rise.between().map(|rank| first[rank]);
    sketch.draw_the_stretch(id, from, to);

    assert!(!enclosed(&curves_of(&sketch), [0.0, 0.5]));
    sketch.add_segment(first[0], first[1]);
    let curves = curves_of(&sketch);
    assert!(enclosed(&curves, [0.0, 0.5]));
    assert!(!enclosed(&curves, [0.0, -0.5]));
}

fn turned_by_thirty_degrees(at: DVec2) -> DVec2 {
    DVec2::from_angle(30f64.to_radians()).rotate(at)
}

#[test]
fn a_hair_the_walk_welds_only_once_moved_out_may_be_read_otherwise() {
    let mut curves = chain(&[[1.0, 1.0], [3.0, 1.0], [2.0, 3.0], [1.5, 1.000002]]);
    curves.push(circle([105.0, 105.0], 5.0));
    let enclosure = Enclosure::of(&curves);
    assert!(enclosure.may_read_otherwise(|at| at + DVec2::new(24.0, -16.0)));
    assert!(!enclosure.may_read_otherwise(|at| at));
}

#[test]
fn a_hair_near_the_origin_does_not_keep_a_far_drawing_from_being_turned() {
    let mut curves = chain(&[[5.0, 5.0], [5.0, 0.00006]]);
    curves.push(straight([0.0, 0.0], [10.0, 0.0]));
    curves.push(circle([400.0, 400.0], 10.0));
    assert!(!Enclosure::of(&curves).may_read_otherwise(turned_by_thirty_degrees));
}

#[test]
fn a_touch_the_walk_welds_wherever_it_lands_is_read_alike() {
    let curves = vec![
        straight([0.0, 0.0], [200.0, 0.0]),
        straight([150.0, 10.0], [150.0, 0.00000017]),
    ];
    let enclosure = Enclosure::of(&curves);
    assert!(!enclosure.may_read_otherwise(turned_by_thirty_degrees));
    assert!(!enclosure.may_read_otherwise(|at| at + DVec2::new(64.0, -32.0)));
}

#[test]
fn a_quarter_turn_never_reads_a_miss_otherwise() {
    let curves = vec![
        straight([0.0, 0.0], [50.0, 0.0]),
        straight([35.0, 10.0], [35.0, 0.00000508]),
    ];
    let enclosure = Enclosure::of(&curves);
    assert!(enclosure.may_read_otherwise(|at| at + DVec2::new(64.0, -32.0)));
    assert!(!enclosure.may_read_otherwise(|at| carried(Relaying::Turned(90.0), at)));
}

fn a_chain_folded_back_along_its_first_trait(lift: f64) -> Vec<Curve> {
    vec![
        straight(
            [2.2297911684811087, 3.757361582676507e-10 * lift],
            [4.196607412878203, 6.754318231874565e-10 * lift],
        ),
        straight(
            [4.196607412878203, 6.754318231874565e-10 * lift],
            [0.0, 0.0],
        ),
        straight(
            [0.0, 0.0],
            [11.21127180268251, 1.8355107801149555e-9 * lift],
        ),
    ]
}

#[test]
fn a_chain_folded_back_along_itself_closes_nothing_when_its_traits_lie_along_each_other_exactly() {
    assert_eq!(
        Enclosure::of(&a_chain_folded_back_along_its_first_trait(0.0)).areas_closed(),
        Some(0)
    );
}

#[test]
fn a_chain_folded_back_along_itself_closes_nothing_when_its_traits_lie_within_one_place_of_each_other()
 {
    assert_eq!(
        Enclosure::of(&a_chain_folded_back_along_its_first_trait(1.0)).areas_closed(),
        Some(0),
        "traits a few 1e-10 apart, inside one place, enclose no surface between them"
    );
}

/// Two traits ten long crossing at the origin, the second's ends `rise` off
/// the first's, each pair of ends joined so that the two halves close.
fn assert_a_bow_tie_closes_its_two_halves(rise: f64) {
    let bow_tie = [
        straight([-5.0, 0.0], [5.0, 0.0]),
        straight([-5.0, -rise], [5.0, rise]),
        straight([-5.0, 0.0], [-5.0, -rise]),
        straight([5.0, 0.0], [5.0, rise]),
    ];
    let enclosure = Enclosure::of(&bow_tie);
    assert_eq!(
        enclosure.areas_closed(),
        Some(2),
        "the two halves of a bow tie whose ends rise {rise}"
    );
    assert_eq!(
        enclosure.encloses(DVec2::new(-2.5, -rise / 4.0)),
        Some(true)
    );
    assert_eq!(enclosure.encloses(DVec2::new(2.5, rise / 4.0)), Some(true));
    assert_eq!(
        enclosure.encloses(DVec2::new(-2.5, rise / 4.0)),
        Some(false)
    );
}

#[test]
fn two_traits_nearly_parallel_crossing_in_their_middle_are_cut_once_where_they_cross() {
    assert_a_bow_tie_closes_its_two_halves(5.0 * 1e-4f64.tan());
}

#[test]
fn two_traits_whose_ends_stand_just_beyond_the_doubt_of_lying_along_each_other_are_cut_once_where_they_cross()
 {
    assert_a_bow_tie_closes_its_two_halves(6e-6);
}

#[test]
fn a_long_trait_beside_a_short_one_farther_than_one_place_from_it_closes_nothing() {
    let gap = 1e-5;
    let enclosure = Enclosure::of(&[
        straight([-1.0, 0.0], [1.0, 0.0]),
        straight([-70.0, gap], [70.0, gap]),
    ]);
    assert_eq!(
        enclosure.areas_closed(),
        Some(0),
        "two traits apart by {gap} near the origin, where one place is 4e-7"
    );
    assert_eq!(enclosure.encloses(DVec2::new(0.0, gap / 2.0)), Some(false));
}

/// The circle seed 30002413 settles into, and a round ellipse on its centre
/// whose radius stands `apart` from it, its first axis turned away from where
/// the circle starts.
fn a_circle_and_a_round_ellipse_turned_from_it(apart: f64) -> Vec<Curve> {
    let centre = DVec2::new(10.26801538484842, 3.6915722659915917);
    let radius = 10.911454794741264;
    let turned = DVec2::new(10.268015384851186, 3.6915722659838983).normalize();
    vec![
        circle(centre.into(), radius),
        Curve::Oval {
            drawn: EllipseDraft {
                centre,
                first: turned * (radius + apart),
                second: radius + apart,
            },
            from: 0.0,
            sweep: std::f64::consts::TAU,
        },
    ]
}

#[test]
fn a_circle_and_a_round_ellipse_lying_on_it_within_one_place_close_one_area() {
    let enclosure = Enclosure::of(&a_circle_and_a_round_ellipse_turned_from_it(3e-9));
    assert_eq!(
        enclosure.areas_closed(),
        Some(1),
        "a circle and an ellipse 3e-9 apart, inside one place, are one disc"
    );
    assert_eq!(enclosure.encloses(DVec2::new(10.0, 4.0)), Some(true));
    assert_eq!(enclosure.encloses(DVec2::new(30.0, 4.0)), Some(false));
}

#[test]
fn a_circle_and_a_round_ellipse_a_hair_apart_beyond_one_place_close_two_areas() {
    let apart = 2.0 * hair(22.0);
    assert_eq!(
        Enclosure::of(&a_circle_and_a_round_ellipse_turned_from_it(apart)).areas_closed(),
        Some(2),
        "a circle and an ellipse {apart} apart enclose a sliver between them"
    );
}

#[test]
fn an_ellipse_and_its_mirror_lying_on_it_within_one_place_close_one_area() {
    let ellipse_and_mirror = [
        ellipse(
            [1.9430826598650506e-9, 7.602026630243957],
            [1.4232288077731736, -3.5270808496079553e-10],
            7.602026630248487,
        ),
        ellipse(
            [-1.9610611732446543e-9, 7.60202663022837],
            [-1.423228807764167, -3.5622260696754893e-10],
            7.602026630252278,
        ),
    ];
    let enclosure = Enclosure::of(&ellipse_and_mirror);
    assert_eq!(
        enclosure.areas_closed(),
        Some(1),
        "an ellipse and its mirror a few 1e-9 apart, inside one place, are one area"
    );
    assert_eq!(enclosure.encloses(DVec2::new(0.0, 7.0)), Some(true));
    assert_eq!(enclosure.encloses(DVec2::new(3.0, 7.0)), Some(false));
}

#[test]
fn a_side_through_an_ellipse_s_centre_crosses_it_twice() {
    let sketch = laid(&[
        Gesture::Ellipse {
            centre: [8.0, 3.0],
            reach: [8.0, -2.0],
            across: 3.0,
            construction: false,
        },
        Gesture::Rectangle {
            corner: [8.0, 5.0],
            opposite: [2.0, 9.5],
            construction: false,
        },
    ]);

    assert_eq!(Enclosure::of(&curves_of(&sketch)).areas_closed(), Some(3));
    assert_eq!(as_many_as_closed(&areas_of(&sketch), 3), Ok(()));
}

#[test]
fn a_side_through_a_turned_ellipse_s_centre_crosses_it_twice() {
    let sketch = laid(&[
        Gesture::Ellipse {
            centre: [9.0, 6.0],
            reach: [9.707106781186548, 6.707106781186548],
            across: 0.5,
            construction: false,
        },
        Gesture::Rectangle {
            corner: [6.0, 6.0],
            opposite: [11.0, 9.0],
            construction: false,
        },
    ]);

    assert_eq!(Enclosure::of(&curves_of(&sketch)).areas_closed(), Some(3));
    assert_eq!(as_many_as_closed(&areas_of(&sketch), 3), Ok(()));
}
