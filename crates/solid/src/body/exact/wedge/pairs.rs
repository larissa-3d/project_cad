//! Two edges of one step facing each other across a face they share
//! (#698): the bound they set on the one value, so that their set-backs do
//! not cross on it, the pairs whose set-backs meet across it, which are laid
//! as one — a chain of cuts beside fills as one slope (#813) — and the edges
//! of the list such a pair is mitred against at a corner (#803): each edge
//! at its end, bounding the face that is gone, one across it or the two of a
//! pointed or a V-notched end (#843) — one of a V-notched end's alone, the
//! other left sharp (#917) —, the upright at a point's tip with
//! them, and any upright meeting one of the pair, at either end, alone or
//! with the edge at their end (#837) or a point's edge at its shoulder
//! (#911) — the front's upright at a slope's nose among them (#814); and the
//! edge at a round bottom's end, which its fill meets in a valley (#838), the
//! uprights there, alone or with that edge (#919), and the two edges of a
//! pointed or a notched one (#921).

mod ends;

use std::collections::BTreeSet;

use glam::DVec3;

use super::chain::TANGENT;
use super::{MADE, Prism, Wedge, too_short};
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, FaceId, Scale};

pub(super) use ends::at_their_ends;

/// What raises the one prism two edges whose set-backs meet are taken off
/// with, the earlier edge's wedge first, at the value whose set-backs meet
/// across the face they share: as [`Wedge::rounded_with`] raises it.
pub(in crate::body::exact) type Raise =
    fn(&Wedge, &Wedge, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>;

/// Which two edges whose set-backs meet across the face they share a kind
/// lays as one, and how.
#[derive(Clone, Copy)]
pub(in crate::body::exact) enum Together {
    /// Two whose faces meet the same way, taken off or filled in as one by
    /// the prism this raises — a full round, a round bottom. A cut beside a
    /// fill is laid edge by edge.
    Raised(Raise),
    /// A cut beside a fill, as [`Exact::sloped`] lays them at the value
    /// their set-backs meet at: one slope where their flats lie at an angle
    /// the kernel tells from none, the fill raised on the cut's plane, and
    /// two flats meeting in a crease where it tells it. Two whose faces meet
    /// the same way are laid edge by edge.
    Sloped,
}

/// Two edges of a step by their ranks in its list, the earlier first, and
/// the value whose set-backs meet across the face they share, as [`bound`]
/// reads it of them alone.
pub(super) type Pair = (usize, usize, f64);

/// Each two of `edges`, read on `body` as `wedges`, parallel within
/// [`TANGENT`], that face each other across a face they share, the earlier
/// first, and the least distance across it from one to the other, at the
/// points they are read at or at an end of either: where they part by a
/// hair along it — a riser turned by one — the narrowest the face gets
/// between them, whichever end that is. Two that do not face each other
/// across it, or are not parallel, bound nothing here:
/// [`super::set_backs::bound`] reads where their set-backs would meet, and
/// they are laid in turn.
pub(super) fn facing(
    body: &brep::Body,
    edges: &[EdgeId],
    wedges: &[Wedge],
) -> Vec<(usize, usize, f64)> {
    let sides = |wedge: &Wedge| wedge.faces.into_iter().zip(wedge.into);
    let ends = |rank: usize| {
        let ends = body.edge(edges[rank]).ends.into_iter().flatten();
        ends.map(|end| body.vertex(end).point)
    };
    let mut facing = Vec::new();
    for (rank, one) in wedges.iter().enumerate() {
        for (later, other) in wedges.iter().enumerate().skip(rank + 1) {
            for (face, into) in sides(one) {
                for (_, towards) in sides(other).filter(|&(theirs, _)| theirs == face) {
                    let parallel = one.along.cross(other.along).length() <= TANGENT;
                    let across = into.dot(other.point - one.point);
                    if parallel && across > 0.0 && into.dot(towards) < 0.0 {
                        let narrowest = (ends(later).map(|end| into.dot(end - one.point)))
                            .chain(ends(rank).map(|end| towards.dot(end - other.point)))
                            .fold(across, f64::min);
                        facing.push((rank, later, narrowest));
                    }
                }
            }
        }
    }
    facing
}

/// The most value at which no two set-backs cross on a face two of the
/// edges share: for each two that face each other across it, the distance
/// between them over the set-backs a value of one makes from either.
pub(super) fn bound(
    wedges: &[Wedge],
    facing: &[(usize, usize, f64)],
    set_back: impl Fn(&Wedge) -> f64,
) -> f64 {
    facing
        .iter()
        .map(|&(one, other, across)| across / (set_back(&wedges[one]) + set_back(&wedges[other])))
        .fold(f64::INFINITY, f64::min)
}

/// The two edges facing each other, the earlier first, whose set-backs at
/// `taken` meet across the face they share, leaving none of it the kernel
/// could keep, and which `together` lays as one, the face gone, each with
/// the value it is laid at.
pub(super) fn meeting(
    wedges: &[Wedge],
    facing: &[(usize, usize, f64)],
    taken: f64,
    set_back: impl Fn(&Wedge) -> f64,
    together: Together,
    scale: Scale,
) -> Result<Vec<Pair>, Declined> {
    let pairs: Vec<Pair> = facing
        .iter()
        .filter_map(|&(one, other, across)| {
            let [first, second] = [&wedges[one], &wedges[other]];
            let left = across - taken * (set_back(first) + set_back(second));
            let value = bound(wedges, &[(one, other, across)], &set_back);
            let laid = match together {
                Together::Raised(_) => first.inwards == second.inwards,
                Together::Sloped => first.inwards != second.inwards,
            };
            (laid && too_short(left, scale)).then_some((one, other, value))
        })
        .collect();
    Ok(pairs)
}

/// Declined `Unsupported` where an edge of `pairs` that `together` raises as
/// one would meet two others, every face round it gone, but where they
/// close on themselves, as [`super::ring::closed`] reads them. Cuts beside
/// fills meeting in a chain — a flight's noses and feet — are laid as one
/// slope across it, as [`Exact::sloped`] lays them.
pub(super) fn apart(pairs: &[Pair], together: Together) -> Result<(), Declined> {
    if matches!(together, Together::Sloped) {
        return Ok(());
    }
    let ranks: BTreeSet<usize> = pairs
        .iter()
        .flat_map(|&(one, other, _)| [one, other])
        .collect();
    if ranks.len() < 2 * pairs.len() {
        return Err(Declined::Unsupported);
    }
    Ok(())
}

/// The ranks of the edges laid as one round with the edge of rank `rank`,
/// itself among them, as `pairs` join them one to the next, ascending.
pub(in crate::body::exact) fn together(rank: usize, pairs: &[Pair]) -> Vec<usize> {
    let mut joined = BTreeSet::from([rank]);
    loop {
        let more: Vec<usize> = (pairs.iter())
            .filter_map(
                |&(one, other, _)| match (joined.contains(&one), joined.contains(&other)) {
                    (true, false) => Some(other),
                    (false, true) => Some(one),
                    _ => None,
                },
            )
            .collect();
        if more.is_empty() {
            return joined.into_iter().collect();
        }
        joined.extend(more);
    }
}

impl Exact {
    /// The body with the two edges of `pair`, each with its wedge as read on
    /// `on`, the body as given, the earlier first, whose set-backs meet
    /// across the face they share, cut off as one by the prism `raise` raises
    /// over both at the value they meet at — or filled in as one, where their
    /// faces meet inwards, as [`Exact::filled`] fills: its first two walls,
    /// the later edge's then the earlier's, laid as one face where they lie on
    /// one surface, numbered as [`Exact::paired`] numbers it, meeting the
    /// faces `mitred` numbers in a mitre. The edges laid before move none of
    /// their planes, and the rounds at their ends may have moved both ends of
    /// each, so that no name finds them. Declined as [`Exact::cut_off`] says.
    pub(super) fn laid_together(
        &self,
        on: &brep::Body,
        pair: [(EdgeId, &Wedge); 2],
        number: u32,
        raise: Raise,
        mitred: &[u32],
    ) -> Result<Exact, Declined> {
        let [(_, first), (_, second)] = pair;
        let bounds = self.brep.bounds().ok_or(Declined::Unsupported)?;
        let prism = raise(first, second, self.brep.scale(), bounds)?;
        let (outline, frame, travel) = &prism;
        let raised = Exact::raised(outline, &[], *frame, *travel)?;
        let tool = Exact {
            brep: raised.brep.renumbered_by(|wall| match wall {
                0 | 1 => wall + MADE,
                2 | 3 => wall - MADE,
                other => other,
            }),
            ..raised
        };
        let cut = if first.inwards {
            self.filled(on, &pair, (&[], &[]), &prism, tool)?
        } else {
            self.cut_by(&Exact { next: 2, ..tool })?
        };
        cut.paired(self, &[first, second], &[self.next, number], mitred)
    }

    /// The body the edges of `wedges`, read on `before`, were laid on as
    /// one, each laying a wall, its faces left alone as [`Exact::cut_along`]
    /// reads it of all their wedges and of the faces laid before it that
    /// `mitred` numbers: what answered to each number from where the steps'
    /// numbers of `before` ended, in the order the walls were laid, answering
    /// to the number of `numbers` at that rank instead. Declined
    /// `Unsupported` where that reading fails.
    pub(super) fn paired(
        self,
        before: &Exact,
        wedges: &[&Wedge],
        numbers: &[u32],
        mitred: &[u32],
    ) -> Result<Exact, Declined> {
        if !self.cut_along(wedges, before, numbers.len() as u32, mitred, &[], &[]) {
            return Err(Declined::Unsupported);
        }
        let Exact {
            mut brep,
            next,
            pieces,
            roots,
            drawn,
            ..
        } = self;
        let laid: Vec<u32> = (before.next..).take(numbers.len()).collect();
        let faces: Vec<Vec<FaceId>> = (laid.iter())
            .map(|&number| {
                (brep.face_ids())
                    .filter(|&face| brep.numbers(face).contains(&number))
                    .collect()
            })
            .collect();
        for ((faces, &from), &to) in faces.iter().zip(&laid).zip(numbers) {
            brep.rename(faces, from, to);
        }
        let renamed = Exact::of(brep, next, pieces, roots);
        if let Some(drawn) = drawn.into_inner() {
            let _ = renamed.drawn.set(drawn);
        }
        Ok(renamed)
    }
}
