//! The rows of a round's grid a face's strips are cut at, between the
//! face's ends along the meridian and away from its levels: none within a
//! hundredth of a step of a level, nor within the room the caller asks of a
//! level or an end if that is wider, so that no strip beside a level is a
//! sliver thinner than the faces across it sag (#956's ball cut by a ball).

use std::f64::consts::TAU;

use super::super::round::rows;

/// The grid: 36 rows a turn, a step of 10°.
const STEPS: usize = 36;

/// The rows `rows` keeps between `low` and `high` away from `levels` and
/// `room`, all in steps of the grid, each as its rank on it — a row
/// standing exactly on it.
fn kept(low: f64, high: f64, levels: &[f64], room: f64) -> Vec<i64> {
    let step = TAU / STEPS as f64;
    let levels: Vec<f64> = levels.iter().map(|level| level * step).collect();
    rows(STEPS, low * step, high * step, &levels, room * step)
        .into_iter()
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
