//! A sphere read about another axis (#938). A sphere's chart is the axis it
//! was turned about, an artefact of how it was made: the sphere is round
//! about every line through its centre. A plane of the other operand at a
//! slant through a pole of that chart cuts it along a circle where `θ` has
//! no value, which no trace reads where an arc of it is kept; a boolean so
//! declined is asked again of the operands with the sphere read about the
//! circle's own axis, where that circle is a latitude, and the result
//! carries it so. Only a boolean declined is asked again: a plane through a
//! pole whose arcs are all left out, a wedge beside a ball's top, declined
//! nothing, and the ball keeps the axis a later bore along it turns about.
//!
//! The operand is laid again whole on the sphere so read, each face holding
//! the poles of the new chart it holds. A sphere is read so only where every
//! circle on its faces, and every circle a plane of the other operand cuts
//! it along, stands clear of the new poles, and no surface but a plane, in
//! either operand, meets it along curves its axis decides; anywhere else it
//! is left as it is, and the step stays declined.

use crate::brep::assembly::tidied;
use crate::brep::curve::{Circle, Curve};
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Sphere, Surface};
use crate::brep::topology::{Body, FaceId, SurfaceId};

/// Each operand with every sphere the other's planes cut through a pole of
/// its chart read about the circle they cut it along, where it may be; none
/// for an operand left as it is.
pub(super) fn charted(first: &Body, second: &Body, scale: Scale) -> [Option<Body>; 2] {
    [(first, second), (second, first)].map(|(body, other)| recharted(body, other, scale))
}

fn recharted(body: &Body, other: &Body, scale: Scale) -> Option<Body> {
    let read: Vec<(usize, Sphere)> = (body.surfaces.iter().enumerate())
        .filter_map(|(rank, surface)| match surface {
            Surface::Sphere(sphere) => {
                about(body, SurfaceId(rank as u32), sphere, other, scale).map(|about| (rank, about))
            }
            Surface::Plane(_)
            | Surface::Cylinder(_)
            | Surface::Cone(_)
            | Surface::Elliptic(_)
            | Surface::Torus(_) => None,
        })
        .collect();
    if read.is_empty() {
        return None;
    }
    let mut turned = body.clone();
    for (rank, about) in read {
        turned.surfaces[rank] = Surface::Sphere(about);
    }
    let faces = turned.faces.clone();
    tidied(turned, faces, &[])
        .ok()
        .map(|laid| laid.noted(body.unstable))
}

/// `sphere`, the surface `rank` of `body`, read about the axis of a circle a
/// plane of `other` whose face may meet one of `body` on it cuts it along
/// through a pole of its chart, as the module says; none where it is to be
/// left as it is.
fn about(
    body: &Body,
    rank: SurfaceId,
    sphere: &Sphere,
    other: &Body,
    scale: Scale,
) -> Option<Sphere> {
    let eps = scale.eps();
    let own = Surface::Sphere(*sphere);
    let lying: Vec<FaceId> = body
        .face_ids()
        .filter(|&face| body.face(face).surface == rank)
        .collect();
    let planes = || {
        (other.surfaces.iter().enumerate())
            .filter(|(_, surface)| matches!(surface, Surface::Plane(_)))
            .map(|(at, plane)| (SurfaceId(at as u32), relation(&own, plane, scale)))
    };
    let through = planes().find_map(|(plane, found)| match found {
        Relation::Circle(circle)
            if sphere.through_a_pole(&circle, eps) && facing(body, &lying, other, plane, eps) =>
        {
            Some(circle)
        }
        _ => None,
    })?;
    let besides = (body.surfaces.iter().enumerate())
        .filter(|&(at, _)| at != rank.0 as usize)
        .map(|(_, surface)| surface);
    let ruled_by_the_axis = (other.surfaces.iter().chain(besides)).any(|surface| {
        !matches!(surface, Surface::Plane(_))
            && relation(&own, surface, scale) != Relation::Unsupported
    });
    if ruled_by_the_axis {
        return None;
    }
    let cut = planes().flat_map(|(_, found)| found.curves());
    let edged = lying
        .iter()
        .flat_map(|&face| body.face(face).loops.iter().flatten())
        .map(|coedge| *body.curve(body.edge(coedge.edge).curve));
    let about = sphere.about(through.axis);
    circles(cut.chain(edged).collect())?
        .iter()
        .all(|circle| !about.through_a_pole(circle, eps))
        .then_some(about)
}

/// The circles among `curves`; none where one is no circle.
fn circles(curves: Vec<Curve>) -> Option<Vec<Circle>> {
    curves
        .into_iter()
        .map(|curve| match curve {
            Curve::Circle(circle) => Some(circle),
            Curve::Line(_) | Curve::Meet(_) | Curve::Conic(_) => None,
        })
        .collect()
}

/// Whether a face of `other` on `plane` stands in a box meeting that of one
/// of the faces `lying` of `body`.
fn facing(body: &Body, lying: &[FaceId], other: &Body, plane: SurfaceId, eps: f64) -> bool {
    let boxes: Vec<_> = lying.iter().map(|&face| body.boxed(face, eps)).collect();
    other
        .face_ids()
        .filter(|&face| other.face(face).surface == plane)
        .map(|face| other.boxed(face, eps))
        .any(|[low, high]| {
            boxes
                .iter()
                .any(|[from, to]| low.cmple(*to).all() && from.cmple(high).all())
        })
}
