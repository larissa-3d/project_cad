//! The library's column on the home screen, driven whole: what it is called,
//! the button that makes a folder, the filter typed above its tree, and a part
//! dragged onto a folder (`docs/redesign/home-screen.md` A1).
//!
//! Closes #743 — the whole transcription is in
//! `crates/app/src/screens/explorer/state/tests.rs`; these bullets are what
//! the application driven whole shows of it.
//! - A column « Toutes les pièces » on the left of the home screen —
//!   `the_home_screen_column_is_called_all_the_parts`
//! - « + Dossier » — `the_column_s_plus_folder_button_makes_a_folder_in_it`;
//!   « ↺ », « Renommer » and « Supprimer » read as they did
//! - Filter, by name: typed in « Filtrer… », only the matching parts remain —
//!   `typing_in_the_filter_leaves_only_the_matching_parts_in_the_column`
//! - Drag a part onto a folder to move it there, and it keeps its recent entry
//!   pointing to it — `a_part_dragged_onto_a_folder_moves_there_and_its_recent_entry_follows`

mod driver;

use std::path::PathBuf;

use cao_app::adapters::files::DiskFiles;

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
