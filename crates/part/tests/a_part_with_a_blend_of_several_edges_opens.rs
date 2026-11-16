//! A part saved with a blend of several edges opens, and what was laid after
//! the blend finds its faces again by the numbers the file holds. The fixture
//! `tests/fixtures/a_part_with_a_blend_of_several_edges.caopart` was written
//! by `PartDocument::save` at 8fe8d2f8, its design alone so that opening it
//! replays every step, and is kept as those bytes: it is never to be written
//! again.
//!
//! The face a sketch stands on is written as a number, and the numbers after a
//! blend depend on how many it takes: three per edge for a step of several,
//! one for a step of one. Any change to either — three per edge made two, a
//! step of one edge padded — moves the face the file names, and the sketch on
//! the boss's top lands on another face.
//!
//! Closes #979 — the whole transcription is in
//! `crates/app/tests/several_edges_in_one_step.rs`.
//! - found in review, beyond the issue's eight: the face numbers a blend of
//!   several edges takes are written into every part saved after it, and are
//!   frozen by a part saved with a corner of three edges, a blend of one edge
//!   and a sketch on a boss raised after both —
//!   `a_part_saved_with_a_blend_of_several_edges_opens_with_its_sketch_on_the_face_it_was_laid_on`

use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use cao_part::{FileError, Files, PartDocument};
use glam::DVec3;

struct Fixtures;

impl Files for Fixtures {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        std::fs::read(path).map_err(|_| FileError::Absent(path.to_path_buf()))
    }

    fn write(&self, path: &Path, _bytes: &[u8]) -> Result<(), FileError> {
        panic!("a fixture is never written again, and {path:?} was about to be")
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
}

fn saved_with_a_blend_of_several_edges() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("a_part_with_a_blend_of_several_edges.caopart")
}

/// Planes the exact kernel computes: what is left is rounding.
const EXACT: f64 = 1e-9;

/// The steps of the fixture's `design/history.json`, as written.
fn steps_as_written() -> Vec<serde_json::Value> {
    let bytes = std::fs::read(saved_with_a_blend_of_several_edges()).expect("the fixture");
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("the fixture is a zip");
    let mut text = String::new();
    archive
        .by_name("design/history.json")
        .expect("the fixture holds its history")
        .read_to_string(&mut text)
        .expect("the history reads");
    let history: serde_json::Value = serde_json::from_str(&text).expect("the history is JSON");
    history["steps"]
        .as_array()
        .expect("the history lists its steps")
        .clone()
}

/// A block 30 along X, 10 along Y, 20 high; the three edges meeting at its
/// corner (30, 0, 20) chamfered 2 in one step; the edge at the foot of its
/// back rounded 3; a boss 10 by 10 raised 5 from x = 40; a sketch on the
/// boss's top, face 17 as the file names it, and a square 6 by 6 drawn on it
/// at its middle, raised 3.
#[test]
fn a_part_saved_with_a_blend_of_several_edges_opens_with_its_sketch_on_the_face_it_was_laid_on() {
    let written = steps_as_written();
    let blends: Vec<usize> = written
        .iter()
        .filter(|step| step["kind"] == "blend")
        .map(|step| step["edges"].as_array().map_or(0, Vec::len))
        .collect();
    assert_eq!(
        blends,
        [3, 1],
        "the fixture no longer holds a blend of three edges then one of one"
    );
    let anchors: Vec<&serde_json::Value> = written
        .iter()
        .filter(|step| step["kind"] == "sketch" && !step["on"].is_null())
        .map(|step| &step["on"]["face"])
        .collect();
    assert_eq!(
        anchors,
        [&serde_json::json!(17)],
        "the fixture's sketch on the boss no longer names face 17",
    );

    let part = PartDocument::load(&Fixtures, &saved_with_a_blend_of_several_edges())
        .unwrap_or_else(|err| {
            panic!("a part saved with a blend of several edges no longer opens: {err:?}")
        });

    assert_eq!(part.name(), "Corner");
    assert_eq!(
        part.metadata.id.to_string(),
        "4fdc820b-af69-4571-b7bd-083890a7ac88",
        "part.json came back as another part",
    );
    assert_eq!(part.history.operations().len(), 11);
    assert_eq!(part.history.steps().len(), 8);
    for (number, _) in part.history.replay_order() {
        assert_eq!(
            part.declined_because(number),
            None,
            "operation {number} is no longer built",
        );
    }

    let on_the_boss = 2;
    assert!(
        !part.is_adrift(on_the_boss),
        "the sketch on the boss is adrift"
    );
    let plane = part.sketches()[on_the_boss].plane;
    assert!(
        plane.origin.abs_diff_eq(DVec3::new(40.0, 0.0, 5.0), EXACT)
            && plane.u.abs_diff_eq(DVec3::X, EXACT)
            && plane.v.abs_diff_eq(DVec3::Y, EXACT),
        "the sketch on the boss stands at {} along {} and {}, where it stood on the boss's top, \
         at (40, 0, 5) along X and Y",
        plane.origin,
        plane.u,
        plane.v,
    );

    let body = part.body();
    assert!(body.is_exact(), "the part replayed is no longer exact");
    let raised_on_the_boss = 100.0
        - body
            .ray_hit(DVec3::new(45.0, 5.0, 100.0), -DVec3::Z)
            .expect("a ray down the boss's middle meets the part")
            .distance;
    assert!(
        (raised_on_the_boss - 8.0).abs() < EXACT,
        "the square raised on the boss's top stands {raised_on_the_boss} high in its middle, where \
         it stood 8",
    );
    let volume = body.volume();
    let as_saved = 6435.390837450643;
    assert!(
        (volume - as_saved).abs() <= EXACT * as_saved,
        "the part replayed holds {volume}, where it held {as_saved}",
    );
    let (low, high) = body.bounds().expect("the part holds matter");
    assert!(
        low.abs_diff_eq(DVec3::ZERO, EXACT)
            && high.abs_diff_eq(DVec3::new(50.0, 10.0, 20.0), EXACT),
        "the part replayed stands from {low:?} to {high:?}",
    );
    let faces = (0..body.faces_end())
        .filter(|face| body.has_face(*face))
        .count();
    assert_eq!(faces, 22, "the part replayed has another count of faces");
    assert_eq!(
        body.drawn_edges().len(),
        48,
        "the part replayed draws another set of edges",
    );
}
