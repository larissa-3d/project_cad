//! The Part panel under a history long enough to fill it, on a window as tall
//! as the one the overlap was first seen on, 1280 × 713.
//!
//! Closes #992.
//! - With the history unfolded and six or more lines in it, « Tout rendre
//!   opaque » and every volume row are on screen and answer a click —
//!   `under_a_long_history_make_everything_opaque_and_every_volume_row_answer_a_click`
//! - No widget of the Part panel is drawn over another, whatever the history's
//!   length — `no_widget_of_the_tree_is_drawn_under_the_reader_or_the_history_however_long_it_grows`;
//!   read as the widgets of the tree — its headings, its rows and their
//!   buttons — against the reader's card and the history's header, lines and
//!   link, at every length the fixture passes through from none to ten, folded
//!   and unfolded
//!
//! "On screen" is read as the issue's "what does not fit scrolls": a widget the
//! tree's list has to scroll to is brought into view first, as a user would,
//! then it must stand wholly inside the window and the panel, inside what the
//! tree's list shows, and clear of the reader's card and of the history. A
//! click answers when the pointer, sent to the middle of what is seen, changes
//! what the row says.
//!
//! The accessibility tree hands a widget's whole rect, even where a scrolled
//! list hides part of it, so a widget of the tree is weighed only by what of it
//! the tree's list shows: the vertical scroll bar beside it spans that. The
//! card is read from the frame, as the rectangle filled in the card's colour
//! around ◀, since its edge is no widget.

mod driver;

use cao_app::theme::tokens::BACKGROUND_CARD;
use egui::accesskit::Role;
use egui_kittest::kittest::{NodeT, Queryable};
use glam::DVec2;

const THE_FRAMES_WINDOW: egui::Vec2 = egui::vec2(1280.0, 713.0);

/// How far two rects may run into each other and still be taken as side by
/// side: a hair of anti-aliasing, not a row.
const TOLERANCE: f32 = 0.5;

/// Where the hole of `a_block_with_a_hole` is drawn, on YZ inside the block's
/// face, as two corners and a place inside them — those of `the_step_reader.rs`.
const THE_HOLE: [DVec2; 3] = [
    DVec2::new(20.0, 20.0),
    DVec2::new(60.0, -20.0),
    DVec2::new(40.0, 0.0),
];

const THE_TREE_S_HEADINGS: [&str; 4] = ["Pièce", "ESQUISSES", "VOLUMES", "VARIABLES"];

/// « Bloc » with a hole cut through it: two volumes, « Extrusion 1 » and
/// « Extrusion 2 », and six lines of history.
fn a_block_with_a_hole(test: &str) -> driver::App {
    let mut app = driver::a_block(test);
    driver::bring_into_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::start_a_sketch(&mut app);
    let [corner, opposite, inside] = THE_HOLE;
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at_sketch_point(&mut app, corner);
    driver::click_at_sketch_point(&mut app, opposite);
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_the_button(&mut app, "2");
    driver::click(&mut app, "Enlèvement de matière");
    driver::click_at_sketch_point(&mut app, inside);
    driver::click(&mut app, "Appliquer");
    driver::press(&mut app, egui::Key::Escape);
    driver::finish_the_sketch(&mut app);
    driver::click_the_button(&mut app, "Esquisse");
    assert!(
        driver::in_the_tree(&app, "Extrusion 2"),
        "the fixture wants the hole « Extrusion 2 » in the tree: {:?}",
        driver::on_screen(&app),
    );
    app
}

/// A sketch with one trait drawn on it, finished: two more lines of history.
fn add_a_sketch(app: &mut driver::App) {
    driver::bring_into_the_ribbon(app, "Nouvelle esquisse");
    driver::start_a_sketch(app);
    driver::draw_a_segment(app);
    driver::finish_the_sketch(app);
}

fn history_length(app: &mut driver::App) -> usize {
    let folded = driver::steps(app).is_empty();
    if folded {
        driver::open_the_history(app);
    }
    let length = driver::steps(app).len();
    if folded {
        driver::press(app, egui::Key::H);
    }
    length
}

fn unfold_or_fold_the_history(app: &mut driver::App, unfolded: bool) {
    if driver::steps(app).is_empty() == unfolded {
        driver::press(app, egui::Key::H);
    }
    assert_eq!(
        !driver::steps(app).is_empty(),
        unfolded,
        "H did not leave the history unfolded = {unfolded}: {:?}",
        driver::on_screen(app),
    );
}

fn on_the_frames_window(app: &mut driver::App) {
    app.set_size(THE_FRAMES_WINDOW);
    app.run();
}

/// Whether a node stands in the Part panel: beside the view on its left, at or
/// under the panel's title.
fn in_the_part_panel(app: &driver::App, rect: egui::Rect) -> bool {
    let title = driver::where_it_reads(app, "Pièce").expect("the Part panel reads « Pièce »");
    driver::stands_beside_the_view(app, rect)
        && rect.center().x < app.ctx.content_rect().center().x
        && rect.top() >= title.top() - TOLERANCE
}

fn is_the_history_s(label: &str) -> bool {
    label == "Historique"
        || label.starts_with("Compacter")
        || label
            .split_once(". ")
            .is_some_and(|(rank, _)| rank.parse::<usize>().is_ok())
}

/// The reader's card as the frame painted it, around its ◀.
fn the_card(app: &driver::App) -> Option<egui::Rect> {
    let previous = app.query_by_label("Étape précédente")?.rect();
    let mut pending: Vec<(&egui::Shape, egui::Rect)> = app
        .output()
        .shapes
        .iter()
        .map(|clipped| (&clipped.shape, clipped.clip_rect))
        .collect();
    let mut found = None;
    while let Some((shape, clip)) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group.iter().map(|one| (one, clip))),
            egui::Shape::Rect(painted)
                if painted.fill == BACKGROUND_CARD && painted.rect.contains(previous.center()) =>
            {
                found = Some(painted.rect.intersect(clip));
            }
            _ => {}
        }
    }
    found
}

/// What the bottom of the Part panel holds, each with what it is: the reader's
/// card, then the history's header, lines and link.
fn the_bottom(app: &driver::App) -> Vec<(String, egui::Rect)> {
    let history = app
        .query_all_by_role(Role::Button)
        .filter_map(|node| Some((node.accesskit_node().label()?, node.rect())))
        .filter(|(label, rect)| is_the_history_s(label) && in_the_part_panel(app, *rect));
    the_card(app)
        .map(|card| ("the reader's card".to_string(), card))
        .into_iter()
        .chain(history)
        .collect()
}

/// What the tree's list shows, top to bottom, when it scrolls: the span of the
/// vertical scroll bar beside it, the one standing above the bottom's first
/// widget. `None` when the list shows all it holds.
fn the_tree_s_window(app: &driver::App, bottom: &[(String, egui::Rect)]) -> Option<egui::Rangef> {
    let bottom_top = bottom
        .iter()
        .map(|(_, rect)| rect.top())
        .fold(f32::INFINITY, f32::min);
    app.query_all_by_role(Role::ScrollBar)
        .map(|node| node.rect())
        .filter(|bar| bar.height() > bar.width() && in_the_part_panel(app, *bar))
        .filter(|bar| bar.top() < bottom_top)
        .min_by(|one, other| one.top().total_cmp(&other.top()))
        .map(|bar| bar.y_range())
}

/// What of `rect` the tree's list shows.
fn shown(rect: egui::Rect, window: Option<egui::Rangef>) -> Option<egui::Rect> {
    let Some(window) = window else {
        return Some(rect);
    };
    let seen = egui::Rect::from_x_y_ranges(rect.x_range(), rect.y_range().intersection(window));
    (seen.height() > TOLERANCE).then_some(seen)
}

/// Every widget of the tree, as much of it as its list shows: the panel's
/// title and the tree's headings, then every button of the panel that is no
/// widget of the history's or of the card's.
fn the_tree_s_widgets(
    app: &driver::App,
    window: Option<egui::Rangef>,
) -> Vec<(String, egui::Rect)> {
    let headings = app
        .get_all_by(|_| true)
        .filter(|node| node.accesskit_node().role() == Role::Label)
        .filter_map(|node| Some((node.accesskit_node().value()?, node.rect())))
        .filter(|(text, _)| THE_TREE_S_HEADINGS.contains(&text.as_str()))
        .filter_map(|(text, rect)| match text.as_str() {
            "Pièce" => Some((text, rect)),
            _ => Some((text, shown(rect, window)?)),
        });
    let buttons = app
        .query_all_by_role(Role::Button)
        .filter_map(|node| Some((node.accesskit_node().label()?, node.rect())))
        .filter(|(label, rect)| {
            in_the_part_panel(app, *rect) && !is_the_history_s(label) && !label.starts_with("Étape")
        })
        .filter_map(|(label, rect)| Some((label, shown(rect, window)?)));
    headings.chain(buttons).collect()
}

fn run_into(one: egui::Rect, other: egui::Rect) -> bool {
    let both = one.intersect(other);
    both.width() > TOLERANCE && both.height() > TOLERANCE
}

/// Each widget of the tree drawn where the bottom of the panel is, with what
/// it runs into there.
fn drawn_over(app: &driver::App) -> Vec<String> {
    let bottom = the_bottom(app);
    let window = the_tree_s_window(app, &bottom);
    let mut found = Vec::new();
    for (widget, at) in the_tree_s_widgets(app, window) {
        for (under, there) in &bottom {
            if run_into(at, *there) {
                found.push(format!(
                    "« {widget} » at {at:?} runs into {under} at {there:?}"
                ));
            }
        }
    }
    found
}

/// Why `rect`, a widget of the tree, is not wholly on screen where a user can
/// click it; `None` when it is.
fn hidden(app: &driver::App, what: &str, rect: egui::Rect) -> Option<String> {
    let bottom = the_bottom(app);
    let window = the_tree_s_window(app, &bottom);
    if !app.ctx.content_rect().contains_rect(rect) {
        return Some(format!(
            "« {what} » at {rect:?} is off the window {:?}",
            app.ctx.content_rect()
        ));
    }
    if !in_the_part_panel(app, rect) {
        return Some(format!("« {what} » at {rect:?} is not in the Part panel"));
    }
    if let Some(window) = window
        && !(window.min <= rect.top() + TOLERANCE && rect.bottom() - TOLERANCE <= window.max)
    {
        return Some(format!(
            "« {what} » at {rect:?} is not wholly within what the tree's list shows, {window:?}"
        ));
    }
    bottom
        .iter()
        .find(|(_, there)| run_into(rect, *there))
        .map(|(under, there)| format!("« {what} » at {rect:?} stands under {under} at {there:?}"))
}

fn the_row(app: &driver::App, name: &str) -> egui::Rect {
    driver::the_row(app, name)
        .unwrap_or_else(|| panic!("no row « {name} »: {:?}", driver::on_screen(app)))
        .rect()
}

/// The volume's visibility button: the one on its row's line.
fn the_eye_of<'a>(app: &'a driver::App, volume: &str) -> egui_kittest::Node<'a> {
    let row = the_row(app, volume);
    let eye = app
        .query_all_by_role(Role::Button)
        .filter(|node| {
            node.accesskit_node()
                .label()
                .is_some_and(|label| label.starts_with("Rendre "))
        })
        .min_by(|one, other| {
            let off = |rect: egui::Rect| (rect.center().y - row.center().y).abs();
            off(one.rect()).total_cmp(&off(other.rect()))
        })
        .unwrap_or_else(|| panic!("no visibility button: {:?}", driver::on_screen(app)));
    assert!(
        (eye.rect().center().y - row.center().y).abs() < row.height() / 2.0,
        "no visibility button on the row « {volume} »: {:?}",
        driver::on_screen(app),
    );
    eye
}

/// What the row of `volume` writes of how it is drawn.
fn state_written_on(app: &driver::App, volume: &str) -> Vec<String> {
    let line = the_row(app, volume);
    app.get_all_by(|_| true)
        .filter(|node| node.accesskit_node().bounding_box().is_some())
        .filter(|node| {
            let at = node.rect().center().y;
            at > line.top() && at < line.bottom()
        })
        .flat_map(|node| {
            let read = node.accesskit_node();
            [read.label(), read.value()]
        })
        .flatten()
        .filter(|said| ["opaque", "transparente", "invisible"].contains(&said.as_str()))
        .collect()
}

/// Scrolls the tree's list to the row of `volume`, says why it is not on
/// screen if it is not, then clicks its visibility button where it is seen.
fn see_and_click_the_eye_of(app: &mut driver::App, volume: &str) -> Vec<String> {
    driver::the_row(app, volume)
        .expect("a row to scroll to")
        .scroll_to_me();
    app.run();
    let mut hidden_here: Vec<String> = [
        hidden(app, volume, the_row(app, volume)),
        hidden(
            app,
            &format!("the eye of {volume}"),
            the_eye_of(app, volume).rect(),
        ),
    ]
    .into_iter()
    .flatten()
    .collect();
    the_eye_of(app, volume).click();
    app.run();
    if state_written_on(app, volume) != ["transparente"] {
        hidden_here.push(format!(
            "a click on the eye of « {volume} » did not make it see-through: its row reads {:?}",
            state_written_on(app, volume),
        ));
    }
    hidden_here
}

fn see_and_click_make_everything_opaque(app: &mut driver::App) -> Vec<String> {
    app.get_by_label("Tout rendre opaque").scroll_to_me();
    app.run();
    let button = app.get_by_label("Tout rendre opaque");
    let mut hidden_here: Vec<String> = hidden(app, "Tout rendre opaque", button.rect())
        .into_iter()
        .collect();
    button.click();
    app.run();
    for volume in ["Extrusion 1", "Extrusion 2"] {
        if state_written_on(app, volume) != ["opaque"] {
            hidden_here.push(format!(
                "a click on « Tout rendre opaque » left « {volume} » reading {:?}",
                state_written_on(app, volume),
            ));
        }
    }
    hidden_here
}

#[test]
fn under_a_long_history_make_everything_opaque_and_every_volume_row_answer_a_click() {
    let mut app = a_block_with_a_hole("under_a_long_history_every_volume_row_answers_a_click");
    let mut missed = Vec::new();
    for lines in [6, 10] {
        while history_length(&mut app) < lines {
            add_a_sketch(&mut app);
        }
        assert_eq!(
            history_length(&mut app),
            lines,
            "the fixture wants {lines} lines of history"
        );
        unfold_or_fold_the_history(&mut app, true);
        on_the_frames_window(&mut app);

        for volume in ["Extrusion 1", "Extrusion 2"] {
            missed.extend(
                see_and_click_the_eye_of(&mut app, volume)
                    .into_iter()
                    .map(|why| format!("{lines} lines: {why}")),
            );
        }
        missed.extend(
            see_and_click_make_everything_opaque(&mut app)
                .into_iter()
                .map(|why| format!("{lines} lines: {why}")),
        );

        unfold_or_fold_the_history(&mut app, false);
        driver::hold_the_canvas(&mut app);
    }

    assert!(
        missed.is_empty(),
        "on a {THE_FRAMES_WINDOW:?} window, history unfolded:\n  {}",
        missed.join("\n  "),
    );
}

#[test]
fn no_widget_of_the_tree_is_drawn_under_the_reader_or_the_history_however_long_it_grows() {
    let mut overlaps = Vec::new();
    let mut check = |app: &mut driver::App, lines: usize| {
        for unfolded in [false, true] {
            if lines > 0 {
                unfold_or_fold_the_history(app, unfolded);
            }
            on_the_frames_window(app);
            overlaps.extend(
                drawn_over(app)
                    .into_iter()
                    .map(|overlap| format!("{lines} lines, unfolded = {unfolded}: {overlap}")),
            );
            driver::hold_the_canvas(app);
        }
    };

    let mut empty = driver::open("no_widget_of_the_tree_is_drawn_under_an_empty_history");
    driver::create_a_part(&mut empty, "Bloc");
    check(&mut empty, 0);

    let mut block = driver::a_block("no_widget_of_the_tree_is_drawn_under_the_history");
    assert_eq!(
        history_length(&mut block),
        3,
        "the fixture wants a block of three lines"
    );
    check(&mut block, 3);

    let mut app = a_block_with_a_hole("no_widget_of_the_tree_is_drawn_under_a_long_history");
    for lines in [6, 8, 10] {
        while history_length(&mut app) < lines {
            add_a_sketch(&mut app);
        }
        assert_eq!(
            history_length(&mut app),
            lines,
            "the fixture wants {lines} lines of history"
        );
        check(&mut app, lines);
    }

    assert!(
        overlaps.is_empty(),
        "on a {THE_FRAMES_WINDOW:?} window, widgets of the tree drawn where the bottom of \
         the Part panel is:\n  {}",
        overlaps.join("\n  "),
    );
}
