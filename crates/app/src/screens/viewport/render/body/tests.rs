//! What app · screens/viewport/render/body.rs is held to.
//!
//! Closes #343. The issue carries no "Done when": the bullets of "What it
//! does" and "What must not break" stand as its criteria, word for word.
//! - A curved face is drawn smooth: the light varies continuously across it,
//!   and its facets can no longer be told apart by their shade —
//!   `a_curved_face_is_drawn_smooth_with_no_facet_told_apart_by_its_shade`
//! - Flat faces are drawn exactly as today —
//!   `a_flat_face_is_drawn_in_the_one_shade_the_way_it_faces_gives_it`
//! - The edge where a curved face meets another face stays sharp: the top of a
//!   cylinder does not blur into its wall —
//!   `the_top_of_a_shaft_does_not_blur_into_its_wall`
//! - The geometry is unchanged: only how it is lit —
//!   `the_part_is_drawn_with_the_very_triangles_it_is_made_of`
//! - The light following the camera: a face turned towards the viewer is
//!   always the bright one —
//!   `the_side_of_a_curved_wall_turned_to_the_eye_is_the_bright_one`
//! - The hovered face, the refused tint of #192, and the matter in front of a
//!   sketch plane (#319) — no test: in this file; the hovered face and the
//!   refused tint are flat fills of their own, `push_under_the_cursor` in
//!   `render/planes.rs`, which shade nothing, held by
//!   `the_face_under_the_cursor_is_lit_with_triangles_the_part_is_drawn_with`
//!   in `render/planes/tests.rs`; the matter in front of a sketch plane by the
//!   tests under `Closes #319` in `render/tests.rs` and `matter/tests.rs`
//! - The part's picture in the explorer — no test: in this file; held by
//!   `a_part_that_has_been_extruded_shows_its_matter` in
//!   `crates/app/src/picture/scene/tests.rs`

use cao_part::{ExtrusionMode, Operation, PartDocument, PointRef};
use cao_prefs::config::ViewportConfig;
use cao_render::camera::OrbitCamera;
use cao_render::{SceneFrame, Vertex};
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::{DVec2, Vec3};

use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::render::build_frame;
use crate::screens::viewport::{ViewScale, ViewportState};

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

const SHAFT_RADIUS: f64 = 10.0;
const SHAFT_HEIGHT: f64 = 30.0;

/// Two corners closer than this are one point of the part: the kernel hands
/// the same point out a few ulps apart from one triangle to the next.
const SAME_POINT: f32 = 1e-4;

const SAME_SHADE: f32 = 1e-4;

/// A shaft Ø20 raised 30 on XY: two flat ends and one curved wall.
fn a_shaft() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        radius: SHAFT_RADIUS,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::ZERO]),
        distance: SHAFT_HEIGHT.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

fn a_frame_of(state: &ViewportState, document: &mut PartDocument) -> SceneFrame {
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let millimetres = document.scale();
    let context = SketchContext {
        document,
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

    build_frame(state, SCREEN, SCREEN, scale, &context, None, None)
}

fn triangles(vertices: &[Vertex]) -> impl Iterator<Item = &[Vertex; 3]> {
    vertices.as_chunks::<3>().0.iter()
}

fn at(vertex: &Vertex) -> Vec3 {
    Vec3::from(vertex.position)
}

/// The corners of the shaft's wall as drawn opaque: every triangle reaching
/// from one end to the other, where an end's triangles lie at one height.
fn corners_of_the_wall(frame: &SceneFrame) -> Vec<Vertex> {
    triangles(&frame.scene_solids)
        .filter(|corners| {
            let height = at(&corners[0]).z;
            corners
                .iter()
                .any(|corner| (at(corner).z - height).abs() > SAME_POINT)
        })
        .flatten()
        .copied()
        .collect()
}

fn brightness(vertex: &Vertex) -> f32 {
    vertex.color[0] + vertex.color[1] + vertex.color[2]
}

#[test]
fn a_curved_face_is_drawn_smooth_with_no_facet_told_apart_by_its_shade() {
    let wall = corners_of_the_wall(&a_frame_of(&ViewportState::default(), &mut a_shaft()));
    assert!(!wall.is_empty(), "the shaft's wall is drawn");

    for (index, corner) in wall.iter().enumerate() {
        for other in &wall[index + 1..] {
            if at(corner).distance(at(other)) > SAME_POINT {
                continue;
            }
            let apart = corner
                .color
                .iter()
                .zip(other.color)
                .map(|(one, another)| (one - another).abs())
                .fold(0.0f32, f32::max);
            assert!(
                apart <= SAME_SHADE,
                "two facets of the wall meet at {:?} lit {:?} and {:?}: they are told \
                 apart by their shade",
                at(corner),
                corner.color,
                other.color,
            );
        }
    }

    let (darkest, brightest) = wall
        .iter()
        .map(brightness)
        .fold((f32::MAX, f32::MIN), |(low, high), shade| {
            (low.min(shade), high.max(shade))
        });
    assert!(
        brightest > darkest * 1.2,
        "the wall is lit from {darkest} to {brightest}: the light does not vary across it",
    );
}
