//! What the panel read before the arrow in the view was first dragged, which
//! « Annuler » gives back before it closes anything — as long as nothing was
//! typed over what the drag wrote.

use super::{ExtrusionState, NOTHING};

/// What the panel read before the first drag, and what the last drag left in
/// it.
pub struct BeforeTheDrag {
    before: Typed,
    left: Typed,
}

/// The depth and the angle as written in the panel, and the direction.
#[derive(PartialEq)]
struct Typed {
    distance_input: String,
    angle_input: String,
    reversed: bool,
}

impl ExtrusionState {
    /// Writes in the panel what a drag of the arrow came to — a depth in
    /// millimetres along the plane's normal, or for a revolution an angle in
    /// degrees round its axis: its size in the field, the direction reversed
    /// for a value below zero. What the panel read before the first drag is
    /// kept aside, or before this one when something was typed since the last.
    /// A value the field would read as nothing leaves the panel as it was.
    pub fn drag_to(&mut self, value: f64) {
        let typed = crate::wording::dimension::short(value.abs());
        if !typed.parse::<f64>().is_ok_and(|size| size > NOTHING) {
            return;
        }
        let before = match self.before_the_drag.take() {
            Some(memory) if memory.left == self.typed() => memory.before,
            _ => self.typed(),
        };
        match self.is_revolving() {
            true => self.angle_input = typed,
            false => self.distance_input = typed,
        }
        self.reversed = value < 0.0;
        self.before_the_drag = Some(BeforeTheDrag {
            before,
            left: self.typed(),
        });
    }

    /// Gives back what the panel read before the arrow was first dragged, or
    /// closes it when it was never dragged or something was typed since:
    /// whether it closed.
    pub fn cancel(&mut self) -> bool {
        match self.before_the_drag.take() {
            Some(memory) if memory.left == self.typed() => {
                let before = memory.before;
                self.distance_input = before.distance_input;
                self.angle_input = before.angle_input;
                self.reversed = before.reversed;
                false
            }
            _ => {
                self.close();
                true
            }
        }
    }

    /// Lets go of what the panel read before a drag, so that the next cancel
    /// closes it: « Fermer » closes at once, where « Annuler » gives a drag
    /// back first.
    pub fn forget_the_drag(&mut self) {
        self.before_the_drag = None;
    }

    fn typed(&self) -> Typed {
        Typed {
            distance_input: self.distance_input.clone(),
            angle_input: self.angle_input.clone(),
            reversed: self.reversed,
        }
    }
}
