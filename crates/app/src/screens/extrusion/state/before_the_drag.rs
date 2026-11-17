//! What the panel read before the arrow in the view was first dragged, which
//! « Annuler » gives back before it closes anything.

use super::ExtrusionState;

/// The depth as it was typed, and the direction, before the first drag.
pub struct BeforeTheDrag {
    distance_input: String,
    reversed: bool,
}

impl ExtrusionState {
    /// Writes in the depth field what a drag of the arrow came to, what the
    /// panel read before the first drag kept aside.
    pub fn drag_depth_to(&mut self, typed: String) {
        self.before_the_drag.get_or_insert_with(|| BeforeTheDrag {
            distance_input: self.distance_input.clone(),
            reversed: self.reversed,
        });
        self.distance_input = typed;
    }

    /// Gives back what the panel read before the arrow was first dragged, or
    /// closes it when it was never dragged: whether it closed.
    pub fn cancel(&mut self) -> bool {
        match self.before_the_drag.take() {
            Some(before) => {
                self.distance_input = before.distance_input;
                self.reversed = before.reversed;
                false
            }
            None => {
                self.close();
                true
            }
        }
    }
}
