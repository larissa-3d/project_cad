//! What a mirror holding its copy to the original asks for.

use cao_sketch::{ChosenAxis, Element};
use serde::{Deserialize, Serialize};

/// The elements to copy across an axis, the copy held symmetric to them and
/// carrying the rules they hold within themselves.
///
/// The elements are recorded rather than worked out again on replay, for the
/// reason `Trim` records its points: what a click took hold of depends on the
/// reach the cursor had at the time.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MirrorAsked {
    pub sketch: usize,
    pub elements: Vec<Element>,
    pub axis: ChosenAxis,
}
