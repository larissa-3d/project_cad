//! A face of a sphere bounded by the curve a cylinder of another axis meets
//! it along (#805) that neither the strips nor the central chart can cut: a
//! ball bored or notched off its axis, which holds both poles, or a band of a
//! sphere one such curve winds round.
//!
//! It is laid out in the sphere's own `(θ, φ)` as the strips lay it — each
//! sample of that curve, and of a circle of another axis, where it is read,
//! each pole the face holds along its floor, a face going round cut open at
//! an angle of the grid, its walls split at the grid's rows on lattice points
//! both walls share — then swept and refined as the central chart is
//! ([`super::central`]), each point added lifted onto the sphere from
//! `(θ, φ)`. A triangle with two corners on a floor stands for its pole twice
//! and is dropped, as on a cone, leaving a fan from the pole. Only a face
//! whose every such curve stands at least [`FRONT`] off both poles all along
//! it — a circle as [`super::charted`] reads it, the curve a cylinder meets
//! the sphere along at each of its samples — is laid so: the chart folds a
//! floor to a point, and a curve passing near one sweeps round it between two
//! of its samples.

use glam::{DVec2, DVec3};

use super::super::sampling::{Rounded, Samples, rows};
use super::super::sweep;
use super::central::FRONT;
use super::central::refinement::refined;
use super::charted::highest;
use super::round::Read;
use super::strips::Lattice;
use super::{Outline, laps};
use crate::brep::curve::{Curve, Wall};
use crate::brep::surface::Sphere;
use crate::brep::topology::{Body, FaceId};

/// Whether `curve` is the curve a cylinder meets `round`, a sphere, along,
/// and the face is read so that its samples stand where they are read.
pub(super) fn pierces(curve: &Curve, read: Read, round: &Rounded, eps: f64) -> bool {
    match (curve, round) {
        (Curve::Meet(meet), Rounded::Sphere(sphere)) => {
            read == Read::Tilted && meet.own(&Wall::Sphere(*sphere), eps) == Some(false)
        }
        _ => false,
    }
}

impl Outline {
    /// The face `id` of `sphere` laid out in its `(θ, φ)`, cut and refined
    /// within `tolerance`: the outline, its points inside added to
    /// `samples`, and its triangles. None for a face with no curve a
    /// cylinder meets it along, one a sample of which stands nearer a pole
    /// than the module's rule allows, and one the sweep or the refinement
    /// cannot cut.
    pub(super) fn pierced(
        body: &Body,
        samples: &mut Samples,
        id: FaceId,
        sphere: &Sphere,
        tolerance: f64,
    ) -> Option<(Outline, Vec<[usize; 3]>)> {
        let face = body.face(id);
        let eps = body.scale().eps();
        let round = Rounded::Sphere(*sphere);
        let steep = (1.0 - FRONT * FRONT).sqrt();
        let mut met = false;
        for coedge in face.loops.iter().flatten() {
            let edge = body.edge(coedge.edge);
            let curve = body.curve(edge.curve);
            let near = match curve {
                Curve::Circle(circle) => {
                    !round.holds(circle, eps)
                        && !round.meridian(circle, eps)
                        && highest(sphere, circle, edge.from, edge.to) > steep
                }
                _ if pierces(curve, Read::Tilted, &round, eps) => {
                    met = true;
                    samples.edge(coedge.edge).iter().any(|&sample| {
                        let from = samples.point(sample) - sphere.middle();
                        from.dot(sphere.axis).abs() > steep * sphere.radius
                    })
                }
                Curve::Line(_) | Curve::Meet(_) | Curve::Conic(_) => return None,
            };
            if near {
                return None;
            }
        }
        if !met {
            return None;
        }
        let laps = laps(body, samples, id)?;
        let grid = (samples.steps(face.surface)?, samples.rows(face.surface)?);
        let mut outline = Outline::default();
        let mut lattice =
            outline.laid_out(body, samples, face, &laps, grid, (&round, Read::Tilted))?;
        outline.walls_split(samples, &mut lattice);
        let swept = sweep::triangles(&outline.points, &outline.segments)?;
        let places: Vec<DVec3> = (outline.samples.iter())
            .map(|&id| samples.point(id) - sphere.middle())
            .collect();
        let (added, triangles) = refined(
            sphere,
            (&outline.points, &places),
            &outline.segments,
            swept,
            tolerance,
        )?;
        for at in added {
            outline.points.push(at);
            outline.samples.push(samples.added(sphere.point(at)));
        }
        Some((outline, triangles))
    }

    /// Every wall closing the face cut open split at each row of the
    /// lattice's grid it crosses, on the lattice's point there, so that its
    /// chords stand within the tolerance of the meridian it runs along as
    /// the strips' walls do; the two walls of the cut, a turn apart, stand
    /// for the same points.
    fn walls_split(&mut self, samples: &mut Samples, lattice: &mut Lattice) {
        let step = std::f64::consts::TAU / lattice.steps as f64;
        for index in std::mem::take(&mut self.walls) {
            let [from, to] = self.segments[index];
            let (start, end) = (self.points[from], self.points[to]);
            let (low, high) = (start.y.min(end.y), start.y.max(end.y));
            let mut levels = rows(lattice.rows, low, high, &lattice.levels, lattice.room);
            if end.y < start.y {
                levels.reverse();
            }
            let rank = (start.x / step).round() as i64;
            let mut last = from;
            for (at, level) in levels.into_iter().enumerate() {
                let sample = lattice.sample(samples, rank, level);
                let point = self.point(DVec2::new(start.x, level), sample);
                if at == 0 {
                    self.segments[index] = [last, point];
                } else {
                    self.segments.push([last, point]);
                }
                last = point;
            }
            if last != from {
                self.segments.push([last, to]);
            }
        }
    }
}
