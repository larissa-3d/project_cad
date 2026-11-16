//! The volume a step of matter raises or takes away, shown while its panel is
//! still open: its areas extruded or turned alone, neither joined to the part
//! nor cut from it, and nothing recorded. The part hands the volume; the view
//! colours it by the mode.
//!
//! Closes #740 — the whole transcription is in
//! `crates/app/tests/extruding_in_a_panel.rs`; these bullets are the half of
//! criterion 5 the part answers, before any frame draws it.
//! - the view draws the extruded volume alone, not fused into the part — the
//!   part's half: a 10 by 10 area extruded 30 mm across half of a block is a
//!   volume of 3000 standing where the area is raised, the block left as it
//!   was and no step recorded, and the same volume for an addition and a cut —
//!   `an_extrusion_previewed_across_a_block_is_its_volume_alone_whatever_its_mode`
//! - so is a revolution, about U, V or a drawn trait: the strip beyond the
//!   block turned whole about each is the ring, the disc or the tube the
//!   arithmetic gives, standing where it turns, the block left as it was and
//!   no step recorded, and the same for an addition and a cut —
//!   `a_revolution_previewed_across_a_block_is_its_turn_alone_whatever_its_mode`

use std::f64::consts::PI;

use cao_part::{ExtrusionMode, Operation, PartDocument, PointRef, RevolutionAxis};
use cao_sketch::{SegmentId, SketchAxis, WorkPlane};
use cao_solid::Body;
use chrono::Utc;
use glam::{DVec2, DVec3};

/// How far from the arithmetic a volume of flat-sided matter may stand.
const VOLUME: f64 = 1e-6;

/// How far a corner of a box may stand from where it was drawn.
const PLACE: f64 = 1e-9;

/// A point inside each half of the square (0, 0)–(10, 10).
const ON_THE_BLOCK: [DVec2; 2] = [DVec2::new(2.0, 5.0), DVec2::new(7.0, 5.0)];

/// A point inside each half of the square (5, 0)–(15, 10): the right half of
/// the block, and the strip beyond it.
const ACROSS_THE_BLOCK_S_HALF: [DVec2; 2] = [DVec2::new(7.0, 5.0), DVec2::new(12.0, 5.0)];

/// A point inside the strip (10, 0)–(15, 10), beyond the block.
const BEYOND_THE_BLOCK: [DVec2; 1] = [DVec2::new(12.0, 5.0)];

fn rectangle(document: &mut PartDocument, corner: DVec2, opposite: DVec2) {
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(corner),
        opposite: PointRef::New(opposite),
        construction: false,
    });
}

fn extruded(
    document: &PartDocument,
    inside: &[DVec2],
    distance: f64,
    mode: ExtrusionMode,
) -> Operation {
    Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, inside),
        distance: distance.into(),
        mode,
    }
}

/// A whole turn of these areas about that axis.
fn turned(
    document: &PartDocument,
    inside: &[DVec2],
    axis: RevolutionAxis,
    mode: ExtrusionMode,
) -> Operation {
    Operation::Revolve {
        sketch: 0,
        areas: document.areas_at(0, inside),
        axis,
        angle: 360.0.into(),
        mode,
    }
}

/// A trait drawn along V from (20, 0) to (20, 10), crossing nothing, by the
/// number the sketch gave it.
fn a_trait_along_v_at_20(document: &mut PartDocument) -> SegmentId {
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(20.0, 0.0)),
        end: PointRef::New(DVec2::new(20.0, 10.0)),
        construction: false,
    });
    SegmentId(document.sketches()[0].segments().len() - 1)
}

/// A block 10 by 10 by 10 at the origin, raised from the square (0, 0)–(10,
/// 10) on XY; drawn on the same sketch and left unraised, the square (5, 0)–
/// (15, 10) across its right half.
fn a_block_with_a_square_across_its_half() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    rectangle(&mut document, DVec2::ZERO, DVec2::splat(10.0));
    rectangle(&mut document, DVec2::new(5.0, 0.0), DVec2::new(15.0, 10.0));
    let block = extruded(&document, &ON_THE_BLOCK, 10.0, ExtrusionMode::Add);
    document.apply(block);
    assert_volume(document.body(), 1000.0, "the block");
    document
}

fn assert_volume(body: &Body, expected: f64, what: &str) {
    let volume = body.volume();
    assert!(
        (volume - expected).abs() <= VOLUME * expected.max(1.0),
        "{what} holds {volume}, not {expected}",
    );
}

fn assert_bounds(body: &Body, low: DVec3, high: DVec3, what: &str) {
    let (from, to) = body
        .bounds()
        .unwrap_or_else(|| panic!("{what} holds no matter"));
    assert!(
        from.distance(low) < PLACE && to.distance(high) < PLACE,
        "{what} spans {from} to {to}, not {low} to {high}",
    );
}

#[test]
fn an_extrusion_previewed_across_a_block_is_its_volume_alone_whatever_its_mode() {
    let document = a_block_with_a_square_across_its_half();
    let steps = document.history.applied_operations().len();

    for mode in [ExtrusionMode::Add, ExtrusionMode::Cut] {
        let extrusion = extruded(&document, &ACROSS_THE_BLOCK_S_HALF, 30.0, mode);

        let previewed = document
            .tool_of(&extrusion)
            .unwrap_or_else(|| panic!("an extrusion as {mode:?} previews no volume"));

        let what = format!("the volume previewed as {mode:?}");
        assert_volume(&previewed, 3000.0, &what);
        assert_bounds(
            &previewed,
            DVec3::new(5.0, 0.0, 0.0),
            DVec3::new(15.0, 10.0, 30.0),
            &what,
        );
    }
    assert_volume(document.body(), 1000.0, "the block, once previewed across");
    assert_eq!(
        document.history.applied_operations().len(),
        steps,
        "previewing an extrusion recorded a step",
    );
}

#[test]
fn a_revolution_previewed_across_a_block_is_its_turn_alone_whatever_its_mode() {
    let mut document = a_block_with_a_square_across_its_half();
    let drawn = a_trait_along_v_at_20(&mut document);
    let steps = document.history.applied_operations().len();

    for (axis, turn, low, high) in [
        (
            RevolutionAxis::Sketch(SketchAxis::V),
            PI * (15.0 * 15.0 - 10.0 * 10.0) * 10.0,
            DVec3::new(-15.0, 0.0, -15.0),
            DVec3::new(15.0, 10.0, 15.0),
        ),
        (
            RevolutionAxis::Sketch(SketchAxis::U),
            PI * 10.0 * 10.0 * 5.0,
            DVec3::new(10.0, -10.0, -10.0),
            DVec3::new(15.0, 10.0, 10.0),
        ),
        (
            RevolutionAxis::Segment(drawn),
            PI * (10.0 * 10.0 - 5.0 * 5.0) * 10.0,
            DVec3::new(10.0, 0.0, -10.0),
            DVec3::new(30.0, 10.0, 10.0),
        ),
    ] {
        for mode in [ExtrusionMode::Add, ExtrusionMode::Cut] {
            let revolution = turned(&document, &BEYOND_THE_BLOCK, axis, mode);

            let previewed = document.tool_of(&revolution).unwrap_or_else(|| {
                panic!("a revolution about {axis:?} as {mode:?} previews no volume")
            });

            let what = format!("the turn about {axis:?} previewed as {mode:?}");
            assert_volume(&previewed, turn, &what);
            assert_bounds(&previewed, low, high, &what);
        }
    }
    assert_volume(document.body(), 1000.0, "the block, once previewed across");
    assert_eq!(
        document.history.applied_operations().len(),
        steps,
        "previewing a revolution recorded a step",
    );
}
