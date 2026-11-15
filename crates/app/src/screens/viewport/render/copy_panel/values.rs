//! The step of a pattern's number and step: its fields two by two, side by
//! side, each drawn as its kind asks — a count with its − and +, a step with
//! what it comes to. The rectangular pattern's two rows, along its direction
//! and across it, are named.

use super::super::super::input::count_brought_down;
use super::super::live_fields::{taken_whole_when_reached, value_field};
use super::Asked;
use super::steps::{Held, title};
use crate::screens::SketchContext;
use crate::screens::sketch::{CopySlot, CopyValue, Tool};
use crate::ui::number_field::number_field;
use crate::wording::dimension;

/// The fewest copies − leaves in the circular pattern's count: the original
/// and one more.
const LEAST_AROUND: f64 = 2.0;

/// The fewest − leaves in a count of the rectangular pattern: one copy is a
/// way it does not repeat in.
const LEAST_IN_A_ROW: f64 = 1.0;

/// What the rows of a pattern laid two ways are called, top to bottom.
const WAYS: [&str; 2] = ["copy_panel.along", "copy_panel.across"];

/// What the panel holds, and whether its active slot was reached this frame:
/// the field standing there then takes the keyboard.
type Reached<'a> = (&'a Held, bool);

/// The fields in the order `Entrée` walks them, two to a row; under them, what
/// a circular pattern's count brought down to a full turn says.
pub(super) fn count_and_step(
    ui: &mut egui::Ui,
    context: &mut SketchContext<'_>,
    reached: Reached<'_>,
    asked: &mut Asked,
) {
    let kinds = context
        .editor
        .copy_panel()
        .map_or(&[][..], |panel| panel.value_kinds());
    let rows: Vec<_> = kinds.chunks(2).enumerate().collect();
    for (row, pair) in rows.iter().copied() {
        if let Some(way) = WAYS.get(row).filter(|_| rows.len() > 1) {
            ui.weak(context.lang.t(way));
        }
        ui.columns(2, |columns| {
            for (column, (rank, kind)) in columns.iter_mut().zip(pair.iter().enumerate()) {
                field(column, context, (reached, 2 * row + rank, *kind), asked);
            }
        });
    }
    if context.editor.tool != Tool::CircularPattern {
        return;
    }
    let (lang, variables) = (context.lang, context.document.variables());
    if let Some(largest) = count_brought_down(&mut context.editor.live, variables) {
        ui.weak(crate::wording::pattern::count_brought_down(lang, largest));
    }
}

/// One field, as its kind asks.
fn field(
    ui: &mut egui::Ui,
    context: &mut SketchContext<'_>,
    (reached, rank, kind): (Reached<'_>, usize, CopyValue),
    asked: &mut Asked,
) {
    let least = match context.editor.tool {
        Tool::CircularPattern => LEAST_AROUND,
        _ => LEAST_IN_A_ROW,
    };
    match kind {
        CopyValue::Count => count_field(ui, context, (reached, rank, least), asked),
        CopyValue::StepInDegrees => {
            step_field(ui, context, (reached, rank, dimension::degrees), asked);
        }
        CopyValue::StepInMillimetres => {
            step_field(ui, context, (reached, rank, dimension::millimetres), asked);
        }
    }
}

/// « Nombre » over its field, with − and + down to `least`.
fn count_field(
    ui: &mut egui::Ui,
    context: &mut SketchContext<'_>,
    ((held, arrived), rank, least): (Reached<'_>, usize, f64),
    asked: &mut Asked,
) {
    let (lang, variables) = (context.lang, context.document.variables());
    let marked = held.marked == Some(CopySlot::Value(rank));
    if title(ui, lang.t("copy_panel.count"), marked) {
        asked.activate = Some(CopySlot::Value(rank));
    }
    let live = &mut context.editor.live;
    let current = held.active == CopySlot::Value(rank);
    let typed = &mut live.field(rank).text;
    let field = number_field(ui, typed, current, least);
    taken_whole_when_reached(ui.ctx(), &field, typed, arrived && current);
    typed_into(ui, &field, rank, asked);
    if field.changed() {
        live.take(rank, variables, None);
    }
}

/// « Pas » over its field, which takes a formula, and what it comes to beside
/// it, written by `comes_to`.
fn step_field(
    ui: &mut egui::Ui,
    context: &mut SketchContext<'_>,
    ((held, arrived), rank, comes_to): (Reached<'_>, usize, fn(f64) -> String),
    asked: &mut Asked,
) {
    let (lang, variables) = (context.lang, context.document.variables());
    let marked = held.marked == Some(CopySlot::Value(rank));
    if title(ui, lang.t("copy_panel.step"), marked) {
        asked.activate = Some(CopySlot::Value(rank));
    }
    let live = &mut context.editor.live;
    let reached = arrived && held.active == CopySlot::Value(rank);
    ui.horizontal(|ui| {
        let id = egui::Id::new(("copy_panel_value", rank));
        let typed = &mut live.field(rank).text;
        let field = value_field(ui, id, typed, "", reached, variables);
        let field = field.response.response;
        typed_into(ui, &field, rank, asked);
        if field.changed() {
            live.take(rank, variables, None);
        }
        if let Some(value) = live.typed(rank) {
            ui.weak(comes_to(value));
        }
    });
}

/// What a field of the panel asks for: its slot once it is reached or typed
/// into, and `Entrée` when the keyboard leaves it on that key.
fn typed_into(ui: &egui::Ui, field: &egui::Response, rank: usize, asked: &mut Asked) {
    if field.gained_focus() || field.changed() {
        asked.activate = Some(CopySlot::Value(rank));
    }
    if field.changed() {
        ui.ctx().request_repaint();
    }
    asked.enter |= field.lost_focus()
        && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
}
