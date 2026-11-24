//! What the part says of the sections it keeps, and of the drawings laid on
//! them.

use std::collections::BTreeMap;

use super::PartDocument;
use crate::history::{Operation, StepKind};
use crate::sections::SavedSection;

/// Whether each kept section's eye was opened (`true`) or shut (`false`) by
/// hand, by section. One left to the rule has no entry.
pub(super) type SectionChoices = BTreeMap<usize, bool>;

impl PartDocument {
    /// The sections the part keeps, in the order they were made, each found
    /// on the part as it stands now.
    pub fn sections(&self) -> Vec<SavedSection> {
        self.state.sections_standing()
    }

    /// The kept section a drawing was laid on, when it was laid on one.
    pub fn section_under(&self, sketch: usize) -> Option<usize> {
        match self.sketch_openings().nth(sketch)? {
            Operation::CreateSketchOnSection { section, .. } => Some(*section),
            _ => None,
        }
    }

    /// Each drawing laid on a kept section, with that section, found in one
    /// pass over the openings rather than one per drawing.
    pub fn drawings_on_sections(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.sketch_openings()
            .enumerate()
            .filter_map(|(sketch, opening)| match opening {
                Operation::CreateSketchOnSection { section, .. } => Some((sketch, *section)),
                _ => None,
            })
    }

    /// Whether a drawing stands on that section, which deleting it would leave
    /// following nothing more.
    pub fn is_stood_on(&self, section: usize) -> bool {
        self.sketch_openings().any(|opening| {
            matches!(opening, Operation::CreateSketchOnSection { section: under, .. } if *under == section)
        })
    }

    /// What the hand chose for a section's eye: `Some(true)` opened,
    /// `Some(false)` shut, `None` left to the rule — and nothing for a section
    /// undone or deleted, whose choice is kept for when it comes back.
    pub fn section_choice(&self, section: usize) -> Option<bool> {
        self.state
            .section_stands(section)
            .then(|| self.section_choices.get(&section).copied())
            .flatten()
    }

    /// Opens a section's eye by hand, every other one opened by hand given
    /// back to the rule, a deleted or undone one included. One shut by hand
    /// stays shut.
    pub fn open_section_by_hand(&mut self, section: usize) {
        self.none_opened_by_hand();
        self.set_section_choice(section, Some(true));
    }

    /// Opens or shuts a section's eye by hand, or leaves it to the rule again.
    /// A section the part does not keep takes no choice.
    pub fn set_section_choice(&mut self, section: usize, choice: Option<bool>) {
        match choice {
            Some(shown) if section < self.state.sections.len() => {
                self.section_choices.insert(section, shown);
            }
            _ => {
                self.section_choices.remove(&section);
            }
        }
    }

    /// How many sections the part has made, when a redo would bring more:
    /// taken before an operation is recorded, which drops what was undone.
    pub(super) fn sections_made_if_undone(&self) -> Option<usize> {
        self.history.can_redo().then_some(self.state.sections.len())
    }

    /// What recording an operation does to the hand's choices: what was
    /// undone is dropped, and the choices of the sections only it had made go
    /// with it, since their ranks will be given again; a section saved gives
    /// every one opened by hand back to the rule.
    pub(super) fn choices_on_recording(&mut self, operation: &Operation, made: Option<usize>) {
        self.forget_choices_past(made);
        if matches!(operation, Operation::SaveSection(_)) {
            self.none_opened_by_hand();
        }
    }

    /// Forgets the choices of the sections past the `made` first, when
    /// something was undone.
    pub(super) fn forget_choices_past(&mut self, made: Option<usize>) {
        if let Some(made) = made {
            self.section_choices.retain(|&section, _| section < made);
        }
    }

    fn none_opened_by_hand(&mut self) {
        self.section_choices.retain(|_, shown| !*shown);
    }

    fn sketch_openings(&self) -> impl Iterator<Item = &Operation> {
        self.history
            .applied_operations()
            .iter()
            .filter(|operation| StepKind::opened_by(operation) == Some(StepKind::Sketch))
    }
}
