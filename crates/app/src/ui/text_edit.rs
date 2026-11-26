/// A single-line text field with its width and its placeholder baked in.
pub fn text_edit(ui: &mut egui::Ui, value: &mut String, width: f32, hint: &str) -> egui::Response {
    ui.add(
        egui::TextEdit::singleline(value)
            .desired_width(width)
            .hint_text(hint),
    )
}

/// A single-line text field `height` tall and as wide as the room left, its
/// text centred on its height: the one entry of a card.
pub fn tall_text_edit(ui: &mut egui::Ui, value: &mut String, height: f32) -> egui::Response {
    ui.add(
        egui::TextEdit::singleline(value)
            .desired_width(f32::INFINITY)
            .min_size(egui::vec2(0.0, height))
            .vertical_align(egui::Align::Center),
    )
}

#[cfg(test)]
mod tests;
