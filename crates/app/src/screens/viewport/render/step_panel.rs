//! What the panels laid out in numbered steps share, the copy tools' and the
//! section view's: a step's row under its dot, a step's title, and the row of
//! buttons at the panel's foot.

use crate::screens::sketch::StepState;
use crate::ui::step_dot::{Progress, step_dot};
use crate::ui::theme::tokens::SPACING_M;

/// How tall C15 stands a panel's buttons.
const BUTTON_HEIGHT: f32 = 32.0;

/// One numbered step: its dot as `state` reads, and beside it what `contents`
/// draws. True when the dot was clicked.
pub(super) fn step(
    ui: &mut egui::Ui,
    (number, state): (usize, StepState),
    contents: impl FnOnce(&mut egui::Ui),
) -> bool {
    ui.horizontal_top(|ui| {
        let clicked = step_dot(ui, number, progress(state)).clicked();
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(SPACING_M, SPACING_M);
            contents(ui);
        });
        clicked
    })
    .inner
}

fn progress(state: StepState) -> Progress {
    match state {
        StepState::Done => Progress::Done,
        StepState::Current => Progress::Current,
        StepState::ToCome => Progress::ToCome,
    }
}

/// A step's title, warned while `Entrée` found its step empty. Says whether
/// it was clicked.
pub(super) fn title(ui: &mut egui::Ui, said: String, marked: bool) -> bool {
    let warn = ui.visuals().warn_fg_color;
    let shown = match marked {
        true => ui.colored_label(warn, said),
        false => ui.label(said),
    };
    shown.interact(egui::Sense::click()).clicked()
}

/// The buttons at the right of the panel's foot, which `contents` draws right
/// to left.
///
/// The row is given its own height: a layout centred on what is left of the
/// panel would fill the height the panel had last frame, and a panel that
/// stood taller for another tool would never shrink back.
pub(super) fn foot(ui: &mut egui::Ui, contents: impl FnOnce(&mut egui::Ui)) {
    let right = egui::Layout::right_to_left(egui::Align::Center);
    let row = egui::vec2(ui.available_width(), BUTTON_HEIGHT);
    ui.allocate_ui_with_layout(row, right, |ui| {
        ui.spacing_mut().interact_size.y = BUTTON_HEIGHT;
        contents(ui);
    });
}
