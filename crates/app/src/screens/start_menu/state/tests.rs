//! What app · screens/start_menu/state.rs is held to: when the home screen
//! reads what each recent part holds and the picture it carries, and what a
//! line says when that cannot be read. What a line reads on screen is driven whole in
//! `crates/app/tests/the_recent_parts.rs`, where #745 is transcribed.

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use cao_part::{ExtrusionMode, FileError, Files, InMemoryFiles, Operation, PartDocument, Picture};
use cao_prefs::RecentEntry;
use cao_sketch::WorkPlane;
use chrono::{DateTime, Utc};

use super::*;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

const PARTS: &str = "/parts";

/// The files, noting every one that is read.
#[derive(Default)]
struct CountedFiles {
    files: InMemoryFiles,
    read: RefCell<Vec<PathBuf>>,
}

impl CountedFiles {
    /// How many times a file has been read, any file.
    fn reads(&self) -> usize {
        self.read.borrow().len()
    }

    /// How many times this file has been read.
    fn reads_of(&self, path: &Path) -> usize {
        self.read
            .borrow()
            .iter()
            .filter(|read| *read == path)
            .count()
    }
}

impl Files for CountedFiles {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        self.read.borrow_mut().push(path.to_path_buf());
        self.files.read(path)
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), FileError> {
        self.files.write(path, bytes)
    }

    fn exists(&self, path: &Path) -> bool {
        self.files.exists(path)
    }
}

/// A part holding one sketch, raised as many times as asked, written at the
/// root of the parts folder.
fn a_part_raised(files: &impl Files, name: &str, extrusions: usize) -> PathBuf {
    let mut part = PartDocument::new(name, at("2026-11-01T09:00:00Z"));
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for _ in 0..extrusions {
        part.apply(Operation::Extrude {
            sketch: 0,
            areas: Vec::new(),
            distance: 4.0.into(),
            mode: ExtrusionMode::Add,
        });
    }
    let path = Path::new(PARTS).join(format!("{name}.caopart"));
    part.save(files, &path, at("2026-11-01T09:00:00Z"))
        .expect("the part is written");
    path
}

fn opened(path: &Path, name: &str, when: &str) -> RecentEntry {
    RecentEntry {
        path: path.to_path_buf(),
        name: name.to_string(),
        opened_at: at(when),
    }
}

fn details(menu: &StartMenu) -> Vec<String> {
    menu.lines(Path::new(PARTS), &Catalogue::french(), &Utc)
        .into_iter()
        .map(|line| line.detail)
        .collect()
}

#[test]
fn what_a_recent_part_holds_is_read_once_while_the_list_stays_as_it_was() {
    let files = CountedFiles::default();
    let part = a_part_raised(&files, "Support", 1);
    let recents = [opened(&part, "Support", "2026-11-05T15:32:00Z")];
    let mut menu = StartMenu::default();
    menu.follow(&recents, &files);
    let on_the_first_frame = files.reads();

    for _frame in 0..3 {
        menu.follow(&recents, &files);
    }

    assert_eq!(
        files.reads(),
        on_the_first_frame,
        "the home screen is drawn every frame, and a part is read again at each \
         of them",
    );
    assert_eq!(
        details(&menu),
        ["Dossier des pièces · 1 esquisse · 1 extrusion"]
    );
}

#[test]
fn what_a_recent_part_holds_is_read_again_once_it_is_opened_anew() {
    let files = InMemoryFiles::default();
    let part = a_part_raised(&files, "Support", 1);
    let mut menu = StartMenu::default();
    menu.follow(&[opened(&part, "Support", "2026-11-05T15:32:00Z")], &files);

    a_part_raised(&files, "Support", 2);
    menu.follow(&[opened(&part, "Support", "2026-11-06T08:10:00Z")], &files);

    assert_eq!(
        details(&menu),
        ["Dossier des pièces · 1 esquisse · 2 extrusions"],
        "a part worked on and put away comes back to the home screen with what it \
         holds now",
    );
}

#[test]
fn a_recent_part_with_nothing_drawn_yet_reads_its_zeros_in_the_singular() {
    let files = InMemoryFiles::default();
    let path = Path::new(PARTS).join("Neuve.caopart");
    PartDocument::new("Neuve", at("2026-11-01T09:00:00Z"))
        .save(&files, &path, at("2026-11-01T09:00:00Z"))
        .expect("the part is written");
    let mut menu = StartMenu::default();

    menu.follow(&[opened(&path, "Neuve", "2026-11-05T15:32:00Z")], &files);

    assert_eq!(
        details(&menu),
        ["Dossier des pièces · 0 esquisse · 0 extrusion"],
        "French counts nothing in the singular, and every part just made comes back to \
         the home screen holding nothing",
    );
}

fn a_drawn_picture() -> Picture {
    Picture::new(4, 3, vec![200; 4 * 3 * 4]).expect("the pixels fill the size")
}

#[test]
fn the_picture_of_a_recent_part_is_read_once_while_the_list_stays_as_it_was() {
    let files = CountedFiles::default();
    let mut part = PartDocument::new("Support", at("2026-11-01T09:00:00Z"));
    part.set_picture(a_drawn_picture());
    let path = Path::new(PARTS).join("Support.caopart");
    part.save(&files, &path, at("2026-11-01T09:00:00Z"))
        .expect("the part is written");
    let recents = [opened(&path, "Support", "2026-11-05T15:32:00Z")];
    let mut menu = StartMenu::default();
    menu.follow(&recents, &files);
    let on_the_first_frame = files.reads();

    for _frame in 0..3 {
        menu.follow(&recents, &files);
    }

    assert_eq!(menu.picture_of(&path), Some(&a_drawn_picture()));
    assert_eq!(
        files.reads(),
        on_the_first_frame,
        "the home screen is drawn every frame, and a part is read again at each \
         of them",
    );
}

#[test]
fn a_part_whose_line_has_not_changed_is_not_read_again_when_another_is_opened() {
    let files = CountedFiles::default();
    let support = a_part_raised(&files, "Support", 1);
    let equerre = a_part_raised(&files, "Équerre", 2);
    let mut menu = StartMenu::default();
    menu.follow(
        &[
            opened(&support, "Support", "2026-11-05T15:32:00Z"),
            opened(&equerre, "Équerre", "2026-11-04T10:00:00Z"),
        ],
        &files,
    );
    let (support_before, equerre_before) = (files.reads_of(&support), files.reads_of(&equerre));

    menu.follow(
        &[
            opened(&equerre, "Équerre", "2026-11-06T08:10:00Z"),
            opened(&support, "Support", "2026-11-05T15:32:00Z"),
        ],
        &files,
    );

    assert_eq!(
        (
            files.reads_of(&support) - support_before,
            files.reads_of(&equerre) > equerre_before,
        ),
        (0, true),
        "(times « Support » was read again, whether « Équerre » was): opening « Équerre » \
         gave « Support » nothing new to show",
    );
}

#[test]
fn a_recent_part_s_file_is_read_at_most_once_to_show_its_line() {
    let files = CountedFiles::default();
    let mut part = PartDocument::new("Support", at("2026-11-01T09:00:00Z"));
    part.set_picture(a_drawn_picture());
    let path = Path::new(PARTS).join("Support.caopart");
    part.save(&files, &path, at("2026-11-01T09:00:00Z"))
        .expect("the part is written");
    let mut menu = StartMenu::default();

    menu.follow(&[opened(&path, "Support", "2026-11-05T15:32:00Z")], &files);

    assert!(
        files.reads_of(&path) <= 1,
        "the whole file of « Support » is read {} times to show one line: what it holds \
         and its picture are in the one archive",
        files.reads_of(&path),
    );
    assert_eq!(
        (details(&menu), menu.picture_of(&path)),
        (
            vec!["Dossier des pièces · 0 esquisse · 0 extrusion".to_string()],
            Some(&a_drawn_picture()),
        ),
    );
}

#[test]
fn a_recent_part_whose_contents_cannot_be_read_says_only_where_it_lies() {
    let files = InMemoryFiles::default();
    let damaged = Path::new(PARTS).join("Abîmée.caopart");
    files
        .write(&damaged, b"not an archive")
        .expect("the file is written");
    let mut menu = StartMenu::default();

    menu.follow(
        &[opened(&damaged, "Abîmée", "2026-11-05T15:32:00Z")],
        &files,
    );

    assert_eq!(details(&menu), ["Dossier des pièces"]);
}
