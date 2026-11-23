//! The key steps of a part — each sketch, each volume and each section kept,
//! a section deleted taking its step with it — in the order of the part, which
//! is the one the step reader shows and the one a part read to a step, or a
//! step put in, counts them by.

use super::{History, Operation, Step, StepKind};

impl History {
    /// The key steps in effect, in the order of the part, each as its place
    /// among the steps and the place in the list of the operation opening it.
    pub fn key_steps(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.steps
            .iter()
            .enumerate()
            .filter(|(_, step)| step.kind().is_key() && !self.is_deleted(step))
            .filter_map(|(at, step)| {
                let opened_at = self.position_of(*step.operations().first()?)?;
                (opened_at < self.applied).then_some((at, opened_at))
            })
    }

    /// Whether a deletion in effect is recorded under that step: its section
    /// is gone from the part.
    fn is_deleted(&self, step: &Step) -> bool {
        step.operations()
            .iter()
            .filter(|number| self.in_effect(**number))
            .any(|number| matches!(self.at(*number), Some(Operation::DeleteSection { .. })))
    }

    /// Where the `key_step`th key step in effect stands among the steps,
    /// counted from nothing in the order of the part.
    pub(crate) fn place_of_key_step(&self, key_step: usize) -> Option<usize> {
        self.key_steps().nth(key_step).map(|(at, _)| at)
    }

    /// The number of the operation opening the first step in effect after the
    /// one at `place` among the steps, a blend as much as a key step: a step
    /// undone and kept to be redone is never replayed, so it is never met.
    pub(crate) fn opening_after(&self, place: usize) -> Option<u32> {
        self.steps
            .get(place + 1..)?
            .iter()
            .filter_map(|step| step.operations().first().copied())
            .find(|number| self.in_effect(*number))
    }

    /// Whether the operation of that number is in effect: recorded, and not
    /// undone.
    pub(crate) fn in_effect(&self, number: u32) -> bool {
        self.position_of(number).is_some_and(|at| at < self.applied)
    }

    /// The rank among the key steps in effect of the one that opened that
    /// sketch; nothing while it is undone.
    pub fn key_step_of_sketch(&self, sketch: usize) -> Option<usize> {
        self.key_step_of(StepKind::Sketch, sketch)
    }

    /// The same for the section of that id; nothing while it is undone or
    /// deleted.
    pub fn key_step_of_section(&self, section: usize) -> Option<usize> {
        self.key_step_of(StepKind::Section, section)
    }

    fn key_step_of(&self, kind: StepKind, rank: usize) -> Option<usize> {
        let (_, place) = *kind.in_the_order_made(&self.steps).get(rank)?;
        self.key_steps().position(|(at, _)| at == place)
    }

    /// The rank among the key steps in effect of the one the operation of that
    /// number opened; nothing for any other operation.
    pub fn key_step_opened_by(&self, number: u32) -> Option<usize> {
        self.key_steps()
            .position(|(_, opened_at)| self.number_at(opened_at) == Some(number))
    }
}
