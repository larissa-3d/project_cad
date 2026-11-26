//! What app · ui/shown_field.rs is held to: a field that shows a value, as
//! tall as a card's field, cut where it ends.

use super::*;
use crate::ui::icon::FOLDER;

/// How far a size may stray from the one asked for: shapes are placed on
/// rounded points.
const SIZED_WITHIN: f32 = 0.5;

/// How wide the screen the field is drawn on stands: wider than any field
/// these tests ask for, so a text left uncut would run past the field.
const SCREEN_WIDTH: f32 = 600.0;

#[test]
fn a_shown_field_stands_40_points_tall_as_wide_as_asked_and_writes_what_it_shows() {
    let drawn = drawn("Dossier des pièces / Brides", 300.0);

    assert!(
        (drawn.rect.height() - 40.0).abs() < SIZED_WITHIN
            && (drawn.rect.width() - 300.0).abs() < SIZED_WITHIN,
        "the field stands {:?}, where it is asked 300 wide and 40 tall",
        drawn.rect.size(),
    );
    assert!(
        drawn.cut_at("Dossier des pièces / Brides").is_some(),
        "the field writes {:?}",
        drawn.texts,
    );
}

#[test]
fn a_value_too_long_for_the_field_is_cut_where_the_field_ends() {
    let long = "Dossier des pièces / Brides / Petites / Anciennes / À reprendre";

    let drawn = drawn(long, 120.0);

    let cut = drawn
        .cut_at(long)
        .unwrap_or_else(|| panic!("the field writes {:?}", drawn.texts));
    assert!(
        cut.right() <= drawn.rect.right(),
        "the value is cut at {}, past the field's end at {}",
        cut.right(),
        drawn.rect.right(),
    );
}

#[test]
fn the_value_is_written_after_its_icon_clear_of_it_and_whole_when_it_fits() {
    let drawn = drawn("Dossier des pièces", 300.0);

    let icon = *drawn
        .pictures
        .first()
        .unwrap_or_else(|| panic!("the field paints no icon, writing {:?}", drawn.texts));
    let (value, cut) = drawn
        .placed("Dossier des pièces")
        .unwrap_or_else(|| panic!("the field writes {:?}", drawn.texts));
    assert!(
        value.left() >= icon.right(),
        "the value starts at {}, over the icon ending at {}",
        value.left(),
        icon.right(),
    );
    assert!(
        cut.contains_rect(value),
        "the value at {value:?}, which fits a field 300 wide, is cut to {cut:?}",
    );
}

/// A shown field as one frame of a themed context paints it, with no window.
struct Drawn {
    rect: egui::Rect,
    /// Every text painted: where it stands, and the rectangle it is cut to.
    texts: Vec<(String, egui::Rect, egui::Rect)>,
    /// Where every picture is painted, the icon's among them: a shape filled
    /// with a texture, as `egui::Image` paints one.
    pictures: Vec<egui::Rect>,
}

impl Drawn {
    fn cut_at(&self, said: &str) -> Option<egui::Rect> {
        self.placed(said).map(|(_, cut)| cut)
    }

    /// Where the text reading exactly `said` stands, and what it is cut to.
    fn placed(&self, said: &str) -> Option<(egui::Rect, egui::Rect)> {
        self.texts
            .iter()
            .find(|(text, _, _)| text == said)
            .map(|(_, at, cut)| (*at, *cut))
    }
}

fn drawn(shown: &str, width: f32) -> Drawn {
    let ctx = egui::Context::default();
    crate::ui::install(&ctx);
    let mut rect = egui::Rect::NOTHING;
    let mut painted = (Vec::new(), Vec::new());

    for _ in 0..2 {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, 200.0),
            )),
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            rect = shown_field(ui, FOLDER, shown, width).rect;
        });
        output.textures_delta.clear();
        painted = painted_in(output.shapes);
    }

    let (texts, pictures) = painted;
    Drawn {
        rect,
        texts,
        pictures,
    }
}

type Texts = Vec<(String, egui::Rect, egui::Rect)>;

/// Every text painted, where it stands and what it is cut to, and where every
/// picture is.
fn painted_in(clipped: Vec<egui::epaint::ClippedShape>) -> (Texts, Vec<egui::Rect>) {
    let mut pending: Vec<(egui::Shape, egui::Rect)> = clipped
        .into_iter()
        .map(|clipped| (clipped.shape, clipped.clip_rect))
        .collect();
    let mut texts = Vec::new();
    let mut pictures = Vec::new();
    while let Some((shape, cut)) = pending.pop() {
        let at = shape.visual_bounding_rect();
        if shape.texture_id() != egui::TextureId::default() {
            pictures.push(at);
        }
        match shape {
            egui::Shape::Vec(group) => pending.extend(group.into_iter().map(|shape| (shape, cut))),
            egui::Shape::Text(text) => texts.push((text.galley.text().to_owned(), at, cut)),
            _ => {}
        }
    }
    (texts, pictures)
}
