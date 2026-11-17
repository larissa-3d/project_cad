//! The step reopened in the edge panel laid as the panel holds it, on the
//! part before it kept from the reopening, every volume opaque: what
//! « Appliquer » meets. Laid again only when the panel changes.

use std::rc::Rc;

use cao_part::history::BlendShape;
use cao_part::{Operation, PartDocument, Reduced, Shown, StepReopened};
use cao_solid::{Declined, EdgeName};

use super::super::ViewportState;
use crate::screens::blend::BlendState;

/// The step reopened as last laid, with the panel as it then stood and what
/// the kernel made of it.
pub(super) struct Previewed {
    on: Rc<StepReopened>,
    step: u32,
    shape: Option<BlendShape>,
    size_input: String,
    edges: Vec<EdgeName>,
    laid: Option<Operation>,
    shown: Option<Rc<Shown>>,
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
        let said = blend.tried(previewed.declined, previewed.reduced);
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
        let laid = blend.laid(variables);
        let mut previewed = Previewed {
            on: Rc::clone(&on),
            step,
            shape: blend.shape,
            size_input: blend.size_input.clone(),
            edges: blend.edges.clone(),
            laid: None,
            shown: None,
            declined: None,
            reduced: None,
        };
        match (reads, kept) {
            (false, Some(kept)) => {
                (previewed.laid, previewed.shown) = (kept.laid, kept.shown);
            }
            (true, Some(kept)) if laid.is_some() && kept.laid == laid => {
                (previewed.laid, previewed.shown) = (kept.laid, kept.shown);
                (previewed.declined, previewed.reduced) = (kept.declined, kept.reduced);
            }
            (true, _) if let Some(laid) = laid => {
                let tried = on.laid(&laid);
                let (built, _) = self.visibility.previews;
                self.visibility.previews = (built + 1, 1);
                previewed.shown = Some(Rc::new(Shown {
                    body: tried.body,
                    ghosts: Vec::new(),
                }));
                (previewed.declined, previewed.reduced) = (tried.declined, tried.reduced);
                previewed.laid = Some(laid);
            }
            _ => {}
        }
        previewed
    }

    /// The step reopened as last laid, drawn opaque in place of the part.
    pub(super) fn previewed(&self) -> Option<Rc<Shown>> {
        let previewed = self.visibility.preview.as_ref()?;
        let shown = previewed.shown.as_ref()?;
        (self.reopened == Some(previewed.step)).then(|| Rc::clone(shown))
    }

    /// How many times the step reopened was laid as its panel held it, and
    /// how many operations the last of them laid.
    #[cfg(feature = "test-support")]
    pub(crate) fn previews_built(&self) -> (usize, usize) {
        self.visibility.previews
    }
}
