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

/// The folder a part lies in, said from inside the parts folder: « Brides /
/// Anciennes », the root as the parts folder, and a folder outside it by its
/// path as it stands.
pub fn where_a_part_lies(lang: &Catalogue, root: &Path, folder: &Path) -> String {
    let Ok(below) = folder.strip_prefix(root) else {
        return folder.display().to_string();
    };
    if below.as_os_str().is_empty() {
        return parts_folder(lang);
    }
    below
        .iter()
        .map(|name| name.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(" / ")
}
