pub mod alert_line;
pub mod alerts_button;
pub mod badge;
pub mod chip;
pub mod dashed;
pub mod dropping;
pub mod empty_state;
pub mod eye;
pub mod floating_bar;
pub mod floating_panel;
pub mod formula_field;
pub mod history_line;
pub mod icon;
pub mod line_button;
pub mod logo;
pub mod number_field;
pub mod panel_toggle;
pub mod pill;
pub mod primary_button;
pub mod question;
pub mod quiet_link;
pub mod quiet_note;
pub mod reader_card;
pub mod secondary_button;
pub mod segmented;
pub mod sheet;
pub mod shown_field;
pub mod slider;
pub mod step_box;
pub mod step_dot;
pub mod text_edit;
pub mod theme;
pub mod thumbnail;
pub mod tool_button;
pub mod tree_row;

/// What every primitive draws with: the theme, and the icons.
pub fn install(ctx: &egui::Context) {
    theme::install(ctx);
    icon::install(ctx);
}
