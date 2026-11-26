//! What stands where a list has nothing to show: a dashed frame round an icon,
//! what is missing, and how to fill it, centred.

use egui::{Frame, Label, Margin, Response, RichText, Stroke, Ui};

use crate::ui::dashed::dashed_border;
use crate::ui::icon::{Icon, IconSize};
use crate::ui::theme::tokens::{
    BORDER_STRONG, RADIUS_CARD, SPACING_L, SPACING_XXL, TEXT_QUIET, TEXT_SUBDUED,
    TYPOGRAPHY_SIZES_DETAIL,
};

pub fn empty_state(ui: &mut Ui, icon: Icon, missing: &str, hint: &str) -> Response {
    let framed = Frame::new()
        .inner_margin(Margin::same(SPACING_XXL as i8))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(icon.image(IconSize::Large).tint(TEXT_QUIET));
                ui.add_space(SPACING_L);
                ui.add(Label::new(RichText::new(missing).color(TEXT_SUBDUED)).selectable(false));
                let hint = RichText::new(hint)
                    .size(TYPOGRAPHY_SIZES_DETAIL)
                    .color(TEXT_QUIET);
                ui.add(Label::new(hint).selectable(false));
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
