//! The panel the variables are edited in: one row each — its name, its
//! formula, what it comes to — and a row at the foot that adds one.

use cao_part::{PartDocument, VariableId};

use crate::lang::Catalogue;
use crate::screens::variables::state::{Problem, Row, VariablesPanel};
use crate::screens::variables::{NAMING, offered};
use crate::ui::dropping::grip;
use crate::ui::formula_field::{Offer, formula_field};
use crate::ui::text_edit::text_edit;
use crate::wording;

/// What the user asked of the panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VariablesAction {
    None,
    /// Ask the part for what the row being edited says.
    Commit,
    /// Drop what was typed into the row being edited.
    Cancel,
    /// Ask the part for the variable the foot of the table names.
    Add,
    Erase(VariableId),
}

/// How wide each column is, the same for the headers, the rows that scroll
/// and the row that adds one: the grip, the name, the formula, the value.
const GRIP_WIDTH: f32 = 40.0;
const NAME_WIDTH: f32 = 40.0;
const FORMULA_WIDTH: f32 = 50.0;
const VALUE_WIDTH: f32 = 40.0;

/// How few rows the list shows before it scrolls.
const FEWEST_ROWS: f32 = 2.0;

/// The panel, chrome and all, beside the history, and how far down the window
/// it reaches at the least, its rows scrolled to [`FEWEST_ROWS`].
pub fn panel(
    ui: &mut egui::Ui,
    panel: &mut VariablesPanel,
    document: &PartDocument,
    lang: &Catalogue,
) -> (VariablesAction, f32) {
    egui::Panel::left("variables_panel")
        .resizable(true)
        .default_size(320.0)
        .show(ui, |ui| show(ui, panel, document, lang))
        .inner
}

fn show(
    ui: &mut egui::Ui,
    panel: &mut VariablesPanel,
    document: &PartDocument,
    lang: &Catalogue,
) -> (VariablesAction, f32) {
    let mut action = VariablesAction::None;
    ui.heading(lang.t("variables.title"));
    ui.add_space(4.0);
    if let Some(problem) = panel.problem() {
        let said = match problem {
            Problem::Formula(wrong) => wording::formula::unusable(lang, wrong),
            Problem::Refused(refused) => wording::variables::refused(lang, document, refused),
        };
        ui.colored_label(ui.visuals().error_fg_color, said);
        ui.add_space(4.0);
    }

    let rows = panel.rows(document.variables());
    let offers = offered(document.variables());
    let lit = match panel.rows_blinking(ui.input(|input| input.time)) {
        Some((named, lit)) => {
            ui.ctx().request_repaint();
            if lit { named.to_vec() } else { Vec::new() }
        }
        None => Vec::new(),
    };
    egui::Grid::new("variables_headers")
        .num_columns(4)
        .show(ui, |ui| {
            cell(ui, GRIP_WIDTH, |_| ());
            cell(ui, NAME_WIDTH, |ui| ui.strong(lang.t("variables.name")));
            cell(ui, FORMULA_WIDTH, |ui| {
                ui.strong(lang.t("variables.formula"))
            });
            cell(ui, VALUE_WIDTH, |ui| ui.strong(lang.t("variables.value")));
            ui.end_row();
        });
    let foot_id = egui::Id::new("variables_foot_height");
    let foot = ui
        .data(|data| data.get_temp::<f32>(foot_id))
        .unwrap_or(ui.spacing().interact_size.y);
    let between = ui.spacing().item_spacing.y;
    // The row that adds one is laid out as a variable's, so it gives their height.
    let margin = egui::Margin {
        left: 0,
        ..ui.spacing().scroll.content_margin
    };
    let fewest = match rows.len() {
        0 => 0.0,
        shown => (shown as f32).min(FEWEST_ROWS) * (foot + between) + margin.sum().y,
    };
    let room = (ui.available_height() - foot - between).max(fewest);
    let above = ui.cursor().top();
    if !rows.is_empty() {
        egui::ScrollArea::vertical()
            .id_salt("variables_rows")
            .content_margin(margin)
            .max_height(room)
            .min_scrolled_height(fewest)
            .show(ui, |ui| {
                egui::Grid::new("variables_grid")
                    .num_columns(5)
                    .striped(true)
                    .show(ui, |ui| {
                        for row in &rows {
                            let lit = lit.contains(&row.variable);
                            if let Some(asked) = one_row(ui, panel, row, (lit, &offers), lang) {
                                action = asked;
                            }
                            ui.end_row();
                        }
                    });
            });
    }
    let scrolled = ui.cursor().top() - above;
    let adding = egui::Grid::new("variables_adding")
        .num_columns(5)
        .show(ui, |ui| {
            cell(ui, GRIP_WIDTH, |_| ());
            let name = cell(ui, NAME_WIDTH, |ui| {
                text_edit(
                    ui,
                    &mut panel.adding.name,
                    NAME_WIDTH,
                    &lang.t("variables.new_name"),
                )
            });
            let formula = cell(ui, FORMULA_WIDTH, |ui| {
                formula_field(
                    ui,
                    egui::Id::new("variable_formula_added"),
                    &mut panel.adding.formula,
                    (FORMULA_WIDTH, &lang.t("variables.new_formula")),
                    &offers,
                    NAMING,
                )
                .response
                .response
            });
            cell(ui, VALUE_WIDTH, |_| ());
            if ui.small_button(lang.t("variables.add")).clicked()
                || entered(ui, &name)
                || entered(ui, &formula)
            {
                action = VariablesAction::Add;
            }
            ui.end_row();
        });
    let height = adding.response.rect.height();
    if (height - foot).abs() > 0.5 {
        ui.data_mut(|data| data.insert_temp(foot_id, height));
        ui.ctx().request_repaint();
    }
    if rows.is_empty() {
        ui.weak(lang.t("variables.empty"));
    }
    let least = ui.cursor().top() - scrolled + fewest;
    (action, least)
}

/// One cell of a grid, `width` wide whatever it holds, so that three grids
/// stand in the same columns.
fn cell<R>(ui: &mut egui::Ui, width: f32, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let height = ui.spacing().interact_size.y;
    ui.allocate_ui(egui::vec2(width, height), |ui| {
        ui.set_min_width(width);
        add(ui)
    })
    .inner
}

/// One variable: its two fields, what it comes to, and the button that erases
/// it. A row is asked of the part when the keyboard leaves it, the way a
/// spreadsheet takes a cell. A row a refusal named and `lit` at this instant
/// is written in the colour of what is wrong; its formula completes a name
/// from what `offers` hands back.
fn one_row(
    ui: &mut egui::Ui,
    panel: &mut VariablesPanel,
    row: &Row,
    (lit, offers): (bool, &[Offer]),
    lang: &Catalogue,
) -> Option<VariablesAction> {
    let written_as = ui.visuals().override_text_color;
    if lit {
        ui.visuals_mut().override_text_color = Some(ui.visuals().error_fg_color);
    }
    cell(ui, GRIP_WIDTH, |ui| {
        grip(
            ui,
            egui::Id::new(("variable_grip", row.variable.0)),
            &row.name,
            &lang.t("variables.drag_hint"),
        )
    });
    let mut typed = panel.texts(row);
    let name = cell(ui, NAME_WIDTH, |ui| {
        text_edit(ui, &mut typed.name, NAME_WIDTH, "")
    });
    let formula = cell(ui, FORMULA_WIDTH, |ui| {
        formula_field(
            ui,
            egui::Id::new(("variable_formula", row.variable.0)),
            &mut typed.formula,
            (FORMULA_WIDTH, ""),
            offers,
            NAMING,
        )
        .response
        .response
    });
    if name.changed() || formula.changed() {
        panel.typed_into(row, typed.name, typed.formula);
    }
    cell(ui, VALUE_WIDTH, |ui| match row.value {
        Some(value) => ui.label(wording::dimension::short(value)),
        None => ui.weak(lang.t("variables.no_value")),
    });
    let erase = ui.small_button(lang.t("variables.erase")).clicked();
    ui.visuals_mut().override_text_color = written_as;

    // Escape takes the keyboard away before any field is drawn, so a field it
    // was pressed in reads as one the keyboard has just left.
    let holding = name.has_focus() || formula.has_focus();
    let left = name.lost_focus() || formula.lost_focus();
    if (holding || left) && ui.input(|input| input.key_pressed(egui::Key::Escape)) {
        return Some(VariablesAction::Cancel);
    }
    match (erase, left && !holding && panel.is_editing(row.variable)) {
        (true, _) => Some(VariablesAction::Erase(row.variable)),
        // An Enter that gave the keyboard back is eaten, as `entered` eats
        // its own, so that it commits the row and nothing drawn after it.
        (false, true) => {
            ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
            Some(VariablesAction::Commit)
        }
        (false, false) => None,
    }
}

/// Whether Enter was pressed in a field: it gives the keyboard back as it
/// goes, and is eaten so the shortcut bound to it does not fire too.
fn entered(ui: &mut egui::Ui, field: &egui::Response) -> bool {
    field.lost_focus()
        && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
}
