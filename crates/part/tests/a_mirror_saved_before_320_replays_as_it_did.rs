//! A part saved before a mirrored copy was held to its original replays its
//! mirror as it was laid then: a bare copy, every point copied, no rule.
//!
//! Closes #320.
//! - found in review: a mirror read from a file written before #320 replays to
//!   the same points, ids and rules as it did, and a later step naming a point
//!   by its id still lands on the one it named —
//!   `a_mirror_saved_before_320_lays_the_bare_copy_it_laid_then`,
//!   `a_step_after_a_mirror_saved_before_320_lands_on_the_point_it_named`

use cao_part::history::{Operation, PointRef};
use cao_part::{History, PartState};
use cao_sketch::{PointId, WorkPlane};
use glam::DVec2;

const TOLERANCE: f64 = 1e-9;

/// Half a profile whose two ends stand on the V axis, mirrored across it as a
/// file written before #320 carries the step, then a trait drawn from the
/// copy's upper corner by its id.
fn a_history_saved_before_320() -> History {
    let mirror_as_written: Operation = serde_json::from_str(
        r#"{"Mirror":{"sketch":0,"elements":[{"Segment":0},{"Segment":1},{"Segment":2}],"axis":{"Sketch":"V"}}}"#,
    )
    .expect("a mirror as a file written before #320 holds it");
    let mut history = History::default();
    for operation in [
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(0.0, 1.0)),
            end: PointRef::New(DVec2::new(3.0, 1.0)),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PointId(2)),
            end: PointRef::New(DVec2::new(3.0, 4.0)),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PointId(3)),
            end: PointRef::New(DVec2::new(0.0, 4.0)),
            construction: false,
        },
        mirror_as_written,
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PointId(7)),
            end: PointRef::New(DVec2::new(-3.0, 7.0)),
            construction: false,
        },
    ] {
        history.push(operation);
    }
    history
}

#[test]
fn a_mirror_saved_before_320_lays_the_bare_copy_it_laid_then() {
    let state = PartState::rebuild(&a_history_saved_before_320());
    let sketch = &state.sketches[0];

    let laid_then = [
        (PointId(5), DVec2::new(0.0, 1.0)),
        (PointId(6), DVec2::new(-3.0, 1.0)),
        (PointId(7), DVec2::new(-3.0, 4.0)),
        (PointId(8), DVec2::new(0.0, 4.0)),
    ];
    for (id, place) in laid_then {
        assert!(
            !sketch.is_erased_point(id) && sketch.point(id).distance(place) < TOLERANCE,
            "{id:?} stands at {}, where the mirror laid it then is {place}",
            sketch.point(id)
        );
    }
    assert_eq!(
        sketch.live_segments().count(),
        7,
        "three traits, their three copies, and the trait drawn after"
    );
    assert!(
        sketch.constraints().is_empty(),
        "a mirror saved before #320 laid no rule, and now lays {:?}",
        sketch.constraints()
    );
}

#[test]
fn a_step_after_a_mirror_saved_before_320_lands_on_the_point_it_named() {
    let state = PartState::rebuild(&a_history_saved_before_320());
    let sketch = &state.sketches[0];

    let drawn_after = sketch
        .live_segments()
        .map(|(_, side)| side)
        .find(|side| sketch.point(side.end).distance(DVec2::new(-3.0, 7.0)) < TOLERANCE)
        .expect("the trait drawn after the mirror, ending where it was drawn to");
    assert_eq!(
        drawn_after.start,
        PointId(7),
        "the trait drawn after the mirror does not start from the point it named"
    );
    assert!(
        sketch
            .point(drawn_after.start)
            .distance(DVec2::new(-3.0, 4.0))
            < TOLERANCE,
        "the point it named, the copy's upper corner then, stands at {}",
        sketch.point(drawn_after.start)
    );
}
