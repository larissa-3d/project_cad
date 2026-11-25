use std::path::{Path, PathBuf};

use cao_part::library::{self, Folder};

use crate::lang::Catalogue;
use crate::ui::badge::badge;
use crate::ui::dropping::{carry, take_a_dropped_path};
use crate::ui::text_edit::text_edit;
use crate::ui::theme::tokens;
use crate::wording;

use super::state::{Explorer, Naming};

/// What the panel asks of whoever put it on screen. The panel itself holds no
/// port: the shell decides what a file system is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExplorerAction {
    None,
    /// Open this part.
    Open(PathBuf),
    /// Ask for the name of a folder that does not exist yet.
    NewFolder,
    /// Ask for another name for what is selected.
    Rename,
    /// Write down the name being typed.
    ConfirmNaming,
    /// Throw away what the question was about.
    Discard,
    /// Read the library off the disk again.
    Refresh,
    /// Move this part into this folder.
    DropOn {
        part: PathBuf,
        folder: PathBuf,
    },
}

/// Where each name typed in the panel is given room to be read.
const NAME_WIDTH: f32 = 150.0;

/// What the user did to the tree this frame, gathered while the panel is only
/// being read so that acting on it comes afterwards.
#[derive(Default)]
struct Touched {
    select: Option<PathBuf>,
    open: Option<PathBuf>,
    fold: Option<(PathBuf, bool)>,
    /// A part let go over a folder: the part, then the folder.
    dropped: Option<(PathBuf, PathBuf)>,
    /// The parts a row was drawn for this frame. Their pictures are read after
    /// the walk, so only what is on screen is ever read off the disk.
    shown: Vec<PathBuf>,
}

/// The panel the library is browsed in, chrome and all.
pub fn panel(ui: &mut egui::Ui, explorer: &mut Explorer, lang: &Catalogue) -> ExplorerAction {
    egui::Panel::left("explorer_panel")
        .resizable(true)
        .default_size(300.0)
        .size_range(280.0..=320.0)
        .show(ui, |ui| show(ui, explorer, lang))
        .inner
}

fn show(ui: &mut egui::Ui, explorer: &mut Explorer, lang: &Catalogue) -> ExplorerAction {
    let mut action = ExplorerAction::None;
    let mut touched = Touched::default();

    ui.horizontal(|ui| {
        ui.heading(lang.t("explorer.title"));
        if ui
            .small_button(lang.t("explorer.refresh"))
            .on_hover_text(lang.t("explorer.refresh_hint"))
            .clicked()
        {
            action = ExplorerAction::Refresh;
        }
    });
    ui.horizontal(|ui| {
        if ui.small_button(lang.t("explorer.new_folder")).clicked() {
            action = ExplorerAction::NewFolder;
        }
        ui.add_enabled_ui(explorer.selected().is_some(), |ui| {
            if ui.small_button(lang.t("explorer.rename")).clicked() {
                action = ExplorerAction::Rename;
            }
            if ui.small_button(lang.t("explorer.discard")).clicked() {
                action = ExplorerAction::Discard;
            }
        });
    });
    let mut query = explorer.query().to_string();
    let width = ui.available_width();
    if text_edit(ui, &mut query, width, &lang.t("explorer.filter_hint")).changed() {
        explorer.set_query(&query);
    }
    // Above the tree, which takes the rest of the column down to its foot.
    if let Some(fate) = explorer.trouble() {
        ui.colored_label(
            tokens::STATE_ERROR_TEXT,
            wording::part_file::say(lang, fate),
        );
    }
    ui.separator();

    // The name being typed is taken out for the length of the walk and put
    // back at the end: that is what lets the tree be read while the field it
    // holds is written to.
    let mut naming = explorer.naming().cloned();
    egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| {
            let reading: &Explorer = explorer;
            let tree = reading.tree();
            contents(ui, reading, tree, &mut naming, &mut touched, lang);
            if holds_nothing(reading.library()) {
                ui.weak(lang.t("explorer.empty"));
            } else if holds_nothing(tree) {
                ui.weak(lang.t("explorer.nothing_found"));
            }
            // What is left of the column below the tree stands for the root,
            // a row high at least once the tree fills the column.
            let height = ui.available_height().max(ui.spacing().interact_size.y);
            let room = egui::vec2(ui.available_width(), height);
            let below = ui.allocate_response(room, egui::Sense::hover());
            if let Some(part) = take_a_dropped_path(ui, &below) {
                touched.dropped = Some((part, tree.path.clone()));
            }
        });

    explorer.set_naming(naming);
    if let Some((path, open)) = touched.fold {
        explorer.fold(&path, open);
    }
    if let Some(path) = touched.select {
        explorer.select(&path);
    }
    if let Some(path) = touched.open {
        action = ExplorerAction::Open(path);
    }
    if let Some((part, folder)) = touched.dropped {
        action = ExplorerAction::DropOn { part, folder };
    }
    explorer.wants_pictures_of(touched.shown);
    if explorer.naming().is_some() {
        if ui.input(|input| input.key_pressed(egui::Key::Enter)) {
            action = ExplorerAction::ConfirmNaming;
        } else if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
            explorer.cancel_naming();
        }
    }
    action
}

/// One folder's worth of rows, its subfolders before its parts.
fn contents(
    ui: &mut egui::Ui,
    explorer: &Explorer,
    folder: &Folder,
    naming: &mut Option<Naming>,
    touched: &mut Touched,
    lang: &Catalogue,
) {
    for under in &folder.folders {
        if being_renamed(naming, &under.path) {
            field(ui, naming, lang);
            continue;
        }
        let unfolded = explorer.unfolded(&under.path);
        let row = ui.horizontal(|ui| {
            let arrow = lang.t(if unfolded {
                "explorer.unfolded"
            } else {
                "explorer.folded"
            });
            if ui.small_button(arrow).clicked() {
                touched.fold = Some((under.path.clone(), !unfolded));
            }
            if ui
                .selectable_label(explorer.selected() == Some(&under.path), &under.name)
                .clicked()
            {
                touched.select = Some(under.path.clone());
            }
            let count = under.count_of_parts().to_string();
            badge(
                ui,
                &count,
                (tokens::BACKGROUND_ACTIVE_NEUTRAL, tokens::TEXT_SUBDUED),
            );
        });
        if let Some(part) = take_a_dropped_path(ui, &row.response) {
            touched.dropped = Some((part, under.path.clone()));
        }
        if unfolded {
            ui.indent(&under.path, |ui| {
                contents(ui, explorer, under, naming, touched, lang);
            });
        }
    }

    for part in &folder.parts {
        if being_renamed(naming, &part.path) {
            field(ui, naming, lang);
            continue;
        }
        touched.shown.push(part.path.clone());
        let named = ui
            .horizontal(|ui| {
                thumbnail(ui, explorer.picture_of(&part.path));
                ui.selectable_label(explorer.selected() == Some(&part.path), &part.name)
            })
            .inner;
        let response = carry(named, &part.path).on_hover_text(lang.t("explorer.open_hint"));
        if response.clicked() {
            touched.select = Some(part.path.clone());
        }
        if response.double_clicked() {
            touched.open = Some(part.path.clone());
        }
    }

    if let Some(Naming::NewFolder { inside, .. }) = naming
        && inside == &folder.path
    {
        field(ui, naming, lang);
    }
}

/// How wide and how high a part's picture is drawn beside its name.
const THUMBNAIL: egui::Vec2 = egui::vec2(22.0, 18.0);

/// The part's picture, or the room it will take once it has one.
///
/// A quiet frame rather than nothing, so a folder half of whose parts have
/// been put away since does not read as two columns.
fn thumbnail(ui: &mut egui::Ui, picture: Option<&cao_part::Picture>) {
    let Some(picture) = picture else {
        let (rect, _) = ui.allocate_exact_size(THUMBNAIL, egui::Sense::hover());
        ui.painter()
            .rect_filled(rect, 2.0, ui.visuals().faint_bg_color);
        return;
    };
    ui.add(egui::Image::new(&texture(ui, picture)).fit_to_exact_size(THUMBNAIL));
}

/// The part's pixels as something the GPU can draw, kept in egui's own store
/// between frames: uploading a texture every frame for every row on screen is
/// the one way to make a picture cost more than the drawing it stands for.
fn texture(ui: &egui::Ui, picture: &cao_part::Picture) -> egui::TextureHandle {
    let id = egui::Id::new(("explorer_thumbnail", picture.pixels().as_ptr() as usize));
    if let Some(held) = ui.data(|data| data.get_temp::<egui::TextureHandle>(id)) {
        return held;
    }
    let image = egui::ColorImage::from_rgba_unmultiplied(
        [picture.width() as usize, picture.height() as usize],
        picture.pixels(),
    );
    let made = ui
        .ctx()
        .load_texture("explorer_thumbnail", image, egui::TextureOptions::LINEAR);
    ui.data_mut(|data| data.insert_temp(id, made.clone()));
    made
}

fn holds_nothing(folder: &Folder) -> bool {
    folder.folders.is_empty() && folder.parts.is_empty()
}

fn being_renamed(naming: &Option<Naming>, path: &Path) -> bool {
    matches!(naming, Some(Naming::Rename { path: at, .. }) if at == path)
}

fn field(ui: &mut egui::Ui, naming: &mut Option<Naming>, lang: &Catalogue) {
    let Some(naming) = naming else { return };
    let response = text_edit(ui, naming.typed_mut(), NAME_WIDTH, &lang.t("explorer.name"));
    // Only when nothing else has the keyboard: asking every frame takes focus
    // back the frame after the user clicks anywhere else, and the field can
    // then only be left by Entrée or Échap.
    if ui.memory(|memory| memory.focused().is_none()) {
        response.request_focus();
    }
}

/// The question asked before a part or a folder is thrown away.
///
/// `Some(true)` once confirmed, `Some(false)` once backed out of, `None` while
/// the user has not answered.
pub fn discard_confirm(ui: &mut egui::Ui, path: &Path, lang: &Catalogue) -> Option<bool> {
    let mut decision = None;
    let modal = egui::Modal::new(egui::Id::new("explorer_discard_confirm")).show(ui.ctx(), |ui| {
        ui.heading(lang.t("explorer.discard_confirm_title"));
        ui.label(lang.t_with(
            "explorer.discard_confirm_body",
            &[("name", &library::name_of(path))],
        ));
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.button(lang.t("explorer.discard_confirm_ok")).clicked() {
                decision = Some(true);
            }
            if ui
                .button(lang.t("explorer.discard_confirm_cancel"))
                .clicked()
            {
                decision = Some(false);
            }
        });
    });
    if decision.is_none() && modal.should_close() {
        decision = Some(false);
    }
    decision
}
