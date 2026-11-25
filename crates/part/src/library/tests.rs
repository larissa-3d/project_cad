//! What part · library/mod.rs is held to.
//!
//! Closes #743 — the whole transcription is in
//! `crates/app/src/screens/explorer/state/tests.rs`; these bullets are the
//! library's share.
//! - Filter, by name, keeping the folders that hold a match: the tree keeps
//!   only the parts whose name holds the query, whatever the case, and the
//!   folders that hold one at any depth; a folder whose own name holds the
//!   query but which holds no matching part is not kept for its name —
//!   `a_filter_keeps_the_parts_whose_name_holds_it_and_the_folders_that_lead_to_them`;
//!   an empty query keeps everything, an empty folder with it —
//!   `an_empty_filter_keeps_the_whole_library`
//! - Folders with their count of parts: a folder counts every part under it,
//!   its subfolders' included —
//!   `a_folder_counts_every_part_under_it_subfolders_included`

use super::*;

use crate::adapters::InMemoryFiles;
use crate::ports::Files;

fn library_of(paths: &[&str]) -> InMemoryFiles {
    let files = InMemoryFiles::default();
    for path in paths {
        files.write(Path::new(path), b"PK").expect("writes");
    }
    files
}

#[test]
fn a_folder_hands_back_the_parts_it_holds_and_leaves_the_rest_alone() {
    let files = library_of(&[
        "/CAO/support.caopart",
        "/CAO/notes.txt",
        "/CAO/bride.caopart",
    ]);

    let library = read(&files, Path::new("/CAO")).expect("reads");

    let names: Vec<&str> = library
        .parts
        .iter()
        .map(|part| part.name.as_str())
        .collect();
    assert_eq!(names, ["bride", "support"]);
}

#[test]
fn a_part_is_offered_under_the_name_its_file_carries_without_the_extension() {
    let files = library_of(&["/CAO/support 2.caopart"]);

    let library = read(&files, Path::new("/CAO")).expect("reads");

    assert_eq!(library.parts[0].name, "support 2");
    assert_eq!(
        library.parts[0].path,
        PathBuf::from("/CAO/support 2.caopart")
    );
}

#[test]
fn a_subfolder_comes_with_everything_under_it() {
    let files = library_of(&["/CAO/drafts/first/support.caopart"]);

    let library = read(&files, Path::new("/CAO")).expect("reads");

    assert_eq!(library.folders[0].name, "drafts");
    assert_eq!(library.folders[0].folders[0].name, "first");
    assert_eq!(library.folders[0].folders[0].parts[0].name, "support");
}

#[test]
fn folders_and_parts_each_come_in_the_order_a_reader_expects() {
    let files = library_of(&[
        "/CAO/zinc/part.caopart",
        "/CAO/Alpha/part.caopart",
        "/CAO/beta.caopart",
        "/CAO/Alpha.caopart",
    ]);

    let library = read(&files, Path::new("/CAO")).expect("reads");

    let folders: Vec<&str> = library
        .folders
        .iter()
        .map(|folder| folder.name.as_str())
        .collect();
    let parts: Vec<&str> = library
        .parts
        .iter()
        .map(|part| part.name.as_str())
        .collect();
    assert_eq!(
        folders,
        ["Alpha", "zinc"],
        "a capital does not send a folder to the top"
    );
    assert_eq!(parts, ["Alpha", "beta"]);
}

fn everything_in(folder: &Folder) -> Vec<String> {
    let mut laid = Vec::new();
    for under in &folder.folders {
        laid.push(under.path.display().to_string());
        laid.extend(everything_in(under));
    }
    laid.extend(
        folder
            .parts
            .iter()
            .map(|part| part.path.display().to_string()),
    );
    laid
}

#[test]
fn a_filter_keeps_the_parts_whose_name_holds_it_and_the_folders_that_lead_to_them() {
    let files = library_of(&[
        "/CAO/Bride.caopart",
        "/CAO/support.caopart",
        "/CAO/Brides/plate.caopart",
        "/CAO/drafts/plate.caopart",
        "/CAO/drafts/old/bride 2.caopart",
    ]);
    let library = read(&files, Path::new("/CAO")).expect("reads");

    let kept = filter(&library, "BRI");

    assert_eq!(
        everything_in(&kept),
        [
            "/CAO/drafts",
            "/CAO/drafts/old",
            "/CAO/drafts/old/bride 2.caopart",
            "/CAO/Bride.caopart",
        ],
        "the parts whose name holds `bri` whatever the case, and the folders on the way to \
         one; `Brides` holds it only in its own name",
    );
}

#[test]
fn an_empty_filter_keeps_the_whole_library() {
    let files = library_of(&["/CAO/support.caopart", "/CAO/drafts/old/bride.caopart"]);
    files
        .create(Path::new("/CAO/empty"))
        .expect("the folder is made");
    let library = read(&files, Path::new("/CAO")).expect("reads");

    assert_eq!(
        filter(&library, ""),
        library,
        "nothing typed hides nothing, the folder that holds no part with the rest",
    );
}

#[test]
fn a_folder_counts_every_part_under_it_subfolders_included() {
    let files = library_of(&[
        "/CAO/support.caopart",
        "/CAO/notes.txt",
        "/CAO/drafts/plate.caopart",
        "/CAO/drafts/old/bride.caopart",
        "/CAO/drafts/old/bride 2.caopart",
    ]);
    files
        .create(Path::new("/CAO/empty"))
        .expect("the folder is made");
    let library = read(&files, Path::new("/CAO")).expect("reads");
    let drafts = &library.folders[0];
    let old = &drafts.folders[0];
    let empty = &library.folders[1];

    assert_eq!(
        [
            library.count_of_parts(),
            drafts.count_of_parts(),
            old.count_of_parts(),
            empty.count_of_parts(),
        ],
        [4, 3, 2, 0],
        "the library, `drafts`, `drafts/old` and `empty`: each counts its own parts and \
         those of every folder under it, and a file that is not a part counts for none",
    );
}

#[test]
fn a_library_nobody_has_started_reads_as_an_empty_one() {
    let library = read(&InMemoryFiles::default(), Path::new("/CAO")).expect("reads");

    assert!(library.folders.is_empty());
    assert!(library.parts.is_empty());
    assert_eq!(library.name, "CAO");
}
