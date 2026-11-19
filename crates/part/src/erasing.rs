//! What a part does when a selection is erased from a drawing: a mirrored
//! twin goes with its original, and the erase notes the twins it took along.

use cao_sketch::{Constraint, DimensionTarget, Element};

use crate::outcome::Outcome;
use crate::state::PartState;

impl PartState {
    pub(crate) fn erase_many(
        &mut self,
        sketch: usize,
        elements: &[Element],
        dimensions: &[DimensionTarget],
        constraints: &[Constraint],
    ) -> Option<Outcome> {
        let scale = self.scale();
        let sketch = self.sketches.get_mut(sketch)?;
        let mut twins: Vec<Element> = Vec::new();
        for twin in elements.iter().flat_map(|e| sketch.twins_of(*e)) {
            if !elements.contains(&twin) && !twins.contains(&twin) {
                twins.push(twin);
            }
        }
        for rule in constraints {
            sketch.erase_constraint(*rule);
        }
        for target in dimensions {
            sketch.erase_dimension(*target);
        }
        for element in elements.iter().chain(&twins) {
            sketch.erase(*element);
        }
        // What is left may have room to move again, so it settles into
        // whatever the remaining values still ask of it.
        sketch.resolve(scale);
        if !twins.is_empty() {
            self.twins_erased.insert(self.replaying, twins);
        }
        None
    }
}
