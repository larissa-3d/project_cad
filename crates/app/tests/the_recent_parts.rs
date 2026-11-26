//! The recent parts on the home screen, driven whole: what each line shows,
//! what the card says when there is none, and the header above them
//! (`docs/redesign/home-screen.md` A3, A4).
//!
//! Closes #745.
//! - Each recent part shows a 64 × 48 thumbnail (the picture saved in the
//!   part, as in the parts tree), its name, a detail line « dossier · N
//!   esquisses · N extrusions », the date `%d/%m/%Y %H:%M` in monospace and a ›
//!   chevron; a click opens it —
//!   `a_recent_part_shows_the_picture_it_was_put_away_with` for the thumbnail,
//!   `a_recent_part_reads_its_name_what_it_holds_when_it_was_opened_and_a_chevron`
//!   for the rest of the line, `a_click_on_a_recent_part_s_line_opens_it` for
//!   the click; the proportions and the monospace are looked at, in the last
//!   bullet
//! - The folder in the detail line is the part's folder relative to the parts
//!   folder, and the root reads as the root, not as an empty string —
//!   `a_recent_part_reads_its_folder_below_the_parts_folder_and_the_root_as_the_parts_folder`;
//!   decided on the way, the issue saying nothing of it: a part lying outside
//!   the parts folder reads the folder it is in, its path as it stands, since
//!   reading it as the root would send the user looking in the wrong place
//! - « Pièces récentes » carries « N pièces » on the right of the title, N
//!   being the number of lines shown —
//!   `the_recent_parts_title_counts_the_lines_shown_on_its_right`
//! - The sketch and extrusion counts are read without opening the whole part:
//!   the recents list builds no `Part` to show a line —
//!   `a_recent_part_whose_steps_cannot_be_read_still_says_what_it_holds`;
//!   beneath it, `crates/part/src/document/tests.rs` counts from the archive's
//!   index alone, and `crates/app/src/screens/start_menu/state/tests.rs` reads
//!   it once while the list stays as it was
//! - With no recent part, the card shows a dashed frame, the part icon,
//!   « Aucune pièce ouverte pour le moment. » and « Donnez un nom à gauche,
//!   puis Créer. ». —
//!   `with_no_recent_part_the_card_says_none_was_opened_and_how_to_make_one`;
//!   the dashed frame is looked at, in the last bullet
//! - The header shows « CAO » large in amber above today's subtitle,
//!   unchanged — `the_header_reads_cao_above_today_s_subtitle`; large and amber
//!   are looked at, in the last bullet
//! - A recent part moved or deleted outside the app behaves as today —
//!   `a_recent_part_moved_or_deleted_outside_the_app_still_shows_and_says_its_file_is_gone`
//! - no test: the amber, the dashed frame, the 64 × 48 proportions and the
//!   monospace date are looked at in « To try it ».

mod driver;

use std::path::PathBuf;

use cao_app::adapters::files::DiskFiles;
use cao_part::{ExtrusionMode, Operation, PartDocument, StepCounts};
use cao_sketch::WorkPlane;
use chrono::{DateTime, Utc};
use egui_kittest::kittest::{NodeT, Queryable};

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

/// A part holding two sketches, the first of them raised.
fn two_sketches_and_an_extrusion(name: &str) -> PartDocument {
    let mut part = PartDocument::new(name, at("2026-11-01T09:00:00Z"));
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    part.apply(Operation::Extrude {
        sketch: 0,
        areas: Vec::new(),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    part
}

/// Every text standing on the recent parts' line that reads this name: from
/// its name rightwards and downwards, as low as a thumbnail reaches below it.
fn the_line_of(app: &driver::App, name: &str) -> Vec<String> {
    let named = driver::the_recent_part_named(app, name);
    let line = egui::Rect::from_min_max(
        egui::pos2(named.left() - 1.0, named.top() - 12.0),
        egui::pos2(f32::INFINITY, named.bottom() + 40.0),
    );
    app.get_all_by(|_| true)
        .filter_map(|node| {
            let read = node.accesskit_node();
            let text = read.label().or_else(|| read.value())?;
            line.contains(node.rect().center()).then_some(text)
        })
        .collect()
}

#[test]
fn a_recent_part_reads_its_name_what_it_holds_when_it_was_opened_and_a_chevron() {
    let test = "a_recent_part_reads_its_name_what_it_holds";
    let opened_at = at("2026-11-05T15:32:00Z");
    let app = driver::open_remembering_parts(
        test,
        vec![driver::RecentPart {
            lies_at: "Support.caopart".into(),
            part: two_sketches_and_an_extrusion("Support"),
            opened_at,
        }],
    );
    let laid = cao_prefs::default_projects_dir(&driver::locations_of(test)).join("Support.caopart");
    assert_eq!(
        PartDocument::step_counts_in(&DiskFiles, &laid),
        Some(StepCounts {
            sketches: 2,
            extrusions: 1,
        }),
        "the part laid down holds two sketches and an extrusion",
    );

    let read = the_line_of(&app, "Support");

    let when = opened_at
        .with_timezone(&cao_app::adapters::clock::reader_zone())
        .format("%d/%m/%Y %H:%M")
        .to_string();
    let missing: Vec<&str> = ["Support", "2 esquisses · 1 extrusion", &when, "›"]
        .into_iter()
        .filter(|wanted| !read.iter().any(|text| text.contains(wanted)))
        .collect();
    assert!(
        missing.is_empty(),
        "the recent line of « Support » does not read {missing:?}; it reads {read:?}",
    );
}

/// What the recent line of this part says of where it lies and what it holds:
/// the text of its line joining them with « · ».
fn the_detail_of(app: &driver::App, name: &str) -> Option<String> {
    the_line_of(app, name)
        .into_iter()
        .find(|text| text.contains(" · "))
}

#[test]
fn a_recent_part_reads_its_folder_below_the_parts_folder_and_the_root_as_the_parts_folder() {
    let test = "a_recent_part_reads_its_folder";
    let elsewhere = driver::locations_of(test)
        .documents
        .expect("the test keeps its documents in a folder of its own")
        .join("Ailleurs");
    let opened_at = at("2026-11-05T15:32:00Z");
    let laid = |lies_at: PathBuf, name: &str| driver::RecentPart {
        lies_at,
        part: PartDocument::new(name, opened_at),
        opened_at,
    };
    let app = driver::open_remembering_parts(
        test,
        vec![
            laid("Équerre.caopart".into(), "Équerre"),
            laid("Brides/Platine.caopart".into(), "Platine"),
            laid("Brides/Anciennes/Cale.caopart".into(), "Cale"),
            laid(elsewhere.join("Support.caopart"), "Support"),
        ],
    );

    let wrong: Vec<String> = [
        ("Équerre", "Dossier des pièces".to_string()),
        ("Platine", "Brides".to_string()),
        ("Cale", "Brides / Anciennes".to_string()),
        ("Support", elsewhere.display().to_string()),
    ]
    .into_iter()
    .filter_map(|(name, folder)| {
        let detail = the_detail_of(&app, name);
        let reads_it = detail
            .as_deref()
            .is_some_and(|said| said.starts_with(&format!("{folder} · ")));
        (!reads_it).then(|| format!("« {name} » reads {detail:?}, not « {folder} · … »"))
    })
    .collect();

    assert!(
        wrong.is_empty(),
        "a recent part does not say where it lies below the parts folder — the \
         root as the parts folder, a part outside it by the folder it is really \
         in:\n  {}",
        wrong.join("\n  "),
    );
}
