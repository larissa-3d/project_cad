//! The « Nouveau » card of the home screen, driven whole: where it saves the
//! part it makes, the folders « Choisir… » offers, the name it gives, and the
//! type it makes (`docs/redesign/home-screen.md` A2).
//!
//! Closes #744.
//! - The default folder of the card is the one chosen in « Toutes les pièces »,
//!   else the root of the parts folder —
//!   `the_card_saves_in_the_folder_chosen_in_the_column_else_at_the_root`
//! - « Choisir… » opens a popover listing the folders inside the parts folder
//!   as a flat list, each indented by its depth, and offers no folder outside
//!   it — `choosing_lists_the_folders_inside_the_parts_folder_each_indented_by_its_depth`
//! - Creating a part puts it in the folder shown in « Enregistrer dans », and
//!   the home screen lists it under that folder —
//!   `a_part_created_goes_in_the_folder_shown_and_the_column_lists_it_under_that_folder`
//! - With no folder chosen, a name and ↵ still create the part at the root —
//!   `with_no_folder_chosen_a_name_and_enter_create_the_part_at_the_root`
//! - A name already taken in the target folder is not written over: the part
//!   takes the next free name (« Support 2 »), as today; a name taken only in
//!   another folder is kept as typed —
//!   `a_name_taken_in_the_target_folder_takes_the_next_free_name` for the
//!   first half, `a_name_taken_only_in_another_folder_is_kept_as_typed` for
//!   the second
//! - « Assemblage » is disabled and creates nothing —
//!   `the_assembly_segment_is_disabled_and_creates_nothing`
//! - no test: the « bientôt » tag, the 40 px field and the full-width
//!   « Créer ↵ » are looked at in « To try it ».
//!
//! Readings settled while the tests were written, the issue left as it is: a
//! part chosen in the column stands for its folder; the last of a click in the
//! column and a pick in « Choisir… » is what the card shows —
//! `a_folder_clicked_in_the_column_after_a_pick_replaces_the_pick`; the popover opens
//! on the root itself, « Dossier des pièces », then the folders by depth; the
//! home screen listing the part means the column's tree; and ↵ in the name
//! field making the part is new, not kept from before.

mod driver;

use std::path::{Path, PathBuf};

use egui_kittest::kittest::{NodeT, Queryable};

/// How « Enregistrer dans » names the root of the parts folder, and what every
/// folder under it is shown after.
const THE_PARTS_FOLDER: &str = "Dossier des pièces";

/// Every text a node carries, its name and its value both: a field named after
/// the label above it holds what it shows as its value.
fn texts_of(node: &egui_kittest::Node<'_>) -> Vec<String> {
    let read = node.accesskit_node();
    read.label().into_iter().chain(read.value()).collect()
}

/// How far right of the end of « Enregistrer dans : » its field may start: the
/// gap a row leaves between a label and the field it names, were the two laid
/// side by side rather than one under the other.
const THE_GAP_BESIDE_A_LABEL: f32 = 16.0;

/// What the card's « Enregistrer dans » shows, read from « Dossier des pièces »
/// on so that a folder icon drawn before it is left out.
///
/// Of the texts naming the parts folder, the one nearest the label among those
/// standing under it or beside it and starting no further right than it ends:
/// a list « Choisir… » opens beside the field names the parts folder too, and
/// is not what the card will save in. A node's place is read only once its
/// text names the parts folder: a container drawn around widgets has no place,
/// and asking for one panics.
fn where_the_card_saves(app: &driver::App) -> Option<String> {
    let label = app
        .get_all_by(|_| true)
        .find(|node| {
            texts_of(node)
                .iter()
                .any(|text| text.starts_with("Enregistrer dans"))
        })?
        .rect();
    app.get_all_by(|_| true)
        .flat_map(|node| {
            let shown: Vec<String> = texts_of(&node)
                .into_iter()
                .filter_map(|text| Some(text[text.find(THE_PARTS_FOLDER)?..].trim().to_string()))
                .collect();
            if shown.is_empty() {
                return Vec::new();
            }
            let at = node.rect();
            shown.into_iter().map(|text| (text, at)).collect()
        })
        .filter(|(_, at)| {
            at.bottom() >= label.center().y && at.left() <= label.right() + THE_GAP_BESIDE_A_LABEL
        })
        .min_by(|(_, one), (_, other)| {
            let from_the_label = |at: &egui::Rect| label.center().distance(at.center());
            from_the_label(one).total_cmp(&from_the_label(other))
        })
        .map(|(shown, _)| shown)
}

/// Makes these folders in the library, then reads the library again with
/// « ↺ », as a user who had made them by hand would. Says where the library
/// is.
fn folders_laid_by_hand(app: &mut driver::App, test: &str, folders: &[&str]) -> PathBuf {
    let library = cao_prefs::default_projects_dir(&driver::locations_of(test));
    for folder in folders {
        std::fs::create_dir_all(library.join(folder)).expect("the folder is made");
    }
    driver::click(app, "↺");
    library
}

/// What a part laid by hand holds: no part the application writes reads this,
/// so finding it still there says nothing was written over it.
const LAID_BY_HAND: &[u8] = b"PK, laid by hand";

/// Lays these part files in the library, then reads the library again with
/// « ↺ », as a user who had put them there by hand would. Says where the
/// library is.
fn parts_laid_by_hand(app: &mut driver::App, test: &str, parts: &[&str]) -> PathBuf {
    let library = cao_prefs::default_projects_dir(&driver::locations_of(test));
    for part in parts {
        let path = library.join(part);
        std::fs::create_dir_all(path.parent().expect("a folder")).expect("the folder is made");
        std::fs::write(&path, LAID_BY_HAND).expect("the part file is written");
    }
    driver::click(app, "↺");
    library
}

/// Whether the part laid by hand there still holds what it was laid with.
fn still_as_laid(part: &Path) -> bool {
    std::fs::read(part).is_ok_and(|held| held == LAID_BY_HAND)
}

/// Clicks this folder's row in the column, and makes sure the card took it.
fn choose_in_the_column(app: &mut driver::App, folder: &str) {
    let row = the_row_reading(app, folder);
    driver::click_at(app, row.center());
    assert_eq!(
        where_the_card_saves(app),
        Some(format!("{THE_PARTS_FOLDER} / {folder}")),
        "the card does not show « {folder} » once it is chosen, showing {:?}",
        driver::on_screen(app),
    );
}

/// The segment of « Type » reading this word: its name holds the word whole,
/// whatever icon is drawn before it, and « Pièces récentes » is not « Pièce ».
fn the_type<'tree>(app: &'tree driver::App, word: &str) -> egui_kittest::Node<'tree> {
    app.get_all_by(|_| true)
        .find(|node| {
            node.accesskit_node()
                .label()
                .is_some_and(|name| name.split_whitespace().any(|read| read == word))
        })
        .unwrap_or_else(|| {
            panic!(
                "no « {word} » on the card, showing {:?}",
                driver::on_screen(app),
            )
        })
}

fn is_chosen(node: &egui_kittest::Node<'_>) -> bool {
    node.accesskit_node().toggled() == Some(egui::accesskit::Toggled::True)
}

fn a_part_is_open(app: &driver::App) -> bool {
    app.state()
        .where_the_view_shows(glam::DVec3::ZERO)
        .is_some()
}

/// Every text on screen with where it stands. Only a node the tree gives a
/// place is read: a container drawn around widgets has none.
fn texts_and_places(app: &driver::App) -> Vec<(String, egui::Rect)> {
    app.get_all_by(|_| true)
        .filter(|node| node.accesskit_node().bounding_box().is_some())
        .flat_map(|node| {
            let at = node.rect();
            texts_of(&node).into_iter().map(move |text| (text, at))
        })
        .collect()
}

/// Clicks « Choisir… » and says what it opened: every text on screen after
/// the click that was not there before, where it stands. The field of
/// « Enregistrer dans » and the column's rows read the same names, and are
/// already there.
fn open_the_choice(app: &mut driver::App) -> Vec<(String, egui::Rect)> {
    assert!(
        driver::shows(app, "Choisir"),
        "no « Choisir… » on the card, showing {:?}",
        driver::on_screen(app),
    );
    let before = texts_and_places(app);
    driver::click(app, "Choisir");
    texts_and_places(app)
        .into_iter()
        .filter(|text| !before.contains(text))
        .collect()
}

/// Where the entries reading this name stand: the name whole, after whatever
/// icon or indentation is drawn before it.
fn entries_reading(opened: &[(String, egui::Rect)], name: &str) -> Vec<egui::Rect> {
    opened
        .iter()
        .filter(|(text, _)| {
            let text = text.trim_end();
            text == name || text.ends_with(&format!(" {name}"))
        })
        .map(|(_, at)| *at)
        .collect()
}

/// Where the one entry reading this name stands.
fn the_entry(opened: &[(String, egui::Rect)], name: &str) -> egui::Rect {
    match entries_reading(opened, name)[..] {
        [at] => at,
        ref found => panic!(
            "« Choisir… » offers « {name} » {} times, opening {opened:?}",
            found.len(),
        ),
    }
}

/// How many files stand in this folder itself, not under it, named this before
/// their extension.
fn files_named(folder: &Path, name: &str) -> usize {
    std::fs::read_dir(folder)
        .map(|entries| {
            entries
                .flatten()
                .map(|entry| entry.path())
                .filter(|path| path.is_file() && path.file_stem().is_some_and(|stem| stem == name))
                .count()
        })
        .unwrap_or(0)
}

/// Where the column draws the one row reading this.
fn the_row_reading(app: &driver::App, name: &str) -> egui::Rect {
    *driver::rows_of_the_file_list_reading(app, name)
        .first()
        .unwrap_or_else(|| {
            panic!(
                "no row of the column reads « {name} », showing {:?}",
                driver::on_screen(app),
            )
        })
}

#[test]
fn the_card_saves_in_the_folder_chosen_in_the_column_else_at_the_root() {
    let test = "the_card_saves_in_the_folder_chosen_in_the_column";
    let mut app = driver::open(test);
    folders_laid_by_hand(&mut app, test, &["Brides"]);
    let brides = driver::rows_of_the_file_list_reading(&app, "Brides");
    assert!(
        !brides.is_empty(),
        "the folder laid for the test is not in the column, showing {:?}",
        driver::on_screen(&app),
    );

    assert_eq!(
        where_the_card_saves(&app).as_deref(),
        Some(THE_PARTS_FOLDER),
        "with nothing chosen in « Toutes les pièces », « Enregistrer dans » does not \
         show the root of the parts folder, showing {:?}",
        driver::on_screen(&app),
    );

    driver::click_at(&mut app, brides[0].center());

    assert_eq!(
        where_the_card_saves(&app).as_deref(),
        Some("Dossier des pièces / Brides"),
        "« Brides » chosen in « Toutes les pièces », « Enregistrer dans » does not \
         show it, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn a_part_created_goes_in_the_folder_shown_and_the_column_lists_it_under_that_folder() {
    let test = "a_part_created_goes_in_the_folder_shown";
    let mut app = driver::open(test);
    let library = folders_laid_by_hand(&mut app, test, &["Brides"]);
    driver::create_a_part(&mut app, "Plaque");
    driver::click(&mut app, "Accueil");
    let brides = the_row_reading(&app, "Brides");
    driver::click_at(&mut app, brides.center());
    assert_eq!(
        where_the_card_saves(&app).as_deref(),
        Some("Dossier des pièces / Brides"),
        "the card does not show « Brides » once it is chosen, showing {:?}",
        driver::on_screen(&app),
    );

    driver::create_a_part(&mut app, "Support");
    driver::click(&mut app, "Accueil");

    assert_eq!(
        (
            files_named(&library.join("Brides"), "Support"),
            files_named(&library, "Support"),
        ),
        (1, 0),
        "the part created with « Brides » shown is not in « Brides » alone: \
         (in « Brides », at the root)",
    );
    let support = the_row_reading(&app, "Support");
    let folder = the_row_reading(&app, "Brides");
    let at_the_root = the_row_reading(&app, "Plaque");
    assert!(
        support.top() > folder.top() && support.left() > at_the_root.left(),
        "the column does not draw « Support » under « Brides »: at {support:?}, the \
         folder at {folder:?}, a part at the root at {at_the_root:?}",
    );
}

#[test]
fn with_no_folder_chosen_a_name_and_enter_create_the_part_at_the_root() {
    let test = "with_no_folder_chosen_a_name_and_enter_create_the_part";
    let mut app = driver::open(test);
    let library = folders_laid_by_hand(&mut app, test, &["Brides"]);

    driver::type_beside(&mut app, "Nom :", "Support");
    assert_eq!(
        driver::field_beside(&app, "Nom :"),
        "Support",
        "the name never reached the field beside « Nom : »",
    );
    driver::press(&mut app, egui::Key::Enter);

    assert_eq!(
        (
            files_named(&library, "Support"),
            files_named(&library.join("Brides"), "Support"),
        ),
        (1, 0),
        "« Support » typed with nothing chosen and ↵ pressed is not one part at the \
         root of the parts folder: (at the root, in « Brides »)",
    );
}

#[test]
fn a_name_taken_in_the_target_folder_takes_the_next_free_name() {
    let test = "a_name_taken_in_the_target_folder_takes_the_next_free_name";
    let mut app = driver::open(test);
    let library = parts_laid_by_hand(&mut app, test, &["Brides/Support.caopart"]);
    choose_in_the_column(&mut app, "Brides");

    driver::create_a_part(&mut app, "Support");
    driver::click(&mut app, "Accueil");

    let brides = library.join("Brides");
    assert!(
        still_as_laid(&brides.join("Support.caopart")),
        "the « Support » already in « Brides » was written over",
    );
    assert_eq!(
        (
            files_named(&brides, "Support 2"),
            files_named(&library, "Support"),
        ),
        (1, 0),
        "« Support » created in « Brides », which holds one, is not « Support 2 » \
         there: (« Support 2 » in « Brides », « Support » at the root)",
    );
}

#[test]
fn a_name_taken_only_in_another_folder_is_kept_as_typed() {
    let test = "a_name_taken_only_in_another_folder_is_kept_as_typed";
    let mut app = driver::open(test);
    let library = parts_laid_by_hand(&mut app, test, &["Support.caopart"]);
    folders_laid_by_hand(&mut app, test, &["Brides"]);
    choose_in_the_column(&mut app, "Brides");

    driver::create_a_part(&mut app, "Support");
    driver::click(&mut app, "Accueil");

    let brides = library.join("Brides");
    assert!(
        still_as_laid(&library.join("Support.caopart")),
        "the « Support » at the root was written over",
    );
    assert_eq!(
        (
            files_named(&brides, "Support"),
            files_named(&brides, "Support 2"),
            files_named(&library, "Support 2"),
        ),
        (1, 0, 0),
        "« Support » created in « Brides », taken only at the root, is not kept as \
         typed there: (« Support » in « Brides », « Support 2 » in « Brides », \
         « Support 2 » at the root)",
    );
}

#[test]
fn the_assembly_segment_is_disabled_and_creates_nothing() {
    let test = "the_assembly_segment_is_disabled_and_creates_nothing";
    let mut app = driver::open(test);
    let library = cao_prefs::default_projects_dir(&driver::locations_of(test));
    driver::type_beside(&mut app, "Nom :", "Support");
    assert!(
        is_chosen(&the_type(&app, "Pièce")),
        "« Pièce » is not the type chosen on the card",
    );
    let assembly = the_type(&app, "Assemblage");
    assert!(
        assembly.accesskit_node().is_disabled(),
        "« Assemblage » can be chosen on the card",
    );
    let at = assembly.rect();

    driver::click_at(&mut app, at.center());

    assert!(
        !a_part_is_open(&app) && files_named(&library, "Support") == 0,
        "clicking « Assemblage » with « Support » typed made something: a part open \
         {}, files named « Support » in the parts folder {}",
        a_part_is_open(&app),
        files_named(&library, "Support"),
    );
    assert!(
        is_chosen(&the_type(&app, "Pièce")) && !is_chosen(&the_type(&app, "Assemblage")),
        "clicking « Assemblage » took the card off « Pièce »",
    );
}

#[test]
fn choosing_lists_the_folders_inside_the_parts_folder_each_indented_by_its_depth() {
    let test = "choosing_lists_the_folders_inside_the_parts_folder";
    let mut app = driver::open(test);
    let library = cao_prefs::default_projects_dir(&driver::locations_of(test));
    let outside = library
        .parent()
        .expect("the parts folder has a parent")
        .join("Ailleurs");
    std::fs::create_dir_all(&outside).expect("a folder beside the parts folder");
    parts_laid_by_hand(
        &mut app,
        test,
        &["Plaque.caopart", "Brides/Équerre.caopart"],
    );
    folders_laid_by_hand(&mut app, test, &["Brides/Petites", "Boîtiers"]);

    let opened = open_the_choice(&mut app);

    let root = the_entry(&opened, THE_PARTS_FOLDER);
    let brides = the_entry(&opened, "Brides");
    let petites = the_entry(&opened, "Petites");
    let boitiers = the_entry(&opened, "Boîtiers");
    let not_offered: Vec<&str> = ["Ailleurs", "Plaque", "Équerre"]
        .into_iter()
        .filter(|name| !entries_reading(&opened, name).is_empty())
        .collect();
    assert!(
        not_offered.is_empty(),
        "« Choisir… » offers {not_offered:?}, which are no folders inside the parts \
         folder, opening {opened:?}",
    );
    assert!(
        [brides, petites, boitiers]
            .iter()
            .all(|folder| folder.top() >= root.bottom()),
        "« Choisir… » does not open on the root: at {root:?}, the folders at \
         {brides:?}, {petites:?} and {boitiers:?}",
    );
    assert!(
        root.left() <= brides.left()
            && (brides.left() - boitiers.left()).abs() < 0.5
            && petites.left() > brides.left(),
        "the entries are not indented by their depth: the root at {root:?}, \
         « Brides » at {brides:?} and « Boîtiers » at {boitiers:?} inside it, \
         « Petites » at {petites:?} inside « Brides »",
    );

    driver::click_at(&mut app, petites.center());

    assert_eq!(
        where_the_card_saves(&app).as_deref(),
        Some("Dossier des pièces / Brides / Petites"),
        "« Petites » picked, « Enregistrer dans » does not show it, showing {:?}",
        driver::on_screen(&app),
    );
    driver::create_a_part(&mut app, "Support");
    assert_eq!(
        files_named(&library.join("Brides").join("Petites"), "Support"),
        1,
        "the part created once « Petites » was picked is not in it",
    );
}

#[test]
fn a_folder_clicked_in_the_column_after_a_pick_replaces_the_pick() {
    let test = "a_folder_clicked_in_the_column_after_a_pick_replaces_the_pick";
    let mut app = driver::open(test);
    folders_laid_by_hand(&mut app, test, &["Brides", "Boîtiers"]);
    let opened = open_the_choice(&mut app);
    driver::click_at(&mut app, the_entry(&opened, "Boîtiers").center());
    assert_eq!(
        where_the_card_saves(&app).as_deref(),
        Some("Dossier des pièces / Boîtiers"),
        "« Boîtiers » picked, « Enregistrer dans » does not show it, showing {:?}",
        driver::on_screen(&app),
    );

    let brides = the_row_reading(&app, "Brides");
    driver::click_at(&mut app, brides.center());

    assert_eq!(
        where_the_card_saves(&app).as_deref(),
        Some("Dossier des pièces / Brides"),
        "« Brides » clicked in the column after « Boîtiers » was picked, « Enregistrer \
         dans » does not show it, showing {:?}",
        driver::on_screen(&app),
    );
}
