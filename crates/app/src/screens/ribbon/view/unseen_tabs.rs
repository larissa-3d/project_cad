use cao_prefs::Item;
use egui::{Id, Pos2, RawInput, Rect, Ui, UiBuilder, ViewportId, vec2};

use super::super::state::{Context, Ribbon};
use super::groups;

/// How many times the tabs are laid out at most in one frame: a block goes to
/// the next line on the size it came out at the time before.
const SETTLED_WITHIN: usize = 4;

/// Lays out every tab of `items` where the open tab stands, out of sight, and
/// answers how tall the tallest stands.
///
/// Laid out in a context of their own: in the window's, the accessibility
/// tree would read the buttons of the tabs not open as on screen.
pub(super) fn tallest(
    ui: &Ui,
    items: &[Item],
    state: &Context<'_>,
    ribbon: &mut Ribbon,
    bar_width: f32,
) -> f32 {
    let unseen = unseen_context(ui.ctx());
    let mut input = RawInput {
        screen_rect: Some(Rect::from_min_size(
            Pos2::ZERO,
            vec2(ui.available_size_before_wrap().x, 10_000.0),
        )),
        ..RawInput::default()
    };
    input
        .viewports
        .entry(ViewportId::ROOT)
        .or_default()
        .native_pixels_per_point = Some(ui.ctx().pixels_per_point());
    let style = ui.style().clone();
    let heights_id = Id::new("tab_heights");
    let mut heights: Vec<f32> = unseen
        .data(|data| data.get_temp(heights_id))
        .unwrap_or_default();
    for _ in 0..SETTLED_WITHIN {
        let mut now = Vec::new();
        unseen
            .run_ui(input.clone(), |ui| {
                ui.set_style(style.clone());
                now = tab_heights(ui, items, state, ribbon, bar_width);
            })
            .drop_without_applying_deltas();
        let settled = now == heights;
        heights = now;
        if settled {
            break;
        }
    }
    unseen.data_mut(|data| data.insert_temp(heights_id, heights.clone()));
    heights.into_iter().fold(0.0, f32::max)
}

fn tab_heights(
    ui: &mut Ui,
    items: &[Item],
    state: &Context<'_>,
    ribbon: &mut Ribbon,
    bar_width: f32,
) -> Vec<f32> {
    let room = ui.max_rect();
    items
        .iter()
        .enumerate()
        .filter_map(|(rank, item)| {
            let Item::Group { items, .. } = item else {
                return None;
            };
            let mut tab = ui.new_child(UiBuilder::new().id_salt(rank).max_rect(room));
            groups::open_tab(
                &mut tab,
                items,
                state,
                &mut Vec::new(),
                ribbon,
                false,
                bar_width,
            );
            Some(tab.min_rect().height())
        })
        .collect()
}

fn unseen_context(ctx: &egui::Context) -> egui::Context {
    let id = Id::new("ribbon_unseen_tabs");
    if let Some(unseen) = ctx.data(|data| data.get_temp::<egui::Context>(id)) {
        return unseen;
    }
    let unseen = egui::Context::default();
    crate::ui::install(&unseen);
    ctx.data_mut(|data| data.insert_temp(id, unseen.clone()));
    unseen
}
