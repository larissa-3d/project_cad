//! What the panel read before the arrow in the view was first dragged, which
//! « Annuler » gives back before it closes anything.

use super::{ExtrusionState, NOTHING};

/// The depth as it was typed, and the direction, before the first drag.
pub struct BeforeTheDrag {
    distance_input: String,
    reversed: bool,
}

impl ExtrusionState {
    /// Writes in the panel the depth a drag of the arrow came to, in
    /// millimetres along the plane's normal: its size in the field, the
    /// direction reversed below the plane. What the panel read before the
    /// first drag is kept aside. A depth the field would read as nothing
    /// leaves the panel as it was.
    pub fn drag_depth_to(&mut self, depth: f64) {
        let typed = crate::wording::dimension::short(depth.abs());
        if !typed.parse::<f64>().is_ok_and(|size| size > NOTHING) {
            return;
        }
        self.before_the_drag.get_or_insert_with(|| BeforeTheDrag {
            distance_input: self.distance_input.clone(),
            reversed: self.reversed,
        });
        self.distance_input = typed;
        self.reversed = depth < 0.0;
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
