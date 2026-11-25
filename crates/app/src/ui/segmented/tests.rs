//! What app · ui/segmented.rs is held to: the segment clicked is handed back,
//! a greyed one never is, and the one chosen alone stands on the active fill.

use super::*;

const PART: &str = "Pièce";
const ASSEMBLY: &str = "Assemblage";

fn both(assembly_enabled: bool) -> [Segment<'static>; 2] {
    [
        Segment {
            label: PART,
            enabled: true,
            tag: None,
        },
        Segment {
            label: ASSEMBLY,
            enabled: assembly_enabled,
            tag: (!assembly_enabled).then_some("bientôt"),
        },
    ]
}

#[test]
fn a_segment_clicked_is_handed_back() {
    let ctx = themed();
    let segments = both(true);

    assert_eq!(clicked_on(&ctx, &segments, ASSEMBLY), Some(1));
}

#[test]
fn a_greyed_segment_clicked_hands_back_nothing() {
    let ctx = themed();
    let segments = both(false);

    assert_eq!(clicked_on(&ctx, &segments, ASSEMBLY), None);
}

#[test]
fn the_chosen_segment_alone_stands_on_the_active_fill() {
    for (chosen, named) in [(0, PART), (1, ASSEMBLY)] {
        let drawn = drawn(&themed(), &both(true), chosen, Vec::new());
        let active: Vec<egui::Rect> = drawn
            .fills
            .iter()
            .filter(|(fill, _)| *fill == BACKGROUND_ACTIVE_NEUTRAL)
            .map(|(_, rect)| *rect)
            .collect();
        let name = drawn.written(named).expect("the segment writes its name");

        assert!(
            active.len() == 1 && active[0].contains(name),
            "with « {named} » chosen, the active fill stands at {active:?}, its name at {name:?}",
        );
    }
}

/// The segments as one frame of a themed context paints them, with no window.
struct Drawn {
    clicked: Option<usize>,
    fills: Vec<(egui::Color32, egui::Rect)>,
    texts: Vec<(String, egui::Pos2)>,
}

impl Drawn {
    /// Where the selector writes exactly `said`.
    fn written(&self, said: &str) -> Option<egui::Pos2> {
        self.texts
            .iter()
            .find(|(text, _)| text == said)
            .map(|(_, at)| *at)
    }
}

fn themed() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::install(&ctx);
    ctx
}

/// Presses and lets go on the segment writing `named`, « Pièce » chosen, and
/// says what the selector handed back.
fn clicked_on(ctx: &egui::Context, segments: &[Segment<'_>], named: &str) -> Option<usize> {
    let at = drawn(ctx, segments, 0, Vec::new())
        .written(named)
        .unwrap_or_else(|| panic!("the selector writes no « {named} »"));
    let mut clicked = None;
    for pressed in [true, false] {
        let click = vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            },
        ];
        clicked = clicked.or(drawn(ctx, segments, 0, click).clicked);
    }
    clicked
}

/// Two frames of the selector, the second hearing `events`.
fn drawn(
    ctx: &egui::Context,
    segments: &[Segment<'_>],
    chosen: usize,
    events: Vec<egui::Event>,
) -> Drawn {
    let mut clicked = None;
    let mut shapes = Vec::new();

    for events in [Vec::new(), events] {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(400.0, 200.0),
            )),
            events,
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            clicked = clicked.or(segmented(ui, segments, chosen));
        });
        output.textures_delta.clear();
        shapes = every_shape(output.shapes);
    }

    let fills = shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect) => Some((rect.fill, rect.rect)),
            _ => None,
        })
        .collect();
    let texts = shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Text(text) => Some((
                text.galley.text().to_owned(),
                shape.visual_bounding_rect().center(),
            )),
            _ => None,
        })
        .collect();
    Drawn {
        clicked,
        fills,
        texts,
    }
}

fn every_shape(clipped: Vec<egui::epaint::ClippedShape>) -> Vec<egui::Shape> {
    let mut pending: Vec<egui::Shape> = clipped.into_iter().map(|clipped| clipped.shape).collect();
    let mut shapes = Vec::new();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            shape => shapes.push(shape),
        }
    }
    shapes
}
