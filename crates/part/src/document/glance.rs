//! A part looked at without being opened: what it holds and the picture it
//! carries, out of one read of its archive.

use std::path::Path;

use super::{PartDocument, StepCounts, archive_in, design, picture_entries};
use crate::picture::Picture;
use crate::ports::Files;

/// What a line listing a part shows of it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Glance {
    /// Nothing when the index of its design cannot be read.
    pub held: Option<StepCounts>,
    /// Nothing when it carries none, or a damaged one.
    pub picture: Option<Picture>,
}

impl PartDocument {
    /// What `step_counts_in` and `picture_in` say, out of one read of the
    /// file: a panel listing parts reads each one once.
    pub fn glance_in(files: &impl Files, path: &Path) -> Glance {
        let Some(mut archive) = archive_in(files, path) else {
            return Glance::default();
        };
        Glance {
            held: design::index_in(&mut archive)
                .ok()
                .map(|index| StepCounts::of(&index)),
            picture: picture_entries::read(&mut archive),
        }
    }
}
