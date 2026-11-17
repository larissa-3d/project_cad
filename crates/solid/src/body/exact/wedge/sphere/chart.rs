//! The axis a piece of sphere closing a corner is read about: its chart's two
//! poles where the line along it through the centre pierces the sphere, each
//! of which must stand at a corner of the piece that stays one, or off its
//! boundary — a pole inside an edge has no corner to anchor the piece's flux
//! at, and the assembly joins no two edges of one circle at a pole, so that a
//! seam there would keep two names (#911).

use std::f64::consts::TAU;

use glam::DVec3;

use crate::brep::{self, Coedge, Curve, Scale};

/// The first of `ways`, then of the ways from `centre` to each corner of
/// `lap`, whose poles on the sphere of `radius` about `centre` both stand
/// where [`unanchored`] does not put them. None where none does.
pub(super) fn about(
    body: &brep::Body,
    lap: &[Coedge],
    (centre, radius): (DVec3, f64),
    ways: impl IntoIterator<Item = DVec3>,
) -> Option<DVec3> {
    let eps = body.scale().eps();
    let corners = (lap.iter())
        .filter_map(|coedge| body.edge(coedge.edge).ends)
        .map(|[from, _]| (body.vertex(from).point - centre).normalize());
    ways.into_iter().chain(corners).find(|&way| {
        [1.0, -1.0]
            .into_iter()
            .all(|side| !unanchored(body, lap, centre + way * (side * radius), eps))
    })
}

/// Whether a pole at `point` stands where no corner of `lap` anchors it: on an
/// arc of it short of both its ends, or at a corner between two arcs of one
/// circle, which is no corner once the assembly joins them.
fn unanchored(body: &brep::Body, lap: &[Coedge], point: DVec3, eps: f64) -> bool {
    let circle = |coedge: &Coedge| match body.curve(body.edge(coedge.edge).curve) {
        Curve::Circle(circle) => Some(*circle),
        _ => None,
    };
    let inside = lap.iter().any(|coedge| {
        let edge = body.edge(coedge.edge);
        let Some(circle) = circle(coedge) else {
            return false;
        };
        let (low, high) = (edge.from.min(edge.to), edge.from.max(edge.to));
        let at = low + (circle.parameter(point) - low).rem_euclid(TAU);
        let ends = [low, high].map(|end| circle.point(end));
        circle.point(at).distance(point) <= eps
            && at < high
            && ends.iter().all(|end| end.distance(point) > eps)
    });
    let joined = (lap.iter().zip(lap.iter().cycle().skip(1))).any(|(one, next)| {
        let ends = body.edge(one.edge).ends;
        let end = ends.map(|[from, to]| if one.forward { to } else { from });
        let one_plane = circle(one)
            .zip(circle(next))
            .is_some_and(|(one, next)| one.axis.cross(next.axis).length() <= Scale::RELATIVE);
        end.is_some_and(|end| body.vertex(end).point.distance(point) <= eps) && one_plane
    });
    inside || joined
}
