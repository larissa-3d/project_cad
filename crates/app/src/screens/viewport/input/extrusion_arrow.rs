//! The extrusion's arrow taken and pulled in the view.

use super::{PICK_PIXELS, ViewScale, ViewportState, to_ndc};
use crate::screens::SketchContext;
use crate::screens::viewport::extrusion_handle::ExtrusionHandle;
use crate::screens::viewport::navigation::drag_asked;

/// Whether the gesture this frame belongs to the extrusion's arrow: a drag of
/// the primary button, navigating nothing, that took hold of it, or a click
/// on its tip, which changes nothing.
pub(crate) fn point_the_extrusion_arrow(
    ui: &egui::Ui,
    state: &mut ViewportState,
    response: &egui::Response,
    (rect, scale): (egui::Rect, ViewScale),
    context: &mut SketchContext<'_>,
) -> bool {
    let camera = state.camera;
    let ray = |at| {
        let (origin, direction) = camera.ray(to_ndc(at, rect), rect.width() / rect.height());
        (origin.as_dvec3(), direction.as_dvec3())
    };
    let reach = scale.world_size_of(PICK_PIXELS);
    if response.clicked_by(egui::PointerButton::Primary)
        && let Some(at) = response.interact_pointer_pos()
    {
        let (origin, direction) = ray(at);
        return ExtrusionHandle::is_over(
            context.extrusion,
            context.document,
            (origin, direction),
            reach,
        );
    }
    let navigating = ui.input(|input| drag_asked(input, &state.config).is_some());
    let handle = &mut state.extrusion_handle;
    if response.drag_started_by(egui::PointerButton::Primary)
        && !navigating
        && let Some(at) = ui.input(|input| input.pointer.press_origin())
    {
        let (origin, direction) = ray(at);
        handle.grab(
            context.extrusion,
            context.document,
            origin,
            direction,
            reach,
        );
    }
    if !handle.is_held() {
        return false;
    }
    match response.interact_pointer_pos() {
        Some(at) if response.dragged_by(egui::PointerButton::Primary) => {
            let (origin, direction) = ray(at);
            handle.pull(context.extrusion, origin, direction);
        }
        _ => handle.let_go(),
    }
    true
}
