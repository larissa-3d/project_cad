//! The drawing of the view bar, through the `ui/` primitives.

use crate::screens::ribbon::icons;
use crate::ui::floating_bar::{
    BarForm, bar_button, bar_button_words, bar_menu, bar_menu_words, bar_reading,
    bar_reading_width, bar_separator, floating_bar,
};

use super::state::{Offered, ViewBar};
use crate::commands::Asked;

/// Draws the bar against the bottom right corner of `view`, and says what was
/// picked on it.
pub fn show(ctx: &egui::Context, view: egui::Rect, bar: &ViewBar) -> Option<Asked> {
    floating_bar(ctx, egui::Id::new("view_bar"), view, |ui| {
        let form = form(ui, view, bar);
        let mut picked = None;
        for offered in [&bar.reframe, &bar.section_view] {
            if offered_button(ui, offered, form.is_worded()).clicked() {
                picked = Some(Asked::Command(offered.command));
            }
        }
        let menu = &bar.matter_in_front;
        let entries: Vec<(&str, bool)> = menu
            .choices
            .iter()
            .map(|choice| (choice.label.as_str(), choice.current))
            .collect();
        let worn = (menu.icon, menu.label.as_str());
        if let Some(rank) = bar_menu(ui, menu.enabled, worn, &entries, form.is_worded()) {
            picked = Some(Asked::MatterInFront(menu.choices[rank].state));
        }
        if form.reads() {
            bar_separator(ui);
            bar_reading(ui, &bar.coordinates, &bar.widest_coordinates);
        }
        picked
    })
}

fn form(ui: &egui::Ui, view: egui::Rect, bar: &ViewBar) -> BarForm {
    let words = [&bar.reframe, &bar.section_view]
        .iter()
        .map(|offered| bar_button_words(ui, &offered.name, offered.key.as_deref()))
        .sum::<f32>()
        + bar_menu_words(ui, &bar.matter_in_front.label);
    let reading = bar_reading_width(ui, &bar.coordinates, &bar.widest_coordinates);
    BarForm::fitting(view, 3, words, reading)
}

/// A command offered on a floating row, worn as the ribbon wears it, or by
/// its icon alone with its name and key said under the pointer ahead of its
/// hint.
pub(crate) fn offered_button(ui: &mut egui::Ui, offered: &Offered, worded: bool) -> egui::Response {
    let worn = icons::of(offered.command);
    let said = (offered.name.as_str(), offered.key.as_deref());
    let response = bar_button(ui, offered.enabled, offered.on, worn, said, worded);
    if worded {
        return response.on_hover_text(&offered.hint);
    }
    let named = match &offered.key {
        Some(key) => format!("{} {key}", offered.name),
        None => offered.name.clone(),
    };
    response.on_hover_text(format!("{named}\n{}", offered.hint))
}
