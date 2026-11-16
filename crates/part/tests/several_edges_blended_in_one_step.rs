//! A blend step holding several edges, as the part keeps it: laid in one
//! step, written and read back, undone and redone, and compacted whole.
//!
//! Closes #979 — the whole transcription is in
//! `crates/app/tests/several_edges_in_one_step.rs`; this is the part's half of
//! criterion 4, and of what the issue says must not break.
//! - « Appliquer » with the four top edges of a block chosen lays one step in
//!   the history, and the body afterwards is the one `Body::fillet_edges`
//!   (resp. `chamfer_edges`) gives for those four edges at that size — the
//!   step as the part lays it, by
//!   `a_blend_of_the_four_top_edges_lays_one_step_and_the_body_the_kernel_gives`;
//!   written and opened again, its design alone and with its cache, by
//!   `a_blend_of_the_four_top_edges_opens_again_as_it_was_saved_or_put_away`
//! - Undo, redo and the history's compaction of a blend step, which must not
//!   break — by `undo_takes_a_blend_of_four_edges_off_whole_and_redo_lays_it_again`
//!   and `a_history_compacted_keeps_a_blend_of_four_edges_whole`

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use cao_part::history::{BlendShape, Operation, PointRef, StepKind};
use cao_part::{ExtrusionMode, FileError, Files, PartDocument};
use cao_sketch::WorkPlane;
use cao_solid::{Body, EdgeName};
use glam::{DVec2, DVec3};

/// Planes and cylinders the exact kernel computes: what is left between two
/// replays of the same step is rounding.
const EXACT: f64 = 1e-9;

const LENGTH: f64 = 30.0;
const BREADTH: f64 = 10.0;
const HEIGHT: f64 = 20.0;
const SIZE: f64 = 2.0;

fn at_nine() -> chrono::DateTime<chrono::Utc> {
    "2026-11-16T09:00:00Z".parse().expect("a date")
}

/// A block 30 along X, 10 along Y, raised 20 from XY.
fn a_block() -> PartDocument {
    let mut part = PartDocument::new("Block", at_nine());
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    part.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(LENGTH, BREADTH)),
        construction: false,
    });
    let areas = part.areas_at(0, &[DVec2::new(LENGTH / 2.0, BREADTH / 2.0)]);
    part.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: HEIGHT.into(),
        mode: ExtrusionMode::Add,
    });
    part
}

/// The one face facing `normal` whose plane stands `level` along it.
fn face(body: &Body, normal: DVec3, level: f64) -> u32 {
    let faces: Vec<usize> = (0..body.faces_end())
        .filter(|face| {
            body.plane_of(*face).is_some_and(|plane| {
                plane.normal.distance(normal) < EXACT
                    && (plane.corners[0].dot(normal) - level).abs() < EXACT
            })
        })
        .collect();
    assert_eq!(
        faces.len(),
        1,
        "one face facing {normal} at {level}: {faces:?}"
    );
    faces[0] as u32
}

fn the_edge_between(body: &Body, first: u32, second: u32) -> EdgeName {
    let sides = [vec![first.min(second)], vec![first.max(second)]];
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides)
        .collect();
    assert_eq!(named.len(), 1, "one edge between {first} and {second}");
    named[0].clone()
}

/// The four edges of the block's top, towards -Y, +X, +Y and -X.
fn the_four_top_edges(body: &Body) -> Vec<EdgeName> {
    let top = face(body, DVec3::Z, HEIGHT);
    [
        (-DVec3::Y, 0.0),
        (DVec3::X, LENGTH),
        (DVec3::Y, BREADTH),
        (-DVec3::X, 0.0),
    ]
    .into_iter()
    .map(|(normal, level)| the_edge_between(body, top, face(body, normal, level)))
    .collect()
}

/// What the kernel gives for the four top edges of `body` at `SIZE`.
fn what_the_kernel_gives(body: &Body, shape: BlendShape) -> Body {
    let edges = the_four_top_edges(body);
    match shape {
        BlendShape::Chamfer => body.chamfer_edges(&edges, SIZE).map(|made| made.body),
        BlendShape::Fillet => body.fillet_edges(&edges, SIZE).map(|made| made.body),
    }
    .unwrap_or_else(|declined| {
        panic!("the kernel declines {shape:?} on the four top edges: {declined:?}")
    })
}

/// The four top edges blended at `SIZE` as one step: the number of its
/// operation.
fn blended_on_the_four_top_edges(part: &mut PartDocument, shape: BlendShape) -> u32 {
    let steps = part.history.steps().len();
    let edges = the_four_top_edges(part.body());
    let outcome = part.apply(Operation::Blend {
        edges,
        shape,
        size: SIZE.into(),
    });
    assert_eq!(outcome, None, "{shape:?} on the four top edges: built");
    assert_eq!(
        part.history.steps().len(),
        steps + 1,
        "{shape:?} on the four top edges: one new step",
    );
    let step = part.history.steps().last().expect("a step");
    assert_eq!(step.kind(), StepKind::Blend, "{shape:?}: its kind");
    assert_eq!(step.len(), 1, "{shape:?}: one operation under it");
    step.operations()[0]
}

fn faces_of(body: &Body) -> usize {
    (0..body.faces_end())
        .filter(|face| body.has_face(*face))
        .count()
}

fn assert_same_body(made: &Body, expected: &Body, what: &str) {
    assert!(
        (made.volume() - expected.volume()).abs() <= EXACT * expected.volume(),
        "{what}: {} where the kernel gives {}",
        made.volume(),
        expected.volume(),
    );
    assert_eq!(faces_of(made), faces_of(expected), "{what}: faces");
    assert_eq!(
        made.drawn_edges().len(),
        expected.drawn_edges().len(),
        "{what}: edges drawn",
    );
    let (made_low, made_high) = made.bounds().expect("some matter");
    let (low, high) = expected.bounds().expect("some matter");
    assert!(
        made_low.abs_diff_eq(low, EXACT) && made_high.abs_diff_eq(high, EXACT),
        "{what}: from {made_low:?} to {made_high:?}, where the kernel's stands from {low:?} to {high:?}",
    );
}

#[test]
fn a_blend_of_the_four_top_edges_lays_one_step_and_the_body_the_kernel_gives() {
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let mut part = a_block();
        let expected = what_the_kernel_gives(part.body(), shape);

        let step = blended_on_the_four_top_edges(&mut part, shape);

        assert_eq!(part.declined_because(step), None, "{shape:?}: built");
        assert_eq!(part.reduced_at(step), None, "{shape:?}: nothing reduced");
        assert!(part.body().is_exact(), "{shape:?}: still exact");
        assert_same_body(
            part.body(),
            &expected,
            &format!("{shape:?} on the four top edges"),
        );
    }
}

#[derive(Default)]
struct Memory(RefCell<HashMap<PathBuf, Vec<u8>>>);

impl Files for Memory {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        self.0
            .borrow()
            .get(path)
            .cloned()
            .ok_or_else(|| FileError::Absent(path.to_path_buf()))
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), FileError> {
        self.0
            .borrow_mut()
            .insert(path.to_path_buf(), bytes.to_vec());
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.0.borrow().contains_key(path)
    }
}

#[test]
fn a_blend_of_the_four_top_edges_opens_again_as_it_was_saved_or_put_away() {
    let files = Memory::default();
    let path = Path::new("/parts/Block.caopart");
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let mut part = a_block();
        let expected = what_the_kernel_gives(part.body(), shape);
        let step = blended_on_the_four_top_edges(&mut part, shape);

        for how in ["saved", "put away"] {
            let written = match how {
                "saved" => part.save(&files, path, at_nine()),
                _ => part.put_away(&files, path, at_nine()),
            };
            written.unwrap_or_else(|err| panic!("{shape:?} {how}: not written, {err:?}"));
            let opened = PartDocument::load(&files, path)
                .unwrap_or_else(|err| panic!("{shape:?} {how}: does not open, {err:?}"));
            let what = format!("{shape:?} on the four top edges, {how} and opened again");

            assert_eq!(opened.history.steps(), part.history.steps(), "{what}");
            assert_eq!(
                opened.history.operations(),
                part.history.operations(),
                "{what}: the four edges named as they were",
            );
            assert_eq!(opened.declined_because(step), None, "{what}: built");
            assert_same_body(opened.body(), &expected, &what);
        }
    }
}

#[test]
fn undo_takes_a_blend_of_four_edges_off_whole_and_redo_lays_it_again() {
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let mut part = a_block();
        let whole = part.body().clone();
        let expected = what_the_kernel_gives(part.body(), shape);
        let step = blended_on_the_four_top_edges(&mut part, shape);
        let laid = part.history.steps().to_vec();

        assert!(part.undo(), "{shape:?}: something to undo");
        assert!(
            part.history
                .replay_order()
                .iter()
                .all(|(number, _)| *number != step),
            "{shape:?} undone: the step is still replayed",
        );
        assert_same_body(
            part.body(),
            &whole,
            &format!("{shape:?} undone: the block whole"),
        );

        assert!(part.redo(), "{shape:?}: something to redo");
        let what = format!("{shape:?} redone");
        assert_eq!(part.history.steps(), laid.as_slice(), "{what}");
        assert_eq!(part.declined_because(step), None, "{what}: built");
        assert_same_body(part.body(), &expected, &what);
    }
}

#[test]
fn a_history_compacted_keeps_a_blend_of_four_edges_whole() {
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let mut part = a_block();
        let expected = what_the_kernel_gives(part.body(), shape);
        let edges = the_four_top_edges(part.body());
        blended_on_the_four_top_edges(&mut part, shape);

        part.compact_history();
        let what = format!("{shape:?} on the four top edges, the history compacted");

        let blends: Vec<&Operation> = part
            .history
            .operations()
            .iter()
            .filter(|operation| matches!(operation, Operation::Blend { .. }))
            .collect();
        assert_eq!(blends.len(), 1, "{what}: one blend left, {blends:?}");
        assert!(
            matches!(blends[0], Operation::Blend { edges: kept, .. } if *kept == edges),
            "{what}: the blend no longer names the four top edges, {:?}",
            blends[0],
        );
        let step = part
            .history
            .steps()
            .iter()
            .find(|step| step.kind() == StepKind::Blend)
            .unwrap_or_else(|| panic!("{what}: the step is gone"))
            .operations()[0];
        assert_eq!(part.declined_because(step), None, "{what}: built");
        assert_same_body(part.body(), &expected, &what);
    }
}
