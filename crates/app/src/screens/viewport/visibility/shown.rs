//! The part as its volumes' states show it, and the raw block of the volume
//! marked, each replayed only when what it is replayed from changes: a frame
//! and a hover read them, and a replay costs far more than either can afford.

use std::collections::BTreeMap;
use std::rc::Rc;

use cao_part::{History, Operation, PartDocument, Reduced, Shown, VolumeState};
use cao_solid::{Body, Declined};

use super::super::ViewportState;
use crate::screens::blend::BlendState;

/// A part shown, with the history, the states and the reading point it was
/// replayed from. The history is compared by value: the document keeps no
/// revision of it, and it is all the replay reads.
pub(super) struct Replayed {
    history: History,
    states: BTreeMap<u32, VolumeState>,
    reading_point: Option<usize>,
    reopened: Option<u32>,
    reopened_as: Option<Operation>,
    shown: Rc<Shown>,
    preview: Option<Preview>,
}

/// What the kernel made of the step reopened as its panel holds it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct Preview {
    declined: Option<Declined>,
    reduced: Option<Reduced>,
}

/// The part as it stood before the step of that number, every volume
/// opaque, with the history it was replayed from.
pub(super) struct Before {
    history: History,
    step: u32,
    shown: Rc<Shown>,
}

/// The raw block of the volume made by the step of that number, with the
/// history it was replayed from: nothing when that step raises nothing.
pub(super) struct Blocked {
    history: History,
    number: u32,
    block: Option<Rc<Body>>,
}

impl ViewportState {
    /// The part as its volumes' states show it, read through the step the
    /// reader is on, or through the step reopened as its panel holds it, as it
    /// stood before that step while the panel holds nothing to lay; nothing
    /// while every volume of the document is opaque, the part is read to its
    /// end and no step is reopened, when the part shown is the part itself.
    pub(crate) fn part_shown(&self, document: &PartDocument) -> Option<Rc<Shown>> {
        let states = self.volume_states(document);
        let (reading_point, reopened) = (self.reading.point, self.reopened);
        let reopened_as = reopened.and(self.reopened_as.as_ref());
        if states.is_empty() && reading_point.is_none() && reopened.is_none() {
            return None;
        }
        if states.is_empty() && reopened.is_some() && reopened_as.is_none() {
            return self.part_reopened(document);
        }
        let mut kept = self.visibility.shown.borrow_mut();
        if let Some(replayed) = kept.as_ref()
            && replayed.states == states
            && replayed.reading_point == reading_point
            && replayed.reopened == reopened
            && replayed.reopened_as.as_ref() == reopened_as
            && replayed.history == document.history
        {
            return Some(Rc::clone(&replayed.shown));
        }
        let (shown, preview) = match (reopened, reopened_as, reading_point) {
            (Some(step), Some(to), _) => {
                let tried = document.shown_changed(&states, step, to.clone());
                let (built, _) = self.visibility.previews.get();
                self.visibility
                    .previews
                    .set((built + 1, tried.operations_laid));
                let (declined, reduced) = (tried.declined, tried.reduced);
                (tried.shown, Some(Preview { declined, reduced }))
            }
            (Some(step), None, _) => (document.shown_before(&states, step), None),
            (None, _, Some(key_step)) => (document.shown_through(&states, key_step), None),
            (None, _, None) => (document.shown(&states), None),
        };
        let shown = Rc::new(shown);
        *kept = Some(Replayed {
            history: document.history.clone(),
            states,
            reading_point,
            reopened,
            reopened_as: reopened_as.cloned(),
            shown: Rc::clone(&shown),
            preview,
        });
        Some(shown)
    }

    /// Lays the step reopened as its panel now holds it, a size that does
    /// not read keeping the last laid, and hands the panel what the kernel
    /// made of it: true when what the panel says changed.
    pub(crate) fn keep_up_with_the_step_reopened(
        &mut self,
        document: &PartDocument,
        blend: &mut BlendState,
    ) -> bool {
        let variables = document.variables();
        let reads = blend.size(variables).is_some();
        self.reopened_as = match (blend.reopened, reads) {
            (None, _) => None,
            (Some(_), true) => blend.laid(variables),
            (Some(_), false) => self.reopened_as.take(),
        };
        let preview = match reads {
            true => self.preview(document).unwrap_or_default(),
            false => Preview::default(),
        };
        blend.reopened.is_some() && blend.tried(preview.declined, preview.reduced)
    }

    /// How many times the step reopened was laid as its panel held it, and
    /// how many operations of the history the last of them laid.
    #[cfg(feature = "test-support")]
    pub(crate) fn previews_built(&self) -> (usize, usize) {
        self.visibility.previews.get()
    }

    /// What the kernel made of the step reopened as its panel holds it, read
    /// from the part shown; nothing while the panel holds nothing to lay.
    fn preview(&self, document: &PartDocument) -> Option<Preview> {
        self.reopened_as.as_ref()?;
        self.part_shown(document)?;
        let kept = self.visibility.shown.borrow();
        kept.as_ref().and_then(|replayed| replayed.preview)
    }

    /// The part as it stood before the step reopened, every volume opaque
    /// whatever is drawn, which its edges are named on as a new step's are on
    /// the part; nothing while no step is.
    pub(crate) fn part_reopened(&self, document: &PartDocument) -> Option<Rc<Shown>> {
        let step = self.reopened?;
        let mut kept = self.visibility.before.borrow_mut();
        if let Some(before) = kept.as_ref()
            && before.step == step
            && before.history == document.history
        {
            return Some(Rc::clone(&before.shown));
        }
        let shown = Rc::new(document.shown_before(&BTreeMap::new(), step));
        *kept = Some(Before {
            history: document.history.clone(),
            step,
            shown: Rc::clone(&shown),
        });
        Some(shown)
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
}
