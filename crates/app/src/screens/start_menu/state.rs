use std::path::{Path, PathBuf};

use cao_part::{Files, PartDocument, Picture, StepCounts};
use cao_prefs::RecentEntry;
use chrono::TimeZone;

use crate::lang::Catalogue;
use crate::screens::new_card::NewCard;
use crate::wording;

/// What the home screen holds between frames: the « Nouveau » card, and the
/// recent parts as they were last read.
#[derive(Default)]
pub struct StartMenu {
    card: NewCard,
    recents: Vec<ReadPart>,
}

/// A recent part as it was last read.
struct ReadPart {
    entry: RecentEntry,
    /// What its index says it holds — nothing when that cannot be read.
    held: Option<StepCounts>,
    /// The picture it was put away with, when it carries one.
    picture: Option<Picture>,
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

    /// Reads what each recent part holds and the picture it carries, once the
    /// list has changed since it was last read rather than every frame.
    /// Opening a part changes the list, so a part worked on is read again on
    /// the way back.
    pub fn follow(&mut self, recents: &[RecentEntry], files: &impl Files) {
        let unchanged = recents.len() == self.recents.len()
            && recents.iter().zip(&self.recents).all(|(now, read)| {
                let read = &read.entry;
                now.path == read.path && now.name == read.name && now.opened_at == read.opened_at
            });
        if unchanged {
            return;
        }
        self.recents = recents
            .iter()
            .map(|entry| ReadPart {
                entry: entry.clone(),
                held: PartDocument::step_counts_in(files, &entry.path),
                picture: PartDocument::picture_in(files, &entry.path),
            })
            .collect();
    }

    /// The picture a recent part was put away with, as last read.
    pub fn picture_of(&self, part: &Path) -> Option<&Picture> {
        self.recents
            .iter()
            .find(|read| read.entry.path == part)?
            .picture
            .as_ref()
    }

    /// How many lines the recent parts show, « 2 pièces », said beside their
    /// title; nothing while they show none.
    pub fn lines_counted(&self, lang: &Catalogue) -> Option<String> {
        let key = match self.recents.len() {
            0 => return None,
            1 => "start_menu.parts_one",
            _ => "start_menu.parts_many",
        };
        Some(lang.t_with(key, &[("count", &self.recents.len().to_string())]))
    }

    /// The lines of the recent parts, most recently opened first, their
    /// folders read below `root` and their hours on the clock of `zone`.
    pub fn lines(&self, root: &Path, lang: &Catalogue, zone: &impl TimeZone) -> Vec<RecentLine> {
        self.recents
            .iter()
            .map(|read| RecentLine {
                path: read.entry.path.clone(),
                name: read.entry.name.clone(),
                detail: detail(lang, root, &read.entry.path, read.held),
                opened: lang.t_moment("start_menu.recent_date", read.entry.opened_at, zone),
            })
            .collect()
    }
}

/// « Dossier des pièces · 2 esquisses · 1 extrusion », nothing counted in the
/// singular as French says it, or the folder alone when what the part holds
/// could not be read.
fn detail(lang: &Catalogue, root: &Path, part: &Path, held: Option<StepCounts>) -> String {
    let folder = wording::library::where_a_part_lies(lang, root, part.parent().unwrap_or(root));
    let Some(held) = held else {
        return folder;
    };
    let sketches = match held.sketches {
        0 | 1 => "start_menu.sketches_one",
        _ => "start_menu.sketches_many",
    };
    let extrusions = match held.extrusions {
        0 | 1 => "start_menu.extrusions_one",
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

#[cfg(test)]
mod tests;
