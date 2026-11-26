//! Where a circle lying on a sphere about another line crosses the steps of
//! that sphere's grid (#940): the angles a face of the sphere going round its
//! axis between two such circles is cut open at.

use glam::DVec3;

use super::Axes;
use crate::brep::Scale;
use crate::brep::curve::Circle;

impl Axes {
    /// Where `circle` crosses the steps of the grid of each sphere carrying
    /// it about another line.
    pub(super) fn across_spheres<'a>(
        &'a self,
        circle: &'a Circle,
        eps: f64,
    ) -> impl Iterator<Item = DVec3> + 'a {
        (self.steps.iter())
            .filter(move |(round, _)| round.carries(circle, eps) && !round.holds(circle, eps))
            .flat_map(move |(round, steps)| {
                let line = (round.origin(), round.axis());
                (steps.iter()).flat_map(move |way| crossings(circle, line, *way))
            })
    }
}

/// The directions from the centre of `circle` to where it crosses the
/// half-plane from the line through `origin` along `axis` out along `way`,
/// square to it: none where it passes it by or only touches it, reaching
/// past the half-plane's plane by no more than the kernel's relative
/// tolerance of its radius. Read with rounding, a touch would cross it
/// twice, the square root of a rounding apart.
pub(super) fn crossings(circle: &Circle, (origin, axis): (DVec3, DVec3), way: DVec3) -> Vec<DVec3> {
    let across = axis.cross(way);
    let (along_u, along_v) = (circle.u.dot(across), circle.v.dot(across));
    let off = (circle.center - origin).dot(across) / circle.radius;
    let reach = along_u.hypot(along_v);
    if reach - off.abs() <= Scale::RELATIVE {
        return Vec::new();
    }
    let (crest, spread) = (along_v.atan2(along_u), (-off / reach).acos());
    [crest - spread, crest + spread]
        .into_iter()
        .map(|at| circle.u * at.cos() + circle.v * at.sin())
        .filter(|ray| (circle.center + *ray * circle.radius - origin).dot(way) > 0.0)
        .collect()
}
