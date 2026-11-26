use super::StartMenuAction;
use super::state::RecentLine;
use crate::lang::Catalogue;
use crate::screens::explorer::Explorer;
use crate::screens::new_card::{self, NewCard};

pub fn show(
    ui: &mut egui::Ui,
    card: &mut NewCard,
    lines: &[RecentLine],
    explorer: &Explorer,
    lang: &Catalogue,
) -> StartMenuAction {
    let mut action = StartMenuAction::None;

    ui.vertical_centered(|ui| {
        ui.add_space(24.0);
        ui.heading(lang.t("start_menu.title"));
        ui.label(lang.t("start_menu.subtitle"));
        ui.add_space(24.0);
    });

    ui.separator();

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            if let Some((name, folder)) = new_card::show(ui, card, explorer, lang) {
                action = StartMenuAction::CreateNew { name, folder };
            }
        });

        columns[1].vertical(|ui| {
            ui.heading(lang.t("start_menu.recents"));
            ui.add_space(8.0);
            if lines.is_empty() {
                ui.weak(lang.t("start_menu.no_recents"));
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for line in lines {
                        if recent_line(ui, line) {
                            action = StartMenuAction::Open(line.path.clone());
                        }
                    }
                });
            }
        });
    });

    action
}

/// The name over where the part lies and what it holds, the hour it was
/// opened and a chevron on the right. Says whether the name was clicked.
fn recent_line(ui: &mut egui::Ui, line: &RecentLine) -> bool {
    ui.horizontal(|ui| {
        let clicked = ui
            .vertical(|ui| {
                let clicked = ui.button(&line.name).clicked();
                ui.weak(&line.detail);
                clicked
            })
            .inner;
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.weak("›");
            ui.monospace(&line.opened);
        });
        clicked
    })
    .inner
}
