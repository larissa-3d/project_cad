//! The numbered steps of a copy tool's panel, as P13 of
//! `docs/redesign/part-screen.md` draws them: the elements to copy; what the
//! copy turns about or runs along — the circular pattern's centre, the
//! mirror's axis, the rectangular pattern's direction; a pattern's number and
//! step; the circular pattern's direction. Then « Annuler » and « Valider ↵ ».

use cao_sketch::PointId;

use super::super::step_panel::{foot, step, title};
use super::{Asked, values};
use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::sketch::{
    CopyPanel, CopySlot, CopyStep, CopyTarget, Rotation, StepState, Tool,
};
use crate::ui::chip::{ChipKind, chip};
use crate::ui::primary_button::primary_button;
use crate::ui::quiet_link::quiet_link;
use crate::ui::secondary_button::secondary_button;
use crate::ui::theme::tokens::SPACING_XXL;
use crate::wording::copy_panel as said;

/// What the panel holds, read before anything of it is drawn.
pub(super) struct Held {
    steps: Vec<(CopyStep, StepState)>,
    pub(super) active: CopySlot,
    pub(super) marked: Option<CopySlot>,
    picked: Vec<String>,
    target: Option<String>,
    rotation: Rotation,
}

impl Held {
    fn of(panel: &CopyPanel, context: &SketchContext<'_>) -> Self {
        let target = panel.target().and_then(|target| match target {
            CopyTarget::Centre(point) => centre_at(context, point),
            CopyTarget::Axis(axis) => Some(said::axis(context.lang, axis)),
        });
        Self {
            steps: panel.steps(&context.editor.live),
            active: panel.active(),
            marked: panel.marked(),
            picked: panel
                .picked()
                .map(|element| said::element(context.lang, element))
                .collect(),
            target,
            rotation: panel.rotation(),
        }
    }
}

/// Draws the tool's steps and the two buttons, and says what was asked in
/// them.
pub(super) fn steps(ui: &mut egui::Ui, context: &mut SketchContext<'_>, arrived: bool) -> Asked {
    let mut asked = Asked::default();
    let Some(panel) = context.editor.copy_panel() else {
        return asked;
    };
    let held = Held::of(panel, context);
    let lang = context.lang;
    let to_copy = match context.editor.tool {
        Tool::CircularPattern => "copy_panel.elements_to_copy",
        _ => "copy_panel.elements_title",
    };

    ui.spacing_mut().item_spacing.y = SPACING_XXL;
    for (rank, (copy_step, state)) in held.steps.iter().copied().enumerate() {
        let dot = step(ui, (rank + 1, state), |ui| match copy_step {
            CopyStep::Elements => elements(ui, (lang, to_copy), &held, &mut asked),
            CopyStep::Centre => target(ui, (lang, "copy_panel.centre"), &held, &mut asked),
            CopyStep::Axis => target(ui, (lang, "copy_panel.axis"), &held, &mut asked),
            CopyStep::Direction => target(ui, (lang, "copy_panel.direction"), &held, &mut asked),
            CopyStep::CountAndStep => {
                values::count_and_step(ui, context, (&held, arrived), &mut asked);
            }
            CopyStep::Rotation => rotation(ui, lang, &held, &mut asked),
        });
        if dot {
            asked.activate = Some(first_slot(copy_step));
        }
    }
    if let Some(panel) = context.editor.copy_panel.as_mut() {
        panel.unmark_if_filled(&context.editor.live);
    }
    footer(ui, lang, &mut asked);
    asked
}

/// The slot a click on a step's dot makes active: the first it asks for.
fn first_slot(step: CopyStep) -> CopySlot {
    match step {
        CopyStep::Elements => CopySlot::Elements,
        CopyStep::Centre | CopyStep::Axis | CopyStep::Direction => CopySlot::Target,
        CopyStep::CountAndStep => CopySlot::Value(0),
        CopyStep::Rotation => CopySlot::Rotation,
    }
}

/// The centre chosen, by where it stands on the sketch.
fn centre_at(context: &SketchContext<'_>, point: PointId) -> Option<String> {
    let index = context.editor.active_sketch()?;
    let at = *context
        .document
        .sketches()
        .get(index)?
        .points()
        .get(point.0)?;
    let millimetres = |units| context.document.to_millimeters(units);
    Some(said::centre_at(
        context.lang,
        (millimetres(at.x), millimetres(at.y)),
    ))
}

/// What is held, a chip each, and the invitation to click more in the view.
/// `titled` is the key of the step's title.
fn elements(ui: &mut egui::Ui, (lang, titled): (&Catalogue, &str), held: &Held, asked: &mut Asked) {
    let marked = held.marked == Some(CopySlot::Elements);
    let mut clicked = title(ui, lang.t(titled), marked);
    ui.horizontal_wrapped(|ui| {
        for name in &held.picked {
            clicked |= chip(ui, name, ChipKind::Selection).clicked();
        }
        let invited = lang.t("copy_panel.click_in_the_view");
        clicked |= chip(ui, &invited, ChipKind::Invitation).clicked();
    });
    if clicked {
        asked.activate = Some(CopySlot::Elements);
    }
}

/// The centre, the axis or the direction: what was picked with « Changer »
/// beside it, or the invitation to click it in the view. `titled` is the key
/// of the step's title.
fn target(ui: &mut egui::Ui, (lang, titled): (&Catalogue, &str), held: &Held, asked: &mut Asked) {
    let marked = held.marked == Some(CopySlot::Target);
    let mut clicked = title(ui, lang.t(titled), marked);
    ui.horizontal(|ui| match &held.target {
        Some(picked) => {
            clicked |= chip(ui, picked, ChipKind::Selection).clicked();
            clicked |= quiet_link(ui, &lang.t("copy_panel.change")).clicked();
        }
        None => {
            let invited = lang.t("copy_panel.click_in_the_view");
            clicked |= chip(ui, &invited, ChipKind::Invitation).clicked();
        }
    });
    if clicked {
        asked.activate = Some(CopySlot::Target);
    }
}

fn rotation(ui: &mut egui::Ui, lang: &Catalogue, held: &Held, asked: &mut Asked) {
    if title(ui, lang.t("copy_panel.rotation"), false) {
        asked.activate = Some(CopySlot::Rotation);
    }
    ui.horizontal(|ui| {
        for (way, key) in [
            (Rotation::CounterClockwise, "copy_panel.counter_clockwise"),
            (Rotation::Clockwise, "copy_panel.clockwise"),
        ] {
            if ui
                .selectable_label(held.rotation == way, lang.t(key))
                .clicked()
            {
                asked.rotation = Some(way);
                asked.activate = Some(CopySlot::Rotation);
            }
        }
    });
}

/// « Annuler » and « Valider ↵ », at the right of the panel's foot.
fn footer(ui: &mut egui::Ui, lang: &Catalogue, asked: &mut Asked) {
    foot(ui, |ui| {
        if primary_button(ui, &lang.t("copy_panel.validate_enter")).clicked() {
            asked.activate = Some(CopySlot::Validate);
            asked.enter = true;
        }
        asked.cancel = secondary_button(ui, true, &lang.t("copy_panel.cancel")).clicked();
    });
}
