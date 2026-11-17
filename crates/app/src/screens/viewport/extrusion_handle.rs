//! The amber arrow on an extrusion's preview, which a drag pulls to set the
//! depth — or, round an arc, a revolution's angle.

use cao_part::{Operation, PartDocument, RevolutionAxis};
use cao_sketch::Sketch;
use glam::{DVec2, DVec3};

use super::section::{GRAZING_DEGREES, closest_along, grazes};
use crate::screens::extrusion::ExtrusionState;

/// How many straight pieces a revolution's arc is drawn in.
const ARC_PIECES: usize = 32;

#[derive(Default)]
pub(crate) struct ExtrusionHandle {
    held: Option<Held>,
}

/// Where the arrow was taken.
enum Held {
    /// The line it slides along, how far along it the pointer was, the depth
    /// the arrow reached then, and how many millimetres a unit of the view is
    /// worth.
    Depth {
        line: (DVec3, DVec3),
        taken_at: f64,
        millimetres: f64,
        millimetres_per_unit: f64,
    },
    /// The arc it turns round, the angle the arrow reached then, how far round
    /// the pointer was then, and how far round it has been followed since, in
    /// degrees.
    Angle {
        turn: Turn,
        degrees: f64,
        taken_at: f64,
        followed_to: f64,
    },
}

/// What the arrow pulls, signed by the direction of the matter.
#[derive(Clone, Copy)]
enum Pulls {
    /// A depth along the plane's normal, in millimetres.
    Depth {
        normal: DVec3,
        millimetres: f64,
        millimetres_per_unit: f64,
    },
    /// An angle round the axis, in degrees.
    Angle { turn: Turn, degrees: f64 },
}

/// The circle a revolution turns the middle of its areas along: the point of
/// the axis square to that middle, the axis, of length one, and the way from
/// that point out to the middle.
#[derive(Clone, Copy)]
struct Turn {
    centre: DVec3,
    axis: DVec3,
    out: DVec3,
}

/// The arrow as the view draws it, and what its pill reads.
pub(crate) struct DrawnArrow {
    pub foot: DVec3,
    pub tip: DVec3,
    pub pill: String,
    pulls: Pulls,
}

impl DrawnArrow {
    /// The places the arrow runs through, from its foot to its tip.
    pub(crate) fn run(&self) -> Vec<DVec3> {
        match self.pulls {
            Pulls::Depth { .. } => vec![self.foot, self.tip],
            Pulls::Angle { turn, degrees } => (0..=ARC_PIECES)
                .map(|piece| turn.turned(degrees * piece as f64 / ARC_PIECES as f64))
                .collect(),
        }
    }
}

impl ExtrusionHandle {
    /// The arrow on the extrusion previewed, none while nothing is.
    pub(crate) fn drawn(extrusion: &ExtrusionState, document: &PartDocument) -> Option<DrawnArrow> {
        let (sketch, areas) = match extrusion.preview(document)?.operation {
            Operation::Extrude { sketch, areas, .. } | Operation::Revolve { sketch, areas, .. } => {
                (sketch, areas)
            }
            _ => return None,
        };
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
        if low.x > high.x {
            return None;
        }
        let foot = drawing.plane.to_world((low + high) / 2.0);
        let variables = document.variables();
        let millimetres_per_unit = document.scale();
        let (tip, pill, pulls) = match extrusion.is_revolving() {
            true => {
                let degrees = extrusion.angle(variables)?.1;
                let turn = Turn::of(drawing, extrusion.axis, foot)?;
                let pill = crate::wording::dimension::degrees(degrees.abs());
                (turn.turned(degrees), pill, Pulls::Angle { turn, degrees })
            }
            false => {
                let millimetres = extrusion.distance(variables)?.1;
                let normal = drawing.plane.normal();
                let tip = foot + normal * (millimetres / millimetres_per_unit);
                let pill = crate::wording::dimension::millimetres(millimetres.abs());
                let pulls = Pulls::Depth {
                    normal,
                    millimetres,
                    millimetres_per_unit,
                };
                (tip, pill, pulls)
            }
        };
        Some(DrawnArrow {
            foot,
            tip,
            pill,
            pulls,
        })
    }

    pub(crate) fn is_held(&self) -> bool {
        self.held.is_some()
    }

    /// Takes hold of the arrow when the ray passes within `reach` of its tip,
    /// and runs neither along the normal it slides on nor along the plane its
    /// arc turns in.
    pub(crate) fn grab(
        &mut self,
        extrusion: &ExtrusionState,
        document: &PartDocument,
        origin: DVec3,
        direction: DVec3,
        reach: f64,
    ) -> bool {
        self.held = None;
        let Some(arrow) = Self::drawn(extrusion, document) else {
            return false;
        };
        let direction = direction.normalize_or_zero();
        let along = (arrow.tip - origin).dot(direction);
        if (origin + direction * along - arrow.tip).length() > reach {
            return false;
        }
        self.held = match arrow.pulls {
            Pulls::Depth {
                normal,
                millimetres,
                millimetres_per_unit,
            } => {
                let line = (arrow.foot, normal);
                let taken_at = closest_along(line, (origin, direction));
                taken_at
                    .filter(|_| !grazes(normal, direction))
                    .map(|taken_at| Held::Depth {
                        line,
                        taken_at,
                        millimetres,
                        millimetres_per_unit,
                    })
            }
            Pulls::Angle { turn, degrees } => {
                turn.round(origin, direction).map(|taken_at| Held::Angle {
                    turn,
                    degrees,
                    taken_at,
                    followed_to: taken_at,
                })
            }
        };
        self.held.is_some()
    }

    /// Moves the depth by as much as the pointer moved along the normal since
    /// the arrow was taken — or the angle by as far as it went round the axis,
    /// a whole turn at most — and writes it in the panel, the direction
    /// turned round once the pointer passes the plane.
    pub(crate) fn pull(&mut self, extrusion: &mut ExtrusionState, origin: DVec3, direction: DVec3) {
        let direction = direction.normalize_or_zero();
        match &mut self.held {
            None => {}
            Some(Held::Depth {
                line,
                taken_at,
                millimetres,
                millimetres_per_unit,
            }) => {
                if let Some(at) =
                    closest_along(*line, (origin, direction)).filter(|_| !grazes(line.1, direction))
                {
                    extrusion.drag_to(*millimetres + (at - *taken_at) * *millimetres_per_unit);
                }
            }
            Some(Held::Angle {
                turn,
                degrees,
                taken_at,
                followed_to,
            }) => {
                if let Some(round) = turn.round(origin, direction) {
                    *followed_to += (round - *followed_to + 180.0).rem_euclid(360.0) - 180.0;
                    extrusion.drag_to((*degrees + *followed_to - *taken_at).clamp(-360.0, 360.0));
                }
            }
        }
    }

    pub(crate) fn let_go(&mut self) {
        self.held = None;
    }
}

impl Turn {
    /// The circle `foot` turns along about a revolution's axis; none when it
    /// stands on the axis, or the axis is not there.
    fn of(drawing: &Sketch, axis: RevolutionAxis, foot: DVec3) -> Option<Self> {
        let (origin, along) = axis_in_drawing(drawing, axis)?;
        let plane = drawing.plane;
        let axis = (plane.u * along.x + plane.v * along.y).normalize_or_zero();
        let origin = plane.to_world(origin);
        let centre = origin + axis * axis.dot(foot - origin);
        let out = foot - centre;
        (out.length() > 1e-9 && axis != DVec3::ZERO).then_some(Self { centre, axis, out })
    }

    /// Where the middle stands turned that many degrees, by the right hand
    /// about the axis, the way the kernel turns.
    fn turned(&self, degrees: f64) -> DVec3 {
        let (sin, cos) = degrees.to_radians().sin_cos();
        self.centre + self.out * cos + self.axis.cross(self.out) * sin
    }

    /// How far round the axis from the middle, in degrees, the ray meets the
    /// plane square to the axis through it; none for a ray running within a
    /// few degrees of that plane.
    fn round(&self, origin: DVec3, direction: DVec3) -> Option<f64> {
        let across = direction.dot(self.axis);
        if across.abs() < GRAZING_DEGREES.to_radians().sin() {
            return None;
        }
        let met = origin + direction * ((self.centre - origin).dot(self.axis) / across);
        let from = met - self.centre;
        let degrees = from
            .dot(self.axis.cross(self.out))
            .atan2(from.dot(self.out));
        Some(degrees.to_degrees())
    }
}

/// Where a revolution's axis lies in its drawing: a point of it, and its way,
/// of length one; none for a trait the drawing no longer holds — as
/// `render::extrusion` draws it.
fn axis_in_drawing(drawing: &Sketch, axis: RevolutionAxis) -> Option<(DVec2, DVec2)> {
    match axis {
        RevolutionAxis::Sketch(axis) => Some((DVec2::ZERO, axis.direction())),
        RevolutionAxis::Segment(segment) => {
            if segment.0 >= drawing.segments().len() {
                return None;
            }
            let (start, end) = drawing.endpoints(segment);
            Some((start, (end - start).normalize_or(DVec2::X)))
        }
    }
}

#[cfg(test)]
mod tests;
