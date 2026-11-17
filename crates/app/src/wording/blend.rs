//! What the edge panel says of the edges chosen, and of the size taken.

use cao_part::Reduced;

use crate::lang::Catalogue;
use crate::wording::dimension;

/// How many edges the panel holds, in the singular for one.
pub fn edges_chosen(lang: &Catalogue, count: usize) -> String {
    let key = match count {
        1 => "blend.edges_chosen_one",
        _ => "blend.edges_chosen_many",
    };
    lang.t_with(key, &[("count", &count.to_string())])
}

/// « 15 mm demandé, 10 mm appliqué »: the size asked and the one the faces
/// along the edges gave.
pub fn reduced(lang: &Catalogue, reduced: Reduced) -> String {
    lang.t_with(
        "part_tree.history.blend_reduced",
        &[
            ("asked", &dimension::short(reduced.asked)),
            ("taken", &dimension::short(reduced.taken)),
        ],
    )
}
