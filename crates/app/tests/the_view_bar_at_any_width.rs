//! The view bar in windows narrower than the one it was drawn for: its
//! buttons shed their names and keys first, its coordinates last, and the
//! whole stays inside the view.
//!
//! Closes #964.
//! - at 1280 × 800, the view bar is unchanged: every button shows its icon,
//!   its name and its key, and the coordinates are read —
//!   `at_the_default_window_the_bar_reads_every_name_key_and_the_coordinates`
//! - with the view too narrow for the full bar but wide enough for its icons
//!   and reading, every button shows its icon alone, and its tooltip carries
//!   its name and key ahead of its hint; the coordinates are still read —
//!   `a_view_just_short_of_the_full_bar_shows_icons_alone_and_still_the_coordinates`
//! - with the view too narrow even for that, the coordinates are gone and
//!   every button is still shown, icon alone —
//!   `a_view_too_narrow_for_the_icons_and_the_reading_drops_the_coordinates_and_keeps_every_button`
//! - at any view width, the bar's rectangle lies inside the view's: it never
//!   reaches under the left panel —
//!   `the_bar_stays_inside_the_view_however_narrow_the_window`
//! - widening the view again gives the bar back its names, keys and
//!   coordinates — `widening_the_window_again_gives_the_bar_back_its_words`
//! - at a view width on a threshold, the bar keeps one form from one frame to
//!   the next rather than switching every frame —
//!   `on_a_threshold_the_bar_keeps_one_form_from_frame_to_frame`

mod driver;

use cao_app::theme::tokens::SPACING_XXL;
use egui_kittest::kittest::NodeT;

/// The bar's buttons, by the name each is read by.
const THE_BAR_S_BUTTONS: [&str; 3] = ["Recadrer", "Vue en coupe", "Matière devant"];

/// The narrowest window the sweep goes down to, and its step: a window a user
/// can still drag to, where the view is still a few hundred points wide.
const NARROWEST: f32 = 560.0;
const STEP: f32 = 40.0;

/// The window widths a hand dragging the window's edge passes through, from
/// the default width down to the narrowest, then back up again.
fn narrowed_then_widened() -> Vec<f32> {
    let steps = ((driver::SCREEN.x - NARROWEST) / STEP) as usize;
    let down = (0..=steps).map(|step| driver::SCREEN.x - step as f32 * STEP);
    let up = (0..steps)
        .rev()
        .map(|step| driver::SCREEN.x - step as f32 * STEP);
    down.chain(up).collect()
}

/// Every shape the last frame painted, groups opened.
fn painted(app: &driver::App) -> Vec<egui::Shape> {
    let mut found = Vec::new();
    for clipped in &app.output().shapes {
        let mut pending = vec![clipped.shape.clone()];
        while let Some(shape) = pending.pop() {
            match shape {
                egui::Shape::Vec(group) => pending.extend(group),
                shape => found.push(shape),
            }
        }
    }
    found
}

/// The bar's buttons wherever they stand, under the left panel included.
/// Panics on a button the bar no longer holds.
fn the_bar_s_buttons(app: &driver::App, seen: &str) -> Vec<egui::Rect> {
    THE_BAR_S_BUTTONS
        .iter()
        .map(|name| {
            driver::the_bar_s_wherever_it_stands(app, name)
                .unwrap_or_else(|| {
                    panic!(
                        "{seen}, the view bar holds no button read as « {name} », showing {:?}",
                        driver::on_screen(app)
                    )
                })
                .rect()
        })
        .collect()
}

/// The view bar's frame: the smallest filled rectangle painted around all its
/// buttons, its shadow left aside.
fn the_bar_s_frame(app: &driver::App, seen: &str) -> egui::Rect {
    let holds = the_bar_s_buttons(app, seen)
        .into_iter()
        .fold(egui::Rect::NOTHING, |held, rect| held.union(rect));
    painted(app)
        .into_iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect)
                if rect.blur_width == 0.0
                    && rect.fill != egui::Color32::TRANSPARENT
                    && rect.rect.contains_rect(holds) =>
            {
                Some(rect.rect)
            }
            _ => None,
        })
        .min_by(|one, other| one.area().total_cmp(&other.area()))
        .unwrap_or_else(|| panic!("{seen}, no frame is painted around the view bar"))
}

fn assert_inside_the_view_however_narrow(app: &mut driver::App, mode: &str) {
    for width in narrowed_then_widened() {
        app.set_size(egui::vec2(width, driver::SCREEN.y));
        app.run();
        let seen = format!("{mode}, in a window {width} wide");
        let frame = the_bar_s_frame(app, &seen);
        assert!(
            driver::stands_inside_the_view(app, frame),
            "{seen}, the view bar stands at {frame:?}, {} wide, past the edge of a view {} \
             wide: under the left panel",
            frame.width(),
            driver::the_view_s_width(app),
        );
    }
}

#[test]
fn the_bar_stays_inside_the_view_however_narrow_the_window() {
    let mut app = driver::open_on("the_bar_stays_inside_the_view_free", driver::SCREEN);
    driver::create_a_part(&mut app, "Platine");
    assert_inside_the_view_however_narrow(&mut app, "in the free view");

    let mut app = driver::open_on("the_bar_stays_inside_the_view_sketch", driver::SCREEN);
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    assert_inside_the_view_however_narrow(&mut app, "in a sketch");
}

/// How many points short of the bar it shows and its margin to the view's
/// right edge the view is narrowed to: too narrow for that bar whatever margin
/// is kept on the left, and far wider than the form the bar sheds into next.
const SHORT_OF_THE_BAR: f32 = 4.0;

/// Every text the last frame painted, and where.
fn written(app: &driver::App) -> Vec<(String, egui::Rect)> {
    painted(app)
        .into_iter()
        .filter_map(|shape| match shape {
            egui::Shape::Text(text) => {
                Some((text.galley.text().to_owned(), text.visual_bounding_rect()))
            }
            _ => None,
        })
        .collect()
}

/// The pointer left over the ribbon, off the view.
fn hover_off_the_view(app: &mut driver::App) {
    let off_the_view = driver::the_ribbon_s(app, "Nouvelle esquisse")
        .expect("the ribbon offers « Nouvelle esquisse »")
        .rect()
        .center();
    app.hover_at(off_the_view);
    app.run();
}

/// What hovering the bar's button says that was not written with the pointer
/// off the view: its tooltip.
fn the_tooltip_of(app: &mut driver::App, name: &str, seen: &str) -> String {
    hover_off_the_view(app);
    let before: Vec<String> = written(app).into_iter().map(|(text, _)| text).collect();
    let at = driver::the_bar_s(app, name).rect().center();
    app.hover_at(at);
    app.run_steps(4);
    let said: Vec<String> = written(app)
        .into_iter()
        .map(|(text, _)| text)
        .filter(|text| !before.contains(text))
        .collect();
    hover_off_the_view(app);
    match said.as_slice() {
        [tooltip] => tooltip.clone(),
        _ => panic!("{seen}, hovering « {name} » writes {said:?}, not one tooltip"),
    }
}

/// Narrows the window until the view is `SHORT_OF_THE_BAR` short of the bar
/// it shows, `what` it shows, and its margin to the right edge, and says so.
fn narrowed_just_short_of(app: &mut driver::App, what: &str) -> String {
    let shown = the_bar_s_frame(app, &format!("showing {what}")).width();
    let view = shown + SPACING_XXL - SHORT_OF_THE_BAR;
    let window = app.ctx.content_rect().size();
    app.set_size(window - egui::vec2(driver::the_view_s_width(app) - view, 0.0));
    app.run();
    let seen = format!(
        "in a view {} wide, short of the {shown} {what} takes and its {SPACING_XXL} margin",
        driver::the_view_s_width(app),
    );
    assert!(
        (driver::the_view_s_width(app) - view).abs() <= 0.5,
        "{seen}, where the window was narrowed for a view {view} wide",
    );
    seen
}

/// What the bar writes, left to right: every text painted inside its frame.
fn the_bar_shows(app: &driver::App, seen: &str) -> Vec<String> {
    let frame = the_bar_s_frame(app, seen);
    let mut shown: Vec<(String, egui::Rect)> = written(app)
        .into_iter()
        .filter(|(_, at)| frame.contains(at.center()))
        .collect();
    shown.sort_by(|one, other| one.1.min.x.total_cmp(&other.1.min.x));
    shown.into_iter().map(|(text, _)| text).collect()
}

/// Whether an icon is painted inside this button: an image off a texture of
/// its own, which no text is.
fn wears_an_icon(app: &driver::App, button: egui::Rect) -> bool {
    painted(app).into_iter().any(|shape| match shape {
        egui::Shape::Rect(rect) => {
            rect.fill_texture_id() != egui::TextureId::default()
                && button.expand(1.0).contains_rect(rect.rect)
        }
        egui::Shape::Mesh(mesh) => {
            mesh.texture_id != egui::TextureId::default()
                && button.expand(1.0).contains_rect(mesh.calc_bounds())
        }
        _ => false,
    })
}

fn assert_every_button_wears_its_icon(app: &driver::App, seen: &str) {
    for name in THE_BAR_S_BUTTONS {
        let button = driver::the_bar_s(app, name).rect();
        assert!(
            wears_an_icon(app, button),
            "{seen}, the bar's « {name} » wears no icon"
        );
    }
}

/// Whether every one of these words is read, each after the one before it.
fn in_this_order(read: &[String], words: &[&str]) -> bool {
    let mut rest = read.iter();
    words
        .iter()
        .all(|word| rest.any(|text| text.starts_with(word)))
}

/// A sketch open on the default window, the pointer over a point of it so that
/// the bar reads its coordinates.
fn a_sketch_on_the_default_window(test: &str) -> driver::App {
    let mut app = driver::open_on(test, driver::SCREEN);
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::hover_at_sketch_point(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    app
}

fn assert_icon_alone(app: &driver::App, name: &str, seen: &str) {
    let button = driver::the_bar_s(app, name).rect();
    let inside: Vec<String> = written(app)
        .into_iter()
        .filter(|(_, at)| button.contains(at.center()))
        .map(|(text, _)| text)
        .collect();
    assert!(
        inside.is_empty(),
        "{seen}, the bar's « {name} » still writes {inside:?} beside its icon",
    );
}

#[test]
fn a_view_just_short_of_the_full_bar_shows_icons_alone_and_still_the_coordinates() {
    let mut app = driver::a_block("a_view_just_short_of_the_full_bar");
    let commands = ["Recadrer", "Vue en coupe"];
    let worded = commands.map(|name| {
        let label = driver::the_bar_s(&app, name)
            .accesskit_node()
            .label()
            .unwrap_or_default();
        let hint = the_tooltip_of(&mut app, name, "at the default window");
        (label, hint)
    });
    let reading = driver::the_bar_s_coordinates(&app);

    let seen = narrowed_just_short_of(&mut app, "the full bar");
    for name in THE_BAR_S_BUTTONS {
        assert_icon_alone(&app, name, &seen);
    }
    assert_eq!(
        driver::the_bar_s_coordinates(&app),
        reading,
        "{seen}, the bar no longer ends on the coordinates it read",
    );
    for (name, (label, hint)) in commands.into_iter().zip(worded) {
        let tooltip = the_tooltip_of(&mut app, name, &seen);
        assert!(
            tooltip
                .strip_prefix(label.as_str())
                .is_some_and(|after| after.contains(hint.as_str())),
            "{seen}, hovering « {name} » says {tooltip:?}, not « {label} », its name and key, \
             ahead of its hint {hint:?}",
        );
    }

    let mut app = driver::open_on("a_view_just_short_of_the_full_bar_sketch", driver::SCREEN);
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    let seen = narrowed_just_short_of(&mut app, "the full bar");
    assert_icon_alone(&app, "Matière devant", &seen);
    let tooltip = the_tooltip_of(&mut app, "Matière devant", &seen);
    assert!(
        tooltip.starts_with("Matière devant"),
        "{seen}, hovering « Matière devant » says {tooltip:?}, not its name",
    );
}

#[test]
fn at_the_default_window_the_bar_reads_every_name_key_and_the_coordinates() {
    let app = a_sketch_on_the_default_window("at_the_default_window_the_bar_reads_every_word");
    let seen = "at 1280 × 800";

    let shows = the_bar_shows(&app, seen);
    assert!(
        in_this_order(
            &shows,
            &[
                "Recadrer",
                "F",
                "Vue en coupe",
                "Matière devant : cachée",
                "X "
            ],
        ),
        "{seen}, the bar writes {shows:?}, not « Recadrer » with its key F, « Vue en coupe », \
         « Matière devant : cachée » and the coordinates, in that order",
    );
    let last = shows.last().cloned().unwrap_or_default();
    assert!(
        last.starts_with("X ") && last.contains(" Y ") && last.ends_with(" mm"),
        "{seen}, the bar ends on {last:?}, not on the coordinates of the point under the cursor",
    );
    assert_every_button_wears_its_icon(&app, seen);
}

#[test]
fn a_view_too_narrow_for_the_icons_and_the_reading_drops_the_coordinates_and_keeps_every_button() {
    let mut app = driver::open_on(
        "a_view_too_narrow_for_the_icons_and_the_reading",
        driver::SCREEN,
    );
    driver::create_a_part(&mut app, "Platine");
    hover_off_the_view(&mut app);
    narrowed_just_short_of(&mut app, "the full bar");
    let seen = narrowed_just_short_of(&mut app, "its icons and its reading");

    let shows = the_bar_shows(&app, &seen);
    assert!(
        shows.is_empty(),
        "{seen}, the bar still writes {shows:?}, where its icons alone fit",
    );
    assert_every_button_wears_its_icon(&app, &seen);
    let frame = the_bar_s_frame(&app, &seen);
    assert!(
        driver::stands_inside_the_view(&app, frame),
        "{seen}, the bar stands at {frame:?}, past the view's edge",
    );
}

#[test]
fn widening_the_window_again_gives_the_bar_back_its_words() {
    let mut app = a_sketch_on_the_default_window("widening_the_window_again_gives_back_its_words");
    let before = the_bar_shows(&app, "at 1280 × 800");

    hover_off_the_view(&mut app);
    narrowed_just_short_of(&mut app, "the full bar");
    let seen = narrowed_just_short_of(&mut app, "its icons and its reading");
    assert!(
        the_bar_shows(&app, &seen).is_empty(),
        "{seen}, the bar was to show its icons alone before the window widened again",
    );
    app.set_size(driver::SCREEN);
    app.run();
    driver::hover_at_sketch_point(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);

    let after = the_bar_shows(&app, "widened back to 1280 × 800");
    assert_eq!(
        after, before,
        "widened back to 1280 × 800, the bar writes {after:?} where it wrote {before:?}",
    );
}

/// How many frames in a row the bar is watched on a threshold.
const FRAMES_WATCHED: usize = 12;

/// Whether the bar wears its words at this window width, once settled.
fn worded_at(app: &mut driver::App, width: f32) -> bool {
    app.set_size(egui::vec2(width, driver::SCREEN.y));
    let _ = app.try_run();
    the_bar_shows(app, &format!("in a window {width} wide"))
        .iter()
        .any(|text| text == "Recadrer")
}

#[test]
fn on_a_threshold_the_bar_keeps_one_form_from_frame_to_frame() {
    let mut app = driver::open_on("on_a_threshold_the_bar_keeps_one_form", driver::SCREEN);
    driver::create_a_part(&mut app, "Platine");
    hover_off_the_view(&mut app);
    let full = the_bar_s_frame(&app, "at 1280 × 800").width();
    let beside_the_view = driver::SCREEN.x - driver::the_view_s_width(&app);

    let mut icons = beside_the_view + full + SPACING_XXL - SHORT_OF_THE_BAR;
    let mut worded = driver::SCREEN.x;
    while worded - icons > 0.1 {
        let middle = (icons + worded) / 2.0;
        if worded_at(&mut app, middle) {
            worded = middle;
        } else {
            icons = middle;
        }
    }

    let mut watched = Vec::new();
    for width in [icons, (icons + worded) / 2.0, worded] {
        app.set_size(egui::vec2(width, driver::SCREEN.y));
        let _ = app.try_run();
        let seen = format!("in a window {width} wide, on the bar's threshold");
        let forms: Vec<Vec<String>> = (0..FRAMES_WATCHED)
            .map(|_| {
                app.step();
                the_bar_shows(&app, &seen)
            })
            .collect();
        assert!(
            forms.windows(2).all(|pair| pair[0] == pair[1]),
            "{seen}, the bar changes form from frame to frame: {forms:?}",
        );
        assert!(
            app.try_run().is_ok(),
            "{seen}, the bar never stops asking for another frame",
        );
        watched.push(forms[0].iter().any(|text| text == "Recadrer"));
    }
    assert!(
        !watched[0] && watched[2],
        "the bar was to shed its words between a window {icons} and {worded} wide; \
         worded at the narrow side, the middle and the wide side: {watched:?}",
    );
}
