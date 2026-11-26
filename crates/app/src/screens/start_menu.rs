use std::path::PathBuf;

use cao_prefs::RecentEntry;

use crate::adapters::clock;
use crate::lang::Catalogue;
use crate::screens::explorer::Explorer;
use crate::screens::new_card::{self, NewCard};

pub enum StartMenuAction {
    None,
    /// A part of this name, in the folder the card shows.
    CreateNew {
        name: String,
        folder: PathBuf,
    },
    Open(PathBuf),
}

pub fn show(
    ui: &mut egui::Ui,
    recents: &[RecentEntry],
    card: &mut NewCard,
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
            if recents.is_empty() {
                ui.weak(lang.t("start_menu.no_recents"));
            } else {
                let zone = clock::reader_zone();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for entry in recents {
                        ui.horizontal(|ui| {
                            if ui.button(&entry.name).clicked() {
                                action = StartMenuAction::Open(entry.path.clone());
                            }
                            ui.weak(lang.t_moment(
                                "start_menu.recent_date",
                                entry.opened_at,
                                &zone,
                            ));
                        });
                    }
                });
            }
        });
    });

    action
}
