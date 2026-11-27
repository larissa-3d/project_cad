//! Where a circle lying on a sphere about another line crosses the steps of
//! that sphere's grid (#940): the angles a face of the sphere going round its
//! axis between two such circles is cut open at. None on a sphere whose
//! faces want none of them ([`opened`]).

use glam::DVec3;

use super::super::cone::read_about;
use super::super::round::Rounded;
use super::Axes;
use crate::brep::Scale;
use crate::brep::curve::{Circle, Curve};
use crate::brep::surface::Surface;
use crate::brep::tessellation::contact::lies_on;
use crate::brep::topology::{Body, Face, SurfaceId};

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
/// it cross the steps of its grid at. Not where every such circle lies on a
/// wall of the body the grid of the sphere read about the circle's line is
/// lined up with ([`read_about`]), and bounds no face of the sphere but
/// faces every edge of which is a circle of one line: those are cut in
/// strips about that line (`Outline::about`) and cut open on its grid, which
/// every rim of such a wall takes with the wall's. Taken, the places stood
/// between the columns of the strips, and the wall's triangles ran from them
/// to its other rim's, a part of a step round: a ball turned at a slant less
/// a shaft a little under its radius had the strip from a rim to the next
/// row cross the wall's chords over a ring thinner than they sag (#1144). A
/// rim far off the sphere — a shaft of 4.9 through a ball of 5 — keeps them,
/// and so does a circle on no wall of the body, the ball's circle on the end
/// plane of a shaft laid onto its touch, whose wall's other rim takes them
/// too (#1109).
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
        circles.is_some_and(|circles| {
            (circles.iter())
                .all(|circle| circle.axis.cross(circles[0].axis).length() <= Scale::RELATIVE)
        })
    };
    let lined_up = |circle: &Circle| {
        let (_, walls) = read_about(body, sphere, circle.axis, rows, fineness);
        walls.into_iter().any(|wall| match body.surface(wall) {
            Surface::Cylinder(cylinder) => lies_on(circle, cylinder, eps),
            _ => false,
        })
    };
    body.edge_ids().any(|edge| {
        let Curve::Circle(circle) = body.curve(body.edge(edge).curve) else {
            return false;
        };
        let mut faces = (body.uses(edge).into_iter())
            .map(|(id, _)| body.face(id))
            .filter(|face| face.surface == surface);
        round.carries(circle, eps)
            && !round.holds(circle, eps)
            && !(faces.all(one_line) && lined_up(circle))
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
