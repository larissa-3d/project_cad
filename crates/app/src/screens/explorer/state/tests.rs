//! What app · screens/explorer/state.rs is held to.
//!
//! Closes #743 — the issue has no Done when of its own; these are the points of
//! its « What it does » and « What must not break », read as its criteria when
//! the work opened, after a survey of the code had found the column already on
//! the home screen and already gone from the part screen. The bullets the
//! library and the recent list hold are transcribed again, checked, beside
//! them: in `crates/part/src/library/tests.rs`,
//! `crates/part/src/library/tidying/tests.rs` and
//! `crates/prefs/src/recents/tests.rs`.
//! - Filter, by name, keeping the folders that hold a match: the library hands
//!   back its tree keeping only the parts whose name holds the query, whatever
//!   the case, and the folders that hold one at any depth; an empty query keeps
//!   everything; a folder whose own name holds the query but which holds no
//!   matching part is not kept for its name — no test: in this file; held by
//!   `a_filter_keeps_the_parts_whose_name_holds_it_and_the_folders_that_lead_to_them`
//!   and `an_empty_filter_keeps_the_whole_library`, in
//!   `crates/part/src/library/tests.rs`
//! - Folders with their count of parts: a folder counts every part under it,
//!   its subfolders' included — no test: in this file; held by
//!   `a_folder_counts_every_part_under_it_subfolders_included`, in
//!   `crates/part/src/library/tests.rs`
//! - Drag a part onto a folder to move it there: the part lands in the folder
//!   under the file name it had, with what its archive holds, its picture and
//!   its name among it, and the new path is handed back; it is refused where
//!   the folder already holds a part of that name; moved into the folder it is
//!   already in, it stays where it is — no test: in this file; held by
//!   `a_part_moved_into_a_folder_keeps_its_file_name_its_picture_and_its_name`,
//!   `a_part_is_not_moved_into_a_folder_already_holding_one_of_that_name` and
//!   `a_part_moved_into_the_folder_it_is_in_stays_where_it_is`, in
//!   `crates/part/src/library/tidying/tests.rs`
//! - A part moved by drag keeps its recent entry pointing to it: the recent
//!   list can be told that a path became another, and the entry keeps its
//!   place and its name under the new path; one that would then repeat a path
//!   already listed is listed once, as a push does — no test: in this file;
//!   held by
//!   `a_recent_part_moved_elsewhere_keeps_its_place_and_its_name_under_its_new_path`
//!   and `a_recent_part_moved_onto_a_path_already_listed_is_listed_once`, in
//!   `crates/prefs/src/recents/tests.rs`
//! - the column holds what is typed in « Filtrer… », and the tree it draws is
//!   the filtered one — `the_tree_drawn_keeps_only_what_the_filter_keeps`
//! - a part dropped on a folder moves there, and the counts follow —
//!   `a_part_dropped_on_a_folder_moves_there_and_the_counts_follow`
//! - the selection follows the part to its new place —
//!   `the_selection_follows_a_part_dropped_on_a_folder`
//! - the move says where the part was and where it went, which is what the
//!   application hands the recent list —
//!   `a_part_dropped_on_a_folder_says_where_it_was_and_where_it_went`
//! - the call that hands that pair to the recent list — no test: a line of
//!   wiring in the shell, between this presenter and `Remembered`, which reads
//!   and writes the real disk; the headless driver could reach it only through
//!   a drag drawn by hand on the column. Both ends of it are held above
//! - Everything the Files panel does today, as it was: refresh, new folder,
//!   rename, delete confirmed and to the bin, thumbnails —
//!   `a_panel_just_opened_shows_the_folders_and_the_parts_of_the_library`,
//!   `a_new_folder_lands_inside_the_folder_selected`,
//!   `renaming_a_part_leaves_the_selection_on_it_under_its_new_name`,
//!   `nothing_is_thrown_away_before_the_question_is_answered`,
//!   `a_part_thrown_away_leaves_the_library_and_the_selection`,
//!   `only_the_parts_a_row_was_drawn_for_are_read_off_the_disk`
//! - The column « Toutes les pièces », 280 to 320 px wide; « ↺ »,
//!   « + Dossier », « Renommer », « Supprimer » and the « Filtrer… » field;
//!   folders that fold; one part per line with its 22 × 18 thumbnail; the
//!   chosen part lit amber; the drag drawn under the pointer — no test:
//!   drawing, in `view.rs`, which decides nothing; what each gesture asks of
//!   the presenter is held above, and the look is Tom's to try in the app
//!
//! Set aside when the work opened, not criteria: the double-click that opens a
//! part, in a second window when one is already open, stays as it is today;
//! the folder chosen in the tree as the default of the « Nouveau » card's
//! « Enregistrer dans » waits for that card to exist.

use std::path::Path;

use cao_part::{Files, InMemoryFiles, PartDocument, PartFileError};

use super::*;

fn library_of(parts: &[&str]) -> InMemoryFiles {
    let files = InMemoryFiles::default();
    for path in parts {
        files.write(Path::new(path), b"PK").expect("writes");
    }
    files
}

fn panel_on(files: &InMemoryFiles) -> Explorer {
    let mut explorer = Explorer::at("/CAO".into());
    explorer.refresh(files);
    explorer
}

/// Types `name` into the field the panel has open, the way the view does:
/// the naming is taken out, written to, and handed back.
fn typing(explorer: &mut Explorer, name: &str) {
    let mut naming = explorer.naming().cloned().expect("a name is asked for");
    *naming.typed_mut() = name.into();
    explorer.set_naming(Some(naming));
}

fn written_part(files: &InMemoryFiles, inside: &str, name: &str) -> PathBuf {
    let (_, path) = PartDocument::create_in(
        files,
        Path::new(inside),
        name,
        "2026-01-02T09:00:00Z".parse().expect("a date"),
    )
    .expect("the part is written");
    path
}

#[test]
fn a_panel_just_opened_shows_the_folders_and_the_parts_of_the_library() {
    let files = library_of(&["/CAO/support.caopart", "/CAO/drafts/bride.caopart"]);

    let explorer = panel_on(&files);

    assert_eq!(explorer.library().parts[0].name, "support");
    assert_eq!(explorer.library().folders[0].name, "drafts");
}

#[test]
fn a_folder_is_open_to_begin_with_and_stays_shut_once_folded() {
    let files = library_of(&["/CAO/drafts/bride.caopart"]);
    let mut explorer = panel_on(&files);
    let drafts = Path::new("/CAO/drafts");

    assert!(explorer.unfolded(drafts), "the panel opens showing nothing");
    explorer.fold(drafts, false);

    assert!(!explorer.unfolded(drafts));
}

#[test]
fn a_new_folder_lands_inside_the_folder_selected() {
    let files = library_of(&["/CAO/drafts/bride.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/drafts"));

    explorer.start_new_folder();
    typing(&mut explorer, "Kept");
    explorer.confirm_naming(&files, &files);

    assert_eq!(explorer.selected(), Some(Path::new("/CAO/drafts/Kept")));
    assert_eq!(explorer.library().folders[0].folders[0].name, "Kept");
}

#[test]
fn a_new_folder_lands_beside_the_part_selected_rather_than_inside_it() {
    let files = library_of(&["/CAO/drafts/bride.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/drafts/bride.caopart"));

    explorer.start_new_folder();
    typing(&mut explorer, "Kept");
    explorer.confirm_naming(&files, &files);

    assert_eq!(explorer.selected(), Some(Path::new("/CAO/drafts/Kept")));
}

#[test]
fn a_new_folder_lands_at_the_root_when_nothing_is_selected() {
    let files = library_of(&["/CAO/support.caopart"]);
    let mut explorer = panel_on(&files);

    explorer.start_new_folder();
    typing(&mut explorer, "Brides");
    explorer.confirm_naming(&files, &files);

    assert_eq!(explorer.selected(), Some(Path::new("/CAO/Brides")));
}

#[test]
fn a_name_already_taken_keeps_the_field_open_rather_than_dropping_what_was_typed() {
    let files = library_of(&["/CAO/drafts/bride.caopart"]);
    let mut explorer = panel_on(&files);

    explorer.start_new_folder();
    typing(&mut explorer, "drafts");
    explorer.confirm_naming(&files, &files);

    assert!(matches!(
        explorer.trouble(),
        Some(PartFileError::NameTaken(_))
    ));
    assert!(
        explorer.naming().is_some(),
        "the user has to type the whole name again",
    );
}

#[test]
fn renaming_a_part_leaves_the_selection_on_it_under_its_new_name() {
    let files = InMemoryFiles::default();
    let path = written_part(&files, "/CAO", "Support");
    let mut explorer = panel_on(&files);
    explorer.select(&path);

    explorer.start_rename();
    typing(&mut explorer, "Bride");
    explorer.confirm_naming(&files, &files);

    assert_eq!(explorer.selected(), Some(Path::new("/CAO/Bride.caopart")));
    assert_eq!(explorer.library().parts[0].name, "Bride");
}

#[test]
fn a_rename_starts_from_the_name_the_thing_already_has() {
    let files = library_of(&["/CAO/support.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/support.caopart"));

    explorer.start_rename();

    assert_eq!(
        explorer.naming(),
        Some(&Naming::Rename {
            path: "/CAO/support.caopart".into(),
            typed: "support".into(),
        }),
    );
}

fn part_with_a_picture(files: &InMemoryFiles, inside: &str, name: &str) -> PathBuf {
    let path = written_part(files, inside, name);
    let mut document = PartDocument::load(files, &path).expect("reads");
    document.set_picture(cao_part::Picture::new(2, 2, (0..16).collect()).expect("a picture"));
    document
        .save(
            files,
            &path,
            "2026-01-02T09:00:00Z".parse().expect("a date"),
        )
        .expect("the part is written");
    path
}

#[test]
fn only_the_parts_a_row_was_drawn_for_are_read_off_the_disk() {
    let files = InMemoryFiles::default();
    let shown = part_with_a_picture(&files, "/CAO", "Support");
    let unseen = part_with_a_picture(&files, "/CAO", "Bride");
    let mut explorer = panel_on(&files);

    explorer.wants_pictures_of(vec![shown.clone()]);
    explorer.read_pictures(&files);

    assert!(explorer.picture_of(&shown).is_some());
    assert!(
        explorer.picture_of(&unseen).is_none(),
        "a folder of a thousand parts would be a thousand reads and a thousand textures",
    );
}

#[test]
fn a_part_that_carries_no_picture_is_not_read_again_the_next_frame() {
    let files = InMemoryFiles::default();
    let bare = written_part(&files, "/CAO", "Support");
    let mut explorer = panel_on(&files);

    explorer.wants_pictures_of(vec![bare.clone()]);
    explorer.read_pictures(&files);
    files
        .write(Path::new("/CAO/Support.caopart"), b"not an archive at all")
        .expect("writes");
    explorer.wants_pictures_of(vec![bare.clone()]);
    explorer.read_pictures(&files);

    assert!(
        explorer.picture_of(&bare).is_none(),
        "having none is remembered, so the archive is opened once and not once a frame",
    );
}

#[test]
fn reading_the_library_again_forgets_the_pictures_it_had_read() {
    let files = InMemoryFiles::default();
    let part = part_with_a_picture(&files, "/CAO", "Support");
    let mut explorer = panel_on(&files);
    explorer.wants_pictures_of(vec![part.clone()]);
    explorer.read_pictures(&files);

    explorer.refresh(&files);

    assert!(
        explorer.picture_of(&part).is_none(),
        "a part renamed or drawn on since would keep showing what it used to look like",
    );
}

#[test]
fn nothing_is_thrown_away_before_the_question_is_answered() {
    let files = library_of(&["/CAO/support.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/support.caopart"));

    explorer.ask_to_discard();

    assert_eq!(
        explorer.confirming(),
        Some(Path::new("/CAO/support.caopart"))
    );
    assert_eq!(explorer.library().parts.len(), 1, "it is already gone");
}

#[test]
fn a_part_thrown_away_leaves_the_library_and_the_selection() {
    let files = library_of(&["/CAO/support.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/support.caopart"));

    explorer.ask_to_discard();
    explorer.discard(&files);

    assert!(explorer.library().parts.is_empty());
    assert_eq!(explorer.selected(), None);
}

#[test]
fn a_question_answered_no_leaves_the_part_where_it_is() {
    let files = library_of(&["/CAO/support.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/support.caopart"));

    explorer.ask_to_discard();
    explorer.cancel_discard();
    explorer.discard(&files);

    assert_eq!(explorer.library().parts.len(), 1);
}
