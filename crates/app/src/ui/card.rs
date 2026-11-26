//! The frame of a card: a panel standing a shade above the page, bordered and
//! rounded, what it holds set in from its edge.

use egui::{Frame, Margin, Stroke};

use crate::ui::theme::tokens::{BACKGROUND_CARD, BORDER_GROUP, RADIUS_CARD, SPACING_L};

pub fn card_frame() -> Frame {
    Frame::new()
        .fill(BACKGROUND_CARD)
        .stroke(Stroke::new(1.0, BORDER_GROUP))
        .corner_radius(RADIUS_CARD)
        .inner_margin(Margin::same(SPACING_L as i8))
}
