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
//!   — `an_edge_chosen_where_the_plane_crosses_it_is_lit_along_its_whole_length`
//! - Section off: picking is unchanged — on the pocketed block, by
//!   `with_the_section_off_the_pocket_s_floor_is_not_taken_through_the_matter`;
//!   on the plain block, by the picking tests of `the_edge_panel.rs`, which
//!   this issue leaves as they were

mod driver;

use cao_app::app::EDGE_REACH;
use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_sketch::WorkPlane;
use egui_kittest::kittest::Queryable;
use glam::{DVec2, DVec3};

const SIDE: f64 = 100.0;
const HEIGHT: f64 = 50.0;

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
            35.0,
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

/// The upright between the faces towards +X and -Y, which the cut crosses.
const THE_FRONT_UPRIGHT: [DVec3; 2] = [DVec3::new(SIDE, 0.0, 0.0), DVec3::new(SIDE, 0.0, HEIGHT)];

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

/// How near on screen any edge of the part but `this` passes to where `point`
/// is drawn.
fn nearest_other_edge(app: &driver::App, this: [DVec3; 2], point: DVec3) -> f32 {
    let at = driver::world_point_on_screen(app, point);
    a_block_with_a_pocket()
        .body()
        .drawn_edges()
        .into_iter()
        .filter(|edge| !is_the_edge(Some(edge.points.clone()), this))
        .flat_map(|edge| {
            edge.points
                .windows(2)
                .map(|pair| {
                    let start = driver::world_point_on_screen(app, pair[0]);
                    let along = driver::world_point_on_screen(app, pair[1]) - start;
                    let t = ((at - start).dot(along) / along.length_sq().max(f32::EPSILON))
                        .clamp(0.0, 1.0);
                    at.distance(start + along * t)
                })
                .collect::<Vec<_>>()
        })
        .fold(f32::INFINITY, f32::min)
}

#[test]
fn with_the_section_on_an_edge_wholly_in_front_of_the_plane_is_neither_lit_nor_chosen() {
    let mut app = chamfering_through_the_section("an_edge_wholly_in_front_is_not_taken");
    hover_at_world_point(&mut app, DVec3::new(SIDE, 0.0, THE_CUT / 2.0));
    assert!(
        is_the_edge(lit(&app), THE_FRONT_UPRIGHT),
        "the hand on the front upright below the cut, where it is drawn, lights {:?}",
        lit(&app),
    );

    let in_front = THE_TOP_FRONT[0].lerp(THE_TOP_FRONT[1], 0.75);
    let apart = nearest_other_edge(&app, THE_TOP_FRONT, in_front);
    assert!(
        apart > 2.0 * EDGE_REACH,
        "the hand on the top front stands {apart} points from another edge, within reach of it",
    );
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
