//! Two points held mirror images of each other across an axis: what keeps a
//! mirrored copy the image of what it was made from.

use serde::{Deserialize, Serialize};

use crate::axis::ChosenAxis;
use crate::sketch::PointId;

/// A point and its twin, each the reflection of the other across the axis.
///
/// Neither drives the other: the rule pulls both, and a trait serving as the
/// axis with them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symmetry {
    pub point: PointId,
    pub twin: PointId,
    pub axis: ChosenAxis,
}
