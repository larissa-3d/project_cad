//! What the drawing says the panel waits on: the circular pattern names its
//! current step, the mirror and the rectangular pattern their active slot.

use crate::screens::SketchContext;
use crate::screens::sketch::{CopySlot, CopyStep, Tool};
use crate::wording::copy_panel::{self as said, Asked};

/// A slot clicked in the panel becomes active, and in the circular pattern's
/// the drawing says what its step asks for. The mirror's and the rectangular
/// pattern's leave the message as it was.
pub(crate) fn activate_in_the_panel(context: &mut SketchContext<'_>, slot: CopySlot) {
    let Some(panel) = context.editor.copy_panel_mut() else {
        return;
    };
    panel.activate(slot);
    if context.editor.tool == Tool::CircularPattern {
        say_what_is_asked(context);
    }
}

/// What the drawing says while the panel waits on its active slot.
pub(super) fn say_what_is_asked(context: &mut SketchContext<'_>) {
    let Some(panel) = context.editor.copy_panel() else {
        return;
    };
    if context.editor.tool == Tool::CircularPattern
        && let Some((rank, step)) = panel.current_step()
        && let Some(asked) = asked_in_the_circular_pattern(step)
    {
        context.editor.message = Some(said::circular_step(context.lang, rank + 1, asked));
        return;
    }
    let said = match (panel.active(), context.editor.tool) {
        (CopySlot::Elements, Tool::Mirror) => "sketch.mirror_take_elements",
        (CopySlot::Elements, _) => "sketch.pattern_take_elements",
        (CopySlot::Target, Tool::RectangularPattern) => "sketch.pattern_click_the_direction",
        (CopySlot::Target, _) => "sketch.mirror_click_the_axis",
        _ => {
            context.editor.message = None;
            return;
        }
    };
    context.editor.message = Some(context.lang.t(said));
}

/// What a step asks for, among the circular pattern's: an axis and a
/// direction are none of its steps.
fn asked_in_the_circular_pattern(step: CopyStep) -> Option<Asked> {
    match step {
        CopyStep::Elements => Some(Asked::Elements),
        CopyStep::Centre => Some(Asked::Centre),
        CopyStep::CountAndStep => Some(Asked::Count),
        CopyStep::Rotation => Some(Asked::Rotation),
        CopyStep::Axis | CopyStep::Direction => None,
    }
}
