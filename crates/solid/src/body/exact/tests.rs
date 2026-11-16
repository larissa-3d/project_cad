//! How far a face's number is followed back through the roots of parted
//! faces: to the first face, as the kernel writes them, and no further than
//! once round, as a hand may edit them into a cycle.

use std::collections::BTreeMap;

use super::{PIECES, lineage, root};

/// The lineage of `number`, read no further than one number past the longest
/// lineage `roots` could hold without a cycle: one number for each root, and
/// the number itself. A lineage read that long went round.
fn read_as_far_as_it_could_go(roots: &BTreeMap<u32, u32>, number: u32) -> Vec<u32> {
    lineage(roots, number).take(roots.len() + 2).collect()
}

#[test]
fn a_piece_parted_twice_is_followed_back_to_the_first_face_which_is_its_root() {
    let top = 1;
    let roots = BTreeMap::from([(PIECES, top), (PIECES + 1, PIECES)]);
    assert_eq!(
        read_as_far_as_it_could_go(&roots, PIECES + 1),
        [PIECES + 1, PIECES, top],
        "a piece parted from a piece of the top is followed back through that piece to the top"
    );
    assert_eq!(root(&roots, PIECES + 1), top);
    assert_eq!(
        read_as_far_as_it_could_go(&roots, 2),
        [2],
        "a face no cut parted has itself for its whole lineage"
    );
    assert_eq!(root(&roots, 2), 2);
}

#[test]
fn a_piece_whose_root_was_edited_into_itself_has_itself_for_its_whole_lineage_and_its_root() {
    let roots = BTreeMap::from([(PIECES, PIECES)]);
    assert_eq!(
        read_as_far_as_it_could_go(&roots, PIECES),
        [PIECES],
        "a piece written down as parted from itself is not followed round to itself again"
    );
    assert_eq!(root(&roots, PIECES), PIECES);
}

#[test]
fn two_pieces_whose_roots_were_edited_into_each_other_end_their_lineage_where_root_says() {
    let roots = BTreeMap::from([(PIECES, PIECES + 1), (PIECES + 1, PIECES)]);
    assert_eq!(
        read_as_far_as_it_could_go(&roots, PIECES + 1),
        [PIECES + 1, PIECES],
        "the later piece is followed back to the earlier one, and not round again"
    );
    assert_eq!(root(&roots, PIECES + 1), PIECES);
    assert_eq!(
        read_as_far_as_it_could_go(&roots, PIECES),
        [PIECES],
        "the earlier piece, written down as parted from a later one, is not followed forward"
    );
    assert_eq!(root(&roots, PIECES), PIECES);
}
