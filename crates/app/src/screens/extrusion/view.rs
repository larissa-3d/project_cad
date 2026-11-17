//! The extrusion's panel over the view, as P12 of
//! `docs/redesign/part-screen.md` draws it: the areas chosen, the matter, the
//! depth — or, around an axis, the angle and the axis — the two switches,
//! and « Annuler » / « Appliquer ↵ ». Before a sketch is offered, the
//! sketches to extrude stand at the top left of the view, under the sketch's
//! pill.

use cao_part::{ExtrusionMode, RevolutionAxis};
use cao_prefs::Command;
use cao_sketch::SketchAxis;

use super::state::{Extent, ExtrusionState, Panel, Shape};
use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::sketch_pill::BOTTOM_BELOW_THE_EDGE;
use crate::screens::variables::{NAMING, offered};
use crate::ui::chip::{ChipKind, chip};
use crate::ui::floating_panel::tool_panel;
use crate::ui::formula_field::{Offer, formula_field};
use crate::ui::primary_button::primary_button;
use crate::ui::secondary_button::secondary_button;
use crate::ui::theme::tokens::SPACING_XXL;
use crate::wording::{self, constraints};

/// Draws the panel while an extrusion is being set, or the sketches `read`
/// to choose from while none is offered — nothing when `read` is empty — and
/// says what was asked in it. Open beside another panel, it stands at
/// `beside`, and is moved there apart from where it stands alone, so that it
/// goes back there once the other is closed.
pub(crate) fn paint_extrusion_panel(
    ui: &egui::Ui,
    (view, beside): (egui::Rect, Option<egui::Pos2>),
    read: &[usize],
    context: &mut SketchContext<'_>,
) -> Option<Command> {
    let (id, left_at) = match beside {
        Some(at) => (egui::Id::new("extrusion_panel_beside"), at),
        None => (
            egui::Id::new("extrusion_panel"),
            view.min + egui::vec2(SPACING_XXL, SPACING_XXL),
        ),
    };
    let Some(panel) = context.extrusion.panel(context.document) else {
        if context.extrusion.sketch.is_none() {
            let under_the_pill = egui::vec2(SPACING_XXL, BOTTOM_BELOW_THE_EDGE + SPACING_XXL);
            choose_a_sketch(ui.ctx(), view.min + under_the_pill, read, context);
        }
        return None;
    };
    let lang = context.lang;
    let title = wording::command::label(
        lang,
        match panel.shape {
            Shape::Straight => Command::ExtrusionStraight,
            Shape::Revolution => Command::ExtrusionRevolution,
        },
    );
    let named = wording::sketch::name(lang, context.document, panel.sketch);
    let close = lang.t("panel.close");
    let heading = (title.as_str(), named.as_str(), close.as_str());
    let floated = tool_panel(ui.ctx(), id, heading, (left_at, view), |ui| {
        sections(ui, &panel, context)
    });
    let entered = panel.applies
        && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
    match floated.closed {
        true => {
            context.extrusion.forget_the_drag();
            Some(Command::ExtrusionCancel)
        }
        false => floated.inner.or(entered.then_some(Command::ExtrusionApply)),
    }
}

/// The sketches the part is read through, a button each, standing at `at`:
/// clicking one offers it to extrude.
fn choose_a_sketch(
    ctx: &egui::Context,
    at: egui::Pos2,
    read: &[usize],
    context: &mut SketchContext<'_>,
) {
    if read.is_empty() {
        return;
    }
    let area = egui::Area::new(egui::Id::new("extrusion_sketches"));
    area.order(egui::Order::Foreground)
        .fixed_pos(at)
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(context.lang.t("extrusion.pick_a_sketch"));
                    for &index in read {
                        let number = wording::sketch::number(context.document, index);
                        if ui.button(number.to_string()).clicked() {
                            context.extrusion.offer(index);
                        }
                    }
                });
            });
        });
}

/// What the panel holds, section by section, and the command a click in it
/// asked for.
fn sections(ui: &mut egui::Ui, panel: &Panel, context: &mut SketchContext<'_>) -> Option<Command> {
    let lang = context.lang;
    let offers = offered(context.document.variables());
    areas(ui, lang, &panel.areas);
    let mut asked = matter(ui, lang, panel.matter);
    extent(ui, panel, (lang, &offers), context.extrusion);
    if let Some(wrong) = context.extrusion.wrong(context.document.variables()) {
        let said = wording::formula::unusable(lang, &wrong);
        ui.colored_label(ui.visuals().error_fg_color, said);
    }

    let mut around = panel.around_an_axis;
    let reshaped = ui
        .add_enabled_ui(panel.shape_can_change, |ui| {
            ui.checkbox(&mut around, lang.t("extrusion.around_an_axis"))
                .changed()
        })
        .inner;
    if reshaped {
        asked = Some(match around {
            true => Command::ExtrusionRevolution,
            false => Command::ExtrusionStraight,
        });
    }
    let mut reversed = panel.reversed;
    ui.checkbox(&mut reversed, lang.t("extrusion.reversed"))
        .on_hover_text(lang.t("extrusion.reversed_hint"));
    context.extrusion.reversed = reversed;

    ui.horizontal(|ui| {
        let right = egui::Layout::right_to_left(egui::Align::Center);
        ui.with_layout(right, |ui| {
            let apply = ui.add_enabled_ui(panel.applies, |ui| {
                primary_button(ui, &lang.t("extrusion.apply_enter"))
            });
            if apply.inner.clicked() {
                asked = Some(Command::ExtrusionApply);
            }
            if secondary_button(ui, true, &lang.t("extrusion.cancel")).clicked() {
                asked = Some(Command::ExtrusionCancel);
            }
        });
    });
    asked
}

/// « Aires »: a chip for each area chosen, and the invitation to click more
/// in the view.
fn areas(ui: &mut egui::Ui, lang: &Catalogue, areas: &[usize]) {
    ui.strong(lang.t("extrusion.areas"));
    ui.horizontal_wrapped(|ui| {
        for area in areas {
            let rank = (area + 1).to_string();
            chip(
                ui,
                &lang.t_with("extrusion.area", &[("rank", &rank)]),
                ChipKind::Selection,
            );
        }
        chip(
            ui,
            &lang.t("copy_panel.click_in_the_view"),
            ChipKind::Invitation,
        );
    });
}

/// « Matière »: « Ajout » / « Enlèvement », the one in hand lit. Asks for
/// the ribbon's own command, so that the two always agree.
fn matter(ui: &mut egui::Ui, lang: &Catalogue, held: ExtrusionMode) -> Option<Command> {
    ui.strong(lang.t("extrusion.matter"));
    ui.horizontal(|ui| {
        let mut asked = None;
        for (mode, key, command) in [
            (
                ExtrusionMode::Add,
                "extrusion.matter_add",
                Command::ExtrusionAdd,
            ),
            (
                ExtrusionMode::Cut,
                "extrusion.matter_cut",
                Command::ExtrusionCut,
            ),
        ] {
            if ui.selectable_label(held == mode, lang.t(key)).clicked() {
                asked = Some(command);
            }
        }
        asked
    })
    .inner
}

/// « Profondeur » and its field, with what it comes to on the right; around
/// an axis, the angle the same way, and the axis.
fn extent(
    ui: &mut egui::Ui,
    panel: &Panel,
    (lang, offers): (&Catalogue, &[Offer]),
    extrusion: &mut ExtrusionState,
) {
    match panel.extent {
        Extent::Depth { millimetres } => {
            ui.strong(lang.t("extrusion.depth"));
            let field = (&mut extrusion.distance_input, "extrusion_distance", 70.0);
            let comes_to = millimetres.map(|value| {
                let value = wording::dimension::short(value);
                lang.t_with("extrusion.comes_to_millimetres", &[("value", &value)])
            });
            typed(ui, field, ("mm", offers), comes_to);
        }
        Extent::Turn { degrees, axis } => {
            ui.strong(lang.t("extrusion.angle"));
            let field = (&mut extrusion.angle_input, "extrusion_angle", 60.0);
            let comes_to = degrees.map(|value| {
                let value = wording::dimension::short(value);
                lang.t_with("extrusion.comes_to_degrees", &[("value", &value)])
            });
            typed(ui, field, ("°", offers), comes_to);
            ui.horizontal_wrapped(|ui| turned_about(ui, lang, axis, &mut extrusion.axis));
        }
    }
}

/// A formula field and, on the right, what it comes to.
fn typed(
    ui: &mut egui::Ui,
    (text, id, width): (&mut String, &str, f32),
    (hint, offers): (&str, &[Offer]),
    comes_to: Option<String>,
) {
    ui.horizontal(|ui| {
        formula_field(ui, egui::Id::new(id), text, (width, hint), offers, NAMING);
        if let Some(comes_to) = comes_to {
            let right = egui::Layout::right_to_left(egui::Align::Center);
            ui.with_layout(right, |ui| ui.label(comes_to));
        }
    });
}

/// The axis turned about: U, V, or the trait clicked in the view.
fn turned_about(
    ui: &mut egui::Ui,
    lang: &Catalogue,
    held: RevolutionAxis,
    axis: &mut RevolutionAxis,
) {
    ui.label(lang.t("extrusion.around"));
    for sketch_axis in [SketchAxis::U, SketchAxis::V] {
        let this = RevolutionAxis::Sketch(sketch_axis);
        let name = constraints::axis(lang, sketch_axis);
        if ui.selectable_label(held == this, name).clicked() {
            *axis = this;
        }
    }
    if let RevolutionAxis::Segment(segment) = held {
        let segment = segment.0.to_string();
        ui.selectable_label(
            true,
            lang.t_with("extrusion.segment_axis", &[("segment", &segment)]),
        )
        .on_hover_text(lang.t("extrusion.change_the_axis_trait"));
    } else {
        ui.weak(lang.t("extrusion.or_click_a_trait"));
    }
}
