//! What a part does with a mirror: a second copy appears the other side of the
//! axis, the original stays, and a replay rebuilds the same thing.
//!
//! Closes #320.
//! - Erasing a trait erases its twin, from either side —
//!   `erasing_a_trait_of_the_original_erases_its_twin`,
//!   `erasing_a_trait_of_the_copy_erases_its_twin`, and a point as a trait is —
//!   `erasing_a_point_of_the_original_erases_its_twin_point`
//! - A mirror is still one step in the history, undone in one go —
//!   `undoing_a_mirror_takes_back_its_copy_and_its_symmetry`
//! - Compacting the history (#191) carries the symmetry along —
//!   `a_compacted_mirror_keeps_its_copy_symmetric`

use cao_part::history::{Operation, PointRef};
use cao_part::{PartDocument, PartState};
use cao_sketch::{
    AnnotationMetrics, ChosenAxis, Constraint, Element, PointId, SegmentId, Selection, Sketch,
    SketchAxis, WorkPlane,
};
use glam::DVec2;

const TOLERANCE: f64 = 1e-9;

const METRICS: AnnotationMetrics = AnnotationMetrics {
    offset_pixels: 22.0,
    arrow_pixels: 8.0,
    arc_pixels: 34.0,
    pixel: 1.0,
    nudge: DVec2::ZERO,
};

fn a_trait_east_of_the_axis() -> Vec<Operation> {
    vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(2.0, 1.0)),
            end: PointRef::New(DVec2::new(6.0, 1.0)),
            construction: false,
        },
    ]
}

fn replay(operations: &[Operation]) -> PartState {
    let mut state = PartState::default();
    for operation in operations {
        state.apply(operation);
    }
    state
}

fn mirrored() -> Operation {
    Operation::Mirror {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        axis: ChosenAxis::Sketch(SketchAxis::V),
    }
}

#[test]
fn a_mirrored_trait_stands_the_other_side_of_the_axis_with_the_first_still_there() {
    let mut state = replay(&a_trait_east_of_the_axis());

    state.apply(&mirrored());

    let sketch = &state.sketches[0];
    assert_eq!(
        sketch.live_segments().count(),
        2,
        "the original and its copy"
    );
    let mut ends: Vec<f64> = sketch
        .live_points()
        .map(|(_, at)| at.x)
        .filter(|x| x.abs() > TOLERANCE)
        .collect();
    ends.sort_by(f64::total_cmp);
    assert_eq!(ends, vec![-6.0, -2.0, 2.0, 6.0]);
}

#[test]
fn a_mirror_replayed_rebuilds_the_copy_it_laid() {
    let mut operations = a_trait_east_of_the_axis();
    operations.push(mirrored());

    let ends = |state: &PartState| {
        let sketch = &state.sketches[0];
        sketch
            .live_segments()
            .map(|(id, _)| sketch.endpoints(id))
            .collect::<Vec<_>>()
    };

    assert_eq!(ends(&replay(&operations)), ends(&replay(&operations)));
    assert_eq!(ends(&replay(&operations)).len(), 2);
}

#[test]
fn an_arc_a_dragged_box_took_hold_of_is_mirrored_as_an_arc() {
    let mut state = replay(&[
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddArc {
            sketch: 0,
            center: PointRef::New(DVec2::new(4.0, 1.0)),
            start: PointRef::New(DVec2::new(6.0, 1.0)),
            end: PointRef::New(DVec2::new(4.0, 3.0)),
            construction: false,
        },
    ]);
    let held = state.sketches[0]
        .inside_band(DVec2::new(0.0, 0.0), DVec2::new(10.0, 10.0), METRICS)
        .into_iter()
        .filter_map(|caught| match caught {
            Selection::Element(element) => Some(element),
            _ => None,
        })
        .collect();

    state.apply(&Operation::Mirror {
        sketch: 0,
        elements: held,
        axis: ChosenAxis::Sketch(SketchAxis::V),
    });

    let sketch = &state.sketches[0];
    assert_eq!(sketch.live_arcs().count(), 2, "the original and its copy");
    let radii: Vec<f64> = sketch
        .live_arcs()
        .map(|(id, _)| sketch.arc_radius(id))
        .collect();
    for radius in radii {
        assert!(
            (radius - 2.0).abs() <= TOLERANCE,
            "the copy is as round as the original, and this one is {radius}"
        );
    }
}

#[test]
fn a_mirror_across_a_trait_the_drawing_does_not_have_does_nothing() {
    let mut state = replay(&a_trait_east_of_the_axis());

    let said = state.apply(&Operation::Mirror {
        sketch: 0,
        elements: vec![Element::Point(PointId(1))],
        axis: ChosenAxis::Trait(SegmentId(9)),
    });

    assert_eq!(said, None);
    assert_eq!(state.sketches[0].live_segments().count(), 1);
}

fn a_corner_mirrored_across_the_v_axis() -> PartState {
    replay(&[
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(2.0, 1.0)),
            end: PointRef::New(DVec2::new(6.0, 1.0)),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PointId(2)),
            end: PointRef::New(DVec2::new(6.0, 4.0)),
            construction: false,
        },
        Operation::Mirror {
            sketch: 0,
            elements: vec![
                Element::Segment(SegmentId(0)),
                Element::Segment(SegmentId(1)),
            ],
            axis: ChosenAxis::Sketch(SketchAxis::V),
        },
    ])
}

fn trait_between(state: &PartState, from: DVec2, to: DVec2) -> SegmentId {
    let sketch = &state.sketches[0];
    (0..sketch.segments().len())
        .map(SegmentId)
        .find(|id| {
            let (a, b) = sketch.endpoints(*id);
            (a.distance(from) < TOLERANCE && b.distance(to) < TOLERANCE)
                || (a.distance(to) < TOLERANCE && b.distance(from) < TOLERANCE)
        })
        .unwrap_or_else(|| panic!("a trait from {from} to {to}"))
}

fn erased(state: &mut PartState, segment: SegmentId) {
    state.apply(&Operation::EraseMany {
        sketch: 0,
        elements: vec![Element::Segment(segment)],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });
}

#[test]
fn erasing_a_trait_of_the_original_erases_its_twin() {
    let mut state = a_corner_mirrored_across_the_v_axis();
    let at = |x: f64, y: f64| DVec2::new(x, y);
    let base = trait_between(&state, at(2.0, 1.0), at(6.0, 1.0));
    let its_twin = trait_between(&state, at(-2.0, 1.0), at(-6.0, 1.0));
    let side = trait_between(&state, at(6.0, 1.0), at(6.0, 4.0));
    let its_copy = trait_between(&state, at(-6.0, 1.0), at(-6.0, 4.0));

    erased(&mut state, base);

    let sketch = &state.sketches[0];
    assert!(sketch.is_erased_segment(base));
    assert!(
        sketch.is_erased_segment(its_twin),
        "the copy of the erased trait is still drawn"
    );
    assert!(
        !sketch.is_erased_segment(side) && !sketch.is_erased_segment(its_copy),
        "erasing one trait took a pair nobody touched with it"
    );
}

#[test]
fn erasing_a_trait_of_the_copy_erases_its_twin() {
    let mut state = a_corner_mirrored_across_the_v_axis();
    let at = |x: f64, y: f64| DVec2::new(x, y);
    let base = trait_between(&state, at(2.0, 1.0), at(6.0, 1.0));
    let its_copy = trait_between(&state, at(-2.0, 1.0), at(-6.0, 1.0));
    let side = trait_between(&state, at(6.0, 1.0), at(6.0, 4.0));
    let copied_side = trait_between(&state, at(-6.0, 1.0), at(-6.0, 4.0));

    erased(&mut state, copied_side);

    let sketch = &state.sketches[0];
    assert!(sketch.is_erased_segment(copied_side));
    assert!(
        sketch.is_erased_segment(side),
        "the original of the erased copy is still drawn"
    );
    assert!(
        !sketch.is_erased_segment(base) && !sketch.is_erased_segment(its_copy),
        "erasing one trait took a pair nobody touched with it"
    );
}

#[test]
fn erasing_a_point_of_the_original_erases_its_twin_point() {
    let mut state = a_corner_mirrored_across_the_v_axis();
    let point_at = |state: &PartState, place: DVec2| {
        state.sketches[0]
            .live_points()
            .find(|(_, at)| at.distance(place) < TOLERANCE)
            .map(|(id, _)| id)
            .unwrap_or_else(|| panic!("a point at {place}"))
    };
    let corner = point_at(&state, DVec2::new(2.0, 1.0));
    let its_twin = point_at(&state, DVec2::new(-2.0, 1.0));
    let untouched = point_at(&state, DVec2::new(-6.0, 4.0));

    state.apply(&Operation::EraseMany {
        sketch: 0,
        elements: vec![Element::Point(corner)],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });

    let sketch = &state.sketches[0];
    assert!(sketch.is_erased_point(corner));
    assert!(
        sketch.is_erased_point(its_twin),
        "the copy of the erased point is still drawn"
    );
    assert!(!sketch.is_erased_point(untouched));
}

fn a_document_mirroring_a_ruled_corner() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(2.0, 1.0)),
        end: PointRef::New(DVec2::new(6.0, 1.0)),
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(PointId(2)),
        end: PointRef::New(DVec2::new(6.0, 4.0)),
        construction: false,
    });
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Perpendicular {
            first: SegmentId(0),
            second: SegmentId(1),
        },
    });
    document
}

fn mirrored_corner() -> Operation {
    Operation::Mirror {
        sketch: 0,
        elements: vec![
            Element::Segment(SegmentId(0)),
            Element::Segment(SegmentId(1)),
        ],
        axis: ChosenAxis::Sketch(SketchAxis::V),
    }
}

#[test]
fn undoing_a_mirror_takes_back_its_copy_and_its_symmetry() {
    let mut document = a_document_mirroring_a_ruled_corner();
    let rules = document.sketches()[0].constraints().to_vec();
    let points: Vec<(PointId, DVec2)> = document.sketches()[0].live_points().collect();
    document.apply(mirrored_corner());
    let mirrored_rules = document.sketches()[0].constraints().to_vec();
    assert!(
        mirrored_rules.len() > rules.len(),
        "the mirror laid no symmetry to take back"
    );

    assert!(document.undo(), "nothing to undo");

    let sketch = &document.sketches()[0];
    assert_eq!(sketch.live_segments().count(), 2, "the copy is still drawn");
    assert_eq!(
        sketch.constraints(),
        &rules[..],
        "one undo left some of the mirror's rules behind"
    );
    assert_eq!(sketch.live_points().collect::<Vec<_>>(), points);

    assert!(document.redo(), "nothing to redo");
    assert_eq!(document.sketches()[0].live_segments().count(), 4);
    assert_eq!(document.sketches()[0].constraints(), &mirrored_rules[..]);
}

fn drag(sketch: &mut Sketch, point: PointId, by: DVec2) {
    let pull = sketch.pull(point, 1.0);
    let to = sketch.point(point) + by;
    let landing = sketch.slide(point, to);
    sketch.settle_pulled(&pull, landing, 1.0);
}

fn a_half_profile_mirrored_and_compacted() -> Sketch {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (start, end) in [
        (PointRef::New(DVec2::new(0.0, 1.0)), DVec2::new(3.0, 1.0)),
        (PointRef::Existing(PointId(2)), DVec2::new(3.0, 4.0)),
        (PointRef::Existing(PointId(3)), DVec2::new(0.0, 4.0)),
    ] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start,
            end: PointRef::New(end),
            construction: false,
        });
    }
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Perpendicular {
            first: SegmentId(0),
            second: SegmentId(1),
        },
    });
    document.apply(Operation::Mirror {
        sketch: 0,
        elements: (0..3)
            .map(|side| Element::Segment(SegmentId(side)))
            .collect(),
        axis: ChosenAxis::Sketch(SketchAxis::V),
    });

    document.compact_history();

    document.sketches()[0].clone()
}

fn point_in(sketch: &Sketch, place: DVec2) -> PointId {
    sketch
        .live_points()
        .find(|(_, at)| at.distance(place) < TOLERANCE)
        .map(|(id, _)| id)
        .unwrap_or_else(|| panic!("a point at {place}"))
}

fn trait_in(sketch: &Sketch, from: DVec2, to: DVec2) -> SegmentId {
    sketch
        .live_segments()
        .map(|(id, _)| id)
        .find(|id| {
            let (a, b) = sketch.endpoints(*id);
            (a.distance(from) < TOLERANCE && b.distance(to) < TOLERANCE)
                || (a.distance(to) < TOLERANCE && b.distance(from) < TOLERANCE)
        })
        .unwrap_or_else(|| panic!("a trait from {from} to {to}"))
}

#[test]
fn a_compacted_mirror_keeps_its_copy_symmetric() {
    let compacted = a_half_profile_mirrored_and_compacted();

    let mut sketch = compacted.clone();
    let (corner, its_twin) = (
        point_in(&sketch, DVec2::new(3.0, 4.0)),
        point_in(&sketch, DVec2::new(-3.0, 4.0)),
    );
    drag(&mut sketch, corner, DVec2::new(1.0, 1.0));
    let image = sketch.point(corner) * DVec2::new(-1.0, 1.0);
    assert!(
        sketch.point(its_twin).distance(image) < 1e-6,
        "after a compaction the copy no longer follows: the corner went to {}, its twin \
         stands at {}",
        sketch.point(corner),
        sketch.point(its_twin)
    );

    let mut sketch = compacted.clone();
    let head = point_in(&sketch, DVec2::new(0.0, 4.0));
    drag(&mut sketch, head, DVec2::new(1.0, 1.0));
    assert!(
        sketch.point(head).x.abs() < 1e-6,
        "after a compaction the point shared on the axis left it for {}",
        sketch.point(head)
    );

    let copied_base = trait_in(&compacted, DVec2::new(0.0, 1.0), DVec2::new(-3.0, 1.0));
    let copied_side = trait_in(&compacted, DVec2::new(-3.0, 1.0), DVec2::new(-3.0, 4.0));
    assert!(
        compacted.carries(Constraint::Perpendicular {
            first: copied_base,
            second: copied_side,
        }),
        "after a compaction the copy lost the rule it carried: {:?}",
        compacted.constraints()
    );
}
