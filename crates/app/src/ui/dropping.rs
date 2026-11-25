//! A name dragged from where it is listed and dropped into a field: taken by
//! a grip beside it, and put in where it lands — in place of the text
//! selected or of a plain number, or else at the point of the text nearest
//! the drop. And a row dragged onto another, carrying the path it stands for.

use std::ops::Range;
use std::path::{Path, PathBuf};

use super::formula_field::{place_the_cursor, replaced};

/// What is carried while a name is dragged.
pub struct DraggedName(pub String);

/// What is carried while a row is dragged onto another.
pub struct DraggedPath(pub PathBuf);

/// Lets the hand take `row` away, carrying `path`. The row still answers a
/// click: a press that does not travel is not a drag.
pub fn carry(row: egui::Response, path: &Path) -> egui::Response {
    let row = row.interact(egui::Sense::drag());
    if row.drag_started() {
        row.dnd_set_drag_payload(DraggedPath(path.to_path_buf()));
    }
    row
}

/// The path let go over `target`, which is outlined while one is carried
/// over it.
pub fn take_a_dropped_path(ui: &egui::Ui, target: &egui::Response) -> Option<PathBuf> {
    if target.dnd_hover_payload::<DraggedPath>().is_some() {
        outline(ui, target.rect);
    }
    target
        .dnd_release_payload::<DraggedPath>()
        .map(|dropped| dropped.0.clone())
}

fn outline(ui: &egui::Ui, rect: egui::Rect) {
    ui.painter().rect_stroke(
        rect,
        2.0,
        ui.visuals().selection.stroke,
        egui::StrokeKind::Outside,
    );
}

/// The mark a row is taken by.
const GRIP: &str = "☰";

/// A grip that drags `name` away, saying `hint` when hovered. While it is
/// dragged, the name itself follows the pointer.
pub fn grip(ui: &mut egui::Ui, id: egui::Id, name: &str, hint: &str) -> egui::Response {
    let dragged = ui.ctx().is_being_dragged(id);
    ui.dnd_drag_source(id, DraggedName(name.to_string()), |ui| {
        ui.label(if dragged { name } else { GRIP })
    })
    .response
    .on_hover_text(hint)
}

/// Puts a name dropped on the field `id` into its `text`, and says whether
/// one was. The field then holds the keyboard, its cursor after the name,
/// and reads as changed, as typing the name would have left it. While a name
/// is carried over it, the field is outlined.
pub fn take_a_dropped_name(
    ui: &egui::Ui,
    id: egui::Id,
    output: &mut egui::text_edit::TextEditOutput,
    text: &mut String,
) -> bool {
    let response = &output.response.response;
    if response.dnd_hover_payload::<DraggedName>().is_some() {
        outline(ui, response.rect);
    }
    let Some(dropped) = response.dnd_release_payload::<DraggedName>() else {
        return false;
    };
    let selected = output
        .state
        .cursor
        .char_range()
        .filter(|range| !range.is_empty())
        .map(|range| {
            let sorted = range.as_sorted_char_range();
            sorted.start.0..sorted.end.0
        });
    let at = ui.ctx().pointer_latest_pos().map_or(usize::MAX, |pointer| {
        output
            .galley
            .cursor_from_pos(pointer - output.galley_pos)
            .index
            .0
    });
    let (put, cursor) = dropped_into(text, selected, at, &dropped.0);
    *text = put;
    place_the_cursor(ui.ctx(), id, cursor);
    ui.memory_mut(|memory| memory.request_focus(id));
    output.response.response.mark_changed();
    true
}

/// The text with `name` in place of what is `selected`, or else of a plain
/// number, or else put in at `at`; and the character the cursor stands at
/// after it.
///
/// A plain number gives way whole: it is the value a field opened on, and a
/// field loses what it had selected the moment the keyboard leaves it —
/// which reaching for a name to drag does. A name dropped into a calculation
/// being written goes where it lands.
fn dropped_into(
    text: &str,
    selected: Option<Range<usize>>,
    at: usize,
    name: &str,
) -> (String, usize) {
    let length = text.chars().count();
    let plain = text.trim();
    let a_plain_number = plain.is_empty() || plain.replace(',', ".").parse::<f64>().is_ok();
    let taken = match (selected, a_plain_number) {
        (Some(selected), _) => selected,
        (None, true) => 0..length,
        (None, false) => at.min(length)..at.min(length),
    };
    replaced(text, &taken, name)
}

#[cfg(test)]
mod tests;
