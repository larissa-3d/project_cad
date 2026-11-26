//! A line of a list that answers a click as one button, whatever it holds: a
//! click on any of it, its texts or the room between them, is a click on the
//! line.

use egui::{AsIdSalt, CursorIcon, Response, Sense, Shape, Ui};

use crate::ui::theme::tokens::{BACKGROUND_HOVER, RADIUS_BUTTON};

/// Draws `contents`, then lays one click over all of it, lit while the pointer
/// is on it. Laid after what it covers, so nothing drawn on the line takes the
/// click first. `id_salt` tells the lines of a list apart.
pub fn line_button(
    ui: &mut Ui,
    id_salt: impl AsIdSalt,
    contents: impl FnOnce(&mut Ui),
) -> Response {
    let fill = ui.painter().add(Shape::Noop);
    let rect = ui.scope(contents).response.rect;
    let response = ui
        .interact(rect, ui.id().with(id_salt), Sense::click())
        .on_hover_cursor(CursorIcon::PointingHand);
    if response.hovered() {
        ui.painter().set(
            fill,
            Shape::rect_filled(rect, RADIUS_BUTTON, BACKGROUND_HOVER),
        );
    }
    response
}
