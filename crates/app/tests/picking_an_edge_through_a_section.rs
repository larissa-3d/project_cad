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
//!
//! Found in review, guarding what must not break and what #925 decided rather
//! than new criteria. « Inverser » trades which side is offered, by
//! `inverted_the_edge_on_the_side_now_opaque_is_lit_and_the_one_on_the_side_now_hidden_is_not`.
//! A tilted section offers by the tilted plane, by
//! `tilted_the_section_offers_an_edge_by_the_side_of_the_tilted_plane_it_lies_on`.
//! With no side opaque, no edge is offered, by
//! `with_no_side_opaque_no_edge_is_offered`. A section laid on a slanted face
//! offers every edge of its rim, however the kernel rounded it, by
//! `a_section_laid_on_a_slanted_face_offers_every_edge_of_its_rim`.
//!
//! Closes #979 — the whole transcription is in
//! `several_edges_in_one_step.rs`; this is the section view's half.
//! - Under the section view, Cmd + click on an edge on the hidden side adds
//!   nothing —
//!   `under_the_section_view_cmd_click_on_an_edge_on_the_hidden_side_adds_nothing`

mod driver;

use cao_app::app::EDGE_REACH;
use cao_part::history::{BlendShape, ExtrusionMode, Operation, PointRef};
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
    opened_on(test, a_block_with_a_pocket())
}

/// That part, written over the block `driver::a_block` left in the projects
/// folder, and opened from the start menu.
fn opened_on(test: &str, part: cao_part::PartDocument) -> driver::App {
    let mut app = driver::a_block(test);
    driver::the_part_put_away(&mut app, test, "Bloc");
    part.save(
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
        "the part written did not open, showing {:?}",
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
    assert!(
        driver::a_button_reads(&app, "Plan XY"),
        "the section view is not on as it opens, « Plan XY » missing: {:?}",
        driver::on_screen(&app),
    );
    the_sides_read(&app, ("opaque", "cachée"), "the section view as it opens");
    assert!(
        !driver::is_enabled(&app, "Appliquer"),
        "« Appliquer » offered before any edge is chosen",
    );
    app
}

/// Clicks the section's row for the side in front until it reads `state`.
fn the_front_drawn(app: &mut driver::App, state: &str) {
    the_side_drawn(app, "Devant", state);
}

/// The state the section's panel draws a side in: the chip standing under
/// that side's name, « Derrière : » or « Devant : ».
fn drawn_as(app: &driver::App, side: &str) -> String {
    let name = format!("{side} :");
    let buttons = || app.query_all_by_role(egui::accesskit::Role::Button);
    let named = buttons()
        .find(|node| node.accesskit_node().label().as_deref() == Some(name.as_str()))
        .unwrap_or_else(|| {
            panic!(
                "no side of the panel is named « {name} »: {:?}",
                driver::on_screen(app)
            )
        })
        .rect();
    buttons()
        .filter_map(|node| {
            let read = node.accesskit_node().label()?;
            let at = node.rect();
            let under = at.top() >= named.bottom()
                && at.left() < named.right()
                && at.right() > named.left();
            (under && ["cachée", "transparente", "opaque"].contains(&read.as_str()))
                .then_some((read, at))
        })
        .min_by(|(_, one), (_, other)| one.top().total_cmp(&other.top()))
        .map(|(read, _)| read)
        .unwrap_or_else(|| {
            panic!(
                "no state stands under « {name} »: {:?}",
                driver::on_screen(app)
            )
        })
}

/// Asserts the panel draws the side behind and the side in front as these.
fn the_sides_read(app: &driver::App, (behind, in_front): (&str, &str), what: &str) {
    assert_eq!(
        (drawn_as(app, "Derrière"), drawn_as(app, "Devant")),
        (behind.to_string(), in_front.to_string()),
        "{what}: the sides are not drawn behind {behind}, in front {in_front}",
    );
}

/// Clicks the name of that side in the section's panel until the side reads
/// `state`.
fn the_side_drawn(app: &mut driver::App, side: &str, state: &str) {
    for _ in 0..3 {
        if drawn_as(app, side) == state {
            return;
        }
        driver::click_the_button(app, &format!("{side} :"));
    }
    panic!(
        "« {side} » never read « {state} »: {:?}",
        driver::on_screen(app)
    );
}

/// Where the section's panel stands, read off the widgets it lays out and
/// the margin around them: nothing while it is closed.
fn the_section_s_panel(app: &driver::App) -> Option<egui::Rect> {
    const ITS_OWN: [&str; 12] = [
        "Plan XY",
        "Plan XZ",
        "Plan YZ",
        "Face cliquée",
        "Derrière :",
        "Devant :",
        "cachée",
        "transparente",
        "opaque",
        "Inverser",
        "Enregistrer la coupe",
        "Fermer",
    ];
    app.query_all_by_role(egui::accesskit::Role::Button)
        .filter(|node| {
            node.accesskit_node()
                .label()
                .is_some_and(|read| ITS_OWN.contains(&read.as_str()))
        })
        .map(|node| node.rect())
        .reduce(|one, other| one.union(other))
        .map(|panel| panel.expand(24.0))
}

/// Where a point of the world is drawn, refused where the section's panel
/// stands over it: a hand there is on the panel, and lights no edge whatever
/// the picking does.
fn on_the_view(app: &driver::App, point: DVec3) -> egui::Pos2 {
    let at = driver::world_point_on_screen(app, point);
    if let Some(panel) = the_section_s_panel(app) {
        assert!(
            !panel.contains(at),
            "{point:?} is drawn at {at:?}, under the section's panel {panel:?}",
        );
    }
    at
}

fn hover_at_world_point(app: &mut driver::App, point: DVec3) {
    driver::hold_the_canvas(app);
    let at = on_the_view(app, point);
    app.hover_at(at);
    app.run();
}

fn click_at_world_point(app: &mut driver::App, point: DVec3) {
    driver::hold_the_canvas(app);
    let at = on_the_view(app, point);
    driver::click_at(app, at);
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
    nearest_other_edge_of(&a_block_with_a_pocket(), app, this, point)
}

fn nearest_other_edge_of(
    part: &cao_part::PartDocument,
    app: &driver::App,
    this: [DVec3; 2],
    point: DVec3,
) -> f32 {
    part.body()
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
    click_at_world_point(&mut app, in_front);
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
    click_at_world_point(app, point);
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
    click_at_world_point(app, point);
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
        click_at_world_point(&mut off, point);
        click_at_world_point(&mut both, point);
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

    click_at_world_point(&mut app, ON_THE_UPRIGHT_BELOW);
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

#[test]
fn inverted_the_edge_on_the_side_now_opaque_is_lit_and_the_one_on_the_side_now_hidden_is_not() {
    let mut app = chamfering_through_the_section("inverted_the_sides_trade_places");
    driver::click_the_button(&mut app, "Inverser");
    the_sides_read(&app, ("cachée", "opaque"), "« Inverser » clicked");

    neither_lit_nor_chosen(
        &mut app,
        THE_FRONT_UPRIGHT,
        ON_THE_UPRIGHT_BELOW,
        "inverted, the upright below the cut, now hidden",
    );
    neither_lit_nor_chosen(
        &mut app,
        THE_FLOOR_S_SIDE,
        ON_THE_FLOOR_S_SIDE,
        "inverted, the side of the pocket's floor, now hidden",
    );
    assert_eq!(chosen(&app), None);
    lit_then_chosen(
        &mut app,
        THE_FRONT_UPRIGHT,
        ON_THE_UPRIGHT_ABOVE,
        "inverted, the upright above the cut, now opaque",
    );
    lit_then_chosen(
        &mut app,
        THE_TOP_FRONT,
        ON_THE_TOP_FRONT,
        "inverted, the top front, now opaque",
    );
}

/// The upright between the faces towards +X and +Y, at the back.
const THE_BACK_UPRIGHT: [DVec3; 2] = [DVec3::new(SIDE, SIDE, 0.0), DVec3::new(SIDE, SIDE, HEIGHT)];

/// Turned this far about X, around the middle of the block, the cut rises
/// towards +Y: at the back it passes 50 · tan 20° ≈ 18 mm above the middle,
/// at the front as far below it.
const THE_TILT: &str = "20";

#[test]
fn tilted_the_section_offers_an_edge_by_the_side_of_the_tilted_plane_it_lies_on() {
    let mut app = chamfering_through_the_section("tilted_the_section_offers_by_its_plane");
    driver::type_beside(&mut app, "Inclinaison 1", THE_TILT);

    neither_lit_nor_chosen(
        &mut app,
        THE_FRONT_UPRIGHT,
        ON_THE_UPRIGHT_BELOW,
        "tilted, the front upright below the middle but above the cut there",
    );
    assert_eq!(chosen(&app), None);
    lit_then_chosen(
        &mut app,
        THE_BACK_UPRIGHT,
        DVec3::new(SIDE, SIDE, 35.0),
        "tilted, the back upright above the middle but below the cut there",
    );
}

#[test]
fn with_no_side_opaque_no_edge_is_offered() {
    for (test, behind, in_front) in [
        ("no_side_opaque_both_hidden", "cachée", "cachée"),
        (
            "no_side_opaque_both_see_through",
            "transparente",
            "transparente",
        ),
        (
            "no_side_opaque_hidden_and_see_through",
            "cachée",
            "transparente",
        ),
    ] {
        let mut app = chamfering_through_the_section(test);
        the_side_drawn(&mut app, "Derrière", behind);
        the_side_drawn(&mut app, "Devant", in_front);
        assert!(
            driver::a_button_reads(&app, "Inverser"),
            "{test}: the section view closed on the way: {:?}",
            driver::on_screen(&app),
        );

        for point in WHERE_THE_HAND_GOES {
            hover_at_world_point(&mut app, point);
            assert_eq!(
                lit(&app),
                None,
                "behind {behind}, in front {in_front}: the hand at {point:?} lights an edge",
            );
            click_at_world_point(&mut app, point);
            assert_eq!(
                chosen(&app),
                None,
                "behind {behind}, in front {in_front}: a click at {point:?} chose an edge",
            );
        }
    }
}

/// A size whose rim the kernel lays a hair, some 1e-14, outside the plane of
/// the face it bounds: at 10, 7, 9 or 11 every rim point comes out inside.
const THE_FRONT_CHAMFER: f64 = 13.0;

/// The pocketed block, the top edge of its front chamfered: a face slanted
/// at 45° between the front and the top, whose rim the kernel computes.
fn a_block_with_a_pocket_and_a_slanted_front() -> cao_part::PartDocument {
    let mut part = a_block_with_a_pocket();
    let top_front = part
        .body()
        .drawn_edges()
        .into_iter()
        .find(|edge| is_the_edge(Some(edge.points.clone()), THE_TOP_FRONT))
        .expect("the top front")
        .name;
    let declined = part.apply(Operation::Blend {
        edges: vec![top_front],
        shape: BlendShape::Chamfer,
        size: THE_FRONT_CHAMFER.into(),
    });
    assert_eq!(declined, None, "the fixture's chamfer is laid");
    part
}

/// The edges of the slanted face: every point on the plane
/// `z - y = HEIGHT - THE_FRONT_CHAMFER`, within the kernel's rounding.
fn the_slanted_face_s_rim(part: &cao_part::PartDocument) -> Vec<Vec<DVec3>> {
    part.body()
        .drawn_edges()
        .into_iter()
        .map(|edge| edge.points)
        .filter(|points| {
            points
                .iter()
                .all(|point| (point.z - point.y - (HEIGHT - THE_FRONT_CHAMFER)).abs() < 1e-6)
        })
        .collect()
}

#[test]
fn a_section_laid_on_a_slanted_face_offers_every_edge_of_its_rim() {
    let test = "a_section_on_a_slanted_face";
    let part = a_block_with_a_pocket_and_a_slanted_front();
    let rim = the_slanted_face_s_rim(&part);
    assert_eq!(
        rim.len(),
        4,
        "the slanted face's rim is not four edges: {rim:?}"
    );
    let mut app = opened_on(test, a_block_with_a_pocket_and_a_slanted_front());
    driver::click_in_the_bar(&mut app, "Vue en coupe");
    the_front_drawn(&mut app, "opaque");
    driver::click(&mut app, "Face cliquée");
    let on_the_face = DVec3::new(
        SIDE / 2.0,
        THE_FRONT_CHAMFER / 2.0,
        HEIGHT - THE_FRONT_CHAMFER / 2.0,
    );
    click_at_world_point(&mut app, on_the_face);
    the_front_drawn(&mut app, "cachée");
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, "Chanfrein");
    let on_a_face = app
        .query_all_by_label("Face cliquée")
        .any(|row| row.accesskit_node().toggled() == Some(egui::accesskit::Toggled::True));
    assert!(
        on_a_face,
        "the section is not laid on the face clicked: {:?}",
        driver::on_screen(&app),
    );
    the_sides_read(
        &app,
        ("opaque", "cachée"),
        "the section on the face, with its sides as it opens them",
    );

    let mut unlit = Vec::new();
    for points in &rim {
        let edge = [points[0], points[points.len() - 1]];
        let clear = [0.5, 0.25, 0.75]
            .map(|along| edge[0].lerp(edge[1], along))
            .into_iter()
            .find(|point| nearest_other_edge_of(&part, &app, edge, *point) > 2.0 * EDGE_REACH)
            .unwrap_or_else(|| {
                panic!("no point of the rim edge {edge:?} stands clear of every other edge")
            });
        hover_at_world_point(&mut app, clear);
        if !is_the_edge(lit(&app), edge) {
            unlit.push((edge, lit(&app)));
        }
    }
    assert!(
        unlit.is_empty(),
        "on the slanted face the section lies on, these rim edges are not lit under the hand, \
         each with what is: {unlit:?}",
    );
}

#[test]
fn under_the_section_view_cmd_click_on_an_edge_on_the_hidden_side_adds_nothing() {
    let mut app = chamfering_through_the_section("cmd_click_on_the_hidden_side_adds_nothing");
    let chosen = |app: &driver::App| app.state().the_edges_chosen();
    click_at_world_point(&mut app, ON_THE_UPRIGHT_BELOW);
    assert!(
        chosen(&app).len() == 1 && is_the_edge(chosen(&app).pop(), THE_FRONT_UPRIGHT),
        "the fixture chooses the front upright below the cut, choosing {:?}",
        chosen(&app),
    );

    clear_of_every_other_edge(
        &app,
        THE_TOP_FRONT,
        ON_THE_TOP_FRONT,
        "Cmd + click on the top front",
    );
    driver::hold_the_canvas(&mut app);
    let at = on_the_view(&app, ON_THE_TOP_FRONT);
    driver::click_with(&mut app, at, egui::Modifiers::COMMAND);

    assert!(
        chosen(&app).len() == 1 && is_the_edge(chosen(&app).pop(), THE_FRONT_UPRIGHT),
        "Cmd + click on the top front, wholly on the side the section hides, left {:?} chosen",
        chosen(&app),
    );
}
