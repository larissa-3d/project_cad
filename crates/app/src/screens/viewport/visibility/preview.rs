//! The step reopened in the edge panel laid as the panel holds it, on the
//! part before it kept from the reopening, every volume opaque: what
//! « Appliquer » meets. Laid again only when the panel changes.

use std::rc::Rc;

use cao_part::history::BlendShape;
use cao_part::{Operation, PartDocument, Reduced, Shown, StepReopened};
use cao_solid::{Declined, EdgeName};

use super::super::ViewportState;
use crate::screens::blend::BlendState;

/// The step reopened as last laid, with the panel as it then stood: whether
/// that step is drawn, and whether the panel says what the kernel made of it.
pub(super) struct Previewed {
    on: Rc<StepReopened>,
    step: u32,
    shape: Option<BlendShape>,
    size_input: String,
    edges: Vec<EdgeName>,
    last: Option<Laid>,
    drawn: bool,
    said: bool,
}

/// The step as last laid, and what the kernel made of it.
struct Laid {
    operation: Operation,
    shown: Rc<Shown>,
    declined: Option<Declined>,
    reduced: Option<Reduced>,
}

impl Previewed {
    fn holds(&self, blend: &BlendState) -> bool {
        self.shape == blend.shape
            && self.size_input == blend.size_input
            && self.edges == blend.edges
    }
}

impl ViewportState {
    /// Lays the step reopened as its panel now holds it, when the panel
    /// changed, a size that does not read keeping the last laid; another step
    /// reopened keeps nothing of the one before. Hands the panel what the
    /// kernel made of it: true when what the panel says changed.
    pub(crate) fn keep_up_with_the_step_reopened(
        &mut self,
        document: &PartDocument,
        blend: &mut BlendState,
    ) -> bool {
        let (Some(step), Some(on)) = (blend.reopened, self.step_reopened(document)) else {
            self.visibility.preview = None;
            return false;
        };
        let kept = self
            .visibility
            .preview
            .take()
            .filter(|kept| Rc::ptr_eq(&kept.on, &on));
        let previewed = match kept {
            Some(kept) if kept.holds(blend) => kept,
            kept => self.laid(document, blend, (step, on), kept),
        };
        let verdict = previewed.last.as_ref().filter(|_| previewed.said);
        let said = blend.tried(
            verdict.and_then(|last| last.declined),
            verdict.and_then(|last| last.reduced),
        );
        self.visibility.preview = Some(previewed);
        said
    }

    fn laid(
        &mut self,
        document: &PartDocument,
        blend: &BlendState,
        (step, on): (u32, Rc<StepReopened>),
        kept: Option<Previewed>,
    ) -> Previewed {
        let variables = document.variables();
        let reads = blend.size(variables).is_some();
        let (last, drawn) = match (reads, blend.laid(variables), kept) {
            (false, _, Some(kept)) => (kept.last, kept.drawn),
            (false, _, None) => (None, false),
            (true, None, kept) => (kept.and_then(|kept| kept.last), false),
            (true, Some(operation), Some(kept))
                if kept
                    .last
                    .as_ref()
                    .is_some_and(|last| last.operation == operation) =>
            {
                (kept.last, true)
            }
            (true, Some(operation), _) => (Some(self.lay(&on, operation)), true),
        };
        Previewed {
            on,
            step,
            shape: blend.shape,
            size_input: blend.size_input.clone(),
            edges: blend.edges.clone(),
            last,
            drawn,
            said: reads && drawn,
        }
    }

    fn lay(&mut self, on: &StepReopened, operation: Operation) -> Laid {
        let tried = on.laid(&operation);
        self.visibility.previews_built += 1;
        Laid {
            operation,
            shown: Rc::new(Shown {
                body: tried.body,
                ghosts: Vec::new(),
            }),
            declined: tried.declined,
            reduced: tried.reduced,
        }
    }

    /// The step reopened as last laid, drawn opaque in place of the part.
    pub(super) fn previewed(&self) -> Option<Rc<Shown>> {
        let previewed = self.visibility.preview.as_ref()?;
        let last = previewed.last.as_ref().filter(|_| previewed.drawn)?;
        (self.reopened == Some(previewed.step)).then(|| Rc::clone(&last.shown))
    }

    /// How many times the step reopened was laid as its panel held it.
    #[cfg(feature = "test-support")]
    pub(crate) fn previews_built(&self) -> usize {
        self.visibility.previews_built
    }
}
