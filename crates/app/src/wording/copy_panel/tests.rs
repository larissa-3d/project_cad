//! What app · wording/copy_panel.rs is held to.
//!
//! Closes #739 — the whole transcription is in
//! `crates/app/src/screens/viewport/input/copying/tests/mirror_and_rectangular_steps.rs`;
//! this is how the mirror's step names the axis it holds.
//! - Both tools are laid out by `tool_panel` in numbered steps; the `slots`
//!   and `panel_row` rows of `render/copy_panel.rs` are gone. The controls of
//!   `ui/` already there (`step_dot`, `chip`, `number_field`,
//!   `formula_field`) are reused; none is added — found in review: the axis
//!   step names what it holds, a trait by its row's name and each of the
//!   sketch's two axes by its own —
//!   `the_axis_held_is_named_a_trait_by_its_row_and_each_sketch_axis_by_its_own`

use cao_sketch::SegmentId;

use super::*;

#[test]
fn the_axis_held_is_named_a_trait_by_its_row_and_each_sketch_axis_by_its_own() {
    let lang = Catalogue::french();

    assert_eq!(axis(&lang, ChosenAxis::Trait(SegmentId(2))), "Trait 3");
    assert_eq!(
        axis(&lang, ChosenAxis::Sketch(SketchAxis::U)),
        "Axe horizontal",
        "the sketch's first axis runs along its u, across the drawing"
    );
    assert_eq!(
        axis(&lang, ChosenAxis::Sketch(SketchAxis::V)),
        "Axe vertical",
        "the sketch's second axis runs along its v, up the drawing"
    );
}
