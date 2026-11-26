//! What a part holds, counted from the index of its design alone.

use std::collections::BTreeSet;
use std::io::Cursor;
use std::path::Path;

use super::{PartDocument, design};
use crate::history::{Index, Step, StepKind};
use crate::ports::Files;

/// How many sketches and extrusions a part shows once opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepCounts {
    pub sketches: usize,
    pub extrusions: usize,
}

impl PartDocument {
    /// What a part holds, read from the index of its design on its own.
    ///
    /// Opening the part would replay its whole history, and the index already
    /// says what each step is: a panel listing a folder must pay neither the
    /// replay nor a read of every step's folder once a row.
    pub fn step_counts_in(files: &impl Files, path: &Path) -> Option<StepCounts> {
        let bytes = files.read(path).ok()?;
        let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).ok()?;
        let index = design::index_in(&mut archive).ok()?;
        Some(StepCounts::of(&index))
    }
}

impl StepCounts {
    /// Only the steps in effect are counted: one undone and kept to be redone
    /// is not shown once the part is opened.
    fn of(index: &Index) -> Self {
        let in_effect = in_effect(index);
        let count = |kind: StepKind| {
            index
                .steps
                .iter()
                .filter(|step| step.kind() == kind)
                .filter(|step| {
                    step.operations()
                        .first()
                        .is_some_and(|n| in_effect.contains(n))
                })
                .count()
        };
        Self {
            sketches: count(StepKind::Sketch),
            extrusions: count(StepKind::Extrusion),
        }
    }
}

/// The numbers of the operations in effect: the first `applied` of them in the
/// order they were handed out, the cursor `History::replay_order` replays to.
fn in_effect(index: &Index) -> BTreeSet<u32> {
    let mut numbers: Vec<u32> = index
        .steps
        .iter()
        .flat_map(Step::operations)
        .chain(&index.variables)
        .copied()
        .collect();
    numbers.sort_unstable();
    numbers.into_iter().take(index.applied).collect()
}
