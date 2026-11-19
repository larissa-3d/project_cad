//! The join a fill makes (#778): the prism a concave edge is filled with,
//! raised along it as a cut's is, trimmed flush at each end on the surface of
//! the face the edge ends on there — a plane, a round (#788) or a sphere
//! (#805) — and joined to the body, its ends merged into those faces and the
//! edges either side of the edge's ends made one where they run on along one
//! curve. The later of two fills meeting at a corner beside a convex edge
//! takes with it what both prisms hold past that corner (#801), and the two
//! run on into a mitre. Two fills laid as one, a round bottom, ending on the
//! two walls of a pointed or a notched end are trimmed flush on both (#921).

mod ball;
mod round;

use glam::DVec3;

use super::end::{behind, beyond, end_face};
use super::flush::{Held, flat, holding};
use super::overlap::{Overlap, common};
use super::{Prism, Wedge};
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, FaceId, Plane, Surface, VertexId};

impl Exact {
    /// The body with the corner along each of `edges`, read on `on`, filled in
    /// by `tool`, the prism raised from `prism` — its first walls, one for each
    /// edge, numbered from nought and counted as raised, so that the solids
    /// its ends are trimmed by and the overlaps it takes number their faces
    /// past its own: each end of it trimmed flush on the surface of the face
    /// the first edge ends on — or, where another edge ends on another face
    /// there, on the planes of both, as [`past_two_faces`] trims it, a round
    /// bottom at a pointed or a notched end (#921), or, at an end one of
    /// `held` holds, on the plane it holds it at, in the air, as
    /// [`Held::beyond`] trims it, that end left standing as a face of its own
    /// — joined to each of `overlaps`, what it takes past a corner it shares
    /// with a fill laid before it beside a convex edge,
    /// and joined to the body, every face of the prism that merged into one
    /// of the body's answering to the body's numbers alone, so that the face
    /// it merged into stays the face it was: its ends, merged into the faces
    /// at the edges' ends, and a wall of it lying flush on a face of the
    /// body, as the wall squared off a face taken whole lies on the face
    /// beyond. A face of the prism standing alone keeps its number, past the
    /// steps' count, and that alone is what [`Exact::cut_along`] finds
    /// stray, a face the solid an end is trimmed by leaves on the fill among
    /// them: the fill standing past a round that turns away short of its
    /// reach. Where the face at an end meets one of an edge's two along a
    /// curve running on past the edge's end — the plate's top round a rib
    /// ending short of its edge, or round its rounded end — the two edges
    /// either side of that corner are made one, since the fill covered what
    /// of the edge's faces reached it, and so are those either side of each
    /// point an overlap clears and, where it takes one, of each corner this
    /// body holds on the plane of either of the edge's faces: a fill laid
    /// before it, stopped on that plane, runs on into the overlap past it, and
    /// so does the valley it meets a round bottom along (#921). Declined
    /// `Unsupported`: an end where
    /// no face but the edge's two meets it, or several do, or one the fill
    /// cannot be trimmed flush on, as [`past_the_end`] and [`past_two_faces`]
    /// decline it, and another edge that does not end on the faces the first
    /// ends on or the one other face there.
    pub(super) fn filled(
        &self,
        on: &brep::Body,
        edges: &[(EdgeId, &Wedge)],
        (overlaps, held): (&[Overlap], &[Held]),
        prism: &Prism,
        tool: Exact,
    ) -> Result<Exact, Declined> {
        let &[(edge, wedge), ..] = edges else {
            return Err(Declined::Unsupported);
        };
        let ends = on.edge(edge).ends.ok_or(Declined::Unsupported)?;
        let eps = on.scale().eps();
        let mut trimmed = tool;
        let mut end_faces = Vec::new();
        for (end, past) in ends.into_iter().zip([-wedge.along, wedge.along]) {
            let face = end_face(on, end, wedge.faces).ok_or(Declined::Unsupported)?;
            let point = on.vertex(end).point;
            let elsewhere = (edges[1..].iter()).find_map(|&(edge, wedge)| {
                let ends = on.edge(edge).ends?;
                let along = |end: VertexId| past.dot(on.vertex(end).point);
                let far = ends[usize::from(along(ends[1]) > along(ends[0]))];
                let theirs = end_face(on, far, wedge.faces)?;
                (theirs != face).then(|| (theirs, on.vertex(far).point))
            });
            let beyond_the_end = match (holding(held, point, eps), elsewhere) {
                (Some((end, number)), _) => vec![end.beyond(prism, number)?],
                (None, Some((theirs, at))) => {
                    end_faces.push(theirs);
                    past_two_faces(on, [(face, point), (theirs, at)], past, prism)?
                }
                (None, None) => past_the_end(on, face, point, past, prism)?,
            };
            for solid in &beyond_the_end {
                trimmed = trimmed.cut_by(solid)?;
            }
            end_faces.push(face);
        }
        let mut corners = Vec::new();
        for &(edge, wedge) in edges {
            let ends = on.edge(edge).ends.ok_or(Declined::Unsupported)?;
            for end in ends {
                let face = end_face(on, end, wedge.faces).ok_or(Declined::Unsupported)?;
                if !end_faces.contains(&face) {
                    return Err(Declined::Unsupported);
                }
                corners.push(on.vertex(end).point);
            }
        }
        for overlap in overlaps {
            trimmed = trimmed.joined(&overlap.solid)?;
            corners.extend(overlap.cleared.iter().copied());
        }
        corners.extend(self.seams(wedge, overlaps, eps));
        let next = edges.len() as u32;
        self.joined(&Exact { next, ..trimmed })?.merged_at(&corners)
    }
}

/// The solids to cut from the prism of two edges laid as one so that it ends
/// flush on the planes of the two faces they end on, each at its corner,
/// running on `past` there: the boxes [`behind`] raises on each plane, on the
/// side `past` points to, however far along its long edges cross it — a
/// round bottom carried over the part's box may stop short of a wall leaning
/// across its end. Where the second corner lies past the first's plane — a
/// notched end, whose air runs on past each plane beside the other — what
/// lies past both is cut, and otherwise — a pointed end — what lies past
/// either. Declined `Unsupported` where either face is not flat or runs along
/// the edges; otherwise as the booleans decline.
fn past_two_faces(
    body: &brep::Body,
    [(one, at), (other, theirs)]: [(FaceId, DVec3); 2],
    past: DVec3,
    prism: &Prism,
) -> Result<Vec<Exact>, Declined> {
    let side = |id: FaceId| {
        let (plane, outward) = flat(body, id).ok_or(Declined::Unsupported)?;
        let on = outward.dot(past);
        if on == 0.0 {
            return Err(Declined::Unsupported);
        }
        let away = outward * on.signum();
        Ok((away, behind(prism, plane, away)?))
    };
    let [(away, first), (_, second)] = [side(one)?, side(other)?];
    if away.dot(theirs - at) > body.scale().eps() {
        Ok(vec![common(&first, &second)?])
    } else {
        Ok(vec![first, second])
    }
}

/// The solid to cut from the prism so that it ends flush on the surface of
/// the face `id` its edge ends on at `point`, running on `past` there: on a
/// plane, the box [`beyond`] raises on the side `past` points to — out of
/// the matter where the plane faces on along the edge, into it where it
/// faces back, at a pocket's floor or the plate a block stands on (#801),
/// the fill's end then lying on the face, which the join swallows; on a
/// cylinder or a cone the edge runs out of at `point`, whichever side of it
/// the matter lies — a rib's round end bulging out or drawn in by draft, or a
/// slot's round end turning in toward the slot off its wall, which the fill
/// then runs on into — what [`round::outside_cylinder`] raises or
/// [`round::outside_cone`] turns; on a cylinder the edge runs into, its
/// matter inside — a pillar (#908) — the box [`beyond`] raises past the
/// plane through the axis square to the edge's run off the axis, where
/// each long edge that meets the cylinder stands midway through it, the
/// fill's end then buried in the pillar and short of its far side —
/// [`round::outside_cylinder`] would start its cut where the first edge
/// leaves the pillar, on that far side, and leave a corner on its foot
/// where the fill crosses it there; on a sphere the edge runs out of at
/// `point` — a rib cut from a ball (#805) — what [`ball::outside_sphere`]
/// cuts. Declined `Unsupported` on a plane along the edge, a cylinder the
/// edge runs into with its matter outside, a cone or a sphere the edge runs
/// into, or another surface; otherwise as those declines.
fn past_the_end(
    body: &brep::Body,
    id: FaceId,
    point: DVec3,
    past: DVec3,
    prism: &Prism,
) -> Result<Vec<Exact>, Declined> {
    let face = body.face(id);
    let surface = body.surface(face.surface);
    let normal = surface.normal(surface.parameters(point));
    let outward = if face.flipped { -normal } else { normal };
    match surface {
        Surface::Plane(plane) if outward.dot(past) != 0.0 => Ok(vec![beyond(
            prism,
            plane,
            outward * outward.dot(past).signum(),
        )?]),
        Surface::Cylinder(cylinder) if normal.dot(past) > 0.0 => {
            Ok(vec![round::outside_cylinder(prism, cylinder, past)?])
        }
        Surface::Cylinder(cylinder) if !face.flipped && normal.dot(past) < 0.0 => {
            let towards = past - cylinder.axis * cylinder.axis.dot(past);
            let (plane, _) = Plane::through(cylinder.origin, towards);
            Ok(vec![beyond(prism, &plane, towards.normalize())?])
        }
        Surface::Cone(cone) => round::outside_cone(prism, cone, point, past, body.scale().reach()),
        Surface::Sphere(sphere) if normal.dot(past) > 0.0 => Ok(vec![ball::outside_sphere(
            prism,
            sphere,
            past,
            body.scale().reach(),
        )?]),
        _ => Err(Declined::Unsupported),
    }
}
