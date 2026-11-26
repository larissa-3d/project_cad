//! What stands where a list has nothing to show: a dashed frame round an icon,
//! what is missing, and how to fill it, centred.

use egui::{FontId, Frame, Margin, Response, RichText, Sense, Stroke, Ui};

use crate::ui::dashed::dashed_border;
use crate::ui::icon::{Icon, IconSize};
use crate::ui::theme::tokens::{
    BORDER_STRONG, RADIUS_CARD, SPACING_L, SPACING_XXL, TEXT_QUIET, TEXT_SUBDUED,
    TYPOGRAPHY_SIZES_DETAIL,
};

/// The hint is painted rather than laid as a label: it names a button, and a
/// label reading that name would be found by whoever looks for the button by
/// it.
pub fn empty_state(ui: &mut Ui, icon: Icon, missing: &str, hint: &str) -> Response {
    let framed = Frame::new()
        .inner_margin(Margin::same(SPACING_XXL as i8))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(icon.image(IconSize::Large).tint(TEXT_QUIET));
                ui.add_space(SPACING_L);
                ui.label(RichText::new(missing).color(TEXT_SUBDUED));
                let written = ui.painter().layout(
                    hint.to_owned(),
                    FontId::proportional(TYPOGRAPHY_SIZES_DETAIL),
                    TEXT_QUIET,
                    ui.available_width(),
                );
                let (rect, _) = ui.allocate_exact_size(written.size(), Sense::hover());
                ui.painter().galley(rect.min, written, TEXT_QUIET);
            });
        });
    dashed_border(
        ui.painter(),
        framed.response.rect,
        f32::from(RADIUS_CARD),
        Stroke::new(1.0, BORDER_STRONG),
    );
    framed.response
}
