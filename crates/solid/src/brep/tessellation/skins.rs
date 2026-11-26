//! A skin of no thickness: two faces folded onto each other whole, drawn on
//! the same three samples facing opposite ways ([`super::folds`]). Neither is
//! drawn, and every side of the two is still met once each way by what is
//! left, so the surface stays closed.
//!
//! So too a ring thinner than the drawing's tolerance between a sphere and a
//! wall about an axis through its centre, the two faces bounded by the same
//! circles ([`rings`]): a ball less a shaft a few tens of the kernel's
//! tolerances under its radius, past the hair decision 12 lays onto the
//! touch, its end inside the band the shaft's wall meets the sphere across
//! (#1137). Drawn, the sphere's face between them is cut at the rows of its
//! grid, its triangles fanned from points of a row onto samples of its rims
//! the row has not, the wall's grid's among them, and they lean off the
//! sphere by as far as a chord sags, a thousand times as far as the ring is
//! thick: they cross the wall, and the folds they leave are never undone.

use std::collections::{BTreeMap, BTreeSet};

use super::contact::lies_on;
use crate::brep::curve::Curve;
use crate::brep::surface::{Cylinder, Sphere, Surface};
use crate::brep::topology::{Body, EdgeId, FaceId};

/// The faces of every ring between a face of a sphere and a face of a
/// cylinder bounded by the same edges, each a circle of the cylinder,
/// the sphere standing within `tolerance` of the cylinder all the way
/// between them: the drawing cannot tell it from no ring at all, and draws
/// neither face.
pub(super) fn rings(body: &Body, tolerance: f64) -> BTreeSet<FaceId> {
    let bounds = |face: FaceId| {
        let mut edges: Vec<EdgeId> = (body.face(face).loops.iter().flatten())
            .map(|coedge| coedge.edge)
            .collect();
        edges.sort_unstable();
        edges
    };
    let walls: Vec<(FaceId, &Cylinder, Vec<EdgeId>)> = body
        .face_ids()
        .filter_map(|wall| {
            let Surface::Cylinder(cylinder) = body.surface(body.face(wall).surface) else {
                return None;
            };
            Some((wall, cylinder, bounds(wall)))
        })
        .collect();
    let mut rings = BTreeSet::new();
    for ball in body.face_ids() {
        let Surface::Sphere(sphere) = body.surface(body.face(ball).surface) else {
            continue;
        };
        let edges = bounds(ball);
        for (wall, cylinder, theirs) in &walls {
            if *theirs == edges && thinner(body, sphere, cylinder, &edges, tolerance) {
                rings.extend([ball, *wall]);
            }
        }
    }
    rings
}

/// Whether every one of `edges` is a circle of `cylinder`, and `sphere`
/// stands within `tolerance` of it from the lowest of them along its axis to
/// the highest, the two apart: every such circle lying on the sphere too,
/// the axis passes through the sphere's centre.
fn thinner(
    body: &Body,
    sphere: &Sphere,
    cylinder: &Cylinder,
    edges: &[EdgeId],
    tolerance: f64,
) -> bool {
    let mut heights = Vec::new();
    for edge in edges {
        let Curve::Circle(circle) = body.curve(body.edge(*edge).curve) else {
            return false;
        };
        if !lies_on(circle, cylinder, body.scale().eps()) {
            return false;
        }
        heights.push((circle.center - sphere.middle()).dot(cylinder.axis));
    }
    let low = heights.iter().copied().fold(f64::INFINITY, f64::min);
    let high = heights.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let across = |height: f64| {
        (sphere.radius * sphere.radius - height * height)
            .max(0.0)
            .sqrt()
    };
    let widest = if low <= 0.0 && high >= 0.0 {
        sphere.radius
    } else {
        across(low).max(across(high))
    };
    low < high
        && [widest, across(low), across(high)]
            .iter()
            .all(|radius| (radius - cylinder.radius).abs() <= tolerance)
}

/// Leaves out every two triangles of different faces standing on the same
/// three samples and facing opposite ways, each corner a sample.
pub(super) fn cancelled(oriented: &mut Vec<(FaceId, [usize; 3])>) {
    let mut seen: BTreeMap<[usize; 3], Vec<usize>> = BTreeMap::new();
    for (index, (_, corners)) in oriented.iter().enumerate() {
        let mut key = *corners;
        key.sort_unstable();
        seen.entry(key).or_default().push(index);
    }
    let even = |corners: [usize; 3]| {
        let turns = (0..3)
            .filter(|at| corners[*at] < corners[(at + 1) % 3])
            .count();
        turns == 2
    };
    let mut dropped = BTreeSet::new();
    for indices in seen.values() {
        for (place, &one) in indices.iter().enumerate() {
            if dropped.contains(&one) {
                continue;
            }
            let other = indices[place + 1..].iter().copied().find(|other| {
                !dropped.contains(other)
                    && oriented[*other].0 != oriented[one].0
                    && even(oriented[*other].1) != even(oriented[one].1)
            });
            if let Some(other) = other {
                dropped.insert(one);
                dropped.insert(other);
            }
        }
    }
    let mut index = 0;
    oriented.retain(|_| {
        index += 1;
        !dropped.contains(&(index - 1))
    });
}

#[cfg(test)]
mod tests;
