//! The edge of the part under the hand and those the edge panel holds, lit
//! along the points the kernel draws them through.

use cao_render::{Vertex, srgb};
use cao_solid::Body;
use glam::Vec3;

use super::super::ViewportState;
use crate::ui::theme::tokens;

const WIDTH: f32 = 3.0;
const PULLED_PIXELS: f32 = 4.0;

pub(super) fn push_lit_edges(
    lines: &mut Vec<Vertex>,
    state: &ViewportState,
    body: &Body,
    height_px: f32,
) {
    let camera = &state.camera;
    let eye = camera.eye();
    let pixel_per_unit_of_sight = camera.world_units_per_pixel(height_px) / camera.distance();
    let pulled = |point: Vec3| {
        let sight = point - eye;
        point
            - sight.normalize_or_zero() * (PULLED_PIXELS * pixel_per_unit_of_sight * sight.length())
    };
    let [r, g, b, a] = tokens::ACCENT_AMBER.to_normalized_gamma_f32();
    let colour = srgb(r, g, b, a);
    let lit = state.edges_chosen.iter().chain(&state.edge_under_the_hand);
    for points in lit.filter_map(|name| state.edge_points(body, name)) {
        for pair in points.windows(2) {
            lines.push(Vertex::line(pulled(pair[0].as_vec3()), colour, WIDTH));
            lines.push(Vertex::line(pulled(pair[1].as_vec3()), colour, WIDTH));
        }
    }
}
