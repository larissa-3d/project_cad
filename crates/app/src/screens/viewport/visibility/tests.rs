//! What app · screens/viewport/visibility.rs is held to.
//!
//! Closes #734 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests.rs`; these bullets are the
//! ones the canvas's state answers, before any row or frame reads it.
//! - each volume's visibility cycles opaque → « transparente » → invisible on
//!   its own — `a_volume_s_visibility_cycles_opaque_see_through_invisible_and_round_again`;
//!   it belongs to the step's operation, not to its place in the history, as a
//!   sketch's eye does —
//!   `a_volume_s_visibility_belongs_to_its_operation_not_to_its_place_in_the_history`
//! - « Tout rendre opaque » sets every volume back to opaque, and says
//!   whether it would do anything — it does nothing when they all are —
//!   `making_everything_opaque_puts_every_volume_back_to_opaque`
//! - criteria 4 and 6 as first written, « TOUT LE CORPS » and the faces no
//!   volume made — no test: both struck on 2026-11-10 by Tom, and the tests
//!   that held them deleted with them
//!
//! Closes #740 — the whole transcription is in
//! `crates/app/tests/extruding_in_a_panel.rs`; this is the half of criterion 5
//! that keeps the redrawing cheap, the cost #583 named.
//! - The view draws the extruded volume alone, not fused into the part,
//!   see-through, green for an addition and red for a cut, and redraws it at
//!   every keystroke — the canvas's half: frame after frame, the volume of a
//!   step that has not changed on a part that has not either is the one
//!   already built, and a step retyped or a part changed builds it again —
//!   `the_volume_previewed_is_built_again_only_once_its_step_or_the_part_changes`

use std::rc::Rc;

use cao_part::{ExtrusionMode, PointRef, VolumeState};
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::DVec2;

use super::*;

/// Where the block stands in the history of `a_block_with_a_hole`.
const THE_BLOCK: usize = 3;
/// Where the cut that bores the hole stands.
const THE_HOLE: usize = 4;

/// A block 10 by 20 raised 4 from XY, and a cut through it, round, at its
/// middle: two volumes, an add and a cut.
fn a_block_with_a_hole() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(5.0, 10.0)),
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(1.0, 1.0), DVec2::new(5.0, 10.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    document.apply(a_cut_through_the_middle(&document, 4.0));
    let steps = document.history.applied_operations();
    assert!(
        matches!(
            steps.get(THE_BLOCK),
            Some(Operation::Extrude {
                mode: ExtrusionMode::Add,
                ..
            })
        ) && matches!(
            steps.get(THE_HOLE),
            Some(Operation::Extrude {
                mode: ExtrusionMode::Cut,
                ..
            })
        ) && steps.len() == THE_HOLE + 1,
        "the fixture wants the block then the hole, last: {steps:?}",
    );
    assert!(!document.body().is_empty(), "the block is raised");
    document
}

/// The disc at the block's middle taken out, that deep from XY.
fn a_cut_through_the_middle(document: &PartDocument, depth: f64) -> Operation {
    Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 10.0)]),
        distance: depth.into(),
        mode: ExtrusionMode::Cut,
    }
}

#[test]
fn a_volume_s_visibility_cycles_opaque_see_through_invisible_and_round_again() {
    let document = a_block_with_a_hole();
    let mut state = ViewportState::default();
    assert_eq!(
        state.volume_mode(&document, THE_HOLE),
        VolumeState::Opaque,
        "a volume opens opaque",
    );

    let mut seen = Vec::new();
    for _ in 0..3 {
        state.next_volume_mode(&document, THE_HOLE);
        seen.push(state.volume_mode(&document, THE_HOLE));
    }

    assert_eq!(
        seen,
        [
            VolumeState::SeeThrough,
            VolumeState::Invisible,
            VolumeState::Opaque
        ],
        "opaque, then see-through, then invisible, then opaque again",
    );
    state.next_volume_mode(&document, THE_HOLE);
    assert_eq!(
        state.volume_mode(&document, THE_BLOCK),
        VolumeState::Opaque,
        "the cut made see-through, the block is no longer opaque",
    );
}

#[test]
fn a_volume_s_visibility_belongs_to_its_operation_not_to_its_place_in_the_history() {
    let mut document = a_block_with_a_hole();
    let mut state = ViewportState::default();
    state.next_volume_mode(&document, THE_HOLE);
    assert_eq!(
        state.volume_mode(&document, THE_HOLE),
        VolumeState::SeeThrough
    );

    document.rewind_to(THE_HOLE);
    document.apply(a_cut_through_the_middle(&document, 2.0));

    assert!(
        matches!(
            document.history.applied_operations().get(THE_HOLE),
            Some(Operation::Extrude {
                mode: ExtrusionMode::Cut,
                ..
            })
        ),
        "the fixture wants a new cut where the old one stood",
    );
    assert_eq!(
        state.volume_mode(&document, THE_HOLE),
        VolumeState::Opaque,
        "a new cut recorded where an undone one stood inherits its state",
    );
}

#[test]
fn making_everything_opaque_puts_every_volume_back_to_opaque() {
    let document = a_block_with_a_hole();
    let mut state = ViewportState::default();
    assert!(
        state.all_opaque(&document),
        "a part just opened, every volume opaque, does not read as all opaque"
    );

    state.next_volume_mode(&document, THE_BLOCK);
    state.next_volume_mode(&document, THE_HOLE);
    state.next_volume_mode(&document, THE_HOLE);
    assert!(
        !state.all_opaque(&document),
        "the block see-through and the cut invisible read as all opaque",
    );

    state.make_all_opaque();

    assert_eq!(
        [THE_BLOCK, THE_HOLE].map(|step| state.volume_mode(&document, step)),
        [VolumeState::Opaque; 2],
        "« Tout rendre opaque » leaves a volume in another state",
    );
    assert!(state.all_opaque(&document));
}

#[test]
fn the_volume_previewed_is_built_again_only_once_its_step_or_the_part_changes() {
    let mut document = a_block_with_a_hole();
    let state = ViewportState::default();
    let six_deep = a_cut_through_the_middle(&document, 6.0);
    let previewed = |document: &PartDocument, step: &Operation| {
        state
            .previewed_volume(document, step)
            .unwrap_or_else(|| panic!("the cut through the middle previews no volume: {step:?}"))
    };

    let first = previewed(&document, &six_deep);
    let next_frame = previewed(&document, &six_deep);
    assert!(
        Rc::ptr_eq(&first, &next_frame),
        "the same step on the same part, the next frame builds its volume again",
    );

    let eight_deep = a_cut_through_the_middle(&document, 8.0);
    let retyped = previewed(&document, &eight_deep);
    assert!(
        !Rc::ptr_eq(&first, &retyped) && retyped.volume() > first.volume(),
        "the depth retyped from 6 to 8, the volume previewed is the one 6 deep: {} then {}",
        first.volume(),
        retyped.volume(),
    );

    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(30.0, 0.0)),
        opposite: PointRef::New(DVec2::new(40.0, 10.0)),
        construction: false,
    });
    let after_a_change = previewed(&document, &eight_deep);
    assert!(
        !Rc::ptr_eq(&retyped, &after_a_change),
        "the part changed under the same step, the volume previewed is the one built before",
    );
}
