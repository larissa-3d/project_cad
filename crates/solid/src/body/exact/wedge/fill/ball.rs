//! What a fill is trimmed by at a sphere end (#805): the box standing past
//! where the prism's long edges leave the sphere, less the piece of the ball
//! the box takes in, so that the fill ends flush on it. The piece is turned
//! about the sphere's own axis, so that the fill's end lies on the very
//! surface of the face it merges into, and bounded round the box (#941): a
//! result carries the larger reach of its operands, and the whole ball of a
//! dome far larger than its part would draw every later step coarser. The
//! piece no longer carries the sphere's size, but it is not held inside the
//! box: a sector's arc bulges past it between its planes, a whole turn's
//! disc stands all round the axis, each by no more than about the box's
//! furthest distance from the axis. That reach is the piece's own, read off
//! its corners, edges and bulges; the bound its profile is laid at — its
//! corners' place plus their furthest from the axis — sets only the
//! tolerances it is laid and read to, never the reach it carries (#977). A
//! large fill near its part's edge is so decided a little coarser than its
//! part, never by the dome's size, and leaves the part its own reach (#1023).

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::Prism;
use super::round::long_edges;
use crate::body::exact::Exact;
use crate::body::exact::wedge::end::beyond;
use crate::brep::{Declined, Plane, Sphere};
use crate::profile::{Contour, Frame, Run};
use crate::turning::{Axis, ON_THE_AXIS, Straight, Turn};

/// The solid holding what of the prism stands outside `sphere` past where
/// its long edges, running on `past`, leave it: the box [`beyond`] raises
/// past the plane square to `past` where every long edge that meets the
/// sphere stands inside it — clear short of where the first leaves it, or
/// the plane through the centre where that is nearer — less the piece of
/// the ball [`within`] the box. An edge that misses the sphere is cut where
/// the box starts, as a cylinder's rectangle cuts it. Declined
/// `Unsupported` where no edge meets the sphere and where one leaves it
/// past the prism's end; as [`beyond`] and [`within`] decline; a box the
/// kernel cannot cut, as the cut says.
pub(super) fn outside_sphere(
    prism: &Prism,
    sphere: &Sphere,
    past: DVec3,
    reach: f64,
) -> Result<Exact, Declined> {
    let towards = past.normalize();
    let middle = sphere.middle();
    let (edges, clear) = long_edges(prism, past);
    let mut leaving = f64::INFINITY;
    for [_, far] in &edges {
        let from = *far - middle;
        let along = from.dot(towards);
        let half = (sphere.radius.powi(2) - (from - towards * along).length_squared()).sqrt();
        if half > 0.0 {
            if half >= along {
                return Err(Declined::Unsupported);
            }
            leaving = leaving.min(half);
        }
    }
    if !leaving.is_finite() {
        return Err(Declined::Unsupported);
    }
    let (start, _) = Plane::through(middle + towards * (leaving - clear).max(0.0), towards);
    let around = beyond(prism, &start, towards)?;
    let corners: Vec<DVec3> = (around.brep().vertex_ids())
        .map(|vertex| around.brep().vertex(vertex).point)
        .collect();
    around.cut_by(&within(sphere, &corners, clear / 4.0, reach)?)
}

/// The piece of the ball of `sphere` round `held`, turned about the
/// sphere's own axis: between the levels along the axis `held` spans and
/// out to its furthest from the axis, each `snug` past them, and either
/// round the turn `held` spans about the axis, between planes through it
/// turned `snug` clear of them, from a cylinder `snug` inside the nearest
/// of the segments between them — or, where `held` stands round the axis
/// over half a turn or more or that cylinder would stand on it, a whole
/// turn from the axis out. Every point of the ball within the hull of
/// `held` lies inside the piece. Declined `Unsupported` where the piece
/// misses the ball or the kernel cannot lay it; a piece it cannot turn, as
/// the turn says.
fn within(sphere: &Sphere, held: &[DVec3], snug: f64, reach: f64) -> Result<Exact, Declined> {
    let axis = sphere.axis;
    let (mut low, mut high, mut furthest) = (f64::INFINITY, f64::NEG_INFINITY, DVec3::ZERO);
    let mut flats = Vec::with_capacity(held.len());
    for &at in held {
        let from = at - sphere.origin;
        let h = from.dot(axis);
        let flat = from - axis * h;
        (low, high) = (low.min(h), high.max(h));
        if flat.length() > furthest.length() {
            furthest = flat;
        }
        flats.push(flat);
    }
    let outer = furthest.length() + snug;
    let towards = furthest.normalize_or_zero();
    let aside = axis.cross(towards);
    let (mut first, mut last) = (f64::INFINITY, f64::NEG_INFINITY);
    for flat in &flats {
        let turn = flat.dot(aside).atan2(flat.dot(towards));
        let spread = (snug / flat.length()).min(1.0).asin();
        first = first.min(turn - spread);
        last = last.max(turn + spread);
    }
    let inner = nearest(&flats) - snug;
    let band = ON_THE_AXIS * outer;
    let partial = last - first < PI && inner > band;
    let (inner, angle, v) = if partial {
        let (sin, cos) = first.sin_cos();
        (inner, last - first, towards * cos + aside * sin)
    } else {
        (0.0, TAU, sphere.u)
    };
    let section = in_the_disc(
        [low - snug, high + snug],
        [inner, outer],
        DVec2::new(sphere.center, 0.0),
        sphere.radius,
    )
    .ok_or(Declined::Unsupported)?;
    let frame = Frame {
        origin: sphere.origin,
        u: axis,
        v,
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::X,
        },
        angle,
        resolution: 0.0,
        on_the_axis: band,
    };
    let straight = Straight::of(&section, &[], frame, &turn, reach).ok_or(Declined::Unsupported)?;
    Exact::turned(&straight, frame, &turn, straight.numbers(!partial))
}

/// How near the origin the segments between any two of `flats` come: how
/// near it their hull does, where the origin lies outside it.
fn nearest(flats: &[DVec3]) -> f64 {
    let mut nearest = f64::INFINITY;
    for (rank, &one) in flats.iter().enumerate() {
        for &other in &flats[rank..] {
            let run = other - one;
            let at = if run == DVec3::ZERO {
                0.0
            } else {
                (-one.dot(run) / run.length_squared()).clamp(0.0, 1.0)
            };
            nearest = nearest.min((one + run * at).length());
        }
    }
    nearest
}

/// The rectangle between `heights` along the axis and `radii` from it, read
/// in `(h, r)`, within the disc of `radius` about `center`: anticlockwise,
/// each side of the rectangle where it runs inside the disc, and the circle
/// from where one leaves it to where the next comes back. None where the
/// rectangle's rim misses the disc.
fn in_the_disc(
    [low, high]: [f64; 2],
    [inner, outer]: [f64; 2],
    center: DVec2,
    radius: f64,
) -> Option<Contour> {
    let rim = [
        DVec2::new(low, inner),
        DVec2::new(high, inner),
        DVec2::new(high, outer),
        DVec2::new(low, outer),
    ];
    let mut inside = Vec::new();
    for (rank, &from) in rim.iter().enumerate() {
        let to = rim[(rank + 1) % rim.len()];
        let (run, off) = (to - from, from - center);
        let (square, half) = (run.length_squared(), off.dot(run));
        let room = half * half - square * (off.length_squared() - radius * radius);
        if room <= 0.0 {
            continue;
        }
        let [enter, leave] = [-1.0, 1.0].map(|sign| (sign * room.sqrt() - half) / square);
        let at = |t: f64| {
            if t <= 0.0 {
                from
            } else if t >= 1.0 {
                to
            } else {
                from + run * t
            }
        };
        if enter < 1.0 && leave > 0.0 {
            inside.push((at(enter), at(leave)));
        }
    }
    if inside.is_empty() {
        return None;
    }
    let (mut corners, mut runs) = (Vec::new(), Vec::new());
    for (rank, &(enter, leave)) in inside.iter().enumerate() {
        corners.push(enter);
        runs.push(Run::Straight);
        let next = inside[(rank + 1) % inside.len()].0;
        if leave != next {
            corners.push(leave);
            let turn = ((next - center).to_angle() - (leave - center).to_angle()).rem_euclid(TAU);
            runs.push(Run::Round { center, turn });
        }
    }
    Some(Contour { corners, runs })
}

#[cfg(test)]
mod tests;
