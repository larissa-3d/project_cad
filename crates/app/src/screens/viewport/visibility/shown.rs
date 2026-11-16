//! The part as its volumes' states show it, the raw block of the volume
//! marked, and the volume the extrusion being set previews, each worked out
//! only when what it is worked out from changes: a frame and a hover read
//! them, and a replay or a step of matter costs far more than either can
//! afford (#583).

use std::collections::BTreeMap;
use std::rc::Rc;

use cao_part::{History, Operation, PartDocument, Shown, StepReopened, VolumeState};
use cao_solid::Body;

use super::super::ViewportState;

/// A part shown, with the history, the states and the reading point it was
/// replayed from. The history is compared by value: the document keeps no
/// revision of it, and it is all the replay reads.
pub(super) struct Replayed {
    history: History,
    states: BTreeMap<u32, VolumeState>,
    reading_point: Option<usize>,
    reopened: Option<u32>,
    shown: Rc<Shown>,
}

/// The part as it stood before the step of that number, every volume
/// opaque, with the history it was replayed from.
pub(super) struct Before {
    history: History,
    step: u32,
    shown: Rc<Shown>,
    reopened: Rc<StepReopened>,
}

/// The raw block of the volume made by the step of that number, with the
/// history it was replayed from: nothing when that step raises nothing.
pub(super) struct Blocked {
    history: History,
    number: u32,
    block: Option<Rc<Body>>,
}

/// The volume a step of matter raises or takes away alone, with the step and
/// the history it was worked out from: nothing when the kernel declines it.
pub(super) struct ToolPreviewed {
    history: History,
    operation: Operation,
    volume: Option<Rc<Body>>,
}

impl ViewportState {
    /// The part as its volumes' states show it, read through the step the
    /// reader is on, or as it stood before the step reopened; the step
    /// reopened laid as its panel holds it, opaque, once it is; nothing while
    /// every volume of the document is opaque, the part is read to its end and
    /// no step is reopened, when the part shown is the part itself.
    pub(crate) fn part_shown(&self, document: &PartDocument) -> Option<Rc<Shown>> {
        if let Some(previewed) = self.previewed() {
            return Some(previewed);
        }
        let states = self.volume_states(document);
        let (reading_point, reopened) = (self.reading.point, self.reopened);
        if states.is_empty() && reading_point.is_none() && reopened.is_none() {
            return None;
        }
        if states.is_empty() && reopened.is_some() {
            return self.part_reopened(document);
        }
        let mut kept = self.visibility.shown.borrow_mut();
        if let Some(replayed) = kept.as_ref()
            && replayed.states == states
            && replayed.reading_point == reading_point
            && replayed.reopened == reopened
            && replayed.history == document.history
        {
            return Some(Rc::clone(&replayed.shown));
        }
        let shown = Rc::new(match (reopened, reading_point) {
            (Some(step), _) => document.shown_before(&states, step),
            (None, Some(key_step)) => document.shown_through(&states, key_step),
            (None, None) => document.shown(&states),
        });
        *kept = Some(Replayed {
            history: document.history.clone(),
            states,
            reading_point,
            reopened,
            shown: Rc::clone(&shown),
        });
        Some(shown)
    }

    /// The part as it stood before the step reopened, every volume opaque
    /// whatever is drawn, which its edges are named on as a new step's are on
    /// the part; nothing while no step is.
    pub(crate) fn part_reopened(&self, document: &PartDocument) -> Option<Rc<Shown>> {
        self.before(document).map(|(shown, _)| shown)
    }

    /// The part as it stood before the step reopened, kept to lay the step
    /// on as its panel holds it; nothing while no step is.
    pub(super) fn step_reopened(&self, document: &PartDocument) -> Option<Rc<StepReopened>> {
        self.before(document).map(|(_, reopened)| reopened)
    }

    fn before(&self, document: &PartDocument) -> Option<(Rc<Shown>, Rc<StepReopened>)> {
        let step = self.reopened?;
        let mut kept = self.visibility.before.borrow_mut();
        if let Some(before) = kept.as_ref()
            && before.step == step
            && before.history == document.history
        {
            return Some((Rc::clone(&before.shown), Rc::clone(&before.reopened)));
        }
        let reopened = Rc::new(document.step_reopened(step));
        let shown = Rc::new(Shown {
            body: reopened.body().clone(),
            ghosts: Vec::new(),
        });
        *kept = Some(Before {
            history: document.history.clone(),
            step,
            shown: Rc::clone(&shown),
            reopened: Rc::clone(&reopened),
        });
        Some((shown, reopened))
    }

    /// The raw block of the volume marked: the matter its step raises or
    /// removes, the block it shows see-through, which for a step the kernel
    /// declined stands where it would have been joined to the part.
    pub(crate) fn marked_block(&self, document: &PartDocument) -> Option<Rc<Body>> {
        let number = document.history.number_at(self.marked_volume()?)?;
        let mut kept = self.visibility.block.borrow_mut();
        if let Some(blocked) = kept.as_ref()
            && blocked.number == number
            && blocked.history == document.history
        {
            return blocked.block.clone();
        }
        let see_through = BTreeMap::from([(number, VolumeState::SeeThrough)]);
        let block = document
            .shown(&see_through)
            .ghosts
            .into_iter()
            .find(|ghost| ghost.step == number)
            .map(|ghost| Rc::new(ghost.body));
        *kept = Some(Blocked {
            history: document.history.clone(),
            number,
            block: block.clone(),
        });
        block
    }

    /// The volume the step `operation` raises or takes away alone, as the
    /// part stands: worked out again once the step or the part changes.
    pub(crate) fn previewed_volume(
        &self,
        document: &PartDocument,
        operation: &Operation,
    ) -> Option<Rc<Body>> {
        let mut kept = self.visibility.previewed.borrow_mut();
        if let Some(previewed) = kept.as_ref()
            && previewed.operation == *operation
            && previewed.history == document.history
        {
            return previewed.volume.clone();
        }
        let volume = document.tool_of(operation).map(Rc::new);
        *kept = Some(ToolPreviewed {
            history: document.history.clone(),
            operation: operation.clone(),
            volume: volume.clone(),
        });
        volume
    }
}
