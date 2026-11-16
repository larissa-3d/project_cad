mod state;
mod view;

pub use state::{ExtrusionState, Shape, apply_extrusion_with};
pub(super) use view::extrusion_row;
