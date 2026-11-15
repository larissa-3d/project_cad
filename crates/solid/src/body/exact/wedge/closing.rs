//! A corner three edges of one step meet at (#699), closed once the three are
//! laid: the corner as the body given reads it, and the closing each kind
//! lays there — a triangle among flats, a piece of sphere among rounds, cut
//! where the three meet outwards and joined, hollow, where they meet inwards,
//! at a pocket's bottom corner (#801). Four or more meeting outwards, every
//! edge at the corner named (#804), are closed the same way where the shape
//! allows one surface, a polygon or a piece of one sphere; there each edge's
//! cut, carried past the corner, would take the matter of faces it does not
//! lie on, so the corner is cleared before any edge is laid.

use glam::{DVec2, DVec3};

use super::Wedge;
use super::corners::Closed;
use super::overlap::CLEARED;
use crate::body::exact::Exact;
use crate::brep::Declined;
use crate::profile::{Contour, Frame};

/// What closes a corner edges meet at, on the body they were laid on, the
/// faces they laid numbered as the numbers say, by their ranks, its own face
/// numbered where the steps' numbers end: as [`super::cut_across`] closes it
/// among flats, and [`super::rounded_across`] among rounds.
pub(in crate::body::exact) type Close = fn(&Exact, &Corner, &[u32]) -> Result<Exact, Declined>;

/// What a kind clears a corner four or more edges meet at with, on the body
/// as given, before any of them is laid: the solid cut from it, and how many
/// faces that cut is to leave there.
pub(in crate::body::exact) type Clear = fn(&Exact, &Corner) -> Result<(Exact, usize), Declined>;

/// What a kind closes a corner with: `outwards` where its three edges meet
/// outwards, `inwards` where they meet inwards, and `apex` where four or more
/// meet, outwards.
#[derive(Clone, Copy)]
pub(in crate::body::exact) struct Closing {
    pub outwards: Close,
    pub inwards: Close,
    pub apex: Apex,
}

/// What a kind closes a corner four or more edges meet at with: the clearing
/// `clear` cuts before any edge is laid, and the face `close` lays once all
/// are, in place of what the clearing left.
#[derive(Clone, Copy)]
pub(in crate::body::exact) struct Apex {
    pub clear: Clear,
    pub close: Close,
}

/// How far what fills a concave corner reaches into the matter past the
/// corner's three faces, as a share of the value: far enough that no face of
/// it lies nearly flush with theirs, near enough to stay inside a wall or a
/// floor thicker than that.
const SUNK: f64 = 0.125;

/// A corner edges of a step meet at, read on the body as given: where it
/// stands; where the set-backs of each two beside each other round it cross
/// on the face the two share — the first's and the next's sharing a face
/// with it, and so on round to the last's and the first's: at a corner of
/// three, the first and the second's, the second and the third's, the third
/// and the first's — and the way each of those faces faces; the way each edge
/// runs from the corner; the value its first edge is laid at; and whether
/// each edge runs from it to another corner the step closes. Those of the
/// edges in the order of their ranks.
pub(in crate::body::exact) struct Corner {
    pub at: DVec3,
    pub crossings: Vec<DVec3>,
    pub facing: Vec<DVec3>,
    pub along: Vec<DVec3>,
    pub value: f64,
    pub between: Vec<bool>,
}

/// Each corner three or more edges meet at, read as a [`Corner`] for the
/// value `laid_at` says each edge is laid at, the edge set back `set_back` of
/// a value of one along its faces, with the ranks of its edges and what of
/// `closing` closes it, by the way its edges meet. Declined `Unsupported`
/// where the faces between its edges close no ring round it, two of them
/// beside each other have set-backs that never cross, three do not all meet
/// the same way, or four or more do not all meet outwards.
pub(super) fn closings(
    corners: Vec<Closed>,
    wedges: &[Wedge],
    laid_at: impl Fn(usize) -> f64,
    set_back: impl Fn(&Wedge) -> f64,
    closing: Closing,
) -> Result<Vec<(Corner, Vec<usize>, Close)>, Declined> {
    let closed: Vec<Vec<usize>> = corners.iter().map(|(_, ranks, _)| ranks.clone()).collect();
    corners
        .into_iter()
        .map(|(at, ranks, along)| {
            let met: Vec<&Wedge> = ranks.iter().map(|&rank| &wedges[rank]).collect();
            let backs: Vec<f64> = (ranks.iter())
                .map(|&rank| laid_at(rank) * set_back(&wedges[rank]))
                .collect();
            let between = (ranks.iter())
                .map(|rank| (closed.iter()).filter(|other| other.contains(rank)).count() > 1)
                .collect();
            let corner = read((at, along), &met, &backs, laid_at(ranks[0]), between)
                .ok_or(Declined::Unsupported)?;
            let inwards = met.iter().filter(|wedge| wedge.inwards).count();
            let close = match (ranks.len(), inwards) {
                (3, 0) => closing.outwards,
                (3, 3) => closing.inwards,
                (_, 0) => closing.apex.close,
                _ => return Err(Declined::Unsupported),
            };
            Ok((corner, ranks, close))
        })
        .collect()
}

/// The corner at `at` of edges running from it `along`, each set back
/// `set_back` along its faces, as [`closings`] reads it, round the ring of
/// faces between them from the first.
fn read(
    (at, along): (DVec3, Vec<DVec3>),
    wedges: &[&Wedge],
    set_back: &[f64],
    value: f64,
    between: Vec<bool>,
) -> Option<Corner> {
    let shared = |one: usize, other: usize| {
        let [first, second] = [wedges[one], wedges[other]];
        (0..2)
            .flat_map(|i| (0..2).map(move |j| (i, j)))
            .find(|&(i, j)| first.faces[i] == second.faces[j])
    };
    let crossing = |one: usize, other: usize| {
        let [first, second] = [wedges[one], wedges[other]];
        let (on_first, on_second) = shared(one, other)?;
        let from = first.point + first.into[on_first] * set_back[one];
        let to = second.point + second.into[on_second] * set_back[other];
        let normal = first.along.cross(second.along);
        let along = (to - from).cross(second.along).dot(normal) / normal.length_squared();
        along
            .is_finite()
            .then(|| (from + first.along * along, first.outward[on_first]))
    };
    let mut ring = vec![0];
    while ring.len() < wedges.len() {
        let last = ring[ring.len() - 1];
        let next =
            (0..wedges.len()).find(|at| !ring.contains(at) && shared(last, *at).is_some())?;
        ring.push(next);
    }
    let (crossings, facing) = (ring.iter().zip(ring.iter().cycle().skip(1)))
        .map(|(&one, &other)| crossing(one, other))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .unzip();
    Some(Corner {
        at,
        crossings,
        facing,
        along,
        value,
        between,
    })
}

impl Corner {
    /// `solid` less what of it stands deeper in the matter than [`SUNK`] of
    /// the value past the corner's three faces, every point of it within
    /// `reach` of the corner: what fills a concave corner, kept inside its
    /// walls.
    pub fn sunk(&self, solid: Exact, reach: f64) -> Result<Exact, Declined> {
        let depth = self.value * SUNK;
        (self.facing.iter()).try_fold(solid, |kept, &facing| {
            kept.cut_by(&slab(-facing, self.at - facing * depth, reach)?)
        })
    }
}

/// A box standing on the plane through `from` square to `way`, `half` either
/// side of `from` across it, raised along `way` twice that.
pub(super) fn slab(way: DVec3, from: DVec3, half: f64) -> Result<Exact, Declined> {
    let u = way.any_orthonormal_vector();
    let frame = Frame {
        origin: from,
        u,
        v: way.cross(u),
    };
    let square = Contour::rectangle(DVec2::splat(-half), DVec2::splat(half));
    Exact::raised(&square, &[], frame, way * 2.0 * half)
}

impl Exact {
    /// The body with each corner `closes` holds closed as its closing closes
    /// it, the faces laid on its edges numbered from `start` by their ranks,
    /// in turn.
    pub(super) fn closed(
        self,
        closes: &[(Corner, Vec<usize>, Close)],
        start: u32,
    ) -> Result<Exact, Declined> {
        closes
            .iter()
            .try_fold(self, |body, (corner, ranks, close)| {
                let numbers: Vec<u32> = ranks.iter().map(|&rank| start + rank as u32).collect();
                close(&body, corner, &numbers)
            })
    }

    /// The body with each corner of `closes` four or more edges meet at
    /// cleared as `clear` clears it, before any of them is laid, so that no
    /// edge's cut, carried past the corner, takes the matter of a face it
    /// does not lie on: what each clearing leaves numbered past the steps'
    /// numbers from [`CLEARED`], one number for each corner, in the order of
    /// their ranks, until the corner's closing lays its own face there.
    /// Declined `Unsupported` where a clearing takes a face of the body away
    /// whole, or leaves more or fewer faces answering to its number than it
    /// says; otherwise as it declines.
    pub(super) fn cleared(
        &self,
        closes: &[(Corner, Vec<usize>, Close)],
        clear: Clear,
    ) -> Result<Exact, Declined> {
        (closes.iter())
            .filter(|(_, ranks, _)| ranks.len() > 3)
            .enumerate()
            .try_fold(self.clone(), |body, (index, (corner, _, _))| {
                let (solid, leaves) = clear(&body, corner)?;
                let number = CLEARED + index as u32;
                let tool = Exact {
                    brep: solid.brep.renumbered_by(|_| number),
                    next: 0,
                    ..solid
                };
                let cut = body.cut_by(&tool)?;
                let left = (cut.brep.face_ids())
                    .filter(|&face| cut.brep.numbers(face).contains(&(number + body.next)))
                    .count();
                if body.kept_on(&cut, &[]) && left == leaves {
                    Ok(cut)
                } else {
                    Err(Declined::Unsupported)
                }
            })
    }
}
