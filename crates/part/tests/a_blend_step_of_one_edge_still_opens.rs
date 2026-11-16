//! A part saved while a chamfer or a fillet named one edge opens, once a blend
//! step holds a list of edges, and replays as it did. The fixture
//! `tests/fixtures/a_part_saved_before_979.caopart` was written by
//! `PartDocument::save` before #979, its design alone so that opening it
//! replays every step, and is kept as those bytes: it is never to be written
//! again.
//!
//! Closes #979 — the whole transcription is in
//! `crates/app/tests/several_edges_in_one_step.rs`; this is the part file's
//! half.
//! - A `.caopart` holding a blend step written with the single `edge` field
//!   opens and replays to the same body as before —
//!   `a_part_whose_blend_step_names_a_single_edge_opens_and_replays_to_the_same_body`

use std::f64::consts::PI;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use cao_part::history::StepKind;
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

fn saved_before_979() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("a_part_saved_before_979.caopart")
}

/// Planes and cylinders the exact kernel computes: what is left between it
/// and the arithmetic is rounding.
const EXACT: f64 = 1e-9;

/// The blend steps of the fixture's `design/history.json`, as written: each
/// names its edge under `edge`, alone.
fn blend_steps_as_written() -> Vec<serde_json::Value> {
    let bytes = std::fs::read(saved_before_979()).expect("the fixture is there");
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
        .iter()
        .filter(|step| step["kind"] == "blend")
        .cloned()
        .collect()
}

/// A block 30 along X, 10 along Y, raised 20 from XY; its top front edge,
/// facing -Y, chamfered 2; then its top back edge, facing +Y, rounded 3.
#[test]
fn a_part_whose_blend_step_names_a_single_edge_opens_and_replays_to_the_same_body() {
    let written = blend_steps_as_written();
    assert_eq!(written.len(), 2, "the fixture's blend steps: {written:?}");
    for step in &written {
        assert!(
            step["edge"].is_object() && step.get("edges").is_none(),
            "the fixture no longer holds a step written with the single `edge` field: {step}",
        );
    }

    let part = PartDocument::load(&Fixtures, &saved_before_979())
        .unwrap_or_else(|err| panic!("a part saved before #979 no longer opens: {err:?}"));

    assert_eq!(part.name(), "Block");
    assert_eq!(
        part.metadata.id.to_string(),
        "ee6700fe-2919-49e7-aac8-7efa4d110a25",
        "part.json came back as another part",
    );
    assert_eq!(
        part.history.operations().len(),
        5,
        "the history came back as {:?}",
        part.history.operations(),
    );
    let kinds: Vec<StepKind> = part
        .history
        .steps()
        .iter()
        .map(|step| step.kind())
        .collect();
    assert_eq!(
        kinds,
        [
            StepKind::Sketch,
            StepKind::Extrusion,
            StepKind::Blend,
            StepKind::Blend
        ],
        "the steps came back as {kinds:?}",
    );
    for step in &part.history.steps()[2..] {
        assert_eq!(step.len(), 1, "a blend step holds one operation");
        let number = step.operations()[0];
        assert_eq!(
            part.declined_because(number),
            None,
            "the blend of operation {number} is no longer built",
        );
        assert_eq!(
            part.reduced_at(number),
            None,
            "the blend of operation {number} is reduced, where its faces gave its size",
        );
    }

    let body = part.body();
    assert!(body.is_exact(), "the block replayed is no longer exact");
    let volume = body.volume();
    let chamfered_and_rounded =
        30.0 * 10.0 * 20.0 - 2.0 * 2.0 * 30.0 / 2.0 - (1.0 - PI / 4.0) * 3.0 * 3.0 * 30.0;
    assert!(
        (volume - chamfered_and_rounded).abs() <= EXACT * chamfered_and_rounded,
        "the block replayed holds {volume}, where it held {chamfered_and_rounded}",
    );
    let (low, high) = body.bounds().expect("the block holds matter");
    assert!(
        low.abs_diff_eq(DVec3::ZERO, EXACT)
            && high.abs_diff_eq(DVec3::new(30.0, 10.0, 20.0), EXACT),
        "the block replayed stands from {low:?} to {high:?}",
    );
    let faces = (0..body.faces_end())
        .filter(|face| body.has_face(*face))
        .count();
    assert_eq!(
        faces, 8,
        "six sides, a chamfer and a fillet, where the block replayed has {faces} faces",
    );
    assert_eq!(
        body.drawn_edges().len(),
        18,
        "the block replayed draws another set of edges",
    );
}
