//! What app · screens/start_menu/state.rs is held to: when the home screen
//! reads what each recent part holds, and what a line says when that cannot be
//! read. What a line reads on screen is driven whole in
//! `crates/app/tests/the_recent_parts.rs`, where #745 is transcribed.

use std::cell::Cell;
use std::path::{Path, PathBuf};

use cao_part::{ExtrusionMode, FileError, Files, InMemoryFiles, Operation, PartDocument};
use cao_prefs::RecentEntry;
use cao_sketch::WorkPlane;
use chrono::{DateTime, Utc};

use super::*;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

const PARTS: &str = "/parts";

/// The files, counting how many times one of them is read.
#[derive(Default)]
struct CountedFiles {
    files: InMemoryFiles,
    reads: Cell<usize>,
}

impl Files for CountedFiles {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        self.reads.set(self.reads.get() + 1);
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

    for _frame in 0..3 {
        menu.follow(&recents, &files);
    }

    assert_eq!(
        files.reads.get(),
        1,
        "the home screen is drawn every frame, and a part's index is read again \
         at each of them",
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
