use egui::{Label, Response, RichText, Ui};

use crate::ui::theme::tokens::TEXT_QUIET;

/// A quiet word beside what it speaks of — how many there are, say — read and
/// never selected: there is nothing in it anybody would copy.
pub fn quiet_note(ui: &mut Ui, said: &str) -> Response {
    ui.add(Label::new(RichText::new(said).color(TEXT_QUIET)).selectable(false))
}
