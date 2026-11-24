//! Whether a section the part keeps was shown or hidden by hand goes with the
//! part: written in the `.caopart` beside the history and read back, and read
//! as no choice at all from a part saved before it was written.
//!
//! What the part keeps is the hand's choice alone, one per section: `Some(true)`
//! for an eye opened by hand, `Some(false)` for one shut by hand, `None` for an
//! eye left to the rule. Which section the canvas cuts with is worked out from
//! it above the part, as a sketch's eye is, and is not this file's business.
//!
//! Closes #1108 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests.rs`; these are the part's
//! halves, before any frame reads them.
//! - The state of each section is written in the part and read back: a section
//!   hidden before closing is hidden on reopening. A part test checks it — a
//!   section shut by hand, one opened by hand and one left alone come back
//!   from the file saved or put away, each as the hand left it —
//!   `each_section_comes_back_from_the_caopart_shut_open_or_unset_as_the_hand_left_it`
//! - A part saved before this change opens with its last section shown, none
//!   hand-set — the part's half: a part written before the choices were, its
//!   geometry kept or not, opens with no section set by hand, and takes a
//!   choice and keeps it as any part does —
//!   `a_part_saved_before_sections_were_set_by_hand_opens_with_none_set_and_takes_one_as_any_part`
//! - decided before the red step: a choice is kept only while its section
//!   stands; a section undone takes its choice with it, so that the next one
//!   saved, which takes the same rank, comes with none, in memory and through
//!   the file —
//!   `a_hand_choice_leaves_with_its_section_and_never_passes_to_the_next_one_saved`
//! - decided before the red step: the hand's choice is no step of the history;
//!   undo does not take it back and it moves no key step of #968 —
//!   `a_hand_choice_is_no_step_of_the_history_and_undo_leaves_it_alone`
//! - decided before the red step: stored as a field an old part reads as no
//!   choice, no migration and `SCHEMA_VERSION` unchanged — the fixture of
//!   `a_part_saved_before_sections_were_set_by_hand_opens_with_none_set_and_takes_one_as_any_part`
//!   says schema version 6 and has to open

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};

use cao_part::history::{Operation, SectionAsked, SideDrawing};
use cao_part::{FileError, Files, PartDocument};
use cao_sketch::WorkPlane;
use glam::DVec3;

#[derive(Default)]
struct Memory(RefCell<HashMap<PathBuf, Vec<u8>>>);

impl Files for Memory {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        self.0
            .borrow()
            .get(path)
            .cloned()
            .ok_or_else(|| FileError::Absent(path.to_path_buf()))
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), FileError> {
        self.0
            .borrow_mut()
            .insert(path.to_path_buf(), bytes.to_vec());
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.0.borrow().contains_key(path)
    }
}

fn at_nine() -> chrono::DateTime<chrono::Utc> {
    "2026-11-24T09:00:00Z".parse().expect("a date")
}

/// « Enregistrer la coupe » on a section along `plane`, `offset` from it,
/// nothing tilted.
fn a_section_along(plane: WorkPlane, offset: f64) -> Operation {
    Operation::SaveSection(SectionAsked {
        plane,
        on: None,
        offset,
        behind: SideDrawing::SeeThrough,
        in_front: SideDrawing::Opaque,
        tilt_about_u_degrees: 0.0,
        tilt_about_v_degrees: 0.0,
        pivot: DVec3::ZERO,
    })
}

/// Each section the part keeps, in the order it was made, with what the hand
/// chose for it.
fn choices_of(document: &PartDocument) -> Vec<(usize, Option<bool>)> {
    document
        .sections()
        .iter()
        .map(|kept| (kept.id, document.section_choice(kept.id)))
        .collect()
}

#[test]
fn each_section_comes_back_from_the_caopart_shut_open_or_unset_as_the_hand_left_it() {
    let mut document = PartDocument::new("Part", at_nine());
    for (plane, offset) in [
        (WorkPlane::YZ, 15.0),
        (WorkPlane::XZ, 5.0),
        (WorkPlane::XY, 2.0),
    ] {
        document.apply(a_section_along(plane, offset));
    }
    let ids: Vec<usize> = document.sections().iter().map(|kept| kept.id).collect();
    assert_eq!(ids.len(), 3, "the fixture wants three sections kept");
    document.set_section_choice(ids[0], Some(false));
    document.set_section_choice(ids[1], Some(true));
    let files = Memory::default();
    let saved = Path::new("/parts/saved.caopart");
    let put_away = Path::new("/parts/put-away.caopart");
    document
        .save(&files, saved, at_nine())
        .expect("the part is saved");
    document
        .put_away(&files, put_away, at_nine())
        .expect("the part is put away");

    for (how, part) in [
        ("in memory", document.clone()),
        (
            "saved and reopened",
            PartDocument::load(&files, saved).expect("the saved part opens again"),
        ),
        (
            "put away and reopened",
            PartDocument::load(&files, put_away).expect("the part put away opens again"),
        ),
    ] {
        assert_eq!(
            choices_of(&part),
            [(ids[0], Some(false)), (ids[1], Some(true)), (ids[2], None)],
            "{how}: the first section shut by hand, the second opened by hand and the third left \
             alone did not read so, as (section, choice) where Some(true) is opened by hand, \
             Some(false) shut by hand and None left to the rule",
        );
    }
}

#[test]
fn a_hand_choice_leaves_with_its_section_and_never_passes_to_the_next_one_saved() {
    let mut document = PartDocument::new("Part", at_nine());
    document.apply(a_section_along(WorkPlane::YZ, 15.0));
    let undone = document.sections()[0].id;
    document.set_section_choice(undone, Some(false));
    let while_it_stood = document.section_choice(undone);

    document.undo();
    document.apply(a_section_along(WorkPlane::XZ, 5.0));
    let next = document.sections()[0].id;

    assert_eq!(
        while_it_stood,
        Some(false),
        "the fixture wants the first section shut by hand while it stands",
    );
    assert_eq!(
        next, undone,
        "the fixture wants the section saved after the undo to take the rank of the one undone",
    );
    let files = Memory::default();
    let path = Path::new("/parts/after-an-undo.caopart");
    document
        .save(&files, path, at_nine())
        .expect("the part is saved");
    for (how, part) in [
        ("in memory", document.clone()),
        (
            "saved and reopened",
            PartDocument::load(&files, path).expect("the part opens again"),
        ),
    ] {
        assert_eq!(
            part.section_choice(next),
            None,
            "{how}: the section saved after the first was undone came with the choice the hand \
             made for the one undone",
        );
    }
}

/// The archive a part with two sections, along YZ 15 from it and along XZ 5
/// from it, was written as before the hand's choices were: the files of
/// `tests/fixtures/two_sections_saved_before_1108/`, the cache of its geometry
/// with them or not.
fn a_part_saved_before_the_hand_s_choices(with_its_geometry: bool) -> Vec<u8> {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("two_sections_saved_before_1108");
    let mut entries = vec![
        "part.json",
        "design/history.json",
        "design/section-0/steps.json",
        "design/section-1/steps.json",
    ];
    if with_its_geometry {
        entries.push("geometry.json");
    }
    let mut archive = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for entry in entries {
        let bytes = std::fs::read(folder.join(entry))
            .unwrap_or_else(|err| panic!("the fixture {entry} does not read: {err}"));
        archive.start_file(entry, options).expect("an entry opens");
        archive.write_all(&bytes).expect("an entry is written");
    }
    archive.finish().expect("the archive closes").into_inner()
}

#[test]
fn a_part_saved_before_sections_were_set_by_hand_opens_with_none_set_and_takes_one_as_any_part() {
    for with_its_geometry in [false, true] {
        let files = Memory::default();
        let path = Path::new("/parts/before-1108.caopart");
        files
            .write(
                path,
                &a_part_saved_before_the_hand_s_choices(with_its_geometry),
            )
            .expect("the fixture is laid down");

        let mut part = PartDocument::load(&files, path).unwrap_or_else(|err| {
            panic!(
                "a part saved before #1108, its geometry kept {with_its_geometry}, no longer \
                 opens: {err:?}"
            )
        });
        let opened = choices_of(&part);
        let ids: Vec<usize> = part.sections().iter().map(|kept| kept.id).collect();
        assert_eq!(
            ids.len(),
            2,
            "a part saved before #1108 came back with {:?}, its geometry kept {with_its_geometry}",
            part.sections(),
        );
        part.set_section_choice(ids[1], Some(false));
        part.save(&files, path, at_nine())
            .expect("the part saved before #1108 is saved again");
        let saved_again = PartDocument::load(&files, path)
            .map(|reopened| choices_of(&reopened))
            .expect("the part saved before #1108 opens once saved again");

        assert_eq!(
            opened,
            [(ids[0], None), (ids[1], None)],
            "a part saved before #1108, its geometry kept {with_its_geometry}, opened with a \
             section set by hand, as (section, choice)",
        );
        assert_eq!(
            saved_again,
            [(ids[0], None), (ids[1], Some(false))],
            "a part saved before #1108, its geometry kept {with_its_geometry}, its last section \
             then shut by hand and saved again, did not come back so, as (section, choice)",
        );
    }
}

#[test]
fn a_hand_choice_is_no_step_of_the_history_and_undo_leaves_it_alone() {
    let mut document = PartDocument::new("Part", at_nine());
    document.apply(a_section_along(WorkPlane::YZ, 15.0));
    document.apply(a_section_along(WorkPlane::XZ, 5.0));
    let first = document.sections()[0].id;
    let history = |document: &PartDocument| {
        (
            document.history.operations().len(),
            document.history.steps().len(),
        )
    };
    let before = history(&document);

    document.set_section_choice(first, Some(false));
    let after = history(&document);
    let undone = document.undo();

    assert_eq!(
        after, before,
        "shutting a section's eye by hand wrote into the history, as (operations, steps)",
    );
    assert_eq!(
        (
            undone,
            document.sections().len(),
            document.section_choice(first)
        ),
        (true, 1, Some(false)),
        "the first of two sections shut by hand, then an undo: the undo did not take back the \
         second section, or it took the hand's choice back with it, as (undone, sections left, \
         the first one's choice)",
    );
}
