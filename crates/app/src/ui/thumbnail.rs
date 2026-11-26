//! A picture drawn small beside what it shows, or the room it will take once
//! there is one.

use egui::{Response, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme::tokens::{BACKGROUND_FIELD, BORDER_GROUP, RADIUS_BUTTON, SPACING_S};

/// A picture as plain values: how big it is, and its pixels, four bytes each,
/// unmultiplied.
#[derive(Clone, Copy)]
pub struct Pixels<'a> {
    pub width: u32,
    pub height: u32,
    pub rgba: &'a [u8],
}

/// The picture at `size`, or a quiet frame of that size when there is none —
/// rather than nothing, so a list half of whose entries have a picture does
/// not read as two columns.
///
/// `key` names where the picture is drawn, so that two places showing the same
/// pixels each hold their own texture.
pub fn thumbnail(ui: &mut Ui, key: &str, size: Vec2, pixels: Option<Pixels<'_>>) -> Response {
    let Some(pixels) = pixels else {
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
        ui.painter()
            .rect_filled(rect, 2.0, ui.visuals().faint_bg_color);
        return response;
    };
    ui.add(egui::Image::new(&texture(ui, key, pixels)).fit_to_exact_size(size))
}

/// The picture set in a bordered box of `size`, the box standing there with
/// nothing in it while there is no picture.
pub fn framed_thumbnail(
    ui: &mut Ui,
    key: &str,
    size: Vec2,
    pixels: Option<Pixels<'_>>,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter().rect(
        rect,
        RADIUS_BUTTON,
        BACKGROUND_FIELD,
        Stroke::new(1.0, BORDER_GROUP),
        StrokeKind::Inside,
    );
    if let Some(pixels) = pixels {
        let inside = rect.shrink(SPACING_S);
        let image = egui::Image::new(&texture(ui, key, pixels)).fit_to_exact_size(inside.size());
        ui.put(inside, image);
    }
    response
}

/// The textures drawn under one key, each with what its picture is and the
/// pass it was last drawn in.
#[derive(Clone, Default)]
struct Held(Vec<(u64, egui::TextureHandle, u64)>);

/// The pixels as something the GPU can draw, kept in egui's own store between
/// frames: uploading a texture every frame for every row on screen is the one
/// way to make a picture cost more than the drawing it stands for.
///
/// Found again by what the picture is rather than where its bytes stand: the
/// memory one picture let go of can come back holding another. A texture no
/// picture was drawn with since the last pass is let go of.
fn texture(ui: &Ui, key: &str, pixels: Pixels<'_>) -> egui::TextureHandle {
    let id = egui::Id::new(key);
    let what = egui::util::hash((pixels.width, pixels.height, pixels.rgba));
    let pass = ui.ctx().cumulative_pass_nr();
    let found = ui.data_mut(|data| {
        let held = data.get_temp_mut_or_default::<Held>(id);
        held.0.retain(|(_, _, drawn)| drawn + 1 >= pass);
        let (_, handle, drawn) = held.0.iter_mut().find(|(seen, ..)| *seen == what)?;
        *drawn = pass;
        Some(handle.clone())
    });
    if let Some(handle) = found {
        return handle;
    }
    let image = egui::ColorImage::from_rgba_unmultiplied(
        [pixels.width as usize, pixels.height as usize],
        pixels.rgba,
    );
    let made = ui
        .ctx()
        .load_texture(key, image, egui::TextureOptions::LINEAR);
    ui.data_mut(|data| {
        data.get_temp_mut_or_default::<Held>(id)
            .0
            .push((what, made.clone(), pass));
    });
    made
}

#[cfg(test)]
mod tests;
