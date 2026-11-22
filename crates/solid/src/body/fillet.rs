//! A fillet (#697): one straight edge of a body rounded off by a cylinder of
//! a radius tangent to the edge's two faces — or filled in by one, where they
//! meet inwards (#778) — or several edges in one step, at one radius (#698).

use std::slice;

use super::{Body, Matter};
use crate::brep::{Declined, EdgeName, Lost};

/// The body a fillet left, the radius it was asked, and the radius the
/// edges' faces gave it.
#[derive(Clone, Debug, PartialEq)]
pub struct Filleted {
    pub body: Body,
    pub asked: f64,
    pub taken: f64,
}

impl Body {
    /// The edge a name finds rounded off by a cylinder of `radius` tangent to
    /// its two faces, or of the largest radius they give — filled in by it
    /// where they meet inwards. The round it lays is numbered where the steps'
    /// numbers ended. Flats name no edge: every name is gone, as [`Body::lost`]
    /// says.
    pub fn fillet(&self, edge: &EdgeName, radius: f64) -> Result<Filleted, Declined> {
        self.fillet_edges(slice::from_ref(edge), radius)
    }

    /// The edge each name finds rounded off as [`Body::fillet`] rounds it, at
    /// `radius` or at the one largest radius every edge gives: alone, and for
    /// any two not meeting at a corner, with their set-backs short of reaching
    /// one point of a straight edge bounding a face of each — two parallel
    /// edges facing each other across a face they share, an edge ending on a
    /// face of another, two that are not parallel on a face they share. Each
    /// round is numbered on its own, in the order the
    /// names come, from where the steps' numbers ended — but two edges whose
    /// set-backs meet across the face they share, which one full round takes
    /// off or, where their faces meet inwards, one round bottom fills in, the
    /// face gone, answering to both their numbers. Such a round is laid at the
    /// radius whose set-backs meet across that face, which may stand a hair
    /// above the radius taken: its own radius is read off its face, never from
    /// [`Filleted::taken`]. Two edges meeting at a convex corner are mitred
    /// there, the corner's third edge left sharp, and three are closed there by
    /// a piece of sphere, at a concave corner — a pocket's bottom — a hollow
    /// one facing its centre, numbered after every round, by the lowest ranks
    /// of their edges in the list, then the next. Four or more meeting at a
    /// convex corner, every edge ending there named — a right pyramid's apex
    /// — are closed there by one piece of sphere too, where the faces round
    /// the corner all touch one sphere of the radius, and declined
    /// `Unsupported` where they touch none; so are three or more of the edges
    /// at a convex vertex of four or more named and not all — three at a hip's
    /// foot, three of a pyramid's four hips (#1031) — and two of them, at a
    /// hip's foot on a box or a slab, are mitred where they meet, the others
    /// left sharp, whatever the order (#942), and declined in every order
    /// where the round laid first, carried past the foot, ends on the other's
    /// walls where that one's round does not take it away (#1083). Two
    /// rounded off where they
    /// meet beside a concave edge left sharp — the L's top edges at its inner
    /// corner, a pocket's rim — each stop flush on the face the other runs
    /// along and are mitred past it on an ellipse, down to the concave edge, no
    /// face laid there, and two filled in beside a convex edge left sharp — two
    /// feet of a block standing on another — are mitred outside it the same
    /// way, and two filled in beside a concave edge left sharp, at a pocket's
    /// bottom corner, meet in a valley; at any other corner where an edge meets
    /// inwards they are declined `Unsupported`. The edge at the end of a full
    /// round, bounding the face that is gone and meeting both its edges at a
    /// corner, is rounded at the full round's own radius and mitred against it
    /// on two ellipses up to the crown, as against any round at a convex
    /// corner, and so are the two edges of a pointed end, or of a notch beside
    /// its concave upright, each bounding that face and meeting one of the full
    /// round's edges and the other, mitred against both their neighbours, the
    /// three mitres meeting at the crown — a point's tip, its upright named
    /// too, closed by a piece of sphere about the crown's axis, the two point
    /// edges' rounds meeting it in place of each other; one edge of a notch
    /// alone, the other left sharp, is rounded at that radius too, mitred
    /// against the full round at its shoulder and stopping flush on the sharp
    /// wall's plane past the notch's bottom, the sharp wall growing by the
    /// round's end and keeping its number, while the full round runs on whole
    /// and ends on that wall along an ellipse (#917); and any upright
    /// meeting one of the full round's edges at a convex corner where no
    /// third edge of the list ends — at either end of the part, under a full
    /// round along the top or over one along the floor, the two uprights at
    /// one end a standing full round themselves — is rounded at that radius
    /// and mitred against it on one ellipse, from where the full round leaves
    /// the face the two share to where the upright's round leaves its other
    /// face, the crown at a square
    /// end; named with the edge at the full round's end, the three are closed
    /// at their corner by a piece of sphere, and the tip of a tongue as wide
    /// as two radii, its end edge and both uprights named, is one quarter of a
    /// ball, the end edge's round closing to nothing between the two corners
    /// and the ball answering to its number and both corners'. An upright at a
    /// point's shoulder, named with the four top edges, is rounded at that
    /// radius too, the shoulder closed with the full round and the point's
    /// edge by a piece of the sphere about the crown's axis — at both
    /// shoulders two pieces of it, sharing the crown alone — and, named with
    /// the upright at the tip as well, the point's wall between them gone,
    /// the two corners close on one piece, the two uprights' rounds one face
    /// and the point edge's round closing to nothing between them (#911). The
    /// edge at a
    /// round bottom's end whose faces meet inwards too, bounding the face that
    /// is gone and meeting both its edges, is filled in at the round bottom's
    /// own radius, the two meeting
    /// in a valley on two ellipses down to where both touch that face's
    /// plane, as two fills at a pocket's bottom corner do (#838): the floor's
    /// edge on a pocket's end wall, with the round bottom along its floor,
    /// and the same edge named with the two uprights at that end, a standing
    /// round bottom meeting the edge at its foot. An upright at a pocket's
    /// end named with the round bottom along its floor is filled in at that
    /// radius too, meeting the round bottom in a valley where the floor's edge
    /// beside it, meeting inwards, is left sharp, and closed with it and that
    /// edge, named too,
    /// by a hollow piece of sphere; with both uprights and that edge, in a
    /// pocket as wide as two radii, the two round bottoms stop tangent to one
    /// hollow quarter of a ball, that edge's fill closing to nothing between
    /// its two corners, as at a tongue's tip (#919). The two floor edges of a
    /// pocket's pointed or notched end, named with the round bottom along its
    /// floor, are filled in at its radius too, each meeting it in a valley
    /// from its shoulder, where every edge ending there meets inwards, the
    /// round bottom ending flush on both end walls, and the two meeting each
    /// other in a valley at a point's tip, or, at a notch, mitred outside its
    /// upright, left sharp; the upright at a point's tip, named too, is filled
    /// in and closed with them by a hollow piece of sphere, and the round
    /// bottom alone, or at a point with an upright at a shoulder, ends flush on
    /// both end walls as well, each half on its own whichever edge is named
    /// first — at a stepped end too, whose two walls share no edge, where
    /// both stand square to the edges, the step kept whole (#921). An
    /// edge of a full round meeting any other edge of the list at a corner, a
    /// notch's two where its concave upright is named too (#914), an upright
    /// where a third edge of the list ends but one at its end — the edge
    /// across it, or a pointed end's (#837, #911) —, or
    /// one of a round bottom meeting any but the edge at its end, an upright
    /// there beside a floor edge meeting inwards or the two edges of a pointed
    /// or a notched end — a pit's upright, beside one meeting outwards (#919),
    /// one edge of a notched end alone, and both with an upright at a point's
    /// shoulder or the one at a notch's tip (#921) — or that edge where its
    /// faces meet outwards — an open slot's end, the
    /// rim above a standing round bottom — is declined `Unsupported`, and so is
    /// a round bottom whose two edges end on two faces where either has air
    /// past it — a slot widening past the end of one of them (#921) — or on
    /// the two walls of a stepped end where either stands at a slant to the
    /// edges (#1021). Rounds
    /// that would meet only on a curved edge are not measured, and declined
    /// `Unsupported` where they meet. The first edge declined declines the
    /// step.
    pub fn fillet_edges(&self, edges: &[EdgeName], radius: f64) -> Result<Filleted, Declined> {
        let Matter::Exact(exact) = &self.matter else {
            return Err(Declined::Lost(Lost::Gone));
        };
        let (exact, taken) = exact.filleted(edges, radius)?;
        Ok(Filleted {
            body: Body::of_exact(exact),
            asked: radius,
            taken,
        })
    }
}
