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
//! - found in review, beyond the issue's eight: a blend of the three edges
//!   meeting at a corner lays more faces than it has edges, and a sketch on a
//!   face raised after it stays on that face whatever becomes of the blend —
//!   its size set to nothing, by
//!   `a_corner_s_blend_set_to_nothing_leaves_a_sketch_laid_after_it_on_its_face`;
//!   declined by the kernel on a later replay, an edge parted upstream, by
//!   `a_corner_s_blend_declined_on_replay_leaves_a_sketch_laid_after_it_on_its_face`

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use cao_part::history::FaceAnchor;
use cao_part::history::{BlendShape, Operation, PointRef, StepKind};
use cao_part::{
    ExtrusionMode, FileError, Files, Formula, PartDocument, VariableChange, VariableId,
};
use cao_sketch::WorkPlane;
use cao_solid::{Body, Declined, EdgeName};
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

/// The one face facing `normal`, standing `level` along it, with every corner
/// at `from_x` or further along X.
fn face_from(body: &Body, normal: DVec3, level: f64, from_x: f64) -> usize {
    let faces: Vec<usize> = (0..body.faces_end())
        .filter(|face| {
            body.plane_of(*face).is_some_and(|plane| {
                plane.normal.distance(normal) < EXACT
                    && (plane.corners[0].dot(normal) - level).abs() < EXACT
                    && plane
                        .corners
                        .iter()
                        .all(|corner| corner.x >= from_x - EXACT)
            })
        })
        .collect();
    assert_eq!(
        faces.len(),
        1,
        "one face facing {normal} at {level} from x = {from_x}: {faces:?}"
    );
    faces[0]
}

/// The block, its size held by the variable `size` at `SIZE`, notched 5 high
/// at its foot across its back; the three edges meeting at its corner
/// (30, 0, 0) blended in one step at `size`; then a boss 5 high raised beside
/// it, from x = 40, and a sketch laid on the boss's top.
struct ACornerBlended {
    part: PartDocument,
    blend: u32,
    notch: u32,
    notches: usize,
    on_the_boss: usize,
}

fn a_corner_blended(shape: BlendShape) -> ACornerBlended {
    let mut part = a_block();
    part.apply(Operation::Variable(VariableChange::Added {
        name: "size".to_string(),
        formula: Formula::Number(SIZE),
    }));
    let rectangle = |part: &mut PartDocument, sketch, corner: DVec2, opposite: DVec2| {
        part.apply(Operation::AddRectangle {
            sketch,
            corner: PointRef::New(corner),
            opposite: PointRef::New(opposite),
            construction: false,
        });
    };
    let on_xy = Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    };
    part.apply(on_xy.clone());
    let notches = part.sketches().len() - 1;
    rectangle(
        &mut part,
        notches,
        DVec2::new(10.0, 7.0),
        DVec2::new(20.0, 13.0),
    );
    rectangle(
        &mut part,
        notches,
        DVec2::new(10.0, -3.0),
        DVec2::new(20.0, 3.0),
    );
    part.apply(Operation::Extrude {
        sketch: notches,
        areas: part.areas_at(notches, &[DVec2::new(15.0, 10.0)]),
        distance: 5.0.into(),
        mode: ExtrusionMode::Cut,
    });
    let notch = part.history.steps().last().expect("the notch").operations()[0];

    let body = part.body();
    let [foot, front, side] = [(-DVec3::Z, 0.0), (-DVec3::Y, 0.0), (DVec3::X, LENGTH)]
        .map(|(normal, level)| face(body, normal, level));
    let edges = vec![
        the_edge_between(body, foot, front),
        the_edge_between(body, foot, side),
        the_edge_between(body, front, side),
    ];
    let faces_before = body.faces_end();
    let size = part.variables().read("size").expect("size reads");
    let outcome = part.apply(Operation::Blend { edges, shape, size });
    assert_eq!(
        outcome, None,
        "{shape:?} of the three edges at the corner: built"
    );
    let blend = part.history.steps().last().expect("the blend").operations()[0];
    assert!(
        part.body().faces_end() > faces_before + 3,
        "{shape:?} of the three edges at the corner numbers {} faces past {faces_before}: \
         the fixture means a corner laying more faces than it has edges",
        part.body().faces_end() - faces_before,
    );

    part.apply(on_xy);
    let boss = part.sketches().len() - 1;
    rectangle(
        &mut part,
        boss,
        DVec2::new(40.0, 0.0),
        DVec2::new(50.0, 10.0),
    );
    part.apply(Operation::Extrude {
        sketch: boss,
        areas: part.areas_at(boss, &[DVec2::new(45.0, 5.0)]),
        distance: 5.0.into(),
        mode: ExtrusionMode::Add,
    });
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: Some(FaceAnchor {
            face: face_from(part.body(), DVec3::Z, 5.0, 40.0),
            up: DVec3::Y,
        }),
    });
    let on_the_boss = part.sketches().len() - 1;
    assert_on_the_boss_s_top(
        &part,
        on_the_boss,
        &format!("{shape:?}, the blend standing"),
    );

    ACornerBlended {
        part,
        blend,
        notch,
        notches,
        on_the_boss,
    }
}

fn assert_on_the_boss_s_top(part: &PartDocument, sketch: usize, what: &str) {
    assert!(
        !part.is_adrift(sketch),
        "{what}: the sketch on the boss is adrift"
    );
    let plane = part.sketches()[sketch].plane;
    let facing = plane.u.cross(plane.v);
    assert!(
        facing.distance(DVec3::Z) < 1e-6
            && (plane.origin.z - 5.0).abs() < 1e-6
            && plane.origin.x >= 40.0 - 1e-6,
        "{what}: the sketch on the boss stands at {} facing {facing}, where the boss's top stands 5 \
         high facing up from x = 40",
        plane.origin,
    );
}

#[test]
fn a_corner_s_blend_set_to_nothing_leaves_a_sketch_laid_after_it_on_its_face() {
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let ACornerBlended {
            mut part,
            on_the_boss,
            ..
        } = a_corner_blended(shape);

        part.apply(Operation::Variable(VariableChange::Edited {
            variable: VariableId(0),
            name: "size".to_string(),
            formula: Formula::Number(0.0),
        }));

        assert_on_the_boss_s_top(
            &part,
            on_the_boss,
            &format!("{shape:?} of the corner set to 0"),
        );
    }
}

#[test]
fn a_corner_s_blend_declined_on_replay_leaves_a_sketch_laid_after_it_on_its_face() {
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let ACornerBlended {
            mut part,
            blend,
            notch,
            notches,
            on_the_boss,
        } = a_corner_blended(shape);

        let across_the_front = Operation::Extrude {
            sketch: notches,
            areas: part.areas_at(notches, &[DVec2::new(15.0, 0.0)]),
            distance: 5.0.into(),
            mode: ExtrusionMode::Cut,
        };
        part.apply(Operation::EditMatter {
            raised_by: notch,
            to: Box::new(across_the_front),
        });

        let what = format!("{shape:?} of the corner, the notch moved across the foot of the front");
        assert!(
            matches!(part.declined_because(blend), Some(Declined::Lost(_))),
            "{what}: the blend reads {:?}",
            part.declined_because(blend),
        );
        assert_on_the_boss_s_top(&part, on_the_boss, &what);
    }
}
