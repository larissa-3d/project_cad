use egui::vec2;

use crate::lang::Catalogue;
use crate::screens::explorer::Explorer;
use crate::ui::icon::FOLDER;
use crate::ui::secondary_button::secondary_button;
use crate::ui::shown_field::{self, shown_field};
use crate::ui::tool_button::{menu_row, ribbon_menu};
use crate::wording;

use super::state::{NewCard, folders_offered};

/// How wide « Choisir… » stands beside the field.
const CHOOSE_WIDTH: f32 = 90.0;

/// How far « Choisir… » sets a folder in from the one holding it.
const INDENT: f32 = 16.0;

/// « Enregistrer dans » : the folder a part made now goes in, « Choisir… »
/// beside it opening every folder of the library to pick from, and what the
/// card falls back on.
pub fn saves_in(ui: &mut egui::Ui, card: &mut NewCard, explorer: &Explorer, lang: &Catalogue) {
    ui.label(lang.t("start_menu.saves_in"));
    let root = &explorer.library().path;
    let folder = card.folder(explorer);
    let mut picked = None;
    ui.horizontal(|ui| {
        let width = ui.available_width() - CHOOSE_WIDTH - ui.spacing().item_spacing.x;
        let shown = wording::library::folder(lang, root, &folder);
        shown_field(ui, FOLDER, &shown, width);
        let choose = lang.t("start_menu.choose");
        let room = vec2(CHOOSE_WIDTH, shown_field::HEIGHT);
        let choose = ui.add_sized(room, |ui: &mut egui::Ui| {
            secondary_button(ui, true, &choose)
        });
        ribbon_menu(&choose, |ui| {
            for (depth, offered) in folders_offered(explorer.library()) {
                let name = match depth {
                    0 => wording::library::parts_folder(lang),
                    _ => offered.name.clone(),
                };
                let current = offered.path == folder;
                ui.horizontal(|ui| {
                    ui.add_space(depth as f32 * INDENT);
                    if menu_row(ui, true, current, FOLDER, &name, None, None).clicked() {
                        picked = Some(offered.path.clone());
                    }
                });
            }
        });
    });
    if let Some(folder) = picked {
        card.pick(folder, explorer);
    }
    ui.weak(lang.t("start_menu.saves_in_by_default"));
}
