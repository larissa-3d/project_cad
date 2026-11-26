//! A border drawn in dashes, round at its corners: what marks a place still
//! empty, a chip waiting for a click as much as a list with nothing in it.

use std::f32::consts::{FRAC_PI_2, PI};

use egui::{Painter, Pos2, Rect, Shape, Stroke, vec2};

/// The dashes of the border, and the gaps between them.
const DASH: f32 = 3.0;
const GAP: f32 = 3.0;

/// How many straight pieces each round corner is drawn in.
const ARC_PIECES: usize = 6;

/// A dashed border just inside `rect`, its corners rounded by `radius` — as
/// far as half its height, which makes a pill of it.
pub fn dashed_border(painter: &Painter, rect: Rect, radius: f32, stroke: Stroke) {
    let outline = rounded_outline(rect.shrink(stroke.width / 2.0), radius);
    painter.extend(Shape::dashed_line(&outline, stroke, DASH, GAP));
}

/// The outline of `rect` with its corners rounded, as points a dashed line can
/// follow all the way round.
fn rounded_outline(rect: Rect, radius: f32) -> Vec<Pos2> {
    let radius = radius.min(rect.width() / 2.0).min(rect.height() / 2.0);
    let corners = [
        (rect.right() - radius, rect.top() + radius, -FRAC_PI_2),
        (rect.right() - radius, rect.bottom() - radius, 0.0),
        (rect.left() + radius, rect.bottom() - radius, FRAC_PI_2),
        (rect.left() + radius, rect.top() + radius, PI),
    ];
    let mut outline: Vec<Pos2> = corners
        .into_iter()
        .flat_map(|(x, y, from)| {
            (0..=ARC_PIECES).map(move |piece| {
                let angle = from + FRAC_PI_2 * piece as f32 / ARC_PIECES as f32;
                Pos2::new(x, y) + radius * vec2(angle.cos(), angle.sin())
            })
        })
        .collect();
    outline.dedup();
    if let Some(&start) = outline.first() {
        outline.push(start);
    }
    outline
}
