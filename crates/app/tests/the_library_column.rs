//! The library's column on the home screen, driven whole: what it is called,
//! the button that makes a folder, the counts beside the folders, the filter
//! typed above its tree, a part opened by a double-click, and a part dragged
//! onto a folder (`docs/redesign/home-screen.md` A1).
//!
//! Closes #743 — the whole transcription is in
//! `crates/app/src/screens/explorer/state/tests.rs`; these bullets are what
//! the application driven whole shows of it.
//! - A column « Toutes les pièces » on the left of the home screen —
//!   `the_home_screen_column_is_called_all_the_parts`
//! - « + Dossier » — `the_column_s_plus_folder_button_makes_a_folder_in_it`;
//!   « ↺ », « Renommer » and « Supprimer » read as they did
//! - The tree: folders with their count of parts, drawn beside the folder's
//!   name, the matches alone under a filter —
//!   `a_folder_in_the_column_shows_how_many_parts_it_holds`
//! - Filter, by name: typed in « Filtrer… », only the matching parts remain —
//!   `typing_in_the_filter_leaves_only_the_matching_parts_in_the_column`
//! - Double-click opens a part — `a_part_double_clicked_in_the_column_opens`;
//!   the second window, for a part opened while another is, stays as it was
//!   and is set aside here
//! - Drag a part onto a folder to move it there —
//!   `a_part_dragged_onto_a_folder_moves_there_and_its_recent_entry_follows`
//! - A part moved by drag opens from its new place and keeps its recent entry
//!   pointing to it —
//!   `a_part_dragged_onto_a_folder_moves_there_and_its_recent_entry_follows`
//!   for the entry, `a_part_dragged_onto_a_folder_opens_from_its_new_place`
//!   for the opening, by its row under the folder and by its recent entry
//! - found on the way, not one of the issue's words: a part let go on the
//!   blank of the column under the tree goes to the root of the library, the
//!   one place no row stands for —
//!   `a_part_dropped_under_the_tree_goes_to_the_root_of_the_library`
//! - found in review, not one of the issue's words: a folder made while the
//!   filter holds text shows in the column, the filter cleared as the naming
//!   starts — `a_folder_made_under_a_filter_shows_in_the_column`

mod driver;

use std::path::PathBuf;

use cao_app::adapters::files::DiskFiles;
use egui_kittest::kittest::{NodeT, Queryable};

fn a_part_is_open(app: &driver::App) -> bool {
    app.state()
        .where_the_view_shows(glam::DVec3::ZERO)
        .is_some()
}

/// Where the recent list beside the column names this part.
fn the_recent_entry(app: &driver::App, name: &str) -> egui::Pos2 {
    driver::the_recent_part_named(app, name).center()
}

/// What the count beside a folder's row reads: the number standing on its
/// row, to the right of its name.
fn the_count_beside(app: &driver::App, folder: &str) -> Option<String> {
    let row = *driver::rows_of_the_file_list_reading(app, folder).first()?;
    app.query_all_by_role(egui::accesskit::Role::Label)
        .filter(|node| {
            let at = node.rect();
            at.left() >= row.right() - 1.0
                && at.top() <= row.center().y
                && row.center().y <= at.bottom()
        })
        .filter_map(|node| node.accesskit_node().value())
        .find(|said| said.parse::<usize>().is_ok())
}

/// Lays these part files in the library, then reads the library again with
/// « ↺ », as a user who had put them there by hand would.
fn parts_laid_by_hand(app: &mut driver::App, test: &str, parts: &[&str]) -> PathBuf {
    let library = cao_prefs::default_projects_dir(&driver::locations_of(test));
    for part in parts {
        let path = library.join(part);
        std::fs::create_dir_all(path.parent().expect("a folder")).expect("the folder is made");
        std::fs::write(&path, b"PK").expect("the part file is written");
    }
    driver::click(app, "↺");
    library
}

#[test]
fn a_folder_in_the_column_shows_how_many_parts_it_holds() {
    let test = "a_folder_in_the_column_shows_how_many_parts_it_holds";
    let mut app = driver::open(test);
    parts_laid_by_hand(
        &mut app,
        test,
        &[
            "Brides/bride.caopart",
            "Brides/plate.caopart",
            "Brides/old/bride 2.caopart",
        ],
    );

    assert_eq!(
        the_count_beside(&app, "Brides").as_deref(),
        Some("3"),
        "« Brides » holds three parts, one of them in a folder under it, showing {:?}",
        driver::on_screen(&app),
    );
    driver::type_into(&mut app, "Filtrer…", "bri");
    assert_eq!(
        the_count_beside(&app, "Brides").as_deref(),
        Some("2"),
        "under « bri », « Brides » counts the two parts it still shows",
    );
}

#[test]
fn a_part_double_clicked_in_the_column_opens() {
    let mut app = driver::open("a_part_double_clicked_in_the_column_opens");
    driver::create_a_part(&mut app, "Support");
    driver::click(&mut app, "Accueil");
    assert!(!a_part_is_open(&app), "the home screen draws a part");
    let row = driver::rows_of_the_file_list_reading(&app, "Support")[0];

    driver::double_click_at(&mut app, row.center());

    assert!(
        a_part_is_open(&app),
        "the part double-clicked in the column is not open, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn a_part_dragged_onto_a_folder_opens_from_its_new_place() {
    let test = "a_part_dragged_onto_a_folder_opens_from_its_new_place";
    let mut app = driver::open(test);
    let brides = cao_prefs::default_projects_dir(&driver::locations_of(test)).join("Brides");
    std::fs::create_dir_all(&brides).expect("a folder to drop the part on");
    driver::create_a_part(&mut app, "Support");
    driver::click(&mut app, "Accueil");
    let part = driver::rows_of_the_file_list_reading(&app, "Support")[0];
    let folder = driver::rows_of_the_file_list_reading(&app, "Brides")[0];
    driver::drag_and_drop(&mut app, part.center(), folder.center());
    let under_the_folder = driver::rows_of_the_file_list_reading(&app, "Support")[0];
    assert!(
        under_the_folder.top() > folder.top() && under_the_folder.left() > folder.left(),
        "the part is not drawn inside « Brides »",
    );

    driver::double_click_at(&mut app, under_the_folder.center());

    assert!(
        a_part_is_open(&app),
        "double-clicked under « Brides », the part moved there does not open, showing {:?}",
        driver::on_screen(&app),
    );
    driver::click(&mut app, "Accueil");
    let entry = the_recent_entry(&app, "Support");
    driver::click_at(&mut app, entry);
    assert!(
        a_part_is_open(&app),
        "the recent entry does not open the part moved into « Brides », showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn a_part_dropped_under_the_tree_goes_to_the_root_of_the_library() {
    let test = "a_part_dropped_under_the_tree_goes_to_the_root";
    let mut app = driver::open(test);
    let library = parts_laid_by_hand(&mut app, test, &["Brides/support.caopart"]);
    let part = driver::rows_of_the_file_list_reading(&app, "support")[0];
    let under_the_tree = egui::pos2(part.center().x, part.bottom() + 3.0 * part.height());

    driver::drag_and_drop(&mut app, part.center(), under_the_tree);

    assert!(
        library.join("support.caopart").exists()
            && !library.join("Brides/support.caopart").exists(),
        "the part let go under the tree is not at the root of the library alone",
    );
}

#[test]
fn the_home_screen_column_is_called_all_the_parts() {
    let app = driver::open("the_home_screen_column_is_called_all_the_parts");

    assert!(
        driver::reads(&app, "Toutes les pièces"),
        "the column is not titled « Toutes les pièces », showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn the_column_s_plus_folder_button_makes_a_folder_in_it() {
    let mut app = driver::open("the_column_s_plus_folder_button_makes_a_folder");

    driver::click(&mut app, "+ Dossier");
    driver::type_into(&mut app, "Nom", "Brides");
    driver::press(&mut app, egui::Key::Enter);

    assert!(
        !driver::rows_of_the_file_list_reading(&app, "Brides").is_empty(),
        "no folder « Brides » in the column, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn a_folder_made_under_a_filter_shows_in_the_column() {
    let test = "a_folder_made_under_a_filter_shows_in_the_column";
    let mut app = driver::open(test);
    parts_laid_by_hand(&mut app, test, &["bride.caopart", "support.caopart"]);
    driver::type_into(&mut app, "Filtrer…", "bri");

    driver::click(&mut app, "+ Dossier");
    driver::type_into(&mut app, "Nom", "Plates");
    driver::press(&mut app, egui::Key::Enter);

    assert!(
        !driver::rows_of_the_file_list_reading(&app, "Plates").is_empty(),
        "the folder made under « bri » is not in the column, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn typing_in_the_filter_leaves_only_the_matching_parts_in_the_column() {
    let mut app = driver::open("typing_in_the_filter_leaves_only_the_matching_parts");
    for name in ["Équerre", "Support"] {
        driver::create_a_part(&mut app, name);
        driver::click(&mut app, "Accueil");
    }
    assert!(
        !driver::rows_of_the_file_list_reading(&app, "Support").is_empty(),
        "both parts are in the column before anything is typed, showing {:?}",
        driver::on_screen(&app),
    );

    driver::type_into(&mut app, "Filtrer…", "équ");

    assert!(
        !driver::rows_of_the_file_list_reading(&app, "Équerre").is_empty(),
        "the part whose name holds « équ » is gone, showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        driver::rows_of_the_file_list_reading(&app, "Support").is_empty(),
        "a part whose name does not hold « équ » is still in the column",
    );
}

#[test]
fn a_part_dragged_onto_a_folder_moves_there_and_its_recent_entry_follows() {
    let test = "a_part_dragged_onto_a_folder_moves_there";
    let mut app = driver::open(test);
    let at = driver::locations_of(test);
    let brides = cao_prefs::default_projects_dir(&at).join("Brides");
    std::fs::create_dir_all(&brides).expect("a folder to drop the part on");
    driver::create_a_part(&mut app, "Support");
    driver::click(&mut app, "Accueil");
    let made = the_first_recent_part(&at);
    assert_eq!(
        made.parent(),
        brides.parent(),
        "the part is made at the root of the parts folder",
    );
    let part = driver::rows_of_the_file_list_reading(&app, "Support")[0];
    let folder = driver::rows_of_the_file_list_reading(&app, "Brides")[0];

    driver::drag_and_drop(&mut app, part.center(), folder.center());

    let moved = brides.join(made.file_name().expect("a part file"));
    assert!(
        moved.exists() && !made.exists(),
        "the part is not in « Brides » alone: {} there, {} where it was",
        moved.exists(),
        made.exists(),
    );
    assert_eq!(
        the_first_recent_part(&at),
        moved,
        "the recent list would open the part from where it no longer is",
    );
}

fn the_first_recent_part(at: &cao_prefs::Locations) -> PathBuf {
    cao_prefs::RecentList::load(&DiskFiles, at)
        .expect("the recent list reads")
        .entries()
        .first()
        .expect("the part made is a recent part")
        .path
        .clone()
}
