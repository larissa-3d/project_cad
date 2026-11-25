//! A segmented selector, C6 of `docs/redesign/components.md`: choices side by
//! side in one frame, the one chosen standing out, and one that cannot be
//! chosen yet greyed with a word after its name.

use egui::{
    FontId, Frame, Margin, Rect, Response, Sense, Stroke, Ui, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::ui::theme::tokens::{
    BACKGROUND_ACTIVE_NEUTRAL, BACKGROUND_FIELD, BACKGROUND_HOVER_STRONG, BORDER_GROUP,
    RADIUS_BUTTON, RADIUS_CONTROL, RADIUS_PILL, SPACING_M, TEXT_DISABLED, TEXT_QUIET, TEXT_SUBDUED,
    TEXT_TITLE, TYPOGRAPHY_SIZES_BUTTON,
};

/// How tall a segment stands: C6's 24 px, taken to 28 on a card as wide as the
/// home screen's.
const HEIGHT: f32 = 28.0;

/// What a segment keeps clear on either side, as
/// `docs/redesign/source/home.html` draws it.
const PADDING: f32 = 12.0;

/// The size of the word after a greyed segment's name, which the tokens do
/// not carry.
const TAG_SIZE: f32 = 10.0;

/// What the word after a greyed segment's name keeps clear around it.
const TAG_MARGIN: egui::Vec2 = vec2(SPACING_M, 2.0);

/// One choice of the selector.
pub struct Segment<'a> {
    pub label: &'a str,
    pub enabled: bool,
    /// A word drawn after the name, as « bientôt » says a choice is to come.
    pub tag: Option<&'a str>,
}

/// The segments side by side, `chosen` standing out. Hands back the one
/// clicked, which is never a greyed one.
pub fn segmented(ui: &mut Ui, segments: &[Segment<'_>], chosen: usize) -> Option<usize> {
    let mut clicked = None;
    Frame::new()
        .fill(BACKGROUND_FIELD)
        .stroke(Stroke::new(1.0, BORDER_GROUP))
        .corner_radius(RADIUS_CONTROL)
        .inner_margin(Margin::same(2))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                for (at, segment) in segments.iter().enumerate() {
                    let one = ui.add_enabled_ui(segment.enabled, |ui| {
                        one_segment(ui, segment, at == chosen)
                    });
                    if one.inner.clicked() {
                        clicked = Some(at);
                    }
                }
            });
        });
    clicked
}

fn one_segment(ui: &mut Ui, segment: &Segment<'_>, chosen: bool) -> Response {
    let ink = match (segment.enabled, chosen) {
        (false, _) => TEXT_DISABLED,
        (true, true) => TEXT_TITLE,
        (true, false) => TEXT_SUBDUED,
    };
    let font = FontId::proportional(TYPOGRAPHY_SIZES_BUTTON);
    let name = ui
        .painter()
        .layout_no_wrap(segment.label.to_owned(), font, ink);
    let tag = segment.tag.map(|tag| {
        let font = FontId::proportional(TAG_SIZE);
        ui.painter()
            .layout_no_wrap(tag.to_owned(), font, TEXT_QUIET)
    });
    let tag_room = tag
        .as_ref()
        .map_or(0.0, |tag| SPACING_M + tag.size().x + 2.0 * TAG_MARGIN.x);
    let size = vec2(name.size().x + tag_room + 2.0 * PADDING, HEIGHT);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    response.widget_info(|| {
        WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), chosen, segment.label)
    });

    let painter = ui.painter();
    if chosen {
        painter.rect_filled(rect, RADIUS_BUTTON, BACKGROUND_ACTIVE_NEUTRAL);
    }
    let name_end = rect.left() + PADDING + name.size().x;
    let name_at = pos2(rect.left() + PADDING, rect.center().y - name.size().y / 2.0);
    painter.galley(name_at, name, ink);
    if let Some(tag) = tag {
        let patch_size = tag.size() + 2.0 * TAG_MARGIN;
        let patch_at = pos2(name_end + SPACING_M, rect.center().y - patch_size.y / 2.0);
        let patch = Rect::from_min_size(patch_at, patch_size);
        painter.rect_filled(patch, RADIUS_PILL, BACKGROUND_HOVER_STRONG);
        painter.galley(patch.min + TAG_MARGIN, tag, TEXT_QUIET);
    }
    response
}
