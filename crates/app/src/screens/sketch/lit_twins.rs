//! What erasing the selection would take with it, read once a frame rather
//! than once for every piece the frame draws.

use std::cell::RefCell;

use cao_sketch::{Element, Selection, Sketch};

/// The mirror images of what is selected in the sketch being edited, kept
/// for the frame that read them.
#[derive(Default)]
pub struct LitTwins(RefCell<Option<Read>>);

struct Read {
    sketch: usize,
    selection: Vec<Selection>,
    twins: Vec<Element>,
}

impl LitTwins {
    /// Whether `element` of the sketch at `index` is an image of something
    /// selected there.
    pub(crate) fn lights(
        &self,
        index: usize,
        sketch: &Sketch,
        selection: &[Selection],
        element: Element,
    ) -> bool {
        let mut read = self.0.borrow_mut();
        let current = read
            .as_ref()
            .is_some_and(|read| read.sketch == index && read.selection == selection);
        if !current {
            let twins = selection
                .iter()
                .filter_map(|held| match held {
                    Selection::Element(element) => Some(sketch.twins_of(*element)),
                    _ => None,
                })
                .flatten()
                .collect();
            *read = Some(Read {
                sketch: index,
                selection: selection.to_vec(),
                twins,
            });
        }
        read.as_ref()
            .is_some_and(|read| read.twins.contains(&element))
    }

    /// Lets go of what the last frame read: the drawing may have changed
    /// since, and a twin with it.
    pub fn forget(&self) {
        self.0.take();
    }
}
