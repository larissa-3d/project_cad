//! What app · screens/viewport/render/emphasis.rs is held to.
//!
//! Closes #320.
//! - While one of them is selected, its twin is highlighted with it, so what
//!   will go is seen before it goes —
//!   `a_trait_held_by_the_selection_lights_its_mirrored_twin`
//!
//! The issue's other criteria are held under their own `Closes #320` blocks, in
//! `crates/sketch/tests/a_mirrored_copy_stays_symmetric_to_its_original.rs` and
//! `crates/part/tests/mirroring_a_selection.rs`.

use super::*;
use cao_part::history::PointRef;
use cao_part::{Operation, PartDocument};
use cao_sketch::{ChosenAxis, Element, SegmentId, SketchAxis, ToolState, WorkPlane};

use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, SketchPhase, Tool};

#[test]
fn a_piece_the_click_has_not_laid_yet_is_drawn_faint_rather_than_as_itself() {
    let theme = Theme::default();

    let (shown, width) = drawn_as(&theme, false, false, true, PLAIN, 1.5);

    assert_eq!(
        shown,
        ghost(&theme),
        "what the click would lay reads as a promise, not as drawing already there"
    );
    assert_eq!(width, 1.5, "a promise is faint, not thick");
}
use glam::{DVec2, Vec3};

const TOLERANCE: f64 = 1e-9;
const PLAIN: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

#[test]
fn what_a_rule_has_been_shown_is_not_drawn_in_the_colour_the_cursor_paints_with() {
    let theme = Theme::default();

    let (picked, _) = drawn_as(&theme, true, true, false, PLAIN, 1.0);
    let (hovered, _) = drawn_as(&theme, false, true, false, PLAIN, 1.0);

    assert_eq!(picked, tint_at(theme.picked, 1.0));
    assert_ne!(
        picked, hovered,
        "an element the rule already holds reads as one the next click would take",
    );
}

#[test]
fn a_piece_of_the_drawing_no_one_is_dealing_with_keeps_the_colour_it_came_with() {
    let (color, width) = drawn_as(&Theme::default(), false, false, false, PLAIN, 1.5);

    assert_eq!((color, width), (PLAIN, 1.5));
}

#[test]
fn an_axis_shown_to_a_rule_is_drawn_out_past_both_ends_of_the_drawing() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(-30.0, 5.0));
    let end = sketch.add_point(DVec2::new(30.0, 5.0));
    sketch.add_segment(start, end);

    let mut out = Vec::new();
    push_picked_axes(
        &mut out,
        &sketch,
        &Theme::default(),
        &[RulePick::Axis(SketchAxis::V)],
    );

    let places: Vec<DVec2> = out
        .iter()
        .map(|vertex| {
            sketch
                .plane
                .to_local(Vec3::from(vertex.position).as_dvec3())
        })
        .collect();
    assert_eq!(places.len(), 2, "an axis is one straight step, so two ends");
    assert!(
        places.iter().all(|place| place.x.abs() < TOLERANCE),
        "the vertical axis was drawn somewhere other than x = 0: {places:?}",
    );
    assert!(
        places.iter().any(|place| place.y <= -30.0) && places.iter().any(|place| place.y >= 30.0),
        "the axis stops short of the drawing it is meant to run past: {places:?}",
    );
}

#[test]
fn a_rule_shown_no_axis_draws_none() {
    let sketch = Sketch::new(WorkPlane::XY);
    let theme = Theme::default();

    let mut nothing = Vec::new();
    push_picked_axes(&mut nothing, &sketch, &theme, &[]);
    let mut a_trait = Vec::new();
    push_picked_axes(
        &mut a_trait,
        &sketch,
        &theme,
        &[RulePick::Element(Element::Segment(SegmentId(0)))],
    );

    assert!(nothing.is_empty());
    assert!(
        a_trait.is_empty(),
        "a trait shown to the rule brought an axis along with it",
    );
}

fn a_trait_mirrored_across_the_v_axis() -> (PartDocument, SegmentId, SegmentId) {
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
    document.apply(Operation::Mirror {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        axis: ChosenAxis::Sketch(SketchAxis::V),
    });
    let copy = document.sketches()[0]
        .live_segments()
        .map(|(id, _)| id)
        .find(|id| *id != SegmentId(0))
        .expect("the mirror laid a copy");
    (document, SegmentId(0), copy)
}

fn colour_of(document: &mut PartDocument, held: Element, drawn: Element) -> [f32; 4] {
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Select,
        tool_state: ToolState::Select(Box::new(cao_sketch::SelectState {
            held: vec![Selection::Element(held)],
            ..Default::default()
        })),
        ..Default::default()
    };
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let context = SketchContext {
        document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let theme = Theme::default();
    mark(&context, &theme, Selection::Element(drawn), &[], PLAIN, 1.0).0
}

#[test]
fn a_trait_held_by_the_selection_lights_its_mirrored_twin() {
    let (mut document, original, copy) = a_trait_mirrored_across_the_v_axis();
    let lit = tint_at(Theme::default().highlight, 1.0);
    assert_eq!(
        colour_of(
            &mut document,
            Element::Segment(original),
            Element::Segment(original)
        ),
        lit,
        "the trait held is not lit itself, so nothing here can say anything of its twin"
    );

    assert_eq!(
        colour_of(
            &mut document,
            Element::Segment(original),
            Element::Segment(copy)
        ),
        lit,
        "the original is held and its copy, which erasing it would take too, is not lit"
    );
    assert_eq!(
        colour_of(
            &mut document,
            Element::Segment(copy),
            Element::Segment(original)
        ),
        lit,
        "the copy is held and its original, which erasing it would take too, is not lit"
    );
}
