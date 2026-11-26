//! The start menu, driven whole in the application: what stands on it, and
//! whether it stands inside the window.
//!
//! Closes #1015.
//! - On the start menu, the « ⚙ Préférences » button lies inside the window,
//!   at its full height and at its minimum height —
//!   `the_preferences_button_of_the_start_menu_lies_inside_the_window_at_full_and_at_minimum_height`:
//!   full height read as the window the application opens on and as the one
//!   the bug was seen on; minimum height as the least the native window is
//!   held to on the start menu, which asks none of its own and keeps the one a
//!   part left when it was put away
//! - Clicking it on the start menu opens the preferences —
//!   `clicking_the_preferences_button_of_the_start_menu_opens_the_preferences`
//! - found in review, from the issue's "What must not break": the error bar
//!   at the bottom of the start menu stays on screen beside the button —
//!   `an_error_on_the_start_menu_and_the_preferences_both_lie_inside_the_window`

mod driver;

use egui_kittest::kittest::Queryable;

const THE_PREFERENCES: &str = "⚙ Préférences";

/// The window the bug was first seen on, at the full height of its screen.
const THE_WINDOW_IT_WAS_SEEN_ON: egui::Vec2 = egui::vec2(1512.0, 948.0);

/// Where the start menu's « ⚙ Préférences » stands, or why it cannot be
/// clicked there: off the window, or missing.
fn where_the_preferences_miss(app: &driver::App) -> Option<String> {
    let window = app.ctx.content_rect();
    let Some(button) = app.query_by_label(THE_PREFERENCES) else {
        return Some(format!(
            "no « {THE_PREFERENCES} » on the start menu, showing {:?}",
            driver::on_screen(app),
        ));
    };
    let rect = button.rect();
    (!window.contains_rect(rect)).then(|| format!("at {rect:?}, off the {window:?} window"))
}

fn on_a_window_of(app: &mut driver::App, size: egui::Vec2) {
    app.set_size(size);
    app.run();
}

#[test]
fn the_preferences_button_of_the_start_menu_lies_inside_the_window_at_full_and_at_minimum_height() {
    let mut app = driver::open_on(
        "the_preferences_button_lies_inside_the_window",
        driver::SCREEN,
    );
    let mut misses = Vec::new();
    for size in [driver::SCREEN, THE_WINDOW_IT_WAS_SEEN_ON] {
        on_a_window_of(&mut app, size);
        if let Some(miss) = where_the_preferences_miss(&app) {
            misses.push(format!("on a {size:?} window, {miss}"));
        }
    }

    on_a_window_of(&mut app, driver::SCREEN);
    driver::create_a_part(&mut app, "Équerre");
    driver::click(&mut app, "Accueil");
    let least = driver::the_minimum_size_asked(&app)
        .unwrap_or_else(|| {
            panic!(
                "back on the start menu from a part, the native window is held to no minimum \
                 height: the test has no minimum to set it to"
            )
        })
        .y;
    let at_the_minimum = egui::vec2(driver::SCREEN.x, least);
    on_a_window_of(&mut app, at_the_minimum);
    if let Some(miss) = where_the_preferences_miss(&app) {
        misses.push(format!(
            "on a {at_the_minimum:?} window, as short as it is held, {miss}"
        ));
    }

    assert!(
        misses.is_empty(),
        "the start menu's « {THE_PREFERENCES} » cannot be clicked:\n  {}",
        misses.join("\n  "),
    );
}

#[test]
fn clicking_the_preferences_button_of_the_start_menu_opens_the_preferences() {
    let mut app = driver::open_on("clicking_the_preferences_opens_them", driver::SCREEN);
    assert!(
        !driver::shows(&app, "Apparence"),
        "the preferences are open before anything was clicked: {:?}",
        driver::on_screen(&app),
    );

    driver::click_exactly(&mut app, THE_PREFERENCES);

    assert!(
        driver::shows(&app, "Apparence"),
        "a click on the start menu's « {THE_PREFERENCES} » opened no preferences, \
         showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn an_error_on_the_start_menu_and_the_preferences_both_lie_inside_the_window() {
    let test = "an_error_and_the_preferences_lie_inside";
    let mut app = driver::open_on(test, driver::SCREEN);
    driver::create_a_part(&mut app, "Disparue");
    driver::click(&mut app, "Accueil");
    let gone = driver::the_part_s_file(test, "Disparue");
    std::fs::remove_file(&gone).expect("the part was put away where the test looks for it");

    driver::open_the_recent_part(&mut app, "Disparue");

    let error = format!("Aucun fichier à {}.", gone.display());
    let window = app.ctx.content_rect();
    let said = driver::where_it_reads(&app, &error);
    assert!(
        said.is_some_and(|rect| window.contains_rect(rect)),
        "opening a recent part whose file is gone says « {error} » at {said:?}, which is not \
         inside the {window:?} window; on screen: {:?}",
        driver::on_screen(&app),
    );
    assert_eq!(
        where_the_preferences_miss(&app),
        None,
        "with an error on the start menu, « {THE_PREFERENCES} » cannot be clicked",
    );
}
