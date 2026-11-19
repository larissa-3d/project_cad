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
use cao_render::camera::{OrbitCamera, view_angles_towards};
use cao_render::{SceneFrame, Vertex};
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::{DVec2, Vec3};

use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::render::{build_frame, tint};
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

/// A block 10 by 10 and 4 high over XY: six flat faces.
fn a_block() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 5.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

/// The colour a corner facing `normal` was given before #343, the light
/// following the camera of `state`.
fn lit_as_before(state: &ViewportState, normal: Vec3) -> [f32; 4] {
    let towards = state.camera.forward();
    let light = (towards.normalize_or(Vec3::NEG_Z) * -1.0 + Vec3::new(0.35, 0.2, 0.55))
        .normalize_or(Vec3::Z);
    let shade = 0.42 + 0.58 * normal.normalize_or_zero().dot(light).max(0.0);
    let solid = tint(state.theme.solid);
    [
        solid[0] * shade,
        solid[1] * shade,
        solid[2] * shade,
        solid[3],
    ]
}

fn apart(one: [f32; 4], another: [f32; 4]) -> f32 {
    one.iter()
        .zip(another)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max)
}

fn facing(corners: &[Vertex; 3]) -> Vec3 {
    let [a, b, c] = corners.map(|corner| at(&corner));
    (b - a).cross(c - a)
}

fn lies_level(corners: &[Vertex; 3]) -> bool {
    let height = at(&corners[0]).z;
    corners
        .iter()
        .all(|corner| (at(corner).z - height).abs() <= SAME_POINT)
}

#[test]
fn a_flat_face_is_drawn_in_the_one_shade_the_way_it_faces_gives_it() {
    let state = ViewportState::default();
    let block = a_frame_of(&state, &mut a_block());
    let shaft = a_frame_of(&state, &mut a_shaft());
    let flat = triangles(&block.scene_solids)
        .chain(triangles(&shaft.scene_solids).filter(|corners| lies_level(corners)));

    let mut seen = 0;
    for corners in flat {
        let wanted = lit_as_before(&state, facing(corners));
        for corner in corners {
            assert!(
                apart(corner.color, wanted) <= SAME_SHADE,
                "a corner of a flat face at {:?} is lit {:?}, where the way the face \
                 faces lit it {wanted:?}",
                at(corner),
                corner.color,
            );
        }
        seen += 1;
    }
    assert!(
        seen > 12,
        "the block's faces and the shaft's ends are drawn"
    );
}

/// How far a corner of the wall may be lit from the true cylinder's normal at
/// it: the corner faces the average of the facets meeting there, a few degrees
/// apart, where blurring it into the top tilts it by tens of degrees.
const LIT_AS_THE_CYLINDER: f32 = 0.01;

#[test]
fn the_top_of_a_shaft_does_not_blur_into_its_wall() {
    let state = ViewportState::default();
    let frame = a_frame_of(&state, &mut a_shaft());
    let top = SHAFT_HEIGHT as f32;
    let on_the_rim = |corner: &Vertex| (at(corner).z - top).abs() <= SAME_POINT;
    let lit_as_the_top = lit_as_before(&state, Vec3::Z);

    let (mut on_the_top, mut on_the_wall) = (0, 0);
    let mut a_blur_would_show = 0.0f32;
    for corners in triangles(&frame.scene_solids) {
        let level = lies_level(corners);
        for corner in corners.iter().filter(|corner| on_the_rim(corner)) {
            let wanted = if level {
                on_the_top += 1;
                lit_as_the_top
            } else {
                on_the_wall += 1;
                let outwards = (at(corner) * Vec3::new(1.0, 1.0, 0.0)).normalize();
                let blurred = lit_as_before(&state, outwards + Vec3::Z);
                let wanted = lit_as_before(&state, outwards);
                a_blur_would_show = a_blur_would_show.max(apart(blurred, wanted));
                wanted
            };
            let tolerance = if level {
                SAME_SHADE
            } else {
                LIT_AS_THE_CYLINDER
            };
            assert!(
                apart(corner.color, wanted) <= tolerance,
                "the rim at {:?} is lit {:?} on the {}, where {wanted:?} was wanted: \
                 the top and the wall blur into each other",
                at(corner),
                corner.color,
                if level { "top" } else { "wall" },
            );
        }
    }
    assert!(
        on_the_top > 0 && on_the_wall > 0,
        "the rim is drawn on the top {on_the_top} times and on the wall {on_the_wall}",
    );
    assert!(
        a_blur_would_show > 5.0 * LIT_AS_THE_CYLINDER,
        "the view chosen lights a wall blurred into the top as it lights the wall, \
         at most {a_blur_would_show} apart, and cannot tell a blur",
    );
}

#[test]
fn the_part_is_drawn_with_the_very_triangles_it_is_made_of() {
    let mut shaft = a_shaft();
    let mut made_of: Vec<[[f32; 3]; 3]> = shaft
        .body()
        .triangles()
        .iter()
        .map(|corners| corners.map(|corner| corner.as_vec3().to_array()))
        .collect();
    let frame = a_frame_of(&ViewportState::default(), &mut shaft);
    let mut drawn: Vec<[[f32; 3]; 3]> = triangles(&frame.scene_solids)
        .map(|corners| corners.map(|corner| corner.position))
        .collect();

    let order = |one: &[[f32; 3]; 3], another: &[[f32; 3]; 3]| {
        one.as_flattened()
            .iter()
            .zip(another.as_flattened())
            .map(|(a, b)| a.total_cmp(b))
            .find(|ordering| ordering.is_ne())
            .unwrap_or(std::cmp::Ordering::Equal)
    };
    made_of.sort_by(order);
    drawn.sort_by(order);
    assert_eq!(
        drawn, made_of,
        "the part is drawn with other triangles than it is made of, or wound otherwise",
    );
}

#[test]
fn the_side_of_a_curved_wall_turned_to_the_eye_is_the_bright_one() {
    for step in 0..8 {
        let around = std::f32::consts::TAU * step as f32 / 8.0;
        let looking_from = Vec3::new(around.cos(), around.sin(), 0.35);
        let mut state = ViewportState::default();
        let (yaw, pitch) = view_angles_towards(looking_from);
        state.camera.set_view_angles(yaw, pitch);
        let eye = (state.camera.eye() * Vec3::new(1.0, 1.0, 0.0)).normalize();

        let wall = corners_of_the_wall(&a_frame_of(&state, &mut a_shaft()));
        let brightest = wall
            .iter()
            .max_by(|one, another| brightness(one).total_cmp(&brightness(another)))
            .expect("the wall is drawn");
        let outwards = (at(brightest) * Vec3::new(1.0, 1.0, 0.0)).normalize();

        assert!(
            outwards.angle_between(eye) < 45f32.to_radians(),
            "seen from {eye:?}, the wall is brightest at {:?}, facing {outwards:?}",
            at(brightest),
        );
    }
}
