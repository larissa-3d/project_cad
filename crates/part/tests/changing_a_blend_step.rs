//! A chamfer or a fillet step changed after it was laid: its size, its kind or
//! its edges given new values, the part laid again from that step, in the step
//! it already had.
//!
//! Closes #988 — the gestures, the panel and the view are in
//! `crates/app/tests/reopening_a_blend_step.rs`; these bullets are the part's
//! half, where the change is recorded and replayed.
//! - « Appliquer » on a reopened step with a new size, kind or set of edges
//!   records one `EditMatter` change; the body afterwards is the one the part
//!   gives with the step laid that way — the part's half, the step reading its
//!   new values as `raised_as` hands them to the history row, by
//!   `a_blend_step_changed_in_size_kind_or_edges_records_one_change_and_gives_the_body_laid_that_way`;
//!   the history row read on screen is in the app's file
//! - One Cmd+Z after that « Appliquer » gives back the body and the history
//!   row as they were before it — the part's half, the body and the step as
//!   `raised_as` reads it, by `one_undo_takes_back_a_change_of_blend_step`
//! - A reopened step whose new choice the kernel declines records nothing —
//!   the part's half, the decline handed back and the history untouched, by
//!   `a_change_of_blend_step_the_kernel_declines_records_nothing_and_says_why`;
//!   the panel kept open with the reason and the edges is in the app's file
//! - Taking an edge out of a reopened step of several edges leaves the face
//!   numbers of every later step as they were: a sketch laid on a face after
//!   the step stays on that face —
//!   `taking_an_edge_out_of_a_blend_step_leaves_a_sketch_laid_after_it_on_its_face`
//! - Compaction after a change writes the step down as changed, and the part
//!   replays to the same body —
//!   `compacting_the_history_keeps_the_change_of_a_blend_step`
//!
//! Each part changed is held against the same part laid with the step that way
//! from the start.

use cao_part::history::{BlendShape, FaceAnchor, Operation, PointRef, StepKind};
use cao_part::{ExtrusionMode, Outcome, PartDocument};
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
    "2026-11-17T09:00:00Z".parse().expect("a date")
}

fn rectangle(part: &mut PartDocument, sketch: usize, corner: DVec2, opposite: DVec2) {
    part.apply(Operation::AddRectangle {
        sketch,
        corner: PointRef::New(corner),
        opposite: PointRef::New(opposite),
        construction: false,
    });
}

/// A block 30 along X, 10 along Y, raised 20 from XY.
fn a_block() -> PartDocument {
    let mut part = PartDocument::new("Block", at_nine());
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    rectangle(&mut part, 0, DVec2::ZERO, DVec2::new(LENGTH, BREADTH));
    let areas = part.areas_at(0, &[DVec2::new(LENGTH / 2.0, BREADTH / 2.0)]);
    part.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: HEIGHT.into(),
        mode: ExtrusionMode::Add,
    });
    part
}

/// A boss 10 by 10 raised 5 from XY beside the block, from x = 40: a step of
/// matter after the blend.
fn raise_a_boss_beside(part: &mut PartDocument) {
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let boss = part.sketches().len() - 1;
    rectangle(part, boss, DVec2::new(40.0, 0.0), DVec2::new(50.0, 10.0));
    part.apply(Operation::Extrude {
        sketch: boss,
        areas: part.areas_at(boss, &[DVec2::new(45.0, 5.0)]),
        distance: 5.0.into(),
        mode: ExtrusionMode::Add,
    });
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

/// A blend of the first `count` of the block's four top edges.
fn a_blend(count: usize, shape: BlendShape, size: f64) -> Operation {
    let mut edges = the_four_top_edges(a_block().body());
    edges.truncate(count);
    Operation::Blend {
        edges,
        shape,
        size: size.into(),
    }
}

/// The block, `blend` laid on it as one step, then a boss raised beside it:
/// the part, and the number of the blend's operation.
fn a_block_blended(blend: Operation) -> (PartDocument, u32) {
    let mut part = a_block();
    let outcome = part.apply(blend.clone());
    assert_eq!(outcome, None, "{blend:?} on the block: built");
    let step = part.history.steps().last().expect("the blend");
    assert_eq!(step.kind(), StepKind::Blend, "{blend:?}: its kind");
    let number = step.operations()[0];
    raise_a_boss_beside(&mut part);
    (part, number)
}

fn faces_of(body: &Body) -> usize {
    (0..body.faces_end())
        .filter(|face| body.has_face(*face))
        .count()
}

/// What differs between `made` and `expected`, as sentences: nothing when
/// they hold the same matter.
fn how_the_bodies_differ(made: &Body, expected: &Body, expected_is: &str) -> Vec<String> {
    let mut wrong = Vec::new();
    if (made.volume() - expected.volume()).abs() > EXACT * expected.volume() {
        wrong.push(format!(
            "the part holds {} where {expected_is} holds {}",
            made.volume(),
            expected.volume(),
        ));
    }
    if faces_of(made) != faces_of(expected) {
        wrong.push(format!(
            "{} faces where {expected_is} has {}",
            faces_of(made),
            faces_of(expected),
        ));
    }
    if made.drawn_edges().len() != expected.drawn_edges().len() {
        wrong.push(format!(
            "{} edges drawn where {expected_is} draws {}",
            made.drawn_edges().len(),
            expected.drawn_edges().len(),
        ));
    }
    let (made_low, made_high) = made.bounds().expect("some matter");
    let (low, high) = expected.bounds().expect("some matter");
    if !(made_low.abs_diff_eq(low, EXACT) && made_high.abs_diff_eq(high, EXACT)) {
        wrong.push(format!(
            "it stands from {made_low:?} to {made_high:?}, where {expected_is} stands from {low:?} \
             to {high:?}",
        ));
    }
    wrong
}

fn changes_recorded(part: &PartDocument) -> usize {
    part.history
        .applied_operations()
        .iter()
        .filter(|operation| matches!(operation, Operation::EditMatter { .. }))
        .count()
}

#[test]
fn a_blend_step_changed_in_size_kind_or_edges_records_one_change_and_gives_the_body_laid_that_way()
{
    let cases = [
        (
            "a chamfer of the four top edges given a new size",
            a_blend(4, BlendShape::Chamfer, SIZE),
            a_blend(4, BlendShape::Chamfer, 3.0),
        ),
        (
            "a chamfer of the four top edges turned into a fillet",
            a_blend(4, BlendShape::Chamfer, SIZE),
            a_blend(4, BlendShape::Fillet, SIZE),
        ),
        (
            "a fillet of the four top edges with one taken out",
            a_blend(4, BlendShape::Fillet, SIZE),
            a_blend(3, BlendShape::Fillet, SIZE),
        ),
        (
            "a fillet of three top edges with the fourth added",
            a_blend(3, BlendShape::Fillet, SIZE),
            a_blend(4, BlendShape::Fillet, SIZE),
        ),
    ];
    let mut failures = Vec::new();
    for (what, laid, changed) in cases {
        let (mut part, blend) = a_block_blended(laid);
        let steps = part.history.steps().len();
        let (laid_that_way, _) = a_block_blended(changed.clone());

        let outcome = part.apply(Operation::EditMatter {
            raised_by: blend,
            to: Box::new(changed.clone()),
        });

        let mut wrong = Vec::new();
        if outcome.is_some() {
            wrong.push(format!("the change comes back {outcome:?}"));
        }
        if changes_recorded(&part) != 1 {
            wrong.push(format!(
                "{} changes are recorded where one is",
                changes_recorded(&part),
            ));
        }
        if part.history.steps().len() != steps {
            wrong.push(format!(
                "the design has {} steps where it had {steps}",
                part.history.steps().len(),
            ));
        }
        if part.raised_as(blend) != Some(&changed) {
            wrong.push(format!(
                "the step reads {:?}, not its new values",
                part.raised_as(blend),
            ));
        }
        if part.declined_because(blend).is_some() {
            wrong.push(format!(
                "the step is declined: {:?}",
                part.declined_because(blend),
            ));
        }
        wrong.extend(how_the_bodies_differ(
            part.body(),
            laid_that_way.body(),
            "the part laid that way",
        ));
        if !wrong.is_empty() {
            failures.push(format!("{what}:\n    {}", wrong.join("\n    ")));
        }
    }

    assert!(
        failures.is_empty(),
        "a blend step changed after a boss was raised beside it is not laid again from its new \
         values in its own step:\n  {}",
        failures.join("\n  "),
    );
}

/// The block with a disc of radius 5 raised 10 beside it, centred on
/// (45, 5): the four top edges of the block chamfered at `SIZE` as one step,
/// then a boss raised beside both. The part, the number of the chamfer's
/// operation, and the rim of the disc's top, named on the body before the
/// chamfer.
fn a_block_chamfered_beside_a_disc() -> (PartDocument, u32, EdgeName) {
    let mut part = a_block();
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let disc = part.sketches().len() - 1;
    part.apply(Operation::AddCircle {
        sketch: disc,
        center: PointRef::New(DVec2::new(45.0, 5.0)),
        radius: 5.0,
        rim: Vec::new(),
        construction: false,
    });
    part.apply(Operation::Extrude {
        sketch: disc,
        areas: part.areas_at(disc, &[DVec2::new(45.0, 5.0)]),
        distance: 10.0.into(),
        mode: ExtrusionMode::Add,
    });
    let disc_top = face(part.body(), DVec3::Z, 10.0);
    let rim = part
        .body()
        .edge_names()
        .into_iter()
        .find(|name| name.between.contains(&vec![disc_top]))
        .expect("the rim of the disc's top");
    let mut on_the_rim = part.clone();
    assert!(
        matches!(
            on_the_rim.apply(Operation::Blend {
                edges: vec![rim.clone()],
                shape: BlendShape::Chamfer,
                size: SIZE.into(),
            }),
            Some(Outcome::BlendDeclined(_))
        ),
        "the fixture means a chamfer of the disc's rim, laid as a step, declined by the kernel",
    );
    let outcome = part.apply(Operation::Blend {
        edges: the_four_top_edges(part.body()),
        shape: BlendShape::Chamfer,
        size: SIZE.into(),
    });
    assert_eq!(outcome, None, "the chamfer of the block's top: built");
    let chamfer = part
        .history
        .steps()
        .last()
        .expect("the chamfer")
        .operations()[0];
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let boss = part.sketches().len() - 1;
    rectangle(
        &mut part,
        boss,
        DVec2::new(60.0, 0.0),
        DVec2::new(70.0, 10.0),
    );
    part.apply(Operation::Extrude {
        sketch: boss,
        areas: part.areas_at(boss, &[DVec2::new(65.0, 5.0)]),
        distance: 5.0.into(),
        mode: ExtrusionMode::Add,
    });
    (part, chamfer, rim)
}

#[test]
fn a_change_of_blend_step_the_kernel_declines_records_nothing_and_says_why() {
    let (mut part, chamfer, rim) = a_block_chamfered_beside_a_disc();
    let held = part.history.clone();
    let as_laid = part.raised_as(chamfer).cloned();
    let before = part.body().clone();

    let outcome = part.apply(Operation::EditMatter {
        raised_by: chamfer,
        to: Box::new(Operation::Blend {
            edges: vec![rim],
            shape: BlendShape::Chamfer,
            size: SIZE.into(),
        }),
    });

    let mut wrong = Vec::new();
    if !matches!(outcome, Some(Outcome::BlendDeclined(_))) {
        wrong.push(format!(
            "the change comes back {outcome:?}, not the kernel's reason"
        ));
    }
    if part.history != held {
        wrong.push(format!(
            "the history moved: {} changes are recorded",
            changes_recorded(&part),
        ));
    }
    if part.raised_as(chamfer).cloned() != as_laid {
        wrong.push(format!(
            "the step reads {:?}, not its values as laid",
            part.raised_as(chamfer),
        ));
    }
    if part.declined_because(chamfer).is_some() {
        wrong.push(format!(
            "the step stands declined: {:?}",
            part.declined_because(chamfer),
        ));
    }
    wrong.extend(how_the_bodies_differ(
        part.body(),
        &before,
        "the part before the change",
    ));

    assert!(
        wrong.is_empty(),
        "the chamfer of a block's top, changed to the rim of a disc beside it, which the kernel \
         declines, was taken:\n  {}",
        wrong.join("\n  "),
    );
}

/// The one face facing up 5 high with every corner at x = 40 or further: the
/// top of the boss raised beside the block.
fn the_boss_s_top(body: &Body) -> usize {
    let faces: Vec<usize> = (0..body.faces_end())
        .filter(|face| {
            body.plane_of(*face).is_some_and(|plane| {
                plane.normal.distance(DVec3::Z) < EXACT
                    && (plane.corners[0].z - 5.0).abs() < EXACT
                    && plane.corners.iter().all(|corner| corner.x >= 40.0 - EXACT)
            })
        })
        .collect();
    assert_eq!(faces.len(), 1, "one top to the boss: {faces:?}");
    faces[0]
}

/// Where the sketch `sketch` stands, if not on the boss's top, as a sentence.
fn off_the_boss_s_top(part: &PartDocument, sketch: usize) -> Option<String> {
    if part.is_adrift(sketch) {
        return Some("the sketch on the boss is adrift".to_string());
    }
    let plane = part.sketches()[sketch].plane;
    let facing = plane.u.cross(plane.v);
    let on_it = facing.distance(DVec3::Z) < 1e-6
        && (plane.origin.z - 5.0).abs() < 1e-6
        && plane.origin.x >= 40.0 - 1e-6;
    (!on_it).then(|| {
        format!(
            "the sketch on the boss stands at {} facing {facing}, where the boss's top stands 5 \
             high facing up from x = 40",
            plane.origin,
        )
    })
}

#[test]
fn taking_an_edge_out_of_a_blend_step_leaves_a_sketch_laid_after_it_on_its_face() {
    let mut failures = Vec::new();
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let (mut part, blend) = a_block_blended(a_blend(4, shape, SIZE));
        let top = the_boss_s_top(part.body());
        part.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: Some(FaceAnchor {
                face: top,
                up: DVec3::Y,
            }),
        });
        let on_the_boss = part.sketches().len() - 1;
        assert_eq!(
            off_the_boss_s_top(&part, on_the_boss),
            None,
            "{shape:?} of four edges: the fixture means a sketch laid on the boss's top",
        );

        let outcome = part.apply(Operation::EditMatter {
            raised_by: blend,
            to: Box::new(a_blend(3, shape, SIZE)),
        });

        let mut wrong = Vec::new();
        if outcome.is_some() {
            wrong.push(format!("the change comes back {outcome:?}"));
        }
        if changes_recorded(&part) != 1 {
            wrong.push(format!(
                "{} changes are recorded where one is",
                changes_recorded(&part),
            ));
        }
        let top_now = the_boss_s_top(part.body());
        if top_now != top {
            wrong.push(format!(
                "the boss's top is face {top_now} where it was face {top}"
            ));
        }
        wrong.extend(off_the_boss_s_top(&part, on_the_boss));
        if !wrong.is_empty() {
            failures.push(format!(
                "{shape:?} of four edges, one taken out:\n    {}",
                wrong.join("\n    ")
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "an edge taken out of a blend step moved the faces of a boss raised after it:\n  {}",
        failures.join("\n  "),
    );
}
