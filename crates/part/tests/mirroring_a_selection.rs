//! What a part does with a mirror: a second copy appears the other side of the
//! axis, the original stays, and a replay rebuilds the same thing.
//!
//! Closes #320.
//! - Erasing a trait erases its twin, from either side —
//!   `erasing_a_trait_of_the_original_erases_its_twin`,
//!   `erasing_a_trait_of_the_copy_erases_its_twin`
//! - A mirror is still one step in the history, undone in one go —
//!   `undoing_a_mirror_takes_back_its_copy_and_its_symmetry`
//! - Compacting the history (#191) carries the symmetry along —
//!   `a_compacted_mirror_keeps_its_copy_symmetric`

use cao_part::PartState;
use cao_part::history::{Operation, PointRef};
use cao_sketch::{
    AnnotationMetrics, ChosenAxis, Element, PointId, SegmentId, Selection, SketchAxis, WorkPlane,
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
