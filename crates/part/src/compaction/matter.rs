//! The steps of matter through a compaction: the areas they stand on and the
//! axis they turn about, said in the numbers the re-emitted sketch uses.

use std::collections::HashSet;

use cao_sketch::{Area, SegmentId};

use super::remap::{SketchIdMap, remap_area};
use super::variables::Renumbered;
use crate::history::{History, Operation, RevolutionAxis};
use crate::state::PartState;

/// Every segment a `Revolve` of this sketch turns around.
///
/// A revolution already ran against the segment's place at the time, and
/// stays there whatever is drawn afterwards — including erasing that segment,
/// an ordinary cleanup once it has served as an axis. So this can name a
/// segment `old_sketch.live_segments()` no longer does, and compaction has to
/// bring it back rather than leave the axis pointing at nothing.
pub(super) fn revolve_axis_segments(
    sketch_index: usize,
    operations: &[Operation],
) -> HashSet<SegmentId> {
    operations
        .iter()
        .filter_map(|operation| match operation {
            Operation::Revolve {
                sketch,
                axis: RevolutionAxis::Segment(segment),
                ..
            } if *sketch == sketch_index => Some(*segment),
            _ => None,
        })
        .collect()
}

/// An extrusion's areas, said in the numbers the re-emitted sketch uses.
///
/// A name is read against the drawing **as the old history leaves it** before
/// it is translated: it was written when the area was clicked, and the curves
/// it named may have been chamfered, rounded, trimmed or divided since.
/// Compaction drops those cuts and re-emits what they left, so a name still
/// speaking of what they took out would name nothing at all — which is how a
/// part came out of compaction with its matter gone.
///
/// A name the drawing no longer answers to is carried over as it stands: it
/// was already lost, and it stays lost.
pub(super) fn renamed(
    areas: &[Area],
    old: &PartState,
    maps: &[SketchIdMap],
    sketch: usize,
) -> Vec<Area> {
    let (Some(map), Some(drawing)) = (maps.get(sketch), old.sketches.get(sketch)) else {
        return areas.to_vec();
    };
    let regions = drawing.regions();
    areas
        .iter()
        .map(|area| {
            old.area_rank(sketch, area, &regions)
                .map(|rank| Area::of(&regions[rank], area.inside))
                .and_then(|now| remap_area(&now, map))
                .unwrap_or_else(|| area.clone())
        })
        .collect()
}

/// A blend step as compaction writes it, `raised_by` the number it is about
/// to be given: with its values as they now stand, or, when a change took
/// edges out of it, as first laid and then changed, so that it keeps the block
/// of face numbers the faces after it are named by.
pub(super) fn blend_written(
    first: &Operation,
    now: &Operation,
    renumbered: &Renumbered,
    raised_by: u32,
) -> Vec<Operation> {
    let written = |blend: &Operation| blend_renumbered(blend, renumbered);
    match (first, now) {
        (Operation::Blend { edges: laid, .. }, Operation::Blend { edges, .. })
            if edges.len() < laid.len() =>
        {
            vec![
                written(first),
                Operation::EditMatter {
                    raised_by,
                    to: Box::new(written(now)),
                },
            ]
        }
        _ => vec![written(now)],
    }
}

/// A blend, its size written in the variables compaction keeps.
pub(super) fn blend_renumbered(blend: &Operation, renumbered: &Renumbered) -> Operation {
    match blend {
        Operation::Blend { edges, shape, size } => Operation::Blend {
            edges: edges.clone(),
            shape: *shape,
            size: renumbered.formula(size),
        },
        other => other.clone(),
    }
}

/// Whether a step standing before the blend step of number `blend` was put
/// in after it was laid and before its change `change` was made: the change
/// may name that step's faces, and is then written where it was made rather
/// than with the step, whose number comes before that step's.
pub(super) fn put_in_before(history: &History, blend: u32, change: u32) -> bool {
    history
        .steps()
        .iter()
        .map_while(|step| {
            step.operations()
                .first()
                .filter(|opening| **opening != blend)
        })
        .any(|opening| (blend..change).contains(opening))
}
