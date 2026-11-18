//! The history, folded at the bottom of the Part panel under the step
//! reader's card: its header counts the actions, and unfolded it lists them,
//! one line each, newest last.

use cao_part::PartDocument;
use cao_part::history::BlendShape;

use super::reader::reader;
use crate::lang::Catalogue;
use crate::screens::part_tree::state::{History, TreeAction};
use crate::screens::viewport::ViewportState;
use crate::ui::history_line::{self, HistoryLine, history_line};
use crate::ui::question::{Question, ask};
use crate::ui::quiet_link::quiet_link;
use crate::ui::reader_card;
use crate::ui::theme::tokens::SPACING_S;
use crate::ui::tree_row::{self, TreeRow, tree_row};

/// How many lines the unfolded list shows before it scrolls, and how few
/// while they fit in the panel: for those, the tree gives way.
const MOST_LINES: f32 = 6.0;
const FEWEST_LINES: f32 = 2.0;

/// What of the Part panel's height the tree above keeps, its title and three
/// rows: the history's list scrolls more rather than take it.
const THE_TREE_KEEPS: f32 = 4.0 * tree_row::HEIGHT;

/// The step reader's card, then the history, in a panel of their own along
/// the bottom of the Part panel, as tall as what they hold short of
/// [`THE_TREE_KEEPS`].
///
/// A bottom panel is laid out at the height it had the frame before and only
/// ever grows, so it is given the height its content took last time, and asks
/// for one more frame whenever that changes.
pub(super) fn at_the_bottom(
    ui: &mut egui::Ui,
    document: &PartDocument,
    viewport: &ViewportState,
    unfolded: bool,
    lang: &Catalogue,
) -> TreeAction {
    let frame = frame();
    let margins = frame.total_margin().sum().y;
    let id = egui::Id::new("part_history_height");
    let height = ui.data(|data| data.get_temp::<f32>(id)).unwrap_or(margins);
    let whole = ui.available_height() - margins;
    let shown = egui::Panel::bottom("part_history_panel")
        .resizable(false)
        .show_separator_line(false)
        .frame(frame)
        .exact_size(height)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                let bottom = ui.cursor().top() + whole;
                let floor = bottom - THE_TREE_KEEPS;
                let read = reader(ui, document, viewport, lang);
                ui.separator();
                match history(ui, document, unfolded, [floor, bottom], lang) {
                    TreeAction::None => read,
                    asked => asked,
                }
            })
        });
    let wanted = shown.inner.response.rect.height() + margins;
    if (wanted - height).abs() > 0.5 {
        ui.data_mut(|data| data.insert_temp(id, wanted));
        ui.ctx().request_repaint();
    }
    shown.inner.inner
}

/// How tall the Part panel stands at the least: the tree's share, then the
/// card at its tallest, the header, [`FEWEST_LINES`] and the link, as
/// [`at_the_bottom`] lays them.
pub(super) fn least_height(ctx: &egui::Context) -> f32 {
    let style = ctx.global_style();
    let spacing = &style.spacing;
    let separator = style.separator_style(&Default::default(), Default::default());
    let history = tree_row::HEIGHT
        + FEWEST_LINES * history_line::HEIGHT
        + spacing.scroll.content_margin.topf()
        + link(spacing);
    let stacked = [reader_card::tallest(ctx), separator.spacing, history];
    let between = (stacked.len() - 1) as f32 * spacing.item_spacing.y;
    THE_TREE_KEEPS + frame().total_margin().sum().y + stacked.iter().sum::<f32>() + between
}

fn frame() -> egui::Frame {
    egui::Frame::NONE.inner_margin(egui::Margin::symmetric(0, SPACING_S as i8))
}

/// The room « Compacter l'historique… » takes under the lines.
fn link(spacing: &egui::Spacing) -> f32 {
    SPACING_S + spacing.interact_size.y
}

/// The header, and the lines under it while `unfolded`, their list ending on a
/// whole line above `floor`, or above `bottom` while [`FEWEST_LINES`] fit
/// there; what was asked of them. Nothing at all while nothing was done to the
/// part. Folded, the lines are not even worked out.
fn history(
    ui: &mut egui::Ui,
    document: &PartDocument,
    unfolded: bool,
    [floor, bottom]: [f32; 2],
    lang: &Catalogue,
) -> TreeAction {
    let Some(actions) = History::actions_of(document, lang) else {
        return TreeAction::None;
    };
    let mut action = TreeAction::None;
    ui.spacing_mut().item_spacing.y = 0.0;
    let title = lang.t("part_tree.history.title");
    let header = TreeRow {
        name: &title,
        unfolded: Some(unfolded),
        value: Some(&actions),
        ..TreeRow::default()
    };
    if tree_row(ui, header).row.clicked() {
        action = TreeAction::ToggleHistory;
    }
    if !unfolded {
        return action;
    }
    let history = History::of(document, lang);
    let link = link(ui.spacing());
    let fitting = |limit: f32| ((limit - ui.cursor().top() - link) / history_line::HEIGHT).floor();
    let lines = fitting(floor)
        .max(fitting(bottom).min(FEWEST_LINES))
        .clamp(0.0, MOST_LINES);
    let margin = ui.spacing().scroll.content_margin;
    let all_of_it = history.lines.len() as f32 <= lines;
    let tallest = lines * history_line::HEIGHT
        + margin.topf()
        + if all_of_it { margin.bottomf() } else { 0.0 };
    egui::ScrollArea::vertical()
        .id_salt("part_history")
        .max_height(tallest)
        .min_scrolled_height(tallest)
        .show(ui, |ui| {
            for line in &history.lines {
                let shown = HistoryLine {
                    number: line.number,
                    name: &line.name,
                    detail: &line.detail,
                    current: line.current,
                    undone: line.undone,
                    warned: line.capped.is_some(),
                };
                let mut response = history_line(ui, shown);
                if let Some(said) = &line.capped {
                    response = response.on_hover_text(said);
                }
                if response.clicked() {
                    action = TreeAction::RewindTo(line.back_to);
                }
                let offered = match (line.sketch, line.blend) {
                    (Some(sketch), _) => Some((
                        "part_tree.history.edit_sketch",
                        TreeAction::EditSketch(sketch),
                    )),
                    (None, Some((step, BlendShape::Fillet))) => {
                        Some(("part_tree.history.edit_fillet", TreeAction::EditBlend(step)))
                    }
                    (None, Some((step, BlendShape::Chamfer))) => Some((
                        "part_tree.history.edit_chamfer",
                        TreeAction::EditBlend(step),
                    )),
                    (None, None) => None,
                };
                if let Some((key, asked)) = offered {
                    response.context_menu(|ui| {
                        if ui.button(lang.t(key)).clicked() {
                            action = asked;
                            ui.close();
                        }
                    });
                }
            }
        });
    ui.add_space(SPACING_S);
    if quiet_link(ui, &lang.t("part_tree.history.compact")).clicked() {
        action = TreeAction::AskToCompact;
    }
    action
}

/// The question asked before compacting: `Some(true)` once confirmed,
/// `Some(false)` once backed out of, `None` while it is still asked.
pub fn compact_confirm(ui: &egui::Ui, lang: &Catalogue) -> Option<bool> {
    let question = Question {
        title: lang.t("part_tree.history.compact_confirm_title"),
        body: lang.t("part_tree.history.compact_confirm_body"),
        yes: lang.t("part_tree.history.compact_confirm_ok"),
        no: lang.t("part_tree.history.compact_confirm_cancel"),
    };
    ask(ui, "history_compact_confirm", &question)
}
