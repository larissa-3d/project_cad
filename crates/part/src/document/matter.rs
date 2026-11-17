//! The matter of a part as its steps made it: which faces each one raised or
//! cut, for what names a step to show it where it stands.

use cao_solid::{Body, Declined, Unstable};

use super::PartDocument;
use crate::history::Operation;
use crate::outcome::Outcome;

impl PartDocument {
    /// Records a change of a step of matter and builds the part again from
    /// that step. A blend step the kernel declines laid that way records
    /// nothing, and the part stands as it did.
    pub(super) fn change_matter(&mut self, change: Operation) -> Option<Outcome> {
        let Operation::EditMatter { raised_by, to } = &change else {
            return None;
        };
        let raised_by = *raised_by;
        let before = matches!(**to, Operation::Blend { .. })
            .then(|| (self.history.clone(), self.state.clone()));
        self.history.push(change);
        self.rebuild();
        match (self.declined_because(raised_by), before) {
            (Some(reason), Some((history, state))) => {
                self.history = history;
                self.state = state;
                Some(Outcome::BlendDeclined(reason))
            }
            _ => None,
        }
    }

    /// The matter a step of matter would raise or take away, alone, for a
    /// panel to show while it is still open: the same for an addition and a
    /// cut, neither joined to the part nor cut from it, and nothing recorded.
    /// Nothing for another operation, a size that raises nothing, or a tool
    /// the kernel declines.
    pub fn tool_of(&self, operation: &Operation) -> Option<Body> {
        self.state.tool_of(operation)
    }

    /// The faces of the part a step of matter made, by the number of its
    /// operation — nothing for any other step, and nothing when a geometry
    /// cached before the faces were noted leaves them unknown: better none
    /// than another step's.
    pub fn faces_made_by(&self, step: u32) -> Vec<usize> {
        self.state.faces_made(step)
    }

    /// The operation the step of matter of that number is replayed as now:
    /// its latest change, or the operation that raised it.
    pub fn raised_as(&self, step: u32) -> Option<&Operation> {
        self.history.raised_as(step)
    }

    /// Whether the kernel declined the step of matter of that number: the
    /// part stands as it did before it, and the step is broken.
    pub fn is_declined(&self, step: u32) -> bool {
        self.state.declined.contains_key(&step)
    }

    /// Why the kernel declined the step of matter of that number, for a
    /// developer to read: nothing on screen says it.
    pub fn declined_because(&self, step: u32) -> Option<Declined> {
        self.state.declined.get(&step).copied()
    }

    /// How the kernel leaned the step of matter of that number, when it built
    /// it though it reads it unstably: a change too small to see may change
    /// what is built, which the user is told (#552).
    pub fn unstable_at(&self, step: u32) -> Option<Unstable> {
        self.state.unstable.get(&step).copied()
    }

    /// How the circular pattern of that number falls short of its count, when
    /// a full turn holds fewer copies than it asks for.
    pub fn capped_at(&self, step: u32) -> Option<crate::copying::Capped> {
        self.state.capped.get(&step).copied()
    }

    /// The size the chamfer or fillet of that number asked and the one its
    /// edge's faces gave, when they gave less.
    pub fn reduced_at(&self, step: u32) -> Option<crate::Reduced> {
        self.state.reduced.get(&step).copied()
    }
}
