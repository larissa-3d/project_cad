//! What an extrusion is about to turn into matter: the areas picked for it,
//! the one under the cursor, the axis a revolution would turn around, and the
//! amber arrow a drag pulls, its value beside it in a pill.

use cao_prefs::theme::Theme;
use cao_sketch::Sketch;
use glam::DVec2;

use super::overlays::to_screen;
use super::{tint, tint_at, token};
use crate::screens::SketchContext;
use crate::screens::viewport::extrusion_handle::ExtrusionHandle;
use crate::screens::viewport::{ViewScale, ViewportState};
use crate::ui::theme::tokens::{
    ACCENT_AMBER, ACCENT_AMBER_BACKGROUND, ACCENT_AMBER_BORDER, ACCENT_AMBER_TEXT,
    BACKGROUND_FLOATING, RADIUS_BUTTON, TYPOGRAPHY_SIZES_MONO_VALUE,
};

/// How long the barbs of the arrow's head are, on screen.
const BARB_PIXELS: f64 = 12.0;

/// Where the pill stands from the arrow's tip, and the room inside it.
const PILL_FROM_THE_TIP: egui::Vec2 = egui::vec2(12.0, 10.0);
const PILL_PADDING: egui::Vec2 = egui::vec2(8.0, 3.0);

/// The arrow on the extrusion previewed, from its foot to its tip, a head at
/// the tip the same size at any zoom.
pub(super) fn push_extrusion_arrow(
    lines: &mut Vec<cao_render::Vertex>,
    scale: ViewScale,
    context: &SketchContext<'_>,
) {
    let Some(arrow) = ExtrusionHandle::drawn(context.extrusion, context.document) else {
        return;
    };
    let run = arrow.run();
    let colour = token(ACCENT_AMBER);
    let length = scale.world_size_of(BARB_PIXELS);
    let way = match run.as_slice() {
        [.., before, tip] => (*tip - *before).normalize_or_zero(),
        _ => return,
    };
    let side = way.any_orthonormal_vector() * 0.5;
    let barbs = [side, -side].map(|side| (arrow.tip, arrow.tip + (side - way) * length));
    for (from, to) in run.windows(2).map(|pair| (pair[0], pair[1])).chain(barbs) {
        lines.push(cao_render::Vertex::line(from.as_vec3(), colour, 2.0));
        lines.push(cao_render::Vertex::line(to.as_vec3(), colour, 2.0));
    }
}

/// The pill beside the arrow's tip, reading what the value comes to.
pub(crate) fn paint_extrusion_pill(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
) {
    let Some(arrow) = ExtrusionHandle::drawn(context.extrusion, context.document) else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));
    let Some(at) = to_screen(arrow.tip, view_projection, rect) else {
        return;
    };
    let painter = ui.painter_at(rect);
    let font = egui::FontId::monospace(TYPOGRAPHY_SIZES_MONO_VALUE);
    let text = painter.layout_no_wrap(arrow.pill, font, ACCENT_AMBER_TEXT);
    let pill = egui::Rect::from_min_size(at + PILL_FROM_THE_TIP, text.size() + 2.0 * PILL_PADDING);
    painter.rect_filled(pill, RADIUS_BUTTON, BACKGROUND_FLOATING);
    painter.rect(
        pill,
        RADIUS_BUTTON,
        ACCENT_AMBER_BACKGROUND,
        egui::Stroke::new(1.0, ACCENT_AMBER_BORDER),
        egui::StrokeKind::Inside,
    );
    painter.galley(pill.min + PILL_PADDING, text, ACCENT_AMBER_TEXT);
}

/// Marks the areas picked for an extrusion, and the one under the cursor.
///
/// A chosen area is filled with the colour of the matter it is about to become,
/// which is the only preview needed before the height is typed.
pub(crate) fn push_chosen_areas(
    surfaces: &mut Vec<cao_render::Vertex>,
    lines: &mut Vec<cao_render::Vertex>,
    theme: &Theme,
    context: &SketchContext<'_>,
) {
    if !context.extrusion.is_active() {
        return;
    }
    let Some(sketch) = context
        .extrusion
        .sketch
        .and_then(|index| context.document.sketches().get(index))
    else {
        return;
    };

    let cutting = context.extrusion.mode == Some(cao_part::ExtrusionMode::Cut);
    let chosen = if cutting {
        tint(theme.extrusion_cut)
    } else {
        tint(theme.extrusion_add)
    };

    if context.extrusion.is_revolving() {
        push_revolution_axis(lines, sketch, theme, context);
    }

    for (index, region) in sketch.regions().iter().enumerate() {
        let picked = context
            .extrusion
            .picks
            .iter()
            .any(|pick| region.contains(*pick));
        let hovered = context.extrusion.hovered == Some(index);
        if !picked && !hovered {
            continue;
        }
        let color = if picked {
            chosen
        } else {
            tint_at(theme.highlight, theme.highlight.a * 0.5)
        };

        // Holes stay empty here too: what is shown filled is exactly what will become matter.
        for [a, b, c] in region.face_triangles() {
            for corner in [a, b, c] {
                surfaces.push(cao_render::Vertex::solid(
                    sketch.plane.to_world(corner).as_vec3(),
                    color,
                ));
            }
        }
    }
}

/// Draws the axis a revolution turns around, well past the drawing so it reads
/// as an axis rather than as one more line of the sketch.
fn push_revolution_axis(
    lines: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    context: &SketchContext<'_>,
) {
    let (origin, direction) = match context.extrusion.axis {
        cao_part::RevolutionAxis::Sketch(axis) => (DVec2::ZERO, axis.direction()),
        cao_part::RevolutionAxis::Segment(segment) => {
            if segment.0 >= sketch.segments().len() {
                return;
            }
            let (start, end) = sketch.endpoints(segment);
            (start, (end - start).normalize_or(DVec2::X))
        }
    };

    let reach = sketch
        .bounds()
        .map(|(min, max)| (max - min).length())
        .unwrap_or(1.0)
        .max(1.0);
    let color = tint_at(theme.sketch_free, 0.9);
    lines.push(cao_render::Vertex::line(
        sketch.plane.to_world(origin - direction * reach).as_vec3(),
        color,
        2.0,
    ));
    lines.push(cao_render::Vertex::line(
        sketch.plane.to_world(origin + direction * reach).as_vec3(),
        color,
        2.0,
    ));
}

#[cfg(test)]
mod tests;
