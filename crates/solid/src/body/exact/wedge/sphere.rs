//! The piece of sphere three fillets at a convex corner are closed with
//! (#699): centred one radius from the corner's three faces, where the three
//! rounds' axes cross, it meets each round along the round's end section —
//! the arc square to its edge through the centre — and touches each face at
//! one point. No boolean lays it: the sphere is tangent to each round along
//! that arc, and two of the rounds turn about another axis than any one
//! sphere's chart. The rounds are laid whole, the corner is cut back to the
//! three planes through the centre square to the edges, which leaves each
//! round ending on its arc and three quarter discs meeting at the centre, and
//! those three are replaced by the piece of sphere their arcs bound. At a
//! concave corner, a pocket's bottom (#801), the mirror: the sphere centred
//! one radius out from the three faces, the fills laid trimmed flush, the
//! corner between them filled up to the three planes, and the three quarter
//! discs that leaves, facing the centre, replaced by the piece of sphere
//! facing it, the matter outside the ball. Two corners at the ends of one
//! edge whose centres are one — the edge across a tongue's tip as wide as
//! two radii, whose round closes to nothing between them (#837) — close on
//! one sphere: the later corner's cut takes that round whole, and its piece
//! takes in the earlier corner's, one face answering to the round's number
//! and both corners'. Four or more rounds meeting outwards (#804), a
//! pyramid's apex, are closed the same way where the faces round the corner
//! are all one radius from one centre: the corner cut back to the planes
//! through it square to the edges before any round is laid, so that each
//! round, carried past the corner, meets air there, and the faces that leaves
//! replaced once all are laid.

use glam::DVec3;

use super::closing::{Corner, slab};
use crate::body::drawn::Drawn;
use crate::body::exact::Exact;
use crate::brep::{self, Coedge, Curve, Declined, Face, FaceId, Sphere, Surface, SurfaceId};

/// The body with the corner closed by a piece of sphere of the value's
/// radius, numbered where the steps' numbers end, its centre where the
/// rounds' axes cross, one radius in from the first crossing's face.
/// Declined `Unsupported` where the plane through the centre square to an
/// edge does not stand between the corner and the edge's other end — the
/// sphere would stand at or behind the corner along it —, the corner cut
/// back leaves anything but one face for each of its edges meeting at the
/// centre, bounded towards the rounds by as many arcs about it — one more
/// for the piece an
/// earlier corner laid on the same sphere beside them — or the piece of
/// sphere left anything but its face, its every corner a corner of one of
/// the `rounds` still standing, as [`Exact::cut_along`] reads it, or took
/// whole a round whose edge runs to no other corner the step closes;
/// `Undrawn` where the piece cannot be cut into triangles.
pub(in crate::body::exact) fn rounded_across(
    body: &Exact,
    corner: &Corner,
    rounds: &[u32],
) -> Result<Exact, Declined> {
    let centre = corner.crossings[0] - corner.facing[0] * corner.value;
    ahead(body, corner, centre)?;
    let cut = body.cut_by(&Exact {
        next: 1,
        ..short_of(corner, centre)?
    })?;
    sphered(body, cut, corner, centre, rounds, false)
}

/// The body with a concave corner closed by a hollow piece of sphere of the
/// value's radius, facing its centre, numbered where the steps' numbers end:
/// its centre one radius out from the first crossing's face, where the fills'
/// axes cross, and the corner between the three fills filled up to the three
/// planes through the centre square to the edges, as [`filling`] holds it.
/// Declined as [`rounded_across`] declines.
pub(in crate::body::exact) fn hollowed_across(
    body: &Exact,
    corner: &Corner,
    rounds: &[u32],
) -> Result<Exact, Declined> {
    let centre = corner.crossings[0] + corner.facing[0] * corner.value;
    ahead(body, corner, centre)?;
    let joined = body.joined(&Exact {
        next: 1,
        ..filling(corner, centre)?
    })?;
    sphered(body, joined, corner, centre, rounds, true)
}

/// What of a corner four or more rounds meet at is cut from the body before
/// any of them is laid: what [`short_of`] holds about the centre one radius
/// in from the first crossing's face, and the faces it leaves there, one on
/// each plane through the centre square to an edge. Declined as
/// [`rounded_across`] declines a sphere at or behind the corner.
pub(in crate::body::exact) fn cell(
    body: &Exact,
    corner: &Corner,
) -> Result<(Exact, usize), Declined> {
    let centre = corner.crossings[0] - corner.facing[0] * corner.value;
    ahead(body, corner, centre)?;
    Ok((short_of(corner, centre)?, corner.along.len()))
}

/// The body with the corner [`cell`] cleared closed once its rounds are laid:
/// the faces the cell left, each ending on a round's arc, replaced by the
/// piece of sphere about the centre, numbered where the steps' numbers end,
/// as [`rounded_across`] lays it. Declined as [`rounded_across`] declines:
/// where the faces round the corner touch no one sphere of the radius, a
/// round's axis misses the centre, and its arc is about another point.
pub(in crate::body::exact) fn sphere_across(
    body: &Exact,
    corner: &Corner,
    rounds: &[u32],
) -> Result<Exact, Declined> {
    let centre = corner.crossings[0] - corner.facing[0] * corner.value;
    let laid = Exact {
        next: body.next + 1,
        ..body.clone()
    };
    sphered(body, laid, corner, centre, rounds, false)
}

/// `body` with the faces `laid` left meeting at `centre`, one for each of the
/// corner's edges, and the piece
/// an earlier corner laid on the same sphere beside them, replaced by the
/// piece of sphere about it their arcs bound, `flipped` to face its centre
/// where the matter lies outside the ball, as [`rounded_across`] lays it:
/// answering to the earlier piece's numbers and to those of the `rounds` the
/// cut took whole, between this corner and another the step closes, and read
/// about the edge of such a round where there is
/// one, so that no pole stands inside an arc the piece meets another round
/// along, which then stays one edge.
fn sphered(
    body: &Exact,
    laid: Exact,
    corner: &Corner,
    centre: DVec3,
    rounds: &[u32],
    flipped: bool,
) -> Result<Exact, Declined> {
    let radius = corner.value;
    let eps = body.brep.scale().eps();
    let mut walls = meeting_at(&laid.brep, centre, eps);
    if walls.len() != rounds.len() {
        return Err(Declined::Unsupported);
    }
    let before = laid_before(&laid.brep, &walls, centre, radius, eps);
    walls.extend(&before);
    let lap = bounding(&laid.brep, &walls, centre, radius, eps)
        .filter(|lap| lap.len() == rounds.len() + before.len())
        .ok_or(Declined::Unsupported)?;
    let (gone, kept): (Vec<usize>, Vec<usize>) = (0..rounds.len())
        .partition(|&at| corner.between[at] && !laid.has_face(rounds[at] as usize));
    let about = gone
        .first()
        .map_or(axis(&corner.along), |&at| corner.along[at]);
    let sphere = Sphere::through(centre, about, 0.0, radius);
    let mut numbers: Vec<u32> = (gone.iter().map(|&at| rounds[at]))
        .chain(
            before
                .iter()
                .flat_map(|&face| laid.brep.numbers(face).iter().copied()),
        )
        .chain([body.next])
        .collect();
    numbers.sort_unstable();
    let face = Face {
        surface: SurfaceId(0),
        flipped,
        loops: vec![lap],
        numbers,
        poles: Vec::new(),
    };
    let brep = laid.brep.replaced(&walls, Surface::Sphere(sphere), face)?;
    let closed = laid.patched(brep, laid.next)?;
    let mitred: Vec<u32> = kept.iter().map(|&at| rounds[at]).collect();
    if closed.cut_along(&[], body, 1, &mitred, &[], &[]) {
        Ok(closed)
    } else {
        Err(Declined::Unsupported)
    }
}

/// Declined `Unsupported` where the plane through `centre` square to an edge
/// does not stand between the corner and the edge's other end: the sphere
/// would stand at or behind the corner along it.
fn ahead(body: &Exact, corner: &Corner, centre: DVec3) -> Result<(), Declined> {
    let eps = body.brep.scale().eps();
    if (corner.along.iter()).any(|way| way.dot(centre - corner.at) <= eps) {
        Err(Declined::Unsupported)
    } else {
        Ok(())
    }
}

/// The axis the piece of sphere is read about: square to two of the edges
/// next in rank, and as near along the one after them as any such: at a
/// corner of three, its end sections then lie on two meridians meeting at a
/// pole, where the piece touches the face of the two edges, and as near the
/// equator as the corner lets the third — on it where the third edge stands
/// square to the other two, at a square corner or the end of a prism.
fn axis(along: &[DVec3]) -> DVec3 {
    let count = along.len();
    (0..count)
        .map(|one| {
            let axis = along[one].cross(along[(one + 1) % count]).normalize();
            (axis.dot(along[(one + 2) % count]).abs(), axis)
        })
        .max_by(|(one, _), (other, _)| one.total_cmp(other))
        .map_or(along[0], |(_, axis)| axis)
}

/// What of the corner stands on its side of the three planes through
/// `centre` square to its edges: a cube about the centre, its half side
/// twice the way from the centre to the corner, less what of it stands past
/// each plane.
fn short_of(corner: &Corner, centre: DVec3) -> Result<Exact, Declined> {
    let half = 2.0 * centre.distance(corner.at);
    let last = corner.along[corner.along.len() - 1];
    let mut kept = slab(last, centre - last * half, half)?;
    for &way in &corner.along {
        kept = kept.cut_by(&slab(way, centre, 2.0 * half)?)?;
    }
    Ok(kept)
}

/// What of a concave corner stands on its side of the three planes through
/// `centre` square to its edges and no deeper into the matter than
/// [`Corner::sunk`] keeps it: what [`short_of`] holds, less what of it stands
/// deeper.
fn filling(corner: &Corner, centre: DVec3) -> Result<Exact, Declined> {
    corner.sunk(short_of(corner, centre)?, 4.0 * centre.distance(corner.at))
}

/// The faces with a corner at `centre`, within `eps`.
pub(super) fn meeting_at(body: &brep::Body, centre: DVec3, eps: f64) -> Vec<FaceId> {
    body.face_ids()
        .filter(|&face| {
            (body.face(face).loops.iter().flatten())
                .flat_map(|coedge| body.edge(coedge.edge).ends.into_iter().flatten())
                .any(|end| body.vertex(end).point.distance(centre) <= eps)
        })
        .collect()
}

/// The loop `walls` make towards the rest of the body, the way they run it,
/// as [`lap`] reads it: arcs about `centre` of `radius`. None where an edge
/// of it is no such arc, or they close no loop.
fn bounding(
    body: &brep::Body,
    walls: &[FaceId],
    centre: DVec3,
    radius: f64,
    eps: f64,
) -> Option<Vec<Coedge>> {
    let about = |coedge: &Coedge| match body.curve(body.edge(coedge.edge).curve) {
        Curve::Circle(circle) => {
            circle.center.distance(centre) <= eps && (circle.radius - radius).abs() <= eps
        }
        _ => false,
    };
    let lap = lap(body, walls)?;
    lap.iter().all(about).then_some(lap)
}

/// The faces a corner closed before laid on the sphere of `radius` about
/// `centre`, none of `walls`: the other end of an edge whose round, between
/// two corners whose centres are one, closes to nothing, so that the two
/// pieces are one.
fn laid_before(
    body: &brep::Body,
    walls: &[FaceId],
    centre: DVec3,
    radius: f64,
    eps: f64,
) -> Vec<FaceId> {
    let sphere = |face: FaceId| match body.surface(body.face(face).surface) {
        Surface::Sphere(sphere) => {
            sphere.middle().distance(centre) <= eps && (sphere.radius - radius).abs() <= eps
        }
        _ => false,
    };
    body.face_ids()
        .filter(|&face| !walls.contains(&face) && sphere(face))
        .collect()
}

/// The loop `walls` make towards the rest of the body, the way they run it:
/// each of their edges a face that is none of them meets them along, joined
/// end to end — what bounds a face laid in their place. None where those
/// close no one loop.
pub(super) fn lap(body: &brep::Body, walls: &[FaceId]) -> Option<Vec<Coedge>> {
    let mut arcs: Vec<Coedge> = (walls.iter())
        .flat_map(|&wall| body.face(wall).loops.iter().flatten().copied())
        .filter(|coedge| (body.uses(coedge.edge).iter()).any(|(face, _)| !walls.contains(face)))
        .collect();
    let ends = |coedge: &Coedge| {
        let [from, to] = body.edge(coedge.edge).ends?;
        Some(if coedge.forward {
            [from, to]
        } else {
            [to, from]
        })
    };
    let mut lap = vec![arcs.pop()?];
    while !arcs.is_empty() {
        let [_, end] = ends(lap.last()?)?;
        let next = (arcs.iter()).position(|arc| ends(arc).is_some_and(|[from, _]| from == end))?;
        lap.push(arcs.swap_remove(next));
    }
    let ([start, _], [_, end]) = (ends(&lap[0])?, ends(lap.last()?)?);
    (start == end).then_some(lap)
}

impl Exact {
    /// `brep`, this body with faces laid in place of some of its own, as
    /// [`crate::brep::Body::replaced`] lays them, its pieces and their roots,
    /// its count `next`, drawn again: what closes a corner with faces no
    /// boolean lays. Declined `Undrawn` where it cannot be drawn whole.
    pub(super) fn patched(&self, brep: brep::Body, next: u32) -> Result<Exact, Declined> {
        let drawn = Drawn::of(&brep);
        if !drawn.is_whole() {
            return Err(Declined::Undrawn);
        }
        let closed = Exact::of(brep, next, self.pieces, self.roots.clone());
        let _ = closed.drawn.set(drawn);
        Ok(closed)
    }
}
