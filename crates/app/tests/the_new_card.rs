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
//! column and a pick in « Choisir… » is what the card shows; the popover opens
//! on the root itself, « Dossier des pièces », then the folders by depth; the
//! home screen listing the part means the column's tree; and ↵ in the name
//! field making the part is new, not kept from before.

mod driver;

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
/// « ↺ », as a user who had made them by hand would.
fn folders_laid_by_hand(app: &mut driver::App, test: &str, folders: &[&str]) {
    let library = cao_prefs::default_projects_dir(&driver::locations_of(test));
    for folder in folders {
        std::fs::create_dir_all(library.join(folder)).expect("the folder is made");
    }
    driver::click(app, "↺");
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
