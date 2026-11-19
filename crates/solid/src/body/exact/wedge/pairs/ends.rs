//! The edges of a step that a pair laid as one meets at its ends (#803): each
//! laid in turn against the pair's one prism, at the pair's value, or the
//! step declined where one meets the pair any other way.

use std::collections::{BTreeMap, BTreeSet};

use super::Pair;
use crate::body::exact::wedge::corners::{self, Beside, Closed};
use crate::body::exact::wedge::{Wedge, flush};
use crate::brep::{self, Declined, EdgeId, VertexId};

/// Each edge of `edges` bounding the face a pair of `pairs` whose faces meet
/// outwards meet across, of no pair itself, that meets both of the pair at a
/// corner — the edge at their end — or one of them and another such edge that
/// meets the other — each edge of a pointed or a notched end (#843) — with
/// the third edge of the list ending where those two meet, as `closed` holds
/// the corners three or more of them meet at — the upright at a point's tip, which
/// meets neither of the pair, the three closed there (#837) — or, of a pair
/// laid outwards, one of them alone and stops flush at its other end, no
/// other edge of the list ending there, as [`flush::stops`] reads it — one
/// edge of a V-notched end, the other left sharp, its round stopping on the
/// sharp wall's plane past the notch's bottom while the pair's prism, cut
/// whole, ends on that wall along an ellipse (#917) — and each edge
/// whose faces meet outwards, not bounding that face, that meets one of the
/// pair at a corner where no third edge of the list ends but one at their
/// end — the edge across it, or, of a pair laid outwards, any edge bounding
/// that face that reaches both of the pair, a pointed end's or a notched
/// end's, the latter held off only by the value, which stops short of the
/// full round wherever an upright stands at a notch's shoulder — the three
/// closed there, convex, `beside` holding the
/// corners that are not — an upright at either end, one of a standing full
/// round among them (#837), and the upright at a point's shoulder, closed
/// there with the full round and the point's edge (#911) — and, of a pair
/// not laid outwards, a round bottom, a
/// slope or a crease, each edge bounding the face they meet across that
/// meets both of the pair at a corner, whatever its own sense, and each such
/// upright — the front's upright at a slope's nose, meeting the slope's cut
/// (#814); one meeting its fill, the riser's upright at the foot's end,
/// makes a corner of mixed sense with it, the tread's side edge left out,
/// which the step goes round its odd edge first, as [`Exact::round_the_odd`]
/// lays it, the fill laid alone, so that the pair never forms here — whose
/// cut or fill is laid in turn against the one prism they are taken off or
/// filled in with and against its neighbours, by its rank, with the pair's
/// value, the radius that prism is raised at, so that it meets theirs where
/// they meet across the face even where the value taken stands a hair short
/// of it. Of a pair not laid outwards, an edge at their end is built only
/// where it meets inwards: the floor's edge on a pocket's end wall, or the
/// edge at the foot of a standing round bottom of two uprights at that end,
/// its fill joined to theirs in a valley (#838). Of a pair laid inwards, an
/// upright meeting inwards is built too: closed, as `closed` holds the
/// corner, with the edge at their end, or beside an edge left sharp where
/// every edge ending there meets inwards, a pocket's bottom corner, its fill
/// meeting theirs in a valley — the uprights at a round bottom's end, the two
/// there and the edge at their end closing both corners on one hollow quarter
/// of a ball in a pocket as wide as two radii (#919). An upright beside a slope
/// is built too, convex, its flat mitred against the slope's as two flats
/// meeting at a convex corner are (#814). A list mixing senses at their end
/// — a slope or a crease with the tread's end edge, a round bottom with a
/// convex end edge such as an open slot's end, a standing round bottom with
/// the rim above it — is let in here and refused by the laying, as
/// [`Exact::cut_along`] and [`Exact::sloped`] refuse it: a cut and a fill
/// meeting there do not commute. Declined `Unsupported` where
/// an edge of a pair meets another of `edges` at a corner any other way: a
/// full round meeting an edge bounding its face at one of its edges alone,
/// carried past its other end, or an upright where a third edge of the list
/// ends but one at their end (#837), a notched end with the upright at its
/// bottom, a corner of
/// mixed sense gone round that upright first (#914), a round bottom, a slope
/// or a crease meeting any other edge — an upright at a round bottom's end
/// beside an edge left out that meets outwards, a pit's where the floor drops
/// over it (#919), the edges of a pointed or a notched one (#921).
///
/// [`Exact::round_the_odd`]: crate::body::exact::Exact::round_the_odd
/// [`Exact::cut_along`]: crate::body::exact::Exact::cut_along
/// [`Exact::sloped`]: crate::body::exact::Exact::sloped
pub(in crate::body::exact::wedge) fn at_their_ends(
    body: &brep::Body,
    edges: &[EdgeId],
    wedges: &[Wedge],
    pairs: &[Pair],
    closed: &[Closed],
    beside: &[Beside],
) -> Result<BTreeMap<usize, f64>, Declined> {
    let members: BTreeSet<usize> = pairs
        .iter()
        .flat_map(|&(one, other, _)| [one, other])
        .collect();
    let meets = |member: usize, rank: usize| {
        rank != member && corners::meet(body, [edges[member], edges[rank]])
    };
    let mut ends = BTreeMap::new();
    for &(one, other, value) in pairs {
        let [first, second] = [&wedges[one], &wedges[other]];
        let outwards = !first.inwards && !second.inwards;
        let across = first.faces.iter().find(|face| second.faces.contains(face));
        let bounding = |rank: usize| {
            !members.contains(&rank) && across.is_some_and(|face| wedges[rank].faces.contains(face))
        };
        let thirds = |rank: usize, tip: usize| -> Vec<usize> {
            (closed.iter())
                .filter(|(_, at, _)| at.contains(&rank) && at.contains(&tip))
                .flat_map(|(_, at, _)| at.iter().copied())
                .filter(|third| ![rank, tip].contains(third))
                .collect()
        };
        let tips = |rank: usize, far: usize| {
            (0..edges.len())
                .filter(move |&tip| bounding(tip) && meets(rank, tip) && meets(far, tip))
        };
        let reaching =
            |rank: usize, far: usize| meets(far, rank) || tips(rank, far).next().is_some();
        let at_the_end = |rank: usize| bounding(rank) && meets(one, rank) && meets(other, rank);
        let ending = |rank: usize| {
            if outwards {
                bounding(rank) && reaching(rank, one) && reaching(rank, other)
            } else {
                at_the_end(rank)
            }
        };
        let upright = |rank: usize| {
            let member = if meets(one, rank) { one } else { other };
            let filled = first.inwards && second.inwards && wedges[rank].inwards;
            meets(one, rank) != meets(other, rank)
                && (filled || !wedges[rank].inwards)
                && !across.is_some_and(|face| wedges[rank].faces.contains(face))
                && (closed.iter())
                    .filter(|(_, at, _)| at.contains(&rank) && at.contains(&member))
                    .all(|(_, at, _)| {
                        (at.iter()).all(|&third| [rank, member].contains(&third) || ending(third))
                    })
                && (beside.iter())
                    .filter(|(_, at)| at.contains(&rank) && at.contains(&member))
                    .all(|&(corner, _)| filled && corners::hollow(body, corner))
        };
        let stopping = |rank: usize| {
            let alone = |end: VertexId| {
                (0..edges.len()).all(|listed| {
                    listed == rank
                        || !body
                            .edge(edges[listed])
                            .ends
                            .is_some_and(|at| at.contains(&end))
                })
            };
            outwards
                && bounding(rank)
                && meets(one, rank) != meets(other, rank)
                && (flush::stops(body, edges[rank], &wedges[rank]).iter())
                    .any(|stop| alone(stop.end))
        };
        for rank in (0..edges.len()).filter(|&rank| meets(one, rank) || meets(other, rank)) {
            if !ending(rank) && !upright(rank) && !stopping(rank) {
                return Err(Declined::Unsupported);
            }
            ends.insert(rank, value);
            let at_the_tip = [one, other].into_iter().flat_map(|far| tips(rank, far));
            ends.extend(
                at_the_tip
                    .flat_map(|tip| thirds(rank, tip))
                    .map(|third| (third, value)),
            );
        }
    }
    Ok(ends)
}
