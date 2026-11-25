//! The library's column on the home screen, and what the shell does with
//! what it hands back.

use super::CaoApp;
use crate::adapters::files::DiskFiles;
use crate::{screens, wording};

impl CaoApp {
    /// Draws the library's column, keeps the recent list pointing at a part
    /// it moved, and opens a part double-clicked in it. Says whether one was.
    pub(super) fn show_the_library(&mut self, ui: &mut egui::Ui) -> bool {
        let handed = screens::explorer::run(
            &mut self.explorer,
            ui,
            self.remembered.lang(),
            &DiskFiles,
            &DiskFiles,
        );
        if let Some((from, to)) = handed.moved
            && let Some(err) = self.remembered.part_moved(&from, &to)
        {
            self.error = Some(wording::storage::say(self.remembered.lang(), &err));
        }
        let Some(part) = handed.open else {
            return false;
        };
        self.open_part(part);
        true
    }
}
