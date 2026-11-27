//! A skin of no thickness: two faces folded onto each other whole, drawn on
//! the same three samples facing opposite ways ([`super::folds`]). Neither is
//! drawn, and every side of the two is still met once each way by what is
//! left, so the surface stays closed.
//!
//! So too a ring between a face of a sphere and a face of a cylinder bounded
//! by the same edges, every one a circle of the cylinder, the sphere standing
//! off the cylinder anywhere between them no further than [`THIN`] of the
//! kernel's tolerances, nor than [`SHOWN`] of the tolerance it is drawn at
//! ([`rings`]). A ball less a shaft through its centre leaves one where the
//! shaft stands past the hair decision 12 lays onto the touch and that close
//! under the ball's radius: between the two circles its wall meets the sphere
//! along, or between one of them and the shaft's end plane, where that plane
//! stands inside the band so near the circle that the circle it cuts from the
//! sphere and the shaft's rim are one (#1137). Neither face is drawn: the
//! disc of the end plane and the ball's cap beyond it close against each
//! other along the rim, and of a ball the shaft goes through, no face is left
//! to draw, only the two circles as its edges. Drawn, the sphere's face was
//! cut at a row of its grid inside the band, its triangles fanned from points
//! of the row onto samples of its rims the row has not, the wall's grid's
//! among them, and leaned off the sphere by as far as a chord sags, far
//! further than the ring is thick: they crossed the wall, and the folds they
//! left took tens of seconds to go through, most of them never undone. A
//! thicker ring is drawn as before.

use std::collections::{BTreeMap, BTreeSet};

use super::contact::lies_on;
use crate::brep::Scale;
use crate::brep::curve::Curve;
use crate::brep::surface::{Cylinder, Sphere, Surface};
use crate::brep::topology::{Body, EdgeId, FaceId};

/// How thick, in the kernel's tolerances, a ring between a sphere and a
/// cylinder may stand and not be drawn: `Scale::HAIR` hairs. The first
/// rings past decision 12's hair are a few tens of tolerances thick — the
/// issue's, forty. The tolerances are the body's scale's, as the kernel
/// decides with them, which keeps the reach of the widest tool that ever cut
/// it (#824): a shaft standing out twenty times as far as the ball it bores
/// brings a tolerance twenty times as coarse, and forty of those under the
/// ball's radius leaves a ring as far past a hair as the issue's. Further
/// out still, four hundred of them stand past what the view draws — twenty
/// times its tolerance for a shaft ten thousand times as far — and
/// [`SHOWN`] keeps such a ring drawn.
const THIN: f64 = Scale::HAIR * Scale::HAIR;

/// How much of the tolerance a body is drawn at a ring may stand off its
/// wall and not be drawn: a hundredth. Such a ring was drawn crossed up to
/// about a third of the tolerance, and apart past it, until #1144 drew it
/// uncrossed from that hundredth on; a hundredth keeps the bound well under
/// what any drawing can show, whatever reach the tolerance asked was taken
/// at — an export's gap is the user's — and the issue's ring still under it
/// by twenty-five times at the view's tolerance. The bound is
/// the smaller of this and [`THIN`]: a coarse drawing still draws, as
/// before, every ring thicker than four hundred of the kernel's tolerances,
/// and a ball of 5 bored by a shaft standing out ten thousand times as far,
/// whose tolerances are ten thousand times as coarse, every ring a hundredth
/// of the view's tolerance thick or more.
const SHOWN: f64 = 1e-2;

/// The faces of every ring between a face of a sphere and a face of a
/// cylinder bounded by the same edges, each a circle of the cylinder, the
/// sphere standing within [`THIN`] of the kernel's tolerances, and within
/// [`SHOWN`] of `tolerance`, of the cylinder all the way between them:
/// neither face is drawn.
pub(super) fn rings(body: &Body, tolerance: f64) -> BTreeSet<FaceId> {
    let thin = (THIN * body.scale().eps()).min(SHOWN * tolerance);
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
            if *theirs == edges && thinner(body, sphere, cylinder, &edges, thin) {
                rings.extend([ball, *wall]);
            }
        }
    }
    rings
}

/// Whether every one of `edges` is a circle of `cylinder`, and `sphere`
/// stands within `thin` of it from the lowest of them along its axis to
/// the highest, the two apart: every such circle lying on the sphere too,
/// the axis passes through the sphere's centre.
fn thinner(body: &Body, sphere: &Sphere, cylinder: &Cylinder, edges: &[EdgeId], thin: f64) -> bool {
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
            .all(|radius| (radius - cylinder.radius).abs() <= thin)
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
