//! The tool in progress, in the profile's colour for it: a ring round the
//! place a snap pulled the cursor onto, named beside it, and, while a line is
//! drawn, the arc it turns at its start from the trait it is drawn on from —
//! or from the horizontal — the angle that arc spans written beside it.

use cao_prefs::theme::Theme;
use cao_sketch::{Sketch, Snap, ToolState};
use glam::DVec2;

use super::curves::{push_circle_at, push_curve};
use super::live_fields::LIVE_FIELDS;
use super::overlays::{tint_to_color, to_screen};
use super::tint;
use crate::screens::SketchContext;
use crate::screens::sketch::Tool;
use crate::screens::viewport::{ViewScale, ViewportState};
use crate::ui::theme::tokens::{
    BACKGROUND_FLOATING, RADIUS_PILL, TYPOGRAPHY_SIZES_DETAIL, TYPOGRAPHY_SIZES_MONO_VALUE,
};
use crate::wording;

/// How far round the place the cursor was pulled onto its ring runs, on
/// screen.
const RING_PIXELS: f64 = 8.0;

/// How far from the start of a line its angle's arc runs, on screen.
const ARC_PIXELS: f64 = 30.0;

/// How many straight pieces the arc is drawn in.
const ARC_PIECES: usize = 32;

/// Where a snap's name ends from its ring, up and to the left: down and to the
/// right is where the cursor carries its fields. And how far an angle stands
/// out past its arc, in points — on whichever side of the start keeps it off
/// those fields.
const NAME_FROM_THE_RING: egui::Vec2 = egui::vec2(-10.0, -10.0);
const ANGLE_CLEARS_ITS_ARC: f32 = 6.0;

/// The room a pill leaves around what it says, in points.
const PILL_PADDING: egui::Vec2 = egui::vec2(6.0, 2.0);

/// The arc a line in progress turns at its start: where it starts, the way it
/// is measured from, of length one, how far round it turns to the line, in
/// radians and signed, and the angle the field reads for it.
struct Turning {
    start: DVec2,
    from: DVec2,
    sweep: f64,
    typed: f64,
}

/// The ring and the arc, in the scene.
pub(super) fn push_tool_in_progress(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    scale: ViewScale,
    context: &SketchContext<'_>,
) {
    let colour = tint(theme.tool_in_progress);
    if let Some((_, at)) = snapped(context) {
        let radius = scale.world_size_of(RING_PIXELS);
        push_circle_at(out, sketch, at, radius, colour, 1.5, false, scale);
    }
    if let Some(turning) = turning(sketch, context) {
        let radius = scale.world_size_of(ARC_PIXELS);
        let places = (0..=ARC_PIECES).map(|piece| {
            let angle = turning.sweep * piece as f64 / ARC_PIECES as f64;
            turning.start + DVec2::from_angle(angle).rotate(turning.from) * radius
        });
        push_curve(out, sketch, places, colour, 1.5, false, scale);
    }
}

/// The snap's name beside its ring, and the angle beside its arc, each in a
/// pill bordered in the tool's colour.
pub(crate) fn paint_tool_in_progress(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
) {
    let Some(sketch) =
        (context.editor.active_sketch()).and_then(|index| context.document.sketches().get(index))
    else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));
    let screen = |place: DVec2| to_screen(sketch.plane.to_world(place), view_projection, rect);
    let painter = ui.painter_at(rect);
    let colour = tint_to_color(state.theme.tool_in_progress);

    if let Some((snap, at)) = snapped(context)
        && let Some(ring) = screen(at)
    {
        let said = wording::snap::name(context.lang, snap);
        let font = egui::FontId::proportional(TYPOGRAPHY_SIZES_DETAIL);
        in_a_pill(&painter, (said, font), colour, |size| {
            let ends = ring + NAME_FROM_THE_RING;
            egui::Rect::from_min_max(ends - size, ends)
        });
    }

    let pixel = state
        .camera
        .world_units_per_pixel(rect.height() * ui.ctx().pixels_per_point()) as f64;
    if let Some(turning) = turning(sketch, context) {
        let halfway = DVec2::from_angle(turning.sweep / 2.0).rotate(turning.from);
        let on_the_arc = turning.start + halfway * ARC_PIXELS * pixel;
        if let (Some(start), Some(on_the_arc)) = (screen(turning.start), screen(on_the_arc)) {
            let tenths = (turning.typed.abs() * 10.0).round() / 10.0;
            let said = wording::dimension::degrees(tenths);
            let font = egui::FontId::monospace(TYPOGRAPHY_SIZES_MONO_VALUE);
            let outward = (on_the_arc - start).normalized();
            let fields = ui
                .ctx()
                .memory(|memory| memory.area_rect(egui::Id::new(LIVE_FIELDS)));
            in_a_pill(&painter, (said, font), colour, |size| {
                let reach =
                    (on_the_arc - start).length() + size.length() / 2.0 + ANGLE_CLEARS_ITS_ARC;
                let beside = [outward, -outward, -egui::Vec2::Y, -egui::Vec2::X]
                    .map(|way| egui::Rect::from_center_size(start + way * reach, size));
                beside
                    .into_iter()
                    .find(|pill| fields.is_none_or(|fields| !pill.intersects(fields)))
                    .unwrap_or(beside[0])
            });
        }
    }
}

/// The place a snap pulled the cursor onto, while the cursor is on the
/// canvas: the cursor itself for a point of the drawing, which carries no
/// place of its own. A tool that only picks what is drawn rings nothing: the
/// ring says where a point is about to be laid.
fn snapped(context: &SketchContext<'_>) -> Option<(Snap, DVec2)> {
    let picks = matches!(
        context.editor.tool,
        Tool::Dimension
            | Tool::Measure
            | Tool::Trim
            | Tool::Split
            | Tool::Chamfer
            | Tool::Fillet
            | Tool::Constrain(_)
    );
    if picks {
        return None;
    }
    let (snap, cursor) = (context.editor.snap?, context.editor.cursor?);
    let at = match snap {
        Snap::Point => cursor,
        Snap::Crossing(at) | Snap::Midpoint(at) | Snap::Centre(at) | Snap::OnCurve(at) => at,
    };
    Some((snap, at))
}

/// The arc of the line in progress, measured the way its angle field is: from
/// the way back along the trait it is drawn on from, or else the horizontal.
fn turning(sketch: &Sketch, context: &SketchContext<'_>) -> Option<Turning> {
    let ToolState::Line { anchor, previous } = context.editor.tool_state else {
        return None;
    };
    let start = sketch.anchor_position(anchor)?;
    let cursor = context.editor.cursor?;
    let end = context.editor.aimed.map_or(cursor, |aimed| aimed.position);
    let along = (end - start).try_normalize()?;
    let from = sketch
        .drawn_on_from(anchor, previous)
        .and_then(|on| {
            let (one, other) = sketch.endpoints(on);
            let far = match one.distance(start) < other.distance(start) {
                true => other,
                false => one,
            };
            (far - start).try_normalize()
        })
        .unwrap_or(DVec2::X);
    Some(Turning {
        start,
        from,
        sweep: from.angle_to(along),
        typed: sketch.angle_as_typed(anchor, previous, end)?,
    })
}

/// `said` in a pill bordered in `colour`, laid where `place` puts a pill of
/// the size it asks.
fn in_a_pill(
    painter: &egui::Painter,
    (said, font): (String, egui::FontId),
    colour: egui::Color32,
    place: impl FnOnce(egui::Vec2) -> egui::Rect,
) {
    let galley = painter.layout_no_wrap(said, font, colour);
    let pill = place(galley.size() + 2.0 * PILL_PADDING);
    painter.rect(
        pill,
        RADIUS_PILL,
        BACKGROUND_FLOATING,
        egui::Stroke::new(1.0, colour),
        egui::StrokeKind::Inside,
    );
    painter.galley(pill.min + PILL_PADDING, galley, colour);
}
