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
//!   `the_recent_parts_title_counts_the_lines_shown_on_its_right`; decided on
//!   the way, after the mockup of the empty home screen: with no line, the
//!   title carries no count
//! - The sketch and extrusion counts are read without opening the whole part:
//!   the recents list builds no `Part` to show a line —
//!   `a_recent_part_that_does_not_open_still_says_what_it_holds`, a part the
//!   application refuses to open reading what it holds all the same;
//!   beneath it, `crates/part/src/document/tests.rs` counts from the archive's
//!   index alone, and `crates/app/src/screens/start_menu/state/tests.rs` reads
//!   it once while the list stays as it was
//! - With no recent part, the card shows a dashed frame, the part icon,
//!   « Aucune pièce ouverte pour le moment. » and « Donnez un nom à gauche,
//!   puis Créer. ». —
//!   `with_no_recent_part_the_card_says_none_was_opened_and_how_to_make_one`;
//!   the dashed frame is looked at, in the last bullet
//! - The header shows « CAO » large in amber above today's subtitle,
//!   unchanged — `the_header_reads_cao_above_today_s_subtitle`, « large » read
//!   as at least twice as tall as the subtitle, the mockup's 52 px standing
//!   about three times its subtitle; the amber is looked at, in the last bullet
//! - A recent part moved or deleted outside the app behaves as today —
//!   `a_recent_part_moved_or_deleted_outside_the_app_still_shows_and_says_its_file_is_gone`
//! - no test: the amber, the dashed frame, the 64 × 48 proportions and the
//!   monospace date are looked at in « To try it ».

mod driver;

use std::path::PathBuf;

use cao_app::adapters::files::DiskFiles;
use cao_part::{ExtrusionMode, Operation, PartDocument, Picture, StepCounts};
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

/// Where the recent parts' line that reads this name stands: from its name
/// rightwards and downwards, as low as a thumbnail reaches below it.
fn the_band_of(app: &driver::App, name: &str) -> egui::Rect {
    let named = driver::the_recent_part_named(app, name);
    egui::Rect::from_min_max(
        egui::pos2(named.left() - 1.0, named.top() - 12.0),
        egui::pos2(f32::INFINITY, named.bottom() + 40.0),
    )
}

/// Where a text reading exactly this stands on the recent line of this part.
fn where_on_the_line_of(app: &driver::App, name: &str, text: &str) -> Option<egui::Rect> {
    let line = the_band_of(app, name);
    app.get_all_by(|_| true).find_map(|node| {
        let read = node.accesskit_node();
        let said = read.label().or_else(|| read.value())?;
        let at = node.rect();
        (said == text && line.contains(at.center())).then_some(at)
    })
}

fn a_part_is_open(app: &driver::App) -> bool {
    app.state()
        .where_the_view_shows(glam::DVec3::ZERO)
        .is_some()
}

#[test]
fn a_click_on_a_recent_part_s_line_opens_it() {
    let opened_at = at("2026-11-05T15:32:00Z");
    let mut app = driver::open_remembering_parts(
        "a_click_on_a_recent_part_s_line_opens_it",
        vec![driver::RecentPart {
            lies_at: "Support.caopart".into(),
            part: two_sketches_and_an_extrusion("Support"),
            opened_at,
        }],
    );
    driver::open_the_recent_part(&mut app, "Support");
    assert!(
        a_part_is_open(&app),
        "a click on the recent part's name does not open it, showing {:?}",
        driver::on_screen(&app),
    );
    driver::click(&mut app, "Accueil");
    assert!(!a_part_is_open(&app), "back home, a part is still drawn");
    let chevron = where_on_the_line_of(&app, "Support", "›")
        .expect("the recent line of « Support » ends on a chevron");

    driver::click_at(&mut app, chevron.center());

    assert!(
        a_part_is_open(&app),
        "a click on the chevron of « Support »'s recent line, away from its name, does not \
         open it, showing {:?}",
        driver::on_screen(&app),
    );
}

/// Every text standing on the row of the « Pièces récentes » title, right of
/// it.
fn right_of_the_title(app: &driver::App) -> Vec<String> {
    let title = driver::where_it_reads(app, "Pièces récentes").unwrap_or_else(|| {
        panic!(
            "the recent parts are not titled « Pièces récentes », showing {:?}",
            driver::on_screen(app),
        )
    });
    app.get_all_by(|_| true)
        .filter_map(|node| {
            let read = node.accesskit_node();
            let text = read.label().or_else(|| read.value())?;
            let at = node.rect();
            let on_its_row =
                title.top() - 4.0 <= at.center().y && at.center().y <= title.bottom() + 4.0;
            (on_its_row && at.left() >= title.right() - 1.0).then_some(text)
        })
        .collect()
}

#[test]
fn the_recent_parts_title_counts_the_lines_shown_on_its_right() {
    let opened_at = at("2026-11-05T15:32:00Z");
    let remembered = |name: &str| driver::RecentPart {
        lies_at: format!("{name}.caopart").into(),
        part: PartDocument::new(name, opened_at),
        opened_at,
    };
    let mut wrong = Vec::new();
    for (test, parts, count) in [
        (
            "the_recent_parts_title_counts_two_lines",
            vec![remembered("Équerre"), remembered("Support")],
            vec!["2 pièces"],
        ),
        (
            "the_recent_parts_title_counts_one_line",
            vec![remembered("Support")],
            vec!["1 pièce"],
        ),
        (
            "the_recent_parts_title_counts_no_line",
            Vec::new(),
            Vec::new(),
        ),
    ] {
        let lines = parts.len();
        let app = driver::open_remembering_parts(test, parts);
        let read = right_of_the_title(&app);
        if read != count {
            wrong.push(format!(
                "lines shown: {lines}, read {read:?} where {count:?} was wanted"
            ));
        }
    }

    assert!(
        wrong.is_empty(),
        "« Pièces récentes » does not count the lines it shows on its right — none when it \
         shows none, as the empty frame of the mockup carries no count:\n  {}",
        wrong.join("\n  "),
    );
}

/// What a frame painted: each icon by the file it was drawn from — `part` for
/// `bytes://icon/part.svg` — and each text by what it says.
struct Painted {
    icons: Vec<String>,
    texts: Vec<String>,
}

/// What the last frame painted inside `within`.
fn painted_within(app: &driver::App, within: egui::Rect) -> Painted {
    let textures = app.ctx.tex_manager();
    let textures = textures.read();
    let (mut icons, mut texts) = (Vec::new(), Vec::new());
    let mut pending: Vec<&egui::Shape> = app
        .output()
        .shapes
        .iter()
        .map(|clipped| &clipped.shape)
        .collect();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            egui::Shape::Rect(rect)
                if rect.fill_texture_id() != egui::TextureId::default()
                    && within.contains(rect.rect.center()) =>
            {
                let loaded_as = textures
                    .meta(rect.fill_texture_id())
                    .map(|meta| meta.name.clone())
                    .unwrap_or_default();
                let file = loaded_as.rsplit('/').next().unwrap_or(&loaded_as);
                icons.push(file.strip_suffix(".svg").unwrap_or(file).to_owned());
            }
            egui::Shape::Text(text) if within.contains(shape.visual_bounding_rect().center()) => {
                texts.push(text.galley.text().to_owned());
            }
            _ => {}
        }
    }
    Painted { icons, texts }
}

#[test]
fn with_no_recent_part_the_card_says_none_was_opened_and_how_to_make_one() {
    let mut wearing = driver::open("with_no_recent_part_the_part_icon_is_seen_elsewhere");
    driver::create_a_part(&mut wearing, "Platine");
    assert!(
        painted_within(&wearing, egui::Rect::EVERYTHING)
            .icons
            .iter()
            .any(|icon| icon == "part"),
        "the frame is read for no `part` icon even where the part's « Pièce » toggle wears \
         it: what follows could not tell an icon missing from one unseen",
    );

    let app = driver::open_remembering_parts(
        "with_no_recent_part_the_card_says_none_was_opened",
        Vec::new(),
    );
    let title = driver::where_it_reads(&app, "Pièces récentes").unwrap_or_else(|| {
        panic!(
            "the recent parts are not titled « Pièces récentes », showing {:?}",
            driver::on_screen(&app),
        )
    });
    let the_card = egui::Rect::from_min_max(
        egui::pos2(title.left() - 24.0, title.bottom()),
        egui::pos2(f32::INFINITY, f32::INFINITY),
    );
    let in_it = painted_within(&app, the_card);
    let anywhere = painted_within(&app, egui::Rect::EVERYTHING);

    let mut wrong = Vec::new();
    for sentence in [
        "Aucune pièce ouverte pour le moment.",
        "Donnez un nom à gauche, puis Créer.",
    ] {
        let in_the_card = in_it.texts.iter().filter(|text| *text == sentence).count();
        let on_screen = anywhere
            .texts
            .iter()
            .filter(|text| *text == sentence)
            .count();
        if (in_the_card, on_screen) != (1, 1) {
            wrong.push(format!(
                "« {sentence} » is painted {on_screen} times, {in_the_card} of them under the title"
            ));
        }
        match driver::where_it_reads(&app, sentence) {
            Some(at) if the_card.contains(at.center()) => {}
            at => wrong.push(format!(
                "« {sentence} » is no text under the title for whoever cannot see the screen: \
                 read at {at:?}"
            )),
        }
    }
    if !in_it.icons.iter().any(|icon| icon == "part") {
        wrong.push(format!(
            "no `part` icon under the title, only {:?}",
            in_it.icons
        ));
    }

    assert!(
        wrong.is_empty(),
        "with no recent part, the card under « Pièces récentes » does not say, once each, \
         that none was opened and how to make one, beside the part icon:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn a_recent_part_that_does_not_open_still_says_what_it_holds() {
    let test = "a_recent_part_that_does_not_open";
    let opened_at = at("2026-11-05T15:32:00Z");
    let mut older = two_sketches_and_an_extrusion("Support");
    older.metadata.schema_version = cao_part::SCHEMA_VERSION - 1;
    let app = driver::open_remembering_parts(
        test,
        vec![driver::RecentPart {
            lies_at: "Support.caopart".into(),
            part: older,
            opened_at,
        }],
    );
    let laid = cao_prefs::default_projects_dir(&driver::locations_of(test)).join("Support.caopart");
    assert!(
        PartDocument::load(&DiskFiles, &laid).is_err(),
        "the part laid down is one the application refuses to open: a line reading what it \
         holds can then have built no part to read it",
    );

    assert_eq!(
        the_detail_of(&app, "Support").as_deref(),
        Some("Dossier des pièces · 2 esquisses · 1 extrusion"),
    );
}

#[test]
fn the_header_reads_cao_above_today_s_subtitle() {
    let app = driver::open_remembering_parts("the_header_reads_cao", Vec::new());
    let read = |text: &str| {
        driver::where_it_reads(&app, text).unwrap_or_else(|| {
            panic!(
                "the home screen does not read « {text} », showing {:?}",
                driver::on_screen(&app),
            )
        })
    };
    let cao = read("CAO");
    let subtitle = read("Nouvelle pièce ou reprise d'une pièce récente");

    assert!(
        cao.bottom() <= subtitle.top(),
        "« CAO » at {cao:?} does not stand above its subtitle, at {subtitle:?}",
    );
    assert!(
        cao.height() >= 2.0 * subtitle.height(),
        "« CAO » is {} points tall and its subtitle {}: the title is not large",
        cao.height(),
        subtitle.height(),
    );
}

#[test]
fn a_recent_part_moved_or_deleted_outside_the_app_still_shows_and_says_its_file_is_gone() {
    let test = "a_recent_part_moved_or_deleted_outside_the_app";
    let opened_at = at("2026-11-05T15:32:00Z");
    let remembered = |name: &str| driver::RecentPart {
        lies_at: format!("{name}.caopart").into(),
        part: PartDocument::new(name, opened_at),
        opened_at,
    };
    let mut app =
        driver::open_remembering_parts(test, vec![remembered("Équerre"), remembered("Support")]);
    let library = cao_prefs::default_projects_dir(&driver::locations_of(test));
    let (deleted, moved) = (
        library.join("Équerre.caopart"),
        library.join("Support.caopart"),
    );
    std::fs::remove_file(&deleted).expect("the part is deleted outside the application");
    std::fs::create_dir_all(library.join("Brides")).expect("a folder to move the part to");
    std::fs::rename(&moved, library.join("Brides/Support.caopart"))
        .expect("the part is moved outside the application");
    app.run();

    let mut wrong = Vec::new();
    for (name, gone) in [("Équerre", &deleted), ("Support", &moved)] {
        if driver::among_the_recent_parts(&app, name).is_none() {
            wrong.push(format!("« {name} » is gone from the recent parts"));
            continue;
        }
        driver::open_the_recent_part(&mut app, name);
        let error = format!("Aucun fichier à {}.", gone.display());
        if !driver::reads(&app, &error) {
            wrong.push(format!(
                "a click on « {name} » does not say « {error} », showing {:?}",
                driver::on_screen(&app),
            ));
        }
    }
    let again = driver::open_again(test);
    for name in ["Équerre", "Support"] {
        if driver::among_the_recent_parts(&again, name).is_some() {
            wrong.push(format!(
                "started again, the recent parts still list « {name} », whose file is gone"
            ));
        }
    }

    assert!(
        wrong.is_empty(),
        "a recent part moved or deleted outside the application does not behave as it \
         did: still listed until the next start, saying its file is gone when clicked:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn a_recent_part_s_line_is_one_button_named_after_the_part_and_pressing_it_opens_it() {
    let mut app = driver::open_remembering_parts(
        "a_recent_part_s_line_is_one_button",
        vec![driver::RecentPart {
            lies_at: "Support.caopart".into(),
            part: two_sketches_and_an_extrusion("Support"),
            opened_at: at("2026-11-05T15:32:00Z"),
        }],
    );
    let recents = driver::where_it_reads(&app, "Pièces récentes")
        .expect("the recent parts are titled")
        .left()
        - 24.0;
    let named = |node: &egui_kittest::Node<'_>| {
        node.accesskit_node().label().as_deref() == Some("Support") && node.rect().left() >= recents
    };
    let buttons: Vec<egui::Rect> = app
        .query_all_by_role(egui::accesskit::Role::Button)
        .filter(named)
        .map(|node| node.rect())
        .collect();
    let chevron = where_on_the_line_of(&app, "Support", "›")
        .expect("the recent line of « Support » ends on a chevron");
    assert!(
        matches!(buttons[..], [line] if line.contains(chevron.center())),
        "the recent line of « Support » is not one button named « Support » reaching its \
         chevron, for whoever cannot see it: buttons so named {buttons:?}, chevron at \
         {chevron:?}",
    );

    app.query_all_by_role(egui::accesskit::Role::Button)
        .find(named)
        .expect("the line's button")
        .click();
    app.run();

    assert!(
        a_part_is_open(&app),
        "pressing the button of « Support »'s recent line, as assistive technology does, \
         opens nothing, showing {:?}",
        driver::on_screen(&app),
    );
}

/// Every text standing on the recent parts' line that reads this name.
fn the_line_of(app: &driver::App, name: &str) -> Vec<String> {
    let line = the_band_of(app, name);
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

fn a_drawn_picture() -> Picture {
    Picture::new(4, 3, vec![200; 4 * 3 * 4]).expect("the pixels fill the size")
}

/// Whether an image stands left of this text, from `from_x` on, as high as a
/// line beginning at the text reaches: where a thumbnail goes.
fn an_image_left_of(app: &driver::App, text: egui::Rect, from_x: f32) -> bool {
    let thumbnail = egui::Rect::from_min_max(
        egui::pos2(from_x, text.top() - 12.0),
        egui::pos2(text.left(), text.bottom() + 40.0),
    );
    app.query_all_by_role(egui::accesskit::Role::Image)
        .any(|node| thumbnail.contains(node.rect().center()))
}

#[test]
fn a_recent_part_shows_the_picture_it_was_put_away_with() {
    let test = "a_recent_part_shows_the_picture";
    let opened_at = at("2026-11-05T15:32:00Z");
    let mut pictured = PartDocument::new("Support", opened_at);
    pictured.set_picture(a_drawn_picture());
    let app = driver::open_remembering_parts(
        test,
        vec![
            driver::RecentPart {
                lies_at: "Équerre.caopart".into(),
                part: PartDocument::new("Équerre", opened_at),
                opened_at,
            },
            driver::RecentPart {
                lies_at: "Support.caopart".into(),
                part: pictured,
                opened_at,
            },
        ],
    );
    let laid = cao_prefs::default_projects_dir(&driver::locations_of(test)).join("Support.caopart");
    assert_eq!(
        PartDocument::picture_in(&DiskFiles, &laid),
        Some(a_drawn_picture()),
        "the part laid down carries its picture",
    );
    let row = *driver::rows_of_the_file_list_reading(&app, "Support")
        .first()
        .expect("the part laid down is in the library's column");
    assert!(
        an_image_left_of(&app, row, 0.0),
        "the library's column shows no image beside « Support »: the accessibility tree \
         does not carry the picture, and what follows could not tell",
    );

    let recents = driver::where_it_reads(&app, "Pièces récentes")
        .expect("the recent parts are titled")
        .left()
        - 24.0;
    let shown =
        |name: &str| an_image_left_of(&app, driver::the_recent_part_named(&app, name), recents);

    assert_eq!(
        (shown("Support"), shown("Équerre")),
        (true, false),
        "(the recent part put away with a picture shows it, the one put away with none \
         shows no image), as the library's column does",
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
