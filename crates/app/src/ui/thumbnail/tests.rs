//! What app · ui/thumbnail.rs is held to: the picture it draws is the one it
//! was handed on that frame, whatever it was handed before in the same place.

use std::collections::HashMap;

use super::*;

const SIZE: Vec2 = egui::vec2(64.0, 48.0);

fn filled(colour: egui::Color32, width: u32, height: u32) -> Vec<u8> {
    (0..width * height)
        .flat_map(|_| colour.to_array())
        .collect()
}

/// What the GPU holds of a texture: how wide it is, and its pixels.
type Uploaded = (usize, Vec<egui::Color32>);

/// A context with no window that keeps, for each texture, the pixels the GPU
/// was last handed for it.
struct Screen {
    ctx: egui::Context,
    uploaded: HashMap<egui::TextureId, Uploaded>,
}

impl Screen {
    fn new() -> Self {
        let ctx = egui::Context::default();
        crate::ui::install(&ctx);
        Self {
            ctx,
            uploaded: HashMap::new(),
        }
    }

    /// Draws one frame showing these pixels, and answers what the texture it
    /// painted holds, as the GPU was last handed it.
    fn show(&mut self, pixels: Pixels<'_>) -> Vec<egui::Color32> {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(400.0, 200.0),
            )),
            ..Default::default()
        };
        let mut output = self.ctx.run_ui(input, |ui| {
            thumbnail(ui, "a_list", SIZE, Some(pixels));
        });
        for (id, deltas) in &output.textures_delta.set {
            for delta in deltas {
                let egui::ImageData::Color(image) = &delta.image;
                match delta.pos {
                    None => {
                        self.uploaded
                            .insert(*id, (image.size[0], image.pixels.clone()));
                    }
                    Some([x, y]) => {
                        let (width, held) = self.uploaded.entry(*id).or_default();
                        for (row, line) in image.pixels.chunks(image.size[0]).enumerate() {
                            let start = (y + row) * *width + x;
                            held[start..start + line.len()].copy_from_slice(line);
                        }
                    }
                }
            }
        }
        output.textures_delta.clear();
        let painted: Vec<egui::TextureId> = every_shape(std::mem::take(&mut output.shapes))
            .into_iter()
            .filter_map(|shape| match shape {
                egui::Shape::Rect(rect) if rect.fill_texture_id() != egui::TextureId::default() => {
                    Some(rect.fill_texture_id())
                }
                _ => None,
            })
            .collect();
        match painted[..] {
            [texture] => self
                .uploaded
                .get(&texture)
                .map(|(_, pixels)| pixels.clone())
                .unwrap_or_default(),
            _ => panic!("the thumbnail painted {} textures, not one", painted.len()),
        }
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

#[test]
fn a_thumbnail_shows_the_pixels_it_is_handed_where_others_were_shown_before() {
    let mut screen = Screen::new();
    let (red, blue) = (egui::Color32::RED, egui::Color32::BLUE);
    let mut held = filled(red, 4, 3);
    let first = screen.show(Pixels {
        width: 4,
        height: 3,
        rgba: &held,
    });
    assert!(
        !first.is_empty() && first.iter().all(|pixel| *pixel == red),
        "the first picture is not drawn as it was handed: {first:?}",
    );

    held.copy_from_slice(&filled(blue, 4, 3));
    let second = screen.show(Pixels {
        width: 4,
        height: 3,
        rgba: &held,
    });

    assert!(
        second.iter().all(|pixel| *pixel == blue),
        "a picture handed where another was, from memory the first one let go of, \
         is drawn as the first: {second:?}",
    );
}
