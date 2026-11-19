//! The way a body faces at each corner of the triangles it is drawn with, as
//! the light reads it: the triangles meeting there within one face, weighed by
//! the angle each opens at it. A curved face is then lit smooth, and the edge
//! between two faces stays sharp.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use glam::DVec3;

use crate::body::{Body, Matter};
use crate::brep::Scale;

type Lit = ([DVec3; 3], [DVec3; 3]);

/// The triangles of a body beside the normals at their corners, worked out
/// the first time they are asked for and shared by every copy of the body:
/// they hang on nothing a copy can change, and the view copies the part out
/// of its caches every frame. Whether they were worked out yet says nothing
/// of the body, so any two are alike.
#[derive(Clone, Debug, Default)]
pub(in crate::body) struct CornerNormals(Arc<OnceLock<Vec<Lit>>>);

impl PartialEq for CornerNormals {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Body {
    /// The triangles the body is drawn with, each beside the way the body
    /// faces at each of its corners. A flat face faces one way at all of them.
    pub fn triangles_with_normals(&self) -> Vec<([DVec3; 3], [DVec3; 3])> {
        let lit = match &self.matter {
            Matter::Exact(exact) => {
                let drawn = exact.drawn();
                drawn.normals.0.get_or_init(|| {
                    let faces = drawn.faces.iter().map(|face| face.0 as usize);
                    smoothed(drawn.triangles.iter().copied().zip(faces).collect())
                })
            }
            Matter::Flats(flats) => flats.normals.0.get_or_init(|| {
                smoothed(
                    flats
                        .mesh
                        .polygons
                        .iter()
                        .flat_map(|piece| piece.triangles().map(|triangle| (triangle, piece.face)))
                        .collect(),
                )
            }),
        };
        lit.clone()
    }
}

/// Corners of one face closer than the body's [`Scale::eps`] are one point:
/// a cut across the body works a point of an edge out from either end of it,
/// and hands it out a few ulps apart.
fn smoothed(faced: Vec<([DVec3; 3], usize)>) -> Vec<Lit> {
    let reach = faced
        .iter()
        .flat_map(|(corners, _)| corners)
        .fold(0.0f64, |reach, corner| {
            reach.max(corner.abs().max_element())
        });
    let mut points = Points::new(Scale::of(reach).eps());
    let ids: Vec<[usize; 3]> = faced
        .iter()
        .map(|(corners, face)| corners.map(|corner| points.one(*face, corner)))
        .collect();

    let mut sums = vec![DVec3::ZERO; points.at.len()];
    for (([a, b, c], _), ids) in faced.iter().zip(&ids) {
        let normal = (*b - *a).cross(*c - *a).normalize_or_zero();
        for ((corner, one, other), id) in [(a, b, c), (b, c, a), (c, a, b)].into_iter().zip(ids) {
            let (along, across) = (*one - *corner, *other - *corner);
            let angle = along.cross(across).length().atan2(along.dot(across));
            sums[*id] += normal * angle;
        }
    }

    faced
        .into_iter()
        .zip(ids)
        .map(|((corners, _), ids)| (corners, ids.map(|id| sums[id].normalize_or_zero())))
        .collect()
}

/// The points of the faces met so far, in cells as wide as `eps`: a point
/// within `eps` of another lies in its cell or in one of the 26 around it.
struct Points {
    eps: f64,
    at: Vec<DVec3>,
    cells: HashMap<(usize, [i64; 3]), Vec<usize>>,
}

impl Points {
    fn new(eps: f64) -> Points {
        Points {
            eps,
            at: Vec::new(),
            cells: HashMap::new(),
        }
    }

    /// The point of `face` a corner is, met before within `eps` or new.
    fn one(&mut self, face: usize, corner: DVec3) -> usize {
        let cell = (corner / self.eps).floor();
        let cell = [cell.x as i64, cell.y as i64, cell.z as i64];
        let around = std::iter::once(13)
            .chain((0..27).filter(|&rank| rank != 13))
            .map(|rank: i64| [rank % 3 - 1, rank / 3 % 3 - 1, rank / 9 - 1]);
        let met = around
            .filter_map(|[x, y, z]| {
                self.cells
                    .get(&(face, [cell[0] + x, cell[1] + y, cell[2] + z]))
            })
            .flatten()
            .copied()
            .find(|&id| self.at[id].distance(corner) <= self.eps);
        met.unwrap_or_else(|| {
            self.at.push(corner);
            self.cells
                .entry((face, cell))
                .or_default()
                .push(self.at.len() - 1);
            self.at.len() - 1
        })
    }
}

#[cfg(test)]
mod tests;
