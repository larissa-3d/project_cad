//! A face of a sphere beside a knife-edge rim (#1138): a circle along which
//! it meets a wall about the circle's own axis at a shallow angle, the two
//! running from the circle away from the sphere's equator — a shaft of 4.9
//! less a ball of 5, their surfaces cos⁻¹ 0.98 apart. Beside such a rim the
//! sphere stands inside the wall but within the sag of the chords between
//! the rim's samples the wall is drawn along, over a band reaching the
//! latitude where it comes as near the axis as those chords — the sag over
//! the sine of the angle between the two where that angle is wide, and
//! bounded as the wall's radius nears the sphere's, where it is some three
//! quarters of a row of the grid — and a point of the face there between
//! two of the rim's rays stands outside the wall's triangles: the face's
//! cross them.
//!
//! The central chart's straight lines are the sphere's great circles, and
//! the sweep joins the rim's samples along them. The rim being all but a
//! great circle, they hug it, and the middles the refinement lifts onto the
//! sphere fall in the band: a face so laid is left to the charts after it
//! ([`within_the_walls`]). The strips about the wall's axis put no row in
//! the band, and one at its edge for a row of the grid that fell in it
//! ([`Band`]). The strips of the sphere's own chart need neither: the rim,
//! one of its latitudes, takes its grid as well as the wall's, and each
//! lattice point beside it stands on the ray of one of the rim's samples.
//! Met at a wide angle, the band is narrower than the central chart's lines
//! bow away from the rim, and no point falls in it.
//!
//! A wall can run from a knife-edge rim beside the face instead, towards the
//! equator, and end there a hair inside the round: a half ball less a shaft
//! a hair under its radius up to its centre, the rim a hair under the
//! equator (#1127). The strips leave out a row within a hundredth of a step
//! of the rim, and the strip from the rim to the row past it leans inside
//! the wall and through the face closing it, which stands at the equator a
//! hair under the sphere; the sliver kept beside the rim stands outside the
//! wall's chords, whose meridian is straight. Where the wall ends on such a
//! row, the strips cut the face there ([`ends`]).

use glam::{DVec2, DVec3};

use super::super::sampling::{Rounded, Samples, midway};
use super::laps;
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Sphere, Surface};
use crate::brep::topology::{Body, EdgeId, Face, FaceId};

/// Each rim of `face`, a latitude of `round`, along which a wall runs beside
/// it — a face of a cylinder, the two faces' matter between them a wedge,
/// their outward normals there opposed — with each circle the wall ends at
/// off the round, as their angles along the meridian.
pub(super) fn ends(body: &Body, face: &Face, round: &Rounded, eps: f64) -> Vec<[f64; 2]> {
    let outward = |face: &Face, at: DVec3| {
        let surface = body.surface(face.surface);
        let normal = surface.normal(surface.parameters(at));
        if face.flipped { -normal } else { normal }
    };
    let circle = |edge: EdgeId| match body.curve(body.edge(edge).curve) {
        Curve::Circle(circle) => Some(*circle),
        Curve::Line(_) | Curve::Meet(_) | Curve::Conic(_) => None,
    };
    let mut ends = Vec::new();
    for coedge in face.loops.iter().flatten() {
        let Some(rim) = circle(coedge.edge).filter(|rim| round.holds(rim, eps)) else {
            continue;
        };
        let at = rim.point(midway(body.edge(coedge.edge)));
        for (id, _) in body.uses(coedge.edge) {
            let wall = body.face(id);
            if !matches!(body.surface(wall.surface), Surface::Cylinder(_))
                || outward(wall, at).dot(outward(face, at)) >= 0.0
            {
                continue;
            }
            let off = (wall.loops.iter().flatten())
                .filter_map(|other| circle(other.edge))
                .filter(|end| !round.holds(end, eps));
            ends.extend(off.map(|end| [round.latitude(&rim), round.latitude(&end)]));
        }
    }
    ends
}

/// Whether every sample from `from` on among `samples_of`, the points added
/// inside the face `id` of a sphere, stands within the chords of each wall
/// the face meets along a circle about the wall's axis, wherever it stands
/// inside that wall on the side of the circle the wall's face runs to.
pub(super) fn within_the_walls(
    body: &Body,
    samples: &Samples,
    id: FaceId,
    (samples_of, from): (&[usize], usize),
) -> bool {
    let added: Vec<DVec3> = (samples_of.iter())
        .filter(|&&sample| sample >= from)
        .map(|&sample| samples.point(sample))
        .collect();
    rims(body, samples, id).all(|rim| !added.iter().any(|&point| rim.passed_by(point)))
}

/// The bands beside the knife-edge rims of the face `id` of `sphere`, laid
/// in strips on `round`, the sphere read about the rims' axis; `laid`, the
/// samples the face is laid out from, tell the side of each rim it lies on.
pub(super) fn bands(
    body: &Body,
    samples: &Samples,
    id: FaceId,
    (sphere, round): (&Sphere, &Rounded),
    laid: &[usize],
) -> Vec<Band> {
    (rims(body, samples, id))
        .filter_map(|rim| {
            let over = rim.level - rim.height(sphere.middle());
            let face = (laid.iter())
                .map(|&sample| rim.height(samples.point(sample)) - rim.level)
                .max_by(|one, other| one.abs().total_cmp(&other.abs()))?;
            if rim.side * over <= 0.0 || rim.side * face <= 0.0 {
                return None;
            }
            let at = round.latitude(&rim.circle);
            let edge = (rim.nearest / sphere.radius).clamp(-1.0, 1.0).acos();
            Some(Band {
                rim: at,
                edge: at.signum() * edge,
            })
        })
        .collect()
}

/// The band beside a knife-edge rim, as angles along the meridian of the
/// strips about its axis: from the rim to the latitude where the sphere
/// comes as near the axis as the chords between the rim's samples, inside
/// which a point of the sphere between two of the rim's rays stands outside
/// them.
pub(super) struct Band {
    rim: f64,
    edge: f64,
}

impl Band {
    /// The rows `rows` of a face reaching from `low` to `high` but those
    /// inside the band, and, where one was, a row at its edge instead,
    /// unless that stands within `room` of an end of the face or of one of
    /// `levels`. Every strip beside the rim then stays within a step of the
    /// grid, and the drawing within its tolerance; where no row falls in the
    /// band, the face is cut as it was.
    pub(super) fn cleared(
        &self,
        rows: Vec<f64>,
        (low, high): (f64, f64),
        levels: &[f64],
        room: f64,
    ) -> Vec<f64> {
        let count = rows.len();
        let mut kept: Vec<f64> = (rows.into_iter())
            .filter(|&row| (row - self.rim) * (row - self.edge) >= 0.0)
            .collect();
        let clear = self.edge > low + room
            && self.edge < high - room
            && levels.iter().all(|level| (level - self.edge).abs() > room);
        if kept.len() < count && clear {
            kept.push(self.edge);
        }
        kept
    }
}

/// The rims the face `id` meets a wall along.
fn rims<'a>(body: &'a Body, samples: &'a Samples, id: FaceId) -> impl Iterator<Item = Rim> + 'a {
    (body.face(id).loops.iter().flatten()).flat_map(move |coedge| {
        (body.uses(coedge.edge).into_iter())
            .filter(move |&(wall, _)| wall != id)
            .filter_map(move |(wall, _)| Rim::of(body, samples, coedge.edge, wall))
    })
}

/// A circle a wall's face is bounded by, about the wall's own axis, as its
/// samples draw it.
struct Rim {
    wall: Cylinder,
    circle: Circle,
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
            circle: *circle,
            level,
            side,
            corners,
            nearest,
        })
    }

    /// How high `point` stands along the wall's axis.
    fn height(&self, point: DVec3) -> f64 {
        (point - self.wall.origin).dot(self.wall.axis)
    }

    /// Whether `point` stands inside the wall, on the side of the rim its
    /// face runs to, but outside the chord between the rim's samples either
    /// side of it, which the wall's triangles are drawn along.
    fn passed_by(&self, point: DVec3) -> bool {
        if (self.height(point) - self.level) * self.side <= 0.0 {
            return false;
        }
        let from = point - self.wall.origin;
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

#[cfg(test)]
mod tests;
