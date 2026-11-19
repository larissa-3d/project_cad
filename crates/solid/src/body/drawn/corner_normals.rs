//! The way a body faces at each corner of the triangles it is drawn with, as
//! the light reads it: the triangles meeting there within one face, weighed by
//! the angle each opens at it. A curved face is then lit smooth, and the edge
//! between two faces stays sharp.

use std::collections::HashMap;

use glam::DVec3;

use crate::body::{Body, Matter};
use crate::brep::Scale;

impl Body {
    /// The triangles the body is drawn with, each beside the way the body
    /// faces at each of its corners. A flat face faces one way at all of them.
    pub fn triangles_with_normals(&self) -> Vec<([DVec3; 3], [DVec3; 3])> {
        let faced: Vec<([DVec3; 3], usize)> = match &self.matter {
            Matter::Exact(exact) => {
                let drawn = exact.drawn();
                let faces = drawn.faces.iter().map(|face| face.0 as usize);
                drawn.triangles.iter().copied().zip(faces).collect()
            }
            Matter::Flats(flats) => flats
                .mesh
                .polygons
                .iter()
                .flat_map(|piece| piece.triangles().map(|triangle| (triangle, piece.face)))
                .collect(),
        };
        smoothed(faced)
    }
}

/// Corners closer than the body's [`Scale::eps`] are one: the kernel hands a
/// point of a seam out a few ulps apart from one triangle to the next.
fn smoothed(faced: Vec<([DVec3; 3], usize)>) -> Vec<([DVec3; 3], [DVec3; 3])> {
    let reach = faced
        .iter()
        .flat_map(|(corners, _)| corners)
        .fold(0.0f64, |reach, corner| {
            reach.max(corner.abs().max_element())
        });
    let eps = Scale::of(reach).eps();
    let key = |face: usize, corner: DVec3| {
        let cell = (corner / eps).round();
        (face, cell.x as i64, cell.y as i64, cell.z as i64)
    };

    let mut sums: HashMap<_, DVec3> = HashMap::with_capacity(faced.len() * 3);
    for ([a, b, c], face) in &faced {
        let normal = (*b - *a).cross(*c - *a).normalize_or_zero();
        for (corner, one, other) in [(a, b, c), (b, c, a), (c, a, b)] {
            let (along, across) = (*one - *corner, *other - *corner);
            let angle = along.cross(across).length().atan2(along.dot(across));
            *sums.entry(key(*face, *corner)).or_default() += normal * angle;
        }
    }

    faced
        .into_iter()
        .map(|(corners, face)| {
            let normals = corners.map(|corner| sums[&key(face, corner)].normalize_or_zero());
            (corners, normals)
        })
        .collect()
}

#[cfg(test)]
mod tests;
