//! The part's matter in a frame: what the sketch's cut or the section leaves
//! of the part shown, drawn opaque, and of the ghosts of the volumes shown
//! see-through, drawn see-through; the raw block of the volume marked, in
//! the tree's light; and the volume the extrusion being set previews, in the
//! colour of its mode.

use cao_part::{ExtrusionMode, PartDocument};
use cao_solid::Body;
use glam::{DVec3, Vec3};

use super::{tint, tint_at};
use crate::screens::SketchContext;
use crate::screens::viewport::ViewportState;
use crate::screens::viewport::matter::Drawn;

const SEE_THROUGH_OPACITY: f32 = 0.25;
const MARKED_OPACITY: f32 = 0.4;

pub(super) fn push_matter(
    solids: &mut Vec<cao_render::Vertex>,
    surfaces: &mut Vec<cao_render::Vertex>,
    state: &ViewportState,
    drawn: Option<&Drawn<'_>>,
) {
    let (theme, camera) = (&state.theme, &state.camera);
    let Some(drawn) = drawn else {
        return;
    };
    let solid = tint(theme.solid);
    push_body(
        solids,
        drawn.opaque.triangles_with_normals(),
        solid,
        camera.forward(),
    );
    for veiled in drawn.see_through.iter().chain(&drawn.ghosts) {
        let color = tint_at(theme.solid, SEE_THROUGH_OPACITY);
        push_facing(surfaces, veiled, color, state);
    }
}

/// The raw block of the volume marked, drawn over whatever stands in front of
/// it so that it is seen wherever it is; nothing while no volume is marked, or
/// while its step raises nothing.
pub(super) fn push_marked(
    surfaces: &mut Vec<cao_render::Vertex>,
    state: &ViewportState,
    document: &PartDocument,
) {
    let Some(block) = state.marked_block(document) else {
        return;
    };
    let color = tint_at(state.theme.highlight, MARKED_OPACITY);
    push_facing(surfaces, &block, color, state);
}

/// The volume the extrusion being set would raise or take away, alone, in
/// the see-through colour of an addition or of a cut as its mode says;
/// nothing while nothing is previewed, or the kernel declines it.
pub(super) fn push_previewed(
    surfaces: &mut Vec<cao_render::Vertex>,
    state: &ViewportState,
    context: &SketchContext<'_>,
) {
    let Some(preview) = context.extrusion.preview(context.document) else {
        return;
    };
    let Some(volume) = state.previewed_volume(context.document, &preview.operation) else {
        return;
    };
    let colour = match preview.mode {
        ExtrusionMode::Add => state.theme.extrusion_add,
        ExtrusionMode::Cut => state.theme.extrusion_cut,
    };
    push_facing(surfaces, &volume, tint(colour), state);
}

/// `body` see-through: only its faces turned to the eye.
fn push_facing(
    surfaces: &mut Vec<cao_render::Vertex>,
    body: &Body,
    color: [f32; 4],
    state: &ViewportState,
) {
    let camera = &state.camera;
    let facing = facing(body, camera.eye().as_dvec3());
    push_body(surfaces, facing, color, camera.forward());
}

/// The triangles of `body` turned towards the eye. The surface pass culls
/// nothing: a far side left in would be stacked behind the near one.
fn facing(body: &Body, eye: DVec3) -> Vec<([DVec3; 3], [DVec3; 3])> {
    body.triangles_with_normals()
        .into_iter()
        .filter(|([a, b, c], _)| (*b - *a).cross(*c - *a).dot(eye - *a) > 0.0)
        .collect()
}

fn push_body(
    out: &mut Vec<cao_render::Vertex>,
    triangles: Vec<([DVec3; 3], [DVec3; 3])>,
    color: [f32; 4],
    towards: Vec3,
) {
    let triangles: Vec<_> = triangles
        .iter()
        .map(|(corners, normals)| (corners.map(DVec3::as_vec3), normals.map(DVec3::as_vec3)))
        .collect();
    cao_render::push_smooth_solid(out, &triangles, color, towards);
}

#[cfg(test)]
mod tests;
