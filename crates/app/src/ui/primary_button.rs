use egui::{Button, Response, RichText, Ui, vec2};

use crate::ui::theme::tokens::{ACCENT_AMBER, ACCENT_AMBER_LIGHT, RADIUS_PILL, TEXT_ON_ACCENT};

/// How tall the button stands, in points: `docs/redesign/source/part.html`,
/// which the tokens do not carry.
pub const HEIGHT: f32 = 28.0;

/// The one action a place leads to, in amber with round ends, the key that
/// does it recalled in its label (`docs/redesign/components.md` C15). Under
/// the pointer and while pressed it turns a lighter amber; one that cannot be
/// pressed stays amber whatever the pointer does.
pub fn primary_button(ui: &mut Ui, label: &str) -> Response {
    ui.scope(|ui| {
        // A fill handed to the button would hold in every state: the look of
        // each state is where egui lets the pointer change it.
        let answering = match ui.is_enabled() {
            true => ACCENT_AMBER_LIGHT,
            false => ACCENT_AMBER,
        };
        let widgets = &mut ui.visuals_mut().widgets;
        widgets.noninteractive.weak_bg_fill = ACCENT_AMBER;
        widgets.inactive.weak_bg_fill = ACCENT_AMBER;
        widgets.hovered.weak_bg_fill = answering;
        widgets.active.weak_bg_fill = answering;
        ui.add(
            Button::new(RichText::new(label).color(TEXT_ON_ACCENT))
                .corner_radius(RADIUS_PILL)
                .min_size(vec2(0.0, HEIGHT)),
        )
    })
    .inner
}

#[cfg(test)]
mod tests;
