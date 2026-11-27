//! The rows of a round's grid a face's strips are cut at, between the
//! face's ends along the meridian and away from its levels: none within a
//! hundredth of a step of either, nor within the room the caller asks if
//! that is wider, so that no strip beside an end or a level is a sliver
//! thinner than the faces across it sag (#956's ball cut by a ball). A
//! meridian arc keeps every row of the grid outside its room: where one of
//! its samples stands on a row a face leaves out, the face's column runs on
//! through that point rather than the face being cut there, so the faces
//! either side still share every sample of the arc. That is held through
//! the drawing, in
//! `tests/spheres_of_one_centre_and_radius_are_one_surface_whatever_their_axes.rs`.
//!
//! A row left out within a hundredth of a step of a rim is cut again where a
//! wall running beside the face from that rim ends on it (#1127,
//! `rows_keeping`): the rim and the end each as their angle along the
//! meridian, as `outline/knife.rs`'s `ends` reads them off the body. Which
//! walls those are is held there, in `outline/knife/tests.rs`.

use std::f64::consts::TAU;

use super::super::round::{rows, rows_keeping};

/// The grid: 36 rows a turn, a step of 10°.
const STEPS: usize = 36;

/// The rows `rows` keeps between `low` and `high` away from `levels` and
/// `room`, all in steps of the grid, each as its rank on it — a row
/// standing exactly on it.
fn kept(low: f64, high: f64, levels: &[f64], room: f64) -> Vec<i64> {
    let step = TAU / STEPS as f64;
    let levels: Vec<f64> = levels.iter().map(|level| level * step).collect();
    ranks(rows(STEPS, low * step, high * step, &levels, room * step))
}

/// The rows `rows_keeping` cuts a face from `low` to `high` at, beside the
/// `levels` and the walls' `ends`, each a rim and where the wall from it
/// ends, all in steps of the grid, each as its rank on it.
fn kept_beside(low: f64, high: f64, levels: &[f64], ends: &[[f64; 2]]) -> Vec<i64> {
    let step = TAU / STEPS as f64;
    let levels: Vec<f64> = levels.iter().map(|level| level * step).collect();
    let ends: Vec<[f64; 2]> = (ends.iter())
        .map(|&[rim, end]| [rim * step, end * step])
        .collect();
    ranks(rows_keeping(
        STEPS,
        (low * step, high * step),
        &levels,
        0.0,
        &ends,
    ))
}

/// Each of `rows` as its rank on the grid, a row standing exactly on it.
fn ranks(rows: Vec<f64>) -> Vec<i64> {
    let step = TAU / STEPS as f64;
    rows.into_iter()
        .map(|row| {
            let rank = (row / step).round();
            assert!(
                (row - rank * step).abs() <= 1e-12,
                "{row} is no row of the grid"
            );
            rank as i64
        })
        .collect()
}

#[test]
fn a_row_half_a_hundredth_of_a_step_inside_an_end_is_left_out_and_one_two_hundredths_inside_is_kept()
 {
    assert_eq!(kept(2.995, 7.02, &[], 0.0), vec![4, 5, 6, 7]);
    assert_eq!(kept(2.98, 7.005, &[], 0.0), vec![3, 4, 5, 6]);
}

#[test]
fn a_row_within_a_hundredth_of_a_step_of_a_level_a_whole_number_of_turns_on_is_left_out_and_one_three_hundredths_off_is_kept()
 {
    let turn = STEPS as f64;
    assert_eq!(
        kept(0.5, 9.5, &[5.004, 7.03 + turn, 2.996 - turn], 0.0),
        vec![1, 2, 4, 6, 7, 8, 9]
    );
}

#[test]
fn a_room_wider_than_a_hundredth_of_a_step_leaves_out_every_row_within_it() {
    assert_eq!(kept(2.9, 6.85, &[], 0.2), vec![4, 5, 6]);
    assert_eq!(kept(0.5, 9.5, &[5.15], 0.2), vec![1, 2, 3, 4, 6, 7, 8, 9]);
}

#[test]
fn a_row_within_a_hundredth_of_a_step_of_a_rim_is_cut_where_a_wall_running_from_the_rim_ends_on_it()
{
    assert_eq!(
        kept_beside(0.5, 9.5, &[2.995], &[[2.995, 3.0]]),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
    assert_eq!(
        kept_beside(0.5, 9.5, &[3.004], &[[3.004, 3.0]]),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
    assert_eq!(
        kept_beside(2.995, 9.5, &[2.995], &[[2.995, 3.0]]),
        vec![3, 4, 5, 6, 7, 8, 9],
        "the rim the face's own end"
    );
}

#[test]
fn a_row_within_a_hundredth_of_a_step_of_a_rim_is_left_out_where_no_wall_from_the_rim_ends_on_it() {
    let left_out = vec![1, 2, 4, 5, 6, 7, 8, 9];
    assert_eq!(kept_beside(0.5, 9.5, &[2.995], &[]), left_out);
    assert_eq!(
        kept_beside(0.5, 9.5, &[2.995], &[[2.995, 5.0]]),
        left_out,
        "the wall ending on another row"
    );
    assert_eq!(
        kept_beside(0.5, 9.5, &[2.995], &[[2.995, 3.5]]),
        left_out,
        "the wall ending between two rows"
    );
    assert_eq!(
        kept_beside(0.5, 9.5, &[2.995, 7.5], &[[7.5, 3.0]]),
        left_out,
        "the wall running from another rim"
    );
}

#[test]
fn a_row_a_wall_ends_on_is_left_out_where_it_stands_within_a_hundredth_of_a_step_of_a_level_or_an_end_of_the_face_but_not_of_the_wall_s_rim()
 {
    assert_eq!(
        kept_beside(0.5, 9.5, &[2.5, 6.004], &[[2.5, 6.0]]),
        vec![1, 2, 3, 4, 5, 7, 8, 9],
        "beside another level"
    );
    assert_eq!(
        kept_beside(2.995, 9.5, &[5.5], &[[5.5, 3.0]]),
        vec![4, 5, 6, 7, 8, 9],
        "beside the face's end"
    );
}
