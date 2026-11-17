//! What the panel read before the arrow in the view was first dragged, which
//! « Annuler » gives back before it closes anything.

use super::{ExtrusionState, NOTHING};

/// The depth and the angle as they were typed, and the direction, before the
/// first drag.
pub struct BeforeTheDrag {
    distance_input: String,
    angle_input: String,
    reversed: bool,
}

impl ExtrusionState {
    /// Writes in the panel what a drag of the arrow came to — a depth in
    /// millimetres along the plane's normal, or for a revolution an angle in
    /// degrees round its axis: its size in the field, the direction reversed
    /// for a value below zero. What the panel read before the first drag is
    /// kept aside. A value the field would read as nothing leaves the panel as
    /// it was.
    pub fn drag_to(&mut self, value: f64) {
        let typed = crate::wording::dimension::short(value.abs());
        if !typed.parse::<f64>().is_ok_and(|size| size > NOTHING) {
            return;
        }
        self.before_the_drag.get_or_insert_with(|| BeforeTheDrag {
            distance_input: self.distance_input.clone(),
            angle_input: self.angle_input.clone(),
            reversed: self.reversed,
        });
        match self.is_revolving() {
            true => self.angle_input = typed,
            false => self.distance_input = typed,
        }
        self.reversed = value < 0.0;
    }

    /// Gives back what the panel read before the arrow was first dragged, or
    /// closes it when it was never dragged: whether it closed.
    pub fn cancel(&mut self) -> bool {
        match self.before_the_drag.take() {
            Some(before) => {
                self.distance_input = before.distance_input;
                self.angle_input = before.angle_input;
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
