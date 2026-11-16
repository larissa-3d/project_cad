//! The drawing of the view bar, through the `ui/` primitives.

use crate::screens::ribbon::icons;
use crate::ui::floating_bar::{
    BarForm, bar_button, bar_button_words, bar_menu, bar_menu_words, bar_reading,
    bar_reading_width, bar_separator, floating_bar,
};

use super::state::{MatterInFrontMenu, Offered, ViewBar};
use crate::commands::Asked;

/// Draws the bar against the bottom right corner of `view`, and says what was
/// picked on it.
pub fn show(ctx: &egui::Context, view: egui::Rect, bar: &ViewBar) -> Option<Asked> {
    floating_bar(ctx, egui::Id::new("view_bar"), view, |ui| {
        let held = held(bar);
        let form = form(ui, view, bar, &held);
        let mut picked = None;
        for button in held {
            picked = button.show(ui, form.is_worded()).or(picked);
        }
        if form.reads() {
            bar_separator(ui);
            bar_reading(ui, &bar.coordinates, &bar.widest_coordinates);
        }
        picked
    })
}

/// The buttons of the bar, left to right: what is drawn and what is measured.
fn held(bar: &ViewBar) -> [Held<'_>; 3] {
    [
        Held::Command(&bar.reframe),
        Held::Command(&bar.section_view),
        Held::Menu(&bar.matter_in_front),
    ]
}

fn form(ui: &egui::Ui, view: egui::Rect, bar: &ViewBar, held: &[Held<'_>]) -> BarForm {
    let words = held.iter().map(|button| button.words(ui)).sum();
    let reading = bar_reading_width(ui, &bar.coordinates, &bar.widest_coordinates);
    BarForm::fitting(view, held.len(), words, reading)
}

enum Held<'a> {
    Command(&'a Offered),
    Menu(&'a MatterInFrontMenu),
}

impl Held<'_> {
    fn words(&self, ui: &egui::Ui) -> f32 {
        match self {
            Self::Command(offered) => bar_button_words(ui, &offered.name, offered.key.as_deref()),
            Self::Menu(menu) => bar_menu_words(ui, &menu.label),
        }
    }

    fn show(&self, ui: &mut egui::Ui, worded: bool) -> Option<Asked> {
        match self {
            Self::Command(offered) => offered_button(ui, offered, worded)
                .clicked()
                .then_some(Asked::Command(offered.command)),
            Self::Menu(menu) => {
                let entries: Vec<(&str, bool)> = menu
                    .choices
                    .iter()
                    .map(|choice| (choice.label.as_str(), choice.current))
                    .collect();
                let worn = (menu.icon, menu.label.as_str());
                bar_menu(ui, menu.enabled, worn, &entries, worded)
                    .map(|rank| Asked::MatterInFront(menu.choices[rank].state))
            }
        }
    }
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
