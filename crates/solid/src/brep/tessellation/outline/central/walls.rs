//! Whether the points the central chart added inside a face of a sphere
//! stand within the chords of the walls the face meets along their own
//! circles (#1138).
//!
//! The chart's straight lines are the sphere's great circles, and the sweep
//! joins the samples of a rim along them. Where the sphere meets a wall at a
//! knife edge — a shaft of 4.9 less a ball of 5, their surfaces cos⁻¹ 0.98
//! apart along the rim — the rim is all but a great circle, and those lines
//! hug it: the middles the refinement lifts onto the sphere stand inside the
//! wall, within a chord's sag of it, but outside the chords between the
//! rim's samples the wall is drawn along, and the sphere's triangles there
//! cross the wall's. Met at a wide angle, the sphere stands within a chord's
//! sag of the wall over a band narrower than those lines bow away from the
//! rim, and no point falls in it. A face with such a point is left to the
//! charts after this one: the strips about the wall's axis, where every
//! circle of the face turns about it.

use glam::{DVec2, DVec3};

use super::super::super::sampling::Samples;
use super::super::laps;
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::topology::{Body, EdgeId, FaceId};

/// Whether every sample from `from` on among `samples_of`, the points added
/// inside the face `id` of a sphere, stands within the chords of each wall
/// the face meets along a circle about the wall's axis, wherever it stands
/// inside that wall on the side of the circle the wall's face runs to.
pub(in crate::brep::tessellation::outline) fn within_the_walls(
    body: &Body,
    samples: &Samples,
    id: FaceId,
    (samples_of, from): (&[usize], usize),
) -> bool {
    let added: Vec<DVec3> = (samples_of.iter())
        .filter(|&&sample| sample >= from)
        .map(|&sample| samples.point(sample))
        .collect();
    body.face(id).loops.iter().flatten().all(|coedge| {
        (body.uses(coedge.edge).into_iter())
            .filter(|&(wall, _)| wall != id)
            .all(|(wall, _)| {
                Rim::of(body, samples, coedge.edge, wall)
                    .is_none_or(|rim| !added.iter().any(|&point| rim.passed_by(point)))
            })
    })
}

/// A circle a wall's face is bounded by, about the wall's own axis, as its
/// samples draw it.
struct Rim {
    wall: Cylinder,
    /// The circle's height along the wall's axis.
    level: f64,
    /// Which way along the axis the wall's face runs from the circle.
    side: f64,
    /// The samples, in the wall's plane square to its axis, each joined to
    /// the next.
    corners: Vec<DVec2>,
    /// How near the axis the chords between them come.
    nearest: f64,
}

impl Rim {
    /// The rim the edge `edge` draws on the face `wall`, if the face is of a
    /// wall and the edge a circle about its axis.
    fn of(body: &Body, samples: &Samples, edge: EdgeId, wall: FaceId) -> Option<Rim> {
        let Surface::Cylinder(cylinder) = body.surface(body.face(wall).surface) else {
            return None;
        };
        let Curve::Circle(circle) = body.curve(body.edge(edge).curve) else {
            return None;
        };
        if circle.axis.cross(cylinder.axis).length() > Scale::RELATIVE {
            return None;
        }
        let height = |point: DVec3| (point - cylinder.origin).dot(cylinder.axis);
        let level = height(circle.center);
        let side = (laps(body, samples, wall)?.into_iter().flatten())
            .map(|sample| height(samples.point(sample)) - level)
            .max_by(|one, other| one.abs().total_cmp(&other.abs()))?;
        let mut corners: Vec<DVec2> = (samples.edge(edge).iter())
            .map(|&sample| {
                let from = samples.point(sample) - cylinder.origin;
                DVec2::new(from.dot(cylinder.u), from.dot(cylinder.v))
            })
            .collect();
        if body.edge(edge).ends.is_none() {
            corners.extend(corners.first().copied());
        }
        let nearest = (corners.windows(2))
            .map(|pair| pair[0].perp_dot(pair[1]).abs() / pair[0].distance(pair[1]))
            .fold(f64::INFINITY, f64::min);
        Some(Rim {
            wall: *cylinder,
            level,
            side,
            corners,
            nearest,
        })
    }

    /// Whether `point` stands inside the wall, on the side of the rim its
    /// face runs to, but outside the chord between the rim's samples either
    /// side of it, which the wall's triangles are drawn along.
    fn passed_by(&self, point: DVec3) -> bool {
        let from = point - self.wall.origin;
        if (from.dot(self.wall.axis) - self.level) * self.side <= 0.0 {
            return false;
        }
        let at = DVec2::new(from.dot(self.wall.u), from.dot(self.wall.v));
        let away = at.length();
        if away <= self.nearest || away > self.wall.radius {
            return false;
        }
        self.corners.windows(2).any(|pair| {
            let [one, other] = [pair[0], pair[1]];
            let way = one.perp_dot(other).signum();
            one.perp_dot(at) * way >= 0.0
                && at.perp_dot(other) * way >= 0.0
                && (other - one).perp_dot(at - one) * way < 0.0
        })
    }
}
