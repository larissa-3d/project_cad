//! What app · screens/viewport/render/extrusion.rs is held to.
//!
//! Closes #740 — the whole transcription is in
//! `crates/app/tests/extruding_in_a_panel.rs`; this is the drawing's half of
//! criterion 5, the volume itself being the part's and what is previewed the
//! presenter's.
//! - The view draws the extruded volume alone, not fused into the part,
//!   see-through, green for an addition and red for a cut, and redraws it at
//!   every keystroke — the drawing's half: the frame carries the volume
//!   previewed, rising from the area to the depth typed, see-through in the
//!   theme's colour of an addition or of a cut as the mode says, the same
//!   volume either way; and nothing standing off the plane while nothing is
//!   previewed —
//!   `the_frame_carries_the_volume_previewed_see_through_in_the_colour_of_its_mode`

use cao_part::PartDocument;
use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_prefs::config::ViewportConfig;
use cao_prefs::theme::Rgba;
use cao_render::SceneFrame;
use cao_render::camera::OrbitCamera;
use cao_sketch::WorkPlane;
use chrono::Utc;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::{ExtrusionState, Shape};
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::render::build_frame;
use crate::screens::viewport::{ViewScale, ViewportState};

const INSIDE: DVec2 = DVec2::new(5.0, 10.0);

/// A drawing with one closed area in it, which is what an extrusion picks
/// from.
fn a_part_with_one_area() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 20.0)),
        construction: false,
    });
    document
}

/// What one frame paints of an extrusion being set up.
fn shown(
    prepare: impl FnOnce(&mut ExtrusionState),
) -> (Vec<cao_render::Vertex>, Vec<cao_render::Vertex>) {
    let mut document = a_part_with_one_area();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    extrusion.offer(0);
    prepare(&mut extrusion);

    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let (mut surfaces, mut lines) = (Vec::new(), Vec::new());
    push_chosen_areas(&mut surfaces, &mut lines, &Theme::default(), &context);
    (surfaces, lines)
}

#[test]
fn nothing_is_tinted_while_the_tool_is_only_offered() {
    let (surfaces, lines) = shown(|_| {});

    assert!(
        surfaces.is_empty() && lines.is_empty(),
        "an area was filled before the tool was armed",
    );
}

#[test]
fn the_area_picked_is_filled_with_the_matter_it_would_become() {
    let (added, _) = shown(|extrusion| {
        extrusion.arm(ExtrusionMode::Add);
        extrusion.picks = vec![INSIDE];
    });
    let (cut, _) = shown(|extrusion| {
        extrusion.arm(ExtrusionMode::Cut);
        extrusion.picks = vec![INSIDE];
    });

    assert!(!added.is_empty(), "the area picked was left untinted");
    assert_eq!(
        added.len(),
        cut.len(),
        "taking matter away covers a different area from adding it",
    );
    assert_ne!(
        added[0].color, cut[0].color,
        "matter added and matter taken away are shown in the same colour",
    );
}

#[test]
fn an_area_only_pointed_at_is_shown_apart_from_one_already_picked() {
    let (hovered, _) = shown(|extrusion| {
        extrusion.arm(ExtrusionMode::Add);
        extrusion.hovered = Some(0);
    });
    let (picked, _) = shown(|extrusion| {
        extrusion.arm(ExtrusionMode::Add);
        extrusion.picks = vec![INSIDE];
    });

    assert!(
        !hovered.is_empty(),
        "the area under the cursor shows nothing"
    );
    assert_ne!(
        hovered[0].color, picked[0].color,
        "pointing at an area looks exactly like having chosen it",
    );
}

#[test]
fn a_revolution_shows_the_axis_it_would_turn_around() {
    let (_, straight) = shown(|extrusion| {
        extrusion.arm(ExtrusionMode::Add);
        extrusion.picks = vec![INSIDE];
    });
    let (_, revolving) = shown(|extrusion| {
        extrusion.arm(ExtrusionMode::Add);
        extrusion.picks = vec![INSIDE];
        extrusion.shape = Shape::Revolution;
    });

    assert!(
        straight.is_empty(),
        "a straight extrusion drew an axis it does not turn around",
    );
    assert_eq!(
        revolving.len(),
        2,
        "the axis of a revolution is one straight line and nothing else",
    );
}

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

/// How far a corner drawn in `f32` may stand from where the arithmetic puts
/// it.
const PLACE: f32 = 1e-3;

/// One whole frame of the part with one area, no sketch open, its extrusion
/// set up as `prepare` says, in the view as it opens.
fn a_frame(prepare: impl FnOnce(&mut ExtrusionState)) -> SceneFrame {
    let state = ViewportState::default();
    let mut document = a_part_with_one_area();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    extrusion.offer(0);
    prepare(&mut extrusion);
    let millimetres = document.scale();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let scale = ViewScale::of(
        &OrbitCamera::default(),
        SCREEN,
        1.0,
        &ViewportConfig::default(),
        millimetres,
    );

    build_frame(&state, SCREEN, SCREEN, scale, &context, None, None)
}

/// Whether a vertex wears `colour` as a see-through surface lit by the view
/// does: the colour shaded between the ambient light and the full, at an
/// opacity above none and below one.
fn wears(vertex: &cao_render::Vertex, colour: Rgba) -> bool {
    let token = tint(colour);
    let [red, green, blue, opacity] = vertex.color;
    let shade = red / token[0];
    (0.4..=1.0 + 1e-4).contains(&shade)
        && (green - token[1] * shade).abs() < 1e-4
        && (blue - token[2] * shade).abs() < 1e-4
        && opacity > 0.0
        && opacity < 1.0
}

/// The corners of every triangle the frame's see-through layer draws in
/// `colour` standing off the sketch's plane: a volume, and never the area
/// picked, which lies on the plane in the same colour.
fn worn_off_the_plane(frame: &SceneFrame, colour: Rgba) -> Vec<[[f32; 3]; 3]> {
    frame
        .scene_surfaces
        .as_chunks::<3>()
        .0
        .iter()
        .filter(|triangle| triangle.iter().all(|corner| wears(corner, colour)))
        .filter(|triangle| triangle.iter().any(|corner| corner.position[2] > PLACE))
        .map(|triangle| triangle.each_ref().map(|corner| corner.position))
        .collect()
}

/// The area picked, 30 deep, as `mode`.
fn thirty_deep(mode: ExtrusionMode) -> impl FnOnce(&mut ExtrusionState) {
    move |extrusion| {
        extrusion.arm(mode);
        extrusion.picks = vec![INSIDE];
        extrusion.distance_input = "30".to_string();
    }
}

#[test]
fn the_frame_carries_the_volume_previewed_see_through_in_the_colour_of_its_mode() {
    let theme = ViewportState::default().theme;
    let added = a_frame(thirty_deep(ExtrusionMode::Add));
    let cut = a_frame(thirty_deep(ExtrusionMode::Cut));

    let green = worn_off_the_plane(&added, theme.extrusion_add);
    assert!(
        !green.is_empty(),
        "an addition previewed, the frame carries no see-through volume in the colour of an \
         addition",
    );
    let corners: Vec<[f32; 3]> = green.iter().flatten().copied().collect();
    let top = corners.iter().map(|[_, _, z]| *z).fold(f32::MIN, f32::max);
    assert!(
        (top - 30.0).abs() < PLACE,
        "the volume previewed rises to {top}, not to the 30 typed",
    );
    let within = |value: f32, high: f32| (-PLACE..=high + PLACE).contains(&value);
    assert!(
        corners
            .iter()
            .all(|[x, y, z]| within(*x, 10.0) && within(*y, 20.0) && within(*z, 30.0)),
        "the volume previewed stands outside the 10 by 20 area raised 30: {corners:?}",
    );
    assert!(
        worn_off_the_plane(&added, theme.extrusion_cut).is_empty(),
        "an addition is previewed in the colour of a cut",
    );

    assert_eq!(
        worn_off_the_plane(&cut, theme.extrusion_cut),
        green,
        "the same step as a cut is not the same volume previewed, in the colour of a cut",
    );
    assert!(
        worn_off_the_plane(&cut, theme.extrusion_add).is_empty(),
        "a cut is previewed in the colour of an addition",
    );

    let unread = a_frame(|extrusion| {
        thirty_deep(ExtrusionMode::Add)(extrusion);
        extrusion.distance_input = "depth * 2".to_string();
    });
    assert!(
        worn_off_the_plane(&unread, theme.extrusion_add).is_empty(),
        "a depth that does not read is previewed all the same",
    );
}
