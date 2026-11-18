use cao_part::PartDocument;
use cao_part::VolumeState;
use cao_part::history::ExtrusionMode;

use crate::lang::Catalogue;
use crate::screens::part_tree::state::{
    self, Body, Drawn, Kept, PartTree, Points, Row, TreeAction, Variable, clickable, clicked,
    marked,
};
use crate::screens::ribbon::short_of_the_tallest_tab;
use crate::screens::sketch::SketchEditor;
use crate::screens::theme_colours;
use crate::screens::viewport::ViewportState;
use crate::ui::icon;
use crate::ui::question::{Question, ask};
use crate::ui::secondary_button::secondary_button;
use crate::ui::theme::tokens;
use crate::ui::tree_row::{Eye, TreeRow, folding_row, tree_note, tree_row};
use crate::wording::part_tree::{self as said, Problem, Rubric};

mod history;
mod reader;
pub use history::compact_confirm;

/// What the tree reads to mark its rows, and the one thing asked of it.
struct Asked<'a> {
    document: &'a PartDocument,
    editor: &'a SketchEditor,
    viewport: &'a ViewportState,
    adding: bool,
    action: TreeAction,
    hovering: Option<Points>,
    ends: Vec<f32>,
}

impl Asked<'_> {
    /// Where the row just laid ends, a place the tree's list may stop at.
    fn ended(&mut self, ui: &egui::Ui) {
        self.ends.push(ui.cursor().top());
    }
}

/// The panel the tree is shown in, chrome and all, with the history folded at
/// its bottom unless `history_unfolded`: what was asked of it, and the row the
/// hand is on.
pub fn panel(
    ui: &mut egui::Ui,
    document: &PartDocument,
    editor: &SketchEditor,
    viewport: &ViewportState,
    history_unfolded: bool,
    lang: &Catalogue,
) -> (TreeAction, Option<Points>) {
    let mut asked = Asked {
        document,
        editor,
        viewport,
        adding: ui.input(|input| input.modifiers.command || input.modifiers.shift),
        action: TreeAction::None,
        hovering: None,
        ends: Vec::new(),
    };
    let shown = egui::Panel::left("part_tree_panel")
        .resizable(true)
        .default_size(240.0)
        .show(ui, |ui| {
            asked.action = history::at_the_bottom(ui, document, viewport, history_unfolded, lang);
            let room = ui.max_rect().height();
            show(ui, &mut asked, lang);
            room
        });
    let ctx = ui.ctx();
    hold_the_window(
        ctx,
        ctx.content_rect().height() - shown.inner + history::least_height(ctx),
    );
    (asked.action, asked.hovering)
}

/// Asks the window to stand no shorter than what is around the panel's place,
/// the panel closed, nor than where what the panels beside it hold `ends`:
/// `ui` is where it would stand.
pub fn closed(ui: &egui::Ui, ends: Option<f32>) {
    let ctx = ui.ctx();
    let room = ui.available_rect_before_wrap();
    let held = ends.map_or(0.0, |ends| (ends - room.top()).max(0.0));
    hold_the_window(ctx, ctx.content_rect().height() - room.height() + held);
}

/// Asks the window to stand no shorter than `least`; asked again only when
/// that or the zoom changes, as the platform resizes the window on each ask
/// and holds it in pixels of the zoom it was asked at.
fn hold_the_window(ctx: &egui::Context, least: f32) {
    let least = least + short_of_the_tallest_tab(ctx);
    let zoom = ctx.pixels_per_point();
    let id = egui::Id::new("part_tree_least_window_height");
    let asked = ctx.data(|data| data.get_temp::<(f32, f32)>(id));
    if asked.is_none_or(|(asked, at)| (asked - least).abs() > 0.5 || at != zoom) {
        ctx.data_mut(|data| data.insert_temp(id, (least, zoom)));
        ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(egui::vec2(0.0, least)));
    }
}

fn show(ui: &mut egui::Ui, asked: &mut Asked<'_>, lang: &Catalogue) {
    let tree = PartTree::seen(asked.document, asked.editor, asked.viewport, lang);

    ui.heading(lang.t("part_tree.title"));
    ui.add_space(4.0);

    if tree.is_empty() {
        ui.weak(lang.t("part_tree.empty"));
        return;
    }

    let id = egui::Id::new("part_tree_row_ends");
    let ends: Vec<f32> = ui.data(|data| data.get_temp(id)).unwrap_or_default();
    let room = ui.available_height();
    let fitting = ends.iter().copied().filter(|end| *end <= room);
    let on_a_whole_row = if ends.is_empty() {
        room
    } else {
        fitting.fold(0.0, f32::max)
    };
    let within_what_is_left = egui::ScrollArea::vertical()
        .max_height(on_a_whole_row)
        .min_scrolled_height(0.0);
    let laid = within_what_is_left.show(ui, |ui| {
        let margin = ui.spacing().scroll.content_margin;
        let top = ui.cursor().top() - margin.topf();
        ui.spacing_mut().item_spacing.y = 0.0;
        for (rank, rubric) in tree.rubrics().into_iter().enumerate() {
            if rank > 0 {
                ui.add_space(8.0);
            }
            ui.label(said::rubric(lang, rubric));
            asked.ended(ui);
            match rubric {
                Rubric::Sketches => {
                    for drawing in &tree.sketches {
                        drawn(ui, drawing, asked, lang);
                    }
                }
                Rubric::Volumes => {
                    for body in &tree.bodies {
                        made_of_matter(ui, body, asked, lang);
                    }
                    if let Some(offered) = &tree.make_all_opaque {
                        make_all_opaque(ui, offered, asked);
                    }
                }
                Rubric::Sections => {
                    for kept in &tree.sections {
                        section(ui, kept, asked, lang);
                    }
                }
                Rubric::Variables => variables(ui, &tree.variables, asked, lang),
            }
        }
        asked.ends.push(ui.cursor().top() + margin.bottomf());
        asked
            .ends
            .drain(..)
            .map(|end| end - top)
            .collect::<Vec<f32>>()
    });
    if laid.inner != ends {
        ui.data_mut(|data| data.insert_temp(id, laid.inner));
        ui.ctx().request_repaint();
    }
}

/// « Tout rendre opaque », under the volumes, greyed while they all are.
fn make_all_opaque(ui: &mut egui::Ui, offered: &state::Offered, asked: &mut Asked<'_>) {
    ui.add_space(tokens::SPACING_S);
    if secondary_button(ui, offered.enabled, &offered.label).clicked() {
        asked.action = TreeAction::MakeAllOpaque;
    }
    asked.ended(ui);
}

/// The variables, read only: the name in the variable colour, the value on
/// the right.
fn variables(ui: &mut egui::Ui, variables: &[Variable], asked: &mut Asked<'_>, lang: &Catalogue) {
    if variables.is_empty() {
        tree_note(ui, 0, &lang.t("part_tree.no_variables"));
        asked.ended(ui);
    }
    for variable in variables {
        tree_row(
            ui,
            TreeRow {
                name: &variable.name,
                ink: Some(tokens::VARIABLE_TEXT),
                inert: true,
                value: Some(&variable.value),
                ..TreeRow::default()
            },
        );
        asked.ended(ui);
    }
}

/// One kept section: a click switches it on or off, a double-click reopens it,
/// and its menu deletes it.
fn section(ui: &mut egui::Ui, kept: &Kept, asked: &mut Asked<'_>, lang: &Catalogue) {
    let badge = kept.badge.map(|found| said::badge(lang, found));
    let row = TreeRow {
        name: &kept.name,
        selected: asked.viewport.kept_sections.is_on(kept.id),
        badge: badge
            .as_deref()
            .zip(kept.badge.map(|found| worn(asked, found))),
        ..TreeRow::default()
    };
    let response = tree_row(ui, row).row;
    asked.ended(ui);
    if response.double_clicked() {
        asked.action = TreeAction::EditSection(kept.id);
    } else if response.clicked() {
        asked.action = TreeAction::ToggleSection(kept.id);
    }
    response.context_menu(|ui| {
        if ui.button(lang.t("part_tree.delete")).clicked() {
            asked.action = TreeAction::DeleteSection(kept.id);
            ui.close();
        }
    });
}

/// The patch and the ink of a badge saying `found`, as the theme has its level.
fn worn(asked: &Asked<'_>, found: Problem) -> (egui::Color32, egui::Color32) {
    theme_colours::worn(&asked.viewport.theme, found.level())
}

/// The question asked before deleting a section a drawing stands on.
pub fn section_delete_confirm(ui: &egui::Ui, lang: &Catalogue) -> Option<bool> {
    let question = Question {
        title: lang.t("part_tree.delete_section_title"),
        body: lang.t("part_tree.delete_section_body"),
        yes: lang.t("part_tree.delete"),
        no: lang.t("part_tree.delete_section_cancel"),
    };
    ask(ui, "section_delete_confirm", &question)
}

/// One volume: C11's button stepping it round its three states, whether it
/// adds or cuts, its name, its badge and its state written on the right.
fn made_of_matter(ui: &mut egui::Ui, body: &Body, asked: &mut Asked<'_>, lang: &Catalogue) {
    let badge = body.badge.map(|found| said::badge(lang, found));
    let made = match body.mode {
        ExtrusionMode::Add => (icon::EXTRUSION_ADD, tokens::VOLUME_ADD),
        ExtrusionMode::Cut => (icon::EXTRUSION_CUT, tokens::VOLUME_CUT),
    };
    let drawn = match body.shown {
        VolumeState::Opaque => icon::BODY_OPAQUE,
        VolumeState::SeeThrough => icon::BODY_SEE_THROUGH,
        VolumeState::Invisible => icon::BODY_HIDDEN,
    };
    let heading = TreeRow {
        name: &body.name,
        selected: body.selected,
        dimmed: body.dimmed,
        value: Some(&body.state),
        badge: badge
            .as_deref()
            .zip(body.badge.map(|found| worn(asked, found))),
        eye: Some(Eye::Pictured(drawn, &body.visibility_button)),
        icon: Some(made),
        ..TreeRow::default()
    };
    let id = egui::Id::new(("body", body.step));
    let (touched, open) = folding_row(ui, id, false, heading);
    asked.ended(ui);
    if touched.eye {
        asked.action = TreeAction::NextVolumeMode(body.step);
    }
    if let Some(said) = &body.unstable {
        touched.row.on_hover_text(said);
    }
    if open {
        tree_note(ui, 1, &body.reads);
        asked.ended(ui);
        for area in &body.areas {
            row(ui, area, 1, false, asked);
        }
    }
}

/// One sketch, its eye beside it, and what is drawn on it once unfolded: a
/// single click folds; only a double-click reopens the sketch, so that walking
/// the tree cannot drop the user into drawing by accident. The sketch being
/// drawn on stands selected and open.
fn drawn(ui: &mut egui::Ui, drawing: &Drawn, asked: &mut Asked<'_>, lang: &Catalogue) {
    let lit = asked.viewport.eye_lit(asked.document, drawing.sketch);
    let eye = said::eye(lang, lit);
    let badge = drawing.badge.map(|found| said::badge(lang, found));
    let heading = TreeRow {
        name: &drawing.name,
        selected: drawing.selected,
        dimmed: drawing.dimmed,
        value: Some(&drawing.plane),
        badge: badge
            .as_deref()
            .zip(drawing.badge.map(|found| worn(asked, found))),
        eye: Some(Eye::Open(lit, &eye)),
        ..TreeRow::default()
    };
    let id = egui::Id::new(("sketch", drawing.step));
    let (touched, unfolded) = folding_row(ui, id, drawing.open, heading);
    asked.ended(ui);
    if touched.eye {
        asked.action = TreeAction::ToggleEye(drawing.sketch);
    }
    if touched.row.double_clicked() {
        asked.action = TreeAction::EditSketch(drawing.sketch);
    }
    if !unfolded {
        return;
    }
    for stroke in &drawing.strokes {
        row(ui, stroke, 1, drawing.dimmed, asked);
    }
    for (rank, (key, rows)) in [
        ("part_tree.areas", &drawing.areas),
        ("part_tree.dimensions", &drawing.dimensions),
        ("part_tree.rules", &drawing.rules),
    ]
    .into_iter()
    .enumerate()
    {
        kind(
            ui,
            asked,
            &lang.t(key),
            rows,
            (drawing.step, rank),
            drawing.dimmed,
        );
    }
}

fn kind(
    ui: &mut egui::Ui,
    asked: &mut Asked<'_>,
    heading: &str,
    rows: &[Row],
    salt: (usize, usize),
    dimmed: bool,
) {
    if rows.is_empty() {
        return;
    }
    let count = rows.len().to_string();
    let named = TreeRow {
        name: heading,
        depth: 1,
        dimmed,
        value: Some(&count),
        ..TreeRow::default()
    };
    let unfolded = folding_row(ui, egui::Id::new(("kind", salt)), false, named).1;
    asked.ended(ui);
    if unfolded {
        for one in rows {
            row(ui, one, 2, dimmed, asked);
        }
    }
}

/// One row: a click where it asks something, marked while the open sketch
/// holds it or the canvas lights it.
fn row(ui: &mut egui::Ui, row: &Row, depth: usize, dimmed: bool, asked: &mut Asked<'_>) {
    let (document, editor) = (asked.document, asked.editor);
    let asks = clickable(document, editor, row.points);
    let named = TreeRow {
        name: &row.name,
        depth,
        dimmed,
        inert: !asks,
        selected: marked(document, editor, asked.viewport, row.points),
        value: row.value.as_deref(),
        ..TreeRow::default()
    };
    let response = tree_row(ui, named).row;
    asked.ended(ui);
    if !asks {
        return;
    }
    if response.hovered() {
        asked.hovering = Some(row.points);
    }
    if response.clicked() {
        asked.action = clicked(document, editor, row.points, asked.adding);
    }
}
