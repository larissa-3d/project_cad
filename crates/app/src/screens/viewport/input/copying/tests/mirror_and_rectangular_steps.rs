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
//!   under its own `Closes #739`. Decided on the way: the axis is the mirror's
//!   last step, so a click naming it leaves the panel on it, current, for ↵
//!   to lay the mirror, as the circular pattern waits on its direction —
//!   `the_mirror_waits_on_its_axis_once_the_axis_is_clicked`; the rectangular
//!   pattern's last step holds four fields, ↵ moving from one to the next and
//!   laying on the last; and, as #738 decided, ↵ on the last step with a step
//!   left empty lays nothing and brings the first empty one back, marked —
//!   `enter_on_the_mirror_s_axis_with_no_axis_lays_nothing_and_marks_it`,
//!   `enter_on_the_rectangular_pattern_s_last_step_with_its_direction_empty_brings_it_back_marked`
//! - The message pill follows the step (« Symétrie — étape 2 : axe »,
//!   « Réseau rectangulaire — étape 3 : nombre et pas »); a test on the
//!   wording checks it — `the_message_pill_follows_the_step_of_the_mirror`,
//!   `the_message_pill_follows_the_step_of_the_rectangular_pattern`, and as a
//!   step clicked in the panel moves it —
//!   `a_step_clicked_in_the_mirror_s_or_the_rectangular_pattern_s_panel_takes_the_message_pill_with_it`.
//!   They replace what #738 held these two tools to and this issue changes,
//!   `the_mirror_and_the_rectangular_pattern_still_ask_what_they_asked` and
//!   `a_row_clicked_in_the_mirror_s_or_the_rectangular_pattern_s_panel_leaves_the_message_as_it_was`,
//!   which left `steps.rs` beside this file, their bullet there rewritten.
//!   Decided on the way: each step says its own name, as the issue's two do,
//!   « éléments » and « direction » for the two the issue does not quote; a
//!   copy laid brings the message back to step 1 with the panel; and the axis,
//!   the mirror's last step, goes on saying step 2 once it is clicked
//! - Both tools are laid out by `tool_panel` in numbered steps; the `slots`
//!   and `panel_row` rows of `render/copy_panel.rs` are gone. The controls of
//!   `ui/` already there (`step_dot`, `chip`, `number_field`,
//!   `formula_field`) are reused; none is added — no test: how the panel is
//!   laid out is drawing, which the tests do not reach, and which controls it
//!   is built from is a shape of the code, read in review rather than
//!   asserted; tried by hand, see « To try it ». Found in review: the axis
//!   step names what it holds, held in
//!   `crates/app/src/wording/copy_panel/tests.rs` by
//!   `the_axis_held_is_named_a_trait_by_its_row_and_each_sketch_axis_by_its_own`,
//!   under its own `Closes #739`
//!
//! Section view
//!
//! - The section panel becomes a step panel, in this order: « Face » (the
//!   origin planes and the clicked face), « Distance », the two tilt angles,
//!   « Devant » / « Derrière » with « Inverser », and « Enregistrer la coupe »
//!   as the main button. Each step reads done, current or to come; a
//!   presenter test checks it — no test: here; held in
//!   `crates/app/src/screens/viewport/section/tests/steps.rs` by
//!   `the_section_s_steps_read_done_current_or_to_come`,
//!   `a_face_clicked_in_the_view_fills_the_face_step_and_moves_on_to_the_distance`,
//!   `a_face_step_left_waiting_for_a_face_reads_to_come_rather_than_done`,
//!   `a_step_clicked_in_the_section_s_panel_becomes_current`,
//!   `enter_on_the_section_s_last_step_keeps_it_in_the_part` and
//!   `enter_on_the_section_s_last_step_while_its_face_is_still_to_click_keeps_nothing_and_goes_back_to_it`,
//!   under its own `Closes #739`, which says what was decided on the way;
//!   ↵ through the whole application, in `crates/app/tests/keeping_a_section.rs`,
//!   by `with_the_section_open_and_no_sketch_bare_enter_walks_its_fields_and_on_the_last_keeps_it_and_closes`
//!   and `with_the_section_open_enter_finishes_a_sketch_edited_and_keeps_no_section`;
//!   found in trying it, the panel closes once the section is kept, and the
//!   next « Vue en coupe » opens on the face, by
//!   `keeping_the_section_closes_its_panel_and_the_next_section_view_opens_on_its_face`,
//!   and the two sides stand on one row, by
//!   `the_two_sides_of_the_section_stand_on_one_row_whatever_they_are_drawn_as`;
//!   under its own `Closes #739`
//! - The message pill follows the section's step; a test on the wording
//!   checks it — no test: here; held in
//!   `crates/app/src/screens/viewport/section/tests/steps.rs` by
//!   `the_message_pill_follows_the_step_of_the_section`, and the pill on
//!   screen in `crates/app/tests/keeping_a_section.rs` by
//!   `the_title_bar_pill_says_the_section_s_step_until_the_section_is_closed`
//!   and, a sketch edited under the section saying its own tool,
//!   `with_the_section_open_a_sketch_edited_says_its_own_tool_rather_than_the_section_s_step`,
//!   each under its own `Closes #739`
//! - The values, the section kept in the part (#716, #717) and the handle on
//!   the cutting plane are unchanged; a test compares the section produced
//!   before and after — no test: here; held in
//!   `crates/app/src/screens/viewport/section/tests/steps.rs` by
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
//!   `the_swap_sides_panel_shows_only_while_a_sketch_chamfer_with_two_sides_is_held`
//!   and `the_swap_sides_panel_s_button_swaps_the_sides`, which says how the
//!   condition was read; the row it leaves, in
//!   `crates/app/src/screens/ribbon/view/tests.rs` by
//!   `with_a_chamfer_held_the_ribbon_no_longer_offers_to_swap_its_sides`; the
//!   panel over the view at its top left, in
//!   `crates/app/tests/swapping_a_chamfer_s_sides.rs` by
//!   `a_chamfer_held_puts_its_swap_in_a_panel_at_the_top_left_of_the_view_whose_button_swaps_its_sides`,
//!   none with nothing or a fillet held by
//!   `with_nothing_or_a_fillet_held_no_swap_stands_over_the_view`, and its
//!   « Fermer » letting the chamfer go by
//!   `closing_the_swap_panel_lets_the_chamfer_go_and_the_panel_with_it`;
//!   each under its own `Closes #739`
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
//!   `the_rectangular_pattern_laid_through_its_steps_is_the_one_laid_before`,
//!   each against the operation and the places the tool laid before #739,
//!   written out rather than read back from the code. Since #320 the mirror
//!   lays `HeldMirror` rather than `Mirror`, by decision of that issue: the
//!   copy is held to its original. What it names — the sketch, the elements,
//!   the axis — and the places it lays are those of before #739. The circular pattern,
//!   which this issue does not move, by the tests already there, which stay
//!   green on this branch: `steps.rs` and `panel.rs` beside this file. What a
//!   capped pattern says is only the circular pattern's, and stays held by
//!   the tests #738 left green: the count brought down, by
//!   `a_count_typed_above_a_full_turn_is_brought_down_to_it` in
//!   `copying/tests.rs`; the line under the fields and the history's, by
//!   `a_count_brought_down_names_the_copy_that_would_land_on_the_original` and
//!   `a_capped_step_says_how_many_it_lays_at_its_angle_and_how_many_were_asked`
//!   in `wording/pattern/tests.rs`; the pattern stopped at a full turn, by
//!   `crates/part/tests/a_circular_pattern_stops_at_a_full_turn.rs`

use cao_part::history::MirrorAsked;
use cao_part::history::RepeatsAsked;
use cao_part::{Formula, Operation};
use cao_prefs::Command;
use cao_sketch::{ChosenAxis, SegmentId};
use glam::DVec2;

use super::super::activate_in_the_panel;
use super::bench::Bench;
use crate::screens::sketch::{CopySlot, CopyStep, StepState};

const ON_THE_TRAIT_TO_COPY: DVec2 = DVec2::new(-10.0, 0.0);
const ON_THE_UPRIGHT: DVec2 = DVec2::new(2.0, 0.0);
const TRAIT_TO_COPY: SegmentId = SegmentId(0);
const UPRIGHT: SegmentId = SegmentId(1);

const MIRROR_STEP_1: &str = "Symétrie — étape 1 : éléments";
const MIRROR_STEP_2: &str = "Symétrie — étape 2 : axe";
const GRID_STEP_1: &str = "Réseau rectangulaire — étape 1 : éléments";
const GRID_STEP_2: &str = "Réseau rectangulaire — étape 2 : direction";
const GRID_STEP_3: &str = "Réseau rectangulaire — étape 3 : nombre et pas";

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

#[test]
fn enter_on_the_mirror_s_elements_validates_them_and_moves_on_to_its_axis() {
    let mut bench = a_trait_and_an_upright();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);
    let laid_before = bench.laid();

    bench.enter();

    assert_eq!(
        bench.active(),
        CopySlot::Target,
        "the elements held, ↵ moves on to the axis"
    );
    assert_eq!(bench.laid(), laid_before, "moving on lays nothing");
}

#[test]
fn the_mirror_waits_on_its_axis_once_the_axis_is_clicked() {
    let mut bench = a_trait_and_an_upright();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    let laid_before = bench.laid();

    bench.click(ON_THE_UPRIGHT);

    assert_eq!(
        steps(&bench),
        [
            (CopyStep::Elements, StepState::Done),
            (CopyStep::Axis, StepState::Current),
        ],
        "the axis is the last step: the click that names it leaves it current, \
         for ↵ to lay the mirror"
    );
    assert_eq!(bench.laid(), laid_before, "the click lays nothing");
}

#[test]
fn enter_on_the_mirror_s_axis_applies_the_mirror() {
    let mut bench = a_trait_and_an_upright();
    let elements = bench.a_trait_and_its_ends(TRAIT_TO_COPY);
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    bench.click(ON_THE_UPRIGHT);
    bench.click_the_slot(CopySlot::Target);
    let laid_before = bench.laid();

    bench.enter();

    assert_eq!(
        bench.laid(),
        laid_before + 1,
        "one ↵ on the axis, its last step, lays the mirror, with no Validate to \
         reach first"
    );
    assert_eq!(
        bench.last_laid(),
        Some(&Operation::HeldMirror(MirrorAsked {
            sketch: 0,
            elements,
            axis: ChosenAxis::Trait(UPRIGHT),
        }))
    );
}

#[test]
fn enter_on_the_mirror_s_axis_with_no_axis_lays_nothing_and_marks_it() {
    let mut bench = a_trait_and_an_upright();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    let laid_before = bench.laid();

    bench.enter();

    assert_eq!(bench.laid(), laid_before, "nothing is laid with no axis");
    assert_eq!(
        bench.active(),
        CopySlot::Target,
        "the axis, still empty, is asked for again"
    );
    assert_eq!(bench.panel().marked(), Some(CopySlot::Target));
}

#[test]
fn enter_validates_each_step_of_the_rectangular_pattern_and_moves_on_to_the_next() {
    let mut bench = a_trait_and_an_upright();
    bench.take(Command::ToolRectangularPattern);
    bench.click(ON_THE_TRAIT_TO_COPY);
    let laid_before = bench.laid();

    bench.enter();
    assert_eq!(
        bench.active(),
        CopySlot::Target,
        "the elements held, ↵ moves on to the direction"
    );
    bench.click(ON_THE_UPRIGHT);
    bench.click_the_slot(CopySlot::Target);
    bench.enter();
    assert_eq!(
        bench.active(),
        CopySlot::Value(0),
        "the direction held, ↵ moves on to the number and the step"
    );
    for next in 1..4 {
        bench.enter();
        assert_eq!(
            bench.active(),
            CopySlot::Value(next),
            "the numbers and the steps of both ways are fields of one step: ↵ \
             moves on to the next of them"
        );
    }
    assert_eq!(bench.laid(), laid_before, "moving on lays nothing");
}

#[test]
fn enter_on_the_rectangular_pattern_s_last_step_applies_the_pattern() {
    let mut bench = a_trait_and_an_upright();
    let elements = bench.a_trait_and_its_ends(TRAIT_TO_COPY);
    bench.take(Command::ToolRectangularPattern);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    bench.click(ON_THE_UPRIGHT);
    for _ in 0..3 {
        bench.enter();
    }
    let laid_before = bench.laid();

    bench.enter();

    assert_eq!(
        bench.laid(),
        laid_before + 1,
        "↵ on the last field of the last step lays the pattern, with no \
         Validate to reach first"
    );
    let opened = RepeatsAsked {
        step: Formula::Number(10.0),
        count: Formula::Number(2.0),
    };
    assert_eq!(
        bench.last_laid(),
        Some(&Operation::RectangularPattern {
            sketch: 0,
            elements,
            direction: ChosenAxis::Trait(UPRIGHT),
            along: opened.clone(),
            across: opened,
        }),
        "laid with the values the panel opened on"
    );
}

#[test]
fn enter_on_the_rectangular_pattern_s_last_step_with_its_direction_empty_brings_it_back_marked() {
    let mut bench = a_trait_and_an_upright();
    bench.take(Command::ToolRectangularPattern);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    bench.click_the_slot(CopySlot::Value(3));
    let laid_before = bench.laid();

    bench.enter();

    assert_eq!(
        bench.laid(),
        laid_before,
        "nothing is laid with no direction"
    );
    assert_eq!(
        bench.active(),
        CopySlot::Target,
        "the direction, the first step still empty, is asked for again"
    );
    assert_eq!(bench.panel().marked(), Some(CopySlot::Target));
}

/// What the message pill at the top of the screen reads.
fn said(bench: &Bench) -> Option<&str> {
    bench.editor.message.as_deref()
}

#[test]
fn the_message_pill_follows_the_step_of_the_mirror() {
    let mut bench = a_trait_and_an_upright();

    bench.take(Command::ToolMirror);
    assert_eq!(said(&bench), Some(MIRROR_STEP_1));
    bench.click(ON_THE_TRAIT_TO_COPY);
    assert_eq!(
        said(&bench),
        Some(MIRROR_STEP_1),
        "a trait clicked is held, and the elements stay the step asked for"
    );
    bench.enter();
    assert_eq!(said(&bench), Some(MIRROR_STEP_2));
    bench.click(ON_THE_UPRIGHT);
    assert_eq!(
        said(&bench),
        Some(MIRROR_STEP_2),
        "the axis clicked, the panel waits on it, its last step, for ↵"
    );
    bench.enter();
    assert_eq!(
        said(&bench),
        Some(MIRROR_STEP_1),
        "the mirror laid, the panel is back on its elements"
    );
}

#[test]
fn the_message_pill_follows_the_step_of_the_rectangular_pattern() {
    let mut bench = a_trait_and_an_upright();

    bench.take(Command::ToolRectangularPattern);
    assert_eq!(said(&bench), Some(GRID_STEP_1));
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    assert_eq!(said(&bench), Some(GRID_STEP_2));
    bench.click(ON_THE_UPRIGHT);
    assert_eq!(said(&bench), Some(GRID_STEP_3));
    for field in 1..4 {
        bench.enter();
        assert_eq!(
            said(&bench),
            Some(GRID_STEP_3),
            "the numbers and the steps of both ways are one step: field {field} \
             says what the first said"
        );
    }
    bench.enter();
    assert_eq!(
        said(&bench),
        Some(GRID_STEP_1),
        "the pattern laid, the panel is back on its elements"
    );
}

#[test]
fn a_step_clicked_in_the_mirror_s_or_the_rectangular_pattern_s_panel_takes_the_message_pill_with_it()
 {
    for (command, clicked) in [
        (
            Command::ToolMirror,
            vec![
                (CopySlot::Target, MIRROR_STEP_2),
                (CopySlot::Elements, MIRROR_STEP_1),
            ],
        ),
        (
            Command::ToolRectangularPattern,
            vec![
                (CopySlot::Target, GRID_STEP_2),
                (CopySlot::Value(2), GRID_STEP_3),
                (CopySlot::Elements, GRID_STEP_1),
            ],
        ),
    ] {
        let mut bench = a_trait_and_an_upright();
        bench.take(command);
        bench.click(ON_THE_TRAIT_TO_COPY);

        for (slot, step) in clicked {
            activate_in_the_panel(&mut bench.context(), slot);

            assert_eq!(
                said(&bench),
                Some(step),
                "{command:?}: {slot:?} clicked in the panel"
            );
            assert_eq!(
                bench.active(),
                slot,
                "{command:?}: the click makes that step current"
            );
        }
    }
}

/// A copy is the original reflected or moved: no solver, a few products.
const TOLERANCE: f64 = 1e-9;

fn traits(bench: &Bench) -> usize {
    bench.document.sketches()[0].segments().len()
}

/// The traits the drawing gained past the first `before`, each by where its
/// start and its end stand.
fn traits_gained(bench: &Bench, before: usize) -> Vec<(DVec2, DVec2)> {
    let sketch = &bench.document.sketches()[0];
    sketch.segments()[before..]
        .iter()
        .map(|side| (sketch.point(side.start), sketch.point(side.end)))
        .collect()
}

fn stand_at(gained: &[(DVec2, DVec2)], expected: &[(DVec2, DVec2)]) -> bool {
    gained.len() == expected.len()
        && gained
            .iter()
            .zip(expected)
            .all(|((start, end), (from, to))| {
                (*start - *from).length() < TOLERANCE && (*end - *to).length() < TOLERANCE
            })
}

#[test]
fn the_mirror_laid_through_its_steps_is_the_one_laid_before() {
    let mut bench = a_trait_and_an_upright();
    let elements = bench.a_trait_and_its_ends(TRAIT_TO_COPY);
    let before = traits(&bench);
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    bench.click(ON_THE_UPRIGHT);

    bench.enter();

    assert_eq!(
        bench.last_laid(),
        Some(&Operation::HeldMirror(MirrorAsked {
            sketch: 0,
            elements,
            axis: ChosenAxis::Trait(UPRIGHT),
        })),
        "what the mirror named from its slots before #739, laid held since #320"
    );
    let gained = traits_gained(&bench, before);
    assert!(
        stand_at(&gained, &[(DVec2::new(14.0, -5.0), DVec2::new(14.0, 5.0))]),
        "the trait at x = -10, reflected across the upright at x = 2, stands at \
         x = 14, end for end: {gained:?}"
    );
}

#[test]
fn the_rectangular_pattern_laid_through_its_steps_is_the_one_laid_before() {
    let mut bench = a_trait_and_an_upright();
    let elements = bench.a_trait_and_its_ends(TRAIT_TO_COPY);
    let before = traits(&bench);
    bench.take(Command::ToolRectangularPattern);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    bench.click(ON_THE_UPRIGHT);
    for _ in 0..3 {
        bench.enter();
    }

    bench.enter();

    let opened = RepeatsAsked {
        step: Formula::Number(10.0),
        count: Formula::Number(2.0),
    };
    assert_eq!(
        bench.last_laid(),
        Some(&Operation::RectangularPattern {
            sketch: 0,
            elements,
            direction: ChosenAxis::Trait(UPRIGHT),
            along: opened.clone(),
            across: opened,
        }),
        "the operation the rectangular pattern laid from its slots before #739, \
         with the values its fields opened on"
    );
    let gained = traits_gained(&bench, before);
    assert!(
        stand_at(
            &gained,
            &[
                (DVec2::new(-10.0, 5.0), DVec2::new(-10.0, 15.0)),
                (DVec2::new(-20.0, -5.0), DVec2::new(-20.0, 5.0)),
                (DVec2::new(-20.0, 5.0), DVec2::new(-20.0, 15.0)),
            ]
        ),
        "two by two, ten apart: one copy up the upright's way, one a quarter \
         turn to its left, and one both: {gained:?}"
    );
}
