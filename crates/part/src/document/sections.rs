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

    /// Whether a drawing stands on that section, which deleting it would leave
    /// following nothing more.
    pub fn is_stood_on(&self, section: usize) -> bool {
        self.sketch_openings().any(|opening| {
            matches!(opening, Operation::CreateSketchOnSection { section: under, .. } if *under == section)
        })
    }

    /// What the hand chose for a section's eye: `Some(true)` opened,
    /// `Some(false)` shut, `None` left to the rule.
    pub fn section_choice(&self, section: usize) -> Option<bool> {
        self.section_choices.get(&section).copied()
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

    /// Drops the choice of a section the history no longer reaches, so that
    /// the next one saved, which takes its rank, comes with none. A deleted
    /// section keeps its rank, and its choice with it.
    pub(super) fn forget_choices_of_sections_gone(&mut self) {
        let kept = self.state.sections.len();
        self.section_choices.retain(|&section, _| section < kept);
    }

    fn sketch_openings(&self) -> impl Iterator<Item = &Operation> {
        self.history
            .applied_operations()
            .iter()
            .filter(|operation| StepKind::opened_by(operation) == Some(StepKind::Sketch))
    }
}
