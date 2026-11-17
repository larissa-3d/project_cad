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
/// the areas picked, the plane's normal, the depth its tip reaches along that
/// normal, in millimetres, signed by the direction of the matter, and the tip.
struct Arrow {
    foot: DVec3,
    normal: DVec3,
    depth: f64,
    tip: DVec3,
}

/// The arrow as the view draws it, and what its pill reads.
pub(crate) struct DrawnArrow {
    pub foot: DVec3,
    pub tip: DVec3,
    pub pill: String,
}

impl ExtrusionHandle {
    /// The arrow on the extrusion previewed, none while nothing is.
    pub(crate) fn drawn(extrusion: &ExtrusionState, document: &PartDocument) -> Option<DrawnArrow> {
        let arrow = arrow(extrusion, document)?;
        Some(DrawnArrow {
            foot: arrow.foot,
            tip: arrow.tip,
            pill: crate::wording::dimension::millimetres(arrow.depth.abs()),
        })
    }

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
        let along = (arrow.tip - origin).dot(direction);
        if (origin + direction * along - arrow.tip).length() > reach {
            return false;
        }
        let line = (arrow.foot, arrow.normal);
        self.held = closest_along(line, (origin, direction)).map(|taken_at| Held {
            line,
            taken_at,
            depth: arrow.depth,
            millimetres_per_unit: document.scale(),
        });
        self.held.is_some()
    }

    /// Moves the depth by as much as the pointer moved along the normal since
    /// the arrow was taken, and writes it in the panel, the direction turned
    /// round once the pointer passes the plane.
    pub(crate) fn pull(&mut self, extrusion: &mut ExtrusionState, origin: DVec3, direction: DVec3) {
        let Some(held) = &self.held else {
            return;
        };
        if grazes(held.line.1, direction.normalize_or_zero()) {
            return;
        }
        if let Some(at) = closest_along(held.line, (origin, direction)) {
            extrusion.drag_depth_to(held.depth + (at - held.taken_at) * held.millimetres_per_unit);
        }
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
    (low.x <= high.x).then(|| {
        let foot = drawing.plane.to_world((low + high) / 2.0);
        let normal = drawing.plane.normal();
        Arrow {
            foot,
            normal,
            depth,
            tip: foot + normal * (depth / document.scale()),
        }
    })
}

#[cfg(test)]
mod tests;
