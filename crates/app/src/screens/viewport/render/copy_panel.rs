//! The panel of the mirror and the two patterns: the tool's numbered steps
//! under its title, laid out in `steps`, the fields of a pattern's number and
//! step in `values`.

use super::super::input::copying::{activate_in_the_panel, command_of, enter_in_the_panel};
use crate::screens::SketchContext;
use crate::screens::sketch::{CopySlot, Rotation};
use crate::ui::floating_panel::tool_panel;
use crate::ui::theme::tokens::SPACING_XXL;

mod steps;
mod values;

/// What a gesture in the panel asked for this frame.
#[derive(Default)]
struct Asked {
    activate: Option<CopySlot>,
    enter: bool,
    cancel: bool,
    rotation: Option<Rotation>,
}

/// Draws the panel while a copy tool is in hand, and carries out what was done
/// in it: whether a copy was laid, and where the panel was let go this frame.
pub(crate) fn paint_copy_panel(
    ui: &egui::Ui,
    left_at: Option<[f32; 2]>,
    view: egui::Rect,
    context: &mut SketchContext<'_>,
) -> (bool, Option<[f32; 2]>) {
    let Some(active) = context.editor.copy_panel().map(|panel| panel.active()) else {
        return (false, None);
    };
    let title = crate::wording::command::label(context.lang, command_of(context.editor.tool));
    let fallback = view.min + egui::vec2(SPACING_XXL, SPACING_XXL);
    let left_at = left_at.map_or(fallback, |[x, y]| egui::pos2(x, y));
    let id = egui::Id::new("copy_panel");
    let arrived = ui.ctx().data_mut(|data| {
        let before = data.get_temp::<CopySlot>(id.with("active"));
        data.insert_temp(id.with("active"), active);
        before != Some(active)
    });
    let close = context.lang.t("panel.close");
    let sketch = context.editor.active_sketch().unwrap_or_default();
    let named = crate::wording::sketch::name(context.lang, context.document, sketch);
    let heading = (title.as_str(), named.as_str(), close.as_str());
    let floated = tool_panel(ui.ctx(), id, heading, (left_at, view), |ui| {
        steps::steps(ui, context, arrived)
    });
    let let_go_at = floated.let_go_at.map(|at| [at.x, at.y]);

    let asked = floated.inner;
    if asked.cancel || floated.closed {
        context.editor.cancel_the_copy();
        return (false, let_go_at);
    }
    if let (Some(rotation), Some(panel)) = (asked.rotation, context.editor.copy_panel_mut()) {
        panel.choose_rotation(rotation);
    }
    if let Some(slot) = asked.activate {
        activate_in_the_panel(context, slot);
    }
    let bare_enter = ui.input(|input| input.key_pressed(egui::Key::Enter));
    let laid = (asked.enter || bare_enter) && enter_in_the_panel(context);
    (laid, let_go_at)
}
