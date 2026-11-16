//! The extrusion and the revolution set in the step panel, driven through the
//! whole application: what the panel holds, what it agrees with, and every way
//! of raising or cutting matter it takes over from the row it replaces.
//!
//! Closes #740.
//! - Extruding a sketch opens a step panel titled « Extrusion droite »
//!   (« Révolution » for a revolution), context the sketch, with the sections
//!   « Aires », « Matière », « Profondeur », « Autour d'un axe », « Inverser le
//!   sens » and « Annuler » / « Appliquer ↵ » —
//!   `extruding_a_sketch_opens_the_step_panel_on_that_sketch_with_its_sections`,
//!   `around_an_axis_titles_the_panel_revolution`
//! - « Ajout » / « Enlèvement » is set in the ribbon or in the panel, and the
//!   two always agree — `a_cut_chosen_in_the_panel_turns_the_ribbon_to_a_cut`,
//!   `a_cut_chosen_in_the_ribbon_turns_the_panel_to_a_cut`
//! - The depth accepts a formula with variables, shown as chips, with « = 30
//!   mm » on the right and the completion it has today —
//!   `a_depth_written_with_a_variable_shows_it_as_a_chip_and_what_it_comes_to`;
//!   the completion is held where every field's is, in
//!   `crates/app/tests/completion.rs`, by
//!   `the_start_of_a_name_in_the_extrusion_panel_is_completed_too`
//! - « Autour d'un axe » replaces the depth with an angle and an axis (U, V or
//!   a clicked trait) —
//!   `around_an_axis_puts_an_angle_and_an_axis_in_place_of_the_depth`,
//!   `a_trait_clicked_around_an_axis_is_the_axis_turned_about`
//! - The view draws the extruded volume alone, not fused into the part,
//!   see-through, green for an addition and red for a cut, and redraws it at
//!   every keystroke — no test: what a frame draws is wgpu's, which the driver
//!   does not reach. The volume alone is held in
//!   `crates/part/tests/previewing_a_step_of_matter.rs`, and its colour by
//!   the mode and its following every keystroke in
//!   `crates/app/src/screens/extrusion/state/tests.rs`, each under its own block for
//!   #740; the see-through drawing itself is checked by eye, the renderer
//!   having no test but the `offscreen` example
//! - The extrusion row under the ribbon is gone —
//!   `extruding_a_sketch_lays_no_row_under_the_ribbon`
//! - Every way of extruding today, and editing an existing extrusion, work as
//!   before — from the panel, a straight addition, a cut and a reversed one —
//!   `an_addition_applied_from_the_panel_raises_the_area_by_its_depth`,
//!   `a_cut_applied_from_the_panel_takes_its_depth_out_of_the_part`,
//!   `a_reversed_extrusion_applied_from_the_panel_goes_the_other_way_along_the_plane`;
//!   a revolution about U, V or a clicked trait —
//!   `a_revolution_applied_from_the_panel_turns_about_u_v_or_a_clicked_trait`;
//!   an extrusion reopened from its alert's « Modifier » —
//!   `an_extrusion_reopened_from_its_alert_is_changed_from_the_panel`. The
//!   tests that drove an extrusion through the row, `driver::a_block` first,
//!   go through the panel instead and pass as they did

mod driver;

use egui_kittest::kittest::{NodeT, Queryable};
use glam::DVec3;

/// `driver::INSIDE_THE_RECTANGLE` in the world, the sketch lying on YZ: where
/// the area is clicked once the sketch is finished and the view no longer
/// reads its plane.
const INSIDE_THE_RECTANGLE: DVec3 = DVec3::new(0.0, 62.61, -15.15);

/// Everything the panel of the step says that the user reads in the order the
/// issue lists it: its title, its sections, and its two buttons.
const THE_STRAIGHT_PANEL: [&str; 8] = [
    "Extrusion droite",
    "Aires",
    "Matière",
    "Profondeur",
    "Autour d'un axe",
    "Inverser le sens",
    "Annuler",
    "Appliquer ↵",
];

/// What the row under the ribbon wrote while an area was chosen: the label
/// before its height, its count of the areas chosen, and its checkbox to push
/// the other way. None of them is a word the panel uses.
const THE_OLD_ROW: [&str; 3] = ["Hauteur :", "1 aire(s)", "Sens inverse"];

/// A part with a rectangle drawn on YZ, its sketch finished, which is where
/// the tool is offered on it.
fn a_rectangle_finished(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Bloc");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at_sketch_point(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at_sketch_point(&mut app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::finish_the_sketch(&mut app);
    app
}

/// `a_rectangle_finished`, « Ajout de matière » taken and the rectangle's
/// area clicked: an extrusion being set.
fn an_area_chosen_to_add_matter(test: &str) -> driver::App {
    let mut app = a_rectangle_finished(test);
    driver::click_in_the_ribbon(&mut app, "Ajout de matière");
    driver::click_at_world_point(&mut app, INSIDE_THE_RECTANGLE);
    app
}

/// Every text on screen, as a name or as a label's value, with whether it
/// stands over the view. A container `egui` lays out has no bounds to read.
fn read_on_screen(app: &driver::App) -> Vec<(String, bool)> {
    app.get_all_by(|_| true)
        .filter(|node| node.accesskit_node().bounding_box().is_some())
        .filter_map(|node| {
            let read = node.accesskit_node();
            let text = read.label().or_else(|| read.value())?;
            Some((text, driver::stands_inside_the_view(app, node.rect())))
        })
        .collect()
}

/// Every text standing over the view: what a floating panel reads, and never
/// the ribbon's « Extrusion droite », the title bar's « Annuler » or the
/// tree's rows, which stand beside it.
fn read_over_the_view(app: &driver::App) -> Vec<String> {
    read_on_screen(app)
        .into_iter()
        .filter_map(|(text, over)| over.then_some(text))
        .collect()
}

/// Every text standing outside the view: the ribbon, the row under it, the
/// tree and the title bar.
fn read_outside_the_view(app: &driver::App) -> Vec<String> {
    read_on_screen(app)
        .into_iter()
        .filter_map(|(text, over)| (!over).then_some(text))
        .collect()
}

#[test]
fn extruding_a_sketch_opens_the_step_panel_on_that_sketch_with_its_sections() {
    let app = an_area_chosen_to_add_matter("extruding_a_sketch_opens_the_step_panel");

    let read = read_over_the_view(&app);
    let missing: Vec<&str> = THE_STRAIGHT_PANEL
        .into_iter()
        .filter(|text| !read.iter().any(|over| over == text))
        .collect();
    assert!(
        missing.is_empty(),
        "the sketch extruded, no panel over the view reads {missing:?}; over the view: {read:?}, \
         on screen: {:?}, said: {:?}",
        driver::on_screen(&app),
        driver::said(&app),
    );
    assert!(
        read.iter().any(|over| over.contains("Esquisse 1")),
        "the panel does not say it extrudes « Esquisse 1 »; over the view: {read:?}",
    );
}

#[test]
fn extruding_a_sketch_lays_no_row_under_the_ribbon() {
    let app = an_area_chosen_to_add_matter("extruding_a_sketch_lays_no_row");

    let read = read_outside_the_view(&app);
    let left: Vec<&str> = THE_OLD_ROW
        .into_iter()
        .filter(|text| read.iter().any(|outside| outside == text))
        .collect();
    assert!(
        left.is_empty(),
        "an area chosen to extrude, the row under the ribbon still reads {left:?}; outside the \
         view: {read:?}",
    );
}
