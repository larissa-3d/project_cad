//! The edge panel while the section view is on: an edge is offered where it
//! is seen, on a side of the plane drawn opaque, and matter the section hides
//! or ghosts hides nothing behind it.
//!
//! Closes #967.
//! - Section on, default sides (behind opaque, in front hidden): the hand over
//!   an edge lying wholly in front of the plane lights nothing, and a click
//!   there chooses nothing —
//!   `with_the_section_on_an_edge_wholly_in_front_of_the_plane_is_neither_lit_nor_chosen`
//! - Section on: an edge inside the part, on the opaque side and seen through
//!   the cut, lights under the hand and is chosen by a click —
//!   `with_the_section_on_an_edge_inside_the_part_seen_through_the_cut_is_lit_and_chosen`
//! - Section on: an edge the plane crosses lights when the hand is on its part
//!   on the opaque side, and not when the hand is on its part on the hidden
//!   side —
//!   `an_edge_the_plane_crosses_is_lit_from_its_part_on_the_opaque_side_and_not_from_its_part_on_the_hidden_side`
//! - Section on with the front side see-through: an edge on the opaque side
//!   behind the ghost is lit and chosen; an edge of the see-through side is
//!   not —
//!   `with_the_front_see_through_an_edge_behind_the_ghost_is_lit_and_chosen_and_an_edge_of_the_ghost_is_not`
//! - Section on with both sides opaque: picking is as with the section off —
//!   `with_both_sides_opaque_an_edge_is_picked_as_with_the_section_off`
//! - An edge chosen that crosses the plane is drawn lit along its whole length
//!   — held whole by the panel and the hand, by
//!   `an_edge_chosen_where_the_plane_crosses_it_is_lit_along_its_whole_length`;
//!   drawn whole, its part on the hidden side included, by
//!   `the_edge_chosen_across_the_section_is_drawn_whole_on_both_sides_of_the_plane`
//!   in `crates/app/src/screens/viewport/render/tests.rs`
//! - Section off: picking is unchanged — on the pocketed block, by
//!   `with_the_section_off_the_pocket_s_floor_is_not_taken_through_the_matter`;
//!   on the plain block, by the picking tests of `the_edge_panel.rs`, which
//!   this issue leaves as they were

mod driver;

use cao_app::app::EDGE_REACH;
use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_sketch::WorkPlane;
use egui_kittest::kittest::{NodeT, Queryable};
use glam::{DVec2, DVec3};

const SIDE: f64 = 100.0;
const HEIGHT: f64 = 50.0;

/// How high the pocket's floor stands, below the cut.
const FLOOR: f64 = 10.0;

/// Where the section view first cuts the block: along XY, through its middle.
const THE_CUT: f64 = HEIGHT / 2.0;

/// A block `SIDE` square and `HEIGHT` high on the plane XY, a square pocket
/// sunk into its top whose floor lies below the cut: seen from the corner the
/// view opens on, the floor's edges are behind the pocket's walls until the
/// section takes the top of those walls away.
fn a_block_with_a_pocket() -> cao_part::PartDocument {
    let mut part =
        cao_part::PartDocument::new("Bloc", "2026-11-16T09:00:00Z".parse().expect("a date"));
    let the_top_looking_down = WorkPlane {
        origin: DVec3::new(0.0, 0.0, HEIGHT),
        u: DVec3::X,
        v: DVec3::NEG_Y,
    };
    for (sketch, plane, corner, opposite, distance, mode) in [
        (
            0,
            WorkPlane::XY,
            DVec2::ZERO,
            DVec2::new(SIDE, SIDE),
            HEIGHT,
            ExtrusionMode::Add,
        ),
        (
            1,
            the_top_looking_down,
            DVec2::new(25.0, -25.0),
            DVec2::new(75.0, -75.0),
            HEIGHT - FLOOR,
            ExtrusionMode::Cut,
        ),
    ] {
        part.apply(Operation::CreateSketch { plane, on: None });
        part.apply(Operation::AddRectangle {
            sketch,
            corner: PointRef::New(corner),
            opposite: PointRef::New(opposite),
            construction: false,
        });
        let areas = part.areas_at(sketch, &[(corner + opposite) / 2.0]);
        part.apply(Operation::Extrude {
            sketch,
            areas,
            distance: distance.into(),
            mode,
        });
    }
    part
}

/// The top edge of the face towards -Y, wholly above the cut.
const THE_TOP_FRONT: [DVec3; 2] = [DVec3::new(0.0, 0.0, HEIGHT), DVec3::new(SIDE, 0.0, HEIGHT)];

/// Three quarters along the top front, where no other edge passes near it on
/// screen: halfway, the bottom edge of the face towards -X is drawn under it.
const ON_THE_TOP_FRONT: DVec3 = DVec3::new(0.75 * SIDE, 0.0, HEIGHT);

/// The upright between the faces towards +X and -Y, which the cut crosses.
const THE_FRONT_UPRIGHT: [DVec3; 2] = [DVec3::new(SIDE, 0.0, 0.0), DVec3::new(SIDE, 0.0, HEIGHT)];

/// Below the cut, on the part of the upright drawn opaque.
const ON_THE_UPRIGHT_BELOW: DVec3 = DVec3::new(SIDE, 0.0, THE_CUT / 2.0);

/// Above the cut, on the part of the upright the section hides.
const ON_THE_UPRIGHT_ABOVE: DVec3 = DVec3::new(SIDE, 0.0, 45.0);

/// The edge of the pocket's floor towards -X, below the cut: from the corner
/// the view opens on, the pocket's front wall stands between it and the eye.
/// The floor's edges along X are drawn over the block's bottom edges.
const THE_FLOOR_S_SIDE: [DVec3; 2] = [DVec3::new(25.0, 25.0, FLOOR), DVec3::new(25.0, 75.0, FLOOR)];

const ON_THE_FLOOR_S_SIDE: DVec3 = DVec3::new(25.0, 50.0, FLOOR);

/// `a_block_with_a_pocket`, written over the block `driver::a_block` left in
/// the projects folder, and opened from the start menu.
fn opened(test: &str) -> driver::App {
    let mut app = driver::a_block(test);
    driver::the_part_put_away(&mut app, test, "Bloc");
    a_block_with_a_pocket()
        .save(
            &cao_app::adapters::files::DiskFiles,
            &driver::the_part_s_file(test, "Bloc"),
            "2026-11-16T10:00:00Z".parse().expect("a date"),
        )
        .expect("the part is written");
    app.query_all_by_label("Bloc")
        .last()
        .expect("the start menu lists the part")
        .click();
    app.run();
    assert!(
        driver::in_the_tree(&app, "Esquisse 2"),
        "the pocketed block did not open, showing {:?}",
        driver::on_screen(&app),
    );
    driver::hold_the_canvas(&mut app);
    app
}

/// The pocketed block and « Chanfrein » open, the section view off.
fn chamfering(test: &str) -> driver::App {
    let mut app = opened(test);
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, "Chanfrein");
    assert!(
        !driver::is_enabled(&app, "Appliquer"),
        "« Appliquer » offered before any edge is chosen",
    );
    app
}

/// The pocketed block, the section view on with its sides as it opens them,
/// and « Chanfrein » open.
fn chamfering_through_the_section(test: &str) -> driver::App {
    let mut app = opened(test);
    driver::click_in_the_bar(&mut app, "Vue en coupe");
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, "Chanfrein");
    for side in ["Plan XY", "Derrière : opaque", "Devant : cachée"] {
        assert!(
            driver::a_button_reads(&app, side),
            "the section view is not on as it opens, « {side} » missing: {:?}",
            driver::on_screen(&app),
        );
    }
    assert!(
        !driver::is_enabled(&app, "Appliquer"),
        "« Appliquer » offered before any edge is chosen",
    );
    app
}

/// Clicks the section's row for the side in front until it reads `state`.
fn the_front_drawn(app: &mut driver::App, state: &str) {
    let wanted = format!("Devant : {state}");
    for _ in 0..3 {
        if driver::a_button_reads(app, &wanted) {
            return;
        }
        let now = app
            .query_all_by_role(egui::accesskit::Role::Button)
            .filter_map(|node| node.accesskit_node().label())
            .find(|read| read.starts_with("Devant : "))
            .unwrap_or_else(|| {
                panic!("no row for the side in front: {:?}", driver::on_screen(app))
            });
        driver::click_the_button(app, &now);
    }
    panic!(
        "the side in front never read « {wanted} »: {:?}",
        driver::on_screen(app)
    );
}

fn hover_at_world_point(app: &mut driver::App, point: DVec3) {
    driver::hold_the_canvas(app);
    let at = driver::world_point_on_screen(app, point);
    app.hover_at(at);
    app.run();
}

fn chosen(app: &driver::App) -> Option<Vec<DVec3>> {
    app.state().the_edge_chosen()
}

fn lit(app: &driver::App) -> Option<Vec<DVec3>> {
    app.state().the_edge_lit_by_the_hand()
}

fn is_the_edge(drawn: Option<Vec<DVec3>>, [start, end]: [DVec3; 2]) -> bool {
    let Some(points) = drawn else {
        return false;
    };
    let near = |a: DVec3, b: DVec3| a.distance(b) < 1e-6;
    match (points.first(), points.last()) {
        (Some(first), Some(last)) => {
            (near(*first, start) && near(*last, end)) || (near(*first, end) && near(*last, start))
        }
        _ => false,
    }
}

/// How far on screen the edge drawn through these points passes from where
/// `point` is drawn.
fn apart_on_screen(app: &driver::App, points: &[DVec3], point: DVec3) -> f32 {
    let at = driver::world_point_on_screen(app, point);
    points
        .windows(2)
        .map(|pair| {
            let start = driver::world_point_on_screen(app, pair[0]);
            let along = driver::world_point_on_screen(app, pair[1]) - start;
            let t = ((at - start).dot(along) / along.length_sq().max(f32::EPSILON)).clamp(0.0, 1.0);
            at.distance(start + along * t)
        })
        .fold(f32::INFINITY, f32::min)
}

/// How near on screen any edge of the part but `this` passes to where `point`
/// is drawn.
fn nearest_other_edge(app: &driver::App, this: [DVec3; 2], point: DVec3) -> f32 {
    a_block_with_a_pocket()
        .body()
        .drawn_edges()
        .into_iter()
        .filter(|edge| !is_the_edge(Some(edge.points.clone()), this))
        .map(|edge| apart_on_screen(app, &edge.points, point))
        .fold(f32::INFINITY, f32::min)
}

fn clear_of_every_other_edge(app: &driver::App, this: [DVec3; 2], point: DVec3, what: &str) {
    let apart = nearest_other_edge(app, this, point);
    assert!(
        apart > 2.0 * EDGE_REACH,
        "{what} stands {apart} points from another edge, within reach of it",
    );
}

#[test]
fn with_the_section_on_an_edge_wholly_in_front_of_the_plane_is_neither_lit_nor_chosen() {
    let mut app = chamfering_through_the_section("an_edge_wholly_in_front_is_not_taken");
    hover_at_world_point(&mut app, ON_THE_UPRIGHT_BELOW);
    assert!(
        is_the_edge(lit(&app), THE_FRONT_UPRIGHT),
        "the hand on the front upright below the cut, where it is drawn, lights {:?}",
        lit(&app),
    );

    let in_front = ON_THE_TOP_FRONT;
    clear_of_every_other_edge(&app, THE_TOP_FRONT, in_front, "the hand on the top front");
    hover_at_world_point(&mut app, in_front);
    assert_eq!(
        lit(&app),
        None,
        "the hand over the top front, wholly on the side the section hides, lights an edge",
    );
    driver::click_at_world_point(&mut app, in_front);
    assert_eq!(
        chosen(&app),
        None,
        "a click over the top front, wholly on the side the section hides, chose an edge",
    );
    assert!(!driver::is_enabled(&app, "Appliquer"));
}

/// Lights, then chooses by a click, the edge `edge` under the hand at
/// `point`, which nothing else stands within reach of.
fn lit_then_chosen(app: &mut driver::App, edge: [DVec3; 2], point: DVec3, what: &str) {
    clear_of_every_other_edge(app, edge, point, what);
    hover_at_world_point(app, point);
    assert!(
        is_the_edge(lit(app), edge),
        "{what}: the hand lights {:?}",
        lit(app)
    );
    driver::click_at_world_point(app, point);
    assert!(
        is_the_edge(chosen(app), edge),
        "{what}: a click chose {:?}",
        chosen(app)
    );
    assert!(
        driver::is_enabled(app, "Appliquer"),
        "{what}: « Appliquer » greyed with an edge chosen",
    );
}

/// Neither lights nor, by a click, chooses anything at `point`, which nothing
/// but `edge` stands within reach of.
fn neither_lit_nor_chosen(app: &mut driver::App, edge: [DVec3; 2], point: DVec3, what: &str) {
    clear_of_every_other_edge(app, edge, point, what);
    let before = chosen(app);
    hover_at_world_point(app, point);
    assert_eq!(lit(app), None, "{what}: the hand lights an edge");
    driver::click_at_world_point(app, point);
    assert_eq!(chosen(app), before, "{what}: a click chose an edge");
}

#[test]
fn with_the_section_on_an_edge_inside_the_part_seen_through_the_cut_is_lit_and_chosen() {
    let mut app = chamfering_through_the_section("an_edge_seen_through_the_cut_is_taken");

    lit_then_chosen(
        &mut app,
        THE_FLOOR_S_SIDE,
        ON_THE_FLOOR_S_SIDE,
        "the side of the pocket's floor, seen through the cut",
    );
}

#[test]
fn an_edge_the_plane_crosses_is_lit_from_its_part_on_the_opaque_side_and_not_from_its_part_on_the_hidden_side()
 {
    let mut app = chamfering_through_the_section("an_edge_the_plane_crosses");
    let its_opaque_part = [THE_FRONT_UPRIGHT[0], DVec3::new(SIDE, 0.0, THE_CUT)];
    let apart = apart_on_screen(&app, &its_opaque_part, ON_THE_UPRIGHT_ABOVE);
    assert!(
        apart > 2.0 * EDGE_REACH,
        "the hand above the cut stands {apart} points from the upright's part below it, \
         within reach of it",
    );

    neither_lit_nor_chosen(
        &mut app,
        THE_FRONT_UPRIGHT,
        ON_THE_UPRIGHT_ABOVE,
        "the hand on the upright above the cut, on the side the section hides",
    );
    lit_then_chosen(
        &mut app,
        THE_FRONT_UPRIGHT,
        ON_THE_UPRIGHT_BELOW,
        "the hand on the upright below the cut, on the side drawn opaque",
    );
}

#[test]
fn with_the_front_see_through_an_edge_behind_the_ghost_is_lit_and_chosen_and_an_edge_of_the_ghost_is_not()
 {
    let mut app = chamfering_through_the_section("an_edge_behind_the_ghost_is_taken");
    the_front_drawn(&mut app, "transparente");

    neither_lit_nor_chosen(
        &mut app,
        THE_TOP_FRONT,
        ON_THE_TOP_FRONT,
        "the top front, wholly on the see-through side",
    );
    assert_eq!(chosen(&app), None);
    lit_then_chosen(
        &mut app,
        THE_FLOOR_S_SIDE,
        ON_THE_FLOOR_S_SIDE,
        "the side of the pocket's floor, behind the ghost",
    );
}

/// What the hand lights at each of these points, in turn.
fn lit_at(app: &mut driver::App, points: &[DVec3]) -> Vec<Option<Vec<DVec3>>> {
    points
        .iter()
        .map(|point| {
            hover_at_world_point(app, *point);
            lit(app)
        })
        .collect()
}

const WHERE_THE_HAND_GOES: [DVec3; 4] = [
    ON_THE_TOP_FRONT,
    ON_THE_FLOOR_S_SIDE,
    ON_THE_UPRIGHT_ABOVE,
    ON_THE_UPRIGHT_BELOW,
];

#[test]
fn with_both_sides_opaque_an_edge_is_picked_as_with_the_section_off() {
    let mut off = chamfering("both_sides_opaque_the_section_off");
    let mut both = chamfering_through_the_section("both_sides_opaque");
    the_front_drawn(&mut both, "opaque");

    let without = lit_at(&mut off, &WHERE_THE_HAND_GOES);
    assert!(
        without.iter().any(Option::is_some) && without.iter().any(Option::is_none),
        "the section off, the hand lights an edge everywhere or nowhere: {without:?}",
    );
    assert_eq!(
        lit_at(&mut both, &WHERE_THE_HAND_GOES),
        without,
        "both sides opaque, the hand at {WHERE_THE_HAND_GOES:?} lights otherwise than the \
         section off",
    );
    for point in WHERE_THE_HAND_GOES {
        driver::click_at_world_point(&mut off, point);
        driver::click_at_world_point(&mut both, point);
        assert_eq!(
            chosen(&both),
            chosen(&off),
            "both sides opaque, a click at {point:?} chose otherwise than the section off",
        );
    }
}

#[test]
fn an_edge_chosen_where_the_plane_crosses_it_is_lit_along_its_whole_length() {
    let mut app = chamfering_through_the_section("an_edge_crossed_is_lit_whole");

    driver::click_at_world_point(&mut app, ON_THE_UPRIGHT_BELOW);
    assert!(
        is_the_edge(chosen(&app), THE_FRONT_UPRIGHT),
        "the upright, taken by its part below the cut, is held as {:?}, not whole from its foot \
         to the top",
        chosen(&app),
    );
    assert!(
        is_the_edge(lit(&app), THE_FRONT_UPRIGHT),
        "the upright under the hand is lit as {:?}, not whole from its foot to the top",
        lit(&app),
    );
}

#[test]
fn with_the_section_off_the_pocket_s_floor_is_not_taken_through_the_matter() {
    let mut app = chamfering("the_section_off_on_the_pocket");

    neither_lit_nor_chosen(
        &mut app,
        THE_FLOOR_S_SIDE,
        ON_THE_FLOOR_S_SIDE,
        "the side of the pocket's floor, behind the pocket's front wall",
    );
    assert_eq!(chosen(&app), None);
    lit_then_chosen(
        &mut app,
        THE_TOP_FRONT,
        ON_THE_TOP_FRONT,
        "the top front, the section off",
    );
    lit_then_chosen(
        &mut app,
        THE_FRONT_UPRIGHT,
        ON_THE_UPRIGHT_ABOVE,
        "the upright high up, the section off",
    );
}
