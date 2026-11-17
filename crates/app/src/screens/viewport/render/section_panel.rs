//! The section view's panel, laid out in numbered steps in `steps`, and what
//! the message pill says while it is open — and, on the cutting plane, its
//! square, its arrow and its handle.

use cao_prefs::Command;
use cao_render::{push_plane_outline, push_plane_quad};

use super::overlays::{tint_to_color, to_screen};
use super::{tint, tint_at};
use crate::screens::SketchContext;
use crate::screens::viewport::section::SectionField;
use crate::screens::viewport::{ViewScale, ViewportState};
use crate::ui::floating_panel::{TOOL_PANEL_WIDTH, tool_panel};
use crate::ui::theme::tokens::SPACING_XXL;

/// How far in from the view's top left corner the panel stands.
const INSET: f32 = 16.0;

mod steps;

/// Draws the panel while the section is on, carries out what was done in it,
/// and says its current step over the view. Échap, the control in its title
/// or the section kept closes it, and with it the section, and the step it
/// said goes with it. Returns true when the section was kept in the part.
pub(crate) fn paint_section_panel(
    ui: &egui::Ui,
    state: &mut ViewportState,
    view: egui::Rect,
    context: &mut SketchContext<'_>,
) -> bool {
    let escaped = ui.input(|input| input.key_pressed(egui::Key::Escape))
        && !crate::ui::formula_field::keeps_escape(ui.ctx());
    if escaped {
        state.section = None;
    }
    let id = egui::Id::new("section_panel");
    let Some(section) = state.section.as_mut() else {
        unsay(ui.ctx(), id, context);
        return false;
    };
    let lang = context.lang;
    let title = crate::wording::command::label(lang, Command::SectionView);
    let left_at = view.min + egui::vec2(INSET, INSET);
    let close = lang.t("panel.close");
    let part = context.document.name().to_owned();
    let field = section.field();
    let arrived = ui.ctx().data_mut(|data| {
        let before = data.get_temp::<SectionField>(id.with("field"));
        data.insert_temp(id.with("field"), field);
        before != Some(field)
    });
    let heading = (title.as_str(), part.as_str(), close.as_str());
    let floated = tool_panel(ui.ctx(), id, heading, (left_at, view), |ui| {
        steps::steps(ui, section, context, arrived)
    });
    if floated.closed {
        state.section = None;
        unsay(ui.ctx(), id, context);
        return false;
    }
    let asked = floated.inner;
    if asked.keep {
        section.keep_in(context.document);
    }
    // A sketch edited under the section has the message and `Entrée` to
    // itself.
    let sketching = context.editor.active_sketch().is_some();
    let bare_enter = !sketching && ui.input(|input| input.key_pressed(egui::Key::Enter));
    let entered = !sketching && (asked.enter || bare_enter);
    let kept = asked.keep || (entered && section.enter(context.document));
    // A section kept, its panel closes as Échap closes it.
    if kept {
        state.section = None;
        unsay(ui.ctx(), id, context);
        return true;
    }
    if sketching {
        unsay(ui.ctx(), id, context);
        return false;
    }
    let said = section.said(lang);
    ui.ctx()
        .data_mut(|data| data.insert_temp(id.with("said"), said.clone()));
    context.editor.message = Some(said);
    kept
}

/// Where a panel open beside the section view's stands: to its right, the
/// section's being a tool's panel `TOOL_PANEL_WIDTH` across.
pub(crate) fn right_of_the_section_panel(view: egui::Rect) -> egui::Pos2 {
    view.min + egui::vec2(INSET + TOOL_PANEL_WIDTH + SPACING_XXL, INSET)
}

/// Takes back the step the section said over the view, while that is still
/// what the view says.
fn unsay(ctx: &egui::Context, id: egui::Id, context: &mut SketchContext<'_>) {
    let said = ctx.data_mut(|data| data.remove_temp::<String>(id.with("said")));
    if said.is_some() && context.editor.message == said {
        context.editor.message = None;
    }
}

/// The handle: a dot on the cutting plane over the middle of the part.
pub(crate) fn paint_section_handle(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
) {
    let Some(section) = state.section() else {
        return;
    };
    let handle = section.handle(context.document.body());
    let view_projection = state.camera.view_projection(rect.width() / rect.height());
    if let Some(at) = to_screen(handle, view_projection, rect) {
        let color = tint_to_color(state.theme.highlight);
        ui.painter_at(rect).circle_filled(at, 6.0, color);
    }
}

/// While the section tool is open, the cutting plane as a see-through square
/// centred on the handle, outlined, and the arrow the way it faces, both the
/// same size on screen at any zoom.
pub(super) fn push_section_plane(
    surfaces: &mut Vec<cao_render::Vertex>,
    lines: &mut Vec<cao_render::Vertex>,
    state: &ViewportState,
    scale: ViewScale,
    context: &SketchContext<'_>,
) {
    let Some(section) = state.section() else {
        return;
    };
    let cut = section.cutting_plane();
    let centre = section.handle(context.document.body());
    let (u, v) = (cut.u.as_vec3(), cut.v.as_vec3());
    let half_size = scale.world_size_of(SQUARE_PIXELS) as f32;
    let highlight = state.theme.highlight;
    push_plane_quad(
        surfaces,
        centre.as_vec3(),
        u,
        v,
        half_size,
        tint_at(highlight, 0.15),
    );
    let outline = tint(highlight);
    push_plane_outline(lines, centre.as_vec3(), u, v, half_size, outline, 1.5);
    let facing = section.facing();
    let length = scale.world_size_of(ARROW_PIXELS);
    let tip = centre + facing * length;
    let barb = |side: glam::DVec3| tip + (side * 0.5 - facing) * length * 0.2;
    for (from, to) in [(centre, tip), (tip, barb(cut.u)), (tip, barb(-cut.u))] {
        lines.push(cao_render::Vertex::line(from.as_vec3(), outline, 2.0));
        lines.push(cao_render::Vertex::line(to.as_vec3(), outline, 2.0));
    }
}

const SQUARE_PIXELS: f64 = 80.0;
const ARROW_PIXELS: f64 = 60.0;
