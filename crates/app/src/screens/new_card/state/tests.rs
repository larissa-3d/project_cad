//! What app · screens/new_card/state.rs is held to: where the « Nouveau »
//! card saves, a pick in « Choisir… » against a click in the column, the part
//! it hands over to be made, and the folders « Choisir… » offers. The card
//! driven whole is `crates/app/tests/the_new_card.rs`; these pin the rules its
//! presenter keeps, with no window and no disk.

use std::path::{Path, PathBuf};

use cao_part::{Files, Folders, InMemoryFiles};

use super::*;

const ROOT: &str = "/CAO";

/// A library holding « Brides » with « Petites » inside it, then « Coques »,
/// and a part at the root and one in « Brides ».
fn a_library() -> InMemoryFiles {
    let files = InMemoryFiles::default();
    for part in ["/CAO/Plaque.caopart", "/CAO/Brides/Équerre.caopart"] {
        files.write(Path::new(part), b"PK").expect("writes");
    }
    for folder in ["/CAO/Brides/Petites", "/CAO/Coques"] {
        files.create(Path::new(folder)).expect("makes the folder");
    }
    files
}

fn panel_on(files: &InMemoryFiles) -> Explorer {
    let mut explorer = Explorer::at(ROOT.into());
    explorer.refresh(files);
    explorer
}

fn path(written: &str) -> PathBuf {
    PathBuf::from(written)
}

#[test]
fn with_nothing_picked_the_card_saves_where_the_column_chose_a_part_standing_for_its_folder() {
    let files = a_library();
    let mut explorer = panel_on(&files);
    let mut card = NewCard::default();

    assert_eq!(
        card.folder(&explorer),
        path(ROOT),
        "nothing chosen in the column"
    );
    explorer.select(Path::new("/CAO/Coques"));
    assert_eq!(card.folder(&explorer), path("/CAO/Coques"));
    explorer.select(Path::new("/CAO/Brides/Équerre.caopart"));
    assert_eq!(
        card.folder(&explorer),
        path("/CAO/Brides"),
        "a part chosen in the column stands for the folder it is in",
    );
}

#[test]
fn a_pick_holds_while_the_column_keeps_the_selection_it_was_made_against() {
    let files = a_library();
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/Brides"));
    let mut card = NewCard::default();

    card.pick(path("/CAO/Coques"), &explorer);

    for frame in 1..=2 {
        assert_eq!(
            card.folder(&explorer),
            path("/CAO/Coques"),
            "frame {frame}: the folder picked gave way to the one the column holds",
        );
    }
}

#[test]
fn a_pick_the_column_moved_past_is_forgotten_and_selecting_back_does_not_bring_it_back() {
    let files = a_library();
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/Brides"));
    let mut card = NewCard::default();
    card.pick(path("/CAO/Coques"), &explorer);

    explorer.select(Path::new("/CAO/Brides/Petites"));
    assert_eq!(
        card.folder(&explorer),
        path("/CAO/Brides/Petites"),
        "the column's selection moved on, and has the last word",
    );
    explorer.select(Path::new("/CAO/Brides"));

    assert_eq!(
        card.folder(&explorer),
        path("/CAO/Brides"),
        "the pick made while « Brides » was selected came back with it",
    );
}

#[test]
fn a_pick_whose_folder_left_the_library_gives_way_to_the_column_s_folder() {
    let files = a_library();
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/Brides"));
    let mut card = NewCard::default();
    card.pick(path("/CAO/Brides/Petites"), &explorer);
    assert_eq!(card.folder(&explorer), path("/CAO/Brides/Petites"));

    let without_petites = InMemoryFiles::default();
    for part in ["/CAO/Plaque.caopart", "/CAO/Brides/Équerre.caopart"] {
        without_petites
            .write(Path::new(part), b"PK")
            .expect("writes");
    }
    without_petites
        .create(Path::new("/CAO/Coques"))
        .expect("makes the folder");
    explorer.refresh(&without_petites);

    assert_eq!(
        card.folder(&explorer),
        path("/CAO/Brides"),
        "« Petites » is gone from the library, « Brides » still selected in the column, \
         and the card would still make a part in « Petites », making the folder again",
    );
}

#[test]
fn making_hands_over_the_name_trimmed_and_the_card_s_folder_and_empties_the_field() {
    let files = a_library();
    let explorer = panel_on(&files);
    let mut card = NewCard::default();
    card.name_mut().push_str("  Support  ");
    card.pick(path("/CAO/Coques"), &explorer);

    let made = card.make(&explorer);

    assert_eq!(made, Some(("Support".to_string(), path("/CAO/Coques"))));
    assert!(card.name_mut().is_empty(), "the field keeps the name made");
    assert!(!card.can_make(), "a part can be made again with no name");
}

#[test]
fn a_blank_name_makes_nothing() {
    let files = a_library();
    let explorer = panel_on(&files);

    for typed in ["", "   "] {
        let mut card = NewCard::default();
        card.name_mut().push_str(typed);

        assert!(!card.can_make(), "« {typed} » can be made");
        assert_eq!(card.make(&explorer), None, "« {typed} » made something");
    }
}

#[test]
fn the_folders_offered_are_the_root_then_each_folder_before_those_inside_it_and_no_part() {
    let files = a_library();
    let library = cao_part::library::read(&files, Path::new(ROOT)).expect("the library reads");

    let offered: Vec<(usize, PathBuf)> = folders_offered(&library)
        .into_iter()
        .map(|(depth, folder)| (depth, folder.path.clone()))
        .collect();

    assert_eq!(
        offered,
        vec![
            (0, path("/CAO")),
            (1, path("/CAO/Brides")),
            (2, path("/CAO/Brides/Petites")),
            (1, path("/CAO/Coques")),
        ],
    );
}
