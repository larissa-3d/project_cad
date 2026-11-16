//! The section view's four numbered steps — the face it cuts along, the
//! distance, the two tilts, the two sides with « Inverser » — and its main
//! button, « Enregistrer la coupe ».

use cao_sketch::WorkPlane;

use super::super::live_fields::{left_on_enter, value_field};
use super::super::step_panel::{foot, step, title};
use crate::screens::SketchContext;
use crate::screens::viewport::section::{Section, SectionField, SectionStep};
use crate::ui::floating_panel::panel_row;
use crate::ui::primary_button::primary_button;
use crate::ui::theme::tokens::SPACING_XXL;
use crate::wording::section::{self as said, Side};

/// What a gesture in the panel asked for this frame, beyond what it changed
/// in the section itself.
#[derive(Default)]
pub(super) struct Asked {
    pub(super) enter: bool,
    pub(super) keep: bool,
}

/// Draws the steps and the main button. A step's dot or title clicked makes
/// it current, a field reached makes it the one `Entrée` stands on, and the
/// field `Entrée` stands on takes the keyboard the frame it is `arrived` at.
pub(super) fn steps(
    ui: &mut egui::Ui,
    section: &mut Section,
    context: &SketchContext<'_>,
    arrived: bool,
) -> Asked {
    let mut asked = Asked::default();
    ui.spacing_mut().item_spacing.y = SPACING_XXL;
    for (rank, (shown, state)) in section.steps().into_iter().enumerate() {
        let dot = step(ui, (rank + 1, state), |ui| match shown {
            SectionStep::Face => face(ui, section, context),
            SectionStep::Distance => distance(ui, section, context, (arrived, &mut asked)),
            SectionStep::Tilts => tilts(ui, section, context, (arrived, &mut asked)),
            SectionStep::Sides => sides(ui, section, context),
        });
        if dot {
            section.activate(shown);
        }
    }
    foot(ui, |ui| {
        asked.keep = primary_button(ui, &context.lang.t("section.save")).clicked();
    });
    asked
}

/// The three origin planes and « Face cliquée », the one cut along lit;
/// « Face cliquée » is warned while the face is still to be clicked.
fn face(ui: &mut egui::Ui, section: &mut Section, context: &SketchContext<'_>) {
    let (lang, body) = (context.lang, context.document.body());
    if title(ui, lang.t("section.face"), false) {
        section.activate(SectionStep::Face);
    }
    let waiting = section.is_waiting_for_a_face();
    ui.horizontal_wrapped(|ui| {
        let mut on_an_origin_plane = false;
        for plane in WorkPlane::ORIGIN_PLANES {
            let chosen = !waiting && section.plane == plane;
            on_an_origin_plane |= chosen;
            let named = crate::wording::plane::label(lang, plane.kind());
            if panel_row(ui, named, chosen, false).clicked() {
                section.choose_origin_plane(plane, body);
            }
        }
        let clicked_face = lang.t("section.clicked_face");
        if panel_row(ui, clicked_face, !on_an_origin_plane, waiting).clicked() {
            section.wait_for_a_face();
        }
    });
}

/// « Distance » with its field beside it, in millimetres.
fn distance(
    ui: &mut egui::Ui,
    section: &mut Section,
    context: &SketchContext<'_>,
    (arrived, asked): (bool, &mut Asked),
) {
    let millimetres = context.document.scale();
    let focus = arrived && section.field() == SectionField::Distance;
    let now = section.distance_text(millimetres);
    let typed = ui
        .horizontal(|ui| {
            if title(ui, context.lang.t("section.distance"), false) {
                section.activate(SectionStep::Distance);
            }
            typed_field(ui, ("section_distance", "mm"), (now, focus), context)
        })
        .inner;
    if typed.reached {
        section.reach(SectionField::Distance);
    }
    asked.enter |= typed.entered;
    if let Some(typed) = typed.value {
        section.type_distance(typed, millimetres);
    }
}

/// « Inclinaisons »: the two angles, each with its field beside it, in
/// degrees.
fn tilts(
    ui: &mut egui::Ui,
    section: &mut Section,
    context: &SketchContext<'_>,
    (arrived, asked): (bool, &mut Asked),
) {
    let lang = context.lang;
    if title(ui, lang.t("section.tilts"), false) {
        section.activate(SectionStep::Tilts);
    }
    for (at, id, key) in [
        (SectionField::TiltAboutU, "section_tilt_u", "section.tilt_u"),
        (SectionField::TiltAboutV, "section_tilt_v", "section.tilt_v"),
    ] {
        let now = crate::wording::dimension::short(*tilt(section, at));
        let focus = arrived && section.field() == at;
        let typed = ui
            .horizontal(|ui| {
                ui.label(lang.t(key));
                typed_field(ui, (id, "°"), (now, focus), context)
            })
            .inner;
        if let Some(degrees) = typed.value {
            *tilt(section, at) = degrees;
        }
        if typed.reached {
            section.reach(at);
        }
        asked.enter |= typed.entered;
    }
}

/// The angle a tilt's field holds.
fn tilt(section: &mut Section, at: SectionField) -> &mut f64 {
    match at {
        SectionField::TiltAboutU => &mut section.tilt_about_u_degrees,
        _ => &mut section.tilt_about_v_degrees,
    }
}

/// How the matter behind and in front is drawn, each a button stepping
/// through hidden, see-through and opaque; « Inverser » swaps the two.
fn sides(ui: &mut egui::Ui, section: &mut Section, context: &SketchContext<'_>) {
    let lang = context.lang;
    let mut clicked = false;
    ui.horizontal_wrapped(|ui| {
        if ui
            .button(said::side(lang, Side::Behind, section.behind))
            .clicked()
        {
            section.next_behind();
            clicked = true;
        }
        if ui
            .button(said::side(lang, Side::InFront, section.in_front))
            .clicked()
        {
            section.next_in_front();
            clicked = true;
        }
    });
    if ui.button(lang.t("section.invert")).clicked() {
        section.invert();
        clicked = true;
    }
    if clicked {
        section.activate(SectionStep::Sides);
    }
}

/// What a field of the panel answered this frame: the number typed in it,
/// whether it was reached or typed into, and whether the keyboard left it on
/// `Entrée`.
struct Typed {
    value: Option<f64>,
    reached: bool,
    entered: bool,
}

/// A field taking a number typed or read from the formula field: while it is
/// not being typed in, it reads `now`, which follows the handle for the
/// distance. `focus` hands it the keyboard.
fn typed_field(
    ui: &mut egui::Ui,
    (id, unit): (&str, &str),
    (now, focus): (String, bool),
    context: &SketchContext<'_>,
) -> Typed {
    let variables = context.document.variables();
    let id = egui::Id::new(id);
    let typing = ui.memory(|memory| memory.has_focus(id));
    let mut text = match typing {
        true => ui.data(|data| data.get_temp::<String>(id.with("text"))),
        false => None,
    }
    .unwrap_or(now);
    let field = value_field(ui, id, &mut text, "", focus, variables)
        .response
        .response;
    ui.label(unit);
    let value = match field.changed() {
        true => variables.size_of(&text).ok().map(|(_, typed)| typed),
        false => None,
    };
    ui.data_mut(|data| data.insert_temp(id.with("text"), text));
    Typed {
        value,
        reached: field.gained_focus() || field.changed(),
        entered: left_on_enter(ui, &field),
    }
}
