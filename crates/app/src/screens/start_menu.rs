//! The home screen: the title, then the « Nouveau » card beside the parts
//! lately opened, each saying where it lies and what it holds.

mod state;
mod view;

use std::path::PathBuf;

use cao_part::Files;
use cao_prefs::RecentEntry;

pub use state::StartMenu;

use crate::adapters::clock;
use crate::lang::Catalogue;
use crate::screens::explorer::Explorer;

pub enum StartMenuAction {
    None,
    /// A part of this name, in the folder the card shows.
    CreateNew {
        name: String,
        folder: PathBuf,
    },
    Open(PathBuf),
}

/// Puts the home screen in the middle of the window, what each recent part
/// holds read from `files` only once the list has changed.
pub fn show(
    ui: &mut egui::Ui,
    menu: &mut StartMenu,
    recents: &[RecentEntry],
    explorer: &Explorer,
    lang: &Catalogue,
    files: &impl Files,
) -> StartMenuAction {
    menu.follow(recents, files);
    let lines = menu.lines(&explorer.library().path, lang, &clock::reader_zone());
    egui::CentralPanel::default_margins()
        .show(ui, |ui| {
            view::show(ui, menu.card_mut(), &lines, explorer, lang)
        })
        .inner
}
