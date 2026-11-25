//! What app · ui/primary_button.rs is held to: amber at rest, a lighter amber
//! under the pointer, and a button that cannot be pressed not answering the
//! pointer at all.

use super::*;
use crate::ui::theme::tokens::ACCENT_AMBER_LIGHT;

/// How far two colours may drift per channel and still read as one: what
/// multiplying through a premultiplied alpha costs in rounding.
const SAME_COLOUR: u8 = 2;

/// How far a frame may stand out of the button it is painted for: the
/// expansion a widget's look may give it.
const FRAME_REACH: f32 = 2.0;

fn alike(one: egui::Color32, other: egui::Color32) -> bool {
    one.to_array()
        .iter()
        .zip(other.to_array())
        .all(|(a, b)| a.abs_diff(b) <= SAME_COLOUR)
}

#[test]
fn a_primary_button_at_rest_is_filled_amber() {
    let fill = fill_of(Pressable::Yes, Pointer::Away);

    assert!(
        alike(fill, ACCENT_AMBER),
        "at rest the button is filled {fill:?}, where it is {ACCENT_AMBER:?}",
    );
}

#[test]
fn under_the_pointer_a_primary_button_is_filled_a_lighter_amber() {
    let fill = fill_of(Pressable::Yes, Pointer::OnIt);

    assert!(
        alike(fill, ACCENT_AMBER_LIGHT),
        "under the pointer the button is filled {fill:?}, where it is \
         {ACCENT_AMBER_LIGHT:?}: nothing shows the pointer is on it",
    );
}

#[test]
fn a_primary_button_that_cannot_be_pressed_does_not_answer_the_pointer() {
    let at_rest = fill_of(Pressable::No, Pointer::Away);
    let under_the_pointer = fill_of(Pressable::No, Pointer::OnIt);

    assert!(
        alike(under_the_pointer, at_rest),
        "a button that cannot be pressed turns {under_the_pointer:?} under the pointer, \
         from {at_rest:?} at rest",
    );
}

#[derive(Clone, Copy)]
enum Pressable {
    Yes,
    No,
}

#[derive(Clone, Copy)]
enum Pointer {
    Away,
    OnIt,
}

/// What the button is filled with on the last of three frames of a themed
/// context, with no window: the first lays it out, the second brings the
/// pointer onto it or not, the third is drawn with the pointer resting there.
fn fill_of(pressable: Pressable, pointer: Pointer) -> egui::Color32 {
    let ctx = egui::Context::default();
    crate::ui::install(&ctx);
    let mut rect = egui::Rect::NOTHING;
    let mut shapes = Vec::new();

    for frame in 0..3 {
        let events = match (frame, pointer) {
            (1, Pointer::OnIt) => vec![egui::Event::PointerMoved(rect.center())],
            _ => Vec::new(),
        };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(400.0, 200.0),
            )),
            events,
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            let enabled = matches!(pressable, Pressable::Yes);
            rect = ui
                .add_enabled_ui(enabled, |ui| primary_button(ui, "Créer ↵"))
                .inner
                .rect;
        });
        output.textures_delta.clear();
        shapes = every_shape(output.shapes);
    }

    let frames: Vec<&egui::epaint::RectShape> = shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(frame) => Some(frame),
            _ => None,
        })
        .filter(|frame| {
            frame.rect.contains(rect.center()) && rect.expand(FRAME_REACH).contains_rect(frame.rect)
        })
        .collect();
    match frames[..] {
        [frame] => frame.fill,
        _ => panic!(
            "the button at {rect:?} is not painted one frame: {:?}",
            frames.iter().map(|frame| frame.rect).collect::<Vec<_>>(),
        ),
    }
}

fn every_shape(clipped: Vec<egui::epaint::ClippedShape>) -> Vec<egui::Shape> {
    let mut pending: Vec<egui::Shape> = clipped.into_iter().map(|clipped| clipped.shape).collect();
    let mut shapes = Vec::new();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            shape => shapes.push(shape),
        }
    }
    shapes
}
