//! What the first campaigns over random drawings found, shrunk and named, each
//! waiting on the issue that describes it.
//!
//! A drawing here fails today, which is why it is ignored: the gate cannot
//! hold a test that is red on purpose. It stays in the repository so that the
//! drawing is never lost, and `--ignored` runs them all. The commit that fixes
//! one takes its `ignore` off, and from then on the gate holds it.
//!
//! Closes #500.
//! - what the first campaigns find is written down, the case added to an issue
//!   already open — `a_rectangle_on_part_of_another_s_side_is_tinted_as_it_encloses`,
//!   `a_trait_laid_along_a_side_leaves_the_shape_its_area`,
//!   `an_arc_drawn_along_its_circle_leaves_the_disc_its_area`,
//!   `a_trait_drawn_twice_between_two_points_is_tinted_once`,
//!   `a_circle_drawn_twice_on_one_centre_is_one_area`,
//!   `a_pattern_whose_copies_land_on_each_other_tints_each_place_once`,
//!   `a_rectangle_mirrored_across_its_own_side_closes_the_copy`,
//!   `a_rectangle_copied_onto_its_own_top_closes_the_copy`,
//!   `a_second_rectangle_drawn_from_the_first_one_s_corner_keeps_both_areas`,
//!   `an_ellipse_inside_a_rectangle_keeps_its_areas_when_turned`,
//!   `half_an_ellipse_under_a_rectangle_s_top_keeps_its_areas_when_turned`,
//!   `an_ellipse_and_its_mirror_image_crossing_lie_one_area_apart`,
//!   `two_circles_touching_off_the_lattice_keep_both_their_areas`,
//!   `a_circle_touching_two_sides_of_a_rectangle_keeps_its_areas_when_turned`,
//!   `a_circle_tangent_inside_another_off_the_lattice_keeps_its_areas_in_either_order`,
//!   `an_area_pinched_where_two_circles_touch_is_tinted_on_its_own_side_of_the_pinch`
//! - and two new ones, each its own issue —
//!   `a_circle_touching_the_one_around_it_at_their_lowest_point_is_a_hole_of_it`,
//!   `an_outline_with_a_corner_on_one_of_its_diagonals_is_tinted_as_it_encloses`
//!
//! Closes #493.
//! - the five cases the campaigns filed under it hold, their `ignore` taken
//!   off — `a_rectangle_on_part_of_another_s_side_is_tinted_as_it_encloses`,
//!   `a_trait_laid_along_a_side_leaves_the_shape_its_area`,
//!   `an_arc_drawn_along_its_circle_leaves_the_disc_its_area`,
//!   `a_trait_drawn_twice_between_two_points_is_tinted_once`,
//!   `a_circle_drawn_twice_on_one_centre_is_one_area`
//! - what it brought out where it meets #494 is written down there, ignored
//!   under #494 — `a_rectangle_drawn_into_another_s_corner_is_tinted_as_it_encloses`,
//!   `a_rectangle_with_no_width_along_another_s_side_leaves_it_its_area`
//!
//! Closes #494.
//! - the six cases filed under it hold, their `ignore` taken off —
//!   `a_pattern_whose_copies_land_on_each_other_tints_each_place_once`,
//!   `a_rectangle_mirrored_across_its_own_side_closes_the_copy`,
//!   `a_rectangle_copied_onto_its_own_top_closes_the_copy`,
//!   `a_second_rectangle_drawn_from_the_first_one_s_corner_keeps_both_areas`,
//!   `a_rectangle_drawn_into_another_s_corner_is_tinted_as_it_encloses`,
//!   `a_rectangle_with_no_width_along_another_s_side_leaves_it_its_area`
//!
//! Closes #503.
//! - the drawing filed under it holds every rule, its `ignore` taken off —
//!   `a_circle_touching_the_one_around_it_at_their_lowest_point_is_a_hole_of_it`
//!
//! Closes #504.
//! - the drawing filed under it is tinted as it encloses, its `ignore` taken
//!   off — `an_outline_with_a_corner_on_one_of_its_diagonals_is_tinted_as_it_encloses`
//! - the side a corner lies on is read exactly — no test: the reading is
//!   private to the crate, and held beside it in regions/side/tests.rs
//!
//! Closes #508.
//! - #503's drawing, which broke the second rule while its depth was wrong,
//!   holds both matter rules now that #524 fixed it —
//!   `a_circle_touching_the_one_around_it_at_their_lowest_point_is_a_hole_of_it`
//! - #540's twin windows, which gave 325 mm² of matter for a measure of 500
//!   until #543 fixed the bridging, hold both rules with their two neighbours
//!   and the owner's notch —
//!   `a_frame_whose_windows_share_a_whole_side_raises_its_matter`,
//!   `a_frame_whose_windows_share_part_of_a_side_raises_its_matter`,
//!   `a_frame_whose_windows_touch_at_one_corner_raises_its_matter`,
//!   `a_rectangle_set_in_a_notch_of_a_square_raises_its_matter`
//!
//! Closes #602.
//! - on a region whose hole touches its outline, the matter covers what the
//!   area measures and every closed place lies in the matter of exactly one
//!   area — `an_ellipse_tangent_to_a_rectangle_s_top_and_bottom_raises_its_matter`,
//!   `a_small_circle_tangent_to_both_long_sides_of_a_rectangle_raises_its_matter`,
//!   and the campaign's third drawing,
//!   `a_circle_tangent_to_both_sides_of_a_rectangle_raises_its_matter`, whose
//!   last flaw — the tangency flicker when turned — #516 closed, its ignore
//!   taken off with it
//! - #529's solid-level behaviour does not regress — no test: it is held by
//!   the solid crate's own suite, which the gate runs
//! - the corridors bridged out to holes still pass — no test: the window-frame
//!   tests of regions and the #540 frames above already hold them
//!
//! Closes #603.
//! - cutting a corner of an open chain with a chamfer never changes what the
//!   areas around it claim to cover: the three drawings the hunt kept hold
//!   every rule —
//!   `a_chamfer_on_an_open_chain_closed_by_a_circle_keeps_the_tint_the_measure`,
//!   `a_chamfer_on_an_open_chain_closed_by_an_arc_keeps_the_tint_the_measure`,
//!   `a_chamfer_on_an_open_chain_against_a_pattern_keeps_the_tint_the_measure`
//! - the chamfer's named tests and what a chamfer draws on closed shapes keep
//!   passing — no test: they are chamfer.rs and corner/tests.rs's own, run in
//!   the same gate
//! - where the fault lay — no test: it was not the chamfer's rewiring but the
//!   touches the cut leaves against the closing curve; #604 closed the arc and
//!   pattern drawings and #606 the circle one, and this file pins the family
//!   so no future sighting is lost
//!
//! Closes #509.
//! - the diagonal drawn before the square on its ends breaks it, and is kept
//!   under #494 — `a_diagonal_drawn_before_the_square_on_its_ends_is_held_to_two_areas`.
//!   #494 is closed, so the drawing holds today: what the test rejects is
//!   #494's old answer, the square alone as one area, against the
//!   reckoning's count of two
//! - the faces of no surface are told apart: #493's kept cases break this rule
//!   only where they already broke another, or not at all —
//!   `a_curve_drawn_twice_closes_no_area_of_its_own`, and #493's cases above,
//!   held to every rule by `holds`
//! - a campaign has run with it, and what it found is written down: added to
//!   an open issue, or given one of its own — loops thinner than a billionth
//!   that the walk drops, ignored under #670 —
//!   `a_chain_closing_a_hair_from_a_rectangle_s_corner_keeps_as_many_areas_as_it_closes`,
//!   `a_line_passing_a_hair_from_where_a_circle_touches_a_side_keeps_as_many_areas_as_it_closes`,
//!   `a_diagonal_ending_a_hair_from_a_rectangle_s_corner_keeps_as_many_areas_as_it_closes`
//!
//! Closes #501.
//! - what the first campaigns find is written down, one issue per distinct
//!   failure. Nothing is fixed here — each kept ignored under its issue:
//!   `a_merge_that_would_leave_a_value_measuring_a_point_against_itself_is_refused`,
//!   `a_tangency_between_an_arc_and_a_trait_through_its_centre_is_refused`,
//!   `a_circle_s_centre_put_on_its_own_circle_is_refused`,
//!   `a_green_rectangle_stays_green_when_a_chain_is_joined_at_the_origin`,
//!   `a_green_square_stays_green_when_a_trait_is_joined_at_the_origin`,
//!   `a_green_straight_chain_stays_green_when_a_free_trait_is_drawn_from_the_origin`,
//!   `a_fixed_side_stays_put_when_a_circle_is_made_tangent_to_it_curve_first`,
//!   `a_drag_never_moves_a_fixed_point_it_did_not_grab`,
//!   `a_trait_given_a_horizontal_value_is_not_turned_vertical`,
//!   `half_an_ellipse_whose_end_is_made_one_with_its_centre_encloses_nothing_new`,
//!   `collinear_between_two_equal_sides_an_angle_holds_apart_is_refused`,
//!   `a_point_erased_where_two_curves_cross_leaves_them_their_areas`,
//!   `an_arc_whose_two_ends_are_brought_onto_one_place_keeps_the_rules_of_its_areas`
//! - what is not sorted by cause yet — the bulk of the area-walk failures on
//!   the exact contacts the rules create, a few `Untrue` and `Fixed` cases —
//!   is deferred, not claimed — no test: #1047 lists their seeds, and a case
//!   is pinned here once its cause has an issue
//!
//! Closes #1038.
//! - The circle, arc and half-ellipse cases are refused with the drawing
//!   unchanged — no test: the refusal is `cao_part`'s, held under the same
//!   names in `crates/part/tests/an_impossible_rule_is_refused.rs`
//! - The cases above hold in #501's campaign, the circle's `ignore` taken off
//!   — `a_circle_s_centre_put_on_its_own_circle_is_refused`,
//!   `an_arc_s_centre_put_on_its_own_arc_is_refused`,
//!   `half_an_ellipse_s_centre_put_on_its_own_curve_is_refused`
//!
//! Closes #1045.
//! - adding then erasing a point leaves the areas unchanged, on a crossing and
//!   a hair off it — no test: not in this file; `erased_points.rs` holds it at
//!   the sketch's level, the areas before the point against the areas after
//! - the cases above hold in #501's campaign: the drawing pinned under #1045
//!   with its `ignore` taken off, its twin a hair off the crossing, and the
//!   issue's two seeds, which reach it through a merge leaving the dropped
//!   point standing —
//!   `a_point_erased_where_two_curves_cross_leaves_them_their_areas`,
//!   `a_point_erased_a_hair_off_where_two_curves_cross_leaves_them_their_areas`,
//!   `a_circle_s_centre_made_one_with_a_corner_leaves_its_old_place_out_of_the_areas`,
//!   `a_place_on_a_circle_made_one_with_the_origin_leaves_its_old_place_out_of_the_areas`
//!
//! Closes #1037.
//! - Both kinds of curve refuse the tangency with the drawing unchanged,
//!   whether the radius is typed or not — no test: the refusal is
//!   `cao_part`'s, held under the same names in
//!   `crates/part/tests/an_impossible_rule_is_refused.rs`
//! - The cases above hold in #501's campaign, the arc's `ignore` taken off —
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
//!
//! Closes #1042.
//! - The three cases are refused with the drawing unchanged — no test: the
//!   refusal is `cao_part`'s, held in
//!   `crates/part/tests/an_impossible_rule_is_refused.rs`
//! - The cases above hold in #501's campaign, the trait's `ignore` taken off,
//!   with the half-ellipse of seed 30000105 and the perpendicular of seed
//!   20003072, mode `mix`, as the campaign shrank them —
//!   `a_trait_given_a_horizontal_value_is_not_turned_vertical`,
//!   `half_an_ellipse_given_a_horizontal_value_across_its_axis_is_not_turned`,
//!   `a_trait_given_a_value_along_an_axis_is_not_turned_square_to_a_side_against_it`
//!
//! Closes #1039.
//! - The rectangle stays settled when another shape is attached at the origin,
//!   and the freedom count is 3 — no test: held at the `cao_sketch` API,
//!   with the count corrected to the chain's own 4, in
//!   `crates/sketch/tests/shapes_meeting_at_the_origin.rs`
//! - The cases above hold in #501's campaign — the issue's case and the
//!   comment's two, their `ignore` taken off —
//!   `a_green_rectangle_stays_green_when_a_chain_is_joined_at_the_origin`,
//!   `a_green_square_stays_green_when_a_trait_is_joined_at_the_origin`,
//!   `a_green_straight_chain_stays_green_when_a_free_trait_is_drawn_from_the_origin`
//!
//! Closes #1036.
//! - That join returns false and the drawing is as it was — no test: not in
//!   this file; the join is held at the sketch's level in
//!   `crates/sketch/src/sketch/merging/tests.rs` and the merge step at the
//!   part's in `crates/part/tests/an_impossible_rule_is_refused.rs`, under the
//!   same names
//! - The case above holds in #501's campaign, its `ignore` taken off —
//!   `a_merge_that_would_leave_a_value_measuring_a_point_against_itself_is_refused`
//!
//! Closes #1041.
//! - The fixed point stays; the dragged point stops short — no test: not in
//!   this file; `what_a_drag_does_to_a_fixed_point.rs` holds it at the
//!   sketch's level, beside a fixed point grabbed directly that still follows
//!   the hand
//! - The case above holds in #501's campaign, its `ignore` taken off —
//!   `a_drag_never_moves_a_fixed_point_it_did_not_grab`
//!
//! Closes #1040.
//! - In both cases the fixed trait does not move and the free curve comes to
//!   it — no test: not in this file; `a_rule_brings_the_second_clicked_to_the_first.rs`
//!   lays the two rules directly, in both orders
//! - The cases above hold in #501's campaign, the tangency's `ignore` taken
//!   off, with the centre put on a fixed side drawn by hand, as seed 20000007
//!   no longer draws it —
//!   `a_fixed_side_stays_put_when_a_circle_is_made_tangent_to_it_curve_first`,
//!   `a_circle_s_centre_put_on_a_fixed_side_leaves_the_side_where_it_was`
//!
//! Closes #1044.
//! - The rule is refused with the drawing unchanged — no test: the refusal is
//!   `cao_part`'s, held under the same name in
//!   `crates/part/tests/an_impossible_rule_is_refused.rs`
//! - The case above holds in #501's campaign, its `ignore` taken off —
//!   `collinear_between_two_equal_sides_an_angle_holds_apart_is_refused`
//!
//! Closes #1043.
//! - The merge is refused or the half-ellipse goes; no area appears — no test:
//!   not in this file. As for an arc: with no value holding its size, the
//!   issue's case, the curve goes, whether its ends are the ends of its first
//!   axis, of its second, or points a trim held on it by rules; while a length
//!   is typed on either of its axes the merge is refused, so that no value
//!   typed vanishes. What the curve leaves behind stays as it was: its other
//!   end, its top handle, a value or a rule naming either, a trait on the
//!   other end; a point of it nothing names goes with it, as erasing the curve
//!   takes it, whatever rules hold elsewhere in the drawing. The join is held
//!   at the sketch's level in `crates/sketch/src/sketch/merging/tests.rs`, for
//!   either end and either click order, beside a half-ellipse end still made
//!   one with a point apart, the curve still drawn, and the arc's two cases;
//!   the merge step at the part's in
//!   `crates/part/tests/two_points_made_one.rs` and
//!   `crates/part/tests/an_impossible_rule_is_refused.rs`
//! - The cases above hold in #501's campaign, its `ignore` taken off —
//!   `half_an_ellipse_whose_end_is_made_one_with_its_centre_encloses_nothing_new`
//!
//! Closes #1070.
//! - two traits lying along each other within one place meet at the ends of
//!   each that lie on the other — no test: not in this file;
//!   `what_closes_around_a_place.rs` holds the reckoning to it directly
//! - seeds 31003453 and 30000037, mode `shares` 0,80,0, hold as the campaign
//!   shrank them —
//!   `a_chain_folded_onto_itself_by_collinear_closes_no_area`,
//!   `two_crossed_rectangles_under_a_parallel_and_a_collinear_keep_their_areas`
//! - a crossing between two traits only nearly parallel is still found once —
//!   no test: not in this file; `what_closes_around_a_place.rs` holds it
//!
//! Closes #1071.
//! - two closed curves that are the same curve within one place close one
//!   loop between them — no test: not in this file;
//!   `what_closes_around_a_place.rs` holds the reckoning to it directly
//! - seeds 30002413 and 30001705, mode `shares` 0,80,0, hold as the campaign
//!   shrank them —
//!   `a_circle_and_an_ellipse_made_one_curve_by_coincident_are_one_area`,
//!   `an_ellipse_mirrored_across_the_axis_its_centre_is_brought_onto_is_one_area_with_its_mirror`

// The drawing, the rules and the shrinking are the campaign's; this file only
// lays drawings and holds them.
#[allow(dead_code, unused_imports)]
mod random_sketches;

use glam::DVec2;
use random_sketches::areas::areas_of;
use random_sketches::checking::areas_keep_the_rules;
use random_sketches::enclosing::{Enclosure, curves_of};
use random_sketches::{Axis, Flaw, Gesture, as_many_as_closed, laid};

fn a_rectangle_on_part_of_another_s_side() -> Vec<Gesture> {
    vec![
        Gesture::Rectangle {
            corner: [7.0, 7.0],
            opposite: [4.0, 3.0],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [7.0, 1.0],
            opposite: [11.0, 6.0],
            construction: false,
        },
    ]
}

#[test]
fn a_rectangle_on_part_of_another_s_side_is_tinted_as_it_encloses() {
    random_sketches::holds(&a_rectangle_on_part_of_another_s_side());
}

fn a_trait_laid_along_a_side() -> Vec<Gesture> {
    vec![
        Gesture::Rectangle {
            corner: [10.0, 10.0],
            opposite: [9.0, 7.0],
            construction: false,
        },
        Gesture::Chain {
            through: vec![[9.0, 10.0], [14.0, 10.0]],
            closed: false,
            construction: false,
        },
    ]
}

#[test]
fn a_trait_laid_along_a_side_leaves_the_shape_its_area() {
    random_sketches::holds(&a_trait_laid_along_a_side());
}

fn an_arc_drawn_along_its_circle() -> Vec<Gesture> {
    vec![
        Gesture::Circle {
            centre: [6.0, 1.0],
            radius: 0.5,
            construction: false,
        },
        Gesture::Arc {
            centre: [6.0, 1.0],
            start: [5.5, 1.0],
            degrees: 135.0,
            construction: false,
        },
    ]
}

#[test]
fn an_arc_drawn_along_its_circle_leaves_the_disc_its_area() {
    random_sketches::holds(&an_arc_drawn_along_its_circle());
}

fn a_trait_drawn_twice_between_two_points() -> Vec<Gesture> {
    vec![
        Gesture::Chain {
            through: vec![[0.0, 0.0], [0.0, -4.0], [-3.0, -4.0], [1.0, 1.0]],
            closed: false,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[1.0, 1.0], [0.0, -4.0], [0.0, 0.0]],
            closed: false,
            construction: false,
        },
    ]
}

#[test]
fn a_trait_drawn_twice_between_two_points_is_tinted_once() {
    random_sketches::holds(&a_trait_drawn_twice_between_two_points());
}

fn a_circle_drawn_twice_on_one_centre() -> Vec<Gesture> {
    vec![
        Gesture::Circle {
            centre: [8.0, 4.0],
            radius: 2.0,
            construction: false,
        },
        Gesture::Circle {
            centre: [8.0, 4.0],
            radius: 2.0,
            construction: false,
        },
    ]
}

#[test]
fn a_circle_drawn_twice_on_one_centre_is_one_area() {
    random_sketches::holds(&a_circle_drawn_twice_on_one_centre());
}

#[test]
fn a_pattern_whose_copies_land_on_each_other_tints_each_place_once() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [1.0, 1.0],
            opposite: [3.0, 2.0],
            construction: false,
        },
        Gesture::Point { at: [0.0, 0.0] },
        Gesture::PatternAround {
            of: vec![[2.0, 1.0], [3.0, 1.5], [2.0, 2.0], [1.0, 1.5]],
            centre: [0.0, 0.0],
            degrees: 180.0,
            count: 4,
        },
    ]);
}

#[test]
fn a_rectangle_mirrored_across_its_own_side_closes_the_copy() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [20.0, 20.0],
            opposite: [27.5, 2.5],
            construction: false,
        },
        Gesture::Mirror {
            of: vec![[24.0, 2.5], [27.5, 11.5], [24.0, 20.0]],
            axis: Axis::Trait([20.0, 11.5]),
        },
    ]);
}

#[test]
fn a_rectangle_copied_onto_its_own_top_closes_the_copy() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [20.0, 5.0],
            opposite: [12.5, 15.0],
            construction: false,
        },
        Gesture::PatternAlong {
            of: vec![[20.0, 10.0], [16.5, 15.0], [12.5, 10.0]],
            axis: Axis::V,
            along: (10.0, 2),
            across: (25.0, 1),
        },
    ]);
}

#[test]
fn a_second_rectangle_drawn_from_the_first_one_s_corner_keeps_both_areas() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [7.0, 1.0],
            opposite: [5.5, -3.5],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [7.0, 1.0],
            opposite: [7.5, -3.5],
            construction: false,
        },
    ]);
}

fn a_rectangle_drawn_into_another_s_corner() -> Vec<Gesture> {
    vec![
        Gesture::Rectangle {
            corner: [6.0, 2.0],
            opposite: [13.0, 9.0],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [5.0, 5.0],
            opposite: [9.5, 2.5],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [8.5, 5.0],
            opposite: [9.5, 2.5],
            construction: false,
        },
    ]
}

#[test]
fn a_rectangle_drawn_into_another_s_corner_is_tinted_as_it_encloses() {
    random_sketches::holds(&a_rectangle_drawn_into_another_s_corner());
}

fn a_rectangle_with_no_width_along_another_s_side() -> Vec<Gesture> {
    vec![
        Gesture::Rectangle {
            corner: [8.0, 10.0],
            opposite: [8.0, 9.0],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [8.0, 8.0],
            opposite: [13.0, 10.0],
            construction: false,
        },
    ]
}

#[test]
fn a_rectangle_with_no_width_along_another_s_side_leaves_it_its_area() {
    random_sketches::holds(&a_rectangle_with_no_width_along_another_s_side());
}

#[test]
fn a_curve_drawn_twice_closes_no_area_of_its_own() {
    for (drawing, closes) in [
        (a_rectangle_on_part_of_another_s_side(), 2),
        (a_trait_laid_along_a_side(), 1),
        (an_arc_drawn_along_its_circle(), 1),
        (a_trait_drawn_twice_between_two_points(), 2),
        (a_circle_drawn_twice_on_one_centre(), 1),
        (a_rectangle_drawn_into_another_s_corner(), 4),
        (a_rectangle_with_no_width_along_another_s_side(), 1),
    ] {
        let sketch = laid(&drawing);
        let closed = Enclosure::of(&curves_of(&sketch)).areas_closed();
        assert_eq!(closed, Some(closes), "{drawing:?}");
        assert_eq!(
            as_many_as_closed(&areas_of(&sketch), closes),
            Ok(()),
            "{drawing:?}"
        );
    }
}

#[test]
fn a_diagonal_drawn_before_the_square_on_its_ends_is_held_to_two_areas() {
    let square = Gesture::Rectangle {
        corner: [0.0, 0.0],
        opposite: [2.0, 2.0],
        construction: false,
    };
    let sketch = laid(&[
        Gesture::Chain {
            through: vec![[2.0, 0.0], [0.0, 2.0]],
            closed: false,
            construction: false,
        },
        square.clone(),
    ]);
    let walked = areas_of(&sketch);
    let whole = areas_of(&laid(&[square]));

    assert_eq!(Enclosure::of(&curves_of(&sketch)).areas_closed(), Some(2));
    assert_eq!(whole.len(), 1, "the square alone is one area");
    assert_eq!(as_many_as_closed(&walked, 2), Ok(()));
    assert_eq!(
        as_many_as_closed(&whole, 2),
        Err(Flaw::Miscounted {
            found: 1,
            closed: 2
        })
    );
    assert_eq!(areas_keep_the_rules(&sketch, &walked), Ok(()));
    assert_eq!(
        areas_keep_the_rules(&sketch, &whole),
        Err(Flaw::Miscounted {
            found: 1,
            closed: 2
        })
    );
}

#[test]
#[ignore = "#419"]
fn an_ellipse_inside_a_rectangle_keeps_its_areas_when_turned() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [2.0, 7.0],
            opposite: [7.0, 11.0],
            construction: false,
        },
        Gesture::Ellipse {
            centre: [4.25, 8.75],
            reach: [6.5, 8.75],
            across: 1.75,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#419"]
fn half_an_ellipse_under_a_rectangle_s_top_keeps_its_areas_when_turned() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [4.0, 0.0],
            opposite: [2.5, 2.0],
            construction: false,
        },
        Gesture::HalfEllipse {
            from: [2.5, 2.0],
            to: [4.0, 2.0],
            rise: -1.0,
            construction: false,
        },
    ]);
}

#[test]
fn an_ellipse_and_its_mirror_image_crossing_lie_one_area_apart() {
    random_sketches::holds(&[
        Gesture::Ellipse {
            centre: [2.5, 0.5],
            reach: [4.974873734152916, 2.974873734152916],
            across: 1.5,
            construction: false,
        },
        Gesture::Mirror {
            of: vec![[1.1212254968042847, -2.125655316528075]],
            axis: Axis::V,
        },
    ]);
}

#[test]
fn two_circles_touching_off_the_lattice_keep_both_their_areas() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [35.0, 20.0],
            radius: 22.498995,
            construction: false,
        },
        Gesture::Circle {
            centre: [54.44472584031496, 39.44472584031496],
            radius: 5.0,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#419"]
fn a_circle_touching_two_sides_of_a_rectangle_keeps_its_areas_when_turned() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [120.0, 285.0],
            opposite: [195.0, 225.0],
            construction: false,
        },
        Gesture::Circle {
            centre: [158.0, 255.0],
            radius: 30.0,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#419"]
fn a_circle_tangent_inside_another_off_the_lattice_keeps_its_areas_in_either_order() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [11.414213562373096, f64::from_bits(0x3ff6a09e667f3bcc)],
            radius: 3.0,
            construction: false,
        },
        Gesture::Circle {
            centre: [10.0, 0.0],
            radius: 1.0,
            construction: false,
        },
    ]);
}

#[test]
fn an_area_pinched_where_two_circles_touch_is_tinted_on_its_own_side_of_the_pinch() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [40.0, 45.0],
            opposite: [68.0, 15.0],
            construction: false,
        },
        Gesture::Circle {
            centre: [30.0, 30.0],
            radius: 25.0,
            construction: false,
        },
        Gesture::Circle {
            centre: [25.0, 30.0],
            radius: 20.0,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#419"]
fn a_circle_tangent_under_a_rectangle_s_top_keeps_its_areas_mirrored() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [5.0, 5.0],
            opposite: [0.5, 8.5],
            construction: false,
        },
        Gesture::Circle {
            centre: [1.0, 6.0],
            radius: 2.5,
            construction: false,
        },
    ]);
}

#[test]
fn a_circle_touching_the_one_around_it_at_their_lowest_point_is_a_hole_of_it() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [5.0, 7.0],
            radius: 5.0,
            construction: false,
        },
        Gesture::Circle {
            centre: [5.0, 3.0],
            radius: 1.0,
            construction: false,
        },
    ]);
}

fn a_frame_with_two_windows(windows: [[[f64; 2]; 2]; 2]) -> Vec<Gesture> {
    let mut gestures = vec![Gesture::Rectangle {
        corner: [0.0, 0.0],
        opposite: [30.0, 30.0],
        construction: false,
    }];
    gestures.extend(windows.iter().map(|window| Gesture::Rectangle {
        corner: window[0],
        opposite: window[1],
        construction: false,
    }));
    gestures
}

#[test]
fn a_frame_whose_windows_share_a_whole_side_raises_its_matter() {
    random_sketches::holds(&a_frame_with_two_windows([
        [[5.0, 5.0], [15.0, 25.0]],
        [[15.0, 5.0], [25.0, 25.0]],
    ]));
}

#[test]
fn a_frame_whose_windows_share_part_of_a_side_raises_its_matter() {
    random_sketches::holds(&a_frame_with_two_windows([
        [[5.0, 5.0], [15.0, 25.0]],
        [[15.0, 10.0], [25.0, 20.0]],
    ]));
}

#[test]
fn a_frame_whose_windows_touch_at_one_corner_raises_its_matter() {
    random_sketches::holds(&a_frame_with_two_windows([
        [[5.0, 5.0], [15.0, 15.0]],
        [[15.0, 15.0], [25.0, 25.0]],
    ]));
}

#[test]
fn a_rectangle_set_in_a_notch_of_a_square_raises_its_matter() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [0.0, 0.0],
            opposite: [40.0, 40.0],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [0.0, 20.0],
            opposite: [15.0, 30.0],
            construction: false,
        },
    ]);
}

#[test]
fn an_outline_with_a_corner_on_one_of_its_diagonals_is_tinted_as_it_encloses() {
    random_sketches::holds(&[Gesture::Chain {
        through: vec![
            [2.0, 5.0],
            [0.5303300858899107, 1.237436867076458],
            [0.0, 1.0],
            [0.8838834764831843, 0.8838834764831843],
            [4.383883476483184, -2.6161165235168156],
        ],
        closed: true,
        construction: false,
    }]);
}

#[test]
fn a_circle_tangent_to_both_sides_of_a_rectangle_raises_its_matter() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [240.0, 240.0],
            opposite: [330.0, 90.0],
            construction: false,
        },
        Gesture::Circle {
            centre: [285.0, 165.0],
            radius: 45.0,
            construction: false,
        },
    ]);
}

#[test]
fn an_ellipse_tangent_to_a_rectangle_s_top_and_bottom_raises_its_matter() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [6.0, 0.0],
            opposite: [9.0, -6.0],
            construction: false,
        },
        Gesture::Ellipse {
            centre: [8.0, -3.0],
            reach: [9.0, -3.0],
            across: 3.0,
            construction: false,
        },
    ]);
}

#[test]
fn a_small_circle_tangent_to_both_long_sides_of_a_rectangle_raises_its_matter() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [6.0, 2.0],
            opposite: [8.0, 1.0],
            construction: false,
        },
        Gesture::Circle {
            centre: [7.0, 1.5],
            radius: 0.5,
            construction: false,
        },
    ]);
}

#[test]
fn a_chamfer_on_an_open_chain_closed_by_a_circle_keeps_the_tint_the_measure() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [2.0, 5.0],
            radius: 4.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![
                [2.0, 5.0],
                [8.0, 0.0],
                [6.0, 5.0],
                [6.0, 2.0],
                [2.0000205, 5.0],
            ],
            closed: false,
            construction: false,
        },
        Gesture::Chamfer {
            at: [6.0, 2.0],
            length: 1.0,
        },
    ]);
}

#[test]
fn a_chamfer_on_an_open_chain_closed_by_an_arc_keeps_the_tint_the_measure() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[10.0, 3.0], [4.0, 1.0], [6.0, 10.0]],
            closed: false,
            construction: false,
        },
        Gesture::Chamfer {
            at: [4.0, 1.0],
            length: 1.0,
        },
        Gesture::Arc {
            centre: [7.0, 3.5],
            start: [4.0, 3.5],
            degrees: 180.0,
            construction: false,
        },
    ]);
}

#[test]
fn a_chamfer_on_an_open_chain_against_a_pattern_keeps_the_tint_the_measure() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[9.0, 2.0], [2.0, 3.0], [2.0, 1.0]],
            closed: false,
            construction: false,
        },
        Gesture::Chamfer {
            at: [2.0, 3.0],
            length: 1.0,
        },
        Gesture::Rectangle {
            corner: [1.0, 2.4795],
            opposite: [4.5, 3.5],
            construction: false,
        },
        Gesture::Circle {
            centre: [1.0, 1.0],
            radius: 5.0,
            construction: false,
        },
        Gesture::PatternAround {
            of: vec![[-0.4776010333066978, 5.77668244562803]],
            centre: [4.5, 2.4795],
            degrees: 90.0,
            count: 4,
        },
    ]);
}

#[test]
#[ignore = "#670"]
fn a_chain_closing_a_hair_from_a_rectangle_s_corner_keeps_as_many_areas_as_it_closes() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [5.0, 9.0],
            opposite: [7.0, 5.0],
            construction: false,
        },
        Gesture::Chain {
            through: vec![[5.0, 5.0], [7.5, 0.5], [5.5, 0.0], [5.0000205, 5.0]],
            closed: false,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#670"]
fn a_line_passing_a_hair_from_where_a_circle_touches_a_side_keeps_as_many_areas_as_it_closes() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [3.0, 1.0],
            opposite: [0.0, 3.0],
            construction: false,
        },
        Gesture::Circle {
            centre: [1.501025, 1.75],
            radius: 0.75,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[0.0, 0.0], [6.0, 4.0]],
            closed: false,
            construction: false,
        },
    ]);
}

#[test]
#[ignore = "#670"]
fn a_diagonal_ending_a_hair_from_a_rectangle_s_corner_keeps_as_many_areas_as_it_closes() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [2.999959, 3.0],
            opposite: [4.0, 0.0],
            construction: false,
        },
        Gesture::Chain {
            through: vec![[0.0, 0.0], [3.0, 3.0]],
            closed: false,
            construction: false,
        },
    ]);
}

#[test]
fn a_merge_that_would_leave_a_value_measuring_a_point_against_itself_is_refused() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [2.0, 0.0],
            opposite: [0.5, 1.0],
            construction: false,
        },
        Gesture::Dimension {
            clicks: vec![[0.5, 0.0], [0.0, 0.0], [0.25, -0.1767766952966369]],
            value: 0.5,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[0.5, 0.0], [0.0, 0.0]],
        },
    ]);
}

#[test]
fn a_tangency_between_an_arc_and_a_trait_through_its_centre_is_refused() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[7.5, 5.0], [0.0, 0.0]],
            closed: false,
            construction: false,
        },
        Gesture::Arc {
            centre: [0.0, 0.0],
            start: [0.0, -3.0],
            degrees: 300.0,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Parallel,
            clicks: vec![[0.0, -9.600189391222042], [3.75, 2.5]],
        },
        Gesture::Dimension {
            clicks: vec![
                [1.5000000000000009, 2.5980762113533156],
                [1.834836879470614, 2.739642973376938],
            ],
            value: 3.6,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![
                [-5.9617084529937e-12, 0.6611054965051814],
                [1.7999999999956326, 3.117691453616419],
            ],
        },
    ]);
}

#[test]
fn a_tangency_between_an_arc_of_free_radius_and_a_trait_through_its_centre_is_refused() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[7.5, 5.0], [0.0, 0.0]],
            closed: false,
            construction: false,
        },
        Gesture::Arc {
            centre: [0.0, 0.0],
            start: [0.0, -3.0],
            degrees: 300.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[-2.598076211353316, -1.5], [0.0, -3.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![[3.75, 2.5], [-2.598076211353316, 1.5]],
        },
    ]);
}

#[test]
fn a_tangency_between_a_circle_and_a_trait_through_its_centre_is_refused() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[7.5, 5.0], [0.0, 0.0]],
            closed: false,
            construction: false,
        },
        Gesture::Circle {
            centre: [0.0, 0.0],
            radius: 3.0,
            construction: false,
        },
        Gesture::Dimension {
            clicks: vec![[-2.598076211353316, 1.5], [-3.2, 2.0]],
            value: 7.2,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![[3.75, 2.5], [-3.1176914536239795, 1.8]],
        },
    ]);
}

#[test]
fn a_tangency_between_a_circle_of_free_radius_and_a_trait_through_its_centre_is_refused() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[7.5, 5.0], [0.0, 0.0]],
            closed: false,
            construction: false,
        },
        Gesture::Circle {
            centre: [0.0, 0.0],
            radius: 3.0,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![[3.75, 2.5], [-2.598076211353316, 1.5]],
        },
    ]);
}

#[test]
fn a_tangency_between_a_circle_of_free_radius_and_a_trait_its_centre_is_held_on_is_refused() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [4.0, 8.0],
            radius: 3.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[-4.0, 3.0], [12.0, 13.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[4.0, 8.0], [10.0, 11.75]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![[10.0, 11.75], [2.41000317999046, 10.543994912015265]],
        },
    ]);
}

#[test]
fn a_tangency_between_a_circle_of_free_radius_and_a_trait_its_centre_is_the_middle_of_is_refused() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [4.0, 8.0],
            radius: 3.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[-4.0, 3.0], [12.0, 13.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Midpoint,
            clicks: vec![[4.0, 8.0], [10.0, 11.75]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![[10.0, 11.75], [2.41000317999046, 10.543994912015265]],
        },
    ]);
}

#[test]
fn a_tangency_between_a_circle_and_a_trait_merely_crossing_its_centre_lands() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [4.0, 8.0],
            radius: 3.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[-4.0, 3.0], [12.0, 13.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![[10.0, 11.75], [2.41000317999046, 10.543994912015265]],
        },
    ]);
}

#[test]
fn a_tangency_between_an_arc_of_free_radius_and_a_trait_its_centre_is_held_on_is_refused() {
    random_sketches::holds(&[
        Gesture::Arc {
            centre: [4.0, 8.0],
            start: [4.0, 5.0],
            degrees: 300.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[-4.0, 3.0], [12.0, 13.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[4.0, 8.0], [10.0, 11.75]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![[10.0, 11.75], [2.41000317999046, 10.543994912015265]],
        },
    ]);
}

#[test]
fn a_tangency_between_an_arc_of_free_radius_and_a_trait_its_centre_is_the_middle_of_is_refused() {
    random_sketches::holds(&[
        Gesture::Arc {
            centre: [4.0, 8.0],
            start: [4.0, 5.0],
            degrees: 300.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[-4.0, 3.0], [12.0, 13.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Midpoint,
            clicks: vec![[4.0, 8.0], [10.0, 11.75]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![[10.0, 11.75], [2.41000317999046, 10.543994912015265]],
        },
    ]);
}

#[test]
fn a_tangency_between_an_arc_and_a_trait_merely_crossing_its_centre_lands() {
    random_sketches::holds(&[
        Gesture::Arc {
            centre: [4.0, 8.0],
            start: [4.0, 5.0],
            degrees: 300.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[-4.0, 3.0], [12.0, 13.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![[10.0, 11.75], [2.41000317999046, 10.543994912015265]],
        },
    ]);
}

#[test]
fn a_tangency_between_half_an_ellipse_and_a_trait_through_its_centre_is_refused() {
    random_sketches::holds(&[
        Gesture::HalfEllipse {
            from: [0.0, 0.0],
            to: [40.0, 0.0],
            rise: 15.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[20.0, 0.0], [40.0, 30.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![[30.0, 15.0], [2.6794919243112254, 7.499999999999999]],
        },
    ]);
}

#[test]
fn a_circle_s_centre_held_on_a_trait_it_is_tangent_to_is_refused() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [4.0, 8.0],
            radius: 3.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![
                [-5.5899968200095405, 5.543994912015265],
                [10.41000317999046, 15.543994912015265],
            ],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![
                [8.41000317999046, 14.293994912015265],
                [5.5899968200095405, 5.456005087984735],
            ],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[8.41000317999046, 14.293994912015265], [4.0, 8.0]],
        },
    ]);
}

#[test]
fn a_circle_s_centre_made_the_middle_of_a_trait_it_is_tangent_to_is_refused() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [4.0, 8.0],
            radius: 3.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![
                [-5.5899968200095405, 5.543994912015265],
                [10.41000317999046, 15.543994912015265],
            ],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![
                [8.41000317999046, 14.293994912015265],
                [5.5899968200095405, 5.456005087984735],
            ],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Midpoint,
            clicks: vec![[8.41000317999046, 14.293994912015265], [4.0, 8.0]],
        },
    ]);
}

#[test]
fn an_arc_s_centre_held_on_a_trait_it_is_tangent_to_is_refused() {
    random_sketches::holds(&[
        Gesture::Arc {
            centre: [4.0, 8.0],
            start: [4.0, 5.0],
            degrees: 300.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![
                [-5.5899968200095405, 5.543994912015265],
                [10.41000317999046, 15.543994912015265],
            ],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![
                [8.41000317999046, 14.293994912015265],
                [5.5899968200095405, 5.456005087984735],
            ],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[8.41000317999046, 14.293994912015265], [4.0, 8.0]],
        },
    ]);
}

#[test]
fn an_arc_s_centre_made_the_middle_of_a_trait_it_is_tangent_to_is_refused() {
    random_sketches::holds(&[
        Gesture::Arc {
            centre: [4.0, 8.0],
            start: [4.0, 5.0],
            degrees: 300.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![
                [-5.5899968200095405, 5.543994912015265],
                [10.41000317999046, 15.543994912015265],
            ],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![
                [8.41000317999046, 14.293994912015265],
                [5.5899968200095405, 5.456005087984735],
            ],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Midpoint,
            clicks: vec![[8.41000317999046, 14.293994912015265], [4.0, 8.0]],
        },
    ]);
}

#[test]
fn a_circle_s_centre_put_on_its_own_circle_is_refused() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [20.0, 15.0],
            radius: 25.0,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[20.0, 15.0], [44.0, 22.0]],
        },
    ]);
}

#[test]
fn an_arc_s_centre_put_on_its_own_arc_is_refused() {
    random_sketches::holds(&[
        Gesture::Arc {
            centre: [20.0, 15.0],
            start: [45.0, 15.0],
            degrees: 120.0,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[20.0, 15.0], [35.0, 35.0]],
        },
    ]);
}

#[test]
fn half_an_ellipse_s_centre_put_on_its_own_curve_is_refused() {
    random_sketches::holds(&[
        Gesture::HalfEllipse {
            from: [0.0, 0.0],
            to: [40.0, 0.0],
            rise: 15.0,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[20.0, 0.0], [32.0, 12.0]],
        },
    ]);
}

#[test]
fn a_green_rectangle_stays_green_when_a_chain_is_joined_at_the_origin() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [0.0, 0.0],
            opposite: [4.0, 2.0],
            construction: false,
        },
        Gesture::Green {
            piece: vec![[4.0, 0.0], [4.0, 2.0], [0.0, 2.0]],
            given: vec![
                Gesture::Dimension {
                    clicks: vec![[2.0, 0.0], [2.3, 0.6]],
                    value: 4.0,
                },
                Gesture::Dimension {
                    clicks: vec![[4.0, 1.0], [3.4, 1.3]],
                    value: 2.0,
                },
                Gesture::Rule {
                    rule: cao_sketch::Rule::Perpendicular,
                    clicks: vec![[2.0, 0.0], [4.0, 1.0]],
                },
                Gesture::Rule {
                    rule: cao_sketch::Rule::Perpendicular,
                    clicks: vec![[4.0, 1.0], [2.0, 2.0]],
                },
                Gesture::Rule {
                    rule: cao_sketch::Rule::Perpendicular,
                    clicks: vec![[2.0, 2.0], [0.0, 1.0]],
                },
            ],
        },
        Gesture::Chain {
            through: vec![[6.0, 3.0], [9.0, 5.0], [12.0, 3.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[6.0, 3.0], [0.0, 0.0]],
        },
    ]);
}

#[test]
fn a_green_square_stays_green_when_a_trait_is_joined_at_the_origin() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [0.0, 0.0],
            opposite: [3.5, 3.5],
            construction: true,
        },
        Gesture::Green {
            piece: vec![[3.5, 0.0], [3.5, 3.5], [0.0, 3.5]],
            given: vec![
                Gesture::Dimension {
                    clicks: vec![[1.75, 0.0], [2.396829281940165, 0.27347503408557633]],
                    value: 3.5,
                },
                Gesture::Dimension {
                    clicks: vec![[3.5, 1.75], [4.146829281940165, 2.0234750340855765]],
                    value: 3.5,
                },
                Gesture::Dimension {
                    clicks: vec![[1.75, 3.5], [2.396829281940165, 3.7734750340855765]],
                    value: 3.5,
                },
                Gesture::Dimension {
                    clicks: vec![[0.0, 1.75], [0.646829281940165, 2.0234750340855765]],
                    value: 3.5,
                },
                Gesture::Rule {
                    rule: cao_sketch::Rule::Perpendicular,
                    clicks: vec![[1.75, 0.0], [0.0, 1.75]],
                },
            ],
        },
        Gesture::Chain {
            through: vec![[4.0, 3.0], [4.0, 8.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[4.0, 3.0], [0.0, 0.0]],
        },
    ]);
}

#[test]
fn a_green_straight_chain_stays_green_when_a_free_trait_is_drawn_from_the_origin() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[5.0, 5.0], [5.0, -12.5], [5.0, -22.5], [5.0, -37.5]],
            closed: false,
            construction: false,
        },
        Gesture::Green {
            piece: vec![[5.0, 5.0], [5.0, -12.5], [5.0, -22.5], [5.0, -37.5]],
            given: vec![
                Gesture::Rule {
                    rule: cao_sketch::Rule::Coincident,
                    clicks: vec![[5.0, 5.0], [0.0, 0.0]],
                },
                Gesture::Dimension {
                    clicks: vec![[5.0, -17.5], [7.7975095525956295, -16.31723193180734]],
                    value: 10.0,
                },
                Gesture::Dimension {
                    clicks: vec![[5.0, -30.0], [7.7975095525956295, -28.81723193180734]],
                    value: 15.0,
                },
                Gesture::Dimension {
                    clicks: vec![
                        [2.5, -6.25],
                        [5.0, -17.5],
                        [7.7975095525956295, -11.317231931807338],
                    ],
                    value: 158.19859051364818,
                },
                Gesture::Dimension {
                    clicks: vec![
                        [5.0, -17.5],
                        [5.0, -30.0],
                        [7.7975095525956295, -21.31723193180734],
                    ],
                    value: 180.0,
                },
                Gesture::Dimension {
                    clicks: vec![[5.0, -12.5], [0.0, 0.0], [2.5, -6.25]],
                    value: 13.46291201783626,
                },
            ],
        },
        Gesture::Chain {
            through: vec![[0.0, 0.0], [-3.0, 7.0]],
            closed: false,
            construction: false,
        },
    ]);
}

/// Whether a point of the drawing laid from `gestures` stands at `place`.
fn stands_at(gestures: &[Gesture], place: [f64; 2]) -> bool {
    laid(gestures)
        .live_points()
        .any(|(_, at)| at.distance(DVec2::from(place)) < 1e-6)
}

#[test]
fn a_fixed_side_stays_put_when_a_circle_is_made_tangent_to_it_curve_first() {
    let drawing = [
        Gesture::Rectangle {
            corner: [15.0, 40.0],
            opposite: [25.0, 55.0],
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Fixed,
            clicks: vec![[15.0, 47.0]],
        },
        Gesture::Circle {
            centre: [20.0, 30.0],
            radius: 5.0,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![[20.0, 35.0], [15.0, 47.0]],
        },
    ];
    assert!(
        stands_at(&drawing, [15.0, 40.0]) && stands_at(&drawing, [15.0, 55.0]),
        "the fixed side left (15, 40)–(15, 55)"
    );
    assert!(
        !stands_at(&drawing, [20.0, 30.0]),
        "the circle never came: its centre is still at (20, 30)"
    );
}

#[test]
fn a_circle_s_centre_put_on_a_fixed_side_leaves_the_side_where_it_was() {
    let drawing = [
        Gesture::Rectangle {
            corner: [1.0, 3.0],
            opposite: [6.5, 7.0],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [2.0, 9.0],
            opposite: [3.5, 4.0],
            construction: false,
        },
        Gesture::Circle {
            centre: [9.0, 2.0],
            radius: 1.0,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Fixed,
            clicks: vec![[1.0, 5.0]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Tangent,
            clicks: vec![[2.75, 4.0], [9.0, 3.0]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[3.5, 2.5], [1.0, 5.0]],
        },
    ];
    assert!(
        stands_at(&drawing, [1.0, 3.0]) && stands_at(&drawing, [1.0, 7.0]),
        "the fixed side left (1, 3)–(1, 7)"
    );
    assert!(
        !stands_at(&drawing, [3.5, 2.5]),
        "the circle never came: its centre is still at (3.5, 2.5)"
    );
}

#[test]
fn a_drag_never_moves_a_fixed_point_it_did_not_grab() {
    let drawing = [
        Gesture::Chain {
            through: vec![[2.0, 2.0], [8.0, 2.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Fixed,
            clicks: vec![[8.0, 2.0]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Parallel,
            clicks: vec![[5.0, 2.0], [20.0, 0.0]],
        },
        Gesture::Dimension {
            clicks: vec![[5.0, 2.0], [5.3, 2.6]],
            value: 6.0,
        },
        Gesture::Drag {
            from: [2.0, 2.0],
            to: [4.456, 2.0],
        },
    ];
    assert!(
        stands_at(&drawing, [8.0, 2.0]),
        "the fixed point left (8, 2)"
    );
}

#[test]
fn a_trait_given_a_horizontal_value_is_not_turned_vertical() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[2.0, 1.0], [8.0, 4.0]],
            closed: false,
            construction: false,
        },
        Gesture::Dimension {
            clicks: vec![[2.0, 1.0], [8.0, 4.0], [5.0, -1.0]],
            value: 6.0,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Parallel,
            clicks: vec![[5.0, 2.5], [0.0, 20.0]],
        },
    ]);
}

#[test]
fn half_an_ellipse_given_a_horizontal_value_across_its_axis_is_not_turned() {
    random_sketches::holds(&[
        Gesture::HalfEllipse {
            from: [90.0, 285.0],
            to: [111.21320343559643, 306.2132034355964],
            rise: 90.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[120.0, 180.0], [120.0, 30.0], [255.0, -105.0]],
            closed: false,
            construction: false,
        },
        Gesture::Drag {
            from: [255.0, -105.0],
            to: [273.1731453597358, -86.82685464026417],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[36.966991411008884, 359.24621202458735], [120.0, 180.0]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Parallel,
            clicks: vec![
                [196.5865726798679, -28.413427320132087],
                [0.0, -339.7706031132992],
            ],
        },
        Gesture::Dimension {
            clicks: vec![
                [225.71791125776616, 162.8461907668595],
                [-253.88269705564855, 0.0],
                [239.7483982163787, 168.7781855085551],
            ],
            value: 38.616,
        },
        Gesture::Dimension {
            clicks: vec![
                [350.9255707019138, 158.68860638923226],
                [364.3388543168321, 164.35965174263143],
            ],
            value: 1123.725120328554,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Parallel,
            clicks: vec![
                [-848.7917100641204, 0.0],
                [904.1953030066924, 439.9731851010184],
            ],
        },
    ]);
}

#[test]
fn a_trait_given_a_value_along_an_axis_is_not_turned_square_to_a_side_against_it() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[10.0, 2.0], [0.0, 0.0]],
            closed: false,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[8.0, 3.0], [9.5, 0.5]],
            closed: false,
            construction: false,
        },
        Gesture::Rectangle {
            corner: [3.0, 8.0],
            opposite: [1.0, 7.0],
            construction: false,
        },
        Gesture::Divide { at: [8.75, 1.75] },
        Gesture::Dimension {
            clicks: vec![[9.125, 1.125], [9.556622979794078, 1.3074872689084946]],
            value: 1.875,
        },
        Gesture::Dimension {
            clicks: vec![[4.375, 0.875], [4.820027433087742, 1.0631545808619483]],
            value: 13.385,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Perpendicular,
            clicks: vec![[3.0, 7.25], [11.351973915491572, 1.256579915742154]],
        },
    ]);
}

#[test]
fn half_an_ellipse_whose_end_is_made_one_with_its_centre_encloses_nothing_new() {
    random_sketches::holds(&[
        Gesture::HalfEllipse {
            from: [3.5, 4.0],
            to: [3.5, -1.0],
            rise: 2.5,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[3.5, -1.0], [3.5, 1.5]],
        },
    ]);
}

#[test]
fn collinear_between_two_equal_sides_an_angle_holds_apart_is_refused() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [0.0, 1.0],
            opposite: [1.0, -2.5],
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Equal,
            clicks: vec![[0.5, -2.5], [0.0, -1.0]],
        },
        Gesture::Dimension {
            clicks: vec![[0.0, -2.0], [0.0, -4.0], [0.0, -2.0]],
            value: 0.0,
        },
        Gesture::Dimension {
            clicks: vec![[1.0, -0.75], [1.1005813787660643, -0.7074748751263735]],
            value: 3.535,
        },
        Gesture::Dimension {
            clicks: vec![
                [1.0, -0.7675000000000001],
                [0.49999999999874134, -2.532501458206094],
                [1.101511629771004, -2.4920815713097606],
            ],
            value: 81.258,
        },
        Gesture::Dimension {
            clicks: vec![
                [0.4999999999986858, -2.6118858950394626],
                [-2.725310900930187e-12, -2.182894887207884],
                [0.10560651589716795, -2.6441220713019273],
            ],
            value: 81.25800000001097,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Collinear,
            clicks: vec![
                [-2.725310900930187e-12, -2.182894887207884],
                [0.4999999999986858, -2.6118858950394626],
            ],
        },
    ]);
}

#[test]
fn a_point_erased_where_two_curves_cross_leaves_them_their_areas() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [5.0, 5.0],
            radius: 3.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[1.0, 5.0], [10.0, 5.0], [5.5, 12.0]],
            closed: true,
            construction: false,
        },
        Gesture::Point { at: [2.0, 5.0] },
        Gesture::Erase { at: [2.0, 5.0] },
    ]);
}

#[test]
fn a_point_erased_a_hair_off_where_two_curves_cross_leaves_them_their_areas() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [5.0, 5.0],
            radius: 3.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[1.0, 5.0], [10.0, 5.0], [5.5, 12.0]],
            closed: true,
            construction: false,
        },
        Gesture::Point {
            at: [2.000_000_03, 5.0],
        },
        Gesture::Erase {
            at: [2.000_000_03, 5.0],
        },
    ]);
}

#[test]
fn a_circle_s_centre_made_one_with_a_corner_leaves_its_old_place_out_of_the_areas() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [0.0, 2.0],
            radius: 3.0,
            construction: false,
        },
        Gesture::Rectangle {
            corner: [5.0, 5.0],
            opposite: [10.0, 1.0],
            construction: false,
        },
        Gesture::Chain {
            through: vec![[0.0, 5.0], [0.0, -2.0], [11.0, 2.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[5.0, 5.0], [0.0, 2.0]],
        },
    ]);
}

#[test]
fn a_place_on_a_circle_made_one_with_the_origin_leaves_its_old_place_out_of_the_areas() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [10.0, 5.0],
            opposite: [8.0, 8.0],
            construction: false,
        },
        Gesture::Circle {
            centre: [8.0, 0.0],
            radius: 4.5,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Equal,
            clicks: vec![[10.0, 7.0], [9.0, 8.0]],
        },
        Gesture::Drag {
            from: [8.75, 5.0],
            to: [11.472877613848995, 7.722877613848995],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[12.5, 0.0], [0.0, 0.0]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Collinear,
            clicks: vec![
                [-10.044679889994793, 0.0],
                [10.239575754363916, 9.214033028722536],
            ],
        },
    ]);
}

#[test]
#[ignore = "#1046"]
fn an_arc_whose_two_ends_are_brought_onto_one_place_keeps_the_rules_of_its_areas() {
    random_sketches::holds(&[
        Gesture::Point { at: [0.0, 3.0] },
        Gesture::Arc {
            centre: [7.0, 1.0],
            start: [7.0, -0.5],
            degrees: 90.0,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[8.5, 1.0], [0.0, 3.0]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[7.131839854487193, -2.2632899575621437], [0.0, 0.0]],
        },
    ]);
}

#[test]
fn a_chain_folded_onto_itself_by_collinear_closes_no_area() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[10.0, 7.0], [12.0, 7.0], [0.0, 0.0], [5.0, -5.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Collinear,
            clicks: vec![[2.0, -2.0], [11.0, 7.0]],
        },
    ]);
}

#[test]
fn two_crossed_rectangles_under_a_parallel_and_a_collinear_keep_their_areas() {
    random_sketches::holds(&[
        Gesture::Chain {
            through: vec![[-3.0, 5.0], [1.0, 11.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rectangle {
            corner: [40.0, 15.0],
            opposite: [67.5, -7.5],
            construction: false,
        },
        Gesture::Rectangle {
            corner: [45.0, -7.5],
            opposite: [60.0, 22.5],
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Parallel,
            clicks: vec![[0.0, -49.0], [40.0, 4.0]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Collinear,
            clicks: vec![[52.5, 22.5], [45.0, 7.5]],
        },
    ]);
}

#[test]
fn a_circle_and_an_ellipse_made_one_curve_by_coincident_are_one_area() {
    random_sketches::holds(&[
        Gesture::Circle {
            centre: [10.0, 6.0],
            radius: 4.0,
            construction: false,
        },
        Gesture::Ellipse {
            centre: [10.0, 6.0],
            reach: [18.0, 6.0],
            across: 4.0,
            construction: false,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[14.0, 6.0], [18.0, 6.0]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Fixed,
            clicks: vec![[8.0, 10.0]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[2.0, 6.0], [0.0, 0.0]],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![
                [20.6921262997093, 6.916127641909394],
                [8.000000000028958, 10.0],
            ],
        },
    ]);
}

#[test]
fn an_ellipse_mirrored_across_the_axis_its_centre_is_brought_onto_is_one_area_with_its_mirror() {
    random_sketches::holds(&[
        Gesture::Ellipse {
            centre: [6.0, 9.0],
            reach: [7.060660171779821, 10.060660171779821],
            across: 0.5,
            construction: false,
        },
        Gesture::Mirror {
            of: vec![[5.335988725681406, 9.008487237645364]],
            axis: Axis::V,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Perpendicular,
            clicks: vec![
                [0.0, -7.205304147987005],
                [5.787867965644036, 8.787867965644036],
            ],
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Coincident,
            clicks: vec![[0.0, 0.0], [6.219669914110387, 7.969669914110086]],
        },
    ]);
}
