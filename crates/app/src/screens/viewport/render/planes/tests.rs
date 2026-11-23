//! What app · screens/viewport/render/planes.rs is held to.
//!
//! Closes #526.
//! - the face under the cursor is lit with the triangles the exact kernel
//!   draws the part with —
//!   `the_face_under_the_cursor_is_lit_with_triangles_the_part_is_drawn_with`
//!
//! Closes #734 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests.rs`; this is the bullet the
//! light under the cursor answers.
//! - found in review: the face under the cursor is lit as it is drawn, every
//!   triangle of the drawn face, whatever outline the part gives it — with a
//!   cut invisible, the whole top it fills in —
//!   `the_top_an_invisible_cut_fills_in_is_lit_whole_under_the_cursor`; a
//!   curved one too: a see-through hole's ghost, pointed at above the block,
//!   lights the ghost's wall, not the hole's wall in the part that carries
//!   the same number —
//!   `a_ghost_s_curved_wall_under_the_cursor_lights_itself_not_the_part_s_face_of_its_number`
//!
//! Closes #972.
//! - with the section hiding the matter and a sketch edited, the sketch's
//!   plane is in the list of things lit; a test on the presenter checks it
//!   with no window — reworded: the sketch is being started, and its plane is
//!   the cut offered under the cursor while the plane of a new sketch is
//!   chosen. It is lit in the highlight, as a face of the part is, whether the
//!   section hides the matter over it or draws it see-through, and whether the
//!   section is the one open or one the part keeps —
//!   `the_cut_under_the_cursor_is_lit_as_a_face_is_while_the_section_hides_or_veils_the_matter_over_it`.
//!   The plane of a sketch already open is lit on the orientation cube
//!   whatever the section does, and is not what the issue found missing.
//! - without a section, nothing changes; the existing tests stay green — a
//!   face under the cursor is lit in the highlight as it was, with no section
//!   and under one that draws it opaque —
//!   `a_face_under_the_cursor_is_lit_in_the_highlight_with_no_section_as_under_one_showing_it`

use cao_part::PartDocument;
use cao_part::history::{ExtrusionMode, Operation, PointRef, SectionAsked, SideDrawing};
use cao_render::MatterInFront;
use chrono::Utc;
use glam::{DVec2, DVec3, Vec3};

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewScale;
use crate::screens::viewport::input::handle_sketch_input;
use crate::screens::viewport::section::Section;

/// What the three planes are painted, when one of them is being pointed at.
fn offered(hovered: Option<PlaneChoice>) -> (Vec<cao_render::Vertex>, Vec<cao_render::Vertex>) {
    let state = ViewportState::default();
    let mut document = PartDocument::new("part", Utc::now());
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    editor.hovered_plane = hovered;

    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    let (mut surfaces, mut lines) = (Vec::new(), Vec::new());
    push_choosable_planes(&mut surfaces, &mut lines, &state, &context);
    (surfaces, lines)
}

/// The corners each plane's quad is drawn at, one group per plane, in the
/// order [`WorkPlane::ORIGIN_PLANES`] gives them.
fn per_plane(painted: &[cao_render::Vertex]) -> Vec<&[cao_render::Vertex]> {
    painted
        .chunks(painted.len() / WorkPlane::ORIGIN_PLANES.len())
        .collect()
}

#[test]
fn every_plane_a_sketch_can_start_on_is_offered() {
    let (surfaces, lines) = offered(None);

    assert!(
        !surfaces.is_empty() && !lines.is_empty(),
        "a plane is drawn neither as a surface to click nor as an outline to see",
    );
    for painted in [&surfaces, &lines] {
        assert_eq!(
            painted.len() % WorkPlane::ORIGIN_PLANES.len(),
            0,
            "the three planes are not drawn alike",
        );
    }
    for (index, plane) in WorkPlane::ORIGIN_PLANES.iter().enumerate() {
        let normal = plane.normal().as_vec3();
        for vertex in per_plane(&surfaces)[index] {
            let off = (Vec3::from(vertex.position) - plane.origin.as_vec3()).dot(normal);
            assert!(
                off.abs() < 1e-5,
                "a corner of the plane at {index} stands {off} away from the plane itself",
            );
        }
    }
}

#[test]
fn the_plane_under_the_cursor_is_drawn_apart_from_the_other_two() {
    let (plain, plain_lines) = offered(None);
    let (hovered, hovered_lines) = offered(Some(PlaneChoice::Origin(0)));

    assert_eq!(
        hovered.len(),
        plain.len(),
        "pointing at a plane changed how many are drawn",
    );
    assert_ne!(
        per_plane(&hovered)[0][0].color,
        per_plane(&plain)[0][0].color,
        "the plane under the cursor is filled exactly like the ones that are not",
    );
    assert_eq!(
        per_plane(&hovered)[1][0].color,
        per_plane(&plain)[1][0].color,
        "pointing at one plane changed the look of another",
    );
    assert!(
        per_plane(&hovered_lines)[0][0].width > per_plane(&plain_lines)[0][0].width,
        "the plane under the cursor is outlined no more strongly than the others",
    );
}

#[test]
fn a_part_with_no_face_at_all_is_lit_up_on_none() {
    let mut surfaces = Vec::new();
    let state = ViewportState::default();
    let mut document = PartDocument::new("part", Utc::now());
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    push_hovered_face(&mut surfaces, tint(state.theme.refused), &context, 0);

    assert!(
        surfaces.is_empty(),
        "a part with no face at all was lit up on one",
    );
}

/// A circle 40 across raised 10, which the exact kernel computes.
fn a_raised_disc() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        radius: 20.0,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::ZERO]),
        distance: 10.0.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

/// What lighting `face` paints, three corners to a triangle.
fn lit(document: &mut PartDocument, face: usize) -> Vec<[Vec3; 3]> {
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let context = SketchContext {
        document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let mut surfaces = Vec::new();
    push_hovered_face(&mut surfaces, [1.0; 4], &context, face);
    surfaces
        .chunks(3)
        .map(|corners| corners.iter().map(|at| Vec3::from(at.position)))
        .map(|mut corners| std::array::from_fn(|_| corners.next().expect("three corners")))
        .collect()
}

#[test]
fn the_face_under_the_cursor_is_lit_with_triangles_the_part_is_drawn_with() {
    let mut document = a_raised_disc();
    let drawn: Vec<[Vec3; 3]> = document
        .body()
        .triangles()
        .iter()
        .map(|triangle| triangle.map(|corner| corner.as_vec3()))
        .collect();
    let wall = document
        .body()
        .ray_hit(DVec3::new(60.0, 0.0, 5.0), DVec3::NEG_X)
        .expect("the wall is there to be hit")
        .face;

    let on_the_wall = lit(&mut document, wall);
    assert!(!on_the_wall.is_empty(), "the wall under the cursor is lit");
    for triangle in &on_the_wall {
        assert!(
            drawn.contains(triangle),
            "{triangle:?} is lit, and the part is not drawn with it",
        );
        for corner in triangle {
            let off = corner.truncate().length() - 20.0;
            assert!(
                off.abs() < 1e-4,
                "{corner:?} is lit {off} off the wall of the disc",
            );
        }
    }
    let every_face: usize = (0..document.body().faces_end())
        .map(|face| lit(&mut document, face).len())
        .sum();
    assert_eq!(
        every_face,
        drawn.len(),
        "lit face by face, the part is not lit with every triangle it is drawn \
         with, once each",
    );
}

/// Where the cut of `a_block_notched_deep_at_its_corner` is.
const THE_NOTCH: usize = 4;

/// A block 10 by 10 and 4 high at the origin, notched through by a square
/// from (3, 3) to (15, 15): its top in the part a thin L, 3 wide. Both squares
/// are drawn before either step.
fn a_block_notched_deep_at_its_corner() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (corner, opposite) in [
        (DVec2::ZERO, DVec2::splat(10.0)),
        (DVec2::splat(3.0), DVec2::splat(15.0)),
    ] {
        document.apply(Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(corner),
            opposite: PointRef::New(opposite),
            construction: false,
        });
    }
    for (inside, distance, mode) in [
        (
            vec![DVec2::splat(1.0), DVec2::splat(5.0)],
            4.0,
            ExtrusionMode::Add,
        ),
        (
            vec![DVec2::splat(5.0), DVec2::splat(12.0)],
            10.0,
            ExtrusionMode::Cut,
        ),
    ] {
        document.apply(Operation::Extrude {
            sketch: 0,
            areas: document.areas_at(0, &inside),
            distance: distance.into(),
            mode,
        });
    }
    assert!(
        (document.body().volume() - 204.0).abs() < 1e-6,
        "the notched block holds {}, not 400 less 7 by 7 by 4",
        document.body().volume(),
    );
    document
}

/// Whether a triangle lying flat at the height of `point` holds it.
fn holds([a, b, c]: &[Vec3; 3], point: Vec3) -> bool {
    let flat = [a, b, c]
        .iter()
        .all(|corner| (corner.z - point.z).abs() < 1e-3);
    let side = |p: &Vec3, q: &Vec3| (q.x - p.x) * (point.y - p.y) - (q.y - p.y) * (point.x - p.x);
    let turns = [side(a, b), side(b, c), side(c, a)];
    flat && (turns.iter().all(|turn| *turn >= -1e-4) || turns.iter().all(|turn| *turn <= 1e-4))
}

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

/// Where `at` lands on the canvas, seen as `state` sees it.
fn on_the_screen(state: &ViewportState, at: Vec3) -> egui::Pos2 {
    let view_projection = state
        .camera
        .view_projection(SCREEN.width() / SCREEN.height());
    let clip = view_projection * at.extend(1.0);
    let ndc = clip.truncate() / clip.w;
    egui::pos2(
        SCREEN.center().x + ndc.x * SCREEN.width() * 0.5,
        SCREEN.center().y - ndc.y * SCREEN.height() * 0.5,
    )
}

/// What is lit flat on the block's top, at z = 4, once the pointer has rested
/// over `at` while a plane is being chosen for a new sketch, seen from straight
/// above, the notch clicked `clicks` times round its states — and what was
/// offered there.
fn lit_with_the_pointer_over(at: Vec3, clicks: usize) -> (Vec<[Vec3; 3]>, Option<PlaneChoice>) {
    let document = a_block_notched_deep_at_its_corner();
    let mut state = ViewportState::default();
    for _ in 0..clicks {
        state.next_volume_mode(&document, THE_NOTCH);
    }
    let (lit, offered) = lit_while_choosing_a_plane(document, state, Vec3::Z, at);
    let on_the_top = lit
        .into_iter()
        .filter(|triangle| triangle.iter().all(|corner| (corner.z - 4.0).abs() < 1e-3))
        .collect();
    (on_the_top, offered)
}

/// Every triangle lit once the pointer has rested over `at` while a plane is
/// being chosen for a new sketch on `document`, seen as `state` says from the
/// side `from` points to — and what was offered there.
fn lit_while_choosing_a_plane(
    document: PartDocument,
    state: ViewportState,
    from: Vec3,
    at: Vec3,
) -> (Vec<[Vec3; 3]>, Option<PlaneChoice>) {
    let (painted, offered) = painted_while_choosing_a_plane(document, state, from, at);
    let lit = painted
        .chunks(3)
        .map(|corners| std::array::from_fn(|rank| Vec3::from(corners[rank].position)))
        .collect();
    (lit, offered)
}

/// The same, each corner painted with its colour.
fn painted_while_choosing_a_plane(
    mut document: PartDocument,
    mut state: ViewportState,
    from: Vec3,
    at: Vec3,
) -> (Vec<cao_render::Vertex>, Option<PlaneChoice>) {
    let (yaw, pitch) = cao_render::camera::view_angles_towards(from);
    state.camera.set_view_angles(yaw, pitch);
    let pointer = on_the_screen(&state, at);
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    editor.start_choosing_plane(&lang);
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    let egui = egui::Context::default();
    for events in [
        vec![],
        vec![egui::Event::PointerMoved(pointer)],
        vec![egui::Event::PointerMoved(pointer)],
    ] {
        egui.begin_pass(egui::RawInput {
            screen_rect: Some(SCREEN),
            events,
            ..Default::default()
        });
        egui::Area::new(egui::Id::new("the canvas"))
            .fixed_pos(SCREEN.min)
            .show(&egui, |ui| {
                ui.set_min_size(SCREEN.size());
                let response = ui.interact(
                    SCREEN,
                    egui::Id::new("the canvas's surface"),
                    egui::Sense::click_and_drag(),
                );
                let scale = ViewScale::of(&state.camera, SCREEN, 1.0, &state.config, 1.0);
                handle_sketch_input(ui, &mut state, &response, SCREEN, scale, &mut context);
            });
        egui.end_pass().textures_delta.clear();
    }

    let (mut surfaces, mut lines) = (Vec::new(), Vec::new());
    push_choosable_planes(&mut surfaces, &mut lines, &state, &context);
    (surfaces, context.editor.hovered_plane)
}

/// Whether what is lit holds each of these places.
fn covers(lit: &[[Vec3; 3]], places: [Vec3; 2]) -> [bool; 2] {
    places.map(|place| lit.iter().any(|triangle| holds(triangle, place)))
}

/// On the part's L, and over the notch the cut takes away.
const ON_THE_L: Vec3 = Vec3::new(1.0, 1.0, 4.0);
const OVER_THE_NOTCH: Vec3 = Vec3::new(8.0, 8.0, 4.0);

#[test]
fn the_top_an_invisible_cut_fills_in_is_lit_whole_under_the_cursor() {
    let (opaque, offered) = lit_with_the_pointer_over(ON_THE_L, 0);
    assert!(
        matches!(offered, Some(PlaneChoice::Face { .. })),
        "the fixture wants the L of the top offered under the pointer, not {offered:?}",
    );
    assert_eq!(
        covers(&opaque, [ON_THE_L, OVER_THE_NOTCH]),
        [true, false],
        "the cut opaque, the L under the pointer is not lit as drawn, as (on the L, over the \
         notch)",
    );

    let (filled, _) = lit_with_the_pointer_over(ON_THE_L, 2);
    assert_eq!(
        covers(&filled, [ON_THE_L, OVER_THE_NOTCH]),
        [true, true],
        "the cut invisible, the top under the pointer is not lit whole, as drawn, as (on the \
         L, over the notch it fills in)",
    );

    let (over_nothing, offered) = lit_with_the_pointer_over(OVER_THE_NOTCH, 2);
    assert_eq!(
        offered, None,
        "the cut invisible, the top it fills in over the notch, which the part has nothing of, \
         offers something",
    );
    assert!(
        over_nothing.is_empty(),
        "the cut invisible, nothing offered over the notch, and {} triangles are lit on the top",
        over_nothing.len(),
    );
}

/// A block 10 by 20 and 4 high at the origin, a hole of radius 2 at (5, 10)
/// cut 10 up from XY: a cut far taller than the block, its ghost a cylinder
/// standing 6 over the top. The hole's step.
fn a_block_bored_far_past_its_top() -> (PartDocument, usize) {
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
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(5.0, 10.0)),
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    for (inside, distance, mode) in [
        (
            vec![DVec2::new(1.0, 1.0), DVec2::new(5.0, 10.0)],
            4.0,
            ExtrusionMode::Add,
        ),
        (vec![DVec2::new(5.0, 10.0)], 10.0, ExtrusionMode::Cut),
    ] {
        document.apply(Operation::Extrude {
            sketch: 0,
            areas: document.areas_at(0, &inside),
            distance: distance.into(),
            mode,
        });
    }
    (document, 4)
}

#[test]
fn a_ghost_s_curved_wall_under_the_cursor_lights_itself_not_the_part_s_face_of_its_number() {
    let (document, hole) = a_block_bored_far_past_its_top();
    let mut state = ViewportState::default();
    state.next_volume_mode(&document, hole);
    assert_eq!(
        state.volume_mode(&document, hole),
        cao_part::VolumeState::SeeThrough
    );

    let (lit, offered) =
        lit_while_choosing_a_plane(document, state, Vec3::X, Vec3::new(7.0, 10.0, 7.0));

    assert!(
        matches!(offered, Some(PlaneChoice::Curved(_))),
        "the ghost's wall above the block is offered as {offered:?}, not refused as curved",
    );
    let on_the_circle: Vec<[Vec3; 3]> = lit
        .into_iter()
        .filter(|triangle| {
            triangle
                .iter()
                .all(|corner| ((corner.x - 5.0).hypot(corner.y - 10.0) - 2.0).abs() < 1e-3)
        })
        .collect();
    let highest = on_the_circle
        .iter()
        .flatten()
        .map(|corner| corner.z)
        .fold(f32::NEG_INFINITY, f32::max);
    assert!(
        highest > 9.999,
        "pointed at the ghost's wall above the block, what is lit on the hole's circle reaches \
         {highest} high, not the ghost's top at 10: something other than the ghost's wall is lit",
    );
}

/// A block 10 by 10 and 4 high at the origin.
fn a_block() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::splat(10.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::splat(5.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

/// Where the sections below cut the block: across it, at mid-height.
const THE_CUT: f32 = 2.0;

/// The block under the section view open across it at mid-height, the side
/// under the cut drawn `behind` and the side over it `in_front`.
fn the_block_under_the_section_view(
    behind: MatterInFront,
    in_front: MatterInFront,
) -> (PartDocument, ViewportState) {
    let mut section = Section::new(WorkPlane::XY);
    section.offset = f64::from(THE_CUT);
    section.behind = behind;
    section.in_front = in_front;
    let mut state = ViewportState::default();
    state.section = Some(section);
    (a_block(), state)
}

/// The block keeping a section across it at mid-height, switched on, the side
/// under the cut drawn `behind` and the side over it `in_front`.
fn the_block_under_its_kept_section(
    behind: SideDrawing,
    in_front: SideDrawing,
) -> (PartDocument, ViewportState) {
    let mut document = a_block();
    document.apply(Operation::SaveSection(SectionAsked {
        plane: WorkPlane::XY,
        on: None,
        offset: f64::from(THE_CUT),
        behind,
        in_front,
        tilt_about_u_degrees: 0.0,
        tilt_about_v_degrees: 0.0,
        pivot: DVec3::ZERO,
    }));
    let kept = document.sections().first().expect("the section is kept").id;
    let mut state = ViewportState::default();
    state.kept_sections.switch_on(kept);
    state.kept_sections.keep_up_with(&document, None);
    assert!(
        state.section_shown().is_some(),
        "the fixture wants the section the block keeps switched on and cutting it",
    );
    (document, state)
}

/// Every triangle painted in the highlight once the pointer has rested over
/// `at` while the plane of a new sketch is chosen, seen from the side `from`
/// points to — and what was offered there.
fn lit_in_the_highlight(
    (document, state): (PartDocument, ViewportState),
    from: Vec3,
    at: Vec3,
) -> (Vec<[Vec3; 3]>, Option<PlaneChoice>) {
    let highlight = tint(state.theme.highlight);
    let (painted, offered) = painted_while_choosing_a_plane(document, state, from, at);
    let lit = painted
        .chunks(3)
        .filter(|corners| corners.iter().all(|corner| corner.color == highlight))
        .map(|corners| std::array::from_fn(|rank| Vec3::from(corners[rank].position)))
        .collect();
    (lit, offered)
}

/// What is wrong with the light lying flat at `height` over the block, had it
/// lit there the block's face as it is drawn: a place of the face it leaves
/// dark, or a place off the block it lights.
fn lit_wrong_over_the_block_at(lit: &[[Vec3; 3]], height: f32) -> Option<String> {
    let lights = |x: f32, y: f32| {
        let place = Vec3::new(x, y, height);
        lit.iter().any(|triangle| holds(triangle, place))
    };
    let dark: Vec<(f32, f32)> = [(1.0, 1.0), (5.0, 5.0), (9.0, 9.0)]
        .into_iter()
        .filter(|&(x, y)| !lights(x, y))
        .collect();
    if !dark.is_empty() {
        return Some(format!(
            "of {} triangles lit in the highlight, none lies at z = {height} over {dark:?} of the \
             block",
            lit.len(),
        ));
    }
    lights(15.0, 5.0)
        .then(|| format!("the highlight at z = {height} runs past the block, out to (15, 5)"))
}

#[test]
fn the_cut_under_the_cursor_is_lit_as_a_face_is_while_the_section_hides_or_veils_the_matter_over_it()
 {
    use MatterInFront::{Hidden, Opaque, SeeThrough};

    let over_the_cut = Vec3::new(5.0, 5.0, THE_CUT);
    let mut wrong = Vec::new();
    for (what, seen, from) in [
        (
            "the section view hiding the side over the cut, seen from above",
            the_block_under_the_section_view(Opaque, Hidden),
            Vec3::Z,
        ),
        (
            "the section view drawing the side over the cut see-through, seen from above",
            the_block_under_the_section_view(Opaque, SeeThrough),
            Vec3::Z,
        ),
        (
            "the section view hiding the side under the cut, seen from below",
            the_block_under_the_section_view(Hidden, Opaque),
            Vec3::NEG_Z,
        ),
        (
            "a kept section hiding the side over the cut, seen from above",
            the_block_under_its_kept_section(SideDrawing::Opaque, SideDrawing::Hidden),
            Vec3::Z,
        ),
        (
            "a kept section drawing the side over the cut see-through, seen from above",
            the_block_under_its_kept_section(SideDrawing::Opaque, SideDrawing::SeeThrough),
            Vec3::Z,
        ),
    ] {
        let (lit, offered) = lit_in_the_highlight(seen, from, over_the_cut);

        assert!(
            matches!(offered, Some(PlaneChoice::Section(_))),
            "{what}, the fixture wants the cut offered under the pointer, not {offered:?}",
        );
        if let Some(why) = lit_wrong_over_the_block_at(&lit, THE_CUT) {
            wrong.push(format!("{what}: {why}"));
        }
    }
    assert!(
        wrong.is_empty(),
        "pointed at the cut while the plane of a new sketch is chosen, the cut is not lit in the \
         highlight as a face of the part is:\n{}",
        wrong.join("\n"),
    );
}

#[test]
fn a_face_under_the_cursor_is_lit_in_the_highlight_with_no_section_as_under_one_showing_it() {
    use MatterInFront::{Hidden, Opaque};

    const THE_TOP: f32 = 4.0;
    let on_the_top = Vec3::new(5.0, 5.0, THE_TOP);
    let mut wrong = Vec::new();
    for (what, seen) in [
        ("with no section", (a_block(), ViewportState::default())),
        (
            "under the section view drawing both sides opaque",
            the_block_under_the_section_view(Opaque, Opaque),
        ),
        (
            "under the section view drawing the side over the cut opaque and hiding the one \
             under it",
            the_block_under_the_section_view(Hidden, Opaque),
        ),
    ] {
        let (lit, offered) = lit_in_the_highlight(seen, Vec3::Z, on_the_top);

        assert!(
            matches!(offered, Some(PlaneChoice::Face { .. })),
            "{what}, the fixture wants the block's top offered under the pointer, not {offered:?}",
        );
        if let Some(why) = lit_wrong_over_the_block_at(&lit, THE_TOP) {
            wrong.push(format!("{what}: {why}"));
        }
    }
    assert!(
        wrong.is_empty(),
        "pointed at the block's top while the plane of a new sketch is chosen, the top is not lit \
         in the highlight as it was:\n{}",
        wrong.join("\n"),
    );
}
