//! A field that shows a value rather than taking one typed: framed as a field
//! is, an icon before the value, as `docs/redesign/source/home.html` draws
//! where a new part is saved.

use egui::{
    Align2, FontId, Response, Sense, Stroke, StrokeKind, Ui, Vec2, WidgetInfo, WidgetType, vec2,
};

use crate::ui::icon::{Icon, IconSize};
use crate::ui::theme::tokens::{
    BACKGROUND_FIELD, BORDER_FIELD, RADIUS_CONTROL, TEXT_SUBDUED, TEXT_TITLE, TYPOGRAPHY_SIZES_BODY,
};

/// How tall a card's field stands, which the tokens do not carry.
pub const HEIGHT: f32 = 40.0;

/// What the field keeps clear on either side, and between its icon and its
/// value.
const PADDING: f32 = 12.0;
const ICON_GAP: f32 = 8.0;

/// `shown` after `icon`, in a field `width` wide, cut short where the field
/// ends. Whoever cannot see it is told what it shows.
pub fn shown_field(ui: &mut Ui, icon: Icon, shown: &str, width: f32) -> Response {
    let font = FontId::proportional(TYPOGRAPHY_SIZES_BODY);
    let written = ui
        .painter()
        .layout_no_wrap(shown.to_owned(), font, TEXT_TITLE);
    let (rect, response) = ui.allocate_exact_size(vec2(width, HEIGHT), Sense::hover());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), shown));

    let border = Stroke::new(1.0, BORDER_FIELD);
    let painter = ui.painter();
    painter.rect(
        rect,
        RADIUS_CONTROL,
        BACKGROUND_FIELD,
        border,
        StrokeKind::Inside,
    );
    let inside = rect.shrink2(vec2(PADDING, 0.0));
    let side = Vec2::splat(IconSize::Small.points());
    let icon_at = Align2::LEFT_CENTER.align_size_within_rect(side, inside);
    icon.image(IconSize::Small)
        .tint(TEXT_SUBDUED)
        .paint_at(ui, icon_at);
    let room = inside.with_min_x(icon_at.right() + ICON_GAP);
    let at = Align2::LEFT_CENTER.align_size_within_rect(written.size(), room);
    ui.painter()
        .with_clip_rect(room.intersect(ui.clip_rect()))
        .galley(at.min, written, TEXT_TITLE);
    response
}

#[cfg(test)]
mod tests;
