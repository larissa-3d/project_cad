use std::path::PathBuf;

use cao_prefs::RecentEntry;

use egui::vec2;

use crate::adapters::clock;
use crate::lang::Catalogue;
use crate::screens::explorer::Explorer;
use crate::screens::new_card::{self, NewCard};
use crate::ui::segmented::{Segment, segmented};
use crate::ui::shown_field;
use crate::ui::text_edit::tall_text_edit;

/// How tall « Créer ↵ » stands across the card: the redesign's 44 px, which
/// the tokens do not carry.
const CREATE_HEIGHT: f32 = 44.0;

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
            new_card::saves_in(ui, card, explorer, lang);
            ui.add_space(8.0);
            let room = vec2(ui.available_width(), CREATE_HEIGHT);
            let create = egui::Button::new(lang.t("start_menu.create"));
            let clicked = ui
                .add_enabled_ui(card.can_make(), |ui| ui.add_sized(room, create))
                .inner
                .clicked();
            if (clicked || entered)
                && let Some((name, folder)) = card.make(explorer)
            {
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

/// What the card can make: a part, the one choice there is, and an assembly
/// shown greyed until there are assemblies to make.
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
    segmented(ui, &offered, 0);
}
