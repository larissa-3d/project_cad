//! How a place in the library is said.

use std::path::Path;

use crate::lang::Catalogue;

/// The root of the library.
pub fn parts_folder(lang: &Catalogue) -> String {
    lang.t("library.parts_folder")
}

/// A folder of the library, from the parts folder down to it with each folder
/// on the way named: « Dossier des pièces / Brides ».
pub fn folder(lang: &Catalogue, root: &Path, folder: &Path) -> String {
    let below = folder.strip_prefix(root).unwrap_or(Path::new(""));
    std::iter::once(parts_folder(lang))
        .chain(below.iter().map(|name| name.to_string_lossy().into_owned()))
        .collect::<Vec<_>>()
        .join(" / ")
}
