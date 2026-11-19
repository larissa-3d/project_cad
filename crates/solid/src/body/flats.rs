//! A body made of flat pieces, as the flats kernel leaves it.

use serde::{Deserialize, Serialize};

use super::drawn::corner_normals::CornerNormals;
use crate::mesh::Mesh;

/// Flat pieces, and the number the next face made is given: every number a
/// step named stands below it, kept or not, as in an exact body, so that a
/// part going to the flats never gives a number twice.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct Flats {
    pub(super) mesh: Mesh,
    pub(super) next: usize,
    #[serde(skip)]
    pub(super) normals: CornerNormals,
}

impl Flats {
    pub(super) fn new(mesh: Mesh, next: usize) -> Flats {
        Flats {
            mesh,
            next,
            normals: CornerNormals::default(),
        }
    }
}
