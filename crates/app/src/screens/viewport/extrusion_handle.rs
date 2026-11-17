//! The amber arrow on an extrusion's preview, which a drag pulls to set the
//! depth.

use cao_part::{Operation, PartDocument};
use glam::{DVec2, DVec3};

use super::section::{closest_along, grazes};
use crate::screens::extrusion::ExtrusionState;

#[derive(Default)]
pub(crate) struct ExtrusionHandle {
    held: Option<Held>,
}

/// Where the arrow was taken: the line it slides along, how far along it the
/// pointer was, the depth the arrow reached then, and how many millimetres a
/// unit of the view is worth.
struct Held {
    line: (DVec3, DVec3),
    taken_at: f64,
    depth: f64,
    millimetres_per_unit: f64,
}

/// The arrow of the preview: its foot on the sketch's plane at the middle of
/// the areas picked, the plane's normal, and the depth its tip reaches along
/// that normal, in millimetres, signed by the direction of the matter.
struct Arrow {
    foot: DVec3,
    normal: DVec3,
    depth: f64,
}

impl ExtrusionHandle {
    /// Takes hold of the arrow when the ray passes within `reach` of its tip,
    /// and does not graze the normal.
    pub(crate) fn grab(
        &mut self,
        extrusion: &ExtrusionState,
        document: &PartDocument,
        origin: DVec3,
        direction: DVec3,
        reach: f64,
    ) -> bool {
        self.held = None;
        let Some(arrow) = arrow(extrusion, document) else {
            return false;
        };
        let direction = direction.normalize_or_zero();
        if grazes(arrow.normal, direction) {
            return false;
        }
        let millimetres_per_unit = document.scale();
        let tip = arrow.foot + arrow.normal * (arrow.depth / millimetres_per_unit);
        let along = (tip - origin).dot(direction);
        if (origin + direction * along - tip).length() > reach {
            return false;
        }
        let line = (arrow.foot, arrow.normal);
        self.held = closest_along(line, (origin, direction)).map(|taken_at| Held {
            line,
            taken_at,
            depth: arrow.depth,
            millimetres_per_unit,
        });
        self.held.is_some()
    }

    /// Moves the depth by as much as the pointer moved along the normal since
    /// the arrow was taken, and writes it in the panel's field as a number.
    pub(crate) fn pull(&mut self, extrusion: &mut ExtrusionState, origin: DVec3, direction: DVec3) {
        let Some(held) = &self.held else {
            return;
        };
        if grazes(held.line.1, direction.normalize_or_zero()) {
            return;
        }
        let Some(at) = closest_along(held.line, (origin, direction)) else {
            return;
        };
        let depth = held.depth + (at - held.taken_at) * held.millimetres_per_unit;
        let typed = match extrusion.reversed {
            true => -depth,
            false => depth,
        };
        extrusion.drag_depth_to(crate::wording::dimension::short(typed));
    }

    pub(crate) fn let_go(&mut self) {
        self.held = None;
    }
}

/// The arrow of the extrusion previewed, none while nothing is.
fn arrow(extrusion: &ExtrusionState, document: &PartDocument) -> Option<Arrow> {
    let Operation::Extrude {
        sketch,
        areas,
        distance,
        ..
    } = extrusion.preview(document)?.operation
    else {
        return None;
    };
    let depth = distance.value(&document.variables().values())?;
    let drawing = document.sketches().get(sketch)?;
    let regions = drawing.regions();
    let (low, high) = document
        .areas_standing(sketch, &areas)
        .into_iter()
        .filter_map(|rank| regions.get(rank))
        .flat_map(|region| region.outline.points.iter().copied())
        .fold(
            (DVec2::INFINITY, DVec2::NEG_INFINITY),
            |(low, high), point| (low.min(point), high.max(point)),
        );
    (low.x <= high.x).then(|| Arrow {
        foot: drawing.plane.to_world((low + high) / 2.0),
        normal: drawing.plane.normal(),
        depth,
    })
}

#[cfg(test)]
mod tests;
