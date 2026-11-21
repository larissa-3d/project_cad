//! One rule for how near is the same, measured on the drawing's size.
//!
//! Three levels, each read against [`drawing_size`] so that a drawing means
//! the same thing in any unit and anywhere on the plane. The rest of the
//! crate still carries older constants of its own; they move onto these
//! levels part by part (#514).

use glam::DVec2;

use crate::sketch::{Element, Sketch};

/// How near is treated as one at the use: a tenth of a micron on a part up to
/// a metre.
pub const USE: f64 = 1e-7;

/// How near two places must stand to be the same place: a hundredth of a
/// micron on a part up to a metre.
pub const SAME: f64 = 1e-8;

/// How near a solved value must come to its dimension: a thousandth of a
/// micron on a part up to a metre.
pub const SOLVED: f64 = 1e-9;

/// Whether a trait this long is shorter than [`USE`] on a drawing `scale`
/// across: nothing the user can type tells it from a point.
pub fn is_collapsed(length: f64, scale: f64) -> bool {
    length < scale * USE
}

/// Whether any trait is shorter than [`USE`] on a drawing `scale` across.
pub(crate) fn has_a_collapsed_trait(sketch: &Sketch, scale: f64) -> bool {
    sketch
        .live_segments()
        .any(|(id, _)| is_collapsed(sketch.segment_length(id), scale))
}

/// Each circle's radius and each arc's reach from its centre to either end
/// that is under [`USE`] on a drawing `scale` across: the curve, and whether
/// it is the arc's end rather than its start.
pub(crate) fn collapsed_reaches(sketch: &Sketch, scale: f64) -> Vec<(Element, bool)> {
    let circles =
        (sketch.live_circles()).map(|(id, circle)| ((Element::Circle(id), false), circle.radius));
    let arcs = sketch.live_arcs().flat_map(|(id, arc)| {
        let centre = sketch.point(arc.center);
        [(false, arc.start), (true, arc.end)].map(|(at_end, end)| {
            (
                (Element::Arc(id), at_end),
                centre.distance(sketch.point(end)),
            )
        })
    });
    circles
        .chain(arcs)
        .filter(|(_, reach)| is_collapsed(*reach, scale))
        .map(|(reach, _)| reach)
        .collect()
}

/// Whether a reach [`collapsed_reaches`] did not hold in `small_already` has
/// come under [`USE`] since.
pub(crate) fn has_shrunk_a_curve(
    small_already: &[(Element, bool)],
    sketch: &Sketch,
    scale: f64,
) -> bool {
    collapsed_reaches(sketch, scale)
        .iter()
        .any(|reach| !small_already.contains(reach))
}

/// Whether `sketch` holds a reach under [`USE`] that `was` did not, both read
/// on `was`'s size.
pub(crate) fn has_shrunk_a_curve_since(was: &Sketch, sketch: &Sketch) -> bool {
    let scale = drawing_size(was);
    has_shrunk_a_curve(&collapsed_reaches(was, scale), sketch, scale)
}

/// How big a drawing is: the diagonal of the smallest box holding every live
/// curve by its extent, construction curves included and the origin left out.
/// One unit while nothing is drawn, or while what is drawn spans under a
/// millionth of one — the solver's own guard against a collapsed drawing — so
/// a tolerance read against it still means something.
pub fn drawing_size(sketch: &Sketch) -> f64 {
    let mut corners: Option<(DVec2, DVec2)> = None;
    let mut take = |low: DVec2, high: DVec2| {
        corners = Some(match corners {
            None => (low, high),
            Some((min, max)) => (min.min(low), max.max(high)),
        });
    };
    for (_, segment) in sketch.live_segments() {
        let (from, to) = (sketch.point(segment.start), sketch.point(segment.end));
        take(from.min(to), from.max(to));
    }
    for (id, _) in sketch.live_arcs() {
        let (low, high) = crate::arcing::bounds_of(sketch.arc_draft(id));
        take(low, high);
    }
    for (_, circle) in sketch.live_circles() {
        let centre = sketch.point(circle.center);
        let reach = DVec2::splat(circle.radius);
        take(centre - reach, centre + reach);
    }
    for (id, _) in sketch.live_ellipses() {
        let (low, high) = sketch.ellipse_bounds(id);
        take(low, high);
    }
    corners
        .map(|(min, max)| (max - min).length())
        .filter(|size| *size > 1e-6)
        .unwrap_or(1.0)
}

#[cfg(test)]
mod tests;
