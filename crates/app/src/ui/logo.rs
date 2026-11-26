use egui::{
    Align, Label, Layout, Response, RichText, TextStyle, TextWrapMode, Ui, WidgetText, vec2,
};

use crate::ui::theme::tokens::{
    ACCENT_AMBER, TYPOGRAPHY_SIZES_BODY, TYPOGRAPHY_SIZES_SCREEN_TITLE,
};

/// How far apart the logo's letters stand, in ems.
const LETTER_SPACING: f32 = 0.14;

/// How far apart the letters of a screen's title stand, in ems: A4 of
/// `docs/redesign/home-screen.md`.
const TITLE_LETTER_SPACING: f32 = 0.12;

/// The text logo at the head of the tab row: amber, its letters spaced out,
/// and not a text to select. It stands as tall as a button holding it, its
/// text in the middle, so that it reads on the line of the tabs beside it: a
/// row lays a shorter widget in the height it had when that widget came,
/// before the tabs made it taller.
pub fn logo(ui: &mut Ui, text: &str) -> Response {
    let text = amber(text, TYPOGRAPHY_SIZES_BODY, LETTER_SPACING);
    let galley = WidgetText::from(text).into_galley(
        ui,
        Some(TextWrapMode::Extend),
        f32::INFINITY,
        TextStyle::Body,
    );
    let tall = galley.size().y + 2.0 * ui.spacing().button_padding.y;
    let centred = Layout::left_to_right(Align::Center);
    ui.allocate_ui_with_layout(vec2(0.0, tall), centred, |ui| {
        ui.add(Label::new(galley).selectable(false))
    })
    .inner
}

/// The name of the application at the head of a screen: the logo's amber,
/// drawn as large as the redesign's screen title.
pub fn screen_title(ui: &mut Ui, text: &str) -> Response {
    let text = amber(text, TYPOGRAPHY_SIZES_SCREEN_TITLE, TITLE_LETTER_SPACING);
    ui.add(Label::new(text).selectable(false))
}

/// `text` in the logo's amber at `size`, its letters `spacing` ems apart.
fn amber(text: &str, size: f32, spacing: f32) -> RichText {
    RichText::new(text)
        .strong()
        .size(size)
        .color(ACCENT_AMBER)
        .extra_letter_spacing(spacing * size)
}
