//! The way round the odd edge of a corner of mixed sense where that edge
//! leans on the face its two others share (#944): a pyramid's hip above the
//! two feet where its slopes meet the plate it stands on. No piece of torus
//! about a leaning round touches that face, so the two others stop short of
//! the round instead: each is laid first, alone, held at that corner on the
//! plane through the round's axis square to the face it shares with the odd
//! edge — the plane the round leaves that face along — its end a flat face
//! of its own; the odd edges, and every edge closing a corner of one sense
//! with them, are then laid on what that leaves, each round running on
//! between the two ends down to the face the two others share, which it
//! meets along one sharp edge.

use super::{Odd, ranked_patches};
use crate::body::exact::Exact;
use crate::body::exact::wedge::Wedge;
use crate::body::exact::wedge::corners;
use crate::body::exact::wedge::flush::Held;
use crate::body::exact::wedge::overlap::{ENDS, STRIDE};
use crate::body::exact::wedge::step::{Kind, Step};
use crate::body::exact::wedge::torus::axis_at;
use crate::brep::{Declined, EdgeName, Scale};

/// Whether the odd edge of `corner`, read among `wedges` with its two
/// others, both named, is convex and leans on the face the two others share:
/// its way parts from that face's normal by more than the tolerance across
/// the whole box at `scale`.
pub(super) fn leans(corner: &Odd, wedges: &[Wedge], scale: Scale) -> bool {
    if corner.sharp.is_some() || wedges[corner.lone].inwards {
        return false;
    }
    let Some(&other) = (corner.ranks.iter()).find(|&&rank| rank != corner.lone) else {
        return false;
    };
    let other = &wedges[other];
    (0..2)
        .find(|&side| other.numbers[side] == corner.on)
        .is_some_and(|side| {
            let parting = corner.up.cross(other.outward[side]).length();
            parting * 2.0 * scale.reach() > scale.eps()
        })
}

/// Where the two others of each of `odd`'s corners end, read as `wedges`
/// holds them, each with its rank: held at the corner on the plane through
/// the axis of the odd edge's round of the value `value`, square to the face
/// the odd edge shares with it, and trimmed off toward the corner. Declined
/// `Unsupported` where one shares no face with the odd edge.
fn held(odd: &[Odd], wedges: &[Wedge], value: f64) -> Result<Vec<(usize, Held)>, Declined> {
    let mut held = Vec::new();
    for corner in odd {
        let round = &wedges[corner.lone];
        let through = axis_at(corner, round, value);
        for &rank in (corner.ranks.iter()).filter(|&&rank| rank != corner.lone) {
            let side = (0..2)
                .find(|&side| wedges[rank].faces.contains(&round.faces[side]))
                .ok_or(Declined::Unsupported)?;
            let square = round.along.cross(round.outward[side]).normalize();
            let away = square * square.dot(corner.at - through).signum();
            let (at, along) = (corner.at, wedges[rank].along);
            held.push((
                rank,
                Held {
                    at,
                    along,
                    away,
                    through,
                },
            ));
        }
    }
    Ok(held)
}

impl Exact {
    /// The body with the edges `names` find laid at `taken`, each corner of
    /// mixed sense `odd` holds gone short of the round of its leaning odd
    /// edge, `step` what the names find on this body: the two others of every
    /// corner first, one at a time in the order of their ranks, each followed
    /// onto what those before it left, as [`Exact::following`] finds it, and
    /// laid alone with `kind`, held at its corners as [`held`] reads them;
    /// then every other edge, followed onto what that leaves and laid with
    /// `kind`, as [`Exact::laid_at`] lays each list, each end held numbered
    /// below them a while, a face of the body they are laid on, so that a
    /// wall of a round's prism lying on its plane merges into it. The face
    /// laid on each named edge is numbered where the steps' numbers ended
    /// plus its rank in the list, then the face closing each corner three or
    /// more of the others meet at — a pyramid's apex — in the order of its
    /// corner's ranks, as [`ranked_patches`] numbers them, then each end
    /// held, in the order of the corners and of the edges ending at each.
    /// Declined `Unsupported` where an edge laid first meets another of the
    /// list but at a corner of `odd`, an edge cannot be followed, a list gives
    /// less than `taken`, as [`Step::bound`] reads it on the body it is laid
    /// on, or an end held is not left standing once the two others are laid;
    /// otherwise as [`Exact::laid_at`] declines either list.
    pub(super) fn short_of_the_round(
        &self,
        names: &[EdgeName],
        step: &Step,
        odd: &[Odd],
        taken: f64,
        kind: &Kind,
    ) -> Result<Exact, Declined> {
        let start = self.next;
        let wedges = &step.wedges;
        let held = held(odd, wedges, taken)?;
        let ends: Vec<Held> = held.iter().map(|&(_, end)| end).collect();
        let mut first: Vec<usize> = held.iter().map(|&(rank, _)| rank).collect();
        first.sort();
        first.dedup();
        let at_a_corner = |one: usize, other: usize| {
            (odd.iter()).any(|corner| corner.ranks.contains(&one) && corner.ranks.contains(&other))
        };
        let astray = first.iter().any(|&rank| {
            (0..names.len())
                .filter(|&other| other != rank && !at_a_corner(rank, other))
                .any(|other| corners::meet(&self.brep, [step.edges[rank], step.edges[other]]))
        });
        if astray {
            return Err(Declined::Unsupported);
        }
        let mut filled = self.clone();
        for &rank in &first {
            let name = filled.following(&names[rank], &wedges[rank])?;
            let laid = filled.laid_at(&[name], taken, kind, &ends)?;
            (filled, _) = laid.map_err(|_| Declined::Unsupported)?;
        }
        let lent = filled.next;
        let count = ends.len() as u32;
        let left = ENDS..ENDS + count * STRIDE;
        let held_on = filled.renamed_by(|number| {
            if left.contains(&number) {
                lent + (number - ENDS) / STRIDE
            } else {
                number
            }
        });
        let filled = Exact {
            next: lent + count,
            ..held_on
        };
        if !(lent..filled.next).all(|number| filled.has_face(number as usize)) {
            return Err(Declined::Unsupported);
        }
        let rest: Vec<usize> = (0..names.len())
            .filter(|rank| !first.contains(rank))
            .collect();
        let followed = (rest.iter())
            .map(|&rank| filled.following(&names[rank], &wedges[rank]))
            .collect::<Result<Vec<_>, Declined>>()?;
        let laid = filled.laid_at(&followed, taken, kind, &[])?;
        let (round, closed) = laid.map_err(|_| Declined::Unsupported)?;
        let patches = (closed.iter().enumerate())
            .map(|(index, corner)| {
                let mut ranks: Vec<usize> = corner.iter().map(|&at| rest[at]).collect();
                ranks.sort();
                (ranks, filled.next + (rest.len() + index) as u32)
            })
            .collect();
        let after = start + names.len() as u32;
        let past = after + closed.len() as u32;
        let mut moved = ranked_patches(patches, after);
        let laid_on = (first.iter().map(|&rank| (start, rank)).enumerate())
            .chain(rest.iter().map(|&rank| (filled.next, rank)).enumerate());
        for (index, (from, rank)) in laid_on {
            moved.insert(from + index as u32, start + rank as u32);
        }
        for index in 0..count {
            moved.insert(lent + index, past + index);
        }
        let renamed = round.renamed_by(|number| moved.get(&number).copied().unwrap_or(number));
        let next = past + count;
        Ok(Exact { next, ..renamed })
    }
}
