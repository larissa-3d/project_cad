//! Edges of the part chamfered or rounded together by the kernel, at the size
//! asked or at the most their faces give.

use cao_solid::{Declined, EdgeName};
use serde::{Deserialize, Serialize};

use crate::formula::Formula;
use crate::history::BlendShape;
use crate::outcome::Outcome;
use crate::state::PartState;

/// Closer than this to the size asked, the size taken is the same: the rest
/// is rounding.
const ONE_PLACE: f64 = 1e-9;

/// The face numbers a step of several edges takes, per edge, built or not.
/// The kernel numbers one face per edge, then the pieces closing its corners:
/// a closing, an odd corner's piece and its held end take no more numbers at
/// a corner than there are listed edges ending there, two per edge at most. A
/// build laying more is declined `Unsupported`, a limit of the part rather
/// than of the kernel, instead of shifting every face numbered after it.
const NUMBERS_PER_EDGE: usize = 3;

/// The face numbers a step of `edges` edges takes, so that the faces after it
/// keep theirs whatever becomes of it. A step of one edge keeps the one number
/// it always took, which parts saved before name the faces after it by.
fn numbers_taken(edges: usize) -> usize {
    match edges {
        0 | 1 => 1,
        several => NUMBERS_PER_EDGE * several,
    }
}

/// A chamfer or a fillet the faces of its edge could not give the size it
/// asked, laid at the most they give. In millimetres.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reduced {
    pub asked: f64,
    pub taken: f64,
}

impl PartState {
    /// Chamfers or rounds edges together, noting the faces it lays as a step
    /// of matter notes its own, so that the faces after it are found again by
    /// their numbers whatever becomes of it.
    pub(crate) fn blend(
        &mut self,
        edges: &[EdgeName],
        shape: BlendShape,
        size: &Formula,
    ) -> Option<Outcome> {
        let mut outcome = None;
        self.raising(|state| outcome = state.blended(edges, shape, size));
        outcome
    }

    fn blended(
        &mut self,
        edges: &[EdgeName],
        shape: BlendShape,
        size: &Formula,
    ) -> Option<Outcome> {
        let from = self.body.faces_end();
        let first_laid = self.replay.blends_first_laid.get(&self.replaying);
        let block = numbers_taken(edges.len().max(first_laid.copied().unwrap_or(0)));
        let Some(size) = self.size(size, |size| size >= 1e-6) else {
            self.body.count_past_cut(block);
            return None;
        };
        let edges: Vec<EdgeName> = edges
            .iter()
            .map(|edge| self.edge_named_by(edge, self.replaying))
            .collect();
        let scale = self.scale();
        let blended = match shape {
            BlendShape::Chamfer => self
                .body
                .chamfer_edges(&edges, size / scale)
                .map(|cut| (cut.body, cut.taken)),
            BlendShape::Fillet => self
                .body
                .fillet_edges(&edges, size / scale)
                .map(|round| (round.body, round.taken)),
        };
        let blended = blended.and_then(|(body, taken)| {
            let laid = body.faces_end().saturating_sub(from);
            match edges.len() > 1 && laid > block {
                true => Err(Declined::Unsupported),
                false => Ok((body, taken)),
            }
        });
        match blended {
            Ok((body, taken)) => {
                self.body = body;
                let short = (from + block).saturating_sub(self.body.faces_end());
                self.body.count_past_cut(short);
                let taken = taken * scale;
                if size - taken > ONE_PLACE * size.max(1.0) {
                    self.reduced
                        .insert(self.replaying, Reduced { asked: size, taken });
                }
                None
            }
            Err(reason) => {
                self.declined_with(reason, |body| body.count_past_cut(block));
                Some(Outcome::BlendDeclined(reason))
            }
        }
    }

    /// The edge the operation of number `recorded_by` named, its faces found
    /// again as the part now numbers them, as a drawing's face is.
    fn edge_named_by(&self, edge: &EdgeName, recorded_by: u32) -> EdgeName {
        let side = |numbers: &Vec<u32>| -> Vec<u32> {
            numbers
                .iter()
                .map(|&number| self.face_named_by(number as usize, recorded_by) as u32)
                .collect()
        };
        EdgeName {
            between: edge.between.each_ref().map(side),
            ends: edge
                .ends
                .each_ref()
                .map(|faces| faces.iter().map(side).collect()),
            ..edge.clone()
        }
    }
}
