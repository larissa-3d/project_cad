//! The picture a part carries, as the archive holds it: how big it is in one
//! entry, its pixels in another.

use std::io::{Read, Seek, Write};

use serde::{Deserialize, Serialize};

use crate::errors::PartFileError;
use crate::picture::Picture;

/// How big the picture is. Kept apart from its bytes so that neither entry has
/// to carry a header the other could contradict.
const SHAPE_ENTRY: &str = "picture.json";
const PIXELS_ENTRY: &str = "picture.rgba";

/// How big a picture is, beside the bytes that fill it.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct PictureShape {
    width: u32,
    height: u32,
}

pub(super) fn write<W: Write + Seek>(
    archive: &mut zip::ZipWriter<W>,
    options: zip::write::SimpleFileOptions,
    picture: &Picture,
) -> Result<(), PartFileError> {
    let shape = PictureShape {
        width: picture.width(),
        height: picture.height(),
    };
    archive.start_file(SHAPE_ENTRY, options)?;
    archive
        .write_all(serde_json::to_string(&shape)?.as_bytes())
        .map_err(zip::result::ZipError::from)?;
    archive.start_file(PIXELS_ENTRY, options)?;
    archive
        .write_all(picture.pixels())
        .map_err(zip::result::ZipError::from)?;
    Ok(())
}

/// The picture, when the archive holds a whole one.
///
/// A part written before pictures existed carries neither entry, and a part
/// whose picture is damaged is a part with no picture — never a part that
/// refuses to open. Nothing of the drawing is in there.
pub(super) fn read<R: Read + Seek>(archive: &mut zip::ZipArchive<R>) -> Option<Picture> {
    let shape: PictureShape =
        serde_json::from_str(&super::read_entry(archive, SHAPE_ENTRY).ok()?).ok()?;
    let mut pixels = Vec::new();
    archive
        .by_name(PIXELS_ENTRY)
        .ok()?
        .read_to_end(&mut pixels)
        .ok()?;
    Picture::new(shape.width, shape.height, pixels)
}
