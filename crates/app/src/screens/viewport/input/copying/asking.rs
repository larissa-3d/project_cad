//! What the drawing says the panel waits on: the tool, the number of its
//! current step, and what that step asks for.

use cao_prefs::Command;

use crate::screens::SketchContext;
use crate::screens::sketch::{CopySlot, CopyStep, Tool};
use crate::wording::copy_panel::{self as said, Asked};

/// A slot clicked in the panel becomes active, and when it belongs to a step
/// the drawing says what that step asks for.
pub(crate) fn activate_in_the_panel(context: &mut SketchContext<'_>, slot: CopySlot) {
    let Some(panel) = context.editor.copy_panel_mut() else {
        return;
    };
    panel.activate(slot);
    if panel.current_step().is_some() {
        say_what_is_asked(context);
    }
}

/// What the drawing says while the panel waits on its active slot: nothing on
/// Validate, which belongs to no step.
pub(super) fn say_what_is_asked(context: &mut SketchContext<'_>) {
    let Some(panel) = context.editor.copy_panel() else {
        return;
    };
    let tool = command_of(context.editor.tool);
    context.editor.message = panel
        .current_step()
        .map(|(rank, step)| said::step_asks(context.lang, tool, rank + 1, asked(step)));
}

/// The command the copy tool in hand is taken by, which names it.
pub(crate) fn command_of(tool: Tool) -> Command {
    match tool {
        Tool::CircularPattern => Command::ToolCircularPattern,
        Tool::RectangularPattern => Command::ToolRectangularPattern,
        _ => Command::ToolMirror,
    }
}

fn asked(step: CopyStep) -> Asked {
    match step {
        CopyStep::Elements => Asked::Elements,
        CopyStep::Centre => Asked::Centre,
        CopyStep::Axis => Asked::Axis,
        CopyStep::Direction => Asked::Direction,
        CopyStep::CountAndStep => Asked::CountAndStep,
        CopyStep::Rotation => Asked::Rotation,
    }
}
