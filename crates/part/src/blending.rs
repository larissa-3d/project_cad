//! Edges of the part chamfered or rounded together by the kernel, at the size
//! asked or at the most their faces give.

use cao_solid::{Body, Declined, EdgeName};
use serde::{Deserialize, Serialize};

use crate::document::PartDocument;
use crate::formula::Formula;
use crate::history::{BlendShape, Operation};
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
        let naming = self.named_against(self.replaying);
        let edges: Vec<EdgeName> = edges
            .iter()
            .map(|edge| self.edge_named_by(edge, naming))
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

    /// The number of the operation the edges of the step of that number were
    /// named by: its change in effect, taken on the part as it stood then, or
    /// the step itself.
    fn named_against(&self, step: u32) -> u32 {
        let changed = self.replay.changes_named_by.get(&step);
        changed.copied().unwrap_or(step)
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

impl PartDocument {
    /// The edges the blend step of that number takes that the part as it
    /// stood before it still has, named as the part numbers its faces now:
    /// what the step offers once reopened, all in the naming a change of it
    /// is recorded in.
    pub fn edges_before(&self, step: u32) -> Vec<EdgeName> {
        let Some(Operation::Blend { edges, .. }) = self.raised_as(step) else {
            return Vec::new();
        };
        let before = self.state_before(step);
        let naming = before.named_against(step);
        let found = before.body.edge_names();
        edges
            .iter()
            .map(|edge| before.edge_named_by(edge, naming))
            .filter(|edge| found.contains(edge))
            .collect()
    }
}

/// The part as it stood just before a blend step, every volume opaque, kept
/// to lay the step on as each change of it would be recorded.
#[derive(Clone, Debug)]
pub struct StepReopened {
    before: PartState,
    step: u32,
    /// The number the change would be recorded under, which its edges are
    /// named against.
    change: u32,
}

/// The step laid on the part before it, and what the kernel made of it: why
/// it declined it, or the size it took when it gave less.
#[derive(Clone, Debug)]
pub struct Tried {
    pub body: Body,
    pub declined: Option<Declined>,
    pub reduced: Option<Reduced>,
}

impl PartDocument {
    /// The blend step of that number reopened: the part as it stood before
    /// it, replayed once.
    pub fn step_reopened(&self, step: u32) -> StepReopened {
        StepReopened {
            before: self.state_before(step),
            step,
            change: self.history.next_number(),
        }
    }
}

impl StepReopened {
    /// The part as it stood before the step.
    pub fn body(&self) -> &Body {
        &self.before.body
    }

    /// The step laid as `blend`, on the part before it, as the replay of its
    /// change would lay it: under its own number, in the block of face
    /// numbers it was first laid with, its edges named as the change names
    /// them.
    pub fn laid(&self, blend: &Operation) -> Tried {
        let mut state = self.before.clone();
        state.replay.changes_named_by.insert(self.step, self.change);
        let outcome = state.applied_live(self.step, blend);
        Tried {
            declined: match outcome {
                Some(Outcome::BlendDeclined(reason)) => Some(reason),
                _ => None,
            },
            reduced: state.reduced.get(&self.step).copied(),
            body: state.body,
        }
    }
}
