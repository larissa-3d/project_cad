mod state;
mod view;

pub use state::{ExtrusionState, Shape, apply_extrusion_with};
pub(crate) use view::paint_extrusion_panel;
