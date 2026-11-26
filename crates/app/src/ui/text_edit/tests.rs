//! What app · ui/text_edit.rs is held to: the tall field of a card stands as
//! tall as asked, as wide as the room left, its text on its middle.

use super::*;

/// How far a size may stray from the one asked for: shapes are placed on
/// rounded points.
const SIZED_WITHIN: f32 = 0.5;

/// How far the middle of the text may stand from the middle of the field: a
/// galley is laid on whole points, and its line is a little taller than its
/// letters.
const CENTRED_WITHIN: f32 = 1.5;

#[test]
fn a_tall_text_edit_stands_as_tall_as_asked_and_as_wide_as_the_room_left() {
    let drawn = drawn("Support", 40.0);

    assert!(
        (drawn.rect.height() - 40.0).abs() < SIZED_WITHIN,
        "the field stands {} tall, where it is asked 40",
        drawn.rect.height(),
    );
    assert!(
        (drawn.rect.width() - drawn.room).abs() < SIZED_WITHIN,
        "the field stands {} wide, where {} is left",
        drawn.rect.width(),
        drawn.room,
    );
}

#[test]
fn a_tall_text_edit_writes_its_text_on_its_middle() {
    let drawn = drawn("Support", 40.0);

    let text = drawn
        .texts
        .iter()
        .find(|(text, _)| text == "Support")
        .map(|(_, at)| *at)
        .unwrap_or_else(|| panic!("the field writes {:?}", drawn.texts));
    assert!(
        (text.center().y - drawn.rect.center().y).abs() < CENTRED_WITHIN,
        "the text stands on {}, the field's middle on {}",
        text.center().y,
        drawn.rect.center().y,
    );
}

/// A tall text edit as one frame of a themed context paints it, with no
/// window.
struct Drawn {
    rect: egui::Rect,
    /// How wide the room was before the field took it.
    room: f32,
    texts: Vec<(String, egui::Rect)>,
}

fn drawn(holding: &str, height: f32) -> Drawn {
    let ctx = egui::Context::default();
    crate::ui::install(&ctx);
    let mut value = holding.to_string();
    let mut rect = egui::Rect::NOTHING;
    let mut room = 0.0;
    let mut texts = Vec::new();

    for _ in 0..2 {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(400.0, 200.0),
            )),
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            room = ui.available_width();
            rect = tall_text_edit(ui, &mut value, height).rect;
        });
        output.textures_delta.clear();
        texts = written(output.shapes);
    }

    Drawn { rect, room, texts }
}

fn written(clipped: Vec<egui::epaint::ClippedShape>) -> Vec<(String, egui::Rect)> {
    let mut pending: Vec<egui::Shape> = clipped.into_iter().map(|clipped| clipped.shape).collect();
    let mut texts = Vec::new();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            egui::Shape::Text(text) => {
                let at = text.visual_bounding_rect();
                texts.push((text.galley.text().to_owned(), at));
            }
            _ => {}
        }
    }
    texts
}
