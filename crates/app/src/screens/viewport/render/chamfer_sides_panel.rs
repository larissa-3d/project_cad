//! « Inverser les côtés » in a small panel of its own over the view, at its
//! top left as the step panels stand, while a chamfer of the sketch whose two
//! sides can trade places is held.

use cao_prefs::Command;

use super::step_panel::foot;
use crate::screens::SketchContext;
use crate::screens::viewport::{chamfer_to_swap, swap_held_chamfer};
use crate::ui::floating_panel::tool_panel;
use crate::ui::primary_button::primary_button;
use crate::ui::theme::tokens::SPACING_XXL;
use crate::wording::command::label;

/// Draws the panel while such a chamfer is held, and swaps its sides when its
/// button is clicked; the control in its title lets go of the chamfer. True
/// when the sides were swapped.
pub(crate) fn paint_chamfer_sides_panel(
    ui: &egui::Ui,
    view: egui::Rect,
    context: &mut SketchContext<'_>,
) -> bool {
    if chamfer_to_swap(context.document, context.editor).is_none() {
        return false;
    }
    let lang = context.lang;
    let title = label(lang, Command::ToolChamfer);
    let sketch = context.editor.active_sketch().unwrap_or_default();
    let named = crate::wording::sketch::name(lang, context.document, sketch);
    let close = lang.t("panel.close");
    let left_at = view.min + egui::vec2(SPACING_XXL, SPACING_XXL);
    let id = egui::Id::new("chamfer_sides_panel");
    let heading = (title.as_str(), named.as_str(), close.as_str());
    let floated = tool_panel(ui.ctx(), id, heading, (left_at, view), |ui| {
        let mut swap = false;
        foot(ui, |ui| {
            swap = primary_button(ui, &label(lang, Command::SwapChamferSides)).clicked();
        });
        swap
    });
    if floated.closed
        && let Some(select) = context.editor.select_state()
    {
        select.held.clear();
    }
    floated.inner && swap_held_chamfer(context)
}
