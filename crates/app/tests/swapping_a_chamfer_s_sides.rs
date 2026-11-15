//! « Inverser les côtés » in a small panel of its own over the view, driven
//! through the whole application: where it stands, and what its one button
//! does.
//!
//! Closes #739 — the whole transcription is in
//! `crates/app/src/screens/viewport/input/copying/tests/mirror_and_rectangular_steps.rs`;
//! this is the half only the whole application holds, the panel on screen.
//! - In the sketch, « Inverser les côtés » leaves the row under the ribbon
//!   for a small floating panel of the same look as the step panel, holding
//!   that button alone, at the top left of the view like the step panel. A
//!   presenter test checks that it appears only during a sketch chamfer or
//!   fillet, and that the button swaps the sides — the panel over the view,
//!   at its top left, its button swapping the sides of the chamfer held —
//!   `a_chamfer_held_puts_its_swap_in_a_panel_at_the_top_left_of_the_view_whose_button_swaps_its_sides`

mod driver;

use cao_part::history::{ChamferAsked, PointRef};
use cao_part::{Operation, PartDocument};
use cao_sketch::{Chamfer, Corner, SegmentId, WorkPlane};
use egui_kittest::kittest::Queryable;
use glam::DVec2;

/// Halfway along the chamfer `a_plate_with_a_chamfer_of_two_sides` lays, from
/// 20 along the plate's foot to 30 up its right side.
const ON_THE_CHAMFER: DVec2 = DVec2::new(90.0, 15.0);

/// A plate a hundred by sixty on XY, the corner of its foot and its right side
/// chamfered 20 along the first and 30 along the second.
fn a_plate_with_a_chamfer_of_two_sides() -> PartDocument {
    let mut part = PartDocument::new("Plaque", "2026-11-16T09:00:00Z".parse().expect("a date"));
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    part.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(100.0, 60.0)),
        construction: false,
    });
    part.apply(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0).into(), SegmentId(1).into())],
        mode: ChamferAsked::from(Chamfer::Sided {
            first: 20.0,
            second: 30.0,
        }),
        construction: false,
    });
    part
}

/// The plate opened from the start menu, its sketch open, and the chamfer
/// clicked with the selection tool.
fn holding_the_chamfer(test: &str) -> driver::App {
    let mut app = driver::open_on(test, egui::vec2(1920.0, 800.0));
    driver::create_a_part(&mut app, "Plaque");
    driver::the_part_put_away(&mut app, test, "Plaque");
    a_plate_with_a_chamfer_of_two_sides()
        .save(
            &cao_app::adapters::files::DiskFiles,
            &driver::the_part_s_file(test, "Plaque"),
            "2026-11-16T10:00:00Z".parse().expect("a date"),
        )
        .expect("the part is written");
    app.query_all_by_label("Plaque")
        .last()
        .expect("the start menu lists the part")
        .click();
    app.run();
    driver::double_click(&mut app, "Esquisse 1");
    driver::click(&mut app, "Sélection (S)");
    driver::click_at_sketch_point(&mut app, ON_THE_CHAMFER);
    app
}

#[test]
fn a_chamfer_held_puts_its_swap_in_a_panel_at_the_top_left_of_the_view_whose_button_swaps_its_sides()
 {
    let mut app = holding_the_chamfer("a_chamfer_held_puts_its_swap_in_a_panel");
    let view = driver::canvas(&app);

    let swap = app
        .query_all_by_label("Inverser les côtés")
        .find(|node| view.contains_rect(node.rect()))
        .unwrap_or_else(|| {
            panic!(
                "the chamfer held, no « Inverser les côtés » stands over the view: {:?}",
                driver::buttons_over_the_view(&app)
            )
        })
        .rect();
    assert!(
        swap.right() < view.center().x && swap.bottom() < view.center().y,
        "the panel stands at the top left of the view, as the step panel does: \
         the button at {swap:?}, the view {view:?}"
    );

    driver::click_the_button(&mut app, "Inverser les côtés");

    driver::open_the_history(&mut app);
    assert!(
        driver::shows(&app, "Côtés du chanfrein inversés"),
        "the button swapped the chamfer's sides, as one step: {:?}",
        driver::steps(&app)
    );
}
