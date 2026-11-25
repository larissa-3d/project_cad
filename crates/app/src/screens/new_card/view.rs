use std::path::PathBuf;

use egui::vec2;

use crate::lang::Catalogue;
use crate::screens::explorer::Explorer;
use crate::ui::icon::FOLDER;
use crate::ui::primary_button::primary_button;
use crate::ui::secondary_button::secondary_button;
use crate::ui::segmented::{Segment, segmented};
use crate::ui::shown_field::{self, shown_field};
use crate::ui::text_edit::tall_text_edit;
use crate::ui::tool_button::{menu_row, ribbon_menu};
use crate::wording;

use super::state::{NewCard, folders_offered};

/// How tall « Créer ↵ » stands across the card: the redesign's 44 px, which
/// the tokens do not carry.
const CREATE_HEIGHT: f32 = 44.0;

/// How wide « Choisir… » stands beside the field.
const CHOOSE_WIDTH: f32 = 90.0;

/// How far « Choisir… » sets a folder in from the one holding it.
const INDENT: f32 = 16.0;

/// What the list of folders leaves clear above the window's foot: the menu's
/// margin and border, and a gap.
const BELOW_THE_LIST: f32 = 16.0;

/// The least the list of folders is given, a few rows, however little of the
/// window is left under « Choisir… ».
const LIST_AT_LEAST: f32 = 96.0;

/// « Pièce », the first segment of « Type » and the one kind the card makes.
const PART: usize = 0;

/// The « Nouveau » card: what it makes, the name, the folder the part goes
/// in, and « Créer ↵ ». Hands over the part to make once the button or ↵ in
/// the name field asks for it.
pub fn show(
    ui: &mut egui::Ui,
    card: &mut NewCard,
    explorer: &Explorer,
    lang: &Catalogue,
) -> Option<(String, PathBuf)> {
    ui.heading(lang.t("start_menu.new"));
    ui.add_space(8.0);
    kinds(ui, lang);
    ui.add_space(8.0);
    let entered = ui
        .horizontal(|ui| {
            ui.label(lang.t("start_menu.name"));
            // A single-line field gives the keyboard up on ↵, the frame it is pressed.
            let field = tall_text_edit(ui, card.name_mut(), shown_field::HEIGHT);
            field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter))
        })
        .inner;
    ui.add_space(8.0);
    saves_in(ui, card, explorer, lang);
    ui.add_space(8.0);
    let room = vec2(ui.available_width(), CREATE_HEIGHT);
    let create = lang.t("start_menu.create");
    let clicked = ui
        .add_enabled_ui(card.can_make(), |ui| {
            ui.add_sized(room, |ui: &mut egui::Ui| primary_button(ui, &create))
        })
        .inner
        .clicked();
    if clicked || entered {
        card.make(explorer)
    } else {
        None
    }
}

/// What the card can make: a part, and an assembly shown greyed until there
/// are assemblies to make. A click can only land on « Pièce », already
/// chosen, so what the selector hands back changes nothing.
fn kinds(ui: &mut egui::Ui, lang: &Catalogue) {
    ui.label(lang.t("start_menu.kind"));
    let (part, assembly, soon) = (
        lang.t("start_menu.part"),
        lang.t("start_menu.assembly"),
        lang.t("start_menu.soon"),
    );
    let offered = [
        Segment {
            label: &part,
            enabled: true,
            tag: None,
        },
        Segment {
            label: &assembly,
            enabled: false,
            tag: Some(&soon),
        },
    ];
    segmented(ui, &offered, PART);
}

/// « Enregistrer dans » : the folder a part made now goes in, « Choisir… »
/// beside it opening every folder of the library to pick from, and what the
/// card falls back on.
fn saves_in(ui: &mut egui::Ui, card: &mut NewCard, explorer: &Explorer, lang: &Catalogue) {
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
        // Measured from the button the list opens under, not from inside the
        // list: a popup has no place yet the frame it first opens.
        let below = ui.ctx().content_rect().bottom() - choose.rect.bottom() - BELOW_THE_LIST;
        ribbon_menu(&choose, |ui| {
            egui::ScrollArea::vertical()
                .max_height(below.max(LIST_AT_LEAST))
                .show(ui, |ui| {
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
    });
    if let Some(folder) = picked {
        card.pick(folder, explorer);
    }
    ui.weak(lang.t("start_menu.saves_in_by_default"));
}
