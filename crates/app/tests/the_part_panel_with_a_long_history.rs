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
//! - found in review: the tree keeps three rows in view above the longest
//!   history, the history's list scrolling rather than take them —
//!   `at_the_frames_height_the_tree_keeps_three_rows_in_view_above_a_long_history`
//! - found in review: wherever the reader's card, the history's header, two
//!   lines and the link fit in the panel — a window of 520 points and up —
//!   the history's list keeps two lines in view, the tree giving way first,
//!   and a click on one goes back to it —
//!   `on_a_short_window_the_history_keeps_two_lines_in_view_and_a_click_on_one_goes_back_to_it`;
//!   below that the list takes what is left and nothing of the history runs
//!   off the window's bottom —
//!   `on_a_window_too_short_for_two_lines_nothing_of_the_history_runs_off_its_bottom`
//! - found in review: on a window tall enough, six lines of history show
//!   whole, the list not scrolling by a point: no scroll bar runs beside
//!   them —
//!   `on_a_tall_window_six_lines_of_history_show_whole`
//! - found on the rendered frames: at rest, no row of the tree and no line of
//!   the history shows in part, at 713 and at 600 points high —
//!   `at_rest_no_row_of_the_tree_nor_line_of_the_history_shows_in_part`
//!
//! Closes #998.
//! - The native window's minimum inner height is the height the Part panel
//!   needs for its title, three tree rows, the reader's card, the history
//!   header, two history lines and the « Compacter » link, plus the ribbon
//!   above it — computed from the same constants the panel uses, not a number
//!   typed twice —
//!   `the_window_asks_for_the_height_the_part_panel_needs_and_no_more`: read
//!   as the minimum the application asks of its native window, at which, the
//!   ribbon on top, ten lines of history unfolded, the panel shows its title,
//!   three rows' worth of the tree, the card, the header, two whole lines and
//!   the link, nothing drawn over anything, while a row less loses some of
//!   it. That no number is typed twice is read at review
//! - found in review: the minimum follows the ribbon's tab during the session,
//!   taller under the sketch's tab whose groups wrap on two rows than under the
//!   extrusion's on one, by as much as the ribbon is — no test: #1011 replaced
//!   it with the tallest tab's minimum on every tab, and rewrote its test below
//! - found in review: a change of zoom asks the minimum again, as the window
//!   holds it in pixels of the zoom it was asked at —
//!   `a_change_of_zoom_asks_the_window_for_its_minimum_again`
//!
//! Closes #1010.
//! - With the Part panel closed, the minimum height the window asks for is
//!   that of what stays on screen — title bar and ribbon — with nothing for
//!   the Part panel —
//!   `with_the_part_panel_closed_the_window_asks_for_the_title_bar_and_the_ribbon_and_no_more`:
//!   read as a minimum shorter than the one asked with the panel open, at
//!   which every labelled widget standing above the view still stands whole
//!   in the window, while a row less pushes some of them off its bottom.
//!   The Variables panel, open, is what stays on screen too, read the way the
//!   history is: at the minimum asked, its title, its column headers, its two
//!   first rows and the row that adds one stand whole in the window —
//!   `with_the_part_panel_closed_and_the_variables_open_the_window_keeps_their_header_two_rows_and_the_add_row`;
//!   the minimum does not grow with how many variables there are —
//!   `with_the_part_panel_closed_the_minimum_asked_for_the_variables_does_not_grow_with_their_count`;
//!   and on a window held for two, thirty keep their header and the row that
//!   adds one, the last scrolling into view —
//!   `on_a_window_held_for_two_variables_thirty_keep_their_header_and_add_row_and_the_last_scrolls_into_view`
//! - found in review: the Variables panel's rows scrolling apart from its
//!   headers and the row that adds one, every header still begins where its
//!   fields do, and « Ajouter » where « Retirer » does, as in the one grid
//!   they were laid in before —
//!   `the_variables_panel_s_headers_fields_and_buttons_stand_in_columns`
//! - found in review: the panel keeps the look it had in one grid, no wider
//!   and its columns where they began —
//!   `the_variables_panel_keeps_the_width_and_columns_it_had_in_one_grid`
//! - With the Part panel reopened, the window asks again for the panel's
//!   minimum, as with the panel never closed —
//!   `reopened_the_part_panel_asks_the_window_for_its_minimum_again_as_if_never_closed`
//! - The least height recorded for `window.json` is the last one asked: quit
//!   with the panel closed, the small one —
//!   `with_the_part_panel_closed_the_last_minimum_asked_stays_the_small_one_until_quit`;
//!   the driver never writes `window.json`, so the rest of the way is held
//!   beside the keeper: handed the last height asked rather than the tallest,
//!   `the_keeper_is_handed_the_last_least_height_asked_not_the_tallest` in
//!   `adapters/window_place`, and writing down the last one handed,
//!   `the_last_least_height_asked_while_the_application_ran_is_written_with_the_place`
//!   in `cao_prefs`
//!
//! Closes #1011.
//! - At a given width, the minimum height the window asks for is the same on
//!   every ribbon tab: the one of the tab whose ribbon is tallest at that
//!   width —
//!   `on_every_tab_the_minimum_asked_is_the_one_of_the_tab_whose_ribbon_is_tallest`:
//!   read on the screen's window, where the sketch's groups wrap on two rows
//!   and the extrusion's hold on one, the toolbar's « Extrusion » moved first
//!   so the part opens on it and its minimum is asked before the sketch's
//!   ribbon was ever drawn; then on « Esquisse » and « Export ». The same with
//!   the Part panel closed —
//!   `with_the_part_panel_closed_the_minimum_asked_is_the_tallest_tab_s_on_every_tab`
//! - Switching tab on a window at that minimum asks for no new minimum, so the
//!   window does not move —
//!   `switching_tab_on_a_window_at_its_minimum_asks_for_no_new_minimum`:
//!   read as no minimum asked on any frame while the tabs are clicked through,
//!   each shown after a taller and a shorter one, on a window set as short as
//!   the minimum, the Part panel open, and on another part closed before any
//!   tab but « Extrusion » was shown. The window moving is the native
//!   window's doing, which the driver has none of
//! - `the_minimum_asked_follows_the_ribbon_s_tab_by_as_much_as_the_ribbon_grows`
//!   is rewritten to assert criterion 1, not deleted — rewritten and renamed
//!   `on_every_tab_the_minimum_asked_is_the_one_of_the_tab_whose_ribbon_is_tallest`,
//!   its old name saying what it no longer asserts; it keeps its guard that the
//!   two ribbons differ by a row
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

/// The shortest window whose Part panel holds the reader's card, the
/// history's header, two of its lines and its link, as measured on the branch.
const A_SHORT_WINDOW: egui::Vec2 = egui::vec2(1280.0, 520.0);

/// A window too short for the card, the header, two lines and the link.
const A_WINDOW_TOO_SHORT_FOR_TWO_LINES: egui::Vec2 = egui::vec2(1280.0, 450.0);

/// A window tall enough for six lines of history under the card, and the tree.
const A_TALL_WINDOW: egui::Vec2 = egui::vec2(1280.0, 800.0);

/// A row of the tree, 28 points high: C10, which `the_part_panel.rs` holds.
const ROW: f32 = 28.0;

/// How tall the tree's list stays above the longest history, at the frames'
/// height: three rows, short of half a row for the margins between them.
const THE_TREE_KEEPS_IN_VIEW: f32 = 3.0 * ROW - ROW / 2.0;

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

/// Whether a rect stands beside the view on its left, where the Part panel is.
fn left_of_the_view(app: &driver::App, rect: egui::Rect) -> bool {
    driver::stands_beside_the_view(app, rect) && rect.center().x < app.ctx.content_rect().center().x
}

/// The Part panel's own « Pièce », left of the view: the bar along the top of
/// the window reads « Pièce » too.
fn the_part_panel_s_title(app: &driver::App) -> Option<egui::Rect> {
    app.get_all_by(|_| true)
        .filter(|node| {
            let read = node.accesskit_node();
            read.label().or_else(|| read.value()).as_deref() == Some("Pièce")
        })
        .map(|node| node.rect())
        .find(|rect| left_of_the_view(app, *rect))
}

/// Whether a node stands in the Part panel: beside the view on its left, at or
/// under the panel's title.
fn in_the_part_panel(app: &driver::App, rect: egui::Rect) -> bool {
    let title = the_part_panel_s_title(app).expect("the Part panel reads « Pièce »");
    left_of_the_view(app, rect) && rect.top() >= title.top() - TOLERANCE
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
    let history: Vec<(String, egui::Rect)> = history.collect();
    let card = the_card(app);
    assert!(
        card.is_some() || history.is_empty(),
        "the history has lines and no reader's card was found filled around ◀: {:?}",
        driver::on_screen(app),
    );
    card.map(|card| ("the reader's card".to_string(), card))
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
            "Pièce" => left_of_the_view(app, rect).then_some((text, rect)),
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

fn spanning(one: egui::Rangef, other: egui::Rangef) -> egui::Rangef {
    egui::Rangef::new(one.min.min(other.min), one.max.max(other.max))
}

fn inside(rect: egui::Rect, span: egui::Rangef) -> bool {
    span.min <= rect.top() + TOLERANCE && rect.bottom() - TOLERANCE <= span.max
}

fn outside(rect: egui::Rect, span: egui::Rangef) -> bool {
    rect.bottom() <= span.min + TOLERANCE || span.max - TOLERANCE <= rect.top()
}

/// The buttons of the tree, whole, whether its list shows them or not.
fn the_tree_s_buttons(app: &driver::App) -> Vec<(String, egui::Rect)> {
    the_tree_s_widgets(app, None)
        .into_iter()
        .filter(|(widget, _)| !THE_TREE_S_HEADINGS.contains(&widget.as_str()))
        .collect()
}

/// The span the tree's list shows, its whole extent when it does not scroll.
fn the_tree_s_height(app: &driver::App) -> f32 {
    let bottom = the_bottom(app);
    match the_tree_s_window(app, &bottom) {
        Some(window) => window.span(),
        None => the_tree_s_buttons(app)
            .into_iter()
            .map(|(_, rect)| rect.y_range())
            .reduce(spanning)
            .map_or(0.0, |extent| extent.span()),
    }
}

/// The lines of the history unfolded, whole, whether its list shows them or
/// not.
fn the_history_s_lines(app: &driver::App) -> Vec<(String, egui::Rect)> {
    app.query_all_by_role(Role::Button)
        .filter_map(|node| Some((node.accesskit_node().label()?, node.rect())))
        .filter(|(label, rect)| {
            is_the_history_s(label)
                && label != "Historique"
                && !label.starts_with("Compacter")
                && in_the_part_panel(app, *rect)
        })
        .collect()
}

/// What the history's list shows, top to bottom: the span of the vertical
/// scroll bar that runs most along its lines. `None` when no bar runs along
/// them: ten lines never fit whole on the windows these tests open, so the
/// list then shows none of them.
fn the_history_s_bar(app: &driver::App, lines: &[(String, egui::Rect)]) -> Option<egui::Rangef> {
    let extent = lines
        .iter()
        .map(|(_, rect)| rect.y_range())
        .reduce(spanning)
        .expect("the history unfolded lists its lines");
    let along = |bar: &egui::Rect| extent.intersection(bar.y_range()).span().max(0.0);
    app.query_all_by_role(Role::ScrollBar)
        .map(|node| node.rect())
        .filter(|bar| bar.height() > bar.width() && in_the_part_panel(app, *bar))
        .max_by(|one, other| along(one).total_cmp(&along(other)))
        .filter(|bar| along(bar) > 0.0)
        .map(|bar| bar.y_range())
}

/// `the_history_s_bar`, where the list has to be showing some of its lines.
fn the_history_s_window(app: &driver::App, lines: &[(String, egui::Rect)]) -> egui::Rangef {
    the_history_s_bar(app, lines).unwrap_or_else(|| {
        panic!(
            "{} lines of history on a {:?} window and no scroll bar beside them: the list \
                 cannot be showing them all, lines at {lines:?}",
            lines.len(),
            app.ctx.content_rect().size(),
        )
    })
}

/// The lines of the history its list shows whole, inside the window.
fn history_lines_in_view(app: &driver::App) -> Vec<(String, egui::Rect)> {
    let lines = the_history_s_lines(app);
    let list = the_history_s_window(app, &lines);
    let window = app.ctx.content_rect();
    lines
        .into_iter()
        .filter(|(_, rect)| inside(*rect, list) && window.contains_rect(*rect))
        .collect()
}

/// A block with a hole and sketches after it, ten lines of history unfolded,
/// on a window of this size, nothing scrolled.
fn ten_lines_unfolded_on(test: &str, size: egui::Vec2) -> driver::App {
    let mut app = a_block_with_a_hole(test);
    while history_length(&mut app) < 10 {
        add_a_sketch(&mut app);
    }
    assert_eq!(
        history_length(&mut app),
        10,
        "the fixture wants ten lines of history"
    );
    unfold_or_fold_the_history(&mut app, true);
    app.set_size(size);
    app.run();
    app
}

#[test]
fn at_the_frames_height_the_tree_keeps_three_rows_in_view_above_a_long_history() {
    let app = ten_lines_unfolded_on("the_tree_keeps_three_rows_in_view", THE_FRAMES_WINDOW);

    let height = the_tree_s_height(&app);

    assert!(
        height >= THE_TREE_KEEPS_IN_VIEW,
        "on a {THE_FRAMES_WINDOW:?} window, ten lines of history unfolded, the tree's list \
         shows {height} points, short of {THE_TREE_KEEPS_IN_VIEW}: the bottom of the panel \
         took its room",
    );
}

#[test]
fn on_a_short_window_the_history_keeps_two_lines_in_view_and_a_click_on_one_goes_back_to_it() {
    let mut app = ten_lines_unfolded_on(
        "on_a_short_window_the_history_keeps_two_lines",
        A_SHORT_WINDOW,
    );
    assert!(
        !driver::is_enabled(&app, "Rétablir"),
        "the fixture wants nothing undone before the click",
    );

    let seen = history_lines_in_view(&app);
    assert!(
        seen.len() >= 2,
        "on a {A_SHORT_WINDOW:?} window, ten lines of history unfolded, the list shows {} \
         of them whole: {seen:?}",
        seen.len(),
    );
    let line = seen
        .iter()
        .map(|(label, _)| label.clone())
        .find(|label| !label.starts_with("10. "))
        .expect("a line other than the last one in view");
    app.get_by_label(&line).click();
    app.run();

    assert!(
        driver::is_enabled(&app, "Rétablir"),
        "a click on « {line} », seen in the list, did not take the part back to it: {:?}",
        driver::on_screen(&app),
    );
}

/// Each row of the tree and each line of the history that its list shows
/// only in part.
fn cut_short(app: &driver::App) -> Vec<String> {
    let bottom = the_bottom(app);
    let mut cut = Vec::new();
    if let Some(tree) = the_tree_s_window(app, &bottom) {
        for (row, rect) in the_tree_s_buttons(app) {
            if !inside(rect, tree) && !outside(rect, tree) {
                cut.push(format!(
                    "the tree's « {row} » at {rect:?}, its list showing {tree:?}"
                ));
            }
        }
    }
    let lines = the_history_s_lines(app);
    let list = the_history_s_window(app, &lines);
    for (line, rect) in lines {
        if !inside(rect, list) && !outside(rect, list) {
            cut.push(format!(
                "the history's « {line} » at {rect:?}, its list showing {list:?}"
            ));
        }
    }
    cut
}

#[test]
fn at_rest_no_row_of_the_tree_nor_line_of_the_history_shows_in_part() {
    let mut cut = Vec::new();
    for (test, size) in [
        (
            "at_rest_no_row_shows_in_part_at_the_frames_height",
            THE_FRAMES_WINDOW,
        ),
        (
            "at_rest_no_row_shows_in_part_at_600",
            egui::vec2(1280.0, 600.0),
        ),
    ] {
        let app = ten_lines_unfolded_on(test, size);
        cut.extend(
            cut_short(&app)
                .into_iter()
                .map(|which| format!("{size:?}: {which}")),
        );
    }

    assert!(
        cut.is_empty(),
        "ten lines of history unfolded, nothing scrolled, shown in part:\n  {}",
        cut.join("\n  "),
    );
}

#[test]
fn on_a_window_too_short_for_two_lines_nothing_of_the_history_runs_off_its_bottom() {
    let app = ten_lines_unfolded_on(
        "on_a_window_too_short_for_two_lines_nothing_runs_off",
        A_WINDOW_TOO_SHORT_FOR_TWO_LINES,
    );
    let window = app.ctx.content_rect();
    let lines = the_history_s_lines(&app);
    let whole = the_history_s_bar(&app, &lines).map_or(0, |list| {
        lines
            .iter()
            .filter(|(_, rect)| inside(*rect, list) && window.contains_rect(*rect))
            .count()
    });
    assert!(
        whole < 2,
        "the fixture wants a window too short for two lines, and {whole} show whole on \
         {A_WINDOW_TOO_SHORT_FOR_TWO_LINES:?}",
    );
    let drawn = the_history_s_bar(&app, &lines).map_or_else(Vec::new, |list| {
        lines
            .into_iter()
            .filter(|(_, rect)| !outside(*rect, list))
            .collect()
    });
    let link = driver::where_it_reads(&app, "Compacter l'historique…").unwrap_or_else(|| {
        panic!(
            "no « Compacter l'historique… » under the unfolded history: {:?}",
            driver::on_screen(&app),
        )
    });

    let off: Vec<String> = drawn
        .into_iter()
        .chain([("Compacter l'historique…".to_string(), link)])
        .filter(|(_, rect)| rect.bottom() > window.bottom() + TOLERANCE)
        .map(|(what, rect)| format!("« {what} » at {rect:?}"))
        .collect();
    assert!(
        off.is_empty(),
        "on a {A_WINDOW_TOO_SHORT_FOR_TWO_LINES:?} window, ten lines of history unfolded, \
         run off the window's bottom at {}:\n  {}",
        window.bottom(),
        off.join("\n  "),
    );
}

#[test]
fn on_a_tall_window_six_lines_of_history_show_whole() {
    let mut app = a_block_with_a_hole("on_a_tall_window_six_lines_show_whole");
    assert_eq!(
        history_length(&mut app),
        6,
        "the fixture wants six lines of history"
    );
    unfold_or_fold_the_history(&mut app, true);
    app.set_size(A_TALL_WINDOW);
    app.run();

    let lines = the_history_s_lines(&app);
    let scrolled = the_history_s_bar(&app, &lines);

    assert!(
        scrolled.is_none(),
        "on a {A_TALL_WINDOW:?} window, six lines of history unfolded, the list scrolls \
         beside them, showing {scrolled:?} of lines at {lines:?}",
    );
}

/// What the Part panel fails to hold of its title, three rows of the tree, the
/// reader's card, the history's header, two whole lines and the link, and what
/// of it is drawn over something else.
fn what_the_part_panel_misses(app: &driver::App) -> Vec<String> {
    let window = app.ctx.content_rect();
    let mut missed = Vec::new();
    match the_part_panel_s_title(app) {
        Some(title) if window.contains_rect(title) => {}
        title => missed.push(format!("the title « Pièce » at {title:?}")),
    }
    let tree = the_tree_s_height(app);
    if tree < THE_TREE_KEEPS_IN_VIEW {
        missed.push(format!(
            "the tree's list shows {tree} points, short of {THE_TREE_KEEPS_IN_VIEW}"
        ));
    }
    let bottom = the_bottom(app);
    for wanted in ["the reader's card", "Historique", "Compacter l'historique…"] {
        match bottom.iter().find(|(what, _)| what == wanted) {
            Some((_, rect)) if window.contains_rect(*rect) => {}
            found => missed.push(format!("« {wanted} » at {found:?}")),
        }
    }
    let lines = the_history_s_lines(app);
    let lines: Vec<(String, egui::Rect)> = match the_history_s_bar(app, &lines) {
        Some(list) => lines
            .into_iter()
            .filter(|(_, rect)| inside(*rect, list) && window.contains_rect(*rect))
            .collect(),
        None => Vec::new(),
    };
    if lines.len() < 2 {
        missed.push(format!(
            "{} lines of history shown whole: {lines:?}",
            lines.len()
        ));
    }
    missed.extend(drawn_over(app));
    missed
}

#[test]
fn the_window_asks_for_the_height_the_part_panel_needs_and_no_more() {
    let mut app = ten_lines_unfolded_on(
        "the_window_asks_for_the_height_the_part_panel_needs",
        A_TALL_WINDOW,
    );
    let minimum = driver::the_minimum_size_asked(&app).unwrap_or_else(|| {
        panic!(
            "the application, on its Part screen, ten lines of history unfolded, never asked \
             its native window for a minimum inner size"
        )
    });

    app.set_size(egui::vec2(A_TALL_WINDOW.x, minimum.y));
    app.run();
    let at_the_minimum = what_the_part_panel_misses(&app);
    app.set_size(egui::vec2(A_TALL_WINDOW.x, minimum.y - ROW));
    app.run();
    let a_row_less = what_the_part_panel_misses(&app);

    assert!(
        at_the_minimum.is_empty(),
        "on a window as short as the minimum asked, {minimum:?}, ten lines of history \
         unfolded, the Part panel misses:\n  {}",
        at_the_minimum.join("\n  "),
    );
    assert!(
        !a_row_less.is_empty(),
        "on a window a row shorter than the minimum asked, {minimum:?}, the Part panel still \
         holds its title, three rows of the tree, the card, the header, two lines and the \
         link: the window is held taller than it needs",
    );
}

/// The minimum last asked, and where the view begins under the ribbon: the
/// ribbon's groups stand beside the Part panel, over the view, so the panel's
/// own top does not move with them.
fn the_minimum_and_the_view_s_top(app: &driver::App) -> (f32, f32) {
    let minimum = driver::the_minimum_size_asked(app)
        .unwrap_or_else(|| panic!("no minimum asked on the Part screen"))
        .y;
    (minimum, driver::canvas(app).top())
}

/// The default toolbar with its « Esquisse » group moved after « Extrusion »,
/// as the settings' « Descendre » does: the part opens on « Extrusion », whose
/// ribbon is the shorter, and the sketch's is not drawn until asked for.
fn the_sketch_group_moved_second() -> cao_prefs::Settings {
    let mut settings = cao_prefs::Settings::default();
    settings.toolbar.items.swap(0, 1);
    settings
}

/// A part opened on the screen's window, the toolbar's « Extrusion » tab
/// first, so the first minimum is asked before the sketch's tab was ever shown.
fn a_part_opened_on_the_extrusion_s_tab(test: &str) -> driver::App {
    let mut app = driver::open_with(test, driver::SCREEN, the_sketch_group_moved_second());
    driver::create_a_part(&mut app, "Bloc");
    app
}

/// The minimum asked and where the view begins, on each tab in turn: the one
/// the part opened on first, then « Esquisse », then « Export ».
fn on_every_tab(app: &mut driver::App) -> [(&'static str, f32, f32); 3] {
    let (first, first_top) = the_minimum_and_the_view_s_top(app);
    driver::click_the_button(app, "Esquisse");
    let (sketch, sketch_top) = the_minimum_and_the_view_s_top(app);
    driver::click_the_button(app, "Export");
    let (export, export_top) = the_minimum_and_the_view_s_top(app);
    [
        ("Extrusion", first, first_top),
        ("Esquisse", sketch, sketch_top),
        ("Export", export, export_top),
    ]
}

/// That the sketch's ribbon is the tallest on this window, a row taller than
/// the extrusion's: the fixture's own premise.
fn the_sketch_s_ribbon_is_the_tallest(tabs: &[(&str, f32, f32)]) {
    let top = |name: &str| {
        tabs.iter()
            .find(|(tab, _, _)| *tab == name)
            .map(|(_, _, top)| *top)
            .unwrap_or_else(|| panic!("no tab « {name} » measured"))
    };
    assert!(
        top("Esquisse") - top("Extrusion") > ROW / 2.0
            && tabs
                .iter()
                .all(|(_, _, other)| *other <= top("Esquisse") + TOLERANCE),
        "the fixture wants the sketch's ribbon the tallest on a {:?} window, a row taller \
         than the extrusion's; the view begins at {tabs:?}",
        driver::SCREEN,
    );
}

/// Every tab whose minimum asked is not the sketch's, the tallest ribbon's.
fn not_the_tallest_tab_s(tabs: &[(&'static str, f32, f32)]) -> Vec<String> {
    let tallest = tabs
        .iter()
        .find(|(tab, _, _)| *tab == "Esquisse")
        .map(|(_, minimum, _)| *minimum)
        .unwrap_or_else(|| panic!("no minimum measured on « Esquisse »"));
    tabs.iter()
        .filter(|(_, minimum, _)| (minimum - tallest).abs() > TOLERANCE)
        .map(|(tab, minimum, top)| {
            format!("« {tab} »: {minimum}, the view beginning at {top}, against {tallest}")
        })
        .collect()
}

#[test]
fn on_every_tab_the_minimum_asked_is_the_one_of_the_tab_whose_ribbon_is_tallest() {
    let mut app =
        a_part_opened_on_the_extrusion_s_tab("on_every_tab_the_minimum_asked_is_the_tallest");

    let tabs = on_every_tab(&mut app);

    the_sketch_s_ribbon_is_the_tallest(&tabs);
    let apart = not_the_tallest_tab_s(&tabs);
    assert!(
        apart.is_empty(),
        "on a {:?} window, the part opened on « Extrusion », the minimum asked is not the one of \
         « Esquisse », whose ribbon is the tallest:\n  {}",
        driver::SCREEN,
        apart.join("\n  "),
    );
}

#[test]
fn with_the_part_panel_closed_the_minimum_asked_is_the_tallest_tab_s_on_every_tab() {
    let mut app = a_part_opened_on_the_extrusion_s_tab(
        "with_the_part_panel_closed_the_minimum_is_the_tallest",
    );
    driver::click_the_button(&mut app, "Pièce");
    assert!(
        the_part_panel_s_title(&app).is_none(),
        "the fixture wants the Part panel closed: {:?}",
        driver::on_screen(&app),
    );

    let tabs = on_every_tab(&mut app);

    the_sketch_s_ribbon_is_the_tallest(&tabs);
    let apart = not_the_tallest_tab_s(&tabs);
    assert!(
        apart.is_empty(),
        "on a {:?} window, the part opened on « Extrusion » and its panel closed, the minimum \
         asked is not the one of « Esquisse », whose ribbon is the tallest:\n  {}",
        driver::SCREEN,
        apart.join("\n  "),
    );
}

/// The tabs clicked in turn, from « Extrusion »: each one shown at least once
/// after another, and each tab left for one taller and for one shorter.
const THE_TABS_IN_TURN: [&str; 5] = ["Esquisse", "Export", "Extrusion", "Esquisse", "Extrusion"];

/// Sets the window as short as the minimum last asked, then clicks through
/// the tabs, and answers every minimum asked on the way, frame by frame, with
/// the tab clicked when it was.
fn asked_while_switching_tabs_at_the_minimum(app: &mut driver::App) -> (f32, Vec<String>) {
    let minimum = the_minimum_height_asked(app);
    app.set_size(egui::vec2(driver::SCREEN.x, minimum));
    app.run();
    let mut asked = Vec::new();
    for tab in THE_TABS_IN_TURN {
        let before = driver::how_many_minimums_asked(app);
        driver::click_the_button(app, tab);
        asked.extend(
            driver::the_minimums_asked(app)
                .into_iter()
                .skip(before)
                .map(|size| format!("{} on clicking « {tab} »", size.y)),
        );
    }
    (minimum, asked)
}

#[test]
fn switching_tab_on_a_window_at_its_minimum_asks_for_no_new_minimum() {
    let mut open_part =
        a_part_opened_on_the_extrusion_s_tab("switching_tab_on_a_window_at_its_minimum");

    let (open, asked_open) = asked_while_switching_tabs_at_the_minimum(&mut open_part);
    let mut closed_part = a_part_opened_on_the_extrusion_s_tab(
        "switching_tab_on_a_window_at_its_minimum_the_panel_closed",
    );
    driver::click_the_button(&mut closed_part, "Pièce");
    assert!(
        the_part_panel_s_title(&closed_part).is_none(),
        "the fixture wants the Part panel closed: {:?}",
        driver::on_screen(&closed_part),
    );
    let (closed, asked_closed) = asked_while_switching_tabs_at_the_minimum(&mut closed_part);

    assert!(
        asked_open.is_empty(),
        "on a window as short as the minimum asked, {open}, the Part panel open, switching \
         tabs asked the window for a new minimum:\n  {}",
        asked_open.join("\n  "),
    );
    assert!(
        asked_closed.is_empty(),
        "on a window as short as the minimum asked, {closed}, the Part panel closed, switching \
         tabs asked the window for a new minimum:\n  {}",
        asked_closed.join("\n  "),
    );
}

/// A window wide enough for the ribbon to lay every group on one line at 100 %
/// and at 125 % alike: zoomed in, the window holds fewer points, and a ribbon
/// that wrapped would change the minimum in points by itself.
const WIDE_ENOUGH_AT_EITHER_ZOOM: egui::Vec2 = egui::vec2(2000.0, 1000.0);

#[test]
fn a_change_of_zoom_asks_the_window_for_its_minimum_again() {
    let mut app = driver::open_on(
        "a_change_of_zoom_asks_the_minimum_again",
        WIDE_ENOUGH_AT_EITHER_ZOOM,
    );
    driver::create_a_part(&mut app, "Bloc");
    let before = driver::how_many_minimums_asked(&app);
    let at_100 = driver::the_minimum_size_asked(&app);

    app.ctx.set_zoom_factor(1.25);
    app.run();
    let after = driver::how_many_minimums_asked(&app);
    let at_125 = driver::the_minimum_size_asked(&app);

    assert!(
        before > 0,
        "the fixture wants a minimum asked on the Part screen"
    );
    assert!(
        at_100
            .zip(at_125)
            .is_none_or(|(one, other)| (one.y - other.y).abs() <= TOLERANCE),
        "the fixture wants the same minimum in points at either zoom, the ribbon on one line; \
         it went from {at_100:?} to {at_125:?}",
    );
    assert!(
        after > before,
        "zoomed from 100 % to 125 %, the application asked no minimum again: the window \
         still holds the {at_100:?} points asked at 100 %, in pixels of that zoom",
    );
}

/// Every labelled widget standing wholly above the view: the title bar's and
/// the ribbon's.
fn above_the_view(app: &driver::App) -> Vec<(String, egui::Rect)> {
    let top = driver::canvas(app).top();
    labelled(app)
        .into_iter()
        .filter(|(_, rect)| rect.height() > 0.0 && rect.bottom() <= top + TOLERANCE)
        .collect()
}

fn labelled(app: &driver::App) -> Vec<(String, egui::Rect)> {
    app.get_all_by(|_| true)
        .filter_map(|node| {
            let read = node.accesskit_node();
            read.label()
                .or_else(|| read.value().filter(|value| !value.is_empty()))
                .or_else(|| read.placeholder().map(str::to_owned))
                .map(|label| (label, node.rect()))
        })
        .collect()
}

/// What of `wanted` the window does not hold whole, on a window this high:
/// each label counted among the widgets no higher than the topmost wanted,
/// so that the ribbon's « Variables » does not stand for the panel's title.
fn what_the_window_misses(
    app: &mut driver::App,
    height: f32,
    wanted: &[(String, egui::Rect)],
) -> Vec<String> {
    app.set_size(egui::vec2(driver::SCREEN.x, height));
    app.run();
    let window = app.ctx.content_rect();
    let from = wanted
        .iter()
        .map(|(_, rect)| rect.top())
        .fold(f32::INFINITY, f32::min);
    let held: Vec<(String, egui::Rect)> = labelled(app)
        .into_iter()
        .filter(|(_, rect)| rect.top() >= from - TOLERANCE)
        .collect();
    let mut labels: Vec<&String> = wanted.iter().map(|(label, _)| label).collect();
    labels.dedup();
    let mut missed = Vec::new();
    for label in labels {
        let wanted_so_many = wanted.iter().filter(|(one, _)| one == label).count();
        let whole = held
            .iter()
            .filter(|(one, rect)| one == label && window.contains_rect(*rect))
            .count();
        if whole < wanted_so_many && !missed.iter().any(|(seen, _)| seen == label) {
            missed.push((
                label.clone(),
                format!("« {label} »: {whole} of {wanted_so_many} whole in {window:?}"),
            ));
        }
    }
    missed.into_iter().map(|(_, said)| said).collect()
}

#[test]
fn with_the_part_panel_closed_the_window_asks_for_the_title_bar_and_the_ribbon_and_no_more() {
    let mut app = driver::open_on(
        "with_the_part_panel_closed_the_window_asks_for_the_ribbon",
        driver::SCREEN,
    );
    driver::create_a_part(&mut app, "Bloc");
    let open = driver::the_minimum_size_asked(&app)
        .unwrap_or_else(|| panic!("no minimum asked on the Part screen, its panel open"))
        .y;

    driver::click_the_button(&mut app, "Pièce");
    assert!(
        the_part_panel_s_title(&app).is_none(),
        "the fixture wants the Part panel closed: {:?}",
        driver::on_screen(&app),
    );
    let closed = driver::the_minimum_size_asked(&app)
        .unwrap_or_else(|| panic!("no minimum asked on the Part screen"))
        .y;
    let what_stays = above_the_view(&app);
    assert!(
        what_stays.iter().any(|(label, _)| label == "Pièce"),
        "the fixture wants the ribbon's « Pièce » above the view: {what_stays:?}",
    );
    let at_the_minimum = what_the_window_misses(&mut app, closed, &what_stays);
    let a_row_less = what_the_window_misses(&mut app, closed - ROW, &what_stays);

    assert!(
        closed < open,
        "the Part panel closed, the window is still asked for {closed} points, as with the \
         panel open ({open}): it is held tall for a panel no longer on screen",
    );
    assert!(
        at_the_minimum.is_empty(),
        "on a window as short as the minimum asked with the Part panel closed, {closed}, the \
         title bar and the ribbon lose:\n  {}",
        at_the_minimum.join("\n  "),
    );
    assert!(
        !a_row_less.is_empty(),
        "on a window a row shorter than the minimum asked with the Part panel closed, {closed}, \
         the title bar and the ribbon still stand whole: the window is held taller than what \
         stays on screen",
    );
}

fn the_minimum_height_asked(app: &driver::App) -> f32 {
    driver::the_minimum_size_asked(app)
        .unwrap_or_else(|| panic!("no minimum asked on the Part screen"))
        .y
}

/// A part open on the screen's window, the minimum it asked with its Part
/// panel open, and the one asked once the panel is closed.
fn a_part_with_its_panel_closed(test: &str) -> (driver::App, f32, f32) {
    let mut app = driver::open_on(test, driver::SCREEN);
    driver::create_a_part(&mut app, "Bloc");
    let open = the_minimum_height_asked(&app);
    driver::click_the_button(&mut app, "Pièce");
    assert!(
        the_part_panel_s_title(&app).is_none(),
        "the fixture wants the Part panel closed: {:?}",
        driver::on_screen(&app),
    );
    let closed = the_minimum_height_asked(&app);
    assert!(
        closed < open - ROW,
        "the fixture wants a smaller minimum asked with the Part panel closed, {closed}, than \
         open, {open}",
    );
    (app, open, closed)
}

#[test]
fn reopened_the_part_panel_asks_the_window_for_its_minimum_again_as_if_never_closed() {
    let (mut app, never_closed, closed) =
        a_part_with_its_panel_closed("reopened_the_part_panel_asks_its_minimum_again");
    let between = (closed + never_closed) / 2.0;
    app.set_size(egui::vec2(driver::SCREEN.x, between));
    app.run();
    let before = driver::how_many_minimums_asked(&app);

    driver::click_the_button(&mut app, "Pièce");
    let after = driver::how_many_minimums_asked(&app);
    let reopened = the_minimum_height_asked(&app);

    assert!(
        the_part_panel_s_title(&app).is_some(),
        "the fixture wants the Part panel open again: {:?}",
        driver::on_screen(&app),
    );
    assert!(
        after > before,
        "the Part panel reopened on a window {between} points high, the application asked its \
         window for no minimum: it is still held to the {closed} asked with the panel closed",
    );
    assert!(
        (reopened - never_closed).abs() <= TOLERANCE,
        "the Part panel reopened, the window is asked for {reopened} points, where the panel \
         never closed asked for {never_closed}",
    );
}

#[test]
fn with_the_part_panel_closed_the_last_minimum_asked_stays_the_small_one_until_quit() {
    let (mut app, open, closed) =
        a_part_with_its_panel_closed("the_last_minimum_asked_stays_the_small_one");
    let mut asked = Vec::new();
    let mut gesture = |app: &mut driver::App, when: String, act: &dyn Fn(&mut driver::App)| {
        let from = driver::how_many_minimums_asked(app);
        act(app);
        asked.extend(
            driver::the_minimums_asked(app)[from..]
                .iter()
                .map(|minimum| (when.clone(), minimum.y)),
        );
    };

    for tab in ["Extrusion", "Esquisse"] {
        gesture(&mut app, format!("on the « {tab} » tab"), &|app| {
            driver::click_the_button(app, tab);
        });
    }
    for zoom in [1.25, 1.0] {
        gesture(&mut app, format!("at a zoom of {zoom}"), &|app| {
            app.ctx.set_zoom_factor(zoom);
            app.run();
        });
    }
    for height in [closed, driver::SCREEN.y] {
        gesture(&mut app, format!("on a window {height} high"), &|app| {
            app.set_size(egui::vec2(driver::SCREEN.x, height));
            app.run();
        });
    }
    let at_quit = the_minimum_height_asked(&app);

    let tall: Vec<String> = asked
        .iter()
        .filter(|(_, minimum)| *minimum >= open - TOLERANCE)
        .map(|(when, minimum)| format!("{when}: {minimum}"))
        .collect();
    assert!(
        tall.is_empty(),
        "the Part panel closed all along, the window was asked again for the panel's \
         {open}, each ask resizing the native window:\n  {}",
        tall.join("\n  "),
    );
    assert!(
        (at_quit - closed).abs() <= TOLERANCE,
        "the Part panel closed, back on the tab, the zoom and the window it was closed on, \
         the last minimum asked before quitting is {at_quit}, not the {closed} asked once it \
         closed: the next launch would be held to it",
    );
}

/// Every widget beside the view on its left, under the ribbon, the Part panel
/// closed: the Variables panel's.
fn the_variables_panel(app: &driver::App) -> Vec<(String, egui::Rect)> {
    let view = driver::canvas(app);
    labelled(app)
        .into_iter()
        .filter(|(_, rect)| {
            rect.height() > 0.0
                && rect.right() <= view.left() + TOLERANCE
                && rect.top() >= view.top() - TOLERANCE
        })
        .collect()
}

/// A window tall enough for thirty variables laid one under the other, so
/// that the row that adds one stays within reach while they are added.
const TALL_ENOUGH_FOR_THIRTY_VARIABLES: egui::Vec2 = egui::vec2(1280.0, 1600.0);

/// The Variables panel's title, its column headers, its two first rows and
/// the row that adds one.
const WHAT_THE_VARIABLES_KEEP: [&str; 9] = [
    "Variables",
    "Nom",
    "Formule",
    "Valeur",
    "v1",
    "v2",
    "nom",
    "formule",
    "Ajouter",
];

/// A part with its Part panel closed and its Variables panel open.
fn a_part_with_the_variables_open(test: &str) -> driver::App {
    let (mut app, _, _) = a_part_with_its_panel_closed(test);
    app.set_size(TALL_ENOUGH_FOR_THIRTY_VARIABLES);
    app.run();
    driver::click_the_button(&mut app, "Variables");
    app
}

/// Adds `v{rank}` at `10 × rank` for each rank of `ranks`, in the panel.
fn add_variables(app: &mut driver::App, ranks: std::ops::RangeInclusive<usize>) {
    for rank in ranks {
        driver::type_into(app, "nom", &format!("v{rank}"));
        driver::type_into(app, "formule", &format!("{}", 10 * rank));
        driver::click_exactly(app, "Ajouter");
    }
}

/// What of the Variables panel's title, headers, two first rows and add row
/// stands left of the view, as the fixture wants them all there.
fn what_the_variables_keep(app: &driver::App) -> Vec<(String, egui::Rect)> {
    let kept: Vec<(String, egui::Rect)> = the_variables_panel(app)
        .into_iter()
        .filter(|(label, _)| WHAT_THE_VARIABLES_KEEP.contains(&label.as_str()))
        .collect();
    let read: Vec<&str> = kept.iter().map(|(label, _)| label.as_str()).collect();
    assert!(
        WHAT_THE_VARIABLES_KEEP
            .iter()
            .all(|wanted| read.contains(wanted)),
        "the fixture wants the Variables panel's title, headers, two first rows and add row \
         left of the view: {read:?}",
    );
    kept
}

#[test]
fn with_the_part_panel_closed_and_the_variables_open_the_window_keeps_their_header_two_rows_and_the_add_row()
 {
    let mut app = a_part_with_the_variables_open("with_the_variables_open_the_window_keeps_them");
    add_variables(&mut app, 1..=6);
    let kept = what_the_variables_keep(&app);
    let minimum = the_minimum_height_asked(&app);

    let missed = what_the_window_misses(&mut app, minimum, &kept);

    assert!(
        missed.is_empty(),
        "the Part panel closed and the Variables panel open on six variables, a window as short \
         as the minimum asked, {minimum}, cuts off:\n  {}",
        missed.join("\n  "),
    );
}

#[test]
fn with_the_part_panel_closed_the_minimum_asked_for_the_variables_does_not_grow_with_their_count() {
    let mut app = a_part_with_the_variables_open("the_minimum_for_the_variables_does_not_grow");
    add_variables(&mut app, 1..=2);
    let for_two = the_minimum_height_asked(&app);

    add_variables(&mut app, 3..=30);
    let for_thirty = the_minimum_height_asked(&app);

    assert!(
        (for_thirty - for_two).abs() <= TOLERANCE,
        "the Part panel closed and the Variables panel open, the window is asked for {for_two} \
         points under two variables and {for_thirty} under thirty: past two, the rows are to \
         scroll rather than hold the window taller",
    );
}

#[test]
fn on_a_window_held_for_two_variables_thirty_keep_their_header_and_add_row_and_the_last_scrolls_into_view()
 {
    let mut app = a_part_with_the_variables_open("thirty_variables_on_a_window_held_for_two");
    add_variables(&mut app, 1..=2);
    let for_two = the_minimum_height_asked(&app);
    let kept = what_the_variables_keep(&app);
    add_variables(&mut app, 3..=30);

    let missed = what_the_window_misses(&mut app, for_two, &kept);
    let window = app.ctx.content_rect();
    let header = labelled(&app)
        .into_iter()
        .filter(|(label, rect)| label == "Nom" && rect.right() < window.center().x)
        .map(|(_, rect)| rect.bottom())
        .fold(f32::NEG_INFINITY, f32::max);
    let foot = labelled(&app)
        .into_iter()
        .filter(|(label, _)| label == "nom")
        .map(|(_, rect)| rect.top())
        .fold(f32::INFINITY, f32::min);
    let the_last = app.query_all_by_value("v30").next().unwrap_or_else(|| {
        panic!(
            "no « v30 » in the Variables panel: {:?}",
            driver::on_screen(&app)
        )
    });
    the_last.scroll_to_me();
    app.run();
    let scrolled_to: Vec<egui::Rect> = labelled(&app)
        .into_iter()
        .filter(|(label, _)| label == "v30")
        .map(|(_, rect)| rect)
        .collect();

    assert!(
        missed.is_empty(),
        "thirty variables, on a window as short as the minimum asked for two, {for_two}, the \
         Variables panel loses:\n  {}",
        missed.join("\n  "),
    );
    assert!(
        !scrolled_to.is_empty()
            && scrolled_to.iter().all(|rect| {
                window.contains_rect(*rect)
                    && rect.top() >= header - TOLERANCE
                    && rect.bottom() <= foot + TOLERANCE
            }),
        "thirty variables, on a window {for_two} high, « v30 » scrolled to stands at \
         {scrolled_to:?}, not whole between the headers ending at {header} and the row that \
         adds one starting at {foot}, in {window:?}",
    );
}

/// Every widget of the Variables panel, its role and what it reads.
fn the_variables_panel_s_widgets(app: &driver::App) -> Vec<(Role, String, egui::Rect)> {
    let view = driver::canvas(app);
    app.get_all_by(|_| true)
        .filter_map(|node| {
            let read = node.accesskit_node();
            let text = read
                .label()
                .or_else(|| read.value().filter(|value| !value.is_empty()))
                .or_else(|| read.placeholder().map(str::to_owned))?;
            Some((read.role(), text, node.rect()))
        })
        .filter(|(_, _, rect)| {
            rect.height() > 0.0
                && rect.right() <= view.left() + TOLERANCE
                && rect.top() >= view.top() - TOLERANCE
        })
        .collect()
}

#[test]
fn the_variables_panel_s_headers_fields_and_buttons_stand_in_columns() {
    let mut app = a_part_with_the_variables_open("the_variables_stand_in_columns");
    add_variables(&mut app, 1..=3);
    let widgets = the_variables_panel_s_widgets(&app);
    let lefts = |wanted: &dyn Fn(&Role, &str) -> bool| -> Vec<(String, f32)> {
        widgets
            .iter()
            .filter(|(role, text, _)| wanted(role, text))
            .map(|(_, text, rect)| (text.clone(), rect.left()))
            .collect()
    };
    let names: Vec<String> = (1..=3).map(|rank| format!("v{rank}")).collect();
    let formulas: Vec<String> = (1..=3).map(|rank| format!("{}", 10 * rank)).collect();
    let formula_right = widgets
        .iter()
        .filter(|(role, text, _)| *role == Role::TextInput && formulas.contains(text))
        .map(|(_, _, rect)| rect.right())
        .fold(f32::NEG_INFINITY, f32::max);
    let columns = [
        (
            "the names",
            lefts(&|role, text| {
                (*role != Role::TextInput && text == "Nom")
                    || (*role == Role::TextInput
                        && (names.iter().any(|name| name == text) || text == "nom"))
            }),
        ),
        (
            "the formulas",
            lefts(&|role, text| {
                (*role != Role::TextInput && text == "Formule")
                    || (*role == Role::TextInput
                        && (formulas.iter().any(|formula| formula == text) || text == "formule"))
            }),
        ),
        (
            "the buttons",
            lefts(&|role, text| *role == Role::Button && (text == "Retirer" || text == "Ajouter")),
        ),
    ];
    let values: Vec<(String, f32)> = widgets
        .iter()
        .filter(|(role, text, rect)| {
            (*role != Role::TextInput && text == "Valeur")
                || (*role != Role::TextInput
                    && formulas.contains(text)
                    && rect.left() >= formula_right - TOLERANCE)
        })
        .map(|(_, text, rect)| (text.clone(), rect.left()))
        .collect();
    let wanted = [
        (
            "the names",
            &columns[0].1,
            vec!["Nom", "v1", "v2", "v3", "nom"],
        ),
        (
            "the formulas",
            &columns[1].1,
            vec!["Formule", "10", "20", "30", "formule"],
        ),
        (
            "the buttons",
            &columns[2].1,
            vec!["Retirer", "Retirer", "Retirer", "Ajouter"],
        ),
        ("the values", &values, vec!["Valeur", "10", "20", "30"]),
    ];
    for (which, column, texts) in wanted {
        let missing: Vec<&str> = texts
            .iter()
            .enumerate()
            .filter(|(rank, text)| {
                let before = texts[..*rank].iter().filter(|one| one == text).count();
                column.iter().filter(|(one, _)| one == *text).count() <= before
            })
            .map(|(_, text)| *text)
            .collect();
        assert!(
            missing.is_empty(),
            "the fixture wants {texts:?} in {which}, and {missing:?} are not there: {column:?}",
        );
    }

    let off: Vec<String> = columns
        .iter()
        .chain([("the values", values)].iter())
        .filter_map(|(which, column)| {
            let first = column
                .iter()
                .map(|(_, left)| *left)
                .fold(f32::INFINITY, f32::min);
            let last = column
                .iter()
                .map(|(_, left)| *left)
                .fold(f32::NEG_INFINITY, f32::max);
            (last - first > TOLERANCE)
                .then(|| format!("{which} begin from {first} to {last}: {column:?}"))
        })
        .collect();
    assert!(
        off.is_empty(),
        "three variables in their panel, a header, its fields and the row that adds one do not \
         begin at one edge:\n  {}",
        off.join("\n  "),
    );
}

/// Where the Variables panel's columns began, from the window's left edge, the
/// Part panel closed and three short variables in it, as the one grid laid
/// them out at 2072f76c: the grip, the names, the formulas, the values and the
/// buttons.
const THE_VARIABLES_COLUMNS_IN_ONE_GRID: [(&str, f32); 5] = [
    ("the grips", 8.0),
    ("the names", 56.0),
    ("the formulas", 104.0),
    ("the values", 161.8),
    ("the buttons", 209.8),
];

/// Where « Ajouter », the widest of the buttons, ended in the one grid.
const THE_VARIABLES_RIGHT_EDGE_IN_ONE_GRID: f32 = 270.0;

/// Where the view began right of the Variables panel in the one grid.
const THE_VIEW_RIGHT_OF_THE_VARIABLES_IN_ONE_GRID: f32 = 279.0;

/// How far the panel may stand from where the one grid laid it: a rounding
/// of egui's layout, not a column.
const AS_IN_ONE_GRID: f32 = 2.0;

#[test]
fn the_variables_panel_keeps_the_width_and_columns_it_had_in_one_grid() {
    let mut app = a_part_with_the_variables_open("the_variables_keep_their_width");
    add_variables(&mut app, 1..=3);
    let widgets = the_variables_panel_s_widgets(&app);
    let left_of = |wanted: &dyn Fn(&Role, &str) -> bool| {
        widgets
            .iter()
            .filter(|(role, text, _)| wanted(role, text))
            .map(|(_, _, rect)| rect.left())
            .fold(f32::INFINITY, f32::min)
    };
    let formula_right = widgets
        .iter()
        .filter(|(role, text, _)| *role == Role::TextInput && text == "10")
        .map(|(_, _, rect)| rect.right())
        .fold(f32::NEG_INFINITY, f32::max);
    let begins = [
        left_of(&|_, text| text == "☰"),
        left_of(&|role, text| *role == Role::TextInput && text == "v1"),
        left_of(&|role, text| *role == Role::TextInput && text == "10"),
        widgets
            .iter()
            .filter(|(role, text, rect)| {
                *role == Role::Label && text == "10" && rect.left() >= formula_right - TOLERANCE
            })
            .map(|(_, _, rect)| rect.left())
            .fold(f32::INFINITY, f32::min),
        left_of(&|role, text| *role == Role::Button && text == "Retirer"),
    ];
    let right_edge = widgets
        .iter()
        .filter(|(role, text, _)| *role == Role::Button && (text == "Retirer" || text == "Ajouter"))
        .map(|(_, _, rect)| rect.right())
        .fold(f32::NEG_INFINITY, f32::max);
    let window = app.ctx.content_rect().left();
    let view = driver::canvas(&app).left();

    let mut moved: Vec<String> = THE_VARIABLES_COLUMNS_IN_ONE_GRID
        .iter()
        .zip(begins)
        .filter(|((_, was), now)| ((now - window) - was).abs() > AS_IN_ONE_GRID)
        .map(|((which, was), now)| {
            format!(
                "{which} begin at {}, where they began at {was}",
                now - window
            )
        })
        .collect();
    if right_edge - window > THE_VARIABLES_RIGHT_EDGE_IN_ONE_GRID + AS_IN_ONE_GRID {
        moved.push(format!(
            "the buttons end at {}, where « Ajouter » ended at {THE_VARIABLES_RIGHT_EDGE_IN_ONE_GRID}",
            right_edge - window,
        ));
    }
    if view - window > THE_VIEW_RIGHT_OF_THE_VARIABLES_IN_ONE_GRID + AS_IN_ONE_GRID {
        moved.push(format!(
            "the view begins at {}, where it began at {THE_VIEW_RIGHT_OF_THE_VARIABLES_IN_ONE_GRID}",
            view - window,
        ));
    }
    assert!(
        moved.is_empty(),
        "three short variables, the Variables panel no longer looks as its one grid laid it \
         out:\n  {}",
        moved.join("\n  "),
    );
}
