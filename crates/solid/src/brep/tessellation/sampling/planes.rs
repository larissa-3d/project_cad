//! The planes touching a wall along a line, and the places of the wall that
//! stand on such a plane: a sample there would lay a strip of the wall on
//! the plane's face. And the planes touching a sphere at a point, on which
//! the samples inside its faces standing within the tolerance of them are
//! laid.

use glam::DVec3;

use super::Samples;
use crate::brep::domain::Location;
use crate::brep::scale::Scale;
use crate::brep::surface::{Plane, Sphere, Surface};
use crate::brep::tessellation::contact;
use crate::brep::topology::{Body, SurfaceId};

/// Whether `point`, on a wall about `center` and `axis` that `plane` touches
/// along a line, stands within a fifth of `eps` of the plane but not on
/// that line: a sample there would lay a strip of the wall on the plane.
pub(in crate::brep::tessellation) fn on_a_plane(
    plane: &Plane,
    center: DVec3,
    axis: DVec3,
    point: DVec3,
    eps: f64,
) -> bool {
    let foot = center - plane.normal * plane.distance(center);
    let touching = foot + axis * axis.dot(point - foot);
    plane.distance(point).abs() < eps * contact::APART && (point - touching).length() > eps
}

/// The planes of the body touching a wall of `radius` about `center` and
/// `axis` along a line the body holds: a vertex lies on both, and on a face
/// of the plane. A plane whose faces stand far from that line touches
/// nothing of the wall, though a vertex a hair from it — where a side
/// crosses the wall a hair from where the plane would touch it — was taken
/// to lie on it: no face of the plane is there for the wall to lie on.
pub(in crate::brep::tessellation) fn touching_planes(
    body: &Body,
    center: DVec3,
    axis: DVec3,
    radius: f64,
) -> Vec<Plane> {
    let eps = body.scale().eps();
    let on_wall = |point: DVec3| {
        let from = point - center;
        ((from - axis * axis.dot(from)).length() - radius).abs() <= eps
    };
    let on_a_face = |id: SurfaceId, plane: &Plane, point: DVec3| {
        body.face_ids().any(|face| {
            body.face(face).surface == id
                && !matches!(
                    body.locate(face, plane.parameters(point), eps),
                    Ok(Location::Outside)
                )
        })
    };
    (0..body.surfaces.len() as u32)
        .map(SurfaceId)
        .filter_map(|id| match body.surface(id) {
            Surface::Plane(plane) => Some((id, *plane)),
            Surface::Cylinder(_)
            | Surface::Cone(_)
            | Surface::Elliptic(_)
            | Surface::Sphere(_)
            | Surface::Torus(_) => None,
        })
        .filter(|(id, plane)| {
            plane.normal.dot(axis).abs() <= Scale::RELATIVE
                && (plane.distance(center).abs() - radius).abs() <= eps
                && body
                    .vertex_ids()
                    .map(|vertex| body.vertex(vertex))
                    .any(|vertex| {
                        vertex.on.contains(id)
                            && on_wall(vertex.point)
                            && on_a_face(*id, plane, vertex.point)
                    })
        })
        .map(|(_, plane)| plane)
        .collect()
}

impl Samples {
    /// Each sample from `from` on, added inside a face of `sphere`, laid on
    /// a plane of `body` within the tolerance of touching the sphere where
    /// it stands within the tolerance of the plane: the kernel decided the
    /// two touch at a point there (#939), and a sample on the sphere a hair
    /// through the plane would lay triangles across the plane's face.
    pub(in crate::brep::tessellation) fn on_a_touching_plane(
        &mut self,
        body: &Body,
        sphere: &Sphere,
        from: usize,
    ) {
        let eps = body.scale().eps();
        let middle = sphere.middle();
        for surface in &body.surfaces {
            let Surface::Plane(plane) = surface else {
                continue;
            };
            if (plane.distance(middle).abs() - sphere.radius).abs() > eps {
                continue;
            }
            for point in &mut self.points[from..] {
                let away = plane.distance(*point);
                if away.abs() <= eps {
                    *point -= plane.normal * away;
                }
            }
        }
    }
}
