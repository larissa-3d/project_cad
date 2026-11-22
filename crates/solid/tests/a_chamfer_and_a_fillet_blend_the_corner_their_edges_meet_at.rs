//! A chamfer and a fillet on straight edges of an exact body that meet at a
//! corner, at one value. Where the corner is convex — each of its three edges
//! meeting outwards — two of its edges cut trim each other in a mitre and
//! leave the third sharp, and all three are closed by a patch of their own
//! kind: a triangle among flats, a piece of sphere among rounds. A corner with
//! an edge meeting inwards is declined here, but for those #801 builds, held
//! in `a_chamfer_and_a_fillet_stop_and_mitre_beside_a_concave_edge.rs`. An
//! operation of the kernel on [`Body`], as the step on several edges of #698
//! is; the step of the history that calls it comes with the application, in
//! #701.
//!
//! Closes #699.
//! - Three edges of a convex corner chamfered at one value leave a closed
//!   body with a triangular face at that corner, meeting the three chamfers;
//!   three filleted at one value leave a closed body with a sphere piece
//!   there, meeting the three fillets. Volume exact, soundness checks pass —
//!   on the block's square corner,
//!   `a_chamfer_on_the_three_edges_of_a_corner_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`;
//!   and on a convex corner that is not square, as the third decision asks,
//!   the slanted prism's end, two of its edges square to the third —
//!   `a_chamfer_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`;
//!   and on one where no two of the three edges are square, a block whose
//!   top is cut by a plane tilted two ways, its flats closed by their
//!   triangle —
//!   `a_chamfer_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_triangle_where_the_flats_edges_cross`
//!   — and its rounds by their piece of sphere, since #800 draws it; a broken
//!   step with its reason, `Undrawn`, before —
//!   `a_fillet_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`;
//!   and several corners closed in one step, the body the same whatever the
//!   order the list names their edges: an upright of the block and the four
//!   edges at its two ends, two corners, and the twelve edges of the block,
//!   eight —
//!   `a_chamfer_and_a_fillet_on_an_upright_and_the_four_edges_at_its_two_ends_close_both_corners_whatever_the_order`,
//!   `a_chamfer_and_a_fillet_on_the_twelve_edges_of_a_block_close_its_eight_corners_into_one_body_whatever_the_order`;
//!   and two corners that are not square, at both ends of the slanted
//!   prism's top edge on its end, where the two cuts on either side cross
//!   that edge apart, not at one point —
//!   `a_chamfer_and_a_fillet_on_the_edges_at_both_ends_of_an_edge_whose_corners_are_not_square_close_both_whatever_the_order`;
//!   and edges named before a later join merged the faces at their corner
//!   with a neighbour's, the names still finding them, mitred or closed in
//!   any order —
//!   `a_chamfer_and_a_fillet_on_edges_named_before_a_join_merged_the_faces_at_their_corner_mitre_or_close_it_whatever_the_order`
//! - The triangle answers to the number after its edges' flats: on a list of
//!   `n` edges from where the steps' numbers ended at `start`, the flats are
//!   `start` to `start + n − 1` in the order the list names their edges, as
//!   #698 numbers them, and the triangle `start + n`, the step ending past it
//!   so that the next step numbers its own faces after the triangle. The
//!   triangle belongs to no one edge, so no edge's rank can name it; placed
//!   after them all, it moves no flat's number, whichever edge the list names
//!   first. The piece of sphere of three rounds is numbered the same way.
//!   Beyond the issue's wording, a choice of this branch —
//!   `a_chamfer_on_the_three_edges_of_a_corner_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`.
//!   Several corners closed in one step number their patches after every
//!   edge's face, one each, in the order of the lowest rank the list gives
//!   one of a corner's three edges, and of the next where two corners share
//!   that edge: the order the user named them in, never the kernel's own
//!   order of its vertices. Two corners share at most one edge, so the order
//!   is whole —
//!   `a_chamfer_and_a_fillet_on_an_upright_and_the_four_edges_at_its_two_ends_close_both_corners_whatever_the_order`,
//!   `a_chamfer_and_a_fillet_on_the_twelve_edges_of_a_block_close_its_eight_corners_into_one_body_whatever_the_order`,
//!   `a_chamfer_and_a_fillet_on_the_edges_at_both_ends_of_an_edge_whose_corners_are_not_square_close_both_whatever_the_order`,
//!   each on lists naming the same edges in different orders
//! - Decision: the triangle of three chamfers has its corners where the
//!   chamfer edges cross on each of the three faces; the sphere piece of three
//!   fillets is centred at the point one value from all three faces, and
//!   meets each fillet along that fillet's end section. Both hold for any
//!   convex trihedral corner, not only a square one. This branch held the
//!   triangle at any convex corner, and the sphere where one edge is square
//!   to the other two, as at the block's corner and the slanted prism's;
//!   where no two edges are square, the sphere's third arc is a great circle
//!   that is neither a meridian nor the equator of its axis, which the
//!   `(θ, φ)` chart of the sphere cannot follow, and the step was a broken
//!   step with its reason, `Undrawn`. Decision of #800: such a piece of
//!   sphere is drawn on a chart of its own, its central projection from the
//!   sphere's centre onto a plane tangent at a direction the whole piece lies
//!   in front of, where every great circle is a straight line, its boundary
//!   sampled as its neighbours share it; a face the `(θ, φ)` chart draws
//!   keeps its path. The tests of the criterion above, which hold the
//!   triangle's three corners and the sphere's centre, radius and the arcs
//!   it shares with the rounds:
//!   `a_chamfer_on_the_three_edges_of_a_corner_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`,
//!   `a_chamfer_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`,
//!   `a_chamfer_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`
//! - Two of a convex corner's three edges chamfered or rounded leave a closed
//!   body mitred at that corner, the third edge sharp. Volume exact, soundness
//!   checks pass — on each of the corner's three pairs, named in either order,
//!   `a_chamfer_on_two_of_a_corner_s_three_edges_mitres_their_flats_and_leaves_the_third_edge_sharp_one_distance_short`,
//!   `a_fillet_on_two_of_a_corner_s_three_edges_mitres_their_rounds_on_an_ellipse_and_leaves_the_third_edge_sharp_one_radius_short`;
//!   and on the case the issue opens on, the four top edges of a block, a
//!   mitre at each of its four corners —
//!   `a_chamfer_and_a_fillet_on_the_four_top_edges_of_a_block_mitre_its_four_corners`;
//!   and on a convex corner that is not square, the end of a prism whose
//!   sides slant, every pair of its three edges — the slanted upright among
//!   them — mitred whatever the angles —
//!   `a_chamfer_and_a_fillet_on_two_of_the_three_edges_of_a_corner_that_is_not_square_mitre_them_the_third_left_sharp`;
//!   and where the corner's uncut edge runs at its far end into a round an
//!   earlier step laid, which does not make that edge any less sharp at the
//!   corner —
//!   `a_chamfer_and_a_fillet_mitre_two_edges_whose_corner_s_uncut_edge_runs_into_an_earlier_round_at_its_far_end`
//! - Decision: two of three is a mitre. The two cuts trim each other: a
//!   straight mitre line for chamfers, a planar ellipse arc for fillets,
//!   running from where the two cuts' edges cross on their shared face down to
//!   the third edge, which stays sharp and ends one value short of the old
//!   vertex. No patch is added —
//!   `a_chamfer_on_two_of_a_corner_s_three_edges_mitres_their_flats_and_leaves_the_third_edge_sharp_one_distance_short`,
//!   `a_fillet_on_two_of_a_corner_s_three_edges_mitres_their_rounds_on_an_ellipse_and_leaves_the_third_edge_sharp_one_radius_short`
//! - A corner with a concave edge, and any corner the kernel cannot blend,
//!   gives a broken step with its reason, never an open body. Decision: a
//!   corner was blended only when every one of its edges was convex; the
//!   inner corner of an L-shaped block and the bottom of a pocket were
//!   declined `Unsupported`, the kernel's word for a shape it does not build,
//!   until #801, the follow-up issue the decision promised, built those
//!   corners. Its first slice builds every edge at such a corner alone — the
//!   L's top and floor edges at its inner corner, a pocket's rim, a standing
//!   block's uprights, each stopping flush on the face it runs into — and
//!   two convex edges meeting where the third is concave, mitred, the third
//!   left sharp: the L's two top edges at its inner corner and two edges of a
//!   pocket's rim, held now in
//!   `a_chamfer_and_a_fillet_stop_and_mitre_beside_a_concave_edge.rs`. Its
//!   second slice builds two concave edges meeting where the third is convex,
//!   mitred, the third left sharp: two feet of a block standing on another,
//!   meeting where its upright meets outwards, and its four feet, held now in
//!   `a_chamfer_and_a_fillet_fill_concave_edges_meeting_at_a_corner.rs`. Its
//!   third slice builds a pocket's floor edges and uprights alone, each fill
//!   trimmed flush on the faces at its ends, and two of the three concave
//!   edges at a pocket's bottom corner, mitred, the third left sharp: two
//!   edges of its floor, or one and the upright it meets, held now in that
//!   same file. #829's first slice builds the three edges at a standing
//!   block's foot corner, two feet and the upright above them, and its second
//!   the three at the top of the L's inner corner, its two top edges and the
//!   inner upright below them, held now in
//!   `a_chamfer_and_a_fillet_go_round_the_odd_edge_at_a_corner_of_mixed_sense.rs`.
//!   #846 builds a cut meeting a fill at a corner whose third edge is left
//!   sharp — a foot and the upright above it, the other foot left sharp, and
//!   the L's top edge and its inner upright, the other top edge left sharp —
//!   held now in
//!   `a_chamfer_and_a_fillet_go_round_the_odd_edge_and_stop_where_the_sharp_edge_begins.rs`
//!   — no test: the test that declined these corners here went with #846,
//!   every list it held having become a body test in the files above; any
//!   other corner the kernel cannot blend is the criterion below's
//! - A full round, two parallel edges whose rounds meet across the face they
//!   share, meeting a third edge of the list at a corner, was not built here:
//!   the fillet was declined `Unsupported` whichever edge the list names
//!   first, where the pair alone is laid. Since #803 the third edge's round
//!   is mitred against the full round, the body held below in place of the
//!   decline —
//!   `a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_mitres_that_edge_s_round_against_the_full_round_whatever_the_order`;
//!   a full round meeting an upright at a corner is #837's
//! - A face laid in place of others through `replaced`, as the piece of
//!   sphere is, is verified against the surface it is laid on, so that a
//!   wrong one is a broken step and never a body read as wrong geometry — no
//!   test: held beside the code, in `crates/solid/src/brep/assembly/tests.rs`,
//!   by `a_face_laid_on_a_surface_its_loop_does_not_lie_on_is_unverified`,
//!   `a_face_whose_corners_lie_on_its_surface_but_whose_edges_bow_away_from_it_is_unverified`
//!   and `a_face_laid_again_on_its_own_plane_is_taken`. The check reads each
//!   edge at its two corners and a quarter, half and three quarters along. Two
//!   corners and three samples settle a line or an arc on a plane, a sphere,
//!   a cylinder or a cone, none of which a line or a circle crosses more than
//!   four times without lying on it; a whole circle, which has no corners, is
//!   settled by its three samples only on a plane or a sphere. Sampled
//!   anywhere else along the same curve, the check decides the same there, so
//!   no fixture on those surfaces tells where along the edge the samples stand
//! - Three named edges at a vertex where four edges end are declined
//!   `Unsupported`, for the fillet and the chamfer, whichever three, and the
//!   four as well — `a_fillet_and_a_chamfer_on_three_of_the_four_edges_at_a_pyramid_s_apex_are_declined_as_unsupported`.
//!   The guard that counts the edges ending there is not alone in holding
//!   it: around such a vertex the faces run in a ring, so any three of its
//!   edges hold two that share no face, and the corner's closing, reading
//!   where each two of them cross on the face they share, declines it too.
//!   The four were declined until #804 closed the pyramid's apex on one
//!   surface, a piece of sphere or a square, held in its block below; the
//!   three stay declined, for #942. #699 held this on a pyramid standing on a
//!   box, whose hips' feet are vertices where four edges end too; since #804
//!   it is held on a pyramid standing on its own base, each hip's foot a
//!   corner of three edges, so that only the apex declines it. The pyramid on
//!   the box waits on #943, which takes a cut ending alone at a vertex of four
//!   edges
//! - A later edge the step's own earlier cut takes whole at their corner is
//!   laid even so, as the body given reads it, so that the step lays both
//!   cuts whole whichever edge the list names first, as every mitre here
//!   does: on the block whose corner a facet of 0.5 cuts off, the top's front
//!   edge's round is capped at 0.5 by the facet's corner on the front, and
//!   there takes the whole of the facet's edge on the front; the fillet on
//!   the two, at the cap or asked more, is the same body in either order,
//!   the two rounds' prisms cut whole, 5998.38 by the booleans. The chamfer
//!   on them is declined `Unsupported` in either order, as the top's front
//!   edge is alone at that cap, its flat passing through the facet's far
//!   edge and taking the facet whole: a list lays no edge its own step
//!   declines —
//!   `a_fillet_where_one_round_takes_the_next_edge_whole_lays_both_rounds_in_either_order_and_a_chamfer_declines_in_both`
//! - A corner whose piece of sphere would stand behind it along one of its
//!   edges, the centre one radius from the three faces lying past the corner
//!   on that edge's line, is declined `Unsupported` among rounds: the round
//!   of that edge would end on a plane through the centre past the corner.
//!   The same corner closes with its triangle among flats —
//!   `a_fillet_on_the_three_edges_of_a_corner_whose_sphere_would_stand_behind_it_along_an_edge_is_declined_as_unsupported`
//! - An edge of the body just cut on a later edge's line but outside its
//!   stretch, which the step must not take for the later edge — no test: it
//!   cannot be reached through the public API. Such an edge has matter right
//!   beside the line, within the margin the later edge's own prism stands off
//!   its faces, which is carried along the line as far as the faces meeting
//!   the later edge's ends reach; the later edge is declined alone,
//!   `Unsupported`, before any corner is read, where that matter lies within
//!   their reach
//! - Any other corner the kernel cannot blend — no test: a corner of more
//!   than three edges, of values that differ, or of a curved edge is out of
//!   the issue's scope, and no list of edges of a block, an L or a pocket
//!   reaches one; the concave corners above are the ones this file can build
//! - The tests of #698 and the single-edge issues still pass — no test: held
//!   by those files, left as they were but for the one test #699 takes back.
//!   `a_fillet_and_a_chamfer_on_two_edges_sharing_a_vertex_are_declined_as_unsupported`
//!   held that two edges meeting at the block's top front right corner are
//!   declined; they now meet in a mitre, and its two pairs are two of the
//!   three the mitre tests above hold
//!
//! Closes #800.
//! - Three fillets at a convex corner where no two edges are square leave a
//!   drawn, closed, uncrossed body with its piece of sphere — on the tilted
//!   block's corner, in every order the list names its three edges,
//!   `a_fillet_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`;
//!   and the drawing alone, on a sector of a ball whose piece of sphere is
//!   bounded by three great-circle arcs none of which is a meridian or the
//!   equator of the sphere's axis, built by hand beside the drawing —
//!   `a_sector_of_a_ball_bounded_by_three_great_circle_arcs_none_a_meridian_nor_the_equator_is_drawn_closed_and_encloses_its_volume`,
//!   in `crates/solid/src/brep/tessellation/tests/rounds.rs`
//! - The test #699 wrote on the tilted block (`tilted_block()`, corner
//!   (30, 0, 15), r = 2, volume 4311.32576 by independent booleans) then
//!   expects the body rather than the decline —
//!   `a_fillet_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`,
//!   the decline it held turned into a body held through
//!   `closes_with_a_sphere` as the slanted prism's corner is, to the volume
//!   `sphere_by_booleans` gives
//! - What must not break, in the issue's words: "Every sphere, ball and
//!   torus test (#553, #576 and the turned parts); the corner tests of
//!   #699." — no test: each is held by its own test, left as it was, but for
//!   the tilted block's fillet in this file, the one test of #699 that #800
//!   turns from a decline into a body
//!
//! Closes #803.
//! - The fillet on `[top-front, top-back, top-right]` of a block 6 wide at
//!   r = 3 leaves an exact, closed, uncrossed, drawn body of volume
//!   `V − 2r²(1 − π/4)(L − r) − 2r³/3`, whichever edge the list names first.
//!   The full round's face and the right edge's round meet along two
//!   quarter-ellipses from the right face's top corners to the crown. The
//!   uprights at the right end stay sharp, and no face is laid at the
//!   corners.
//!   `a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_is_declined_as_unsupported_in_either_order`
//!   becomes this body test, in each of the six orders the list can name the
//!   three edges in, the volume held by the booleans as well. Each arc is the
//!   quarter of the ellipse its plane cuts from both cylinders, from the end
//!   of its long axis on the right face to the end of its short one at the
//!   crown: held as each round drawn on its own cylinder and on its own side
//!   of the two planes, both drawn up to the right face's top corners and to
//!   the crown, and met by two edges between them —
//!   `a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_mitres_that_edge_s_round_against_the_full_round_whatever_the_order`
//! - The four top edges of that block, filleted at 3: the full round mitred
//!   at both ends, in any order the list names them —
//!   `a_fillet_on_the_four_top_edges_of_a_block_6_wide_mitres_its_full_round_against_both_end_edges_whatever_the_order`
//! - A value asked above what fits, 4 on the block 6 wide, is reduced to the
//!   full round's radius, 3, and the end edge is laid at that same value; the
//!   step reports both —
//!   `a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_asked_more_than_fits_lays_both_at_the_full_round_s_radius_and_reports_both`,
//!   in every order, the end edge alone taking 4 there; green as soon as
//!   it was written, answered by the change of the first criterion, the
//!   value the pair bounds the step to being the full round's radius
//!   exactly. A value asked a hair under 3, within what the kernel takes for
//!   set-backs that meet, lays the full round at its own radius and the end
//!   edge at that same radius, so that the two meet at the crown with no
//!   sliver between them —
//!   `a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_asked_a_hair_under_the_full_round_s_radius_lays_both_at_that_radius_meeting_at_the_crown`,
//!   in every order, asked 1e-6 and 1e-7 under, the strip of top left two
//!   of those, under four of the kernel's hairs on this block and over one
//!   or not, as #698 asks its full round; the step reports the value asked,
//!   and takes it or the full round's radius, a hair apart
//! - An end that is not square to the pair: a plate whose end face is cut at
//!   an angle, seen from above. The same mitre, exact, closed, uncrossed and
//!   drawn, its volume held by arithmetic or by the booleans, in either
//!   order —
//!   `a_fillet_on_a_full_round_pair_and_an_end_edge_cut_at_an_angle_seen_from_above_mitres_the_end_edge_s_round_against_the_full_round_in_either_order`,
//!   in every order, on a plate whose front is 30 long and its back 27, the
//!   end wall meeting them at 63.4 and 116.6 degrees; its volume held both
//!   ways, the two agreeing to the last digits, and the rest as on the square
//!   block, through the same check, the two arcs on the planes that halve the
//!   angle the slant makes. Green as soon as it was written, answered by the
//!   changes the first three criteria made: the guard of the first lets the
//!   slanted end edge in as it lets in the square one, its faces the top and
//!   the end wall at convex corners
//! - The faces are numbered as #698 and #699 number them: the full round
//!   answers to the earlier edge of the pair, the end edge to its own rank in
//!   the list, and nothing is numbered at the corner. Read as #698 lays a
//!   full round: one face answering to both its edges' numbers, a ray meeting
//!   it naming the earlier, and the step ending past every edge's number —
//!   held by the tests of the first two criteria, in every order they name
//!   the edges in —
//!   `a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_mitres_that_edge_s_round_against_the_full_round_whatever_the_order`,
//!   `a_fillet_on_the_four_top_edges_of_a_block_6_wide_mitres_its_full_round_against_both_end_edges_whatever_the_order`
//! - What must not break, in the issue's words: "#698's full round and round
//!   bottom, and #699's corner tests, except the decline above, which becomes
//!   a body test." — no test: each is held by its own test, left as it was,
//!   but for the decline the first criterion turns into a body
//! - And: "The chamfer on the same lists, which main already lays: its flats
//!   meet in a ridge, mitred with the end edge's flat (3321 on the block 30 ×
//!   6 × 20 at 3)." No test held it before this branch, which pins it, green
//!   before the fillet is built —
//!   `a_chamfer_on_a_full_round_pair_s_edges_and_an_edge_meeting_them_at_a_corner_mitres_their_ridge_against_that_edge_s_flat_whatever_the_order`
//! - What stays declined, so that the guard letting the end edge in lets in
//!   nothing else: the four top edges of a block 6 by 6, two full rounds
//!   meeting at its corners, declined `Unsupported` in every order the list
//!   can name its edges in, the pair alone laid. Green before this branch and
//!   after, a pin of the guard rather than a red test —
//!   `a_fillet_on_a_full_round_meeting_another_full_round_is_declined_as_unsupported_whatever_the_order`.
//!   A pocket 6 wide whose floor's front and back edges are filled at 3 in a
//!   round bottom, with its floor's edge on its end wall, was held there too,
//!   the test then named for both, until #838 built it, the round bottom
//!   meeting the end edge's fill in a valley: held now, with the pocket, in
//!   `a_round_bottom_meets_the_fill_of_the_edge_at_its_pocket_s_end_in_a_valley.rs`
//!   by
//!   `a_fillet_on_a_pocket_s_round_bottom_and_the_floor_edge_at_its_end_meets_that_edge_s_fill_in_a_valley_whatever_the_order`.
//!   The full round with the front's right edge, the upright at its end, was
//!   held there too until #837's first slice built it, the upright's round
//!   mitred against the full round on an ellipse; and the full round with the
//!   top's right edge and both uprights at that end, the tongue, until #837's
//!   second slice built it, its tip a quarter of a ball. Both are held now in
//!   `a_full_round_meets_the_round_of_an_upright_at_its_end_on_an_ellipse.rs`.
//!   A full round ending on a pointed end — a plate 6 wide whose right end
//!   comes to a point at (33, 3) seen from above, filleted at 3 on the top's
//!   front and back edges and the two edges of the point, each of which
//!   meets one edge of the pair alone — was held here too, declined
//!   `Unsupported` or stopping short of the full round in every order, until
//!   #843 built it: each point edge's round laid at the full round's radius
//!   and mitred against its two neighbours, the three mitres meeting at the
//!   crown, held now in
//!   `a_full_round_ends_on_a_pointed_end_mitred_against_each_edge_there.rs`
//!   by
//!   `a_fillet_on_a_full_round_and_the_two_edges_of_its_pointed_end_mitres_each_point_round_against_its_neighbours_up_to_the_crown`;
//!   the test that held the decline here went with #843. A bevelled end, its
//!   corners cut off upright, does not reach that far: with the end edge or a
//!   bevel edge in the list, the step takes a value short of the full round,
//!   where the rounds would meet through an end face, and no pair is laid
//!   together; raising it to the value asked is #894's
//! - What stayed declined beside, though no guard of this branch held it:
//!   the chamfer on the full round's edges, the end edge and both uprights,
//!   `Unsupported` in every order, where on the other lists above the chamfer
//!   is built, until #837's second slice built it — no test: the decline went
//!   with that slice, and the body, the end edge's flat closed to nothing
//!   between the two corners' triangles, is held in
//!   `a_full_round_meets_the_round_of_an_upright_at_its_end_on_an_ellipse.rs`
//!   by
//!   `a_chamfer_on_a_full_round_the_edge_across_its_end_and_both_uprights_there_closes_the_end_edge_s_flat_to_a_ridge_between_the_corners_triangles_whatever_the_order`
//!
//! Closes #804.
//! - `pyramid()`, its four hips filleted at 1: built in every order (24), one
//!   body, exact, closed and uncrossed. Each hip's round one face; the apex
//!   closed by one face on the sphere of radius 1 about (5, 5, 10 − √2),
//!   meeting each round along its end section. The volume the arithmetic
//!   gives —
//!   `a_fillet_on_the_four_hips_of_a_pyramid_closes_its_apex_with_a_piece_of_sphere_one_radius_from_its_four_faces_whatever_the_order`.
//!   Read on the pyramid standing on its own base, `pyramid_on_its_base`
//!   10 by 10, apex (5, 5, 5), rather than on `pyramid()`, by the decision of
//!   2026-11-13 in a comment on the issue: `pyramid()` stood on a box, and its
//!   hips' feet are vertices where four edges end, which a cut ending there
//!   alone does not reach yet, #943's; here each hip's foot is a corner of
//!   three edges, the sphere's centre (5, 5, 5 − √2), and the apex the only
//!   vertex of four. Read as: before the step, the centre one radius in from
//!   each of the four slopes; in each of the 24 orders, the value taken
//!   whole; the body exact, closed, uncrossed, holding 164.84162207554385, the
//!   arithmetic below, and its triangles enclosing that within the drawing's
//!   tolerance; one round for each hip, numbered by its rank in the list, and
//!   the piece of sphere numbered after the four, as #699 numbers its patch,
//!   the step ending past it, every face of the pyramid kept; the piece
//!   curved, drawn on the sphere and on the apex's side of every round's end
//!   section, its corners the centre's four feet on the slopes, met first
//!   from outside on rays toward its centre, and meeting the four rounds, one
//!   side each, and nothing else; each round curved, drawn on its cylinder of
//!   radius 1 about the line through the centre along its hip and never short
//!   of its end section, that section drawn along its arc between the feet on
//!   its two slopes, its foot ending on the base as at a corner of three
//!   edges, drawn along the curve the base cuts from its cylinder from one
//!   edge of the base to the other, the round met first from outside, and
//!   meeting its two slopes, the base and the piece of sphere, one edge each;
//!   each slope a triangle, from the foot the piece of sphere touches down
//!   along its two rounds to the base, meeting the base and those two rounds;
//!   the base meeting each slope along one sharp edge and the four rounds, no
//!   face left with a corner at the apex or at a corner of the base; and "one
//!   body" read as every order laying the body the first order lays, each
//!   face meeting what it meets there and each flat face keeping its corners
//! - The same four hips chamfered at 1: built in every order, one body, the
//!   apex closed by one square face on a plane square to the axis through the
//!   four flats' crossings; the volume the arithmetic gives —
//!   `a_chamfer_on_the_four_hips_of_a_pyramid_closes_its_apex_with_a_square_where_the_flats_edges_cross_whatever_the_order`.
//!   Read on the same pyramid on its base, for the same reason. Read as:
//!   before the step, the four crossings at one height, 5 − √(3/2); in each
//!   of the 24 orders, the value taken whole; the body exact, closed,
//!   uncrossed, holding 152.89141153805824, the arithmetic below; one flat for
//!   each hip by its rank, the square numbered after the four, the step ending
//!   past it, every face of the pyramid kept; the square's corners the four
//!   crossings, facing up along the axis, meeting the four flats one side
//!   each and nothing else; each flat facing halfway between its two slopes,
//!   its corners the crossings on those slopes and where its edges on them
//!   reach the base, meeting the square, its two slopes and the base, one
//!   edge each, its foot cut off along a straight side as at a corner of
//!   three edges; each slope a triangle from its crossing down to the base,
//!   meeting the base and the flats of its two hips; the base an octagon, its
//!   corners those eight points, meeting each slope along one sharp edge and
//!   the four flats; no face left with a corner at the apex; and every order
//!   laying the body the first lays, as for the fillet
//! - Throughout: #699's corner tests and #698's keep their bodies — no test:
//!   each is held by its own test, left as it was, #699's above in this file
//!   and #698's in `a_chamfer_and_a_fillet_take_several_edges_in_one_step.rs`;
//!   the one test of either that this issue changes is the decline below.
//!   The issue writes this criterion and the next two as one; it is split
//!   here, each part answered on its own
//! - #699's pinned decline
//!   `a_fillet_and_a_chamfer_on_three_of_the_four_edges_at_a_pyramid_s_apex_are_declined_as_unsupported`
//!   keeps its three-hip cases (the follow-up) and flips its four-hip case to
//!   the above. Read as: on the pyramid on its base, moved there from
//!   `pyramid()` so that the apex is the one vertex that declines them, every
//!   three of the four hips, in each of the six orders a list can name them
//!   in, declined `Unsupported` for the fillet and for the chamfer at 1, the
//!   hip left out running into the apex sharp, which neither closing reaches,
//!   until #942; the list of the four hips leaves that test for the two tests
//!   above
//! - An apex of four edges whose faces share no sphere stays declined in
//!   every order, pinned —
//!   `a_fillet_and_a_chamfer_on_the_four_hips_of_a_pyramid_10_by_6_whose_faces_share_no_sphere_are_declined_as_unsupported_whatever_the_order`.
//!   Read as: the pyramid on its base 10 by 6, `pyramid_on_its_base`, its
//!   apex at (5, 3, 5), its front and back slopes steeper than its left and
//!   right; before the step, no point of its axis standing as far in from the
//!   front and back as from the left and right, and the four crossings of the
//!   flats at 1 on no one plane; then its four hips, in each of the 24 orders,
//!   declined `Unsupported` for the fillet and for the chamfer at 1, until
//!   #942. Green before the branch and after, a pin rather than a red test
//! - Decision (2026-11-13): the closings of #699 carry over to a vertex of
//!   four edges where the shape allows one surface, every edge at it named.
//!   Among rounds, where the faces round the vertex are all tangent to one
//!   sphere of the radius, each round's axis passes through its centre and
//!   the vertex is closed by the piece of that sphere about it, meeting each
//!   round along its end section; among flats, where the points at which
//!   neighbouring flats cross lie on one plane, by the polygon through them.
//!   A right pyramid's apex is such a vertex for both —
//!   `a_fillet_on_the_four_hips_of_a_pyramid_closes_its_apex_with_a_piece_of_sphere_one_radius_from_its_four_faces_whatever_the_order`,
//!   `a_chamfer_on_the_four_hips_of_a_pyramid_closes_its_apex_with_a_square_where_the_flats_edges_cross_whatever_the_order`.
//!   Set aside, declined `Unsupported` in every order and pinned, for #942:
//!   some of the vertex's edges named, not all, and an apex whose faces share
//!   no sphere or whose crossings lie on no one plane —
//!   `a_fillet_and_a_chamfer_on_three_of_the_four_edges_at_a_pyramid_s_apex_are_declined_as_unsupported`,
//!   `a_fillet_and_a_chamfer_on_the_four_hips_of_a_pyramid_10_by_6_whose_faces_share_no_sphere_are_declined_as_unsupported_whatever_the_order`
//! - The patch at an apex of four answers to the number after its four
//!   edges' faces, as #699 numbers its triangle and its piece of sphere: on a
//!   list from where the steps' numbers ended at `start`, the rounds or the
//!   flats `start` to `start + 3` by their ranks, the patch `start + 4`, the
//!   step ending past it. Beyond the issue's wording, carried over from
//!   #699 —
//!   `a_fillet_on_the_four_hips_of_a_pyramid_closes_its_apex_with_a_piece_of_sphere_one_radius_from_its_four_faces_whatever_the_order`,
//!   `a_chamfer_on_the_four_hips_of_a_pyramid_closes_its_apex_with_a_square_where_the_flats_edges_cross_whatever_the_order`
//! - A hip cut alone, on either pyramid, ends at the apex at a vertex of four
//!   edges, and is declined there before any corner is read; so is any edge
//!   of `pyramid()`'s box ending at a hip's foot — no test: not this issue's,
//!   a cut ending alone at a vertex of four edges is #943's, whose scope takes
//!   the apex in. Set aside by the same decision, on this branch
//! - Decided on the way, after the green (2026-11-13): the chamfer's guard
//!   that the crossings lie on one plane is held on its own, which the pin
//!   above does not do: on the base 10 by 6 a later check declines the
//!   chamfer whether the guard stands or not. A crossing a little off one
//!   plane is declined either way too: on the base 10 by 9.8, the crossing on
//!   the left 0.025 off the plane through the three others, the chamfer at 1
//!   is declined `Unsupported` in each of the 24 orders, with or without the
//!   guard —
//!   `a_chamfer_on_the_four_hips_of_a_pyramid_10_by_9_8_whose_crossings_lie_a_little_off_one_plane_is_declined_as_unsupported_whatever_the_order`.
//!   The guard alone declines a crossing a hair off, past the kernel's
//!   tolerance, a billionth of the reach the body is drawn at, and short of
//!   some three times it, where the later check lets the closing through: on
//!   the base 10 by 9.999999, the crossing on the left 1.6 tolerances off, the
//!   chamfer at 1 is declined `Unsupported` in each of the 24 orders, until
//!   #942 —
//!   `a_chamfer_on_the_four_hips_of_a_pyramid_whose_crossings_lie_a_hair_off_one_plane_past_the_kernel_s_tolerance_is_declined_as_unsupported_whatever_the_order`.
//!   Read as: before each step, the crossing's distance off the plane read
//!   from the fixture, 0.025 on the first, between 1.2 and 2 tolerances on the
//!   second. Checked in a scratch copy with its own target, the guard's `>`
//!   turned into `==`: the second test fails, the chamfer built, closed and
//!   uncrossed, its patch a quadrilateral laid a hair off the fourth crossing;
//!   the first still passes. From 3.2 tolerances off the later check declines
//!   the chamfer without the guard; below one, the guard lets it through and
//!   the patch is built
//! - Decided on the way, after the green (2026-11-13): an apex beside four
//!   corners of mixed sense — the square pyramid standing on a plate 20 by 20
//!   by 2, `pyramid_on_a_plate`, its four hips named with the four feet where
//!   its slopes meet the plate, each hip the odd edge at its foot, as #829
//!   reads such a corner — chamfered at 1, goes round each hip at its foot and
//!   closes the apex with its square, one body in every order; filleted at 1,
//!   it is declined in every order —
//!   `a_chamfer_on_the_hips_and_feet_of_a_pyramid_on_a_plate_closes_its_apex_and_goes_round_each_hip_at_its_foot_in_one_body_and_a_fillet_is_declined_whatever_the_order`.
//!   Read as: the four hips alone, filleted and chamfered at 1, keep the
//!   plate's 800 with the pyramid's figures above, 964.84162207554385 and
//!   952.89141153805824, the apex closed on the plate as on the pyramid's own
//!   base; then, in 48 of the 40320 orders the eight edges can be named in,
//!   spread over them all: the chamfer taken whole, the body exact, closed,
//!   uncrossed, holding 965.567266914305, the volume measured on this branch
//!   at f2b6bdda, which no arithmetic here holds; one flat for each edge by
//!   its rank and five patches after them, one of them the apex's square,
//!   level, at the four crossings; every order laying the body the first
//!   lays, face for face, each patch found by its plane and its corners; and
//!   the fillet on the same lists declined `Unsupported` in each of those
//!   orders, though the four hips alone are built and so are the four feet
//!   alone: what declines is the hip as the odd edge of its foot corner among
//!   rounds, not the apex. Pinned as found, a decline this branch does not
//!   take, until #944
//! - Decided on the way, after the review (2026-11-13): what the fillet
//!   clears at the apex does not hang on the order the hips are named in.
//!   Matter standing apart in the air above the apex, out of every round's
//!   reach, is kept whole in every order: a cube of side 0.2 about
//!   (3.973, 3.949, 7.098), joined to the pyramid on its base in one body,
//!   over 2 above the apex and 0.6 clear of every slope's plane —
//!   `a_fillet_on_the_four_hips_of_a_pyramid_keeps_whole_a_cube_standing_apart_above_its_apex_whatever_the_order`,
//!   `a_chamfer_on_the_four_hips_of_a_pyramid_keeps_whole_a_cube_standing_apart_above_its_apex_whatever_the_order`.
//!   Read as: before the step, the cube that far clear; in each of the 24
//!   orders, the value taken whole; the body exact, closed, uncrossed, holding
//!   the pyramid's figure above with the cube's 0.008 added, 164.84962208
//!   filleted and 152.89941154 chamfered, its triangles enclosing that
//!   within the drawing's tolerance; one face for each hip by its rank and the
//!   patch after them; each of the cube's six faces with the corners it had
//!   and meeting what it met; and every order laying the body the first lays.
//!   Found by the review: the clearing about the sphere's centre is a cube
//!   turned by the way the last-named hip runs, and takes in that matter in
//!   some orders only — the fillet declined `Unsupported` in the six orders
//!   that name the front left hip last, built in the eighteen others; the
//!   chamfer built in all 24
//! - Decided on the way, after the review (2026-11-13): two apexes closed in
//!   one step, two pyramids on their own bases in one body, 10 apart, their
//!   eight hips named in one list —
//!   `a_fillet_on_the_hips_of_two_pyramids_in_one_step_closes_both_apexes_in_one_body_whatever_the_order`,
//!   `a_chamfer_on_the_hips_of_two_pyramids_in_one_step_closes_both_apexes_in_one_body_whatever_the_order`.
//!   Read as: each pyramid's four hips alone built, that pyramid closed and the
//!   other whole, `500/3` plus the closed one's figure; then, in 48 of the
//!   40320 orders, spread over them all, the value taken whole, the body
//!   exact, closed, uncrossed, holding twice the figure, 329.68324415
//!   filleted and 305.78282308 chamfered; one face for each hip by its
//!   rank, then one patch for each apex, numbered by the lowest rank of its
//!   hips, as #699 numbers several corners closed in one step; each patch on
//!   its own apex, a piece of the sphere about that apex's centre or the
//!   square through its crossings, meeting that apex's four faces and nothing
//!   else; and every order laying the body the first lays, each patch taken
//!   by its apex. Found by the review: both declined `Unsupported` in every
//!   order, the two clearings numbered alike
//! - Decided on the way, after the review (2026-11-13): an apex closed in
//!   the same step as #699's corners, the square pyramid on its base with its
//!   four hips and the four edges of its base named, each corner of the base
//!   a convex corner of three edges, its hip and two edges of the base; the
//!   fillet built in every order since #805 met a sphere and a plane at a
//!   slant, which fixed the boolean #946 found declining at the second
//!   corner of the base —
//!   `a_chamfer_and_a_fillet_on_the_hips_and_the_base_edges_of_a_pyramid_close_its_apex_and_its_four_base_corners_in_one_body_whatever_the_order`,
//!   the fillet there in 12 of the chamfer's 48 orders, every fourth, and in
//!   all 48 in a campaign run by hand —
//!   `a_fillet_on_the_hips_and_the_base_edges_of_a_pyramid_closes_its_apex_and_its_four_base_corners_in_one_body_in_each_of_48_orders`.
//!   Read as: the fillet on the four base edges alone built, exact, closed
//!   and uncrossed; then, in 48 of the 40320 orders: the chamfer taken whole,
//!   the body exact, closed, uncrossed, holding 141.77296518494913, which
//!   slicing the convex body by level planes gives — the pyramid cut by the
//!   eight flats' planes, the square's, and at each corner of the base the
//!   triangle's through its three crossings, (1, 1, 0) and
//!   (0.707107, 1.931852, 0.707107) and (1.931852, 0.707107, 0.707107) at the
//!   corner on the origin; one flat for each edge by its rank and five
//!   patches after them, one of them the apex's square; every order laying
//!   the body the first lays, each patch found by its plane and its corners.
//!   And the fillet on 12 of the same lists, every fourth — all 48 in the
//!   campaign — pinned declined `Unsupported` in each of those orders until
//!   #805's first slice built it: the value taken
//!   whole; the body exact, closed, uncrossed, holding 125.5604204966659, the
//!   pyramid opened by the ball of radius 1 — the pyramid shrunk about the
//!   centre of its inscribed sphere until its faces stand 1 in, then swelled
//!   by the ball, by Steiner's formula — and its triangles enclosing that
//!   within the drawing's tolerance; one round for each edge by its rank and
//!   five patches after them; each patch a piece of the sphere of radius 1
//!   about one corner of the shrunk pyramid, each of the five taken once,
//!   meeting the rounds of that corner's edges and nothing else; and every
//!   order laying the body the first lays, each patch found by its sphere
//! - Decided on the way, after the review (2026-11-13): the piece of sphere's
//!   chart, read from pairs of hips of consecutive rank, can turn with the
//!   order, so that the piece is drawn with other triangles — no test: the
//!   same-body checks here compare the faces each face meets and the corners
//!   of the flat ones, and the drawing of the piece only as lying on its
//!   sphere, whose centre and radius every order is held to; the axis of a
//!   face's sphere is not offered by the body, and no assertion reaches it but
//!   through the triangles. Measured on the branch, the piece is drawn with
//!   other triangles in 19 of the 24 orders, from 486 to 522 of them: a check
//!   that the triangles are the same in every order would be red today
//! - Decided on the way, after cargo-mutants (2026-11-15): four of the six
//!   mutants the production diff left alive are killed, each checked by hand.
//!   The two that turn `*` into `/` where the sphere's centre is read, one
//!   radius in from the first crossing's face, in the clearing and in the
//!   closing, live at a radius of 1 and die at another: the fillet at 0.5,
//!   read as the first criterion reads it at 1, the centre (5, 5, 5 − √2/2),
//!   holding 166.205785298069 —
//!   `a_fillet_on_the_four_hips_of_a_pyramid_at_half_a_unit_closes_its_apex_on_the_sphere_of_that_radius_whatever_the_order`.
//!   The one that turns `&&` into `||` where a clearing is taken, letting
//!   through a clearing that takes a face of the body away whole, dies on a
//!   cube of side 0.1 on the axis 1.3 above the apex, inside what the fillet
//!   clears about the sphere's centre and out of every round's reach: the
//!   fillet declined `Unsupported` in every order, a broken step rather than
//!   a body short of the cube, and the chamfer, whose clearing stops at the
//!   apex, keeping the cube whole in every order, 152.89241154 —
//!   `a_fillet_whose_clearing_would_take_whole_a_cube_standing_just_above_the_apex_is_declined_and_a_chamfer_keeps_the_cube_whatever_the_order`.
//!   Written while seeking a step where ends held on a plane meet a clearing,
//!   and kept as a pin of what the chamfer builds there, though `CLEARED`
//!   moved down onto `ENDS` leaves it alike: the four hips of the pyramid on
//!   a plate and its front foot, chamfered at 1, one body in 24 of the 120
//!   orders, holding 956.595690631586 as measured at 8c2ae55e —
//!   `a_chamfer_on_the_four_hips_of_a_pyramid_on_a_plate_and_one_of_its_feet_closes_its_apex_in_one_body_whatever_the_order`
//! - Mutants kept, after cargo-mutants (2026-11-15) — no test: each is
//!   equivalent where the kernel reaches today. In `Exact::cleared`, the
//!   field `next` deleted from the clearing's tool hands the cut the tool's
//!   own count, which only moves the numbers later clearings take, each still
//!   apart, and the step sets the count itself before laying each edge; every
//!   pyramid test of this file passes with it, two apexes in one step among
//!   them. In `triangle::tip`, `>` turned into `>=` in the coplanarity guard
//!   differs only where a crossing stands off the plane by the tolerance to
//!   the last bit, which no fixture reaches: the crossings of a regular apex
//!   stand off it by rounding, the hair-off pin's by 1.6 tolerances. In
//!   `overlap`, `CLEARED`'s `1 << 28` turned into `1 >> 28` lays it on
//!   `ENDS`: a clearing's number, `ENDS` with the step's count and its rank,
//!   is then no end's, `ENDS` and a multiple of `STRIDE`, and the range the
//!   odd corners read ends by finds no cleared face still standing — the
//!   pyramid on a plate chamfered at its four hips and three feet, the back
//!   one left out, ends held at its back corners and odd corners at its front
//!   ones, builds the same body either way; the fillet there, where the order
//!   could tell, is declined until #944
//!
//! Closes #946.
//! - First a smaller fixture: a body with an edge whose faces meet at an
//!   acute angle (45°), its two end corners closed by #699 in one step.
//!   Built in every order, or the decline reproduced there and its cause in
//!   the boolean named —
//!   `a_chamfer_and_a_fillet_on_the_edges_at_both_ends_of_an_edge_whose_faces_meet_at_45_degrees_close_both_corners_whatever_the_order`,
//!   and the fillet in every order in a campaign run by hand,
//!   `a_fillet_on_the_edges_at_both_ends_of_an_edge_whose_faces_meet_at_45_degrees_closes_both_corners_in_each_of_120_orders`.
//!   Read as: on the hipped block, 30 by 20 by 10, its front and both ends
//!   sloping in at 45° from the edges of its floor, its back upright and its
//!   top level, the front's foot, where the front meets the floor at 45°,
//!   ends at two corners of three edges, the foot, a hip and an end's foot,
//!   as at the square pyramid's base corners, with no apex; before the step,
//!   the block holding 9500/3, the three feet meeting at 45° and the two hips
//!   at 120°, and the arithmetic below agreeing with the booleans; then the
//!   five edges in each of the 120 orders a list can name them in for the
//!   chamfer at 1, and in 30 of them, every fourth, for the fillet at 1, all
//!   120 in the campaign: the value taken whole; the body exact, closed,
//!   uncrossed, holding 3128.429773960448 chamfered and 3083.3582436557235
//!   filleted, its triangles enclosing that within the drawing's tolerance;
//!   one face for each edge by its rank, meeting that edge's two faces, then
//!   one patch for each corner, numbered by the lowest ranks of its edges as
//!   #699 numbers corners closed in one step, the step ending past them and
//!   every face of the block kept; each patch at its own corner, the triangle
//!   through its three crossings facing the old corner, or the piece of the
//!   sphere of 1 about the point one in from its three faces, curved, drawn
//!   on the corner's side of its three rounds' end sections, touching each
//!   face where the centre's foot on it stands, met first by rays from
//!   outside toward its centre; each patch meeting the three faces laid on
//!   its corner's edges and nothing else; the front's foot's face running
//!   between the two patches, its flat's corners the four crossings on the
//!   floor and the front, facing halfway between the two, or its round drawn
//!   on its cylinder about the line through the two centres and between their
//!   end sections, each section drawn along its arc between the feet on the
//!   floor and the front, the round met first from outside; that face meeting
//!   the floor, the front and the two patches and nothing else; no flat face
//!   left with a corner at either old corner; and every order laying the body
//!   the first lays, face for face, each patch found by its plane and its
//!   corners or by its sphere. Found on main at 54317165: built in every
//!   order, both cuts, so the criterion's first arm holds and no boolean is
//!   left to name. Measured before #947, at 1aeb5750, in a scratch copy with
//!   its own target: the fillet declined `Unsupported` in all 120 orders,
//!   where each corner's three edges alone and any four of the five were
//!   built — the second corner declining once the first is closed, as the
//!   pyramid's did, which this fixture reproduces and #947 took away. A block
//!   bevelled along one edge at 45°, its ends square to that edge, built in
//!   every order before #947 as after: it would not have told, and is not
//!   held
//! - The pinned decline above flips to one body in every order, with the
//!   volume — flipped by #947 before this branch, the test renamed from
//!   `a_chamfer_on_the_hips_and_the_base_edges_of_a_pyramid_closes_its_apex_and_its_four_base_corners_in_one_body_and_a_fillet_is_declined_whatever_the_order`
//!   to
//!   `a_chamfer_and_a_fillet_on_the_hips_and_the_base_edges_of_a_pyramid_close_its_apex_and_its_four_base_corners_in_one_body_whatever_the_order`,
//!   with its campaign
//!   `a_fillet_on_the_hips_and_the_base_edges_of_a_pyramid_closes_its_apex_and_its_four_base_corners_in_one_body_in_each_of_48_orders`.
//!   Read as #804's block above reads it, every order being 48 of the 40320
//!   the eight edges can be named in, spread over them all: the fillet at 1
//!   in every fourth of the 48 in the gate and in all 48 in the campaign, the
//!   value taken whole, the body exact, closed, uncrossed, holding
//!   125.5604204966659, the pyramid opened by the ball of radius 1 by
//!   Steiner's formula, one round for each edge and five pieces of sphere
//!   after them, each on the sphere about one corner of the shrunk pyramid,
//!   and every order laying the body the first lays. Both pass on main at
//!   54317165, the test as #947 left it, this branch touching neither
//!
//! The volumes are arithmetic, measured from the corner along its three
//! edges: a point stands `(a, b, c)` from it, `a` along the edge whose length
//! is `l1`, `b` along `l2`, `c` along `l3`, on a block of volume `V`. At a
//! square corner a chamfer of the first edge alone takes away the prism
//! `b + c < d` along all of it, `d²·l1/2`; a fillet of radius `r`, the strip
//! between the faces and the round, `r²(1 − π/4)·l1`.
//!
//! Two chamfers mitred, the first two edges: the prisms `b + c < d` and
//! `a + c < d` overlap at the corner where both cut, whose section at a depth
//! `c` below their shared face is a square of side `d − c`; it holds the
//! integral from 0 to `d` of `(d − c)²`, `d³/3`, counted twice. The body keeps
//! `V − d²(l1 + l2)/2 + d³/3`.
//!
//! Two fillets mitred: at a depth `c` below the shared face, each round takes
//! the width `w = r − √(r² − (r − c)²)` off its edge's other face; the overlap
//! is a square of side `w`, holding, with `u = r − c`, the integral from 0 to
//! `r` of `(r − √(r² − u²))²`, `2r³ − 2r·πr²/4 − r³/3 = r³(5/3 − π/2)`. The
//! body keeps `V − r²(1 − π/4)(l1 + l2) + r³(5/3 − π/2)`.
//!
//! Three chamfers: outside the cube of side `d` at the corner the three prisms
//! do not overlap — a point of the first's past `a = d` is beyond the other
//! two — each taking `d²(l − d)/2`. Inside it every point goes but those with
//! `a + b + c ≥ 2d`, a tetrahedron of `d³/6` at the cube's far corner, which
//! the triangle through the three crossings bounds: if `a`, `b` and `c` are at
//! most `d`, a sum of `2d` or more leaves each pair `d` or more. The body
//! keeps `V − d²(l1 + l2 + l3)/2 + 3d³/2 − (d³ − d³/6)`, that is
//! `V − d²(l1 + l2 + l3)/2 + 2d³/3`.
//!
//! Three fillets: outside the cube of side `r` the three rounds do not
//! overlap, each taking `r²(1 − π/4)(l − r)`. Inside it every point goes but
//! the eighth of the ball of radius `r` about the cube's far corner, `πr³/6`,
//! which lies inside all three rounds. The body keeps
//! `V − r²(1 − π/4)(l1 + l2 + l3) + 3r³(1 − π/4) − r³(1 − π/6)`, that is
//! `V − r²(1 − π/4)(l1 + l2 + l3) + r³(2 − 7π/12)`: 5888.63 at 3 on the block
//! 30 by 10 by 20. Held another way: the three rounds' prisms alone, cut from
//! the block by the booleans, leave in that cube what lies inside all three
//! cylinders, the eighth of a Steinmetz tricylinder, `(2 − √2)r³`; the sphere
//! leaves of it the eighth of the ball, `πr³/6`, and the two counts agree to
//! the last digit with that body less `(2 − √2 − π/6)r³`. The booleans
//! decline the piece itself, cut as a box less a ball, its sphere tangent to
//! the rounds along their ends: no body of theirs holds every face of it.
//!
//! The four top edges of a block, two of three at each of its four corners:
//! each pair overlaps once, so the chamfered body keeps
//! `V − d²(2L + 2B)/2 + 4d³/3`, and the filleted one
//! `V − r²(1 − π/4)(2L + 2B) + 4r³(5/3 − π/2)`, `L` and `B` its length and
//! breadth: 5676 for the chamfer at 3 on the block 30 by 10 by 20, and
//! 5855.84 for the fillet at 3.
//!
//! The corner that is not square is the end of a prism 30 long whose section
//! is 16 wide at its bottom, 10 at its top and 10 high, its right side
//! slanting in: the end, the top and that side meet at (30, 12, 10). The
//! top's edge on the end, A, and the upright where the side meets the end, C,
//! are square; the top's edge on the side, B, meets at the side's slant, an
//! angle θ with `cos θ = −4/√116`; write `k = sin θ = 10/√116`. Measured
//! from the corner, `x′` in from the end, `w` down from the top and `u` in
//! from the side along Y, the side is `u = 0` and stands `ku` away. Alone, a
//! flat at `d` takes: on A, which runs across the end between two sides that
//! lean out as they go down, a length `10 + 0.6w` at a depth `w`,
//! `5d² + 0.1d³`; on B the triangle `u/d + w/(kd) < 1` along its 30,
//! `15kd²`; on C the square cut `d²/2` along its `10/k`, `5d²/k`. Two flats
//! overlap where both cut: A and B by `kd³(1/2 − k/6)`, `x′ < d − w` and
//! `u < d − w/k` at a depth `w`; B and C by the same; A and C by `d³/(3k)`,
//! `w < d − x′` and `u < (d − x′)/k` at `x′`. A round at `r` alone takes, on
//! A, `10r²(1 − π/4) + 0.6r³(5/6 − π/4)`; on B, `30r²(cot(θ/2) − (π − θ)/2)`;
//! on C, `(10/k)r²(1 − π/4)`; rounds on A and C overlap by
//! `r³(5/3 − π/2)/k`, the square corner's integral stretched by `1/k` across
//! the slant. At 2, the prism of 3900 keeps 3826.06 with flats on A and B,
//! 3860.53 on A and C, 3825.32 on B and C, and 3882.77 with rounds on A and
//! C. Where a round on B meets one on A or C, the overlap is an integral of
//! two arcs at that angle with no short closed form: there the volume
//! promised is the prism's with the two cuts' prisms, each raised whole along
//! its edge's line, taken away by the booleans — the fixture the test also
//! holds every face of the step against.
//!
//! All three edges of the slanted corner cut, in the same coordinates, the
//! flats are `x′ + w < d`, `x′ + s < d` and `w + s < kd`, and they cross on
//! the three faces at `(d, 0, kd)`, `(0, d, d)` and `(d, kd, 0)`: the
//! triangle's plane is `(1 − k/2)x′ + (w + s)/2 = d`. A volume in these
//! coordinates is `k` times the volume it stands for. The three flats overlap
//! in `kd³/2 − k²d³/4` — at `x′`, a square of side `d − x′` cut by
//! `w + s < kd` — and the triangle takes besides the tetrahedron between it
//! and the point the three flats meet at, `(d(1 − k/2), kd/2, kd/2)`, of
//! `kd³(1 − k/2)/6`; at `k = 1` the two are the square corner's `d³/4` and
//! `d³/12`. With the overlaps of pairs above, the prism keeps
//! `V − 5d² − 0.1d³ − 15kd² − 5d²/k + (k + 1/k)d³/3`: 3807.30 at 2.
//!
//! Its three rounds meet the sphere of radius `r` centred `(r, r, r)` in
//! these coordinates, at `(28, 12 − r·cot(θ/2), 8)` for `r = 2`; each ends
//! on the plane through that centre square to its edge, and the three planes
//! bound at the corner a cell whose edges run from the centre the three ways
//! the faces face. Each round alone, less its part on the corner's side of
//! its end section, is laid; the cell is taken away and the ball's sector in
//! it given back. On A's side of its section the round runs on to the side,
//! leaning out: `r·cot(θ/2)·S + 0.4r³(5/6 − π/4)`, with `S = r²(1 − π/4)`,
//! the first moment of the round's section about the top adding the lean; on
//! C's the same, measured from the top to the section along the slant; on
//! B's, `r` along its section `r²(cot(θ/2) − (π − θ)/2)`. The cell is a prism
//! along X, `r` long, whose section holds `r²(1 − 0.6k + 0.232k³)/k`; the
//! three ways the faces face span a solid angle of `π − θ`, and the sector
//! holds a third of `r³` times that. The prism keeps 3873.16 at 2.
//!
//! Both are held a second way, by the booleans, and agree to the last digits:
//! the flats' three prisms and the slab beyond the triangle's plane cut away;
//! the rounds' three prisms cut away, then what is left inside the cell, and
//! the sector added back. On the square corner the same computation gives the
//! formulas' 5748 and 5888.63 exactly. The corner where no two edges are
//! square, the block's top cut by the plane rising 0.1 along X and 0.2 along
//! Y from 12, is measured that way only: 4245.76 with the flats at 2, 4311.33
//! with the rounds.
//!
//! Several corners closed in one step on the block 30 by 10 by 20, each at
//! least the value twice from any other, are each the corner above: every
//! edge's prism along its whole length, and at each corner closed the cube of
//! side the value given back as the corner alone gives it, `2d³/3` among
//! flats and `r³(2 − 7π/12)` among rounds. With `L` the lengths of the edges
//! named summed and `n` the corners closed, the block keeps
//! `V − d²L/2 + 2nd³/3` and `V − r²(1 − π/4)L + nr³(2 − 7π/12)`, where every
//! other end of an edge named meets no other edge named. For an upright and
//! the four edges at its two ends, `L` is 100 and `n` 2: 5586 at 3 with
//! flats, 5815.90 with rounds. For the twelve edges, `L` is 240 and `n` 8:
//! 5064 and 5572.62, the second the box with every edge and corner rounded,
//! `(a − 2r)(b − 2r)(c − 2r)` and its six slabs, twelve quarter-cylinders and
//! eight eighths of a ball, to the last digit.
//!
//! The slanted prism's top edge on its end ends at two corners that are not
//! square: the right side's, `k = 10/√116`, and the left side's, leaning in 1
//! in 5, `k′ = 10/√104`. Its edges on both, chamfered at `d`, close both
//! corners each as above, far enough apart not to meet: the end's top edge
//! alone takes `5d² + 0.1d³`, its two ends leaning out 0.4 and 0.2 a unit of
//! depth; the two top edges on the sides `15kd²` and `15k′d²`; the two
//! slanted uprights `5d²/k` and `5d²/k′`; and each corner gives back
//! `(k + 1/k)d³/3` and `(k′ + 1/k′)d³/3`. The prism keeps 3733.40 at 2, which
//! the booleans give too; the rounds, by the booleans corner by corner,
//! 3849.57.
//!
//! The block whose top's front and right edges and front's right edge are
//! named, then joined to a block 10 by 10 by 20 on its left, flush with its
//! front, top, floor and back, is a block 40 long holding 8000: the top's
//! front edge 40 long, the corner's other two as they were. Mitred, two
//! edges keep 8000 less the mitre's loss on their lengths, 7784 and 7906.02
//! for the top's two edges, 7739 and 7886.70 for the top's front edge and the
//! front's right edge; closed, the three keep 7703 and 7869.32.
//!
//! The block whose floor's front edge was rounded at 3 by an earlier step,
//! its top's front and right edges then mitred at 3, keeps the block less that
//! round's `9(1 − π/4)·30` and less the mitre's own loss, the two far apart:
//! 5771.06 with flats, 5867.39 with rounds, which the booleans give to the
//! last digit.
//!
//! The rounds at the corner where no two edges are square, a broken step
//! until #800, close it with their piece of sphere, held through
//! `closes_with_a_sphere` as at the slanted prism's corner: a body of
//! 4311.33 at 2, the sphere centred at (28, 2, 13.15), 2 in from the right
//! wall, the front and the tilted top, touching each at the foot of the
//! centre on it; each round ending on the plane through that centre square
//! to its edge, along an arc between two of those feet; the three ways the
//! faces face spanning 1.4536 of solid angle for the piece of sphere, whose
//! sector the volume gives back. Its triangles are held at the tolerance it
//! is drawn at, which is not its box's: a cut keeps the larger reach of the
//! part and the tool, and the slab its top is cut off by reaches 90.6, three
//! times the block's 30.
//!
//! At a corner that is not square, the line where the two cuts meet need not
//! run down to the third edge: it can leave by the face the third edge shares
//! with one of the cut ones, and run on along that face. The third edge ends
//! where the cut set back farthest along it crosses it, that cut's set-back
//! over the sine of the angle its edge makes with the third; at a square
//! corner both come to one value.
//!
//! A full round mitred against the edge at its end, on a block `2r` wide: the
//! top's front and back edges rounded at `r` meet across the top in one
//! half-cylinder, which takes `2r² − πr²/2`, that is `2r²(1 − π/4)`, of the
//! section along the length. The end edge's round, raised from its wedge as
//! the body given reads it, is a quarter-cylinder of `r` across the whole
//! width, its axis crossing the full round's one radius in from the end and
//! one radius under the top. Short of the box `r` long, `2r` wide and `r`
//! deep at the end, the full round alone cuts; in that box, of `2r³`, the
//! body keeps what lies inside both cylinders, a quarter of the Steinmetz
//! bicylinder of radius `r`, `4r³/3`. The block keeps
//! `V − 2r²(1 − π/4)(L − r) − 2r³/3`, and with the edge at each end so
//! mitred `V − 2r²(1 − π/4)(L − 2r) − 4r³/3`: 3477.70 and 3471.29 on the
//! block 30 by 6 by 20 at 3. The first is the limit at a width of `2r` of the
//! two mitres a wider block keeps, `V − r²(1 − π/4)(2L + w) + 2r³(5/3 − π/2)`,
//! 3483.68 at 6.01 wide. The booleans give it too, the full round's prism and
//! the end edge's, each raised whole along its line, taken away from the
//! block. Two cylinders of one radius whose axes cross meet along two
//! ellipses, in the two planes through the line square to both axes that
//! halve the angles between them; on the body each is a quarter, from a top
//! corner of the end face, the end of its long axis, up to the crown, the end
//! of its short one, where the two planes cross and the two cylinders touch.
//!
//! An end wall standing at an angle seen from above, meeting the long edges
//! at θ, on a plate whose outline holds the area `A`: the end edge's round's
//! axis, a radius in from the wall and under the top, crosses the full
//! round's at `X` along it. Short of the plane through that crossing
//! parallel to the wall the full round alone cuts, keeping over its axis its
//! half-disc, `πr²/2`, along a length that runs to `X` at the middle of the
//! top and a little more or less either side of it, by as much one way as the
//! other, so that `πr²/2 · X` is what it keeps. Past that plane the body keeps
//! what lies inside both cylinders, the same quarter bicylinder as at a square
//! end, measured by the distance out square to the wall and the distance
//! across the top; an area of the top holds `1/sin θ` times what those two
//! measure, so the quarter bicylinder holds `4r³/(3 sin θ)`. The plate keeps
//! `(H − r)A + πr²X/2 + 4r³/(3 sin θ)`, and at θ = 90° that is the square
//! end's count, 3477.70. On the plate whose front is 30 long and back 27,
//! `A` is 171, `sin θ` is `2/√5` and `X` is `28.5 − 1.5√5`, 25.146: it keeps
//! 3302.74, which the booleans give to the last digits. The two cylinders
//! still meet on two planes through the vertical at the crossing, halving the
//! angles between the two axes, each arc a quarter of an ellipse from a top
//! corner of the end wall up to the crown.
//!
//! The chamfer on the same three edges at `d = r`: the two flats meet in a
//! ridge along the middle of the top, taking `d²` of the section along the
//! length. In the box at the end the body keeps what lies under all three
//! flats, a height over the box's floor at most the least of the distances
//! in from the front, the back and the end, `2d³/3` of the box's `2d³`. The
//! block keeps `V − d²(L − d) − 4d³/3`, 3321 on the block 30 by 6 by 20 at 3.
//!
//! The square pyramid, `pyramid_on_its_base` 10 by 10, stands on its own base
//! with its apex at (5, 5, 5), its four slopes leaning 45 degrees: `500/3`.
//! Each of its four hips runs `5√3` from the apex down to a corner of the
//! base, between two slopes whose outward normals stand 60 degrees apart, 120
//! degrees inside the matter, and ends there at a corner of three edges, the
//! hip and the two edges of the base. A point (5, 5, z) of the axis stands
//! `|z − 5|/√2` in from each slope, so the point one radius in from all four
//! is (5, 5, 5 − √2·r): the centre of the piece of sphere. Each round's axis
//! runs through it along its hip, and each round ends on the plane through it
//! square to its hip, `√(2/3)·r` down the hip from the apex.
//!
//! The four rounds at `r`. Along its hip each takes the section between its
//! two slopes and its arc, `r²(1/√3 − π/6)`, from its end section down to the
//! plane through the hip's foot square to it, `5√3 − r√(2/3)` long. There
//! the base cuts the round's prism slantwise: a line of the prism along the
//! hip, `(w_x, w_y, w_z)` off the foot in that plane, enters the pyramid where
//! it crosses the base, `−√3·w_z` past the plane. With `α` measured into the
//! matter along the line halfway between the slopes and `β` across it, `w_z`
//! is `−2α/√6`, so each line is `√2·α` short; over the section at `r = 1`,
//! `α` sums to `11/18 − π/(3√3)`, and the foot spares
//! `√2·r³(11/18 − π/(3√3))`, 0.0092084 at 1. The four end sections bound at
//! the apex a cell, a square cone from the centre about the axis whose edges
//! run the four ways the slopes face, `|x| + |y| ≤ z` measured from the
//! centre. Within the pyramid, each round's prism lies inside the cell on the
//! apex's side of its own end section, and no two prisms meet outside it: the
//! body is the pyramid less each round's prism past its end section, less the
//! cell, the ball's sector in the cell given back. Of the pyramid the cell
//! holds what lies under the four slopes, `z ≤ √2·r − max(|x|, |y|)`, that is
//! `4√2r³/9`; the sector spans the solid angle between the four ways the
//! slopes face, `4 asin(1/3)`, and holds `4 asin(1/3)·r³/3`. The body keeps
//! `500/3 − 4(r²(1/√3 − π/6)(5√3 − r√(2/3)) − √2·r³(11/18 − π/(3√3))) − 4√2r³/9 + 4 asin(1/3)·r³/3`,
//! 164.84162207554385 at 1. Held a second way, off the kernel: the length
//! each line of a round's prism keeps in the pyramid, from where it enters it
//! to the end section, read by bisection against the base and the slopes
//! alone and integrated over the section, gives each round's share to the
//! last digits, and sampling the pyramid finds no point two prisms past their
//! end sections share, nor any of a prism on the apex's side of its section
//! outside the cell.
//!
//! The four flats at `d`. Neighbouring flats cross on the slope they share,
//! each `d` from its hip, at (5, 5 − √(3/2)·d, 5 − √(3/2)·d) and its three
//! images about the axis: all four at one height, the square through them
//! level, its side `√3·d`. Above it the pyramid's tip, `√(3/2)·d` high on a
//! square base twice as wide, `√6·d³`, goes. Below it no two flats' prisms
//! overlap, the line two neighbours' planes meet along climbing from their
//! crossing into the pyramid. Each prism's section is the triangle between
//! the hip and its two set-back lines, `√3d²/4`, and a line of it along the
//! hip runs from the base, `√2·α` past the plane square to the hip at its
//! foot, up to the square's plane, `√3(5 − √(3/2)·d) + √2·α` past it: every
//! line the same length, `√3(5 − √(3/2)·d)`, and the prism
//! `(3/4)(5 − √(3/2)·d)d²`. The body keeps `500/3 − 15d² + √6d³/2`,
//! 152.89141153805824 at 1, which slicing the body, convex, by level planes
//! gives to the last digits.
//!
//! The pyramid on its base 10 by 6 has its apex at (5, 3, 5), its front and
//! back slopes rising 5 in 3 and its left and right 5 in 5; it holds 100. A
//! point one radius in from both the front and the back stands on the plane
//! y = 3, one in from both the left and the right on x = 5: on the axis,
//! where a point `h` under the apex stands `3h/√34` in from the front and the
//! back and `h/√2` in from the left and the right. No point stands as far in
//! from all four, and no sphere touches them all. The flats at 1 cross on the
//! front and the back 3.683 high, and on the left and the right 3.190 high:
//! the four crossings lie on no one plane.
//!
//! The hipped block, 30 by 20 by 10, is level-sectioned `30 − 2z` by `20 − z`
//! at a height `z` and holds `30·20·10 − (30 + 2·20)·10²/2 + 2·10³/3`,
//! 9500/3. At its corner on the origin the floor, the front `z ≤ y` and the
//! left end `z ≤ x` meet, as at the square pyramid's base corners: the two
//! feet meet at 45° and the hip, along (1, 1, 1)/√3, at 120°. The point one
//! radius in from the three is `(r(1 + √2), r(1 + √2), r)`, and each round
//! ends on the plane through it square to its edge.
//!
//! Its rounds at `r`. A foot's section, between its two faces at 45° and its
//! arc, holds `r²(1 + √2 − 3π/8)`: the front's foot runs it between its two
//! end sections, `30 − 2r(1 + √2)`, and each end's foot from its end section
//! to the back, square to it, `20 − r(1 + √2)`. A hip's section holds
//! `r²(1/√3 − π/6)`, from its end section, `r(3 + 2√2)/√3` up the hip, to
//! the top, which cuts the round's prism slantwise: a line of it `α` into the
//! matter along the line halfway between the slopes reaches the top `√2·α`
//! further on, as the base spares the pyramid's hips at their feet, so that
//! the top adds `√2·r³(11/18 − π/(3√3))` to the section along
//! `10√3 − r(3 + 2√2)/√3`. The three end sections bound at each corner a
//! cell, the cone from the centre along the ways the three faces face.
//! Sliced level `Z` above the centre, the block holds in it the square
//! `(√2·r − Z)²` between the front, the end and the two sections square to
//! the feet; above the centre the hip's section cuts the square's corner
//! off, `Z²/2`, and from `Z = r/√2` leaves of it the triangle
//! `(2√2·r − 3Z)²/2`, up to `Z = 2√2·r/3`: `r³(7/3 + 14√2/9)` from the
//! floor up. The ball's sector in the cell spans the solid angle between the
//! three ways, `2 atan(3 + 2√2)`, and holds a third of `r³` times it. The
//! body keeps
//! `9500/3 − r²(1 + √2 − 3π/8)(70 − 4r(1 + √2)) − 2r²(1/√3 − π/6)(10√3 − r(3 + 2√2)/√3) − 2√2·r³(11/18 − π/(3√3)) − 2r³(7/3 + 14√2/9) + 4 atan(3 + 2√2)·r³/3`,
//! 3083.3582436557235 at 1, which the booleans give to the last digits —
//! the five rounds' prisms cut away, then each cell, its sector given back.
//!
//! Its flats at `d`. A foot's prism has the section `d²/(2√2)`: the front's
//! runs the 30, less the `d³/12` each end's slope spares of it, and each
//! end's the 20, less the `d³/12` the front spares. A hip's has `√3·d²/4`,
//! every line of it `10√3` long between the floor and the top. At a height
//! `z`, measured level in from the left end, `X = x − z`, and in from the
//! front, `Y = y − z`, the feet's flats take `Y < d − √2·z` and
//! `X < d − √2·z`, the hip's `X + Y < √(3/2)·d`, and the triangle through
//! the three crossings, on the plane `x + y + (2√2 − √3 − 2)z = 2d`,
//! `X + Y < 2d − (2√2 − √3)z`. Up to `z = d/√2`, where the feet's flats run
//! out and the triangle's plane meets the hip's flat, the slice taken holds
//! the two feet's strips, their square `(d − √2·z)²` once, and beyond them
//! the triangle `X, Y ≥ d − √2·z` of `3z²/2`, the hip's `3d²/4` lying in it
//! whole: each corner gives back to the prisms the integral over that height
//! of `3d²/4 + (d − √2·z)² − 3z²/2`, `5d³/(6√2)`. The body keeps
//! `9500/3 − d²(15 + 35/√2) + d³(2 + 5√2)/6`, 3128.429773960448 at 1, which
//! the booleans give to the last digits — the five flats' prisms cut away,
//! then the slab beyond each triangle's plane.
//!
//! The campaigns are run by hand, out of the gate, compiled only under the
//! `campaigns` feature: the fillet on the square pyramid's four hips and the
//! four edges of its base in all 48 of the orders the chamfer is held in,
//! and the fillet on the hipped block's five edges in all 120 of theirs,
//! where the gate takes every fourth of each, each held as the gate holds its
//! own —
//!
//! ```text
//! cargo test --release -p cao_solid --features campaigns \
//!     --test a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at -- --ignored
//! ```

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Loop};
use glam::{DMat3, DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it, on a plane:
/// each is placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// The numbers a raised block's faces answer to: its floor nought, its top
/// one, and its sides from the front round to the left, two to five.
const FLOOR: usize = 0;
const TOP: usize = 1;
const FRONT: usize = 2;
const RIGHT: usize = 3;
const BACK: usize = 4;
const LEFT: usize = 5;

/// The block the corners are held on: 30 along X, 10 across, 20 high — three
/// lengths apart, so that each edge of a corner is told by its length.
const LENGTH: f64 = 30.0;
const BREADTH: f64 = 10.0;
const HEIGHT: f64 = 20.0;
const BLOCK_VOLUME: f64 = LENGTH * BREADTH * HEIGHT;

/// The two edges a step names among a corner's three, and the third it leaves,
/// as indices into [`Corner`]'s edges: each of the three pairs, in either
/// order.
const TWO_OF_THREE: [[usize; 3]; 6] = [
    [0, 1, 2],
    [1, 0, 2],
    [0, 2, 1],
    [2, 0, 1],
    [1, 2, 0],
    [2, 1, 0],
];

/// The ground's plane, which a part is drawn on.
fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

/// An outline with its runs, as the drawing hands it to the exact kernel.
fn profile(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

/// An outline drawn on the ground raised `height` as `cao_part` builds a
/// step: its floor numbered nought, its top one, then its walls, one for each
/// run of the outline.
fn raised(outline: Contour, height: f64) -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(&profile(outline), ground(), DVec3::Z * height)
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// A block of a length, a breadth and a height standing on the ground's
/// origin, its sides numbered two to five from the front round to the left.
fn block([length, breadth, height]: [f64; 3]) -> Body {
    raised(
        Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth)),
        height,
    )
}

/// A corner where three edges of a block meet, square: where it stands, and
/// for each of its three edges the way it runs from the corner, how long it
/// is, the face across from it — the one of the corner's three faces it does
/// not lie on —, the face it ends on at its far end, and what it is called.
struct Corner {
    at: DVec3,
    along: [DVec3; 3],
    length: [f64; 3],
    across: [usize; 3],
    far: [usize; 3],
    called: [&'static str; 3],
}

/// The block's top front right corner: the top's front edge running 30 to the
/// left wall, the top's right edge 10 to the back, and the front's right edge
/// 20 down to the floor.
fn top_front_right() -> Corner {
    Corner {
        at: DVec3::new(LENGTH, 0.0, HEIGHT),
        along: [-DVec3::X, DVec3::Y, -DVec3::Z],
        length: [LENGTH, BREADTH, HEIGHT],
        across: [RIGHT, FRONT, TOP],
        far: [LEFT, BACK, FLOOR],
        called: [
            "the top's front edge",
            "the top's right edge",
            "the front's right edge",
        ],
    }
}

impl Corner {
    /// The point `by` from the corner along the edges `order` names: the first
    /// length along the first, the second along the second, the third along
    /// the third.
    fn point(&self, order: [usize; 3], by: [f64; 3]) -> DVec3 {
        order
            .into_iter()
            .zip(by)
            .fold(self.at, |point, (edge, length)| {
                point + self.along[edge] * length
            })
    }

    /// How far a point stands from the corner along the edge `edge`.
    fn along_edge(&self, point: DVec3, edge: usize) -> f64 {
        (point - self.at).dot(self.along[edge])
    }

    /// The round a fillet of `radius` lays on the edge `edge`, whole: its axis
    /// the radius in from the edge's two faces — the one it shares with the
    /// edge `beside` and the one across from `beside` — and running from the
    /// corner's plane to the edge's far end, turning from facing the way the
    /// shared face does to facing the way the other does.
    fn round(&self, edge: usize, [beside, shared]: [usize; 2], radius: f64) -> Round {
        Round {
            start: self.at + (self.along[beside] + self.along[shared]) * radius,
            along: self.along[edge],
            length: self.length[edge],
            radius,
            facing: [-self.along[shared], -self.along[beside]],
        }
    }

    /// The name of the corner's edge `edge` on the body: the edge between the
    /// two faces across from the corner's two other edges.
    fn name(&self, body: &Body, edge: usize) -> EdgeName {
        let [one, other] = [(edge + 1) % 3, (edge + 2) % 3];
        edge_between(body, self.across[one], self.across[other])
    }
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let named = edges_between(body, first, second);
    assert_eq!(
        named.len(),
        1,
        "one edge between {first} and {second}: {named:?}"
    );
    named[0].clone()
}

/// The names of every edge between the faces numbered `first` and `second`.
fn edges_between(body: &Body, first: usize, second: usize) -> Vec<EdgeName> {
    body.edge_names()
        .into_iter()
        .filter(|name| lies_between(name, first, second))
        .collect()
}

/// Whether a name is that of an edge between the faces numbered `first` and
/// `second`, whichever it lists first.
fn lies_between(name: &EdgeName, first: usize, second: usize) -> bool {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    name.between == sides || name.between == [other, one]
}

/// Every number some face of the body answers to: a step's, below where the
/// steps' numbers end, and a piece's, past them.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&number| body.has_face(number))
        .collect()
}

/// The numbers of the faces a step made: those the body answers to that the
/// part it was given did not.
fn faces_made(part: &Body, body: &Body) -> Vec<usize> {
    numbers(body).difference(&numbers(part)).copied().collect()
}

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    for corner in &plane.corners {
        assert!(
            expected.iter().any(|point| point.distance(*corner) < PLACE),
            "{what}: a corner at {corner}, where the corners are {expected:?}"
        );
    }
    for point in expected {
        assert!(
            plane
                .corners
                .iter()
                .any(|corner| corner.distance(*point) < PLACE),
            "{what}: no corner at {point}, among {:?}",
            plane.corners
        );
    }
}

/// Holds that the face numbered `face` is flat and faces the way `facing`
/// points.
fn faces_toward(body: &Body, face: usize, facing: DVec3, what: &str) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    assert!(
        plane.normal.distance(facing) < PLACE,
        "{what}: face {face} faces {}, not {facing}",
        plane.normal
    );
}

/// The sides of the faces the face numbered `face` meets, one for each edge
/// it has, in order.
fn sides_met(body: &Body, face: usize) -> Vec<Vec<u32>> {
    sides_met_by(body, &[face])
}

/// The sides of the faces the face answering to all of `numbers`, and to
/// nothing else, meets, one for each edge it has, in order.
fn sides_met_by(body: &Body, numbers: &[usize]) -> Vec<Vec<u32>> {
    let own: Vec<u32> = numbers.iter().map(|&number| number as u32).collect();
    let mut met: Vec<Vec<u32>> = body
        .edge_names()
        .into_iter()
        .filter_map(|name| {
            let [first, second] = name.between;
            if first == own {
                Some(second)
            } else if second == own {
                Some(first)
            } else {
                None
            }
        })
        .collect();
    met.sort();
    met
}

/// The sides a face is expected to meet, one edge each, in the order
/// [`sides_met`] lists them.
fn sides(faces: &[usize]) -> Vec<Vec<u32>> {
    let mut sorted: Vec<Vec<u32>> = faces.iter().map(|&face| vec![face as u32]).collect();
    sorted.sort();
    sorted
}

/// The sides a face is expected to meet, one edge each, as [`sides`] lists
/// them, each side every number the face across it answers to: a full round
/// answers to both its edges'.
fn sides_answering(faces: &[&[usize]]) -> Vec<Vec<u32>> {
    let mut sorted: Vec<Vec<u32>> = faces
        .iter()
        .map(|numbers| numbers.iter().map(|&number| number as u32).collect())
        .collect();
    sorted.sort();
    sorted
}

/// Holds that a body is exact and keeps the rules every solid must: its
/// surface closed, no face through another, the matter it holds what
/// arithmetic promised, and the matter its triangles enclose within `drawn`
/// of it.
fn sound_drawn_within(body: &Body, promised: f64, drawn: f64, what: &str) {
    assert!(
        body.is_exact(),
        "{what}: the exact kernel computed the body"
    );
    let triangles = body.triangles();
    assert_eq!(closed(&triangles), Ok(()), "{what}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
    assert!(
        (body.volume() - promised).abs() <= EXACT * promised,
        "{what}: the body holds {}, where arithmetic promised {promised}",
        body.volume()
    );
    let enclosed = enclosed(&triangles);
    assert!(
        (enclosed - promised).abs() <= drawn,
        "{what}: its triangles enclose {enclosed}, where arithmetic promised {promised}"
    );
}

/// Holds that a step reports the value it was asked, and took it whole.
fn taken_as_asked(asked: f64, taken: f64, value: f64, what: &str) {
    assert!(
        (asked - value).abs() < PLACE && (taken - value).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {value} fits"
    );
}

/// What a block keeps once two of a square corner's edges, `lengths` long,
/// are chamfered at `distance` and mitred: each flat's prism taken whole, and
/// the pyramid where both cut given back once, as the module's documentation
/// derives it.
fn mitred_by_flats(lengths: [f64; 2], distance: f64) -> f64 {
    let [first, second] = lengths;
    BLOCK_VOLUME - distance * distance * (first + second) / 2.0 + distance.powi(3) / 3.0
}

#[test]
fn a_chamfer_on_two_of_a_corner_s_three_edges_mitres_their_flats_and_leaves_the_third_edge_sharp_one_distance_short()
 {
    let distance = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let corner = top_front_right();
    let steps_end = block.faces_end();

    for [first, second, third] in TWO_OF_THREE {
        let what = format!(
            "the chamfer on {} then {}, {} left",
            corner.called[first], corner.called[second], corner.called[third]
        );
        let names = [corner.name(&block, first), corner.name(&block, second)];

        let chamfered = block
            .chamfer_edges(&names, distance)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_as_asked(chamfered.asked, chamfered.taken, distance, &what);
        let body = &chamfered.body;
        let [long_first, long_second, long_third] =
            [first, second, third].map(|edge| corner.length[edge]);
        let promised = mitred_by_flats([long_first, long_second], distance);
        sound_drawn_within(body, promised, EXACT * promised, &what);
        let [flat_first, flat_second] = [steps_end, steps_end + 1];
        assert_eq!(
            faces_made(&block, body),
            vec![flat_first, flat_second],
            "{what}: one flat for each edge, numbered in the order the step names them, and no \
             face laid at the corner: {:?}",
            numbers(body)
        );
        assert!(
            numbers(&block).is_subset(&numbers(body)),
            "{what}: a face of the block went: {:?}",
            numbers(body)
        );
        let at = |by: [f64; 3]| corner.point([first, second, third], by);
        let d = distance;
        let [across_first, across_second, shared] =
            [first, second, third].map(|edge| corner.across[edge]);
        let [far_first, far_second, far_third] =
            [first, second, third].map(|edge| corner.far[edge]);
        let crossing = at([d, d, 0.0]);
        let short_of_the_corner = at([0.0, 0.0, d]);

        corners_are(
            body,
            flat_first,
            &[
                at([long_first, d, 0.0]),
                at([long_first, 0.0, d]),
                short_of_the_corner,
                crossing,
            ],
            &format!(
                "{what}: the flat of the first edge, from its far end to the mitre, which runs \
                 from where the two flats cross the shared face down to the third edge"
            ),
        );
        faces_toward(
            body,
            flat_first,
            -(corner.along[second] + corner.along[third]).normalize(),
            &format!("{what}: the flat of the first edge"),
        );
        assert_eq!(
            sides_met(body, flat_first),
            sides(&[shared, across_second, far_first, flat_second]),
            "{what}: the flat of the first edge meets its edge's two faces along its sides, the \
             face its edge ends on, and the other flat along the mitre, by one edge each"
        );
        corners_are(
            body,
            flat_second,
            &[
                at([d, long_second, 0.0]),
                at([0.0, long_second, d]),
                short_of_the_corner,
                crossing,
            ],
            &format!("{what}: the flat of the second edge, from its far end to the mitre"),
        );
        faces_toward(
            body,
            flat_second,
            -(corner.along[first] + corner.along[third]).normalize(),
            &format!("{what}: the flat of the second edge"),
        );
        assert_eq!(
            sides_met(body, flat_second),
            sides(&[shared, across_first, far_second, flat_first]),
            "{what}: the flat of the second edge meets its edge's two faces along its sides, the \
             face its edge ends on, and the other flat along the mitre, by one edge each"
        );
        corners_are(
            body,
            shared,
            &[
                crossing,
                at([long_first, d, 0.0]),
                at([long_first, long_second, 0.0]),
                at([d, long_second, 0.0]),
            ],
            &format!(
                "{what}: the face the two edges share, set back by the distance from each, its \
                 corner at the crossing"
            ),
        );
        assert_eq!(
            sides_met(body, shared),
            sides(&[flat_first, flat_second, far_first, far_second]),
            "{what}: the face the two edges share meets the two flats, and no longer the third \
             edge's faces"
        );
        let third_s_far_end = at([0.0, 0.0, long_third]);
        for (face, other, [beside, beyond], far, flat, which) in [
            (
                across_second,
                across_first,
                [at([long_first, 0.0, d]), at([long_first, 0.0, long_third])],
                far_first,
                flat_first,
                "the face the first edge and the third share",
            ),
            (
                across_first,
                across_second,
                [
                    at([0.0, long_second, d]),
                    at([0.0, long_second, long_third]),
                ],
                far_second,
                flat_second,
                "the face the second edge and the third share",
            ),
        ] {
            corners_are(
                body,
                face,
                &[short_of_the_corner, beside, beyond, third_s_far_end],
                &format!(
                    "{what}: {which}, set back by the distance from the flat, the third edge \
                     along it ending one distance short of the corner"
                ),
            );
            assert_eq!(
                sides_met(body, face),
                sides(&[flat, far, far_third, other]),
                "{what}: {which} meets its flat, the faces the edges end on, and the third \
                 edge's other face, sharp"
            );
        }
        assert_eq!(
            edges_between(body, across_first, across_second).len(),
            1,
            "{what}: {} is still one sharp edge, from where it ends one distance short of the \
             corner to its far end",
            corner.called[third]
        );
    }
}

/// What a block keeps once two of a square corner's edges, `lengths` long,
/// are rounded at `radius` and mitred: each round's strip taken whole, and the
/// overlap where both take given back once, as the module's documentation
/// derives it.
fn mitred_by_rounds(lengths: [f64; 2], radius: f64) -> f64 {
    let [first, second] = lengths;
    BLOCK_VOLUME - radius * radius * (1.0 - PI / 4.0) * (first + second)
        + radius.powi(3) * (5.0 / 3.0 - PI / 2.0)
}

/// What a block keeps once the three edges of a square corner, `lengths`
/// long, are chamfered at `distance` and closed by a triangle: each flat's
/// prism taken whole, less what they share inside the cube of side
/// `distance` at the corner, as the module's documentation derives it.
fn closed_by_a_triangle(lengths: [f64; 3], distance: f64) -> f64 {
    let along: f64 = lengths.iter().sum();
    BLOCK_VOLUME - distance * distance * along / 2.0 + 2.0 * distance.powi(3) / 3.0
}

/// What a block keeps once the three edges of a square corner, `lengths`
/// long, are rounded at `radius` and closed by a piece of sphere: each
/// round's strip taken whole, less what they share inside the cube of side
/// `radius` at the corner but the eighth of the ball, as the module's
/// documentation derives it.
fn closed_by_a_sphere(lengths: [f64; 3], radius: f64) -> f64 {
    let along: f64 = lengths.iter().sum();
    BLOCK_VOLUME - radius * radius * (1.0 - PI / 4.0) * along
        + radius.powi(3) * (2.0 - 7.0 * PI / 12.0)
}

/// Every order three things can be named in.
const ORDERS_OF_THREE: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

/// What the block keeps once its four top edges are chamfered at `distance`,
/// mitred at its four top corners: each flat's prism taken whole, and at each
/// corner the pyramid where two cut given back once, as the module's
/// documentation derives it.
fn ringed_by_flats(distance: f64) -> f64 {
    BLOCK_VOLUME - distance * distance * (2.0 * LENGTH + 2.0 * BREADTH) / 2.0
        + 4.0 * distance.powi(3) / 3.0
}

/// What the block keeps once its four top edges are rounded at `radius`,
/// mitred at its four top corners, as [`ringed_by_flats`] counts it.
fn ringed_by_rounds(radius: f64) -> f64 {
    BLOCK_VOLUME - radius * radius * (1.0 - PI / 4.0) * (2.0 * LENGTH + 2.0 * BREADTH)
        + 4.0 * radius.powi(3) * (5.0 / 3.0 - PI / 2.0)
}

/// An edge of the block's top: the wall it runs along and the way that wall
/// faces, the corner it starts at, the way it runs from it — round the top
/// anticlockwise, seen from above — and how long it is.
struct TopEdge {
    wall: usize,
    facing: DVec3,
    start: DVec3,
    along: DVec3,
    length: f64,
}

/// The block's four top edges, round the top from the front: the front, the
/// right, the back and the left.
fn top_edges() -> [TopEdge; 4] {
    [
        TopEdge {
            wall: FRONT,
            facing: -DVec3::Y,
            start: DVec3::new(0.0, 0.0, HEIGHT),
            along: DVec3::X,
            length: LENGTH,
        },
        TopEdge {
            wall: RIGHT,
            facing: DVec3::X,
            start: DVec3::new(LENGTH, 0.0, HEIGHT),
            along: DVec3::Y,
            length: BREADTH,
        },
        TopEdge {
            wall: BACK,
            facing: DVec3::Y,
            start: DVec3::new(LENGTH, BREADTH, HEIGHT),
            along: -DVec3::X,
            length: LENGTH,
        },
        TopEdge {
            wall: LEFT,
            facing: -DVec3::X,
            start: DVec3::new(0.0, BREADTH, HEIGHT),
            along: -DVec3::Y,
            length: BREADTH,
        },
    ]
}

impl TopEdge {
    /// The corner it ends at.
    fn end(&self) -> DVec3 {
        self.start + self.along * self.length
    }

    /// The edge's name on the part.
    fn name(&self, part: &Body) -> EdgeName {
        edge_between(part, TOP, self.wall)
    }

    /// Where a cut set back by `value` on the top from this edge crosses the
    /// cut of the edge before it, at its start, and of the edge after it, at
    /// its end.
    fn crossings(&self, value: f64) -> [DVec3; 2] {
        let into = -self.facing * value;
        [
            self.start + self.along * value + into,
            self.end() - self.along * value + into,
        ]
    }

    /// The flat a chamfer of `distance` lays on the edge, mitred at both its
    /// ends: from the two crossings on the top down to the two uprights, one
    /// distance below the top, and the way it faces.
    fn flat(&self, distance: f64) -> ([DVec3; 4], DVec3) {
        let [at_start, at_end] = self.crossings(distance);
        let down = DVec3::Z * distance;
        (
            [at_start, at_end, self.end() - down, self.start - down],
            (self.facing + DVec3::Z).normalize(),
        )
    }

    /// The round a fillet of `radius` lays on the edge, whole: its axis the
    /// radius in from the top and from the wall, along the whole edge.
    fn round(&self, radius: f64) -> Round {
        Round {
            start: self.start - (self.facing + DVec3::Z) * radius,
            along: self.along,
            length: self.length,
            radius,
            facing: [DVec3::Z, self.facing],
        }
    }

    /// Whether a point stands on this edge's side of the plane halfway between
    /// it and the edge before it, at its start, and of the plane halfway
    /// between it and the edge after it, at its end: no nearer either corner
    /// along this edge than in from this edge's wall.
    fn between_its_mitres(&self, point: DVec3) -> bool {
        let inwards = |corner: DVec3| (point - corner).dot(-self.facing);
        (point - self.start).dot(self.along) >= inwards(self.start) - PLACE
            && (self.end() - point).dot(self.along) >= inwards(self.end()) - PLACE
    }
}

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
}

/// The round a fillet is expected to lay: a cylinder of `radius` about the
/// line from `start` along `along` for `length`, turning about it from facing
/// the way the edge's first face does to facing the way its second does.
/// Facing each face's way where it meets it is what tangent to both comes to.
struct Round {
    start: DVec3,
    along: DVec3,
    length: f64,
    radius: f64,
    facing: [DVec3; 2],
}

impl Round {
    /// The point of the axis `share` of the way along it.
    fn axis_at(&self, share: f64) -> DVec3 {
        self.start + self.along * self.length * share
    }

    /// The way the round faces `share` of the way round it, from its first
    /// face's way to its second's.
    fn facing_at(&self, share: f64) -> DVec3 {
        let [first, second] = self.facing;
        let turn = first.dot(second).acos() * share;
        let across = (second - first * first.dot(second)).normalize();
        first * turn.cos() + across * turn.sin()
    }

    /// Whether a point lies on the round: on its cylinder, between its two
    /// ends, and between its two sides.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.start;
        let along = off.dot(self.along);
        let across = off - self.along * along;
        let [first, second] = self.facing;
        let least = self.radius * first.dot(second);
        (across.length() - self.radius).abs() < PLACE
            && (-PLACE..=self.length + PLACE).contains(&along)
            && across.dot(first) >= least - PLACE
            && across.dot(second) >= least - PLACE
    }

    /// The part of the round from `from` along its axis to `to`.
    fn part(&self, from: f64, to: f64) -> Round {
        Round {
            start: self.start + self.along * from,
            length: to - from,
            ..*self
        }
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// Holds that every ray aimed square at the round's axis from outside, a
/// tenth to nine tenths of the way along it, meets the face numbered `face`
/// first, where it faces away from the axis along the ray.
fn met_from_outside(body: &Body, face: usize, round: &Round, what: &str) {
    for along in [0.11, 0.47, 0.83] {
        for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let facing = round.facing_at(share);
            let from = round.axis_at(along) + facing * (round.radius + 10.0);
            let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
                panic!("{what}: a ray from {from} toward the axis meets nothing")
            });
            assert_eq!(
                hit.face, face,
                "{what}: a ray from {from} toward the axis meets face {} before the round",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{what}: where a ray from {from} meets it, the round faces {}, not {facing}, \
                 away from its axis",
                hit.normal
            );
        }
    }
}

/// The part with a block between the corners `from` and `to`, raised from
/// `from_height` by `height`, joined to it as `cao_part` joins a step: the
/// tool's faces numbered past the part's, its floor first, then its top, then
/// its sides from the front round to the left.
fn joined_to(part: &Body, [from, to]: [DVec2; 2], from_height: f64, height: f64) -> Body {
    part.union(&tool_between(part, [from, to], from_height, height))
        .expect("the join is built")
}

/// The block a step raises between the corners `from` and `to`, from
/// `from_height` by `height`.
fn tool_between(part: &Body, [from, to]: [DVec2; 2], from_height: f64, height: f64) -> Body {
    part.tool_raised(
        &profile(Contour::rectangle(from, to)),
        Frame {
            origin: DVec3::Z * from_height,
            ..ground()
        },
        DVec3::Z * height,
    )
    .expect("the tool raises")
}

#[test]
fn a_fillet_on_two_of_a_corner_s_three_edges_mitres_their_rounds_on_an_ellipse_and_leaves_the_third_edge_sharp_one_radius_short()
 {
    let radius = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let corner = top_front_right();
    let steps_end = block.faces_end();

    for [first, second, third] in TWO_OF_THREE {
        let what = format!(
            "the fillet on {} then {}, {} left",
            corner.called[first], corner.called[second], corner.called[third]
        );
        let names = [corner.name(&block, first), corner.name(&block, second)];

        let filleted = block
            .fillet_edges(&names, radius)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_as_asked(filleted.asked, filleted.taken, radius, &what);
        let body = &filleted.body;
        let [long_first, long_second, long_third] =
            [first, second, third].map(|edge| corner.length[edge]);
        let rounds = [
            corner.round(first, [second, third], radius),
            corner.round(second, [first, third], radius),
        ];
        let area: f64 = rounds.iter().map(Round::area).sum();
        sound_drawn_within(
            body,
            mitred_by_rounds([long_first, long_second], radius),
            DRAWN * reach_of(&block) * area,
            &what,
        );
        let [round_first, round_second] = [steps_end, steps_end + 1];
        assert_eq!(
            faces_made(&block, body),
            vec![round_first, round_second],
            "{what}: one round for each edge, numbered in the order the step names them, and no \
             face laid at the corner: {:?}",
            numbers(body)
        );
        assert!(
            numbers(&block).is_subset(&numbers(body)),
            "{what}: a face of the block went: {:?}",
            numbers(body)
        );
        let at = |by: [f64; 3]| corner.point([first, second, third], by);
        let r = radius;
        let [across_first, across_second, shared] =
            [first, second, third].map(|edge| corner.across[edge]);
        let [far_first, far_second, far_third] =
            [first, second, third].map(|edge| corner.far[edge]);

        for (face, round, own, other, sides_expected, which) in [
            (
                round_first,
                &rounds[0],
                first,
                second,
                sides(&[shared, across_second, far_first, round_second]),
                "the round of the first edge",
            ),
            (
                round_second,
                &rounds[1],
                second,
                first,
                sides(&[shared, across_first, far_second, round_first]),
                "the round of the second edge",
            ),
        ] {
            assert!(
                !body.is_flat(face) && body.plane_of(face).is_none(),
                "{what}: {which} is curved, and offers no plane to draw on"
            );
            let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
            assert!(!corners.is_empty(), "{what}: {which} is drawn");
            for point in corners {
                assert!(
                    round.holds(point),
                    "{what}: {which} is drawn through {point}, off its cylinder of {r} about the \
                     line from {} along {}",
                    round.start,
                    round.along
                );
                assert!(
                    corner.along_edge(point, own) >= corner.along_edge(point, other) - PLACE,
                    "{what}: {which} is drawn through {point}, past the plane halfway between the \
                     two edges, where the other round meets it in the mitre"
                );
            }
            met_from_outside(
                body,
                face,
                &round.part(r, round.length),
                &format!("{what}: {which}, past the mitre"),
            );
            assert_eq!(
                sides_met(body, face),
                sides_expected,
                "{what}: {which} meets its edge's two faces along its sides, the face its edge \
                 ends on, and the other round along the mitre, by one edge each"
            );
        }
        let short_of_the_corner = at([0.0, 0.0, r]);
        corners_are(
            body,
            shared,
            &[
                at([r, r, 0.0]),
                at([long_first, r, 0.0]),
                at([long_first, long_second, 0.0]),
                at([r, long_second, 0.0]),
            ],
            &format!(
                "{what}: the face the two edges share, set back by the radius from each, its \
                 corner where the two rounds' sides cross"
            ),
        );
        assert_eq!(
            sides_met(body, shared),
            sides(&[round_first, round_second, far_first, far_second]),
            "{what}: the face the two edges share meets the two rounds, and no longer the third \
             edge's faces"
        );
        let third_s_far_end = at([0.0, 0.0, long_third]);
        for (face, other, [beside, beyond], far, round, which) in [
            (
                across_second,
                across_first,
                [at([long_first, 0.0, r]), at([long_first, 0.0, long_third])],
                far_first,
                round_first,
                "the face the first edge and the third share",
            ),
            (
                across_first,
                across_second,
                [
                    at([0.0, long_second, r]),
                    at([0.0, long_second, long_third]),
                ],
                far_second,
                round_second,
                "the face the second edge and the third share",
            ),
        ] {
            corners_are(
                body,
                face,
                &[short_of_the_corner, beside, beyond, third_s_far_end],
                &format!(
                    "{what}: {which}, set back by the radius from the round, the third edge \
                     along it ending one radius short of the corner"
                ),
            );
            assert_eq!(
                sides_met(body, face),
                sides(&[round, far, far_third, other]),
                "{what}: {which} meets its round, the faces the edges end on, and the third \
                 edge's other face, sharp"
            );
        }
        assert_eq!(
            edges_between(body, across_first, across_second).len(),
            1,
            "{what}: {} is still one sharp edge, from where it ends one radius short of the \
             corner to its far end",
            corner.called[third]
        );
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_the_four_top_edges_of_a_block_mitre_its_four_corners() {
    let value = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let edges = top_edges();
    let steps_end = block.faces_end();

    for (order, called) in [
        ([0, 1, 2, 3], "named round the top"),
        ([0, 2, 1, 3], "named the front and the back first"),
    ] {
        let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].name(&block)).collect();

        let chamfered = block
            .chamfer_edges(&names, value)
            .unwrap_or_else(|declined| panic!("the chamfer, {called}: declined, {declined:?}"));
        let filleted = block
            .fillet_edges(&names, value)
            .unwrap_or_else(|declined| panic!("the fillet, {called}: declined, {declined:?}"));

        let laid_for = |edge: usize| {
            steps_end
                + order
                    .iter()
                    .position(|&named| named == edge)
                    .expect("every top edge is named")
        };
        let laid: Vec<usize> = (0..4).map(laid_for).collect();
        let rounds: Vec<Round> = edges.iter().map(|edge| edge.round(value)).collect();
        let area: f64 = rounds.iter().map(Round::area).sum();
        for (what, asked, taken, body, promised, drawn) in [
            (
                format!("the chamfer on the four top edges, {called}"),
                chamfered.asked,
                chamfered.taken,
                &chamfered.body,
                ringed_by_flats(value),
                EXACT * ringed_by_flats(value),
            ),
            (
                format!("the fillet on the four top edges, {called}"),
                filleted.asked,
                filleted.taken,
                &filleted.body,
                ringed_by_rounds(value),
                DRAWN * reach_of(&block) * area,
            ),
        ] {
            taken_as_asked(asked, taken, value, &what);
            sound_drawn_within(body, promised, drawn, &what);
            let mut made = laid.clone();
            made.sort();
            assert_eq!(
                faces_made(&block, body),
                made,
                "{what}: one face for each edge, numbered in the order the step names them, and \
                 no face laid at a corner: {:?}",
                numbers(body)
            );
            assert!(
                numbers(&block).is_subset(&numbers(body)),
                "{what}: a face of the block went: {:?}",
                numbers(body)
            );
            let top: Vec<DVec3> = edges.iter().map(|edge| edge.crossings(value)[0]).collect();
            corners_are(
                body,
                TOP,
                &top,
                &format!("{what}: the top, set back by the value from its four edges"),
            );
            assert_eq!(
                sides_met(body, TOP),
                sides(&laid),
                "{what}: the top meets the four faces laid, and no wall"
            );
            for (index, edge) in edges.iter().enumerate() {
                let [before, after] = [(index + 3) % 4, (index + 1) % 4];
                let down = DVec3::Z * value;
                let floor = DVec3::Z * HEIGHT;
                corners_are(
                    body,
                    edge.wall,
                    &[
                        edge.start - down,
                        edge.end() - down,
                        edge.end() - floor,
                        edge.start - floor,
                    ],
                    &format!(
                        "{what}: wall {}, set back by the value from the top, its uprights \
                         ending one value below it",
                        edge.wall
                    ),
                );
                assert_eq!(
                    sides_met(body, edge.wall),
                    sides(&[FLOOR, laid[index], edges[before].wall, edges[after].wall]),
                    "{what}: wall {} meets the floor, the face laid on its top edge, and the \
                     two walls beside it along their uprights, sharp",
                    edge.wall
                );
                assert_eq!(
                    sides_met(body, laid[index]),
                    sides(&[TOP, edge.wall, laid[before], laid[after]]),
                    "{what}: the face laid on wall {}'s top edge meets the top and the wall \
                     along its sides, and the faces laid beside it along the mitres at its two \
                     ends",
                    edge.wall
                );
            }
        }
        for (index, edge) in edges.iter().enumerate() {
            let which = format!(
                "the chamfer on the four top edges, {called}: the flat on wall {}'s top edge",
                edge.wall
            );
            let (corners, facing) = edge.flat(value);
            corners_are(&chamfered.body, laid[index], &corners, &which);
            faces_toward(&chamfered.body, laid[index], facing, &which);
        }
        for (index, (edge, round)) in edges.iter().zip(&rounds).enumerate() {
            let which = format!(
                "the fillet on the four top edges, {called}: the round on wall {}'s top edge",
                edge.wall
            );
            let body = &filleted.body;
            let face = laid[index];
            assert!(
                !body.is_flat(face) && body.plane_of(face).is_none(),
                "{which} is curved, and offers no plane to draw on"
            );
            let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
            assert!(!corners.is_empty(), "{which} is drawn");
            for point in corners {
                assert!(
                    round.holds(point),
                    "{which} is drawn through {point}, off its cylinder of {value} about the \
                     line from {} along {}",
                    round.start,
                    round.along
                );
                assert!(
                    edge.between_its_mitres(point),
                    "{which} is drawn through {point}, past a plane halfway between its edge and \
                     the one beside it, where the round beside it meets it in a mitre"
                );
            }
            met_from_outside(
                body,
                face,
                &round.part(value, round.length - value),
                &format!("{which}, between its mitres"),
            );
        }
    }
}

/// The section of a prism lying along X, drawn on the plane of Y and Z: its
/// bottom 16 wide, its top 10 wide, from 2 to 12 at a height of 10, and its
/// two sides slanting in. Raised along X for [`LENGTH`], its numbers are its
/// start at x = 0 nought, its end one, then its bottom two, its right side
/// three, its top four and its left side five. A prism of a convex section,
/// every edge of it meets outwards.
const SLANTED: [DVec2; 4] = [
    DVec2::new(0.0, 0.0),
    DVec2::new(16.0, 0.0),
    DVec2::new(12.0, 10.0),
    DVec2::new(2.0, 10.0),
];
const SLANTED_START: usize = 0;
const SLANTED_END: usize = 1;
const SLANTED_BOTTOM: usize = 2;
const SLANTED_RIGHT: usize = 3;
const SLANTED_TOP: usize = 4;
const SLANTED_LEFT: usize = 5;

/// The slanted prism's matter: its section's area along its length.
const SLANTED_VOLUME: f64 = (16.0 + 10.0) / 2.0 * 10.0 * LENGTH;

/// The corner where the slanted prism's end, its top and its right side meet.
const SLANTED_CORNER: DVec3 = DVec3::new(LENGTH, 12.0, 10.0);

/// The [`SLANTED`] section raised along X for [`LENGTH`].
fn slanted_prism() -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(
            &profile(Contour::straight(SLANTED.to_vec())),
            Frame {
                origin: DVec3::ZERO,
                u: DVec3::Y,
                v: DVec3::Z,
            },
            DVec3::X * LENGTH,
        )
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// The way the slanted prism's right side faces.
fn side_facing() -> DVec3 {
    DVec3::new(0.0, 10.0, 4.0).normalize()
}

/// The way the slanted prism's right side runs down from its top.
fn side_down() -> DVec3 {
    DVec3::new(0.0, 4.0, -10.0).normalize()
}

/// What a step lays on an edge: a flat or a round.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Cut {
    Flat,
    Round,
}

/// An edge at a corner: the corner it runs from, the two faces it lies
/// between, the way it runs from the corner, for each face the way into it
/// from the edge, square to the edge, and the way it faces, and how far before
/// and past the corner a prism raised along it must reach to take all a cut of
/// it takes.
struct EdgeAt {
    called: &'static str,
    at: DVec3,
    faces: [usize; 2],
    along: DVec3,
    into: [DVec3; 2],
    outward: [DVec3; 2],
    reach: [f64; 2],
}

/// The three edges at [`SLANTED_CORNER`]: the top's edge on the end, square,
/// the top's edge on the right side, at the side's slant, and the slanted
/// upright where the side meets the end, square.
fn slanted_corner_edges() -> [EdgeAt; 3] {
    [
        EdgeAt {
            called: "the top's edge on the end",
            at: SLANTED_CORNER,
            faces: [SLANTED_TOP, SLANTED_END],
            along: -DVec3::Y,
            into: [-DVec3::X, -DVec3::Z],
            outward: [DVec3::Z, DVec3::X],
            reach: [-8.0, 18.0],
        },
        EdgeAt {
            called: "the top's edge on the right side",
            at: SLANTED_CORNER,
            faces: [SLANTED_TOP, SLANTED_RIGHT],
            along: -DVec3::X,
            into: [-DVec3::Y, side_down()],
            outward: [DVec3::Z, side_facing()],
            reach: [-2.0, LENGTH + 2.0],
        },
        EdgeAt {
            called: "the slanted upright where the side meets the end",
            at: SLANTED_CORNER,
            faces: [SLANTED_END, SLANTED_RIGHT],
            along: side_down(),
            into: [-side_facing(), -DVec3::X],
            outward: [DVec3::X, side_facing()],
            reach: [-3.0, 116f64.sqrt() + 3.0],
        },
    ]
}

impl EdgeAt {
    /// The edge's name on the part.
    fn name(&self, part: &Body) -> EdgeName {
        let [first, second] = self.faces;
        edge_between(part, first, second)
    }

    /// The angle its two faces meet at, inside the matter.
    fn angle(&self) -> f64 {
        let [first, second] = self.into;
        first.dot(second).acos()
    }

    /// How far a cut of `value` sets back from the edge along each of its
    /// faces: a flat by the value, a round where it is tangent to the face.
    fn set_back(&self, cut: Cut, value: f64) -> f64 {
        match cut {
            Cut::Flat => value,
            Cut::Round => value / (self.angle() / 2.0).tan(),
        }
    }

    /// The round a fillet of `radius` lays on the edge, whole: its axis on the
    /// line halfway between the two faces, the radius from each, as far as
    /// the edge's prism reaches.
    fn round(&self, radius: f64) -> Round {
        let [first, second] = self.into;
        let halfway = (first + second).normalize();
        let [before, past] = self.reach;
        Round {
            start: self.at + halfway * radius / (self.angle() / 2.0).sin() + self.along * before,
            along: self.along,
            length: past - before,
            radius,
            facing: self.outward,
        }
    }

    /// The prism a cut of `value` on this edge alone takes away, raised along
    /// the edge's whole line and past its faces: cut from the part, it takes
    /// what the arithmetic says the cut takes.
    fn tool(&self, part: &Body, cut: Cut, value: f64) -> Body {
        let u = self.into[0];
        let v = self.along.cross(u);
        let flat = |point: DVec3| DVec2::new(point.dot(u), point.dot(v));
        let [mut into_first, mut into_second] = self.into.map(flat);
        let [mut out_first, mut out_second] = self.outward.map(flat);
        if into_first.perp_dot(into_second) > 0.0 {
            std::mem::swap(&mut into_first, &mut into_second);
            std::mem::swap(&mut out_first, &mut out_second);
        }
        let halfway = (into_first + into_second).normalize();
        let far = 2.0 * value + 2.0;
        let set_back = self.set_back(cut, value);
        let [on_first, on_second] = [into_first * set_back, into_second * set_back];
        let corners = vec![
            on_first,
            on_first + out_first * far,
            -halfway * far,
            on_second + out_second * far,
            on_second,
        ];
        let mut runs = vec![Run::Straight; corners.len()];
        if cut == Cut::Round {
            runs[4] = Run::Round {
                center: halfway * value / (self.angle() / 2.0).sin(),
                turn: -(PI - self.angle()),
            };
        }
        let [before, past] = self.reach;
        part.tool_raised(
            &profile(Contour { corners, runs }),
            Frame {
                origin: self.at + self.along * before,
                u,
                v,
            },
            self.along * (past - before),
        )
        .expect("the tool raises")
    }
}

/// The sine of the angle the slanted prism's top meets its right side at.
fn slant() -> f64 {
    10.0 / 116f64.sqrt()
}

/// What the slanted prism keeps once two edges at [`SLANTED_CORNER`] are cut
/// at `value` and mitred, where the module's documentation derives it by
/// hand: every pair of flats, and the pair of rounds on the two square edges.
/// None for the two pairs of rounds taking the top's edge on the side.
fn slanted_mitre(cut: Cut, [first, second]: [usize; 2], value: f64) -> Option<f64> {
    let k = slant();
    let flat_alone = [
        5.0 * value * value + 0.1 * value.powi(3),
        15.0 * k * value * value,
        5.0 * value * value / k,
    ];
    let round_alone = [
        10.0 * value * value * (1.0 - PI / 4.0) + 0.6 * value.powi(3) * (5.0 / 6.0 - PI / 4.0),
        0.0,
        10.0 / k * value * value * (1.0 - PI / 4.0),
    ];
    let pair = (first.min(second), first.max(second));
    match (cut, pair) {
        (Cut::Flat, (0, 2)) => Some(value.powi(3) / (3.0 * k)),
        (Cut::Flat, _) => Some(k * value.powi(3) * (0.5 - k / 6.0)),
        (Cut::Round, (0, 2)) => Some(value.powi(3) * (5.0 / 3.0 - PI / 2.0) / k),
        (Cut::Round, _) => None,
    }
    .map(|overlap| {
        let alone = match cut {
            Cut::Flat => flat_alone,
            Cut::Round => round_alone,
        };
        SLANTED_VOLUME - alone[first] - alone[second] + overlap
    })
}

/// The sides each face of a body meets, as [`sides_met`] lists them, every
/// number taken through `renumbered` first.
fn sides_renumbered(
    body: &Body,
    face: usize,
    renumbered: impl Fn(usize) -> usize,
) -> Vec<Vec<u32>> {
    let mut met: Vec<Vec<u32>> = sides_met(body, face)
        .into_iter()
        .map(|side| {
            side.into_iter()
                .map(|number| renumbered(number as usize) as u32)
                .collect()
        })
        .collect();
    met.sort();
    met
}

#[test]
fn a_chamfer_and_a_fillet_on_two_of_the_three_edges_of_a_corner_that_is_not_square_mitre_them_the_third_left_sharp()
 {
    let value = 2.0;
    let prism = slanted_prism();
    let edges = slanted_corner_edges();
    let steps_end = prism.faces_end();
    let mut cases = Vec::new();
    for order in TWO_OF_THREE {
        for cut in [Cut::Flat, Cut::Round] {
            cases.push((order, cut));
        }
    }

    let laid: Vec<_> = cases
        .iter()
        .map(|&([first, second, _], cut)| {
            let names = [edges[first].name(&prism), edges[second].name(&prism)];
            match cut {
                Cut::Flat => prism
                    .chamfer_edges(&names, value)
                    .map(|chamfered| (chamfered.body, chamfered.asked, chamfered.taken)),
                Cut::Round => prism
                    .fillet_edges(&names, value)
                    .map(|filleted| (filleted.body, filleted.asked, filleted.taken)),
            }
        })
        .collect();

    let declined: Vec<String> = cases
        .iter()
        .zip(&laid)
        .filter_map(|(&([first, second, _], cut), laid)| {
            laid.as_ref().err().map(|declined| {
                format!(
                    "{cut:?} on {} then {}: {declined:?}",
                    edges[first].called, edges[second].called
                )
            })
        })
        .collect();
    assert!(
        declined.is_empty(),
        "every pair of a convex corner's edges is mitred, the corner square or not; declined:\n{}",
        declined.join("\n")
    );
    for ((order, cut), laid) in cases.into_iter().zip(laid) {
        let [first, second, third] = order;
        let what = format!(
            "the {} on {} then {}, {} left",
            match cut {
                Cut::Flat => "chamfer",
                Cut::Round => "fillet",
            },
            edges[first].called,
            edges[second].called,
            edges[third].called
        );
        let (body, asked, taken) = laid.expect("declines are held above");
        taken_as_asked(asked, taken, value, &what);
        let fixture = prism
            .difference(&edges[first].tool(&prism, cut, value))
            .and_then(|cut_once| cut_once.difference(&edges[second].tool(&prism, cut, value)))
            .expect("the two prisms cut the fixture");
        let promised = slanted_mitre(cut, [first, second], value).unwrap_or(fixture.volume());
        let drawn = match cut {
            Cut::Flat => EXACT * promised,
            Cut::Round => {
                let area: f64 = [first, second]
                    .iter()
                    .map(|&edge| edges[edge].round(value).area())
                    .sum();
                DRAWN * reach_of(&prism) * area
            }
        };
        sound_drawn_within(&body, promised, drawn, &what);
        let laid_faces = [steps_end, steps_end + 1];
        assert_eq!(
            faces_made(&prism, &body),
            laid_faces.to_vec(),
            "{what}: one face for each edge, numbered in the order the step names them, and no \
             face laid at the corner: {:?}",
            numbers(&body)
        );
        assert!(
            numbers(&prism).is_subset(&numbers(&body)),
            "{what}: a face of the prism went: {:?}",
            numbers(&body)
        );
        for face in 0..steps_end {
            let plane = body
                .plane_of(face)
                .unwrap_or_else(|| panic!("{what}: face {face} of the prism is still flat"));
            assert!(
                plane
                    .corners
                    .iter()
                    .all(|corner| corner.distance(SLANTED_CORNER) > PLACE),
                "{what}: face {face} still has a corner at the old vertex {SLANTED_CORNER}"
            );
        }
        let third_edge = &edges[third];
        let short = [first, second]
            .iter()
            .map(|&edge| {
                edges[edge].set_back(cut, value)
                    / edges[edge].along.cross(third_edge.along).length()
            })
            .fold(0.0, f64::max);
        let end = SLANTED_CORNER + third_edge.along * short;
        let [one, other] = third_edge.faces;
        assert_eq!(
            edges_between(&body, one, other).len(),
            1,
            "{what}: {} is still one sharp edge",
            third_edge.called
        );
        for face in third_edge.faces {
            let plane = body
                .plane_of(face)
                .unwrap_or_else(|| panic!("{what}: face {face} is still flat"));
            assert!(
                plane
                    .corners
                    .iter()
                    .any(|corner| corner.distance(end) < PLACE),
                "{what}: {} ends at {end}, {short} from the corner, where the cut set back \
                 farthest along it crosses it — no corner of face {face} there, among {:?}",
                third_edge.called,
                plane.corners
            );
        }
        for (face, &edge) in laid_faces.iter().zip(&[first, second]) {
            let which = format!("{what}: the face laid on {}", edges[edge].called);
            match cut {
                Cut::Flat => {
                    let [one, other] = edges[edge].outward;
                    faces_toward(&body, *face, (one + other).normalize(), &which);
                }
                Cut::Round => {
                    assert!(
                        !body.is_flat(*face) && body.plane_of(*face).is_none(),
                        "{which} is curved, and offers no plane to draw on"
                    );
                    let round = edges[edge].round(value);
                    let corners: Vec<DVec3> = body.triangles_of(*face).flatten().collect();
                    assert!(!corners.is_empty(), "{which} is drawn");
                    for point in corners {
                        assert!(
                            round.holds(point),
                            "{which} is drawn through {point}, off its cylinder of {value} about \
                             the line from {} along {}",
                            round.start,
                            round.along
                        );
                    }
                }
            }
        }
        let made_by_fixture = faces_made(&prism, &fixture);
        let to_fixture = |number: usize| match laid_faces.iter().position(|&face| face == number) {
            Some(rank) => made_by_fixture[rank],
            None => number,
        };
        for face in numbers(&body) {
            assert_eq!(
                sides_renumbered(&body, face, to_fixture),
                sides_met(&fixture, to_fixture(face)),
                "{what}: face {face} meets the faces it meets on the prism with the two cuts' \
                 prisms taken away, whole"
            );
            if body.plane_of(face).is_some() {
                let expected = fixture
                    .plane_of(to_fixture(face))
                    .expect("the fixture's face is flat too");
                corners_are(
                    &body,
                    face,
                    &expected.corners,
                    &format!(
                        "{what}: face {face}, as on the prism with the two cuts' prisms taken \
                         away"
                    ),
                );
            }
        }
    }
}

#[test]
fn a_chamfer_on_the_three_edges_of_a_corner_closes_it_with_a_triangle_where_the_flats_edges_cross()
{
    let distance = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let corner = top_front_right();
    let steps_end = block.faces_end();
    let triangle = steps_end + 3;

    for order in ORDERS_OF_THREE {
        let what = format!(
            "the chamfer on {}, {} and {}",
            corner.called[order[0]], corner.called[order[1]], corner.called[order[2]]
        );
        let names = order.map(|edge| corner.name(&block, edge));

        let chamfered = block
            .chamfer_edges(&names, distance)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_as_asked(chamfered.asked, chamfered.taken, distance, &what);
        let body = &chamfered.body;
        sound_drawn_within(
            body,
            closed_by_a_triangle(corner.length, distance),
            EXACT * closed_by_a_triangle(corner.length, distance),
            &what,
        );
        let flat_of = |edge: usize| {
            steps_end
                + order
                    .iter()
                    .position(|&named| named == edge)
                    .expect("every edge of the corner is named")
        };
        assert_eq!(
            faces_made(&block, body),
            vec![steps_end, steps_end + 1, steps_end + 2, triangle],
            "{what}: one flat for each edge, numbered in the order the step names them, and the \
             triangle after them: {:?}",
            numbers(body)
        );
        assert_eq!(
            body.faces_end(),
            triangle + 1,
            "{what}: the next step numbers its own faces past the triangle"
        );
        assert!(
            numbers(&block).is_subset(&numbers(body)),
            "{what}: a face of the block went: {:?}",
            numbers(body)
        );
        let d = distance;
        let in_order = [0, 1, 2];
        let at = |by: [f64; 3]| corner.point(in_order, by);
        let crossings = [at([d, d, 0.0]), at([d, 0.0, d]), at([0.0, d, d])];

        corners_are(
            body,
            triangle,
            &crossings,
            &format!(
                "{what}: the triangle, its corners where the flats' edges cross on the three faces"
            ),
        );
        faces_toward(
            body,
            triangle,
            -(corner.along[0] + corner.along[1] + corner.along[2]).normalize(),
            &format!(
                "{what}: the triangle, facing away from the corner, square to none of its faces"
            ),
        );
        assert_eq!(
            sides_met(body, triangle),
            sides(&[flat_of(0), flat_of(1), flat_of(2)]),
            "{what}: the triangle meets the three flats, one side each, and nothing else"
        );
        for edge in 0..3 {
            let [one, other] = [(edge + 1) % 3, (edge + 2) % 3];
            let frame = [edge, one, other];
            let at = |by: [f64; 3]| corner.point(frame, by);
            let long = corner.length[edge];
            let flat = flat_of(edge);
            let which = format!("{what}: the flat of {}", corner.called[edge]);
            corners_are(
                body,
                flat,
                &[
                    at([long, d, 0.0]),
                    at([long, 0.0, d]),
                    at([d, 0.0, d]),
                    at([d, d, 0.0]),
                ],
                &format!(
                    "{which}, from its far end to the straight side it shares with the triangle, \
                     one distance from the corner"
                ),
            );
            faces_toward(
                body,
                flat,
                -(corner.along[one] + corner.along[other]).normalize(),
                &which,
            );
            assert_eq!(
                sides_met(body, flat),
                sides(&[
                    corner.across[one],
                    corner.across[other],
                    corner.far[edge],
                    triangle
                ]),
                "{which} meets its edge's two faces along its sides, the face its edge ends on, \
                 and the triangle, by one edge each, and no other flat"
            );
            let face = corner.across[edge];
            let at = |by: [f64; 3]| corner.point([one, other, edge], by);
            let [long_one, long_other] = [corner.length[one], corner.length[other]];
            corners_are(
                body,
                face,
                &[
                    at([d, d, 0.0]),
                    at([long_one, d, 0.0]),
                    at([long_one, long_other, 0.0]),
                    at([d, long_other, 0.0]),
                ],
                &format!(
                    "{what}: face {face}, set back by the distance from both its edges at the \
                     corner, touching the triangle at one of its corners"
                ),
            );
            assert_eq!(
                sides_met(body, face),
                sides(&[
                    flat_of(one),
                    flat_of(other),
                    corner.far[one],
                    corner.far[other]
                ]),
                "{what}: face {face} meets the flats of its two edges at the corner and the faces \
                 they end on, and not the triangle along a side"
            );
        }
        for face in numbers(body) {
            if let Some(plane) = body.plane_of(face) {
                assert!(
                    plane
                        .corners
                        .iter()
                        .all(|point| point.distance(corner.at) > PLACE),
                    "{what}: face {face} still has a corner at the old vertex {}",
                    corner.at
                );
            }
        }
    }
}

#[test]
fn a_fillet_on_the_three_edges_of_a_corner_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces()
 {
    let radius = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let corner = top_front_right();
    let steps_end = block.faces_end();
    let piece = steps_end + 3;
    let r = radius;
    let in_order = [0, 1, 2];
    let centre = corner.point(in_order, [r, r, r]);
    let rounds = [0, 1, 2].map(|edge| corner.round(edge, [(edge + 1) % 3, (edge + 2) % 3], r));
    let area: f64 = rounds.iter().map(Round::area).sum::<f64>() + PI * r * r / 2.0;

    for order in ORDERS_OF_THREE {
        let what = format!(
            "the fillet on {}, {} and {}",
            corner.called[order[0]], corner.called[order[1]], corner.called[order[2]]
        );
        let names = order.map(|edge| corner.name(&block, edge));

        let filleted = block
            .fillet_edges(&names, radius)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_as_asked(filleted.asked, filleted.taken, radius, &what);
        let body = &filleted.body;
        sound_drawn_within(
            body,
            closed_by_a_sphere(corner.length, r),
            DRAWN * reach_of(&block) * area,
            &what,
        );
        let round_of = |edge: usize| {
            steps_end
                + order
                    .iter()
                    .position(|&named| named == edge)
                    .expect("every edge of the corner is named")
        };
        assert_eq!(
            faces_made(&block, body),
            vec![steps_end, steps_end + 1, steps_end + 2, piece],
            "{what}: one round for each edge, numbered in the order the step names them, and the \
             piece of sphere after them: {:?}",
            numbers(body)
        );
        assert_eq!(
            body.faces_end(),
            piece + 1,
            "{what}: the next step numbers its own faces past the piece of sphere"
        );
        assert!(
            numbers(&block).is_subset(&numbers(body)),
            "{what}: a face of the block went: {:?}",
            numbers(body)
        );

        let which = format!("{what}: the piece of sphere");
        assert!(
            !body.is_flat(piece) && body.plane_of(piece).is_none(),
            "{which} is curved, and offers no plane to draw on"
        );
        let drawn: Vec<DVec3> = body.triangles_of(piece).flatten().collect();
        assert!(!drawn.is_empty(), "{which} is drawn");
        for point in &drawn {
            assert!(
                (point.distance(centre) - r).abs() < PLACE,
                "{which} is drawn through {point}, off the sphere of {r} about {centre}, one \
                 radius from each of the corner's three faces"
            );
            for edge in 0..3 {
                assert!(
                    corner.along_edge(*point, edge) <= r + PLACE,
                    "{which} is drawn through {point}, past the end section of the round on {}, \
                     one radius from the corner along it",
                    corner.called[edge]
                );
            }
        }
        for (face, point) in [
            (corner.across[0], corner.point(in_order, [0.0, r, r])),
            (corner.across[1], corner.point(in_order, [r, 0.0, r])),
            (corner.across[2], corner.point(in_order, [r, r, 0.0])),
        ] {
            assert!(
                drawn.iter().any(|corner| corner.distance(point) < PLACE),
                "{which}: a corner at {point}, where it touches face {face}"
            );
        }
        for weights in [
            [1.0, 1.0, 1.0],
            [2.0, 1.0, 1.0],
            [1.0, 2.0, 1.0],
            [1.0, 1.0, 2.0],
        ] {
            let facing = -(0..3)
                .map(|edge| corner.along[edge] * weights[edge])
                .sum::<DVec3>()
                .normalize();
            let from = centre + facing * (r + 10.0);
            let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
                panic!("{which}: a ray from {from} toward its centre meets nothing")
            });
            assert_eq!(
                hit.face, piece,
                "{which}: a ray from {from} toward its centre meets face {} before it",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{which}: where a ray from {from} meets it, it faces {}, not {facing}, away from \
                 its centre",
                hit.normal
            );
        }
        assert_eq!(
            sides_met(body, piece),
            sides(&[round_of(0), round_of(1), round_of(2)]),
            "{which} meets the three rounds, one side each, and nothing else"
        );

        for (edge, round) in rounds.iter().enumerate() {
            let [one, other] = [(edge + 1) % 3, (edge + 2) % 3];
            let face = round_of(edge);
            let which = format!("{what}: the round of {}", corner.called[edge]);
            assert!(
                !body.is_flat(face) && body.plane_of(face).is_none(),
                "{which} is curved, and offers no plane to draw on"
            );
            let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
            assert!(!corners.is_empty(), "{which} is drawn");
            for point in &corners {
                assert!(
                    round.holds(*point),
                    "{which} is drawn through {point}, off its cylinder of {r} about the line \
                     from {} along {}",
                    round.start,
                    round.along
                );
                assert!(
                    corner.along_edge(*point, edge) >= r - PLACE,
                    "{which} is drawn through {point}, short of its end section, one radius from \
                     the corner along its edge"
                );
            }
            let section: Vec<&DVec3> = corners
                .iter()
                .filter(|point| (corner.along_edge(**point, edge) - r).abs() < PLACE)
                .collect();
            let at = |by: [f64; 3]| corner.point([edge, one, other], by);
            for end in [at([r, 0.0, r]), at([r, r, 0.0])] {
                assert!(
                    section.iter().any(|point| point.distance(end) < PLACE),
                    "{which}: its end section, an arc in the plane through the sphere's centre \
                     square to its edge, ends at {end}, among {section:?}"
                );
            }
            assert!(
                distinct_points(&section) > 2,
                "{which}: its end section is drawn along the arc between its two ends: \
                 {section:?}"
            );
            met_from_outside(
                body,
                face,
                &round.part(r, round.length),
                &format!("{which}, past its end section"),
            );
            assert_eq!(
                sides_met(body, face),
                sides(&[
                    corner.across[one],
                    corner.across[other],
                    corner.far[edge],
                    piece
                ]),
                "{which} meets its edge's two faces along its sides, the face its edge ends on, \
                 and the piece of sphere along its end section, by one edge each, and no other \
                 round"
            );

            let across = corner.across[edge];
            let at = |by: [f64; 3]| corner.point([one, other, edge], by);
            let [long_one, long_other] = [corner.length[one], corner.length[other]];
            corners_are(
                body,
                across,
                &[
                    at([r, r, 0.0]),
                    at([long_one, r, 0.0]),
                    at([long_one, long_other, 0.0]),
                    at([r, long_other, 0.0]),
                ],
                &format!(
                    "{what}: face {across}, set back by the radius from both its edges at the \
                     corner, touching the piece of sphere at one of its corners"
                ),
            );
            assert_eq!(
                sides_met(body, across),
                sides(&[
                    round_of(one),
                    round_of(other),
                    corner.far[one],
                    corner.far[other]
                ]),
                "{what}: face {across} meets the rounds of its two edges at the corner and the \
                 faces they end on, and not the piece of sphere along a side"
            );
        }
        for face in numbers(body) {
            if let Some(plane) = body.plane_of(face) {
                assert!(
                    plane
                        .corners
                        .iter()
                        .all(|point| point.distance(corner.at) > PLACE),
                    "{what}: face {face} still has a corner at the old vertex {}",
                    corner.at
                );
            }
        }
    }
}

/// A convex corner where three flat faces meet: where it stands, each face's
/// number and the way it faces, and for the edge across from each face — the
/// one the two others meet along — what it is called, how long it is and the
/// face it ends on.
struct Trihedral {
    at: DVec3,
    faces: [usize; 3],
    outward: [DVec3; 3],
    called: [&'static str; 3],
    length: [f64; 3],
    far: [usize; 3],
}

impl Trihedral {
    /// The edge across from face `n`, running from the corner along the two
    /// other faces, into the matter.
    fn edge(&self, n: usize) -> EdgeAt {
        let [one, other] = [(n + 1) % 3, (n + 2) % 3];
        let mut along = self.outward[one].cross(self.outward[other]).normalize();
        if along.dot(self.outward[n]) > 0.0 {
            along = -along;
        }
        let into = |face: usize, beside: usize| {
            let way = self.outward[face].cross(along).normalize();
            if way.dot(self.outward[beside]) > 0.0 {
                -way
            } else {
                way
            }
        };
        EdgeAt {
            called: self.called[n],
            at: self.at,
            faces: [self.faces[one], self.faces[other]],
            along,
            into: [into(one, other), into(other, one)],
            outward: [self.outward[one], self.outward[other]],
            reach: [-3.0, self.length[n] + 3.0],
        }
    }

    /// Where the flats a chamfer of `distance` lays on the two edges of face
    /// `n` cross on it: the distance from each edge, square to it in the face,
    /// so the distance over the sine of the angle the two edges meet at along
    /// each of them.
    fn crossing(&self, n: usize, distance: f64) -> DVec3 {
        let [one, other] = [(n + 1) % 3, (n + 2) % 3].map(|edge| self.edge(edge).along);
        self.at + (one + other) * distance / one.cross(other).length()
    }

    /// The point `radius` in from each of the three faces.
    fn centre(&self, radius: f64) -> DVec3 {
        let [first, second, third] = self.outward;
        let facing = DMat3::from_cols(first, second, third).transpose();
        self.at + facing.inverse() * DVec3::splat(-radius)
    }

    /// The solid angle between the three ways the faces face: what a piece
    /// of sphere tangent to the three, seen from its centre, covers.
    fn spread(&self) -> f64 {
        let [a, b, c] = self.outward;
        2.0 * a
            .dot(b.cross(c))
            .abs()
            .atan2(1.0 + a.dot(b) + b.dot(c) + c.dot(a))
    }
}

/// The slanted prism's corner at [`SLANTED_CORNER`], its faces the right side,
/// the end and the top: across from them the top's edge on the end, ending on
/// the left side, the top's edge on the side, ending on the start, and the
/// slanted upright, ending on the bottom.
fn slanted_trihedral() -> Trihedral {
    Trihedral {
        at: SLANTED_CORNER,
        faces: [SLANTED_RIGHT, SLANTED_END, SLANTED_TOP],
        outward: [side_facing(), DVec3::X, DVec3::Z],
        called: [
            "the top's edge on the end",
            "the top's edge on the right side",
            "the slanted upright where the side meets the end",
        ],
        length: [10.0, LENGTH, 116f64.sqrt()],
        far: [SLANTED_LEFT, SLANTED_START, SLANTED_BOTTOM],
    }
}

/// The slab beyond the plane through `through` square to `away`, on the side
/// `away` points to: `size` each way across it, and `size` deep.
fn beyond(part: &Body, through: DVec3, away: DVec3, size: f64) -> Body {
    let away = away.normalize();
    let u = away.any_orthonormal_vector();
    part.tool_raised(
        &profile(Contour::rectangle(DVec2::splat(-size), DVec2::splat(size))),
        Frame {
            origin: through,
            u,
            v: away.cross(u),
        },
        away * size,
    )
    .expect("the slab raises")
}

/// The way the block's top is tilted, in two directions at once: rising 0.1
/// along X and 0.2 along Y.
fn tilted_up() -> DVec3 {
    DVec3::new(-0.1, -0.2, 1.0).normalize()
}

/// The block 30 by 10 by 20 with its top cut off by the plane rising from 12
/// at its origin, 0.1 along X and 0.2 along Y: its walls and floor numbered as
/// the block's, the tilted top six. A convex solid, every edge of it meets
/// outwards.
fn tilted_block() -> Body {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    block
        .difference(&tilted_slab(&block))
        .expect("the tilted top is cut")
}

/// The slab the tilted block's top is cut off by: beyond the tilted plane,
/// 60 each way across it and 60 deep.
fn tilted_slab(block: &Body) -> Body {
    beyond(block, DVec3::new(15.0, 5.0, 14.5), tilted_up(), 60.0)
}

/// The reach the tilted block is drawn at: a cut keeps the larger reach of
/// the part and of the tool cut from it, and the slab's far corners reach
/// past the block's box, three times as far.
fn tilted_reach() -> f64 {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    reach_of(&block).max(reach_of(&tilted_slab(&block)))
}

/// The tilted block's corner at (30, 0, 15), where no two of the three edges
/// are square: its faces the right wall, the front and the tilted top; across
/// from them the tilted top's edge on the front, ending on the left wall, its
/// edge on the right, ending on the back, and the upright, ending on the
/// floor.
fn tilted_trihedral() -> Trihedral {
    Trihedral {
        at: DVec3::new(LENGTH, 0.0, 15.0),
        faces: [RIGHT, FRONT, 6],
        outward: [DVec3::X, -DVec3::Y, tilted_up()],
        called: [
            "the tilted top's edge on the front",
            "the tilted top's edge on the right",
            "the upright where the front meets the right",
        ],
        length: [909f64.sqrt(), 104f64.sqrt(), 15.0],
        far: [LEFT, BACK, FLOOR],
    }
}

/// What a body keeps once the three edges at a convex corner are chamfered
/// at `distance` and the corner closed by the triangle through the crossings,
/// computed apart from the step: the three flats' prisms, raised whole along
/// each edge's line, and the slab beyond the triangle's plane on the corner's
/// side, cut away by the booleans. The body being convex, nothing of it but
/// the corner stands on that side.
fn triangle_by_booleans(body: &Body, corner: &Trihedral, distance: f64) -> f64 {
    triangles_by_booleans(
        body,
        &[corner],
        &[0, 1, 2].map(|edge| corner.edge(edge)),
        distance,
    )
}

/// What a body keeps once `edges` are chamfered at `distance` and each of
/// `corners`, three of them meeting there, closed by the triangle through its
/// crossings, computed as [`triangle_by_booleans`] computes one: every flat's
/// prism cut away, then the slab beyond each triangle's plane.
fn triangles_by_booleans(
    body: &Body,
    corners: &[&Trihedral],
    edges: &[EdgeAt],
    distance: f64,
) -> f64 {
    let flattened = edges.iter().fold(body.clone(), |part, edge| {
        part.difference(&edge.tool(&part, Cut::Flat, distance))
            .expect("a flat's prism cuts the body")
    });
    corners
        .iter()
        .fold(flattened, |cut, corner| {
            let crossings = [0, 1, 2].map(|face| corner.crossing(face, distance));
            let mut toward = (crossings[1] - crossings[0])
                .cross(crossings[2] - crossings[0])
                .normalize();
            if toward.dot(corner.at - crossings[0]) < 0.0 {
                toward = -toward;
            }
            cut.difference(&beyond(&cut, crossings[0], toward, 100.0))
                .expect("the triangle's slab cuts the corner")
        })
        .volume()
}

/// What a body keeps once the three edges at a convex corner are rounded at
/// `radius` and the corner closed by a piece of sphere, computed apart from
/// the step: the three rounds' prisms, raised whole, cut away by the
/// booleans; then, of what is left, the part on the corner's side of the
/// three planes through the centre square to the edges — the cell the piece
/// of sphere fills — taken away, and the ball's sector over that cell, a
/// third of the radius cubed times the solid angle the three faces' ways
/// span, given back.
fn sphere_by_booleans(body: &Body, corner: &Trihedral, radius: f64) -> f64 {
    spheres_by_booleans(
        body,
        &[corner],
        &[0, 1, 2].map(|edge| corner.edge(edge)),
        radius,
    )
}

/// What a body keeps once `edges` are rounded at `radius` and each of
/// `corners`, three of them meeting there, closed by a piece of sphere,
/// computed as [`sphere_by_booleans`] computes one, each corner's cell taken
/// apart from the others'.
fn spheres_by_booleans(body: &Body, corners: &[&Trihedral], edges: &[EdgeAt], radius: f64) -> f64 {
    let rounded = edges.iter().fold(body.clone(), |part, edge| {
        part.difference(&edge.tool(&part, Cut::Round, radius))
            .expect("a round's prism cuts the body")
    });
    corners.iter().fold(rounded.volume(), |kept, corner| {
        let centre = corner.centre(radius);
        let cell = (0..3).fold(rounded.clone(), |part, edge| {
            part.difference(&beyond(&part, centre, corner.edge(edge).along, 100.0))
                .expect("a plane through the centre cuts the body")
        });
        kept - cell.volume() + radius.powi(3) * corner.spread() / 3.0
    })
}

/// What the slanted prism keeps once the three edges at [`SLANTED_CORNER`]
/// are chamfered at `distance` and the corner closed by a triangle, as the
/// module's documentation derives it by hand.
fn slanted_triangle(distance: f64) -> f64 {
    let k = slant();
    let d = distance;
    SLANTED_VOLUME - 5.0 * d * d - 0.1 * d.powi(3) - 15.0 * k * d * d - 5.0 * d * d / k
        + (k + 1.0 / k) * d.powi(3) / 3.0
}

/// What the slanted prism keeps once the three edges at [`SLANTED_CORNER`]
/// are rounded at `radius` and the corner closed by a piece of sphere, as the
/// module's documentation derives it by hand: each round whole, less its part
/// on the corner's side of its end section, and the cell those sections bound
/// at the corner, less the ball's sector in it.
fn slanted_sphere(radius: f64) -> f64 {
    let k = slant();
    let r = radius;
    let set_back = 1.0 / k - 0.4;
    let square = r * r * (1.0 - PI / 4.0);
    let slanted = r * r * (set_back - 2.5f64.atan() / 2.0);
    let moment = r.powi(3) * (5.0 / 6.0 - PI / 4.0);
    let whole = (10.0 * square + 0.6 * moment) + LENGTH * slanted + 10.0 / k * square;
    let short_of_the_corner = 2.0 * (r * set_back * square + 0.4 * moment) + r * slanted;
    let cell = r.powi(3) * (1.0 - 0.6 * k + 0.232 * k.powi(3)) / k;
    let sector = r.powi(3) * 2.5f64.atan() / 3.0;
    SLANTED_VOLUME - whole + short_of_the_corner - cell + sector
}

/// Holds that the chamfer on the three edges at `corner`, named in `order`,
/// at `distance`, closes the corner with a triangle where the flats' edges
/// cross, and leaves `promised`.
fn closes_with_a_triangle(
    part: &Body,
    corner: &Trihedral,
    order: [usize; 3],
    distance: f64,
    promised: f64,
) {
    let what = format!(
        "the chamfer on {}, {} and {}",
        corner.called[order[0]], corner.called[order[1]], corner.called[order[2]]
    );
    let names = order.map(|edge| corner.edge(edge).name(part));

    let chamfered = part
        .chamfer_edges(&names, distance)
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

    taken_as_asked(chamfered.asked, chamfered.taken, distance, &what);
    let body = &chamfered.body;
    sound_drawn_within(body, promised, EXACT * promised, &what);
    let steps_end = part.faces_end();
    let triangle = steps_end + 3;
    let flat_of = |edge: usize| {
        steps_end
            + order
                .iter()
                .position(|&named| named == edge)
                .expect("every edge of the corner is named")
    };
    assert_eq!(
        faces_made(part, body),
        vec![steps_end, steps_end + 1, steps_end + 2, triangle],
        "{what}: one flat for each edge, numbered in the order the step names them, and the \
         triangle after them: {:?}",
        numbers(body)
    );
    assert_eq!(
        body.faces_end(),
        triangle + 1,
        "{what}: the next step numbers its own faces past the triangle"
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );
    let crossings = [0, 1, 2].map(|face| corner.crossing(face, distance));
    closed_by_its_triangle(
        body,
        triangle,
        corner,
        distance,
        &format!("{what}: the triangle"),
    );
    assert_eq!(
        sides_met(body, triangle),
        sides(&[flat_of(0), flat_of(1), flat_of(2)]),
        "{what}: the triangle meets the three flats, one side each, and nothing else"
    );
    for edge in 0..3 {
        let [one, other] = [(edge + 1) % 3, (edge + 2) % 3];
        let at = corner.edge(edge);
        let flat = flat_of(edge);
        let which = format!("{what}: the flat of {}", at.called);
        let [first, second] = at.outward;
        faces_toward(body, flat, (first + second).normalize(), &which);
        let plane = body
            .plane_of(flat)
            .unwrap_or_else(|| panic!("{which} offers no plane"));
        for crossing in [crossings[one], crossings[other]] {
            assert!(
                plane
                    .corners
                    .iter()
                    .any(|point| point.distance(crossing) < PLACE),
                "{which} ends on the triangle along a straight side between the crossings on its \
                 two faces: no corner at {crossing}, among {:?}",
                plane.corners
            );
        }
        let [face_one, face_other] = at.faces;
        assert_eq!(
            sides_met(body, flat),
            sides(&[face_one, face_other, corner.far[edge], triangle]),
            "{which} meets its edge's two faces along its sides, the face its edge ends on, and \
             the triangle, by one edge each, and no other flat"
        );
        let face = corner.faces[edge];
        let plane = body
            .plane_of(face)
            .unwrap_or_else(|| panic!("{what}: face {face} is still flat"));
        assert!(
            plane
                .corners
                .iter()
                .any(|point| point.distance(crossings[edge]) < PLACE),
            "{what}: face {face} has a corner where the two flats on it cross, {}",
            crossings[edge]
        );
        let met = sides_met(body, face);
        for laid in [flat_of(one), flat_of(other)] {
            assert!(
                met.contains(&vec![laid as u32]),
                "{what}: face {face} meets the flat {laid} of one of its edges at the corner: \
                 {met:?}"
            );
        }
        assert!(
            !met.contains(&vec![triangle as u32]),
            "{what}: face {face} touches the triangle at a corner only: {met:?}"
        );
    }
    for face in numbers(body) {
        if let Some(plane) = body.plane_of(face) {
            assert!(
                plane
                    .corners
                    .iter()
                    .all(|point| point.distance(corner.at) > PLACE),
                "{what}: face {face} still has a corner at the old vertex {}",
                corner.at
            );
        }
    }
}

/// Holds that the fillet on the three edges at `corner`, named in `order`, at
/// `radius`, closes the corner with a piece of sphere one radius from its
/// three faces, each round ending on the plane through the sphere's centre
/// square to its edge, and leaves `promised`, its triangles within the
/// drawing's tolerance at `drawn_at`, the reach the part is drawn at.
fn closes_with_a_sphere(
    part: &Body,
    corner: &Trihedral,
    order: [usize; 3],
    radius: f64,
    promised: f64,
    drawn_at: f64,
) {
    let what = format!(
        "the fillet on {}, {} and {}",
        corner.called[order[0]], corner.called[order[1]], corner.called[order[2]]
    );
    let names = order.map(|edge| corner.edge(edge).name(part));

    let filleted = part
        .fillet_edges(&names, radius)
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

    taken_as_asked(filleted.asked, filleted.taken, radius, &what);
    let body = &filleted.body;
    let r = radius;
    let centre = corner.centre(r);
    let edges = [0, 1, 2].map(|edge| corner.edge(edge));
    let rounds = [0, 1, 2].map(|edge| Round {
        start: centre,
        along: edges[edge].along,
        length: corner.length[edge] + 2.0 * r,
        radius: r,
        facing: edges[edge].outward,
    });
    let area: f64 = [0, 1, 2]
        .iter()
        .map(|&edge| edges[edge].round(r).area())
        .sum::<f64>()
        + corner.spread() * r * r;
    sound_drawn_within(body, promised, DRAWN * drawn_at * area, &what);
    let steps_end = part.faces_end();
    let piece = steps_end + 3;
    let round_of = |edge: usize| {
        steps_end
            + order
                .iter()
                .position(|&named| named == edge)
                .expect("every edge of the corner is named")
    };
    assert_eq!(
        faces_made(part, body),
        vec![steps_end, steps_end + 1, steps_end + 2, piece],
        "{what}: one round for each edge, numbered in the order the step names them, and the \
         piece of sphere after them: {:?}",
        numbers(body)
    );
    assert_eq!(
        body.faces_end(),
        piece + 1,
        "{what}: the next step numbers its own faces past the piece of sphere"
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );

    let which = format!("{what}: the piece of sphere");
    closed_by_its_piece_of_sphere(body, piece, corner, r, &which);
    assert_eq!(
        sides_met(body, piece),
        sides(&[round_of(0), round_of(1), round_of(2)]),
        "{which} meets the three rounds, one side each, and nothing else"
    );

    for (edge, round) in rounds.iter().enumerate() {
        let [one, other] = [(edge + 1) % 3, (edge + 2) % 3];
        let at = &edges[edge];
        let face = round_of(edge);
        let which = format!("{what}: the round of {}", at.called);
        assert!(
            !body.is_flat(face) && body.plane_of(face).is_none(),
            "{which} is curved, and offers no plane to draw on"
        );
        let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
        assert!(!corners.is_empty(), "{which} is drawn");
        for point in &corners {
            assert!(
                round.holds(*point),
                "{which} is drawn through {point}, off its cylinder of {r} about the line from \
                 {centre} along {}",
                at.along
            );
        }
        let section: Vec<&DVec3> = corners
            .iter()
            .filter(|point| ((**point - centre).dot(at.along)).abs() < PLACE)
            .collect();
        for way in at.outward {
            let foot = centre + way * r;
            assert!(
                section.iter().any(|point| point.distance(foot) < PLACE),
                "{which}: its end section, an arc in the plane through the sphere's centre square \
                 to its edge, ends at {foot}, among {section:?}"
            );
        }
        assert!(
            distinct_points(&section) > 2,
            "{which}: its end section is drawn along the arc between its two ends: {section:?}"
        );
        met_from_outside(
            body,
            face,
            &round.part(r, 0.6 * corner.length[edge]),
            &format!("{which}, past its end section"),
        );
        let [face_one, face_other] = at.faces;
        assert_eq!(
            sides_met(body, face),
            sides(&[face_one, face_other, corner.far[edge], piece]),
            "{which} meets its edge's two faces along its sides, the face its edge ends on, and \
             the piece of sphere along its end section, by one edge each, and no other round"
        );
        let own = corner.faces[edge];
        let foot = centre + corner.outward[edge] * r;
        let plane = body
            .plane_of(own)
            .unwrap_or_else(|| panic!("{what}: face {own} is still flat"));
        assert!(
            plane
                .corners
                .iter()
                .any(|point| point.distance(foot) < PLACE),
            "{what}: face {own} has a corner where the piece of sphere touches it, {foot}"
        );
        let met = sides_met(body, own);
        for laid in [round_of(one), round_of(other)] {
            assert!(
                met.contains(&vec![laid as u32]),
                "{what}: face {own} meets the round {laid} of one of its edges at the corner: \
                 {met:?}"
            );
        }
        assert!(
            !met.contains(&vec![piece as u32]),
            "{what}: face {own} touches the piece of sphere at a corner only: {met:?}"
        );
    }
    for face in numbers(body) {
        if let Some(plane) = body.plane_of(face) {
            assert!(
                plane
                    .corners
                    .iter()
                    .all(|point| point.distance(corner.at) > PLACE),
                "{what}: face {face} still has a corner at the old vertex {}",
                corner.at
            );
        }
    }
}

#[test]
fn a_chamfer_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_triangle_where_the_flats_edges_cross()
 {
    let prism = slanted_prism();
    let corner = slanted_trihedral();
    for order in ORDERS_OF_THREE {
        closes_with_a_triangle(&prism, &corner, order, 2.0, slanted_triangle(2.0));
    }
}

#[test]
fn a_fillet_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces()
 {
    let prism = slanted_prism();
    let corner = slanted_trihedral();
    for order in ORDERS_OF_THREE {
        closes_with_a_sphere(
            &prism,
            &corner,
            order,
            2.0,
            slanted_sphere(2.0),
            reach_of(&prism),
        );
    }
}

#[test]
fn a_chamfer_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_triangle_where_the_flats_edges_cross()
 {
    let block = tilted_block();
    let corner = tilted_trihedral();
    let promised = triangle_by_booleans(&block, &corner, 2.0);
    for order in ORDERS_OF_THREE {
        closes_with_a_triangle(&block, &corner, order, 2.0, promised);
    }
}

#[test]
fn a_fillet_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces()
 {
    let block = tilted_block();
    let corner = tilted_trihedral();
    let promised = sphere_by_booleans(&block, &corner, 2.0);
    for order in ORDERS_OF_THREE {
        closes_with_a_sphere(&block, &corner, order, 2.0, promised, tilted_reach());
    }
}

/// The block's eight corners, each with its faces square to X, to Y and to
/// Z: across from them its edges along X, along Y and along Z.
fn block_corners() -> Vec<Trihedral> {
    let mut corners = Vec::new();
    for (x, x_face, x_far, x_way) in [
        (0.0, LEFT, RIGHT, -DVec3::X),
        (LENGTH, RIGHT, LEFT, DVec3::X),
    ] {
        for (y, y_face, y_far, y_way) in [
            (0.0, FRONT, BACK, -DVec3::Y),
            (BREADTH, BACK, FRONT, DVec3::Y),
        ] {
            for (z, z_face, z_far, z_way) in
                [(0.0, FLOOR, TOP, -DVec3::Z), (HEIGHT, TOP, FLOOR, DVec3::Z)]
            {
                corners.push(Trihedral {
                    at: DVec3::new(x, y, z),
                    faces: [x_face, y_face, z_face],
                    outward: [x_way, y_way, z_way],
                    called: ["its edge along X", "its edge along Y", "its edge along Z"],
                    length: [LENGTH, BREADTH, HEIGHT],
                    far: [x_far, y_far, z_far],
                });
            }
        }
    }
    corners
}

/// The two faces an edge lies between, the smaller first.
fn sorted([one, other]: [usize; 2]) -> [usize; 2] {
    [one.min(other), one.max(other)]
}

/// The corners among `corners` three edges of `list` meet at, each with the
/// ranks of its three edges in the list, from the lowest: in the order a step
/// numbers the patches closing them, by the lowest rank, then the next.
fn closed_corners<'a>(
    list: &[[usize; 2]],
    corners: &'a [Trihedral],
) -> Vec<(&'a Trihedral, [usize; 3])> {
    let rank = |edge: [usize; 2]| list.iter().position(|&named| sorted(named) == edge);
    let mut closed: Vec<(&Trihedral, [usize; 3])> = corners
        .iter()
        .filter_map(|corner| {
            let [Some(one), Some(two), Some(three)] =
                [0, 1, 2].map(|edge| rank(sorted(corner.edge(edge).faces)))
            else {
                return None;
            };
            let mut ranks = [one, two, three];
            ranks.sort();
            Some((corner, ranks))
        })
        .collect();
    closed.sort_by_key(|(_, ranks)| *ranks);
    closed
}

/// What the block keeps once the edges of a list, `along` long summed, are
/// cut at `value` and `closed` corners closed, every other end of them meeting
/// no other edge of the list, as the module's documentation derives it.
fn rounded_box(cut: Cut, along: f64, closed: f64, value: f64) -> f64 {
    match cut {
        Cut::Flat => {
            BLOCK_VOLUME - value * value * along / 2.0 + closed * 2.0 * value.powi(3) / 3.0
        }
        Cut::Round => {
            BLOCK_VOLUME - value * value * (1.0 - PI / 4.0) * along
                + closed * value.powi(3) * (2.0 - 7.0 * PI / 12.0)
        }
    }
}

/// Holds that a chamfer at `value`, and a fillet, on the edges each of
/// `lists` names on `part` — the same edges, `along` long summed, in
/// different orders — close every one of `corners` three of them meet at,
/// leave what `kept` promises for the cut and the corners closed, number each
/// corner's patch after the edges' faces by the lowest ranks of its edges in
/// the list, and leave the same body whatever the order, face for face.
fn closes_its_corners_whatever_the_order(
    part: &Body,
    corners: &[Trihedral],
    lists: &[Vec<[usize; 2]>],
    along: f64,
    value: f64,
    kept: impl Fn(Cut, usize) -> f64,
    called: &str,
) {
    let block = part;
    let start = block.faces_end();
    for cut in [Cut::Flat, Cut::Round] {
        let kind = match cut {
            Cut::Flat => "chamfer",
            Cut::Round => "fillet",
        };
        let laid: Vec<Result<Body, Declined>> = lists
            .iter()
            .map(|list| {
                let names: Vec<EdgeName> = list
                    .iter()
                    .map(|&[one, other]| edge_between(block, one, other))
                    .collect();
                match cut {
                    Cut::Flat => block
                        .chamfer_edges(&names, value)
                        .map(|chamfered| chamfered.body),
                    Cut::Round => block
                        .fillet_edges(&names, value)
                        .map(|filleted| filleted.body),
                }
            })
            .collect();
        let declined: Vec<String> = lists
            .iter()
            .zip(&laid)
            .filter_map(|(list, laid)| {
                laid.as_ref()
                    .err()
                    .map(|declined| format!("named {list:?}: {declined:?}"))
            })
            .collect();
        assert!(
            declined.is_empty(),
            "the {kind} on {called} closes every corner whatever the order; declined:\n{}",
            declined.join("\n")
        );
        let bodies: Vec<Body> = laid.into_iter().flatten().collect();
        for (list, body) in lists.iter().zip(&bodies) {
            let what = format!("the {kind} on {called}, named {list:?}");
            let closed = closed_corners(list, corners);
            let promised = kept(cut, closed.len());
            let drawn = match cut {
                Cut::Flat => EXACT * promised,
                Cut::Round => {
                    DRAWN
                        * reach_of(block)
                        * (PI / 2.0 * value * (along + value * closed.len() as f64))
                }
            };
            sound_drawn_within(body, promised, drawn, &what);
            let laid_end = start + list.len() + closed.len();
            assert_eq!(
                faces_made(block, body),
                (start..laid_end).collect::<Vec<_>>(),
                "{what}: one face for each edge, by its rank in the list, then one patch for \
                 each corner closed"
            );
            assert_eq!(
                body.faces_end(),
                laid_end,
                "{what}: the next step numbers its own faces past the last patch"
            );
            for (index, (corner, ranks)) in closed.iter().enumerate() {
                let patch = start + list.len() + index;
                let which = format!(
                    "{what}: patch {patch}, closing the corner at {}, its edges ranked {ranks:?} \
                     — the {}th corner by the lowest ranks of its edges",
                    corner.at,
                    index + 1
                );
                match cut {
                    Cut::Flat => corners_are(
                        body,
                        patch,
                        &[0, 1, 2].map(|face| corner.crossing(face, value)),
                        &format!("{which}, a triangle where the flats' edges cross"),
                    ),
                    Cut::Round => {
                        let centre = corner.centre(value);
                        let drawn: Vec<DVec3> = body.triangles_of(patch).flatten().collect();
                        assert!(!drawn.is_empty(), "{which} is drawn");
                        for point in drawn {
                            assert!(
                                (point.distance(centre) - value).abs() < PLACE,
                                "{which}: drawn through {point}, off the sphere of {value} about \
                                 {centre}"
                            );
                        }
                    }
                }
                assert_eq!(
                    sides_met(body, patch),
                    sides(&ranks.map(|rank| start + rank)),
                    "{which} meets the three faces laid on its edges, and nothing else"
                );
            }
        }
        let (first_list, first) = (&lists[0], &bodies[0]);
        let first_closed = closed_corners(first_list, corners);
        for (list, body) in lists.iter().zip(&bodies).skip(1) {
            let what = format!(
                "the {kind} on {called}, named {list:?}, against the same named {first_list:?}"
            );
            let closed = closed_corners(list, corners);
            let to_this = |number: usize| -> usize {
                if number < start {
                    return number;
                }
                let rank = number - start;
                if rank < first_list.len() {
                    let edge = sorted(first_list[rank]);
                    return start
                        + list
                            .iter()
                            .position(|&named| sorted(named) == edge)
                            .expect("the lists name the same edges");
                }
                let at = first_closed[rank - first_list.len()].0.at;
                start
                    + list.len()
                    + closed
                        .iter()
                        .position(|(corner, _)| corner.at == at)
                        .expect("the lists close the same corners")
            };
            for face in numbers(first) {
                assert_eq!(
                    sides_renumbered(first, face, to_this),
                    sides_met(body, to_this(face)),
                    "{what}: face {face} meets what it meets in the first order"
                );
                if let Some(plane) = first.plane_of(face) {
                    corners_are(
                        body,
                        to_this(face),
                        &plane.corners,
                        &format!("{what}: face {face}, as in the first order"),
                    );
                }
            }
        }
    }
}

/// An edge of the block as the two faces it lies between.
const TOP_FRONT: [usize; 2] = [TOP, FRONT];
const TOP_RIGHT: [usize; 2] = [TOP, RIGHT];
const TOP_BACK: [usize; 2] = [TOP, BACK];
const TOP_LEFT: [usize; 2] = [TOP, LEFT];
const FLOOR_FRONT: [usize; 2] = [FLOOR, FRONT];
const FLOOR_RIGHT: [usize; 2] = [FLOOR, RIGHT];
const FLOOR_BACK: [usize; 2] = [FLOOR, BACK];
const FLOOR_LEFT: [usize; 2] = [FLOOR, LEFT];
const FRONT_LEFT: [usize; 2] = [FRONT, LEFT];
const FRONT_RIGHT: [usize; 2] = [FRONT, RIGHT];
const BACK_RIGHT: [usize; 2] = [BACK, RIGHT];
const BACK_LEFT: [usize; 2] = [BACK, LEFT];

#[test]
fn a_chamfer_and_a_fillet_on_an_upright_and_the_four_edges_at_its_two_ends_close_both_corners_whatever_the_order()
 {
    closes_its_corners_whatever_the_order(
        &block([LENGTH, BREADTH, HEIGHT]),
        &block_corners(),
        &[
            vec![TOP_FRONT, TOP_RIGHT, FLOOR_FRONT, FLOOR_RIGHT, FRONT_RIGHT],
            vec![TOP_FRONT, FRONT_RIGHT, TOP_RIGHT, FLOOR_FRONT, FLOOR_RIGHT],
            vec![FLOOR_FRONT, FRONT_RIGHT, TOP_FRONT, FLOOR_RIGHT, TOP_RIGHT],
        ],
        2.0 * LENGTH + 2.0 * BREADTH + HEIGHT,
        3.0,
        |cut, closed| {
            rounded_box(
                cut,
                2.0 * LENGTH + 2.0 * BREADTH + HEIGHT,
                closed as f64,
                3.0,
            )
        },
        "the front right upright and the four edges at its two ends",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_the_twelve_edges_of_a_block_close_its_eight_corners_into_one_body_whatever_the_order()
 {
    closes_its_corners_whatever_the_order(
        &block([LENGTH, BREADTH, HEIGHT]),
        &block_corners(),
        &[
            vec![
                TOP_FRONT,
                TOP_RIGHT,
                TOP_BACK,
                TOP_LEFT,
                FLOOR_FRONT,
                FLOOR_RIGHT,
                FLOOR_BACK,
                FLOOR_LEFT,
                FRONT_LEFT,
                FRONT_RIGHT,
                BACK_RIGHT,
                BACK_LEFT,
            ],
            vec![
                FRONT_RIGHT,
                FLOOR_BACK,
                TOP_LEFT,
                TOP_FRONT,
                BACK_LEFT,
                FLOOR_FRONT,
                TOP_RIGHT,
                FLOOR_LEFT,
                BACK_RIGHT,
                TOP_BACK,
                FRONT_LEFT,
                FLOOR_RIGHT,
            ],
        ],
        4.0 * (LENGTH + BREADTH + HEIGHT),
        3.0,
        |cut, closed| rounded_box(cut, 4.0 * (LENGTH + BREADTH + HEIGHT), closed as f64, 3.0),
        "the twelve edges of the block",
    );
}

/// The breadth of the block a full round is laid on, twice [`FULL_ROUND`]:
/// the rounds of its top's front and back edges meet across the top.
const NARROW: f64 = 6.0;

/// The radius the rounds of the top's front and back edges of the block
/// [`NARROW`] wide meet across its top at: half its breadth.
const FULL_ROUND: f64 = NARROW / 2.0;

/// The full round's pair on the block [`NARROW`] wide, the top's front and
/// back edges, and what each is called.
const PAIR: [([usize; 2], &str); 2] = [
    (TOP_FRONT, "the top's front edge"),
    (TOP_BACK, "the top's back edge"),
];

/// How far from a mitre a ray aims, on either side of it, to meet the
/// triangles of the round it aims at rather than the other's: well over the
/// drawing's tolerance on the block [`NARROW`] wide, [`DRAWN`] times its
/// reach.
const CLEAR_OF_THE_MITRE: f64 = 0.1;

/// What a plate keeps once its top's front and back edges are rounded at
/// [`FULL_ROUND`] into one full round, and the top's edges at the ends
/// `named` rounded at the same radius and mitred against it, as the module's
/// documentation derives it: the whole outline up to the full round's axis;
/// over it, the full round's half-disc along its axis from one end to the
/// other — to where the two axes cross at an end named, to the middle of the
/// wall at one not — and at each end named, what lies inside both cylinders
/// past that crossing, `4r³/3` stretched across the slant by one over the
/// sine of the angle the end wall meets the long edges at.
fn mitred_against_a_full_round(plate: &Plate, named: &[&End]) -> f64 {
    let r = FULL_ROUND;
    let reach = |end: &End| {
        if named.iter().any(|one| one.wall == end.wall) {
            end.crossing().x
        } else {
            (end.corners[0].x + end.corners[1].x) / 2.0
        }
    };
    let [right, left] = &plate.ends;
    let past_the_crossings: f64 = named
        .iter()
        .map(|end| 4.0 * r.powi(3) / (3.0 * end.outward.x.abs()))
        .sum();
    (HEIGHT - r) * plate.area()
        + PI * r * r / 2.0 * (reach(right) - reach(left))
        + past_the_crossings
}

/// The prism the full round on the top's front and back edges of the block
/// [`NARROW`] wide takes away, raised whole along X past both its ends: all
/// that stands over the top's two edges outside the half-cylinder about the
/// line [`FULL_ROUND`] under the middle of the top.
fn full_round_tool(part: &Body) -> Body {
    let far = 2.0 * FULL_ROUND + 2.0;
    let foot = HEIGHT - FULL_ROUND;
    let corners = vec![
        DVec2::new(-far, foot),
        DVec2::new(0.0, foot),
        DVec2::new(NARROW, foot),
        DVec2::new(NARROW + far, foot),
        DVec2::new(NARROW + far, HEIGHT + far),
        DVec2::new(-far, HEIGHT + far),
    ];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[1] = Run::Round {
        center: DVec2::new(FULL_ROUND, foot),
        turn: -PI,
    };
    part.tool_raised(
        &profile(Contour { corners, runs }),
        Frame {
            origin: DVec3::new(-2.0, 0.0, 0.0),
            u: DVec3::Y,
            v: DVec3::Z,
        },
        DVec3::X * (LENGTH + 4.0),
    )
    .expect("the tool raises")
}

/// A plate [`NARROW`] wide and [`HEIGHT`] high whose top's front and back
/// edges, rounded at [`FULL_ROUND`], meet across the top in one full round:
/// the part, and its two ends, the right then the left. Its left end is
/// square, at the origin; its right end's wall may stand at an angle, seen
/// from above. Raised as `cao_part` raises a step, its floor is numbered
/// nought, its top one, then its front, its right end, its back and its left
/// end, two to five, as the block's are.
struct Plate {
    part: Body,
    ends: [End; 2],
}

impl Plate {
    /// The plate whose right end's wall runs from `front`, on the front, to
    /// `back`, on the back, seen from above.
    fn with_right_end([front, back]: [DVec2; 2]) -> Plate {
        let left = [DVec2::ZERO, DVec2::new(0.0, NARROW)];
        Plate {
            part: raised(
                Contour::straight(vec![left[0], front, back, left[1]]),
                HEIGHT,
            ),
            ends: [
                End::between(RIGHT, "the top's right edge", [front, back]),
                End::between(LEFT, "the top's left edge", left),
            ],
        }
    }

    /// The area of its outline, between its two ends.
    fn area(&self) -> f64 {
        let [right, left] = &self.ends;
        let along = |end: &End| end.corners[0].x + end.corners[1].x;
        NARROW * (along(right) - along(left)) / 2.0
    }
}

/// The block 30 by [`NARROW`] by 20, both its ends square.
fn narrow_block() -> Plate {
    Plate::with_right_end([DVec2::new(LENGTH, 0.0), DVec2::new(LENGTH, NARROW)])
}

/// How long the back of [`slanted_plate`] is, its front being [`LENGTH`]:
/// its right end's wall leans back by half the plate's breadth across it, its
/// two corners at 63.4 and 116.6 degrees, both convex.
const SLANTED_BACK: f64 = LENGTH - NARROW / 2.0;

/// The plate [`NARROW`] wide whose right end is cut at an angle seen from
/// above: its front [`LENGTH`] long and its back [`SLANTED_BACK`].
fn slanted_plate() -> Plate {
    Plate::with_right_end([DVec2::new(LENGTH, 0.0), DVec2::new(SLANTED_BACK, NARROW)])
}

/// An end of a [`Plate`] whose top edge a list may name with the full round's
/// pair: the wall there, its two corners seen from above, front then back,
/// the way out of the plate square to it, the top's edge on it, and that
/// edge's round, whole.
struct End {
    wall: usize,
    corners: [DVec2; 2],
    outward: DVec3,
    edge: EdgeAt,
    round: Round,
}

impl End {
    /// The end whose wall `wall` runs from `front` to `back`, seen from above:
    /// the top's edge on it, `called`, from its front corner to its back one,
    /// its prism reaching past the front and the back; and its round, a
    /// quarter round about the line [`FULL_ROUND`] in from the wall and under
    /// the top, from the top's way to the wall's.
    fn between(wall: usize, called: &'static str, [front, back]: [DVec2; 2]) -> End {
        let r = FULL_ROUND;
        let at = front.extend(HEIGHT);
        let run = (back - front).extend(0.0);
        let along = run.normalize();
        let middle = DVec3::new(LENGTH / 2.0, NARROW / 2.0, HEIGHT);
        let mut outward = along.cross(DVec3::Z);
        if outward.dot(at - middle) < 0.0 {
            outward = -outward;
        }
        End {
            wall,
            corners: [front, back],
            outward,
            edge: EdgeAt {
                called,
                at,
                faces: [TOP, wall],
                along,
                into: [-outward, -DVec3::Z],
                outward: [DVec3::Z, outward],
                reach: [-2.0 - r, run.length() + 2.0 + r],
            },
            round: Round {
                start: at - (outward + DVec3::Z) * r,
                along,
                length: run.length(),
                radius: r,
                facing: [DVec3::Z, outward],
            },
        }
    }

    /// Where the end edge's round's axis crosses the full round's, under the
    /// middle of the top.
    fn crossing(&self) -> DVec3 {
        let start = self.round.start;
        start + self.round.along * ((FULL_ROUND - start.y) / self.round.along.y)
    }

    /// How far a point stands past the mitre at this end, on the side of the
    /// end edge's round: how far it stands out past the crossing, square to
    /// the end wall, less its distance from the middle of the top across it —
    /// at the crossing, the two distances from the two axes in the plane of
    /// the top. Nought on the two planes the two rounds meet on, through the
    /// line square to both axes where they cross, halving the angles between
    /// them.
    fn past_the_mitre(&self, point: DVec3) -> f64 {
        (point - self.crossing()).dot(self.outward) - (point.y - FULL_ROUND).abs()
    }

    /// The crown, where the two arcs of the mitre meet: on the top's middle
    /// line, over the crossing.
    fn crown(&self) -> DVec3 {
        self.crossing() + DVec3::Z * FULL_ROUND
    }

    /// The end wall's two top corners, front then back, one radius under the
    /// top, where the two arcs of the mitre start.
    fn top_corners(&self) -> [DVec3; 2] {
        self.corners
            .map(|corner| corner.extend(HEIGHT - FULL_ROUND))
    }

    /// The end wall's two corners on the floor, front then back.
    fn foot_corners(&self) -> [DVec3; 2] {
        self.corners.map(|corner| corner.extend(0.0))
    }
}

/// What the block [`NARROW`] wide keeps once the full round and the rounds of
/// the top's edges at `ends` are laid at [`FULL_ROUND`], computed apart from
/// the step: the full round's prism, then each end edge's, raised whole along
/// its line, taken away by the booleans.
fn mitred_against_a_full_round_by_booleans(part: &Body, ends: &[&End]) -> f64 {
    let rounded = part
        .difference(&full_round_tool(part))
        .expect("the full round's prism cuts the block");
    ends.iter()
        .fold(rounded, |cut, end| {
            cut.difference(&end.edge.tool(&cut, Cut::Round, FULL_ROUND))
                .expect("an end edge's round's prism cuts the body")
        })
        .volume()
}

/// The full round of the block [`NARROW`] wide, whole: two quarter rounds
/// about the line [`FULL_ROUND`] under the middle of its top, from the front
/// up to the top and from the top down to the back.
fn full_round() -> [Round; 2] {
    [[-DVec3::Y, DVec3::Z], [DVec3::Z, DVec3::Y]].map(|facing| Round {
        start: DVec3::new(0.0, FULL_ROUND, HEIGHT - FULL_ROUND),
        along: DVec3::X,
        length: LENGTH,
        radius: FULL_ROUND,
        facing,
    })
}

/// Holds that the numbers `laid` answer to one and the same face, which is
/// curved: drawn with the same triangles whichever is asked.
fn one_curved_face_answers_to(body: &Body, laid: &[usize], what: &str) {
    let drawn: Vec<Vec<[DVec3; 3]>> = laid
        .iter()
        .map(|&number| body.triangles_of(number).collect())
        .collect();
    assert!(
        !drawn[0].is_empty() && drawn.iter().all(|each| *each == drawn[0]),
        "{what}: faces {laid:?} are one face, drawn with the same triangles"
    );
    for &number in laid {
        assert!(
            !body.is_flat(number) && body.plane_of(number).is_none(),
            "{what}: face {number} is curved, and offers no plane to draw on"
        );
    }
}

/// Holds that every ray aimed square at the round's axis from outside, over
/// a grid along it and round it, meets the face numbered `face` first,
/// facing away from the axis along the ray, wherever `shows` says the round
/// shows at the point the ray aims at; and that some ray was aimed.
fn met_from_outside_where(
    body: &Body,
    face: usize,
    round: &Round,
    shows: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let mut aimed = 0;
    for tenth in 0..10 {
        let along = (f64::from(tenth) + 0.5) / 10.0;
        for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let facing = round.facing_at(share);
            let aim = round.axis_at(along) + facing * round.radius;
            if !shows(aim) {
                continue;
            }
            aimed += 1;
            let from = aim + facing * 10.0;
            let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
                panic!("{what}: a ray from {from} toward the axis meets nothing")
            });
            assert_eq!(
                hit.face, face,
                "{what}: a ray from {from} toward {aim} meets face {} before the round",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{what}: where a ray from {from} meets it, the round faces {}, not {facing}, \
                 away from its axis",
                hit.normal
            );
        }
    }
    assert!(aimed > 0, "{what}: some ray aims where the round shows");
}

/// Every order `count` things can be named in.
fn every_order(count: usize) -> Vec<Vec<usize>> {
    if count == 0 {
        return vec![Vec::new()];
    }
    every_order(count - 1)
        .into_iter()
        .flat_map(|shorter| {
            (0..count).map(move |at| {
                let mut order = shorter.clone();
                order.insert(at, count - 1);
                order
            })
        })
        .collect()
}

/// Holds that the fillet asked `asked` on the full round's pair of `plate`
/// and on the top's edges at its `ends`, in every order the list can name
/// them in, reports what `reported` holds of the value asked and the value
/// taken, and mitres each end edge's round against the full round, both at
/// [`FULL_ROUND`]: the body exact, closed, uncrossed and drawn, holding what
/// the arithmetic and the booleans promise; the full round answering to both
/// its edges' numbers and each end edge's round to its own, the top gone and
/// nothing laid at a corner; each round drawn on its own cylinder and on its
/// own side of the mitre, the two meeting by two edges, from the end wall's
/// top corners up to the crown; and the walls set back by the radius, their
/// uprights sharp.
fn mitres_each_end_against_the_full_round_whatever_the_order(
    plate: &Plate,
    ends: &[&End],
    asked: f64,
    reported: impl Fn(f64, f64, &str),
) {
    let r = FULL_ROUND;
    let narrow = &plate.part;
    let start = narrow.faces_end();
    let count = PAIR.len() + ends.len();
    let promised = mitred_against_a_full_round(plate, ends);
    let by_booleans = mitred_against_a_full_round_by_booleans(narrow, ends);
    assert!(
        (by_booleans - promised).abs() <= EXACT * promised,
        "the full round's prism and the end edges' rounds', cut from the block by the \
         booleans, leave {by_booleans}, where arithmetic promises {promised}"
    );
    let named: Vec<(EdgeName, &str)> = PAIR
        .iter()
        .map(|&([one, other], called)| (edge_between(narrow, one, other), called))
        .chain(
            ends.iter()
                .map(|end| (end.edge.name(narrow), end.edge.called)),
        )
        .collect();
    let orders = every_order(count);
    let called = |order: &[usize]| {
        order
            .iter()
            .map(|&edge| named[edge].1)
            .collect::<Vec<_>>()
            .join(", then ")
    };

    let laid: Vec<_> = orders
        .iter()
        .map(|order| {
            let names: Vec<EdgeName> = order.iter().map(|&edge| named[edge].0.clone()).collect();
            narrow.fillet_edges(&names, asked)
        })
        .collect();

    let declined: Vec<String> = orders
        .iter()
        .zip(&laid)
        .filter_map(|(order, laid)| {
            laid.as_ref()
                .err()
                .map(|declined| format!("{}: {declined:?}", called(order)))
        })
        .collect();
    assert!(
        declined.is_empty(),
        "the fillet asked {asked} on the top's front and back edges of a plate {NARROW} wide, a \
         full round, and on the top's edge at {} of its ends mitres each end edge's round \
         against the full round, whichever edge the list names first; declined:\n{}",
        ends.len(),
        declined.join("\n")
    );
    let [front_half, back_half] = full_round();
    let area =
        front_half.area() + back_half.area() + ends.iter().map(|end| end.round.area()).sum::<f64>();
    for (order, laid) in orders.iter().zip(laid) {
        let what = format!("the fillet asked {asked} on {}", called(order));
        let filleted = laid.expect("declines are held above");
        reported(filleted.asked, filleted.taken, &what);
        let body = &filleted.body;
        sound_drawn_within(body, promised, DRAWN * reach_of(narrow) * area, &what);
        let rank_of = |edge: usize| {
            start
                + order
                    .iter()
                    .position(|&named| named == edge)
                    .expect("every edge is named")
        };
        let mut pair = [rank_of(0), rank_of(1)];
        pair.sort();
        let laid_at: Vec<usize> = (PAIR.len()..count).map(rank_of).collect();
        assert!(
            !body.has_face(TOP),
            "{what}: the top is gone, the full round over it"
        );
        let mut kept_and_laid = numbers(narrow);
        kept_and_laid.remove(&TOP);
        kept_and_laid.extend(start..start + count);
        assert_eq!(
            numbers(body),
            kept_and_laid,
            "{what}: every face of the block but its top, the full round answering to {pair:?}, \
             each end edge's round to its own rank, {laid_at:?}, and no face laid at the corners"
        );
        assert_eq!(
            body.faces_end(),
            start + count,
            "{what}: the next step numbers its faces past the {count} the list gave its edges"
        );
        one_curved_face_answers_to(body, &pair, &format!("{what}: the full round"));

        let full_drawn: Vec<DVec3> = body.triangles_of(pair[0]).flatten().collect();
        for &point in &full_drawn {
            assert!(
                front_half.holds(point) || back_half.holds(point),
                "{what}: the full round is drawn through {point}, off its half-cylinder of {r} \
                 about the line {r} under the middle of the top"
            );
            for end in ends {
                assert!(
                    end.past_the_mitre(point) <= PLACE,
                    "{what}: the full round is drawn through {point}, past the mitre where the \
                     round of {} meets it",
                    end.edge.called
                );
            }
        }
        for half in [&front_half, &back_half] {
            met_from_outside_where(
                body,
                pair[0],
                half,
                |aim| {
                    ends.iter()
                        .all(|end| end.past_the_mitre(aim) < -CLEAR_OF_THE_MITRE)
                },
                &format!("{what}: the full round, short of the mitres"),
            );
        }
        let full: &[usize] = &pair;
        let mut met_by_full: Vec<Vec<usize>> = vec![vec![FRONT], vec![BACK]];
        for wall in [RIGHT, LEFT] {
            match ends.iter().position(|end| end.wall == wall) {
                Some(at) => met_by_full.extend([vec![laid_at[at]], vec![laid_at[at]]]),
                None => met_by_full.push(vec![wall]),
            }
        }
        let met_by_full: Vec<&[usize]> = met_by_full.iter().map(Vec::as_slice).collect();
        assert_eq!(
            sides_met_by(body, full),
            sides_answering(&met_by_full),
            "{what}: the full round meets the front and the back along its sides, and at each \
             end the wall there, or the end edge's round along the two arcs of the mitre, by one \
             edge each"
        );

        for (end, &number) in ends.iter().zip(&laid_at) {
            let which = format!("the round of {}", end.edge.called);
            assert!(
                !body.is_flat(number) && body.plane_of(number).is_none(),
                "{what}: {which}, face {number}, is curved, and offers no plane to draw on"
            );
            let end_drawn: Vec<DVec3> = body.triangles_of(number).flatten().collect();
            assert!(!end_drawn.is_empty(), "{what}: {which} is drawn");
            for &point in &end_drawn {
                assert!(
                    end.round.holds(point),
                    "{what}: {which} is drawn through {point}, off its quarter-cylinder of {r} \
                     about the line {r} in from the end and under the top"
                );
                assert!(
                    end.past_the_mitre(point) >= -PLACE,
                    "{what}: {which} is drawn through {point}, past the mitre where the full \
                     round meets it"
                );
            }
            let [front_corner, back_corner] = end.top_corners();
            for (point, at) in [
                (front_corner, "the end wall's top front corner"),
                (end.crown(), "the crown"),
                (back_corner, "the end wall's top back corner"),
            ] {
                for (drawn, round) in [(&full_drawn, "the full round"), (&end_drawn, &*which)] {
                    assert!(
                        drawn.iter().any(|corner| corner.distance(point) < PLACE),
                        "{what}: {round} is drawn up to {at}, {point}, where an arc of the \
                         mitre ends"
                    );
                }
            }
            met_from_outside_where(
                body,
                number,
                &end.round,
                |aim| end.past_the_mitre(aim) > CLEAR_OF_THE_MITRE,
                &format!("{what}: {which}, past the mitre"),
            );
            assert_eq!(
                sides_met(body, number),
                sides_answering(&[&[end.wall], full, full]),
                "{what}: {which} meets the end wall along its foot and the full round along the \
                 two arcs of the mitre, and no other face"
            );
            let [front_foot, back_foot] = end.foot_corners();
            corners_are(
                body,
                end.wall,
                &[front_foot, back_foot, back_corner, front_corner],
                &format!(
                    "{what}: the end wall under {which}, set back by the radius from the top, \
                     its two top corners where the arcs of the mitre start"
                ),
            );
            assert_eq!(
                sides_met(body, end.wall),
                sides_answering(&[&[FLOOR], &[FRONT], &[BACK], &[number]]),
                "{what}: the end wall under {which} meets the floor, the front and the back \
                 along its uprights, and that round"
            );
        }
        for (wall, side, which) in [(FRONT, 0, "the front"), (BACK, 1, "the back")] {
            let [right, left] = &plate.ends;
            let [right_foot, left_foot] = [right, left].map(|end| end.foot_corners()[side]);
            let [right_top, left_top] = [right, left].map(|end| end.top_corners()[side]);
            corners_are(
                body,
                wall,
                &[left_foot, right_foot, right_top, left_top],
                &format!("{what}: {which}, up to where the full round takes over"),
            );
            assert_eq!(
                sides_met(body, wall),
                sides_answering(&[&[FLOOR], &[LEFT], &[RIGHT], full]),
                "{what}: {which} meets the floor, the two walls and the full round, by one edge \
                 each, and no end edge's round"
            );
            for end in ends {
                assert_eq!(
                    edges_between(body, wall, end.wall).len(),
                    1,
                    "{what}: the upright where {which} meets the wall under the round of {} is \
                     still one sharp edge, from the floor up to the mitre",
                    end.edge.called
                );
            }
        }
    }
}

#[test]
fn a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_mitres_that_edge_s_round_against_the_full_round_whatever_the_order()
 {
    let plate = narrow_block();
    let [right, _] = &plate.ends;
    mitres_each_end_against_the_full_round_whatever_the_order(
        &plate,
        &[right],
        FULL_ROUND,
        |asked, taken, what| taken_as_asked(asked, taken, FULL_ROUND, what),
    );
}

#[test]
fn a_fillet_on_the_four_top_edges_of_a_block_6_wide_mitres_its_full_round_against_both_end_edges_whatever_the_order()
 {
    let plate = narrow_block();
    let [right, left] = &plate.ends;
    mitres_each_end_against_the_full_round_whatever_the_order(
        &plate,
        &[right, left],
        FULL_ROUND,
        |asked, taken, what| taken_as_asked(asked, taken, FULL_ROUND, what),
    );
}

/// What is asked of the full round's pair and the edge at its end on the
/// block [`NARROW`] wide, more than [`FULL_ROUND`], where the pair's
/// set-backs meet: less than the end edge alone takes.
const MORE_THAN_FITS: f64 = 4.0;

#[test]
fn a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_asked_more_than_fits_lays_both_at_the_full_round_s_radius_and_reports_both()
 {
    let plate = narrow_block();
    let [right, _] = &plate.ends;
    let alone = plate
        .part
        .fillet(&right.edge.name(&plate.part), MORE_THAN_FITS)
        .unwrap_or_else(|declined| panic!("the top's right edge alone: declined, {declined:?}"));
    taken_as_asked(
        alone.asked,
        alone.taken,
        MORE_THAN_FITS,
        "the fillet on the top's right edge alone, whose round the full round's radius does not \
         bound",
    );

    mitres_each_end_against_the_full_round_whatever_the_order(
        &plate,
        &[right],
        MORE_THAN_FITS,
        |asked, taken, what| {
            assert!(
                (asked - MORE_THAN_FITS).abs() < PLACE,
                "{what}: reports {asked} asked, where {MORE_THAN_FITS} was"
            );
            assert!(
                (taken - FULL_ROUND).abs() < PLACE,
                "{what}: took {taken}, where the full round's pair gives {FULL_ROUND}, and the \
                 edge at its end is laid at that one value"
            );
        },
    );
}

/// One of the kernel's hairs on the block [`LENGTH`] long: twenty
/// tolerances, a tolerance a billionth of the reach.
const ONE_HAIR: f64 = 20.0 * 1e-9 * LENGTH;

/// How short a strip left between two set-backs on a face may be and still
/// read as none to the kernel, on the block [`LENGTH`] long: four of its
/// hairs.
const MEETING_HAIR: f64 = 4.0 * ONE_HAIR;

#[test]
fn a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_asked_a_hair_under_the_full_round_s_radius_lays_both_at_that_radius_meeting_at_the_crown()
 {
    let plate = narrow_block();
    let [right, _] = &plate.ends;

    for (under, past_one_hair) in [(1e-6, true), (1e-7, false)] {
        let asked = FULL_ROUND - under;
        let strip = NARROW - 2.0 * asked;
        assert!(
            strip < MEETING_HAIR && (strip > ONE_HAIR) == past_one_hair,
            "asked {asked}: the strip of top the pair's rounds leave, {strip}, is shorter than the \
             kernel keeps, {MEETING_HAIR}, and longer than one of its hairs, {ONE_HAIR}, or not, \
             as {past_one_hair} says"
        );

        mitres_each_end_against_the_full_round_whatever_the_order(
            &plate,
            &[right],
            asked,
            |reported, taken, what| {
                assert!(
                    (reported - asked).abs() < PLACE && (taken - asked).abs() <= MEETING_HAIR,
                    "{what}: asked {reported} and took {taken}, where {asked} is a hair from the \
                     full round's radius, {FULL_ROUND}"
                );
            },
        );
    }
}

#[test]
fn a_fillet_on_a_full_round_pair_and_an_end_edge_cut_at_an_angle_seen_from_above_mitres_the_end_edge_s_round_against_the_full_round_in_either_order()
 {
    let plate = slanted_plate();
    let [right, _] = &plate.ends;
    mitres_each_end_against_the_full_round_whatever_the_order(
        &plate,
        &[right],
        FULL_ROUND,
        |asked, taken, what| taken_as_asked(asked, taken, FULL_ROUND, what),
    );
}

#[test]
fn a_fillet_on_a_full_round_meeting_another_full_round_is_declined_as_unsupported_whatever_the_order()
 {
    let square = block([NARROW, NARROW, HEIGHT]);
    let called =
        "the four top edges of a block 6 by 6, two full rounds meeting at its four corners";
    let name = |[one, other]: [usize; 2]| edge_between(&square, one, other);
    let pair = [TOP_FRONT, TOP_BACK];
    square
        .fillet_edges(&pair.map(name), FULL_ROUND)
        .unwrap_or_else(|declined| {
            panic!("{called}: the pair alone is laid, a full round; declined, {declined:?}")
        });
    let list = [TOP_FRONT, TOP_BACK, TOP_RIGHT, TOP_LEFT];

    let mut laid = Vec::new();
    for order in every_order(list.len()) {
        let faces: Vec<[usize; 2]> = order.iter().map(|&edge| list[edge]).collect();
        let names: Vec<EdgeName> = faces.iter().map(|&edge| name(edge)).collect();
        let filleted = square.fillet_edges(&names, FULL_ROUND).map(|_| ());
        laid.push((format!("{called}, the fillet on {faces:?}"), filleted));
    }

    let not_declined: Vec<String> = laid
        .iter()
        .filter(|(_, result)| *result != Err(Declined::Unsupported))
        .map(|(what, result)| match result {
            Ok(()) => format!("{what}: a body"),
            Err(declined) => format!("{what}: declined, {declined:?}"),
        })
        .collect();
    assert!(
        not_declined.is_empty(),
        "a full round meeting another is a broken step and its reason, whatever the order, the \
         pair alone laid; not so:\n{}",
        not_declined.join("\n")
    );
}

/// What the block [`NARROW`] wide keeps once its top's front and back edges
/// are chamfered at [`FULL_ROUND`], their flats meeting in a ridge along the
/// middle of the top, and its top's right edge chamfered at the same distance
/// and mitred against both, as the module's documentation derives it.
fn ridge_mitred_against_the_end_flat() -> f64 {
    let d = FULL_ROUND;
    LENGTH * NARROW * HEIGHT - d * d * (LENGTH - d) - 4.0 * d.powi(3) / 3.0
}

#[test]
fn a_chamfer_on_a_full_round_pair_s_edges_and_an_edge_meeting_them_at_a_corner_mitres_their_ridge_against_that_edge_s_flat_whatever_the_order()
 {
    let d = FULL_ROUND;
    let plate = narrow_block();
    let narrow = &plate.part;
    let start = narrow.faces_end();
    let [right, _] = &plate.ends;
    let named: Vec<(EdgeName, &str)> = PAIR
        .iter()
        .map(|&([one, other], called)| (edge_between(narrow, one, other), called))
        .chain([(right.edge.name(narrow), right.edge.called)])
        .collect();
    let promised = ridge_mitred_against_the_end_flat();
    let ridge_s_end = DVec3::new(LENGTH - d, d, HEIGHT);
    let [front_corner, back_corner] = right.top_corners();

    for order in every_order(3) {
        let called: Vec<&str> = order.iter().map(|&edge| named[edge].1).collect();
        let what = format!("the chamfer at {d} on {}", called.join(", then "));
        let names: Vec<EdgeName> = order.iter().map(|&edge| named[edge].0.clone()).collect();

        let chamfered = narrow
            .chamfer_edges(&names, d)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_as_asked(chamfered.asked, chamfered.taken, d, &what);
        let body = &chamfered.body;
        sound_drawn_within(body, promised, EXACT * promised, &what);
        let rank_of = |edge: usize| {
            start
                + order
                    .iter()
                    .position(|&named| named == edge)
                    .expect("every edge is named")
        };
        let [front, back, end] = [0, 1, 2].map(rank_of);
        assert!(
            !body.has_face(TOP),
            "{what}: the top is gone, the ridge along its middle"
        );
        let mut kept_and_laid = numbers(narrow);
        kept_and_laid.remove(&TOP);
        kept_and_laid.extend(start..start + 3);
        assert_eq!(
            numbers(body),
            kept_and_laid,
            "{what}: every face of the block but its top, one flat for each edge, numbered in the \
             order the list names them, and no face laid at the corners"
        );
        for (flat, corners, facing, met, which) in [
            (
                front,
                vec![
                    DVec3::new(0.0, 0.0, HEIGHT - d),
                    front_corner,
                    ridge_s_end,
                    DVec3::new(0.0, d, HEIGHT),
                ],
                (DVec3::Z - DVec3::Y).normalize(),
                [FRONT, LEFT, back, end],
                "the front edge's flat, from the front up to the ridge",
            ),
            (
                back,
                vec![
                    DVec3::new(0.0, NARROW, HEIGHT - d),
                    back_corner,
                    ridge_s_end,
                    DVec3::new(0.0, d, HEIGHT),
                ],
                (DVec3::Z + DVec3::Y).normalize(),
                [BACK, LEFT, front, end],
                "the back edge's flat, from the back up to the ridge",
            ),
        ] {
            corners_are(body, flat, &corners, &format!("{what}: {which}"));
            faces_toward(body, flat, facing, &format!("{what}: {which}"));
            assert_eq!(
                sides_met(body, flat),
                sides(&met),
                "{what}: {which} meets its wall, the left wall, the other flat along the ridge, \
                 and the right edge's flat along the mitre, by one edge each"
            );
        }
        corners_are(
            body,
            end,
            &[front_corner, back_corner, ridge_s_end],
            &format!(
                "{what}: the right edge's flat, a triangle from the right wall's two top corners \
                 up to the ridge's end"
            ),
        );
        faces_toward(
            body,
            end,
            (DVec3::Z + DVec3::X).normalize(),
            &format!("{what}: the right edge's flat"),
        );
        assert_eq!(
            sides_met(body, end),
            sides(&[RIGHT, front, back]),
            "{what}: the right edge's flat meets the right wall and the two flats of the ridge, \
             by one edge each"
        );
        corners_are(
            body,
            RIGHT,
            &[
                DVec3::new(LENGTH, 0.0, 0.0),
                DVec3::new(LENGTH, NARROW, 0.0),
                back_corner,
                front_corner,
            ],
            &format!("{what}: the right wall, set back by the distance from the top"),
        );
        for (wall, y, flat, which) in [
            (FRONT, 0.0, front, "the front"),
            (BACK, NARROW, back, "the back"),
        ] {
            corners_are(
                body,
                wall,
                &[
                    DVec3::new(0.0, y, 0.0),
                    DVec3::new(LENGTH, y, 0.0),
                    DVec3::new(LENGTH, y, HEIGHT - d),
                    DVec3::new(0.0, y, HEIGHT - d),
                ],
                &format!("{what}: {which}, up to its edge's flat"),
            );
            assert_eq!(
                sides_met(body, wall),
                sides(&[FLOOR, LEFT, RIGHT, flat]),
                "{what}: {which} meets the floor, the two walls and its edge's flat, by one edge \
                 each"
            );
            assert_eq!(
                edges_between(body, wall, RIGHT).len(),
                1,
                "{what}: the upright where {which} meets the right wall is still one sharp edge"
            );
        }
    }
}

#[test]
fn a_chamfer_and_a_fillet_mitre_two_edges_whose_corner_s_uncut_edge_runs_into_an_earlier_round_at_its_far_end()
 {
    let value = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let rounded = block
        .fillet(&edge_between(&block, FLOOR, FRONT), value)
        .unwrap_or_else(|declined| panic!("the floor's front edge alone: declined, {declined:?}"))
        .body;
    let earlier = value * value * (1.0 - PI / 4.0) * LENGTH;
    let start = rounded.faces_end();
    let names = [TOP_FRONT, TOP_RIGHT].map(|[one, other]| edge_between(&rounded, one, other));

    for (cut, promised) in [
        (
            Cut::Flat,
            mitred_by_flats([LENGTH, BREADTH], value) - earlier,
        ),
        (
            Cut::Round,
            mitred_by_rounds([LENGTH, BREADTH], value) - earlier,
        ),
    ] {
        let what = format!(
            "the {} on the top's front and right edges, the floor's front edge rounded before",
            match cut {
                Cut::Flat => "chamfer",
                Cut::Round => "fillet",
            }
        );

        let laid = match cut {
            Cut::Flat => rounded
                .chamfer_edges(&names, value)
                .map(|chamfered| chamfered.body),
            Cut::Round => rounded
                .fillet_edges(&names, value)
                .map(|filleted| filleted.body),
        }
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        let drawn = match cut {
            Cut::Flat => DRAWN * reach_of(&block) * PI / 2.0 * value * LENGTH,
            Cut::Round => DRAWN * reach_of(&block) * PI / 2.0 * value * (2.0 * LENGTH + BREADTH),
        };
        sound_drawn_within(&laid, promised, drawn, &what);
        assert_eq!(
            faces_made(&rounded, &laid),
            vec![start, start + 1],
            "{what}: one face for each edge, and none at the corner"
        );
        assert_eq!(
            edges_between(&laid, FRONT, RIGHT).len(),
            1,
            "{what}: the front's right edge is still one sharp edge"
        );
        corners_are(
            &laid,
            FRONT,
            &[
                DVec3::new(0.0, 0.0, value),
                DVec3::new(LENGTH, 0.0, value),
                DVec3::new(LENGTH, 0.0, HEIGHT - value),
                DVec3::new(0.0, 0.0, HEIGHT - value),
            ],
            &format!(
                "{what}: the front, from the earlier round up to the cut, its right edge ending \
                 one value short of the corner"
            ),
        );
    }
}

/// The slanted prism's corner where its end, its top and its left side
/// meet, at (30, 2, 10): across from those faces the top's edge on the end,
/// running to the right side, the top's edge on the left side, and the
/// slanted upright where the left side meets the end.
fn left_slanted_trihedral() -> Trihedral {
    Trihedral {
        at: DVec3::new(LENGTH, 2.0, 10.0),
        faces: [SLANTED_LEFT, SLANTED_END, SLANTED_TOP],
        outward: [DVec3::new(0.0, -10.0, 2.0).normalize(), DVec3::X, DVec3::Z],
        called: [
            "the top's edge on the end",
            "the top's edge on the left side",
            "the slanted upright where the left side meets the end",
        ],
        length: [10.0, LENGTH, 104f64.sqrt()],
        far: [SLANTED_RIGHT, SLANTED_START, SLANTED_BOTTOM],
    }
}

/// What the slanted prism keeps once its end's top edge and the two edges at
/// each of its ends are chamfered at `distance`, both corners closed by a
/// triangle, as the module's documentation derives it.
fn slanted_two_triangles(distance: f64) -> f64 {
    let d = distance;
    let [right, left] = [slant(), 10.0 / 104f64.sqrt()];
    SLANTED_VOLUME
        - (5.0 * d * d
            + 0.1 * d.powi(3)
            + 15.0 * (right + left) * d * d
            + 5.0 * d * d * (1.0 / right + 1.0 / left)
            - ((right + 1.0 / right) + (left + 1.0 / left)) * d.powi(3) / 3.0)
}

#[test]
fn a_chamfer_and_a_fillet_on_the_edges_at_both_ends_of_an_edge_whose_corners_are_not_square_close_both_whatever_the_order()
 {
    let value = 2.0;
    let prism = slanted_prism();
    let right = slanted_trihedral();
    let left = left_slanted_trihedral();
    let edges = [
        right.edge(0),
        right.edge(1),
        right.edge(2),
        left.edge(1),
        left.edge(2),
    ];
    let rounds = spheres_by_booleans(&prism, &[&right, &left], &edges, value);
    let [top_right, end_right, top_left, end_left, top_end] = [
        [SLANTED_TOP, SLANTED_RIGHT],
        [SLANTED_END, SLANTED_RIGHT],
        [SLANTED_TOP, SLANTED_LEFT],
        [SLANTED_END, SLANTED_LEFT],
        [SLANTED_TOP, SLANTED_END],
    ];

    closes_its_corners_whatever_the_order(
        &prism,
        &[right, left],
        &[
            vec![top_right, end_right, top_left, end_left, top_end],
            vec![top_end, top_right, end_right, top_left, end_left],
            vec![top_left, top_end, top_right, end_left, end_right],
        ],
        10.0 + 2.0 * LENGTH + 116f64.sqrt() + 104f64.sqrt(),
        value,
        |cut, _| match cut {
            Cut::Flat => slanted_two_triangles(value),
            Cut::Round => rounds,
        },
        "the slanted prism's end's top edge and the two edges at each of its ends",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_edges_named_before_a_join_merged_the_faces_at_their_corner_mitre_or_close_it_whatever_the_order()
 {
    let value = 3.0;
    let drawn_block = block([LENGTH, BREADTH, HEIGHT]);
    let [top_front, top_right, upright] = [TOP_FRONT, TOP_RIGHT, FRONT_RIGHT]
        .map(|[one, other]| edge_between(&drawn_block, one, other));
    let joined = joined_to(
        &drawn_block,
        [DVec2::new(-10.0, 0.0), DVec2::new(0.0, BREADTH)],
        0.0,
        HEIGHT,
    );
    let added = 10.0 * BREADTH * HEIGHT;
    let long = LENGTH + 10.0;
    let start = joined.faces_end();
    let mut cases = Vec::new();
    for cut in [Cut::Flat, Cut::Round] {
        let mitre = |lengths: [f64; 2]| match cut {
            Cut::Flat => mitred_by_flats(lengths, value) + added,
            Cut::Round => mitred_by_rounds(lengths, value) + added,
        };
        let closed = rounded_box(cut, long + BREADTH + HEIGHT, 1.0, value) + added;
        for (what, names, promised, patches) in [
            (
                "the top's front and right edges",
                vec![top_front.clone(), top_right.clone()],
                mitre([long, BREADTH]),
                0,
            ),
            (
                "the top's right and front edges",
                vec![top_right.clone(), top_front.clone()],
                mitre([long, BREADTH]),
                0,
            ),
            (
                "the top's front edge and the front's right edge",
                vec![top_front.clone(), upright.clone()],
                mitre([long, HEIGHT]),
                0,
            ),
            (
                "the front's right edge and the top's front edge",
                vec![upright.clone(), top_front.clone()],
                mitre([long, HEIGHT]),
                0,
            ),
            (
                "the three edges at the corner",
                vec![top_front.clone(), top_right.clone(), upright.clone()],
                closed,
                1,
            ),
            (
                "the three edges at the corner, the front's right edge first",
                vec![upright.clone(), top_front.clone(), top_right.clone()],
                closed,
                1,
            ),
        ] {
            cases.push((cut, what, names, promised, patches));
        }
    }

    let laid: Vec<Result<Body, Declined>> = cases
        .iter()
        .map(|(cut, _, names, _, _)| match cut {
            Cut::Flat => joined
                .chamfer_edges(names, value)
                .map(|chamfered| chamfered.body),
            Cut::Round => joined
                .fillet_edges(names, value)
                .map(|filleted| filleted.body),
        })
        .collect();

    let declined: Vec<String> = cases
        .iter()
        .zip(&laid)
        .filter_map(|((cut, what, ..), laid)| {
            laid.as_ref()
                .err()
                .map(|declined| format!("{cut:?} on {what}: {declined:?}"))
        })
        .collect();
    assert!(
        declined.is_empty(),
        "names taken before the join still find the corner's edges, whatever the order; \
         declined:\n{}",
        declined.join("\n")
    );
    for ((cut, what, names, promised, patches), laid) in cases.into_iter().zip(laid) {
        let what = format!("{cut:?} on {what}, named before the join");
        let body = laid.expect("declines are held above");
        let drawn = match cut {
            Cut::Flat => EXACT * promised,
            Cut::Round => {
                DRAWN * reach_of(&joined) * PI / 2.0 * value * (long + BREADTH + HEIGHT + value)
            }
        };
        sound_drawn_within(&body, promised, drawn, &what);
        assert_eq!(
            faces_made(&joined, &body),
            (start..start + names.len() + patches).collect::<Vec<_>>(),
            "{what}: one face for each edge, and a patch where the three are closed"
        );
    }
}

/// The numbers a pyramid's slopes answer to: each cut by a slab of six faces,
/// the slab's floor first, the front, the back, the left and the right in
/// turn.
const FRONT_SLOPE: usize = 6;
const BACK_SLOPE: usize = 12;
const LEFT_SLOPE: usize = 18;
const RIGHT_SLOPE: usize = 24;

/// How high a pyramid's apex stands over its base.
const APEX_HEIGHT: f64 = 5.0;

/// The base of the square pyramid, seen from above: 10 by 10.
const SQUARE: [f64; 2] = [10.0, 10.0];

/// The base of the pyramid whose faces share no sphere, seen from above: 10
/// by 6.
const OBLONG: [f64; 2] = [10.0, 6.0];

/// What the square pyramid holds: a third of its base times its height.
const PYRAMID_VOLUME: f64 = 100.0 * APEX_HEIGHT / 3.0;

/// A slope of a pyramid: the number its face answers to, the way it faces,
/// and what it is called.
struct Slope {
    face: usize,
    outward: DVec3,
    called: &'static str,
}

/// A hip of a pyramid: what it is called, the two slopes it lies between, as
/// indices into its apex's, the corner of the base it runs down to, and the
/// way it runs there from the apex.
struct Hip {
    called: &'static str,
    slopes: [usize; 2],
    foot: DVec3,
    down: DVec3,
}

/// The apex of a pyramid, where its four hips end: where it stands, its four
/// slopes, front, back, left and right, and its four hips round it.
struct Apex {
    at: DVec3,
    slopes: [Slope; 4],
    hips: [Hip; 4],
}

/// The apex of the pyramid [`pyramid_on_its_base`] stands on a base `length`
/// by `breadth`: over the middle of the base, [`APEX_HEIGHT`] high, each slope
/// running from it down to one edge of the base.
fn apex_over([length, breadth]: [f64; 2]) -> Apex {
    let at = DVec3::new(length / 2.0, breadth / 2.0, APEX_HEIGHT);
    let rise = APEX_HEIGHT;
    let slopes = [
        (FRONT_SLOPE, DVec3::new(0.0, -rise, at.y), "the front slope"),
        (BACK_SLOPE, DVec3::new(0.0, rise, at.y), "the back slope"),
        (LEFT_SLOPE, DVec3::new(-rise, 0.0, at.x), "the left slope"),
        (RIGHT_SLOPE, DVec3::new(rise, 0.0, at.x), "the right slope"),
    ]
    .map(|(face, outward, called)| Slope {
        face,
        outward: outward.normalize(),
        called,
    });
    let hips = [
        ("the front left hip", [0, 2], [0.0, 0.0]),
        ("the front right hip", [0, 3], [length, 0.0]),
        ("the back right hip", [1, 3], [length, breadth]),
        ("the back left hip", [1, 2], [0.0, breadth]),
    ]
    .map(|(called, slopes, [x, y])| {
        let foot = DVec3::new(x, y, 0.0);
        Hip {
            called,
            slopes,
            foot,
            down: (foot - at).normalize(),
        }
    });
    Apex { at, slopes, hips }
}

impl Apex {
    /// The name on the part of hip `hip`: the edge between its two slopes.
    fn name(&self, part: &Body, hip: usize) -> EdgeName {
        let [one, other] = self.hips[hip].slopes.map(|slope| self.slopes[slope].face);
        edge_between(part, one, other)
    }

    /// The two hips on slope `slope`, as indices into the apex's.
    fn hips_on(&self, slope: usize) -> [usize; 2] {
        let on: Vec<usize> = (0..4)
            .filter(|&hip| self.hips[hip].slopes.contains(&slope))
            .collect();
        [on[0], on[1]]
    }

    /// The ways hip `hip`'s two slopes face.
    fn facing(&self, hip: usize) -> [DVec3; 2] {
        self.hips[hip]
            .slopes
            .map(|slope| self.slopes[slope].outward)
    }

    /// Where the flats a chamfer of `distance` lays on the two hips of slope
    /// `slope` cross on it: the distance from each hip, square to it in the
    /// slope, so the distance over the sine of the angle the two hips meet at
    /// along each of them, as [`Trihedral::crossing`] reads a corner's.
    fn crossing(&self, slope: usize, distance: f64) -> DVec3 {
        let [one, other] = self.hips_on(slope).map(|hip| self.hips[hip].down);
        self.at + (one + other) * distance / one.cross(other).length()
    }
}

/// Where the line from `point` along hip `hip`'s way down meets the ground:
/// where a line on a slope beside the hip, parallel to it, crosses the base's
/// edge under that slope.
fn down_to_the_base(apex: &Apex, hip: usize, point: DVec3) -> DVec3 {
    let down = apex.hips[hip].down;
    point - down * point.z / down.z
}

/// The block a pyramid on a base `length` by `breadth` is cut from: standing
/// on the ground, one wider than the base all round and one higher than its
/// apex, so that every plane through the apex and an edge of the base cuts
/// the block's floor along that edge and takes its walls and top away whole.
fn block_under([length, breadth]: [f64; 2]) -> Body {
    raised(
        Contour::rectangle(DVec2::splat(-1.0), DVec2::new(length + 1.0, breadth + 1.0)),
        APEX_HEIGHT + 1.0,
    )
}

/// A pyramid standing on its own base, `length` by `breadth`, its apex where
/// [`apex_over`] puts it: [`block_under`] cut by the four planes through the
/// apex and an edge of the base, its base the block's floor, numbered nought,
/// its slopes [`FRONT_SLOPE`], [`BACK_SLOPE`], [`LEFT_SLOPE`] and
/// [`RIGHT_SLOPE`]. Four edges end at the apex, one between each two slopes
/// beside each other; three at each corner of the base, its hip and the two
/// edges of the base there.
///
/// #699 held its decline on a pyramid standing on a box, the block 10 by 10 by
/// 10 cut down to its walls' halfway. Each hip's foot there is a vertex where
/// four edges end, the hip, two eaves and the box's upright, and a cut ending
/// at such a vertex, alone in its list, is declined before any corner is read:
/// a behaviour of one edge, #943's. The pyramid on the box waits on that
/// issue; the apex is held here on pyramids standing on their own base.
fn pyramid_on_its_base(footprint: [f64; 2]) -> Body {
    let apex = apex_over(footprint);
    apex.slopes
        .iter()
        .fold(block_under(footprint), |part, slope| {
            part.difference(&beyond(&part, apex.at, slope.outward, 50.0))
                .expect("a plane through the apex cuts the block")
        })
}

/// The reach a pyramid of [`pyramid_on_its_base`] is drawn at: a cut keeps
/// the larger reach of the part and of the slab cut from it, and the slabs
/// reach far past the block.
fn pyramid_reach(footprint: [f64; 2]) -> f64 {
    let apex = apex_over(footprint);
    let block = block_under(footprint);
    apex.slopes
        .iter()
        .map(|slope| reach_of(&beyond(&block, apex.at, slope.outward, 50.0)))
        .fold(reach_of(&block), f64::max)
}

/// What the square pyramid keeps once its four hips are rounded at `radius`
/// and its apex closed by a piece of sphere: each round's section along its
/// hip from its end section to the plane square to the hip at its foot, less
/// what the ground spares of the round's prism there, taken away, then the
/// cell the end sections bound at the apex, the ball's sector in it given
/// back, as the module's documentation derives it.
fn pyramid_closed_by_a_sphere(radius: f64) -> f64 {
    let r = radius;
    let section = r * r * (1.0 / 3f64.sqrt() - PI / 6.0);
    let along = APEX_HEIGHT * 3f64.sqrt() - r * (2.0f64 / 3.0).sqrt();
    let at_the_foot = -(2f64.sqrt()) * r.powi(3) * (11.0 / 18.0 - PI / (3.0 * 3f64.sqrt()));
    let cell = 4.0 * 2f64.sqrt() * r.powi(3) / 9.0;
    let sector = 4.0 * (1.0f64 / 3.0).asin() * r.powi(3) / 3.0;
    PYRAMID_VOLUME - 4.0 * (section * along + at_the_foot) - cell + sector
}

/// What the square pyramid keeps once its four hips are chamfered at
/// `distance` and its apex closed by a square: the tip above the square, and
/// each flat's prism below it, taken away, as the module's documentation
/// derives it.
fn pyramid_closed_by_a_square(distance: f64) -> f64 {
    PYRAMID_VOLUME - 15.0 * distance * distance + 6f64.sqrt() / 2.0 * distance.powi(3)
}

/// The hips an order names, as the user reads them.
fn hips_called(apex: &Apex, order: &[usize]) -> String {
    order
        .iter()
        .map(|&hip| apex.hips[hip].called)
        .collect::<Vec<_>>()
        .join(", then ")
}

/// Where `edge` stands in `order`.
fn rank_in(order: &[usize], edge: usize) -> usize {
    order
        .iter()
        .position(|&named| named == edge)
        .expect("every edge is named")
}

/// What a step of `cut` at `value` on the hips of `apex`, named in each of
/// `orders`, leaves on `part`: the value asked, the value taken and the body,
/// each order apart, or why it declined.
fn laid_on_the_hips(
    part: &Body,
    apex: &Apex,
    orders: &[Vec<usize>],
    cut: Cut,
    value: f64,
) -> Vec<Result<(f64, f64, Body), Declined>> {
    orders
        .iter()
        .map(|order| {
            let names: Vec<EdgeName> = order.iter().map(|&hip| apex.name(part, hip)).collect();
            match cut {
                Cut::Flat => part
                    .chamfer_edges(&names, value)
                    .map(|chamfered| (chamfered.asked, chamfered.taken, chamfered.body)),
                Cut::Round => part
                    .fillet_edges(&names, value)
                    .map(|filleted| (filleted.asked, filleted.taken, filleted.body)),
            }
        })
        .collect()
}

/// Holds that a step was laid in every one of `orders`, and hands back what
/// it laid in each.
fn laid_in_every_order(
    apex: &Apex,
    orders: &[Vec<usize>],
    laid: Vec<Result<(f64, f64, Body), Declined>>,
    what: &str,
) -> Vec<(f64, f64, Body)> {
    let declined: Vec<String> = orders
        .iter()
        .zip(&laid)
        .filter_map(|(order, laid)| {
            laid.as_ref()
                .err()
                .map(|declined| format!("{}: {declined:?}", hips_called(apex, order)))
        })
        .collect();
    assert!(
        declined.is_empty(),
        "{what} whatever the order; declined:\n{}",
        declined.join("\n")
    );
    laid.into_iter()
        .map(|laid| laid.expect("declines are held above"))
        .collect()
}

/// Holds that no flat face of the body has a corner at any of `points`.
fn no_corner_left_at(body: &Body, points: &[DVec3], what: &str) {
    for face in numbers(body) {
        if let Some(plane) = body.plane_of(face) {
            for at in points {
                assert!(
                    plane
                        .corners
                        .iter()
                        .all(|point| point.distance(*at) > PLACE),
                    "{what}: face {face} still has a corner at the old vertex {at}"
                );
            }
        }
    }
}

/// Holds that the base of a pyramid whose hips were cut, face nought, meets
/// each slope along one sharp edge, and the face `laid` hands for each hip,
/// and nothing else.
fn base_meets_the_slopes_and_what_was_laid(
    body: &Body,
    apex: &Apex,
    laid: impl Fn(usize) -> usize,
    what: &str,
) {
    for slope in &apex.slopes {
        assert_eq!(
            edges_between(body, FLOOR, slope.face).len(),
            1,
            "{what}: the base's edge under {} is still one sharp edge",
            slope.called
        );
    }
    let mut met: Vec<usize> = apex.slopes.iter().map(|slope| slope.face).collect();
    met.extend((0..4).map(laid));
    assert_eq!(
        sides_met(body, FLOOR),
        sides(&met),
        "{what}: the base meets the four slopes along its edges and what was laid on each hip \
         at its foot, by one edge each"
    );
}

/// Holds that the bodies a step laid on the same edges named in each of
/// `orders`, one body each, are one body: every face numbered from `start` on
/// and short of `patch` taken from the rank its edge has in the first order to
/// the rank it has in this one, and every other number kept, each face meets
/// what it meets in the first order, and each flat face has the corners it has
/// there.
fn lays_one_body_whatever_the_order(
    orders: &[Vec<usize>],
    bodies: &[Body],
    start: usize,
    patch: usize,
    what: &str,
) {
    let (first_order, first) = (&orders[0], &bodies[0]);
    for (order, body) in orders.iter().zip(bodies).skip(1) {
        let which = format!("{what}, named {order:?}, against the same named {first_order:?}");
        let to_this = |number: usize| {
            if number < start || number >= patch {
                return number;
            }
            start + rank_in(order, first_order[number - start])
        };
        for face in numbers(first) {
            assert_eq!(
                sides_renumbered(first, face, to_this),
                sides_met(body, to_this(face)),
                "{which}: face {face} meets what it meets in the first order"
            );
            if let Some(plane) = first.plane_of(face) {
                corners_are(
                    body,
                    to_this(face),
                    &plane.corners,
                    &format!("{which}: face {face}, as in the first order"),
                );
            }
        }
    }
}

/// Holds that the fillet at `r` on the four hips of the square pyramid, in
/// every order, closes its apex with a piece of the sphere of radius `r`
/// one radius in from its four slopes, as the first criterion of #804 reads
/// it at 1.
fn closes_the_apex_on_its_sphere(r: f64) {
    let pyramid = pyramid_on_its_base(SQUARE);
    let apex = apex_over(SQUARE);
    let centre = DVec3::new(5.0, 5.0, APEX_HEIGHT - 2f64.sqrt() * r);
    for slope in &apex.slopes {
        let depth = (apex.at - centre).dot(slope.outward);
        assert!(
            (depth - r).abs() < PLACE,
            "the centre {centre} stands {depth} in from {}, where a point (5, 5, z) stands \
             |z − 5|/√2 in from each slope: one radius",
            slope.called
        );
    }
    let start = pyramid.faces_end();
    let piece = start + 4;
    let rounds = [0, 1, 2, 3].map(|hip| Round {
        start: centre,
        along: apex.hips[hip].down,
        length: apex.hips[hip].foot.distance(apex.at),
        radius: r,
        facing: apex.facing(hip),
    });
    let spread = 4.0 * (1.0f64 / 3.0).asin();
    let area = rounds.iter().map(Round::area).sum::<f64>() + spread * r * r;
    let drawn_within = DRAWN * pyramid_reach(SQUARE) * area;
    let promised = pyramid_closed_by_a_sphere(r);
    let orders = every_order(4);

    let laid = laid_in_every_order(
        &apex,
        &orders,
        laid_on_the_hips(&pyramid, &apex, &orders, Cut::Round, r),
        &format!("the fillet at {r} on the pyramid's four hips closes its apex"),
    );

    let mut bodies = Vec::new();
    for (order, (asked, taken, body)) in orders.iter().zip(laid) {
        let what = format!("the fillet on {}", hips_called(&apex, order));
        taken_as_asked(asked, taken, r, &what);
        sound_drawn_within(&body, promised, drawn_within, &what);
        let round_of = |hip: usize| start + rank_in(order, hip);
        assert_eq!(
            faces_made(&pyramid, &body),
            (start..=piece).collect::<Vec<_>>(),
            "{what}: one round for each hip, numbered in the order the step names them, and the \
             piece of sphere after them: {:?}",
            numbers(&body)
        );
        assert_eq!(
            body.faces_end(),
            piece + 1,
            "{what}: the next step numbers its own faces past the piece of sphere"
        );
        assert!(
            numbers(&pyramid).is_subset(&numbers(&body)),
            "{what}: a face of the pyramid went: {:?}",
            numbers(&body)
        );

        let which = format!("{what}: the piece of sphere");
        assert!(
            !body.is_flat(piece) && body.plane_of(piece).is_none(),
            "{which} is curved, and offers no plane to draw on"
        );
        let drawn: Vec<DVec3> = body.triangles_of(piece).flatten().collect();
        assert!(!drawn.is_empty(), "{which} is drawn");
        for point in &drawn {
            assert!(
                (point.distance(centre) - r).abs() < PLACE,
                "{which} is drawn through {point}, off the sphere of {r} about {centre}, one \
                 radius from each of the four slopes"
            );
            for hip in &apex.hips {
                assert!(
                    (*point - centre).dot(hip.down) <= PLACE,
                    "{which} is drawn through {point}, past the end section of the round on {}",
                    hip.called
                );
            }
        }
        for slope in &apex.slopes {
            let foot = centre + slope.outward * r;
            assert!(
                drawn.iter().any(|point| point.distance(foot) < PLACE),
                "{which}: a corner at {foot}, where it touches {}",
                slope.called
            );
        }
        for weights in [
            [1.0, 1.0, 1.0, 1.0],
            [2.0, 1.0, 1.0, 1.0],
            [1.0, 2.0, 1.0, 1.0],
            [1.0, 1.0, 2.0, 1.0],
            [1.0, 1.0, 1.0, 2.0],
        ] {
            let facing = apex
                .slopes
                .iter()
                .zip(weights)
                .map(|(slope, weight)| slope.outward * weight)
                .sum::<DVec3>()
                .normalize();
            let from = centre + facing * (r + 10.0);
            let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
                panic!("{which}: a ray from {from} toward its centre meets nothing")
            });
            assert_eq!(
                hit.face, piece,
                "{which}: a ray from {from} toward its centre meets face {} before it",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{which}: where a ray from {from} meets it, it faces {}, not {facing}, away from \
                 its centre",
                hit.normal
            );
        }
        assert_eq!(
            sides_met(&body, piece),
            sides(&[0, 1, 2, 3].map(round_of)),
            "{which} meets the four rounds, one side each, and nothing else"
        );

        for (hip, round) in rounds.iter().enumerate() {
            let face = round_of(hip);
            let which = format!("{what}: the round of {}", apex.hips[hip].called);
            assert!(
                !body.is_flat(face) && body.plane_of(face).is_none(),
                "{which} is curved, and offers no plane to draw on"
            );
            let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
            assert!(!corners.is_empty(), "{which} is drawn");
            for point in &corners {
                assert!(
                    round.holds(*point),
                    "{which} is drawn through {point}, off its cylinder of {r} about the line \
                     through {centre} along its hip, or short of its end section there"
                );
            }
            let section: Vec<&DVec3> = corners
                .iter()
                .filter(|point| ((**point - centre).dot(round.along)).abs() < PLACE)
                .collect();
            for way in round.facing {
                let foot = centre + way * r;
                assert!(
                    section.iter().any(|point| point.distance(foot) < PLACE),
                    "{which}: its end section, an arc in the plane through the sphere's centre \
                     square to its hip, ends at {foot}, among {section:?}"
                );
            }
            assert!(
                distinct_points(&section) > 2,
                "{which}: its end section is drawn along the arc between its two ends: \
                 {section:?}"
            );
            let on_the_base: Vec<&DVec3> = corners
                .iter()
                .filter(|point| point.z.abs() < PLACE)
                .collect();
            for way in round.facing {
                let base = down_to_the_base(&apex, hip, centre + way * r);
                assert!(
                    on_the_base.iter().any(|point| point.distance(base) < PLACE),
                    "{which}: at its foot it ends on the base, as at a corner of three edges, \
                     from {base} on an edge of the base, among {on_the_base:?}"
                );
            }
            assert!(
                on_the_base.len() > 2,
                "{which}: its end on the base is drawn along the curve the base cuts from its \
                 cylinder: {on_the_base:?}"
            );
            met_from_outside(
                &body,
                face,
                &round.part(r, 0.7 * round.length),
                &format!("{which}, past its end section"),
            );
            let [one, other] = apex.hips[hip].slopes.map(|slope| apex.slopes[slope].face);
            assert_eq!(
                sides_met(&body, face),
                sides(&[one, other, FLOOR, piece]),
                "{which} meets its two slopes along its sides, the base at its foot and the piece \
                 of sphere along its end section, by one edge each, and no other round"
            );
        }
        for (index, slope) in apex.slopes.iter().enumerate() {
            let foot = centre + slope.outward * r;
            let [one, other] = apex.hips_on(index);
            corners_are(
                &body,
                slope.face,
                &[
                    foot,
                    down_to_the_base(&apex, one, foot),
                    down_to_the_base(&apex, other, foot),
                ],
                &format!(
                    "{what}: {}, a triangle from where the piece of sphere touches it down \
                     along the rounds of its two hips to the base",
                    slope.called
                ),
            );
            assert_eq!(
                sides_met(&body, slope.face),
                sides(&[FLOOR, round_of(one), round_of(other)]),
                "{what}: {} meets the base and the rounds of its two hips, by one edge each, and \
                 the piece of sphere at a corner only",
                slope.called
            );
        }
        base_meets_the_slopes_and_what_was_laid(&body, &apex, round_of, &what);
        let corners_of_the_base: Vec<DVec3> = apex.hips.iter().map(|hip| hip.foot).collect();
        no_corner_left_at(&body, &[apex.at], &what);
        no_corner_left_at(&body, &corners_of_the_base, &what);
        bodies.push(body);
    }
    lays_one_body_whatever_the_order(
        &orders,
        &bodies,
        start,
        piece,
        "the fillet on the pyramid's four hips",
    );
}

#[test]
fn a_fillet_on_the_four_hips_of_a_pyramid_closes_its_apex_with_a_piece_of_sphere_one_radius_from_its_four_faces_whatever_the_order()
 {
    closes_the_apex_on_its_sphere(1.0);
}

#[test]
fn a_fillet_on_the_four_hips_of_a_pyramid_at_half_a_unit_closes_its_apex_on_the_sphere_of_that_radius_whatever_the_order()
 {
    closes_the_apex_on_its_sphere(0.5);
}

#[test]
fn a_chamfer_on_the_four_hips_of_a_pyramid_closes_its_apex_with_a_square_where_the_flats_edges_cross_whatever_the_order()
 {
    let d = 1.0;
    let pyramid = pyramid_on_its_base(SQUARE);
    let apex = apex_over(SQUARE);
    let crossings = [0, 1, 2, 3].map(|slope| apex.crossing(slope, d));
    let level = APEX_HEIGHT - 1.5f64.sqrt() * d;
    for (slope, crossing) in apex.slopes.iter().zip(crossings) {
        assert!(
            (crossing.z - level).abs() < PLACE,
            "the flats of the two hips on {} cross at {crossing}, where the arithmetic puts \
             all four crossings: {level} high",
            slope.called
        );
    }
    let start = pyramid.faces_end();
    let square = start + 4;
    let promised = pyramid_closed_by_a_square(d);
    let orders = every_order(4);

    let laid = laid_in_every_order(
        &apex,
        &orders,
        laid_on_the_hips(&pyramid, &apex, &orders, Cut::Flat, d),
        "the chamfer at 1 on the pyramid's four hips closes its apex",
    );

    let mut bodies = Vec::new();
    for (order, (asked, taken, body)) in orders.iter().zip(laid) {
        let what = format!("the chamfer on {}", hips_called(&apex, order));
        taken_as_asked(asked, taken, d, &what);
        sound_drawn_within(&body, promised, EXACT * promised, &what);
        let flat_of = |hip: usize| start + rank_in(order, hip);
        assert_eq!(
            faces_made(&pyramid, &body),
            (start..=square).collect::<Vec<_>>(),
            "{what}: one flat for each hip, numbered in the order the step names them, and the \
             square after them: {:?}",
            numbers(&body)
        );
        assert_eq!(
            body.faces_end(),
            square + 1,
            "{what}: the next step numbers its own faces past the square"
        );
        assert!(
            numbers(&pyramid).is_subset(&numbers(&body)),
            "{what}: a face of the pyramid went: {:?}",
            numbers(&body)
        );

        corners_are(
            &body,
            square,
            &crossings,
            &format!(
                "{what}: the square, its corners where the flats' edges cross on the four slopes"
            ),
        );
        faces_toward(
            &body,
            square,
            DVec3::Z,
            &format!("{what}: the square, on a plane square to the pyramid's axis"),
        );
        assert_eq!(
            sides_met(&body, square),
            sides(&[0, 1, 2, 3].map(flat_of)),
            "{what}: the square meets the four flats, one side each, and nothing else"
        );

        let mut on_the_base = Vec::new();
        for (hip, at) in apex.hips.iter().enumerate() {
            let flat = flat_of(hip);
            let which = format!("{what}: the flat of {}", at.called);
            let [one, other] = at.slopes;
            let [facing_one, facing_other] = apex.facing(hip);
            faces_toward(&body, flat, (facing_one + facing_other).normalize(), &which);
            let [cross_one, cross_other] = [crossings[one], crossings[other]];
            let [base_one, base_other] =
                [cross_one, cross_other].map(|crossing| down_to_the_base(&apex, hip, crossing));
            on_the_base.extend([base_one, base_other]);
            corners_are(
                &body,
                flat,
                &[cross_one, cross_other, base_one, base_other],
                &format!(
                    "{which}, from its side on the square between the crossings on its two \
                     slopes down to the base, which it meets along a straight side, as at a \
                     corner of three edges"
                ),
            );
            assert_eq!(
                sides_met(&body, flat),
                sides(&[
                    square,
                    apex.slopes[one].face,
                    apex.slopes[other].face,
                    FLOOR
                ]),
                "{which} meets the square, its two slopes along its sides and the base at its \
                 foot, by one edge each, and no other flat"
            );
        }
        for (index, slope) in apex.slopes.iter().enumerate() {
            let crossing = crossings[index];
            let [one, other] = apex.hips_on(index);
            corners_are(
                &body,
                slope.face,
                &[
                    crossing,
                    down_to_the_base(&apex, one, crossing),
                    down_to_the_base(&apex, other, crossing),
                ],
                &format!(
                    "{what}: {}, a triangle from where the flats of its two hips cross on it \
                     down to the base",
                    slope.called
                ),
            );
            assert_eq!(
                sides_met(&body, slope.face),
                sides(&[FLOOR, flat_of(one), flat_of(other)]),
                "{what}: {} meets the base and the flats of its two hips, by one edge each, and \
                 the square at a corner only",
                slope.called
            );
        }
        corners_are(
            &body,
            FLOOR,
            &on_the_base,
            &format!(
                "{what}: the base, an octagon, each corner of it cut off by the flat of the hip \
                 there"
            ),
        );
        base_meets_the_slopes_and_what_was_laid(&body, &apex, flat_of, &what);
        no_corner_left_at(&body, &[apex.at], &what);
        bodies.push(body);
    }
    lays_one_body_whatever_the_order(
        &orders,
        &bodies,
        start,
        square,
        "the chamfer on the pyramid's four hips",
    );
}

#[test]
fn a_fillet_and_a_chamfer_on_three_of_the_four_edges_at_a_pyramid_s_apex_are_declined_as_unsupported()
 {
    let pyramid = pyramid_on_its_base(SQUARE);
    assert!(
        (pyramid.volume() - PYRAMID_VOLUME).abs() < EXACT * PYRAMID_VOLUME,
        "the pyramid on its base holds {}",
        pyramid.volume()
    );
    let apex = apex_over(SQUARE);

    for (left_out, sharp) in apex.hips.iter().enumerate() {
        let three: Vec<usize> = (0..4).filter(|&hip| hip != left_out).collect();
        for order in ORDERS_OF_THREE {
            let list = order.map(|at| three[at]);
            let names = list.map(|hip| apex.name(&pyramid, hip));
            let what = format!("{}, {} left sharp", hips_called(&apex, &list), sharp.called);

            let filleted = pyramid.fillet_edges(&names, 1.0).err();
            let chamfered = pyramid.chamfer_edges(&names, 1.0).err();

            assert_eq!(
                filleted,
                Some(Declined::Unsupported),
                "the fillet on {what}: the hip left out runs into the apex sharp, which the \
                 piece of sphere does not reach — a broken step and its reason, and no body, \
                 until #942"
            );
            assert_eq!(
                chamfered,
                Some(Declined::Unsupported),
                "the chamfer on {what}: the hip left out runs into the apex sharp, which the \
                 square does not reach — a broken step and its reason, and no body, until #942"
            );
        }
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_the_four_hips_of_a_pyramid_10_by_6_whose_faces_share_no_sphere_are_declined_as_unsupported_whatever_the_order()
 {
    let pyramid = pyramid_on_its_base(OBLONG);
    let [length, breadth] = OBLONG;
    let volume = length * breadth * APEX_HEIGHT / 3.0;
    assert!(
        (pyramid.volume() - volume).abs() < EXACT * volume,
        "the pyramid on its base 10 by 6 holds {}, where {volume} is promised",
        pyramid.volume()
    );
    let apex = apex_over(OBLONG);
    let below = apex.at - DVec3::Z;
    let [front, back, left, right] = apex
        .slopes
        .each_ref()
        .map(|slope| (apex.at - below).dot(slope.outward));
    assert!(
        (front - back).abs() < PLACE && (left - right).abs() < PLACE && (left - front).abs() > 0.1,
        "the point of the axis one under the apex stands {front} in from the front and the \
         back and {left} in from the left and the right: every point of the axis stands in \
         from the four in those proportions, so none stands as far in from all four, and no \
         sphere touches them all"
    );
    let [on_front, on_back, on_left, on_right] =
        [0, 1, 2, 3].map(|slope| apex.crossing(slope, 1.0));
    let off_plane = (on_back - on_front).dot((on_left - on_front).cross(on_right - on_front));
    assert!(
        off_plane.abs() > 0.1,
        "the flats at 1 cross on the front and the back at {on_front} and {on_back}, on the \
         left and the right at {on_left} and {on_right}: on no one plane"
    );

    for order in every_order(4) {
        let names: Vec<EdgeName> = order.iter().map(|&hip| apex.name(&pyramid, hip)).collect();
        let what = hips_called(&apex, &order);

        let filleted = pyramid.fillet_edges(&names, 1.0).err();
        let chamfered = pyramid.chamfer_edges(&names, 1.0).err();

        assert_eq!(
            filleted,
            Some(Declined::Unsupported),
            "the fillet on {what}, the four slopes touching no one sphere: a broken step and \
             its reason, and no body, until #942"
        );
        assert_eq!(
            chamfered,
            Some(Declined::Unsupported),
            "the chamfer on {what}, the four flats' crossings on no one plane: a broken step and \
             its reason, and no body, until #942"
        );
    }
}

/// The base of the pyramid whose four crossings lie a little off one plane,
/// seen from above: 10 by 9.8.
const NEARLY_SQUARE: [f64; 2] = [10.0, 9.8];

#[test]
fn a_chamfer_on_the_four_hips_of_a_pyramid_10_by_9_8_whose_crossings_lie_a_little_off_one_plane_is_declined_as_unsupported_whatever_the_order()
 {
    let pyramid = pyramid_on_its_base(NEARLY_SQUARE);
    let apex = apex_over(NEARLY_SQUARE);
    let [on_front, on_back, on_left, on_right] =
        [0, 1, 2, 3].map(|slope| apex.crossing(slope, 1.0));
    let across = (on_back - on_front).cross(on_right - on_front).normalize();
    let off = (on_left - on_front).dot(across).abs();
    assert!(
        off > 0.01 && off < 0.05,
        "the flats at 1 cross on the left {off} off the plane through the three other \
         crossings: a little, far past what the kernel takes for one place"
    );

    for order in every_order(4) {
        let names: Vec<EdgeName> = order.iter().map(|&hip| apex.name(&pyramid, hip)).collect();

        let chamfered = pyramid.chamfer_edges(&names, 1.0).err();

        assert_eq!(
            chamfered,
            Some(Declined::Unsupported),
            "the chamfer on {}, the four flats' crossings a little off one plane: a broken step \
             and its reason, and no body, until #942",
            hips_called(&apex, &order)
        );
    }
}

/// The base of the pyramid whose four crossings lie a hair off one plane,
/// past the kernel's tolerance, seen from above: 10 by 9.999999.
const A_HAIR_OFF_SQUARE: [f64; 2] = [10.0, 9.999999];

#[test]
fn a_chamfer_on_the_four_hips_of_a_pyramid_whose_crossings_lie_a_hair_off_one_plane_past_the_kernel_s_tolerance_is_declined_as_unsupported_whatever_the_order()
 {
    let pyramid = pyramid_on_its_base(A_HAIR_OFF_SQUARE);
    let apex = apex_over(A_HAIR_OFF_SQUARE);
    let [on_front, on_back, on_left, on_right] =
        [0, 1, 2, 3].map(|slope| apex.crossing(slope, 1.0));
    let across = (on_back - on_front).cross(on_right - on_front).normalize();
    let off = (on_left - on_front).dot(across).abs();
    let tolerance = 1e-9 * pyramid_reach(A_HAIR_OFF_SQUARE);
    assert!(
        off > 1.2 * tolerance && off < 2.0 * tolerance,
        "the flats at 1 cross on the left {off} off the plane through the three other \
         crossings: past the kernel's tolerance, {tolerance}, a billionth of the reach the \
         pyramid is drawn at, and short of twice it"
    );

    for order in every_order(4) {
        let names: Vec<EdgeName> = order.iter().map(|&hip| apex.name(&pyramid, hip)).collect();

        let chamfered = pyramid.chamfer_edges(&names, 1.0).err();

        assert_eq!(
            chamfered,
            Some(Declined::Unsupported),
            "the chamfer on {}, the four flats' crossings a hair off one plane, past what the \
             kernel takes for one place: a broken step and its reason, and no body, until #942",
            hips_called(&apex, &order)
        );
    }
}

/// How thick the plate [`pyramid_on_a_plate`] stands on is, and how far the
/// plate reaches past the pyramid's base all round.
const PLATE_THICKNESS: f64 = 2.0;
const PLATE_MARGIN: f64 = 5.0;

/// The apex of [`pyramid_on_a_plate`]: the square pyramid's, raised onto the
/// plate's top, its slopes numbered six past [`apex_over`]'s, as the prism
/// they are cut from is numbered past the plate's six faces.
fn apex_on_a_plate() -> Apex {
    let mut apex = apex_over(SQUARE);
    let up = DVec3::Z * PLATE_THICKNESS;
    apex.at += up;
    for slope in &mut apex.slopes {
        slope.face += 6;
    }
    for hip in &mut apex.hips {
        hip.foot += up;
    }
    apex
}

/// The square pyramid of [`apex_over`], 10 by 10 and 5 high, standing on a
/// plate 20 by 20 by 2 that reaches [`PLATE_MARGIN`] past its base all round:
/// the plate raised first, its top numbered one; then a prism over the base,
/// on the plate's top, numbered past the plate's six faces, cut by the planes
/// through the apex and each edge of the base, their faces numbered six apart
/// past the prism's — the front, back, left and right slopes twelve,
/// eighteen, twenty-four and thirty — and joined to the plate. The feet,
/// where the slopes meet the plate's top, meet inwards, the hips outwards: at
/// each corner of the base the hip above two feet is the odd edge of a corner
/// of mixed sense, and the four hips meet at the apex.
fn pyramid_on_a_plate() -> Body {
    let [length, breadth] = SQUARE;
    let plate = raised(
        Contour::rectangle(
            DVec2::splat(-PLATE_MARGIN),
            DVec2::new(length + PLATE_MARGIN, breadth + PLATE_MARGIN),
        ),
        PLATE_THICKNESS,
    );
    let apex = apex_on_a_plate();
    let prism = plate
        .tool_raised(
            &profile(Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth))),
            Frame {
                origin: DVec3::Z * PLATE_THICKNESS,
                ..ground()
            },
            DVec3::Z * (APEX_HEIGHT + 1.0),
        )
        .expect("the prism raises");
    let pyramid = apex.slopes.iter().fold(prism, |part, slope| {
        part.difference(&beyond(&part, apex.at, slope.outward, 50.0))
            .expect("a plane through the apex cuts the prism")
    });
    plate
        .union(&pyramid)
        .expect("the pyramid stands on the plate")
}

/// The reach [`pyramid_on_a_plate`] is drawn at: the plate's, or a slab's
/// the pyramid was cut by, whichever is the larger.
fn plate_reach(part: &Body) -> f64 {
    let apex = apex_on_a_plate();
    apex.slopes
        .iter()
        .map(|slope| reach_of(&beyond(part, apex.at, slope.outward, 50.0)))
        .fold(reach_of(part), f64::max)
}

/// What [`pyramid_on_a_plate`] keeps once its four hips and its four feet are
/// chamfered at 1, as the branch of #804 measured it at f2b6bdda: no
/// arithmetic holds it, the fills along the hips' flats at their feet having
/// no closed form here.
const PLATE_CHAMFERED: f64 = 965.567266914305;

/// Forty-eight of the orders eight edges can be named in, spread over them
/// all: a stride prime to their count through [`every_order`]'s list.
fn spread_orders_of_eight() -> Vec<Vec<usize>> {
    let all = every_order(8);
    (0..48)
        .map(|k| all[(k * 7919 + 13) % all.len()].clone())
        .collect()
}

/// Whether two lists of points hold the same points, whatever their order
/// and however often each is listed: a face lists a corner once for each of
/// its edges that ends there.
fn same_points(one: &[DVec3], other: &[DVec3]) -> bool {
    let within = |some: &[DVec3], all: &[DVec3]| {
        some.iter()
            .all(|point| all.iter().any(|theirs| theirs.distance(*point) < PLACE))
    };
    within(one, other) && within(other, one)
}

/// How many of `points` stand apart, each counted once however often it is
/// listed: a face's triangles list a corner once for each triangle at it.
fn distinct_points(points: &[&DVec3]) -> usize {
    (points.iter().enumerate())
        .filter(|&(index, point)| {
            (points[..index].iter()).all(|earlier| earlier.distance(**point) >= PLACE)
        })
        .count()
}

#[test]
fn a_chamfer_on_the_hips_and_feet_of_a_pyramid_on_a_plate_closes_its_apex_and_goes_round_each_hip_at_its_foot_in_one_body_and_a_fillet_is_declined_whatever_the_order()
 {
    let part = pyramid_on_a_plate();
    let plate = 20.0 * 20.0 * PLATE_THICKNESS;
    assert!(
        (part.volume() - (plate + PYRAMID_VOLUME)).abs() < EXACT * part.volume(),
        "the pyramid on its plate holds {}",
        part.volume()
    );
    let apex = apex_on_a_plate();
    let reach = plate_reach(&part);
    let hips: Vec<EdgeName> = (0..4).map(|hip| apex.name(&part, hip)).collect();
    let feet: Vec<EdgeName> = (apex.slopes.iter())
        .map(|slope| edge_between(&part, TOP, slope.face))
        .collect();

    let rounded = part
        .fillet_edges(&hips, 1.0)
        .unwrap_or_else(|declined| panic!("the fillet on the four hips alone: {declined:?}"));
    let flattened = part
        .chamfer_edges(&hips, 1.0)
        .unwrap_or_else(|declined| panic!("the chamfer on the four hips alone: {declined:?}"));

    let area = 4.0 * PI / 3.0 * 5.0 * 3f64.sqrt() + 4.0 * (1.0f64 / 3.0).asin();
    sound_drawn_within(
        &rounded.body,
        plate + pyramid_closed_by_a_sphere(1.0),
        DRAWN * reach * area,
        "the fillet on the four hips alone, the apex closed on the plate as on its own base",
    );
    sound_drawn_within(
        &flattened.body,
        plate + pyramid_closed_by_a_square(1.0),
        EXACT * flattened.body.volume(),
        "the chamfer on the four hips alone, the apex closed on the plate as on its own base",
    );

    let feet_rounded = part
        .fillet_edges(&feet, 1.0)
        .unwrap_or_else(|declined| panic!("the fillet on the four feet alone: {declined:?}"));
    let triangles = feet_rounded.body.triangles();
    assert!(
        feet_rounded.body.is_exact()
            && closed(&triangles) == Ok(())
            && uncrossed(&triangles) == Ok(()),
        "the fillet on the four feet alone, each hip left sharp over them: exact, closed and \
         uncrossed"
    );

    let edges: Vec<EdgeName> = hips.iter().chain(&feet).cloned().collect();
    let start = part.faces_end();
    let laid_end = start + 8 + 5;
    let square_at = [0, 1, 2, 3].map(|slope| apex.crossing(slope, 1.0));
    let mut bodies: Vec<(Vec<usize>, Body)> = Vec::new();
    for order in spread_orders_of_eight() {
        let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].clone()).collect();
        let what = format!("the four hips and the four feet, named {order:?}");

        let filleted = part.fillet_edges(&names, 1.0).err();
        let chamfered = part
            .chamfer_edges(&names, 1.0)
            .unwrap_or_else(|declined| panic!("the chamfer on {what}: declined, {declined:?}"));

        assert_eq!(
            filleted,
            Some(Declined::Unsupported),
            "the fillet on {what}: each hip the odd edge at its foot, which the fillet does not \
             go round on this pyramid, though its four hips alone close the apex — a broken step \
             and its reason, and no body"
        );
        let what = format!("the chamfer on {what}");
        taken_as_asked(chamfered.asked, chamfered.taken, 1.0, &what);
        let body = chamfered.body;
        sound_drawn_within(&body, PLATE_CHAMFERED, EXACT * PLATE_CHAMFERED, &what);
        assert_eq!(
            faces_made(&part, &body),
            (start..laid_end).collect::<Vec<_>>(),
            "{what}: one flat for each edge by its rank, then five patches, the apex's square \
             and one at each foot: {:?}",
            numbers(&body)
        );
        assert_eq!(
            body.faces_end(),
            laid_end,
            "{what}: the next step numbers its own faces past the patches"
        );
        let squares: Vec<usize> = (start + 8..laid_end)
            .filter(|&patch| {
                body.plane_of(patch).is_some_and(|plane| {
                    plane.normal.distance(DVec3::Z) < PLACE
                        && same_points(&plane.corners, &square_at)
                })
            })
            .collect();
        assert_eq!(
            squares.len(),
            1,
            "{what}: one patch is the apex's square, level, its corners where the hips' flats \
             cross on the four slopes"
        );
        bodies.push((order, body));
    }

    let (first_order, first) = &bodies[0];
    let patches: Vec<usize> = (start + 8..laid_end).collect();
    let patch_planes: Vec<_> = patches
        .iter()
        .map(|&patch| {
            first
                .plane_of(patch)
                .unwrap_or_else(|| panic!("the chamfer's patch {patch} is flat"))
        })
        .collect();
    for (order, body) in bodies.iter().skip(1) {
        let which = format!(
            "the chamfer on the four hips and the four feet, named {order:?}, against the same \
             named {first_order:?}"
        );
        let patch_here: Vec<usize> = patch_planes
            .iter()
            .map(|plane| {
                *patches
                    .iter()
                    .find(|&&patch| {
                        body.plane_of(patch).is_some_and(|here| {
                            here.normal.distance(plane.normal) < PLACE
                                && same_points(&here.corners, &plane.corners)
                        })
                    })
                    .unwrap_or_else(|| {
                        panic!("{which}: no patch where the first order lays one at {plane:?}")
                    })
            })
            .collect();
        let to_this = |number: usize| {
            if number < start || number >= laid_end {
                number
            } else if number < start + 8 {
                start + rank_in(order, first_order[number - start])
            } else {
                patch_here[number - start - 8]
            }
        };
        for face in numbers(first) {
            assert_eq!(
                sides_renumbered(first, face, to_this),
                sides_met(body, to_this(face)),
                "{which}: face {face} meets what it meets in the first order"
            );
            if let Some(plane) = first.plane_of(face) {
                corners_are(
                    body,
                    to_this(face),
                    &plane.corners,
                    &format!("{which}: face {face}, as in the first order"),
                );
            }
        }
    }
}

/// What a step of `cut` at `value` on `names` leaves on `part`: the value
/// asked, the value taken and the body, or why it declined.
fn cut_on(
    part: &Body,
    names: &[EdgeName],
    cut: Cut,
    value: f64,
) -> Result<(f64, f64, Body), Declined> {
    match cut {
        Cut::Flat => part
            .chamfer_edges(names, value)
            .map(|chamfered| (chamfered.asked, chamfered.taken, chamfered.body)),
        Cut::Round => part
            .fillet_edges(names, value)
            .map(|filleted| (filleted.asked, filleted.taken, filleted.body)),
    }
}

/// What a step of `cut` at `value` on `edges`, named in each of `orders` by
/// their places in `edges`, leaves on `part`, each order apart, as [`cut_on`]
/// reads it.
fn laid_on_the_edges(
    part: &Body,
    edges: &[EdgeName],
    orders: &[Vec<usize>],
    cut: Cut,
    value: f64,
) -> Vec<Result<(f64, f64, Body), Declined>> {
    orders
        .iter()
        .map(|order| {
            let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].clone()).collect();
            cut_on(part, &names, cut, value)
        })
        .collect()
}

/// Holds that a step was laid in every one of `orders`, each named by the
/// places of its edges, and hands back what it laid in each.
fn laid_in_every_listed_order(
    orders: &[Vec<usize>],
    laid: Vec<Result<(f64, f64, Body), Declined>>,
    what: &str,
) -> Vec<(f64, f64, Body)> {
    let declined: Vec<String> = orders
        .iter()
        .zip(&laid)
        .filter_map(|(order, laid)| {
            laid.as_ref()
                .err()
                .map(|declined| format!("named {order:?}: {declined:?}"))
        })
        .collect();
    assert!(
        declined.is_empty(),
        "{what} whatever the order; declined in {} of {} orders:\n{}",
        declined.len(),
        orders.len(),
        declined.join("\n")
    );
    laid.into_iter()
        .map(|laid| laid.expect("declines are held above"))
        .collect()
}

/// The patches `body` lays from `patches` on, one for each of `first`'s
/// patches, found by their planes and their corners: the numbers this body
/// gives the faces the first body numbers `patches`, every patch flat.
fn flat_patches_alike(first: &Body, body: &Body, patches: &[usize], which: &str) -> Vec<usize> {
    patches
        .iter()
        .map(|&patch| {
            let plane = first
                .plane_of(patch)
                .unwrap_or_else(|| panic!("{which}: the first order's patch {patch} is flat"));
            *patches
                .iter()
                .find(|&&here| {
                    body.plane_of(here).is_some_and(|laid| {
                        laid.normal.distance(plane.normal) < PLACE
                            && same_points(&laid.corners, &plane.corners)
                    })
                })
                .unwrap_or_else(|| {
                    panic!("{which}: no patch where the first order lays one at {plane:?}")
                })
        })
        .collect()
}

/// The patches `body` lays from `patches` on, one for each of `first`'s
/// patches, found by their spheres: the numbers this body gives the faces the
/// first body numbers `patches`, each patch a piece of the sphere of 1 about
/// one of `centres`.
fn round_patches_alike(
    first: &Body,
    body: &Body,
    patches: &[usize],
    centres: &[DVec3],
    which: &str,
) -> Vec<usize> {
    patches
        .iter()
        .map(|&patch| {
            let corner = sphere_of(first, patch, centres)
                .unwrap_or_else(|| panic!("{which}: the first order's patch {patch}"));
            *patches
                .iter()
                .find(|&&here| sphere_of(body, here, centres) == Some(corner))
                .unwrap_or_else(|| {
                    panic!("{which}: no patch on the sphere about {}", centres[corner])
                })
        })
        .collect()
}

/// Holds that each of `bodies` past the first is the first, face for face:
/// every face numbered from `start` on and short of `start + edges` taken from
/// the rank its edge has in the first order to the rank it has in this one,
/// every patch past them through `patches`, which hands, from the first order
/// and its body and this order and its body, the number this body gives each
/// of the first's patches, and every other number kept.
fn one_body_face_for_face(
    bodies: &[(Vec<usize>, Body)],
    start: usize,
    edges: usize,
    patches: impl Fn((&[usize], &Body), (&[usize], &Body), &str) -> Vec<usize>,
    what: &str,
) {
    let (first_order, first) = &bodies[0];
    for (order, body) in bodies.iter().skip(1) {
        let which = format!("{what}, named {order:?}, against the same named {first_order:?}");
        let patch_here = patches((first_order, first), (order, body), &which);
        let laid_end = start + edges + patch_here.len();
        let to_this = |number: usize| {
            if number < start || number >= laid_end {
                number
            } else if number < start + edges {
                start + rank_in(order, first_order[number - start])
            } else {
                patch_here[number - start - edges]
            }
        };
        for face in numbers(first) {
            assert_eq!(
                sides_renumbered(first, face, to_this),
                sides_met(body, to_this(face)),
                "{which}: face {face} meets what it meets in the first order"
            );
            if let Some(plane) = first.plane_of(face) {
                corners_are(
                    body,
                    to_this(face),
                    &plane.corners,
                    &format!("{which}: face {face}, as in the first order"),
                );
            }
        }
    }
}

/// The reach a body holding the pyramids of `apexes` is drawn at: its own, or
/// that of a slab one of the pyramids was cut by, whichever is the larger.
fn slabs_reach(part: &Body, apexes: &[&Apex]) -> f64 {
    apexes
        .iter()
        .flat_map(|apex| {
            apex.slopes
                .iter()
                .map(|slope| reach_of(&beyond(part, apex.at, slope.outward, 50.0)))
        })
        .fold(reach_of(part), f64::max)
}

/// Where the cube of [`pyramid_with_a_cube_above`] stands, and its side: 2.1
/// above the apex, off the axis, wholly inside the cell the end sections
/// bound at the apex when the clearing is read from some hips and wholly
/// outside it when read from another, as the review of #804 found.
const CUBE_CENTRE: DVec3 = DVec3::new(3.973, 3.949, 7.098);
const CUBE_SIDE: f64 = 0.2;

/// The square pyramid on its base with a cube of side [`CUBE_SIDE`] standing
/// apart in the air above its apex, about [`CUBE_CENTRE`], joined to it in one
/// body: its six faces numbered thirty to thirty-five, past the pyramid's.
fn pyramid_with_a_cube_above() -> Body {
    pyramid_with_a_cube(CUBE_CENTRE, CUBE_SIDE)
}

/// The square pyramid on its base with a cube of side `side` about `centre`,
/// standing apart in the air, joined to it in one body, its six faces
/// numbered thirty to thirty-five, past the pyramid's.
fn pyramid_with_a_cube(centre: DVec3, side: f64) -> Body {
    let pyramid = pyramid_on_its_base(SQUARE);
    let half = DVec2::splat(side / 2.0);
    let cube = pyramid
        .tool_raised(
            &profile(Contour::rectangle(
                centre.truncate() - half,
                centre.truncate() + half,
            )),
            Frame {
                origin: DVec3::Z * (centre.z - side / 2.0),
                ..ground()
            },
            DVec3::Z * side,
        )
        .expect("the cube raises");
    pyramid
        .union(&cube)
        .expect("the cube stands apart in the same body")
}

/// The corners of the cube faces thirty to thirty-five of `part` hold, face
/// by face.
fn cube_corners(part: &Body) -> Vec<Vec<DVec3>> {
    (30..36)
        .map(|face| {
            part.plane_of(face)
                .unwrap_or_else(|| panic!("face {face} of the cube is flat"))
                .corners
        })
        .collect()
}

/// Holds that a step of `cut` at 1 on the four hips of
/// [`pyramid_with_a_cube_above`], in every order, closes the apex and keeps
/// the cube whole: the volume the pyramid's closing leaves with the cube's
/// added, the cube's six faces as they were, and one body in every order.
fn keeps_the_cube_whole(cut: Cut) {
    let part = pyramid_with_a_cube_above();
    let cube_volume = CUBE_SIDE.powi(3);
    assert!(
        (part.volume() - (PYRAMID_VOLUME + cube_volume)).abs() < EXACT * part.volume(),
        "the pyramid and the cube hold {}",
        part.volume()
    );
    let apex = apex_over(SQUARE);
    let cube_faces: Vec<usize> = (30..36).collect();
    let cube_corners = cube_corners(&part);
    let clear = (cube_corners.iter().flatten())
        .flat_map(|corner| {
            (apex.slopes.iter()).map(move |slope| (*corner - apex.at).dot(slope.outward))
        })
        .fold(f64::INFINITY, f64::min);
    assert!(
        clear > 0.5 && CUBE_CENTRE.z - apex.at.z > 2.0,
        "the cube stands {clear} clear of every slope's plane, its centre over 2 above the apex, \
         out of every round's and every flat's reach"
    );
    let start = part.faces_end();
    let patch = start + 4;
    let reach = pyramid_reach(SQUARE);
    let orders = every_order(4);

    let (kind, promised, drawn) = match cut {
        Cut::Round => {
            let area = 4.0 * PI / 3.0 * 5.0 * 3f64.sqrt() + 4.0 * (1.0f64 / 3.0).asin();
            let promised = pyramid_closed_by_a_sphere(1.0) + cube_volume;
            ("fillet", promised, DRAWN * reach * area)
        }
        Cut::Flat => {
            let promised = pyramid_closed_by_a_square(1.0) + cube_volume;
            ("chamfer", promised, EXACT * promised)
        }
    };
    let laid = laid_in_every_order(
        &apex,
        &orders,
        laid_on_the_hips(&part, &apex, &orders, cut, 1.0),
        &format!(
            "the {kind} at 1 on the pyramid's four hips, a cube standing apart above its \
             apex, closes the apex and keeps the cube"
        ),
    );
    let mut bodies = Vec::new();
    for (order, (asked, taken, body)) in orders.iter().zip(laid) {
        let what = format!("the {kind} on {}", hips_called(&apex, order));
        taken_as_asked(asked, taken, 1.0, &what);
        sound_drawn_within(&body, promised, drawn, &what);
        assert_eq!(
            faces_made(&part, &body),
            (start..=patch).collect::<Vec<_>>(),
            "{what}: one face for each hip by its rank, and the patch after them: {:?}",
            numbers(&body)
        );
        for (face, corners) in cube_faces.iter().zip(&cube_corners) {
            corners_are(
                &body,
                *face,
                corners,
                &format!("{what}: face {face} of the cube, kept whole"),
            );
            assert_eq!(
                sides_met(&body, *face),
                sides_met(&part, *face),
                "{what}: face {face} of the cube meets the cube's other faces only"
            );
        }
        bodies.push(body);
    }
    lays_one_body_whatever_the_order(
        &orders,
        &bodies,
        start,
        patch,
        &format!("the {kind} on the four hips, the cube above"),
    );
}

#[test]
fn a_fillet_on_the_four_hips_of_a_pyramid_keeps_whole_a_cube_standing_apart_above_its_apex_whatever_the_order()
 {
    keeps_the_cube_whole(Cut::Round);
}

#[test]
fn a_chamfer_on_the_four_hips_of_a_pyramid_keeps_whole_a_cube_standing_apart_above_its_apex_whatever_the_order()
 {
    keeps_the_cube_whole(Cut::Flat);
}

/// Where the cube of the next test stands, and its side: on the pyramid's
/// axis 1.3 above its apex, inside the cube about the sphere's centre the
/// fillet clears there, twice the way from the centre to the apex on every
/// side, and on the apex's side of the planes through the centre square to the
/// hips, so that the clearing would take it whole; out of every round's reach.
const LOW_CUBE_CENTRE: DVec3 = DVec3::new(5.0, 5.0, 6.3);
const LOW_CUBE_SIDE: f64 = 0.1;

#[test]
fn a_fillet_whose_clearing_would_take_whole_a_cube_standing_just_above_the_apex_is_declined_and_a_chamfer_keeps_the_cube_whatever_the_order()
 {
    let part = pyramid_with_a_cube(LOW_CUBE_CENTRE, LOW_CUBE_SIDE);
    let apex = apex_over(SQUARE);
    let centre = apex.at - DVec3::Z * 2f64.sqrt();
    let half = 2.0 * centre.distance(apex.at);
    let corners = cube_corners(&part);
    for corner in corners.iter().flatten() {
        let off = *corner - centre;
        assert!(
            off.z.abs() < half
                && off.truncate().length() < half
                && apex.hips.iter().all(|hip| off.dot(hip.down) < 0.0),
            "the cube's corner {corner} stands inside what the fillet at 1 clears about the \
             sphere's centre {centre}: within {half} of it every way, on the apex's side of the \
             four planes square to the hips"
        );
        for hip in &apex.hips {
            let from_line = (*corner - apex.at).reject_from(hip.down).length();
            assert!(
                from_line > 0.9,
                "the cube's corner {corner} stands {from_line} from the line of {}, farther than \
                 the cube above the apex the rounds leave whole",
                hip.called
            );
        }
    }
    let orders = every_order(4);
    let start = part.faces_end();

    for (order, laid) in orders
        .iter()
        .zip(laid_on_the_hips(&part, &apex, &orders, Cut::Round, 1.0))
    {
        assert_eq!(
            laid.err(),
            Some(Declined::Unsupported),
            "the fillet on {}: its clearing about the sphere's centre would take the cube whole, \
             a broken step and its reason rather than a body short of the cube, until #948",
            hips_called(&apex, order)
        );
    }

    let promised = pyramid_closed_by_a_square(1.0) + LOW_CUBE_SIDE.powi(3);
    let laid = laid_in_every_order(
        &apex,
        &orders,
        laid_on_the_hips(&part, &apex, &orders, Cut::Flat, 1.0),
        "the chamfer at 1 on the pyramid's four hips, a cube just above its apex, closes the \
         apex and keeps the cube",
    );
    let mut bodies = Vec::new();
    for (order, (asked, taken, body)) in orders.iter().zip(laid) {
        let what = format!("the chamfer on {}", hips_called(&apex, order));
        taken_as_asked(asked, taken, 1.0, &what);
        sound_drawn_within(&body, promised, EXACT * promised, &what);
        for (face, kept) in (30..36).zip(&corners) {
            corners_are(
                &body,
                face,
                kept,
                &format!("{what}: face {face} of the cube, kept whole"),
            );
        }
        bodies.push(body);
    }
    lays_one_body_whatever_the_order(
        &orders,
        &bodies,
        start,
        start + 4,
        "the chamfer on the four hips, a cube just above the apex",
    );
}

/// What [`pyramid_on_a_plate`] keeps once its four hips and its front foot
/// are chamfered at 1, as the branch of #804 measured it at 8c2ae55e: no
/// arithmetic holds it, the foot's fill ending at both its ends beside a hip
/// with the other foot there left sharp.
const PLATE_HIPS_AND_FRONT_FOOT_CHAMFERED: f64 = 956.595690631586;

#[test]
fn a_chamfer_on_the_four_hips_of_a_pyramid_on_a_plate_and_one_of_its_feet_closes_its_apex_in_one_body_whatever_the_order()
 {
    let part = pyramid_on_a_plate();
    let apex = apex_on_a_plate();
    let mut edges: Vec<EdgeName> = (0..4).map(|hip| apex.name(&part, hip)).collect();
    edges.push(edge_between(&part, TOP, apex.slopes[0].face));
    let all = every_order(5);
    let orders: Vec<Vec<usize>> = (0..24)
        .map(|k| all[(k * 7 + 3) % all.len()].clone())
        .collect();
    let start = part.faces_end();
    let square = [0, 1, 2, 3].map(|slope| apex.crossing(slope, 1.0));

    let laid = laid_in_every_listed_order(
        &orders,
        laid_on_the_edges(&part, &edges, &orders, Cut::Flat, 1.0),
        "the chamfer at 1 on the pyramid's four hips and its front foot closes the apex",
    );
    let laid_end = laid[0].2.faces_end();
    let mut bodies = Vec::new();
    for (order, (asked, taken, body)) in orders.iter().zip(laid) {
        let what = format!("the chamfer on the four hips and the front foot, named {order:?}");
        taken_as_asked(asked, taken, 1.0, &what);
        sound_drawn_within(
            &body,
            PLATE_HIPS_AND_FRONT_FOOT_CHAMFERED,
            EXACT * PLATE_HIPS_AND_FRONT_FOOT_CHAMFERED,
            &what,
        );
        assert_eq!(
            faces_made(&part, &body),
            (start..laid_end).collect::<Vec<_>>(),
            "{what}: one flat for each edge by its rank, then the patches, as many in every \
             order: {:?}",
            numbers(&body)
        );
        let squares = (start + 5..laid_end)
            .filter(|&patch| {
                body.plane_of(patch).is_some_and(|plane| {
                    plane.normal.distance(DVec3::Z) < PLACE && same_points(&plane.corners, &square)
                })
            })
            .count();
        assert_eq!(
            squares, 1,
            "{what}: one patch is the apex's square, level, at its four crossings"
        );
        bodies.push((order.clone(), body));
    }
    let patches: Vec<usize> = (start + 5..laid_end).collect();
    one_body_face_for_face(
        &bodies,
        start,
        5,
        |(_, first), (_, body), which| flat_patches_alike(first, body, &patches, which),
        "the chamfer on the four hips and the front foot",
    );
}

/// How far the second of [`two_pyramids`] stands from the first, along X:
/// ten clear of it.
const BESIDE: DVec3 = DVec3::new(20.0, 0.0, 0.0);

/// The apex of the second of [`two_pyramids`]: the square pyramid's, moved by
/// [`BESIDE`], its slopes numbered thirty past the first's, as the block it is
/// cut from is numbered past the first pyramid's faces.
fn second_apex() -> Apex {
    let mut apex = apex_over(SQUARE);
    apex.at += BESIDE;
    for slope in &mut apex.slopes {
        slope.face += 30;
    }
    for hip in &mut apex.hips {
        hip.foot += BESIDE;
    }
    apex
}

/// Two square pyramids standing on their own bases in one body, apart: the
/// first [`pyramid_on_its_base`]'s, the second the same moved by [`BESIDE`],
/// cut from a block raised past the first's faces, its base numbered thirty
/// and its slopes thirty-six to fifty-four, then joined to it.
fn two_pyramids() -> Body {
    let first = pyramid_on_its_base(SQUARE);
    let apex = second_apex();
    let [length, breadth] = SQUARE;
    let block = first
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::splat(-1.0) + BESIDE.truncate(),
                DVec2::new(length + 1.0, breadth + 1.0) + BESIDE.truncate(),
            )),
            ground(),
            DVec3::Z * (APEX_HEIGHT + 1.0),
        )
        .expect("the second block raises");
    let second = apex.slopes.iter().fold(block, |part, slope| {
        part.difference(&beyond(&part, apex.at, slope.outward, 50.0))
            .expect("a plane through the second apex cuts the block")
    });
    first
        .union(&second)
        .expect("the second pyramid stands apart in the same body")
}

/// Holds that a step of `cut` at 1 on the hips of either of [`two_pyramids`]
/// alone leaves that pyramid closed and the other whole, and on the eight
/// hips, in a spread of orders, closes both apexes in one body: twice what
/// one pyramid's closing leaves, each apex's patch numbered by the lowest
/// rank of its hips, and one body in every order.
fn closes_both_apexes(cut: Cut) {
    let part = two_pyramids();
    assert!(
        (part.volume() - 2.0 * PYRAMID_VOLUME).abs() < EXACT * part.volume(),
        "the two pyramids hold {}",
        part.volume()
    );
    let apexes = [apex_over(SQUARE), second_apex()];
    let hips: Vec<EdgeName> = (apexes.iter())
        .flat_map(|apex| (0..4).map(|hip| apex.name(&part, hip)))
        .collect();
    let reach = slabs_reach(&part, &[&apexes[0], &apexes[1]]);
    let start = part.faces_end();
    let laid_end = start + 8 + 2;
    let orders = spread_orders_of_eight();
    let centres = apexes
        .each_ref()
        .map(|apex| apex.at - DVec3::Z * 2f64.sqrt());
    let squares = apexes
        .each_ref()
        .map(|apex| [0, 1, 2, 3].map(|slope| apex.crossing(slope, 1.0)));

    let (kind, kept) = match cut {
        Cut::Round => ("fillet", pyramid_closed_by_a_sphere(1.0)),
        Cut::Flat => ("chamfer", pyramid_closed_by_a_square(1.0)),
    };
    let within = |promised: f64, pyramids: f64| match cut {
        Cut::Round => {
            let area = 4.0 * PI / 3.0 * 5.0 * 3f64.sqrt() + 4.0 * (1.0f64 / 3.0).asin();
            DRAWN * reach * area * pyramids
        }
        Cut::Flat => EXACT * promised,
    };
    for index in 0..2 {
        let what = format!("the {kind} at 1 on the hips of pyramid {index} alone");
        let (asked, taken, body) = cut_on(&part, &hips[4 * index..4 * index + 4], cut, 1.0)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
        taken_as_asked(asked, taken, 1.0, &what);
        let promised = PYRAMID_VOLUME + kept;
        sound_drawn_within(&body, promised, within(promised, 1.0), &what);
    }

    let laid = laid_in_every_listed_order(
        &orders,
        laid_on_the_edges(&part, &hips, &orders, cut, 1.0),
        &format!("the {kind} at 1 on the eight hips of two pyramids closes both apexes"),
    );
    let mut bodies = Vec::new();
    for (order, (asked, taken, body)) in orders.iter().zip(laid) {
        let what = format!("the {kind} on the eight hips, named {order:?}");
        taken_as_asked(asked, taken, 1.0, &what);
        let promised = 2.0 * kept;
        sound_drawn_within(&body, promised, within(promised, 2.0), &what);
        assert_eq!(
            faces_made(&part, &body),
            (start..laid_end).collect::<Vec<_>>(),
            "{what}: one face for each hip by its rank, then one patch for each apex: {:?}",
            numbers(&body)
        );
        assert_eq!(
            body.faces_end(),
            laid_end,
            "{what}: the next step numbers its own faces past the two patches"
        );
        let lowest = [0, 1].map(|index| {
            (4 * index..4 * index + 4)
                .map(|hip| rank_in(order, hip))
                .min()
                .expect("four hips")
        });
        for index in 0..2 {
            let patch = start + 8 + usize::from(lowest[index] > lowest[1 - index]);
            let which = format!(
                "{what}: patch {patch}, closing apex {index}, by the lowest rank of its hips"
            );
            match cut {
                Cut::Round => {
                    let drawn: Vec<DVec3> = body.triangles_of(patch).flatten().collect();
                    assert!(!drawn.is_empty(), "{which} is drawn");
                    for point in drawn {
                        assert!(
                            (point.distance(centres[index]) - 1.0).abs() < PLACE,
                            "{which}: drawn through {point}, off the sphere of 1 about {}",
                            centres[index]
                        );
                    }
                }
                Cut::Flat => corners_are(
                    &body,
                    patch,
                    &squares[index],
                    &format!("{which}, the square through its four crossings"),
                ),
            }
            let rounds: Vec<usize> = (4 * index..4 * index + 4)
                .map(|hip| start + rank_in(order, hip))
                .collect();
            assert_eq!(
                sides_met(&body, patch),
                sides(&rounds),
                "{which} meets the faces laid on its four hips, and nothing else"
            );
        }
        bodies.push((order.clone(), body));
    }
    one_body_face_for_face(
        &bodies,
        start,
        8,
        |(first_order, _), (order, _), _| {
            let patch_of = |order: &[usize], index: usize| {
                let lowest = |index: usize| {
                    (4 * index..4 * index + 4)
                        .map(|hip| rank_in(order, hip))
                        .min()
                        .expect("four hips")
                };
                start + 8 + usize::from(lowest(index) > lowest(1 - index))
            };
            let mut here = vec![0; 2];
            for index in 0..2 {
                here[patch_of(first_order, index) - start - 8] = patch_of(order, index);
            }
            here
        },
        &format!("the {kind} on the eight hips of two pyramids"),
    );
}

#[test]
fn a_fillet_on_the_hips_of_two_pyramids_in_one_step_closes_both_apexes_in_one_body_whatever_the_order()
 {
    closes_both_apexes(Cut::Round);
}

#[test]
fn a_chamfer_on_the_hips_of_two_pyramids_in_one_step_closes_both_apexes_in_one_body_whatever_the_order()
 {
    closes_both_apexes(Cut::Flat);
}

/// How far the square pyramid's faces stand from the centre of the sphere
/// they all touch, on its axis: the base's distance from it is its height,
/// a slope's is 5 less that height over √2, and the two are one.
fn pyramid_s_inradius() -> f64 {
    5.0 / (1.0 + 2f64.sqrt())
}

/// What the square pyramid keeps once its eight edges are rounded at
/// `radius`, its apex and the four corners of its base each closed by a
/// piece of sphere: the pyramid opened by the ball of `radius`, which every
/// face of it touches. Its faces stood `radius` in, the pyramid shrinks about
/// the centre of its inscribed sphere by `k = (ρ − r)/ρ`; the ball swells the
/// shrunk pyramid back, by Steiner's formula, into its own matter, its area
/// times the radius, the length of its edges times the angle each turns
/// through by half the radius squared — its rounds — and the ball, its
/// corners' pieces making one whole. The base's edges turn through 3π/4, the
/// hips, of 5√3, through π/3.
fn pyramid_opened_by_a_ball(radius: f64) -> f64 {
    let k = (pyramid_s_inradius() - radius) / pyramid_s_inradius();
    let area = 100.0 + 100.0 * 2f64.sqrt();
    let turns = (4.0 * 10.0 * 3.0 * PI / 4.0 + 4.0 * 5.0 * 3f64.sqrt() * PI / 3.0) / 2.0;
    k.powi(3) * PYRAMID_VOLUME
        + k * k * area * radius
        + k * turns * radius * radius
        + 4.0 * PI * radius.powi(3) / 3.0
}

/// The corners of the square pyramid shrunk as [`pyramid_opened_by_a_ball`]
/// shrinks it at a radius of 1: its apex, √2 under the pyramid's, then the
/// corner over each hip's foot, 1 up, in the order of the apex's hips.
fn shrunk_pyramid_s_corners(apex: &Apex) -> [DVec3; 5] {
    let k = (pyramid_s_inradius() - 1.0) / pyramid_s_inradius();
    let middle = DVec3::new(apex.at.x, apex.at.y, 1.0);
    let [one, two, three, four] = apex.hips.each_ref().map(|hip| {
        let foot = DVec3::new(hip.foot.x, hip.foot.y, 1.0);
        middle + (foot - middle) * k
    });
    [apex.at - DVec3::Z * 2f64.sqrt(), one, two, three, four]
}

/// Which of `centres` every point drawn of the face numbered `patch` stands
/// 1 from, if one.
fn sphere_of(body: &Body, patch: usize, centres: &[DVec3]) -> Option<usize> {
    let drawn: Vec<DVec3> = body.triangles_of(patch).flatten().collect();
    centres.iter().position(|centre| {
        !drawn.is_empty()
            && drawn
                .iter()
                .all(|point| (point.distance(*centre) - 1.0).abs() < PLACE)
    })
}

/// What the square pyramid keeps once its four hips and the four edges of its
/// base are chamfered at 1, the apex closed by its square and each corner of
/// the base by a triangle through its three crossings: the pyramid cut by the
/// eight flats' planes, the square's and the four triangles', a convex body,
/// measured by slicing it with level planes, each slice's area exact.
const HIPS_AND_BASE_CHAMFERED: f64 = 141.77296518494913;

/// The square pyramid on its base, its four hips and the four edges of its
/// base, as [`pyramid_on_its_base`] numbers and names them, the hips first.
fn pyramid_and_its_eight_edges() -> (Body, Apex, Vec<EdgeName>) {
    let part = pyramid_on_its_base(SQUARE);
    let apex = apex_over(SQUARE);
    let hips = (0..4).map(|hip| apex.name(&part, hip));
    let base = (apex.slopes.iter()).map(|slope| edge_between(&part, FLOOR, slope.face));
    let edges = hips.chain(base).collect();
    (part, apex, edges)
}

/// Holds the fillet at 1 on the square pyramid's eight edges in each of
/// `orders`: the value taken whole; the body exact, closed and uncrossed,
/// holding [`pyramid_opened_by_a_ball`] at 1, its triangles enclosing that
/// within the drawing's tolerance over its rounds and its five pieces of
/// sphere; one round for each edge by its rank and five patches after them,
/// each a piece of the sphere of 1 about one corner of the shrunk pyramid,
/// each of the five taken once, meeting the rounds of that corner's edges
/// and nothing else; and every order laying the body the first lays, each
/// patch found by its sphere.
fn pyramid_s_eight_edges_rounded_in(orders: &[Vec<usize>]) {
    let (part, apex, edges) = pyramid_and_its_eight_edges();
    let start = part.faces_end();
    let laid_end = start + 8 + 5;
    let centres = shrunk_pyramid_s_corners(&apex);
    let at_corners: [Vec<usize>; 5] = [
        vec![0, 1, 2, 3],
        vec![0, 4 + apex.hips[0].slopes[0], 4 + apex.hips[0].slopes[1]],
        vec![1, 4 + apex.hips[1].slopes[0], 4 + apex.hips[1].slopes[1]],
        vec![2, 4 + apex.hips[2].slopes[0], 4 + apex.hips[2].slopes[1]],
        vec![3, 4 + apex.hips[3].slopes[0], 4 + apex.hips[3].slopes[1]],
    ];
    let opened = pyramid_opened_by_a_ball(1.0);
    let k = (pyramid_s_inradius() - 1.0) / pyramid_s_inradius();
    let curved = k * (4.0 * 10.0 * 3.0 * PI / 4.0 + 4.0 * 5.0 * 3f64.sqrt() * PI / 3.0) + 4.0 * PI;
    let patches: Vec<usize> = (start + 8..laid_end).collect();
    let laid = laid_in_every_listed_order(
        orders,
        laid_on_the_edges(&part, &edges, orders, Cut::Round, 1.0),
        "the fillet at 1 on the four hips and the four base edges closes the apex and the four \
         corners of the base",
    );
    let mut bodies = Vec::new();
    for (order, (asked, taken, body)) in orders.iter().zip(laid) {
        let what = format!("the fillet on the four hips and the four base edges, named {order:?}");
        taken_as_asked(asked, taken, 1.0, &what);
        sound_drawn_within(&body, opened, DRAWN * pyramid_reach(SQUARE) * curved, &what);
        assert_eq!(
            faces_made(&part, &body),
            (start..laid_end).collect::<Vec<_>>(),
            "{what}: one round for each edge by its rank, then five patches, a piece of sphere \
             at the apex and at each corner of the base: {:?}",
            numbers(&body)
        );
        let mut taken_once = [false; 5];
        for &patch in &patches {
            let corner = sphere_of(&body, patch, &centres).unwrap_or_else(|| {
                panic!(
                    "{what}: patch {patch} is drawn on none of the spheres of 1 about the shrunk \
                     pyramid's corners {centres:?}"
                )
            });
            assert!(
                !taken_once[corner],
                "{what}: two patches on the sphere about {}",
                centres[corner]
            );
            taken_once[corner] = true;
            let rounds: Vec<usize> = (at_corners[corner].iter())
                .map(|&edge| start + rank_in(order, edge))
                .collect();
            assert_eq!(
                sides_met(&body, patch),
                sides(&rounds),
                "{what}: patch {patch}, on the sphere about {}, meets the rounds of that \
                 corner's edges, and nothing else",
                centres[corner]
            );
        }
        bodies.push((order.clone(), body));
    }
    one_body_face_for_face(
        &bodies,
        start,
        8,
        |(_, first), (_, body), which| round_patches_alike(first, body, &patches, &centres, which),
        "the fillet on the four hips and the four base edges",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_the_hips_and_the_base_edges_of_a_pyramid_close_its_apex_and_its_four_base_corners_in_one_body_whatever_the_order()
 {
    let (part, apex, edges) = pyramid_and_its_eight_edges();
    let base = &edges[4..];
    let rounded_base = part
        .fillet_edges(base, 1.0)
        .unwrap_or_else(|declined| panic!("the fillet on the four base edges alone: {declined:?}"));
    let triangles = rounded_base.body.triangles();
    assert!(
        rounded_base.body.is_exact()
            && closed(&triangles) == Ok(())
            && uncrossed(&triangles) == Ok(()),
        "the fillet on the four base edges alone, each corner of the base closed among two \
         rounds and its hip left sharp: exact, closed and uncrossed"
    );
    let start = part.faces_end();
    let laid_end = start + 8 + 5;
    let orders = spread_orders_of_eight();
    let square = [0, 1, 2, 3].map(|slope| apex.crossing(slope, 1.0));
    let every_fourth: Vec<Vec<usize>> = orders.iter().step_by(4).cloned().collect();
    pyramid_s_eight_edges_rounded_in(&every_fourth);

    let laid = laid_in_every_listed_order(
        &orders,
        laid_on_the_edges(&part, &edges, &orders, Cut::Flat, 1.0),
        "the chamfer at 1 on the four hips and the four base edges closes the apex and the four \
         corners of the base",
    );
    let mut bodies = Vec::new();
    for (order, (asked, taken, body)) in orders.iter().zip(laid) {
        let what = format!("the chamfer on the four hips and the four base edges, named {order:?}");
        taken_as_asked(asked, taken, 1.0, &what);
        sound_drawn_within(
            &body,
            HIPS_AND_BASE_CHAMFERED,
            EXACT * HIPS_AND_BASE_CHAMFERED,
            &what,
        );
        assert_eq!(
            faces_made(&part, &body),
            (start..laid_end).collect::<Vec<_>>(),
            "{what}: one flat for each edge by its rank, then five patches, the apex's square \
             and a triangle at each corner of the base: {:?}",
            numbers(&body)
        );
        let squares: Vec<usize> = (start + 8..laid_end)
            .filter(|&patch| {
                body.plane_of(patch).is_some_and(|plane| {
                    plane.normal.distance(DVec3::Z) < PLACE && same_points(&plane.corners, &square)
                })
            })
            .collect();
        assert_eq!(
            squares.len(),
            1,
            "{what}: one patch is the apex's square, level, at its four crossings"
        );
        bodies.push((order.clone(), body));
    }
    let patches: Vec<usize> = (start + 8..laid_end).collect();
    one_body_face_for_face(
        &bodies,
        start,
        8,
        |(_, first), (_, body), which| flat_patches_alike(first, body, &patches, which),
        "the chamfer on the four hips and the four base edges",
    );
}

/// The hipped block's breadth and height: it is [`LENGTH`] along X, 20
/// across and 10 high.
const HIPPED_BREADTH: f64 = 20.0;
const HIPPED_HEIGHT: f64 = 10.0;

/// The numbers the hipped block's slopes answer to: each cut by a slab of six
/// faces, the slab's floor first — the front, then the left end, then the
/// right end.
const HIPPED_FRONT: usize = 6;
const HIPPED_LEFT: usize = 12;
const HIPPED_RIGHT: usize = 18;

/// What the hipped block holds: its level section `z` high is `30 − 2z` by
/// `20 − z`.
const HIPPED_VOLUME: f64 = LENGTH * HIPPED_BREADTH * HIPPED_HEIGHT
    - (LENGTH + 2.0 * HIPPED_BREADTH) * HIPPED_HEIGHT * HIPPED_HEIGHT / 2.0
    + 2.0 * HIPPED_HEIGHT * HIPPED_HEIGHT * HIPPED_HEIGHT / 3.0;

/// The block the hipped block is cut from: one wider than it at the front and
/// at each end, so that each slope's plane cuts the floor along the edge it
/// rises from and takes a wall away whole.
fn hipped_block_under() -> Body {
    raised(
        Contour::rectangle(DVec2::splat(-1.0), DVec2::new(LENGTH + 1.0, HIPPED_BREADTH)),
        HIPPED_HEIGHT,
    )
}

/// The planes the hipped block's slopes lie on, front, left end and right
/// end: a point each passes through, on the floor, and the way it faces.
fn hipped_slopes() -> [(DVec3, DVec3); 3] {
    [
        (DVec3::ZERO, DVec3::new(0.0, -1.0, 1.0).normalize()),
        (DVec3::ZERO, DVec3::new(-1.0, 0.0, 1.0).normalize()),
        (
            DVec3::new(LENGTH, 0.0, 0.0),
            DVec3::new(1.0, 0.0, 1.0).normalize(),
        ),
    ]
}

/// The block 30 by 20 by 10 on the ground's origin whose front and two ends
/// slope in at 45 degrees from the edges of its floor, as a hipped roof
/// does, its back upright and its top level: [`hipped_block_under`] cut by
/// [`hipped_slopes`], its floor numbered nought, its top one, its back four,
/// its slopes [`HIPPED_FRONT`], [`HIPPED_LEFT`] and [`HIPPED_RIGHT`]. Its
/// front's foot, where the front meets the floor at 45 degrees, ends at two
/// corners of three edges, the foot, a hip and an end's foot: the square
/// pyramid's base corners, and no apex.
fn hipped_block() -> Body {
    hipped_slopes()
        .iter()
        .fold(hipped_block_under(), |part, &(through, away)| {
            part.difference(&beyond(&part, through, away, 50.0))
                .expect("a slope's plane cuts the block")
        })
}

/// The reach the hipped block is drawn at: a cut keeps the larger reach of
/// the part and of the slab cut from it.
fn hipped_reach() -> f64 {
    let block = hipped_block_under();
    hipped_slopes()
        .iter()
        .map(|&(through, away)| reach_of(&beyond(&block, through, away, 50.0)))
        .fold(reach_of(&block), f64::max)
}

/// The hipped block's two corners at the ends of its front's foot, on the
/// origin and at (30, 0, 0), each where the floor, the front and an end meet:
/// across from those the front's foot, running to the other end, the hip,
/// running up to the top, and the end's foot, running to the back.
fn hipped_corners() -> [Trihedral; 2] {
    let [front, left, right] = hipped_slopes().map(|(_, away)| away);
    let length = [LENGTH, HIPPED_HEIGHT * 3f64.sqrt(), HIPPED_BREADTH];
    [
        Trihedral {
            at: DVec3::ZERO,
            faces: [HIPPED_LEFT, FLOOR, HIPPED_FRONT],
            outward: [left, -DVec3::Z, front],
            called: ["the front's foot", "the left hip", "the left end's foot"],
            length,
            far: [HIPPED_RIGHT, TOP, BACK],
        },
        Trihedral {
            at: DVec3::new(LENGTH, 0.0, 0.0),
            faces: [HIPPED_RIGHT, FLOOR, HIPPED_FRONT],
            outward: [right, -DVec3::Z, front],
            called: ["the front's foot", "the right hip", "the right end's foot"],
            length,
            far: [HIPPED_LEFT, TOP, BACK],
        },
    ]
}

/// The five edges at [`hipped_corners`]: the front's foot, then the left
/// corner's hip and end's foot, then the right corner's.
fn hipped_edges([left, right]: &[Trihedral; 2]) -> [EdgeAt; 5] {
    [
        left.edge(0),
        left.edge(1),
        left.edge(2),
        right.edge(1),
        right.edge(2),
    ]
}

/// What the hipped block keeps once its five edges at [`hipped_corners`] are
/// rounded at `radius` and both corners closed by a piece of sphere, as the
/// module's documentation derives it: each round from its end section to the
/// far end of its edge, the top adding to a hip's where it cuts it slantwise,
/// then each corner's cell, the ball's sector in it given back.
fn hipped_block_rounded(radius: f64) -> f64 {
    let r = radius;
    let [root2, root3] = [2f64.sqrt(), 3f64.sqrt()];
    let in_from = r * (1.0 + root2);
    let foot = r * r * (1.0 + root2 - 3.0 * PI / 8.0);
    let hip = r * r * (1.0 / root3 - PI / 6.0);
    let feet = foot * (LENGTH - 2.0 * in_from + 2.0 * (HIPPED_BREADTH - in_from));
    let hips = 2.0
        * (hip * (HIPPED_HEIGHT * root3 - r * (3.0 + 2.0 * root2) / root3)
            + root2 * r.powi(3) * (11.0 / 18.0 - PI / (3.0 * root3)));
    let cells = 2.0 * r.powi(3) * (7.0 / 3.0 + 14.0 * root2 / 9.0);
    let sectors = 2.0 * 2.0 * (3.0 + 2.0 * root2).atan() * r.powi(3) / 3.0;
    HIPPED_VOLUME - feet - hips - cells + sectors
}

/// What the hipped block keeps once its five edges at [`hipped_corners`] are
/// chamfered at `distance` and both corners closed by a triangle, as the
/// module's documentation derives it: each flat's prism, and what each corner
/// gives back of them.
fn hipped_block_chamfered(distance: f64) -> f64 {
    let d = distance;
    let root2 = 2f64.sqrt();
    let prisms = d * d * ((LENGTH + 2.0 * HIPPED_BREADTH) / (2.0 * root2) + 1.5 * HIPPED_HEIGHT);
    HIPPED_VOLUME - prisms + d.powi(3) * (2.0 + 5.0 * root2) / 6.0
}

/// Holds the step of `cut` at 1 on the hipped block's five edges in each of
/// `orders`, as the module's documentation reads #946's first criterion: the
/// value taken whole; the body exact, closed and uncrossed, holding what the
/// arithmetic gives, its triangles enclosing that within the drawing's
/// tolerance; one face for each edge by its rank and one patch for each
/// corner after them, numbered by the lowest ranks of the corner's edges;
/// each patch the triangle through its corner's crossings or the piece of the
/// sphere about its corner's centre, meeting the faces laid on that corner's
/// edges and nothing else; the front's foot's face running from one patch to
/// the other; no flat face left with a corner at either old corner; and every
/// order laying the body the first lays.
fn hipped_block_s_corners_closed_in(cut: Cut, orders: &[Vec<usize>]) {
    let value = 1.0;
    let part = hipped_block();
    let corners = hipped_corners();
    let [left, right] = &corners;
    let edges = hipped_edges(&corners);
    let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&part)).collect();
    let start = part.faces_end();
    let laid_end = start + edges.len() + corners.len();
    let patches: Vec<usize> = (start + edges.len()..laid_end).collect();
    let centres = corners.each_ref().map(|corner| corner.centre(value));
    let (kind, promised, drawn) = match cut {
        Cut::Flat => {
            let promised = hipped_block_chamfered(value);
            ("chamfer", promised, EXACT * promised)
        }
        Cut::Round => {
            let area = (edges.iter())
                .map(|edge| edge.round(value).area())
                .sum::<f64>()
                + (corners.iter())
                    .map(|corner| corner.spread() * value * value)
                    .sum::<f64>();
            (
                "fillet",
                hipped_block_rounded(value),
                DRAWN * hipped_reach() * area,
            )
        }
    };
    let laid = laid_in_every_listed_order(
        orders,
        laid_on_the_edges(&part, &names, orders, cut, value),
        &format!(
            "the {kind} at 1 on the hipped block's front's foot and the hip and the foot at each \
             of its ends closes both corners"
        ),
    );
    let mut bodies = Vec::new();
    for (order, (asked, taken, body)) in orders.iter().zip(laid) {
        let what = format!("the {kind} on the hipped block's five edges, named {order:?}");
        taken_as_asked(asked, taken, value, &what);
        sound_drawn_within(&body, promised, drawn, &what);
        assert_eq!(
            faces_made(&part, &body),
            (start..laid_end).collect::<Vec<_>>(),
            "{what}: one face for each edge by its rank, then one patch for each corner: {:?}",
            numbers(&body)
        );
        assert_eq!(
            body.faces_end(),
            laid_end,
            "{what}: the next step numbers its own faces past the last patch"
        );
        assert!(
            numbers(&part).is_subset(&numbers(&body)),
            "{what}: a face of the block went: {:?}",
            numbers(&body)
        );
        for (edge, at) in edges.iter().enumerate() {
            let laid = start + rank_in(order, edge);
            let met = sides_met(&body, laid);
            for face in at.faces {
                assert!(
                    met.contains(&vec![face as u32]),
                    "{what}: face {laid}, laid on {} by its rank, meets face {face}, one of that \
                     edge's two: {met:?}",
                    at.called
                );
            }
        }
        let list: Vec<[usize; 2]> = order.iter().map(|&edge| edges[edge].faces).collect();
        for (index, (corner, ranks)) in closed_corners(&list, &corners).iter().enumerate() {
            let patch = patches[index];
            let which = format!(
                "{what}: patch {patch}, closing the corner at {}, its edges ranked {ranks:?}",
                corner.at
            );
            match cut {
                Cut::Flat => closed_by_its_triangle(&body, patch, corner, value, &which),
                Cut::Round => closed_by_its_piece_of_sphere(&body, patch, corner, value, &which),
            }
            assert_eq!(
                sides_met(&body, patch),
                sides(&ranks.map(|rank| start + rank)),
                "{which} meets the three faces laid on its corner's edges, and nothing else"
            );
        }
        let foot = start + rank_in(order, 0);
        let which = format!("{what}: face {foot}, laid on the front's foot");
        match cut {
            Cut::Flat => {
                corners_are(
                    &body,
                    foot,
                    &[left, right]
                        .map(|corner| [1, 2].map(|face| corner.crossing(face, value)))
                        .concat(),
                    &format!(
                        "{which}, its corners the crossings on the floor and the front at both \
                         ends"
                    ),
                );
                let [floor, front] = edges[0].outward;
                faces_toward(&body, foot, (floor + front).normalize(), &which);
            }
            Cut::Round => {
                let round = Round {
                    start: centres[0],
                    along: DVec3::X,
                    length: centres[1].x - centres[0].x,
                    radius: value,
                    facing: edges[0].outward,
                };
                rounded_between_its_corners(&body, foot, &round, &which);
            }
        }
        assert_eq!(
            sides_met(&body, foot),
            sides(&[FLOOR, HIPPED_FRONT, patches[0], patches[1]]),
            "{which} meets the floor and the front along its sides and the two patches at its \
             ends, and nothing else"
        );
        for face in numbers(&body) {
            if let Some(plane) = body.plane_of(face) {
                for corner in &corners {
                    assert!(
                        (plane.corners.iter()).all(|point| point.distance(corner.at) > PLACE),
                        "{what}: face {face} still has a corner at the old vertex {}",
                        corner.at
                    );
                }
            }
        }
        bodies.push((order.clone(), body));
    }
    one_body_face_for_face(
        &bodies,
        start,
        edges.len(),
        |(_, first), (_, body), which| match cut {
            Cut::Flat => flat_patches_alike(first, body, &patches, which),
            Cut::Round => round_patches_alike(first, body, &patches, &centres, which),
        },
        &format!("the {kind} on the hipped block's five edges"),
    );
}

/// Holds that the face numbered `patch` closes `corner` with the triangle
/// through the crossings a chamfer of `distance` makes on its three faces,
/// facing the old corner.
fn closed_by_its_triangle(
    body: &Body,
    patch: usize,
    corner: &Trihedral,
    distance: f64,
    which: &str,
) {
    let crossings = [0, 1, 2].map(|face| corner.crossing(face, distance));
    corners_are(
        body,
        patch,
        &crossings,
        &format!("{which}, its corners where the flats' edges cross on the three faces"),
    );
    let mut away = (crossings[1] - crossings[0])
        .cross(crossings[2] - crossings[0])
        .normalize();
    if away.dot(corner.at - crossings[0]) < 0.0 {
        away = -away;
    }
    faces_toward(
        body,
        patch,
        away,
        &format!("{which}, facing the old corner"),
    );
}

/// Holds that the face numbered `patch` closes `corner` with a piece of the
/// sphere of `radius` about the point that far in from its three faces:
/// curved, drawn on that sphere and on the corner's side of the three rounds'
/// end sections, touching each face where the centre's foot on it stands, and
/// met first by rays from outside toward its centre.
fn closed_by_its_piece_of_sphere(
    body: &Body,
    patch: usize,
    corner: &Trihedral,
    radius: f64,
    which: &str,
) {
    let centre = corner.centre(radius);
    let edges = [0, 1, 2].map(|edge| corner.edge(edge));
    assert!(
        !body.is_flat(patch) && body.plane_of(patch).is_none(),
        "{which} is curved, and offers no plane to draw on"
    );
    let drawn: Vec<DVec3> = body.triangles_of(patch).flatten().collect();
    assert!(!drawn.is_empty(), "{which} is drawn");
    for point in &drawn {
        assert!(
            (point.distance(centre) - radius).abs() < PLACE,
            "{which} is drawn through {point}, off the sphere of {radius} about {centre}, one \
             radius from each of the corner's three faces"
        );
        for edge in &edges {
            assert!(
                (*point - centre).dot(edge.along) <= PLACE,
                "{which} is drawn through {point}, past the end section of the round on {}",
                edge.called
            );
        }
    }
    for (face, way) in corner.faces.iter().zip(corner.outward) {
        let foot = centre + way * radius;
        assert!(
            drawn.iter().any(|point| point.distance(foot) < PLACE),
            "{which}: a corner at {foot}, where it touches face {face}"
        );
    }
    for weights in [
        [1.0, 1.0, 1.0],
        [2.0, 1.0, 1.0],
        [1.0, 2.0, 1.0],
        [1.0, 1.0, 2.0],
    ] {
        let facing = (0..3)
            .map(|face| corner.outward[face] * weights[face])
            .sum::<DVec3>()
            .normalize();
        let from = centre + facing * (radius + 10.0);
        let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
            panic!("{which}: a ray from {from} toward its centre meets nothing")
        });
        assert_eq!(
            hit.face, patch,
            "{which}: a ray from {from} toward its centre meets face {} before it",
            hit.face
        );
        assert!(
            hit.normal.distance(facing) < PLACE,
            "{which}: where a ray from {from} meets it, it faces {}, not {facing}, away from its \
             centre",
            hit.normal
        );
    }
}

/// Holds that the face numbered `face` is `round`, from one end section to the
/// other: curved, drawn on its cylinder and between its two ends, each end
/// drawn along its arc between the feet on the edge's two faces, and met first
/// from outside.
fn rounded_between_its_corners(body: &Body, face: usize, round: &Round, which: &str) {
    assert!(
        !body.is_flat(face) && body.plane_of(face).is_none(),
        "{which} is curved, and offers no plane to draw on"
    );
    let drawn: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!drawn.is_empty(), "{which} is drawn");
    for point in &drawn {
        assert!(
            round.holds(*point),
            "{which} is drawn through {point}, off its cylinder of {} about the line from {} \
             along {} for {}",
            round.radius,
            round.start,
            round.along,
            round.length
        );
    }
    for end in [0.0, round.length] {
        let axis = round.start + round.along * end;
        let section: Vec<&DVec3> = (drawn.iter())
            .filter(|point| ((**point - axis).dot(round.along)).abs() < PLACE)
            .collect();
        for way in round.facing {
            let foot = axis + way * round.radius;
            assert!(
                section.iter().any(|point| point.distance(foot) < PLACE),
                "{which}: its end section through {axis} ends at {foot}, among {section:?}"
            );
        }
        assert!(
            distinct_points(&section) > 2,
            "{which}: its end section through {axis} is drawn along the arc between its two \
             ends: {section:?}"
        );
    }
    met_from_outside(body, face, round, which);
}

#[test]
fn a_chamfer_and_a_fillet_on_the_edges_at_both_ends_of_an_edge_whose_faces_meet_at_45_degrees_close_both_corners_whatever_the_order()
 {
    let part = hipped_block();
    let corners = hipped_corners();
    let [left, right] = &corners;
    let edges = hipped_edges(&corners);
    assert!(
        (part.volume() - HIPPED_VOLUME).abs() <= EXACT * HIPPED_VOLUME,
        "the hipped block holds {}, where its level sections add up to {HIPPED_VOLUME}",
        part.volume()
    );
    for (edge, angle) in edges.iter().zip([45.0, 120.0, 45.0, 120.0, 45.0]) {
        assert!(
            (edge.angle() - f64::to_radians(angle)).abs() < PLACE,
            "{}, from {}, meets at {} degrees, where the slopes put it at {angle}",
            edge.called,
            edge.at,
            edge.angle().to_degrees()
        );
    }
    for (kind, arithmetic, booleans) in [
        (
            "chamfered",
            hipped_block_chamfered(1.0),
            triangles_by_booleans(&part, &[left, right], &edges, 1.0),
        ),
        (
            "filleted",
            hipped_block_rounded(1.0),
            spheres_by_booleans(&part, &[left, right], &edges, 1.0),
        ),
    ] {
        assert!(
            (arithmetic - booleans).abs() <= EXACT * booleans,
            "the hipped block {kind} at 1: the arithmetic gives {arithmetic}, the booleans \
             {booleans}"
        );
    }
    let orders = every_order(edges.len());
    let every_fourth: Vec<Vec<usize>> = orders.iter().step_by(4).cloned().collect();

    hipped_block_s_corners_closed_in(Cut::Flat, &orders);
    hipped_block_s_corners_closed_in(Cut::Round, &every_fourth);
}

/// The block 30 by 10 by 20 with the corner at its top's front right cut off
/// by the plane through the points 0.5 from it along its three edges: a facet,
/// numbered six, an equilateral triangle of side √0.5.
fn small_facet() -> Body {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    block
        .difference(&beyond(
            &block,
            DVec3::new(LENGTH - 0.5, 0.0, HEIGHT),
            DVec3::new(1.0, -1.0, 1.0),
            50.0,
        ))
        .expect("the facet is cut")
}

/// The small facet's corner on the top's front edge, at (29.5, 0, 20): its
/// faces the facet, the front and the top; across from them the top's front
/// edge, running to the left wall, the facet's edge on the top and its edge
/// on the front, both running to the right wall.
fn small_facet_trihedral() -> Trihedral {
    Trihedral {
        at: DVec3::new(LENGTH - 0.5, 0.0, HEIGHT),
        faces: [6, FRONT, TOP],
        outward: [DVec3::new(1.0, -1.0, 1.0).normalize(), -DVec3::Y, DVec3::Z],
        called: [
            "the top's front edge",
            "the facet's edge on the top",
            "the facet's edge on the front",
        ],
        length: [LENGTH - 0.5, 0.5f64.sqrt(), 0.5f64.sqrt()],
        far: [LEFT, RIGHT, RIGHT],
    }
}

#[test]
fn a_fillet_where_one_round_takes_the_next_edge_whole_lays_both_rounds_in_either_order_and_a_chamfer_declines_in_both()
 {
    let cap = 0.5;
    let part = small_facet();
    let corner = small_facet_trihedral();
    let [top_front, facet_front] = [corner.edge(0), corner.edge(2)];
    let promised = [&top_front, &facet_front]
        .iter()
        .fold(part.clone(), |cut, edge| {
            cut.difference(&edge.tool(&cut, Cut::Round, cap))
                .expect("a round's prism cuts the part")
        })
        .volume();
    let names = [top_front.name(&part), facet_front.name(&part)];
    let start = part.faces_end();

    for asked in [cap, 2.0] {
        let orders = [
            ("the top's front edge first", [0, 1]),
            ("the facet's edge first", [1, 0]),
        ];
        let laid: Vec<Body> = orders
            .iter()
            .map(|(what, order)| {
                let list = order.map(|edge| names[edge].clone());
                let filleted = part.fillet_edges(&list, asked).unwrap_or_else(|declined| {
                    panic!("the fillet asked {asked}, {what}: declined, {declined:?}")
                });
                assert!(
                    (filleted.taken - cap).abs() < PLACE,
                    "the fillet asked {asked}, {what}: took {}, where the facet's corner allows \
                     {cap}",
                    filleted.taken
                );
                filleted.body
            })
            .collect();
        for ((what, order), body) in orders.iter().zip(&laid) {
            let what = format!("the fillet asked {asked}, {what}");
            let area = PI / 2.0 * cap * (LENGTH + 1.0);
            sound_drawn_within(body, promised, DRAWN * reach_of(&part) * area, &what);
            assert_eq!(
                faces_made(&part, body),
                vec![start, start + 1],
                "{what}: one round for each edge, by its rank in the list"
            );
            for (rank, &edge) in order.iter().enumerate() {
                assert!(
                    !body.is_flat(start + rank) && body.plane_of(start + rank).is_none(),
                    "{what}: face {} is the round of {}, curved",
                    start + rank,
                    corner.called[[0, 2][edge]]
                );
            }
        }
        let swapped = |number: usize| match number {
            number if number == start => start + 1,
            number if number == start + 1 => start,
            number => number,
        };
        let [first, second] = [&laid[0], &laid[1]];
        for face in numbers(first) {
            let what = format!(
                "the fillet asked {asked}: face {face} named the top's front edge first, against \
                 the facet's edge first"
            );
            assert_eq!(
                sides_renumbered(first, face, swapped),
                sides_met(second, swapped(face)),
                "{what}: it meets the same faces"
            );
            if let Some(plane) = first.plane_of(face) {
                corners_are(second, swapped(face), &plane.corners, &what);
            }
        }

        for (what, order) in orders {
            let list = order.map(|edge| names[edge].clone());

            let chamfered = part.chamfer_edges(&list, asked).err();

            assert_eq!(
                chamfered,
                Some(Declined::Unsupported),
                "the chamfer asked {asked}, {what}: the top's front edge's flat at the cap takes \
                 the facet whole, as it does alone"
            );
        }
    }
}

/// The block 30 by 10 by 20 with the corner at its top's front right cut off
/// steeply: the facet, numbered six, through the points 2 along the top's two
/// edges and 6 down the upright. At its corner on the upright, (30, 0, 14),
/// the facet's two edges rise at wide angles to the upright, 161.6 degrees on
/// the front and on the right.
fn steep_facet() -> Body {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    block
        .difference(&beyond(
            &block,
            DVec3::new(LENGTH - 2.0, 0.0, HEIGHT),
            DVec3::new(0.5, -0.5, 1.0 / 6.0),
            50.0,
        ))
        .expect("the facet is cut")
}

/// The steep facet's corner on the upright: its faces the right wall, the
/// front and the facet; across from them the facet's edge on the front, its
/// edge on the right, both rising to the top, and the upright, running down
/// to the floor.
fn steep_trihedral() -> Trihedral {
    Trihedral {
        at: DVec3::new(LENGTH, 0.0, HEIGHT - 6.0),
        faces: [RIGHT, FRONT, 6],
        outward: [
            DVec3::X,
            -DVec3::Y,
            DVec3::new(0.5, -0.5, 1.0 / 6.0).normalize(),
        ],
        called: [
            "the facet's edge on the front",
            "the facet's edge on the right",
            "the upright",
        ],
        length: [40f64.sqrt(), 40f64.sqrt(), HEIGHT - 6.0],
        far: [TOP, TOP, FLOOR],
    }
}

#[test]
fn a_fillet_on_the_three_edges_of_a_corner_whose_sphere_would_stand_behind_it_along_an_edge_is_declined_as_unsupported()
 {
    let value = 0.5;
    let part = steep_facet();
    let corner = steep_trihedral();
    let behind = (corner.centre(value) - corner.at).dot(corner.edge(2).along);
    let gradient = (0.25 + 0.25 + 1.0 / 36.0f64).sqrt();
    assert!(
        (behind - 6.0 * value * (gradient - 1.0)).abs() < PLACE && behind < 0.0,
        "the sphere's centre, one radius in from the three faces, stands {behind} along the \
         upright from the corner: behind it, where the arithmetic puts it"
    );
    let promised = triangle_by_booleans(&part, &corner, value);

    for order in ORDERS_OF_THREE {
        let names = order.map(|edge| corner.edge(edge).name(&part));

        let filleted = part.fillet_edges(&names, value).err();

        assert_eq!(
            filleted,
            Some(Declined::Unsupported),
            "the fillet on {}, {} and {}, the piece of sphere standing behind the corner along \
             the upright: a broken step and its reason, and no body",
            corner.called[order[0]],
            corner.called[order[1]],
            corner.called[order[2]]
        );
        closes_with_a_triangle(&part, &corner, order, value, promised);
    }
}

#[cfg(feature = "campaigns")]
mod campaign {
    use super::*;

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_fillet_on_the_hips_and_the_base_edges_of_a_pyramid_closes_its_apex_and_its_four_base_corners_in_one_body_in_each_of_48_orders()
     {
        pyramid_s_eight_edges_rounded_in(&spread_orders_of_eight());
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_fillet_on_the_edges_at_both_ends_of_an_edge_whose_faces_meet_at_45_degrees_closes_both_corners_in_each_of_120_orders()
     {
        hipped_block_s_corners_closed_in(Cut::Round, &every_order(5));
    }
}
