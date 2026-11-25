use std::path::{Path, PathBuf};

use cao_part::library::Folder;

use crate::screens::explorer::Explorer;

/// What the « Nouveau » card holds between frames: the name being typed, and
/// the folder picked in « Choisir… », if one was.
#[derive(Default)]
pub struct NewCard {
    name: String,
    picked: Option<Picked>,
}

/// A folder picked in « Choisir… », and what the column had selected then:
/// once the column's selection moves on, the column has the last word.
struct Picked {
    folder: PathBuf,
    against: Option<PathBuf>,
}

impl NewCard {
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Whether a part can be made: a name is typed.
    pub fn can_make(&self) -> bool {
        !self.name.trim().is_empty()
    }

    /// Where a part made now would go: the folder picked, until the column's
    /// selection changes, else the folder the column chose. A pick the column
    /// moved past is forgotten, so selecting back does not bring it back.
    pub fn folder(&mut self, explorer: &Explorer) -> PathBuf {
        if let Some(picked) = &self.picked
            && picked.against.as_deref() == explorer.selected()
        {
            return picked.folder.clone();
        }
        self.picked = None;
        explorer.chosen_folder()
    }

    pub fn pick(&mut self, folder: PathBuf, explorer: &Explorer) {
        self.picked = Some(Picked {
            folder,
            against: explorer.selected().map(Path::to_path_buf),
        });
    }

    /// The part to make, its name trimmed and the folder it goes in, which
    /// empties the field; nothing while no name is typed.
    pub fn make(&mut self, explorer: &Explorer) -> Option<(String, PathBuf)> {
        if !self.can_make() {
            return None;
        }
        let name = std::mem::take(&mut self.name).trim().to_string();
        Some((name, self.folder(explorer)))
    }
}

/// What « Choisir… » offers: the root of the library, then every folder under
/// it, each before the folders inside it, with how deep it stands — the root
/// at 0. A part is no place to save in, and nothing outside the library is
/// read.
pub fn folders_offered(library: &Folder) -> Vec<(usize, &Folder)> {
    fn walk<'a>(folder: &'a Folder, depth: usize, offered: &mut Vec<(usize, &'a Folder)>) {
        offered.push((depth, folder));
        for under in &folder.folders {
            walk(under, depth + 1, offered);
        }
    }
    let mut offered = Vec::new();
    walk(library, 0, &mut offered);
    offered
}

#[cfg(test)]
mod tests;
