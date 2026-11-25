use std::path::PathBuf;

use cao_prefs::RecentEntry;

use crate::adapters::clock;
use crate::lang::Catalogue;
use crate::screens::explorer::Explorer;
use crate::wording;

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
    new_part_name: &mut String,
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
            ui.heading(lang.t("start_menu.new_part"));
            ui.add_space(8.0);
            let entered = ui
                .horizontal(|ui| {
                    ui.label(lang.t("start_menu.name"));
                    // A single-line field gives the keyboard up on ↵, the frame it is pressed.
                    let field = ui.text_edit_singleline(new_part_name);
                    field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter))
                })
                .inner;
            ui.add_space(8.0);
            ui.label(lang.t("start_menu.saves_in"));
            let (root, chosen) = (&explorer.library().path, explorer.chosen_folder());
            ui.label(wording::library::folder(lang, root, &chosen));
            ui.weak(lang.t("start_menu.saves_in_by_default"));
            ui.add_space(8.0);
            let can_create = !new_part_name.trim().is_empty();
            let clicked = ui
                .add_enabled(can_create, egui::Button::new(lang.t("start_menu.create")))
                .clicked();
            if can_create && (clicked || entered) {
                action = StartMenuAction::CreateNew {
                    name: new_part_name.trim().to_string(),
                    folder: chosen,
                };
                new_part_name.clear();
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
