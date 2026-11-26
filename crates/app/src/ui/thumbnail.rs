//! A picture drawn small beside what it shows, or the room it will take once
//! there is one.

use egui::{Response, Ui, Vec2};

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

/// The pixels as something the GPU can draw, kept in egui's own store between
/// frames: uploading a texture every frame for every row on screen is the one
/// way to make a picture cost more than the drawing it stands for.
fn texture(ui: &Ui, key: &str, pixels: Pixels<'_>) -> egui::TextureHandle {
    let id = egui::Id::new((key, pixels.rgba.as_ptr() as usize));
    if let Some(held) = ui.data(|data| data.get_temp::<egui::TextureHandle>(id)) {
        return held;
    }
    let image = egui::ColorImage::from_rgba_unmultiplied(
        [pixels.width as usize, pixels.height as usize],
        pixels.rgba,
    );
    let made = ui
        .ctx()
        .load_texture(key, image, egui::TextureOptions::LINEAR);
    ui.data_mut(|data| data.insert_temp(id, made.clone()));
    made
}
