use std::path::{Path, PathBuf};

use cao_part::{Files, PartDocument, StepCounts};
use cao_prefs::RecentEntry;
use chrono::TimeZone;

use crate::lang::Catalogue;
use crate::screens::new_card::NewCard;
use crate::wording;

/// What the home screen holds between frames: the « Nouveau » card, and the
/// recent parts with what each one holds.
#[derive(Default)]
pub struct StartMenu {
    card: NewCard,
    /// The recent list as it was last read, each part with what its index says
    /// it holds — nothing for one whose index cannot be read.
    recents: Vec<(RecentEntry, Option<StepCounts>)>,
}

/// One line of the recent parts, as it reads.
pub struct RecentLine {
    pub path: PathBuf,
    pub name: String,
    /// Where the part lies, then what it holds.
    pub detail: String,
    /// When it was last opened.
    pub opened: String,
}

impl StartMenu {
    pub fn card_mut(&mut self) -> &mut NewCard {
        &mut self.card
    }

    /// Reads what each recent part holds, once the list has changed since it
    /// was last read rather than every frame. Opening a part changes the list,
    /// so a part worked on is read again on the way back.
    pub fn follow(&mut self, recents: &[RecentEntry], files: &impl Files) {
        let unchanged = recents.len() == self.recents.len()
            && recents.iter().zip(&self.recents).all(|(now, (read, _))| {
                now.path == read.path && now.name == read.name && now.opened_at == read.opened_at
            });
        if unchanged {
            return;
        }
        self.recents = recents
            .iter()
            .map(|entry| {
                let held = PartDocument::step_counts_in(files, &entry.path);
                (entry.clone(), held)
            })
            .collect();
    }

    /// The lines of the recent parts, most recently opened first, their
    /// folders read below `root` and their hours on the clock of `zone`.
    pub fn lines(&self, root: &Path, lang: &Catalogue, zone: &impl TimeZone) -> Vec<RecentLine> {
        self.recents
            .iter()
            .map(|(entry, held)| RecentLine {
                path: entry.path.clone(),
                name: entry.name.clone(),
                detail: detail(lang, root, &entry.path, *held),
                opened: lang.t_moment("start_menu.recent_date", entry.opened_at, zone),
            })
            .collect()
    }
}

/// « Dossier des pièces · 2 esquisses · 1 extrusion », or the folder alone
/// when what the part holds could not be read.
fn detail(lang: &Catalogue, root: &Path, part: &Path, held: Option<StepCounts>) -> String {
    let folder = wording::library::folder(lang, root, part.parent().unwrap_or(root));
    let Some(held) = held else {
        return folder;
    };
    let sketches = match held.sketches {
        1 => "start_menu.sketches_one",
        _ => "start_menu.sketches_many",
    };
    let extrusions = match held.extrusions {
        1 => "start_menu.extrusions_one",
        _ => "start_menu.extrusions_many",
    };
    lang.t_with(
        "start_menu.recent_detail",
        &[
            ("folder", &folder),
            (
                "sketches",
                &lang.t_with(sketches, &[("count", &held.sketches.to_string())]),
            ),
            (
                "extrusions",
                &lang.t_with(extrusions, &[("count", &held.extrusions.to_string())]),
            ),
        ],
    )
}
