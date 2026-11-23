//! Where the corners of the face a step lays on an edge may stand, as
//! [`Exact::cut_along`] checks them, and which of them a later edge's cut is
//! to take away (#942).

use std::collections::BTreeSet;

use glam::DVec3;

use super::Wedge;
use super::corners::lies_on;
use super::flush::Stop;
use crate::body::exact::Exact;
use crate::brep::VertexId;

impl Exact {
    /// Where each corner of the face numbered `made` stands that lies on none
    /// of the faces of `wedges`, on no edge one of them crosses at a convex
    /// corner, as its `crossed` holds them, at no corner of a face numbered
    /// in `mitred`, and on the plane of none of `open`: none, where the face
    /// keeps to its sides.
    pub(super) fn off_its_sides(
        &self,
        made: u32,
        wedges: &[&Wedge],
        mitred: &[u32],
        open: &[Stop],
    ) -> Vec<DVec3> {
        let eps = self.brep.scale().eps();
        let mitres: BTreeSet<VertexId> = mitred
            .iter()
            .flat_map(|&number| self.corners(number))
            .collect();
        (self.corners(made))
            .filter(|corner| !mitres.contains(corner))
            .map(|corner| self.brep.vertex(corner).point)
            .filter(|&point| {
                !open.iter().any(|stop| stop.holds(point, eps))
                    && !wedges.iter().any(|wedge| {
                        let at = point - wedge.point;
                        wedge.outward.iter().any(|side| side.dot(at).abs() <= eps)
                            || (wedge.crossed.iter()).any(|&edge| lies_on(edge, point, eps))
                    })
            })
            .collect()
    }

    /// Whether the face numbered `made` still has a corner at one of
    /// `points`, within the body's tolerance.
    pub(super) fn keeps_a_corner_at(&self, made: u32, points: &[DVec3]) -> bool {
        let eps = self.brep.scale().eps();
        (self.corners(made)).any(|corner| {
            let at = self.brep.vertex(corner).point;
            points.iter().any(|point| point.distance(at) <= eps)
        })
    }

    /// The corners of the faces numbered `number`, once for each edge of
    /// theirs ending there.
    fn corners(&self, number: u32) -> impl Iterator<Item = VertexId> + '_ {
        self.named(number as usize)
            .flat_map(|face| self.brep.face(face).loops.iter().flatten())
            .flat_map(|coedge| self.brep.edge(coedge.edge).ends.into_iter().flatten())
    }
}
