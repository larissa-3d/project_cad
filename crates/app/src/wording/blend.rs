//! What the edge panel says of the edges chosen.

use crate::lang::Catalogue;

/// How many edges the panel holds, in the singular for one.
pub fn edges_chosen(lang: &Catalogue, count: usize) -> String {
    let key = match count {
        1 => "blend.edges_chosen_one",
        _ => "blend.edges_chosen_many",
    };
    lang.t_with(key, &[("count", &count.to_string())])
}
