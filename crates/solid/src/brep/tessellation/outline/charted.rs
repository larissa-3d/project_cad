//! A face of a sphere with a circle of another axis among its edges (#805)
//! that neither the strips nor the central chart can cut: the circle runs
//! along no line of the sphere's `(θ, φ)`, and the face reaches ninety
//! degrees or more from every direction.
//!
//! A face every circle of which turns about one axis — half a ball cut by a
//! plane at a slant through its centre — is read in the chart of its sphere
//! about that axis, where each is a latitude, and cut in strips there; the
//! pole its loops turn round, which it holds, is a point added inside it.
//!
//! Any other — the sphere of a rib cut from a ball, once a fill is trimmed
//! flush on it — is laid out in the sphere's own `(θ, φ)` as the strips lay
//! it, each sample of a circle of another axis where it is read, the strip
//! along a latitude a wall touches it along laid as the strips lay it
//! ([`super::touched`]), then swept and refined as the central chart is
//! ([`super::central`]), each point added lifted onto the sphere from
//! `(θ, φ)`. Only a face that holds no pole, whose every edge stands at least
//! [`FRONT`] off both all along it as the central chart's samples stand off
//! its plane, and that turns round the axis by nought, its region on the
//! bounded side of its loops, is laid so: the chart folds a floor to a
//! point, and a circle passing near one sweeps round it between two of its
//! samples.

use std::f64::consts::PI;

use glam::DVec3;

use super::super::sampling::{Rounded, Samples, read_about};
use super::super::sweep;
use super::central::FRONT;
use super::central::refinement::refined;
use super::round::Read;
use super::touched::touched;
use super::{Outline, knife, laps};
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::Sphere;
use crate::brep::topology::{Body, FaceId};

impl Outline {
    /// The face `id` of `sphere` laid out in its `(θ, φ)`, cut and refined
    /// within `tolerance`: the outline, its points inside added to
    /// `samples`, and its triangles. None for a face with no circle of
    /// another axis, one the module's rule leaves out, and one the sweep or
    /// the refinement cannot cut.
    pub(super) fn charted(
        body: &Body,
        samples: &mut Samples,
        id: FaceId,
        sphere: &Sphere,
        tolerance: f64,
    ) -> Option<(Outline, Vec<[usize; 3]>)> {
        let face = body.face(id);
        let steep = (1.0 - FRONT * FRONT).sqrt();
        let near_a_pole = (face.loops.iter().flatten()).any(|coedge| {
            let edge = body.edge(coedge.edge);
            match body.curve(edge.curve) {
                Curve::Circle(circle) => highest(sphere, circle, edge.from, edge.to) > steep,
                Curve::Line(_) | Curve::Meet(_) | Curve::Conic(_) => true,
            }
        });
        if !face.poles.is_empty() || near_a_pole {
            return None;
        }
        let laps = laps(body, samples, id)?;
        let grid = (samples.steps(face.surface)?, samples.rows(face.surface)?);
        let round = Rounded::Sphere(*sphere);
        let mut outline = Outline::default();
        let mut lattice =
            outline.laid_out(body, samples, face, &laps, grid, (&round, Read::Tilted))?;
        let area: f64 = (outline.segments.iter())
            .map(|&[from, to]| outline.points[from].perp_dot(outline.points[to]))
            .sum();
        if !outline.walls.is_empty() || area <= 0.0 {
            return None;
        }
        let touched = touched(body, face, sphere, body.scale().eps());
        outline.along_touched_latitudes(samples, &mut lattice, &touched);
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

    /// The face `id` of `sphere` read about the axis every circle of it turns
    /// about, and cut in strips there within `tolerance`, on that reading's
    /// own grid of angles ([`read_about`]): cut for the sphere's radius, and
    /// beside a ring a wall of the axis leaves thinner than its chords sag
    /// lined up with that wall, so that every sample of the rim has a lattice
    /// point on its ray. No row stands in the band beside a knife-edge rim
    /// ([`knife::Band`]), whose lattice points would stand outside the
    /// wall's chords. None where an edge is no circle, where two turn about
    /// different axes, and where the strips cannot cut it.
    pub(super) fn about(
        body: &Body,
        samples: &mut Samples,
        id: FaceId,
        sphere: &Sphere,
        tolerance: f64,
    ) -> Option<(Outline, Vec<[usize; 3]>)> {
        let face = body.face(id);
        let mut axes = (face.loops.iter().flatten()).map(|coedge| {
            match body.curve(body.edge(coedge.edge).curve) {
                Curve::Circle(circle) => Some(circle.axis),
                Curve::Line(_) | Curve::Meet(_) | Curve::Conic(_) => None,
            }
        });
        let axis = axes.next()??;
        let shared = |other: Option<DVec3>| {
            other.is_some_and(|other| other.cross(axis).length() <= Scale::RELATIVE)
        };
        if !axes.all(shared) {
            return None;
        }
        let round = Rounded::Sphere(sphere.about(axis));
        let eps = body.scale().eps();
        let [low, high] = round.poles(eps)[..] else {
            return None;
        };
        let poles = [low, high].map(|pole| samples.added(pole.point));
        let laps = laps(body, samples, id)?;
        let rows = samples.rows(face.surface)?;
        let about = read_about(body, sphere, axis, rows, (tolerance, samples.span));
        let grid = (about.steps_of(body, id, sphere, axis), rows);
        let mut outline = Outline::default();
        let mut lattice = outline.laid_out(
            body,
            samples,
            face,
            &laps,
            grid,
            (&round, Read::About(poles)),
        )?;
        lattice.bands = knife::bands(body, samples, id, (sphere, &round), &outline.samples);
        outline.stripped(samples, &mut lattice)?;
        let cut = outline.cut()?;
        Some((outline, cut))
    }
}

/// How near the poles of `sphere` the stretch of `circle` from `from` to `to`
/// comes: the largest sine of its angle along the meridian. Along the circle
/// its height over the centre runs as a constant and a cosine of its
/// parameter, highest or lowest at the stretch's ends or at the cosine's
/// crests.
pub(super) fn highest(sphere: &Sphere, circle: &Circle, from: f64, to: f64) -> f64 {
    let axis = sphere.axis;
    let over = (circle.center - sphere.middle()).dot(axis);
    let (along_u, along_v) = (circle.u.dot(axis), circle.v.dot(axis));
    let crest = along_v.atan2(along_u);
    let (low, high) = (from.min(to), from.max(to));
    let first = ((low - crest) / PI).ceil() as i64;
    let last = ((high - crest) / PI).floor() as i64;
    let crests = (first..=last).map(|turn| crest + PI * turn as f64);
    (crests.chain([from, to]))
        .map(|t| (over + circle.radius * (along_u * t.cos() + along_v * t.sin())).abs())
        .fold(0.0, f64::max)
        / sphere.radius
}
