//! A chamfer or a fillet step reopened from its history line or its alert,
//! its edge panel filled with the step, the part drawn as it stood before it,
//! and the step changed in place or left as it was.
//!
//! Closes #988 — the gestures, the panel and the view are here; what the part
//! records and replays is in `crates/part/tests/changing_a_blend_step.rs`.
//! - « Modifier le congé » (resp. « Modifier le chanfrein ») in the menu of a
//!   blend step's history line opens the edge panel on that step's kind, its
//!   size as written (a formula stays one) and its edges, all chosen —
//!   `modify_in_a_blend_line_s_menu_opens_the_edge_panel_on_its_kind_its_size_as_written_and_its_edges`
//! - « Modifier » on the alert of a blend step that no longer rebuilds does the
//!   same; « Montrer » still offers nothing for it —
//!   `modify_on_the_alert_of_a_blend_step_not_rebuilt_opens_the_panel_on_it_and_show_offers_nothing`
//! - While a blend step is reopened, the body drawn is the part as it stood
//!   just before that step, and the step's edges are the ones lit —
//!   `while_a_blend_step_is_reopened_the_body_drawn_is_the_part_before_it_and_its_edges_are_lit`
//! - « Appliquer » on a reopened step with a new size, kind or set of edges
//!   records one `EditMatter` change; the body afterwards is the one the part
//!   gives with the step laid that way, and the history row reads the new
//!   values — the gesture and the row, by
//!   `apply_on_a_reopened_step_lays_no_new_step_and_its_history_line_reads_the_new_values`;
//!   the change recorded and the body, in the part's file
//! - « Annuler » on a reopened step records nothing and the body drawn is the
//!   whole part again —
//!   `cancel_on_a_reopened_step_records_nothing_and_draws_the_whole_part_again`
//! - A reopened step whose new choice the kernel declines records nothing,
//!   keeps the panel open with the reason, and keeps the edges chosen — the
//!   panel's half, by
//!   `a_reopened_step_whose_new_choice_the_kernel_declines_keeps_the_panel_open_with_the_reason_and_the_edges`;
//!   nothing recorded, in the part's file
//! - A reopened step whose edges are all taken out has « Appliquer » greyed —
//!   `a_reopened_step_with_every_edge_taken_out_greys_apply`
//! - Undoing or rewinding the history while a step is reopened closes the
//!   panel — `undoing_or_rewinding_while_a_step_is_reopened_closes_the_panel`
