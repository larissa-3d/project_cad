//! The numbered steps of the mirror's and the rectangular pattern's panel,
//! driven the way the input layer drives them: which step reads done, current
//! or to come, what ↵ does on each, what the message says, and what each tool
//! lays once its steps are through.
//!
//! Closes #739.
//!
//! Mirror and rectangular pattern
//!
//! - Mirror has two steps (« Éléments », « Axe »); the rectangular pattern
//!   three (« Éléments », « Direction », « Nombre » and « Pas »). Each step
//!   reads done, current or to come from the state of the `CopyPanel`; a
//!   presenter test checks it with no window, for each tool —
//!   `the_mirror_s_two_steps_read_done_current_or_to_come_from_its_panel`,
//!   `the_rectangular_pattern_s_three_steps_read_done_current_or_to_come_from_its_panel`;
//!   and a step reads done only once it holds what it asks for —
//!   `a_step_of_the_mirror_or_the_rectangular_pattern_skipped_while_empty_reads_to_come`
//! - ↵ validates the current step and moves on; on the last step it applies
//!   the copy; Escape cancels. One test per case and per tool — the mirror's,
//!   `enter_on_the_mirror_s_elements_validates_them_and_moves_on_to_its_axis`,
//!   `enter_on_the_mirror_s_axis_applies_the_mirror`; the rectangular
//!   pattern's,
//!   `enter_validates_each_step_of_the_rectangular_pattern_and_moves_on_to_the_next`,
//!   `enter_on_the_rectangular_pattern_s_last_step_applies_the_pattern`;
//!   Escape, which only the whole application's keyboard reaches, in
//!   `crates/app/tests/copying_in_a_panel.rs` by
//!   `escape_on_a_step_of_the_mirror_closes_its_panel_and_lays_nothing` and
//!   `escape_on_a_step_of_the_rectangular_pattern_closes_its_panel_and_lays_nothing`,
//!   under its own `Closes #739`
//! - The message pill follows the step (« Symétrie — étape 2 : axe »,
//!   « Réseau rectangulaire — étape 3 : nombre et pas »); a test on the
//!   wording checks it — `the_message_pill_follows_the_step_of_the_mirror`,
//!   `the_message_pill_follows_the_step_of_the_rectangular_pattern`, and as a
//!   step clicked in the panel moves it —
//!   `a_step_clicked_in_the_mirror_s_or_the_rectangular_pattern_s_panel_takes_the_message_pill_with_it`.
//!   They replace what #738 held these two tools to and this issue changes,
//!   `the_mirror_and_the_rectangular_pattern_still_ask_what_they_asked` and
//!   `a_row_clicked_in_the_mirror_s_or_the_rectangular_pattern_s_panel_leaves_the_message_as_it_was`,
//!   which leave `steps.rs` beside this file, their bullet there with them
//! - Both tools are laid out by `tool_panel` in numbered steps; the `slots`
//!   and `panel_row` rows of `render/copy_panel.rs` are gone. The controls of
//!   `ui/` already there (`step_dot`, `chip`, `number_field`,
//!   `formula_field`) are reused; none is added — no test: how the panel is
//!   laid out is drawing, which the tests do not reach, and which controls it
//!   is built from is a shape of the code, read in review rather than
//!   asserted; tried by hand, see « To try it »
//!
//! Section view
//!
//! - The section panel becomes a step panel, in this order: « Face » (the
//!   origin planes and the clicked face), « Distance », the two tilt angles,
//!   « Devant » / « Derrière » with « Inverser », and « Enregistrer la coupe »
//!   as the main button. Each step reads done, current or to come; a
//!   presenter test checks it — no test: here; held in
//!   `crates/app/src/screens/viewport/section/tests.rs` by
//!   `the_section_s_steps_read_done_current_or_to_come`, under its own
//!   `Closes #739`
//! - The message pill follows the section's step; a test on the wording
//!   checks it — no test: here; held in
//!   `crates/app/src/screens/viewport/section/tests.rs` by
//!   `the_message_pill_follows_the_step_of_the_section`, under its own
//!   `Closes #739`
//! - The values, the section kept in the part (#716, #717) and the handle on
//!   the cutting plane are unchanged; a test compares the section produced
//!   before and after — no test: here; held in
//!   `crates/app/src/screens/viewport/section/tests.rs` by
//!   `a_section_set_through_its_steps_is_kept_as_it_was_before`, under its
//!   own `Closes #739`
//!
//! « Inverser les côtés »
//!
//! - In the sketch, « Inverser les côtés » leaves the row under the ribbon
//!   for a small floating panel of the same look as the step panel, holding
//!   that button alone, at the top left of the view like the step panel. A
//!   presenter test checks that it appears only during a sketch chamfer or
//!   fillet, and that the button swaps the sides — no test: here; held in
//!   `crates/app/src/screens/viewport/input/cut_changes/tests.rs` by
//!   `the_swap_sides_panel_shows_only_during_a_sketch_chamfer_or_fillet` and
//!   `the_swap_sides_panel_s_button_swaps_the_sides`, under its own
//!   `Closes #739`
//! - The sketch's chamfer and fillet keep their fields by the cursor and
//!   their keys; a test checks it — no test: here; held in
//!   `crates/app/src/screens/viewport/input/corner/tests.rs` by
//!   `a_sketch_chamfer_or_fillet_still_takes_its_values_by_the_cursor_and_its_keys`,
//!   under its own `Closes #739`
//!
//! On screen
//!
//! - The panel can still be dragged and keeps its place — no test: the tests
//!   do not reach the screen; it is tried by hand, see « To try it »
//! - What each tool produces, and what a capped pattern says, are unchanged;
//!   a test per tool compares the result —
//!   `the_mirror_laid_through_its_steps_is_the_one_laid_before`,
//!   `the_rectangular_pattern_laid_through_its_steps_is_the_one_laid_before`;
//!   the circular pattern, which this issue does not move, and what a capped
//!   pattern says, by the tests already there, which stay green on this
//!   branch: `steps.rs` and `panel.rs` beside this file, and the cap's, named
//!   in the last bullet of `steps.rs`

use cao_prefs::Command;
use glam::DVec2;

use super::bench::Bench;
use crate::screens::sketch::{CopySlot, CopyStep, StepState};

const ON_THE_TRAIT_TO_COPY: DVec2 = DVec2::new(-10.0, 0.0);
const ON_THE_UPRIGHT: DVec2 = DVec2::new(2.0, 0.0);

/// A trait to copy, ten units tall, and an upright clear of it to mirror it
/// across or to repeat it along.
fn a_trait_and_an_upright() -> Bench {
    Bench::drawn(vec![
        Bench::a_trait(DVec2::new(-10.0, -5.0), DVec2::new(-10.0, 5.0), false),
        Bench::a_trait(DVec2::new(2.0, -5.0), DVec2::new(2.0, 5.0), false),
    ])
}

/// Each numbered step of the panel, top to bottom, as its dot reads.
fn steps(bench: &Bench) -> Vec<(CopyStep, StepState)> {
    bench.panel().steps(&bench.editor.live)
}

#[test]
fn the_mirror_s_two_steps_read_done_current_or_to_come_from_its_panel() {
    let mut bench = a_trait_and_an_upright();

    bench.take(Command::ToolMirror);
    assert_eq!(
        steps(&bench),
        [
            (CopyStep::Elements, StepState::Current),
            (CopyStep::Axis, StepState::ToCome),
        ],
        "the mirror taken with nothing in hand opens on its elements, its axis \
         to come"
    );

    bench.click(ON_THE_TRAIT_TO_COPY);
    assert_eq!(
        steps(&bench),
        [
            (CopyStep::Elements, StepState::Current),
            (CopyStep::Axis, StepState::ToCome),
        ],
        "a trait clicked is held, and the elements stay current for the next one"
    );

    bench.enter();
    assert_eq!(
        steps(&bench),
        [
            (CopyStep::Elements, StepState::Done),
            (CopyStep::Axis, StepState::Current),
        ],
        "the trait held and ↵ pressed, the axis is asked for"
    );
}

#[test]
fn the_rectangular_pattern_s_three_steps_read_done_current_or_to_come_from_its_panel() {
    let mut bench = a_trait_and_an_upright();

    bench.take(Command::ToolRectangularPattern);
    assert_eq!(
        steps(&bench),
        [
            (CopyStep::Elements, StepState::Current),
            (CopyStep::Direction, StepState::ToCome),
            (CopyStep::CountAndStep, StepState::ToCome),
        ],
        "the rectangular pattern taken with nothing in hand opens on its \
         elements"
    );

    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    assert_eq!(
        steps(&bench),
        [
            (CopyStep::Elements, StepState::Done),
            (CopyStep::Direction, StepState::Current),
            (CopyStep::CountAndStep, StepState::ToCome),
        ],
        "the trait held and ↵ pressed, the direction is asked for; the number \
         and the step stand below it, to come"
    );

    bench.click(ON_THE_UPRIGHT);
    assert_eq!(
        steps(&bench),
        [
            (CopyStep::Elements, StepState::Done),
            (CopyStep::Direction, StepState::Done),
            (CopyStep::CountAndStep, StepState::Current),
        ],
        "the direction clicked, the number and the step are asked for, their \
         fields holding the values they open on"
    );

    bench.enter();
    assert_eq!(
        steps(&bench),
        [
            (CopyStep::Elements, StepState::Done),
            (CopyStep::Direction, StepState::Done),
            (CopyStep::CountAndStep, StepState::Current),
        ],
        "the numbers and the steps of both ways are fields of one step: ↵ on \
         its first field stays on that step"
    );
}

#[test]
fn a_step_of_the_mirror_or_the_rectangular_pattern_skipped_while_empty_reads_to_come() {
    for (command, skipped) in [
        (
            Command::ToolMirror,
            vec![
                (CopyStep::Elements, StepState::ToCome),
                (CopyStep::Axis, StepState::Current),
            ],
        ),
        (
            Command::ToolRectangularPattern,
            vec![
                (CopyStep::Elements, StepState::ToCome),
                (CopyStep::Direction, StepState::Current),
                (CopyStep::CountAndStep, StepState::ToCome),
            ],
        ),
    ] {
        let mut bench = a_trait_and_an_upright();
        bench.take(command);

        bench.click_the_slot(CopySlot::Target);

        assert_eq!(
            steps(&bench),
            skipped,
            "{command:?}: nothing is held yet, and a check on the elements would \
             tell the user they were picked"
        );
    }
}
