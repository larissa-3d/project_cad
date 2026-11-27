//! Where a circle lying on a sphere about another line crosses the steps of
//! that sphere's grid (#940): the angles a face of the sphere going round its
//! axis between two such circles is cut open at. None on a sphere whose
//! faces want none of them ([`opened`]).

use glam::DVec3;

use super::super::super::skins::rings;
use super::super::cone::{About, read_about};
use super::super::round::Rounded;
use super::Axes;
use crate::brep::Scale;
use crate::brep::curve::{Circle, Curve};
use crate::brep::topology::{Body, Face, FaceId, SurfaceId};

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

/// Whether a face of `round`, the surface `surface` gridded with `rows` rows
/// a turn at `fineness`, may want the places the circles of another line on
/// it cross the steps of its grid at. Not where every such circle bounds no
/// face of the sphere but faces every edge of which is a circle of one line,
/// and one of them a face standing beside a ring, drawn
/// ([`About::beside_a_ring`]): cut in strips about the line on a grid lined
/// up with the ring's wall, which every rim of the wall takes. Taken, the
/// places stood between the columns of the strips, and the wall's triangles
/// ran from them to its other rim's, a part of a step round: a ball turned
/// at a slant less a shaft a little under its radius had the strip from a
/// rim to the next row cross the wall's chords over a ring thinner than they
/// sag (#1144). Every other sphere keeps them, as before: a shaft of 4.9
/// through a ball of 5, the ball beside a shaft laid onto its touch (#1109),
/// a ring too thin to be drawn beside the cap it leaves.
pub(super) fn opened(
    body: &Body,
    surface: SurfaceId,
    round: &Rounded,
    rows: usize,
    fineness: (f64, f64),
) -> bool {
    let Rounded::Sphere(sphere) = round else {
        return true;
    };
    let eps = body.scale().eps();
    let one_line = |face: &Face| {
        let circles: Option<Vec<&Circle>> = (face.loops.iter().flatten())
            .map(|coedge| match body.curve(body.edge(coedge.edge).curve) {
                Curve::Circle(circle) => Some(circle),
                Curve::Line(_) | Curve::Meet(_) | Curve::Conic(_) => None,
            })
            .collect();
        let axis = circles.as_ref()?.first()?.axis;
        (circles?.iter())
            .all(|circle| circle.axis.cross(axis).length() <= Scale::RELATIVE)
            .then_some(axis)
    };
    let undrawn = rings(body, fineness.0);
    let mut grids: Vec<(DVec3, About)> = Vec::new();
    let mut beside: Vec<FaceId> = Vec::new();
    for id in body
        .face_ids()
        .filter(|id| body.face(*id).surface == surface)
    {
        let Some(axis) = one_line(body.face(id)) else {
            continue;
        };
        if undrawn.contains(&id) || axis.cross(sphere.axis).length() <= Scale::RELATIVE {
            continue;
        }
        let at = match (grids.iter()).position(|(of, _)| of.cross(axis).length() <= Scale::RELATIVE)
        {
            Some(at) => at,
            None => {
                grids.push((axis, read_about(body, sphere, axis, rows, fineness)));
                grids.len() - 1
            }
        };
        if grids[at].1.beside_a_ring(body, id, sphere, axis) {
            beside.push(id);
        }
    }
    beside.is_empty()
        || body.edge_ids().any(|edge| {
            let Curve::Circle(circle) = body.curve(body.edge(edge).curve) else {
                return false;
            };
            let faces: Vec<FaceId> = (body.uses(edge).into_iter())
                .map(|(id, _)| id)
                .filter(|id| body.face(*id).surface == surface)
                .collect();
            round.carries(circle, eps)
                && !round.holds(circle, eps)
                && !(faces.iter().all(|id| one_line(body.face(*id)).is_some())
                    && faces.iter().any(|id| beside.contains(id)))
        })
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
