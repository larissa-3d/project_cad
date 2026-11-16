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
//!   `a_depth_written_with_a_variable_shows_it_as_a_chip_and_what_it_comes_to`,
//!   `the_start_of_a_name_in_the_depth_is_completed`
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
//!   `crates/app/src/screens/extrusion/tests.rs`, each under its own block for
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
