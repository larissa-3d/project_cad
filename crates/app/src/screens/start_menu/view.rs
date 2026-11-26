use super::StartMenuAction;
use super::state::{RecentLine, StartMenu};
use crate::lang::Catalogue;
use crate::screens::explorer::Explorer;
use crate::screens::new_card;
use crate::ui::line_button::line_button;
use crate::ui::thumbnail::thumbnail;

/// How wide and how high a recent part's picture is drawn left of its name.
const THUMBNAIL: egui::Vec2 = egui::vec2(64.0, 48.0);

pub fn show(
    ui: &mut egui::Ui,
    menu: &mut StartMenu,
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
            if let Some((name, folder)) = new_card::show(ui, menu.card_mut(), explorer, lang) {
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
                        if recent_line(ui, menu, line) {
                            action = StartMenuAction::Open(line.path.clone());
                        }
                    }
                });
            }
        });
    });

    action
}

/// The part's picture, its name over where it lies and what it holds, the hour
/// it was opened and a chevron on the right. Says whether the line was
/// clicked, anywhere on it.
fn recent_line(ui: &mut egui::Ui, menu: &StartMenu, line: &RecentLine) -> bool {
    line_button(ui, &line.path, |ui| {
        ui.horizontal(|ui| {
            let picture = menu.picture_of(&line.path).map(crate::picture::pixels);
            thumbnail(ui, "recent_thumbnail", THUMBNAIL, picture);
            ui.vertical(|ui| {
                ui.label(&line.name);
                ui.weak(&line.detail);
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.weak("›");
                ui.monospace(&line.opened);
            });
        });
    })
    .clicked()
}
