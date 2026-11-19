//! A chamfer or a fillet on one straight edge alone, where an end of it is a
//! vertex of four edges. The edge's prism, raised whole along its line, is
//! cut by the faces round each such end as at a corner of three: the fill
//! ends on every face there that its prism crosses, along a broken line or a
//! curve across them. Held on `pyramid()`, #699's 10-cube cut down to a
//! square pyramid 5 high on a box 5 high, whose apex and whose four hips'
//! feet are each a vertex of four edges, and on `pyramid_on_its_base`, the
//! square pyramid 10 by 10 standing on its own base, whose apex alone is.
//! And some of the edges ending at such a vertex, not all, cut in one step,
//! the others left sharp: two at a hip's foot on `pyramid()`, mitred, and
//! three of the four hips at the apex of the pyramid on its own base, each
//! flat cut as the booleans cut it. An operation of the kernel on [`Body`];
//! the step of the history that lays it on one clicked edge is #927's.
//!
//! Closes #943.
//! - Each hip of `pyramid()` alone, filleted and chamfered at 1: built, exact,
//!   closed, the volumes above —
//!   `a_fillet_on_a_lone_hip_of_a_pyramid_on_a_box_ends_on_the_box_s_two_walls_and_on_the_two_other_slopes_at_the_apex`,
//!   `a_chamfer_on_a_lone_hip_of_a_pyramid_on_a_box_lays_a_hexagon_ending_on_the_box_s_two_walls_and_on_the_two_other_slopes_at_the_apex`.
//!   Read as: before the step, the hip's prism cut from the part by the
//!   booleans keeps the volume below; then each of the four hips, alone in
//!   its list, filleted at 1 and chamfered at 1, the value taken whole; the
//!   body exact, closed, uncrossed, holding 666.2069507719303 filleted and
//!   662.9932132211286 chamfered, the arithmetic below, the issue's
//!   662.9932132211239 a rounding of its Python away; its triangles enclosing
//!   that within the drawing's tolerance at the body's own reach, at which it
//!   is drawn since #824 whatever its slabs reached, over the round's area;
//!   one face laid, numbered where the
//!   steps' numbers ended, the step ending past it, every face of the pyramid
//!   kept. The round curved, drawn on its cylinder of radius 1 about the line
//!   one radius in from its two slopes, met first from outside; its end at
//!   the foot drawn on the box's two walls along the curves they cut from its
//!   cylinder, each from the eave down to the upright, and its end at the
//!   apex on the two other slopes, each from the next hip to the opposite
//!   one; meeting its two slopes, the two walls and the two other slopes, by
//!   one edge each. The flat facing halfway between its two slopes, a
//!   hexagon whose corners are where its plane crosses the upright, the two
//!   eaves and the three other hips, meeting the same six faces. No face left
//!   with a corner at the apex or at the hip's foot. The four hips held as
//!   one turned a quarter about the pyramid's axis, each turn carrying the
//!   faces with it
//! - The box's upright under a hip, and an eave, alone: built —
//!   `a_fillet_on_a_lone_upright_of_the_box_under_a_pyramid_ends_on_the_two_slopes_over_it`,
//!   `a_chamfer_on_a_lone_upright_of_the_box_under_a_pyramid_lays_a_pentagon_ending_on_the_two_slopes_over_it`,
//!   `a_fillet_on_a_lone_eave_of_a_pyramid_on_a_box_ends_on_the_wall_and_the_slope_at_each_hip_s_foot`,
//!   `a_chamfer_on_a_lone_eave_of_a_pyramid_on_a_box_lays_a_hexagon_ending_on_the_wall_and_the_slope_at_each_hip_s_foot`.
//!   Read as: "built" held as the hips are, at a volume, though the issue
//!   gives none. Before the step, the booleans keeping it; then each of the
//!   four uprights and each of the four eaves, alone, filleted and chamfered
//!   at 1, the value taken whole; the body exact, closed, uncrossed, holding
//!   665.574317792927 and 664.0833333333333 on an upright, 666.4529467923597
//!   and 663.2001683544628 on an eave, the arithmetic below; one face laid,
//!   numbered where the steps' numbers ended. An upright's round drawn on its
//!   quarter cylinder, ending on the floor along a quarter circle and at its
//!   top on the two slopes over it, each from an eave up to the hip; its flat
//!   a pentagon, from the floor up its two walls to the eaves and on to the
//!   hip; each meeting the floor, its two walls and those two slopes. An
//!   eave's round drawn on its cylinder, ending at each hip's foot on the
//!   wall below the eave, from the upright to the eave across, and on the
//!   slope above it, from there to the hip; its flat a hexagon with those
//!   ends; each meeting its wall, its slope, and the wall and the slope at
//!   each end. No face left with a corner where the edge ended
//! - From the comment of 2026-11-13: on a square pyramid standing on its own
//!   base (10 × 10, apex (5, 5, 5)), one hip alone, filleted and chamfered at
//!   1, is built. The volumes are 166.21230204348723 and 163.1463063300533 —
//!   `a_fillet_on_a_lone_hip_of_a_pyramid_on_its_own_base_ends_on_the_base_and_on_the_two_other_slopes_at_the_apex`,
//!   `a_chamfer_on_a_lone_hip_of_a_pyramid_on_its_own_base_lays_a_pentagon_ending_on_the_base_and_on_the_two_other_slopes_at_the_apex`.
//!   Read as the first criterion, on `pyramid_on_its_base`: the booleans
//!   before the step; each of the four hips alone, the value taken whole; the
//!   body exact, closed, uncrossed, holding 166.21230204348723 filleted and
//!   163.14630633005257 chamfered, the comment's 163.1463063300533 a rounding
//!   away; the round ending on the base along the curve it cuts from its
//!   cylinder, from one edge of the base to the other, as at a corner of
//!   three edges, and at the apex on the two other slopes; the flat a
//!   pentagon, its foot one straight side on the base, its top where its
//!   plane crosses the three other hips
//! - **Visible to the user**: the PR carries a To try it section and waits
//!   for Tom's word — no test: it is the pull request's body and a human's
//!   word, which no assertion of the workspace reaches
//! - Reading of the issue (2026-11-19), recorded on it in the comment
//!   "Correction to "What it does"": its "What it does" calls a hip's flat
//!   on `pyramid()` a pentagon. The flat the issue's own volume describes,
//!   which the booleans cut too, is a hexagon: at the foot its plane crosses
//!   the upright between the two walls, a side on each, and at the apex the
//!   opposite hip between the two other slopes, a side on each. It is a
//!   pentagon on the pyramid standing on its own base, its foot one side on
//!   the base, and on the upright, held so above —
//!   `a_chamfer_on_a_lone_hip_of_a_pyramid_on_a_box_lays_a_hexagon_ending_on_the_box_s_two_walls_and_on_the_two_other_slopes_at_the_apex`,
//!   `a_chamfer_on_a_lone_hip_of_a_pyramid_on_its_own_base_lays_a_pentagon_ending_on_the_base_and_on_the_two_other_slopes_at_the_apex`,
//!   `a_chamfer_on_a_lone_upright_of_the_box_under_a_pyramid_lays_a_pentagon_ending_on_the_two_slopes_over_it`
//! - Decided on the way, after the review (2026-11-19): a lone edge is
//!   carried past a vertex of four edges only where every edge ending there
//!   meets outwards. On the pyramid on its own base standing on a plate 20 by
//!   20 by 2, flush with the plate's front and left walls, the front left
//!   hip's foot is such a vertex, as on `pyramid()`'s box, and the hip alone
//!   is built; the front right hip's foot is a vertex of four where the right
//!   slope meets the plate's top inwards, and the hip ending there alone is
//!   declined `Unsupported`, and so is the front eave, which ends there too,
//!   with their mirrors across the diagonal, the back left hip and the left
//!   eave, until #1025 —
//!   `a_lone_hip_or_eave_of_a_pyramid_on_a_plate_is_declined_as_unsupported_where_its_vertex_of_four_edges_has_a_concave_one_and_cut_where_it_has_none`.
//!   Read as: the front left hip alone, chamfered and filleted at 1, as on
//!   the box: the booleans before the step, the value taken whole, the body
//!   exact, closed, uncrossed, holding the plate's 800 with the pyramid's
//!   figures, 962.9932132211286 and 966.2069507719303, its flat or round
//!   laid as on the box, ending on the plate's front and left walls; the back
//!   right hip alone, its foot a corner of three with the two feet beside it
//!   meeting inwards, stopped flush on the plate's top as #801 stops an edge
//!   running into a face, built at 963.14630633005257 and
//!   966.21230204348723, one face laid; then the four others declined
//!   `Unsupported` for the chamfer and the fillet. A pin of the guard, green
//!   on the branch, as the reviewer measured it
//! - Decided on the way, after the review (2026-11-19): two edges of one step
//!   ending at one vertex of four edges are not carried past it as one alone
//!   is. On `pyramid()`, at each of the four hips' feet, every two of the
//!   four edges ending there — the hip, the two eaves and the upright — named
//!   in one step, in either order, are declined `Unsupported` for the
//!   chamfer and the fillet at 1: some of a vertex's edges named and not all,
//!   #942's case. Pinned as found, green on the branch, by
//!   `a_chamfer_and_a_fillet_on_two_edges_ending_at_one_hip_s_foot_on_a_box_are_declined_as_unsupported_in_either_order`,
//!   until #942 turned that decline round: every such pair is built since,
//!   mitred, the two edges left out sharp, and the pin went, the two tests of
//!   #942's first criterion below in its place —
//!   `a_chamfer_on_two_edges_ending_at_one_hip_s_foot_on_a_box_mitres_their_flats_and_leaves_the_two_others_sharp_in_either_order`,
//!   `a_fillet_on_two_edges_ending_at_one_hip_s_foot_on_a_box_mitres_their_rounds_and_leaves_the_two_others_sharp_in_either_order`
//! - Decided on the way, after the review (2026-11-19): two edges in one
//!   step, each ending alone at its vertices of four, are each laid as alone,
//!   which `main` declined. On `pyramid()`, the front and back eaves, and the
//!   right and left ones, named in one step in either order, chamfered and
//!   filleted at 1 —
//!   `a_chamfer_and_a_fillet_on_two_eaves_of_a_pyramid_on_a_box_sharing_no_vertex_lay_each_as_alone_in_one_step_whatever_the_order`.
//!   Read as: the value taken whole; the body exact, closed, uncrossed,
//!   holding the pyramid less two lone cuts, 659.7336700422588 chamfered and
//!   666.2392269180529 filleted, its triangles enclosing that within the
//!   drawing's tolerance over both rounds; one face for each eave, numbered by
//!   its rank in the list, each holding what that eave alone lays; no face
//!   left with a corner where either eave ended
//! - Decided on the way, after cargo-mutants (2026-11-19): a corner the face
//!   of a lone cut takes on an edge ending at the vertex it is carried past
//!   lies on that edge, between its two ends, or the step is declined. On
//!   `pyramid()`'s box cut down to a slab 0.1 high, `pyramid_on_a_slab`, a
//!   hip cut alone at 1 would cross the line of the slab's upright below the
//!   slab's floor, the flat's plane 0.612 under the foot and the round 0.189,
//!   so that its face would end across the floor, its corners there on the
//!   floor's front and left edges, which do not end at the foot: declined
//!   `Unsupported`, chamfer and fillet, on each of the four hips —
//!   `a_chamfer_and_a_fillet_on_a_lone_hip_whose_face_would_end_past_the_foot_of_the_slab_s_upright_are_declined_as_unsupported`.
//!   Read as: before the step, that crossing below the floor, read from the
//!   fixture, and the booleans cutting the hip's prism from the slab; then
//!   the decline. Found by the mutants on 6dca11c2, where `lies_on` returning
//!   `true`, and its `<=` turned `>`, which with three edges crossed at a foot
//!   accepts any corner, both lived. Each, applied by hand in a scratch copy,
//!   builds all four hips here exact, closed and uncrossed, the body the
//!   booleans cut, its face meeting what theirs meets, corner for corner on
//!   the flat: the decline keeps out no wrong body but one the kernel has not
//!   been shown to build, its face ending on a face that does not meet the
//!   vertex; building that case is #1029's. Both mutants lay every pair of
//!   the test above at a hip's foot as
//!   well, in either order, each the body the booleans cut from both prisms,
//!   so that test, written after the campaign, killed them too, until #942
//!   turned it into the bodies those mutants lay
//!
//! Closes #942, its Done when decided in the comment "Picked up 2026-11-19".
//! - **Two edges at a convex vertex of four or more edges, the others left
//!   sharp: built.** On #699's `pyramid()` (a 10-cube under a square
//!   pyramid), take any two of the four edges ending at a hip's foot: the
//!   hip, the two eaves, the upright. Chamfer and fillet at 1, in either
//!   order. Each pair is mitred as #699 mitres two edges at a corner, and the
//!   body is the booleans' cut of both prisms. #943's test writer found that
//!   body built right by a mutant of #943's guard, for example
//!   659.782983090225 for the hip and the front eave chamfered. #943 pins
//!   every such pair declined today
//!   (`a_chamfer_and_a_fillet_on_two_edges_ending_at_one_hip_s_foot_on_a_box_are_declined_as_unsupported_in_either_order`),
//!   and that pin turns into the build test —
//!   `a_chamfer_on_two_edges_ending_at_one_hip_s_foot_on_a_box_mitres_their_flats_and_leaves_the_two_others_sharp_in_either_order`,
//!   `a_fillet_on_two_edges_ending_at_one_hip_s_foot_on_a_box_mitres_their_rounds_and_leaves_the_two_others_sharp_in_either_order`.
//!   Read as: at each of the four hips' feet, each two of the four edges
//!   ending there named in one step, in either order, forty-eight lists,
//!   chamfered and filleted at 1. Before the step, the booleans cutting the
//!   two prisms from the part, one after the other, keep the volume below.
//!   Then the value taken whole; the body exact, closed, uncrossed, holding
//!   that volume, its triangles enclosing it within the drawing's tolerance
//!   over both rounds; two faces laid, numbered by their edges' ranks from
//!   where the steps' numbers ended, and none at the foot; the body the
//!   booleans' cut face for face, each face meeting what it meets there and
//!   each flat face keeping its corners; mitred as #699's tests read a mitre,
//!   where the two meet: the two faces meeting along one edge; two flats each
//!   facing halfway between its edge's two faces and sharing the mitre's two
//!   ends as corners, and no other; two rounds each drawn on its own cylinder
//!   and met first from outside, every point both are drawn through on the
//!   plane through the crossing of their axes halfway between the ways the
//!   two edges leave the foot, where the two cylinders cut each other; each
//!   of the two edges left out still one sharp edge; no face left with a
//!   corner at the foot. Each reading is held on the booleans' cut first.
//!   The pin went, these two tests in its place. "As #699 mitres" is read
//!   where the two meet and no farther: the hip and the upright filleted, the
//!   upright's round runs on past that plane, up to 0.043 from it, beside the
//!   eaves, where the hip's round does not reach to trim it, as the booleans
//!   cut it; at #699's square corner each round keeps to its own side
//!   throughout
//! - **A chamfer on some, not all, of the edges at a regular apex: built.**
//!   Three hips of a square pyramid's four, at 1, in every order. The body is
//!   the part cut by each named edge's flat, as the booleans give it, the
//!   unnamed hip left sharp. Exact, closed, at that volume —
//!   `a_chamfer_on_three_of_the_four_hips_at_a_pyramid_s_apex_cuts_the_part_by_each_hip_s_flat_and_leaves_the_fourth_hip_sharp_whatever_the_order`.
//!   Read as: on `pyramid_on_its_base`, each three of the four hips, in each
//!   of the six orders a list can name them in, chamfered at 1. Before the
//!   step, the booleans cutting the three prisms from the pyramid keep
//!   1865/12 + √6/2, the arithmetic below. Then the value taken whole; the
//!   body exact, closed, uncrossed, holding that; three flats laid, numbered
//!   by their hips' ranks, and no face at the apex; the body the booleans'
//!   cut face for face, the three flats meeting at one point of the axis √6/4
//!   under the apex, two by two along a line; each flat facing halfway
//!   between its two slopes; the hip left out still one sharp edge, from its
//!   foot up to where the flats of the two hips beside it both cross it,
//!   3√2/4 from the apex, each of its two slopes with a corner at both ends;
//!   no face left with a corner at the apex. Each reading is held on the
//!   booleans' cut first. #804's pin of three hips,
//!   `a_fillet_and_a_chamfer_on_three_of_the_four_edges_at_a_pyramid_s_apex_are_declined_as_unsupported`,
//!   keeps its fillet, renamed for it, and gives its chamfer to this test
//! - **What must not break:** #804's apex closed when every edge is named,
//!   #949's regular apex, #943's lone edges and their guards, #699's corners
//!   — no test: each is held by its own test, left as it was. #804's apex,
//!   which #949 closed, by the four hips' tests in
//!   `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs`;
//!   #943's lone edges by the tests above, its guards by
//!   `a_lone_hip_or_eave_of_a_pyramid_on_a_plate_is_declined_as_unsupported_where_its_vertex_of_four_edges_has_a_concave_one_and_cut_where_it_has_none`
//!   and
//!   `a_chamfer_and_a_fillet_on_a_lone_hip_whose_face_would_end_past_the_foot_of_the_slab_s_upright_are_declined_as_unsupported`;
//!   #699's corners by their tests in the file of #804's
//! - Out of scope, kept declined, split off into #1031 by the same comment:
//!   a fillet on some of an apex's edges; an apex whose faces share no
//!   sphere, or whose chamfers' crossings are not coplanar. "The pinned
//!   declines of #804 for those stay as they are until #1031 decides" —
//!   no test: the pins stay where they were, in
//!   `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs`, their
//!   messages naming #1031 now —
//!   `a_fillet_on_three_of_the_four_edges_at_a_pyramid_s_apex_is_declined_as_unsupported`,
//!   `a_fillet_and_a_chamfer_on_the_four_hips_of_a_pyramid_10_by_6_whose_faces_share_no_sphere_are_declined_as_unsupported_whatever_the_order`,
//!   `a_chamfer_on_the_four_hips_of_a_pyramid_10_by_9_8_whose_crossings_lie_a_little_off_one_plane_is_declined_as_unsupported_whatever_the_order`,
//!   `a_chamfer_on_the_four_hips_of_a_pyramid_whose_crossings_lie_a_hair_off_one_plane_past_the_kernel_s_tolerance_is_declined_as_unsupported_whatever_the_order`
//!
//! A lone cut on an edge of a convex body takes the edge's prism where it
//! lies inside the body: each line of the prism along the edge keeps the
//! stretch between the face it enters by and the face it leaves by. At a
//! vertex of four edges two faces bound that stretch's end, and the end is
//! kinked where both meet the line at one point. Every volume below is held a
//! second way, apart from the kernel, by Gauss–Legendre quadrature of those
//! stretches read against every face of the body, and a third by the
//! kernel's booleans, each lone cut's prism raised whole and cut from the
//! part; the three agree to the last digits.
//!
//! A hip of `pyramid()` runs `5√3` from its foot (0, 0, 5) up to the apex
//! (5, 5, 10), between two slopes meeting at 120 degrees inside the matter.
//! Measure its section by `α`, in from the hip along the line halfway
//! between the slopes, and `β`, across toward the front slope. A line of the
//! prism enters by the front wall or the left wall `√3(|β|/√2 − α/√6)` past
//! the foot along the hip, the two together along the upright, and leaves by
//! the right or the back slope `(√3/2)(10 + α/√6 − |β|/√2)` past it, the two
//! together along the opposite hip: it keeps
//! `5√3 + (3√3/2)(α/√6 − |β|/√2)`. Over a section of area `S` and moments
//! `M_α` and `M_|β|`, the cut takes
//! `5√3·S + 3M_α/(2√2) − 3√3·M_|β|/(2√2)`. The flat's section is the
//! triangle between the hip and its two set-backs, `S = √3d²/4`,
//! `M_α = √3d³/12`, `M_|β| = d³/8`: it takes `15d²/4 − √6d³/32`, and the
//! pyramid on its box, `500 + 500/3`, keeps 662.9932132211286 at 1. The
//! round's is the section between the slopes and the arc, `S = r²(1/√3 −
//! π/6)`, `M_α = r³(11/18 − π/(3√3))` as #804 finds it, and `M_|β|`, twice
//! the triangle between the hip, a tangent point and the axis less half the
//! sector, `r³(1/(6√3) + 1/√3 − 2/3)`: the body keeps 666.2069507719303 at 1.
//! The flat's plane crosses the upright `d√6/4` under the foot, the eaves
//! `d√6/2` from it, the two next hips `d√6/4` from the apex and the opposite
//! one `d√6/8`, each measured along every axis.
//!
//! On the pyramid standing on its own base, the hip's foot at the origin and
//! its apex at (5, 5, 5), a line enters by the base, `√2·α` past the foot,
//! and leaves as above: it keeps `5√3 − (3α + √3|β|)/(2√2)`. The flat takes
//! `15d²/4 − 3√6d³/32`, the round `5√3·S − (3M_α + √3·M_|β|)/(2√2)`: of
//! `500/3` the body keeps 163.14630633005257 chamfered and 166.21230204348723
//! filleted at 1.
//!
//! An upright of the box, `a` in from the front wall and `b` from the left:
//! a line runs from the floor up to the lower of the two slopes over it,
//! `5 + min(a, b)`. The flat takes `5d²/2 + d³/12`, 664.0833333333333 kept at
//! 1; the round, over `S = r²(1 − π/4)`, where the integral of `min(a, b)` is
//! `r³((√2 + 1)/3 − π/4)`, takes `5r²(1 − π/4) + r³((√2 + 1)/3 − π/4)`,
//! 665.574317792927 kept.
//!
//! An eave between the front wall and the front slope, meeting at 135
//! degrees, `z′` measured up from the eave: a line runs from the left wall,
//! or above the eave from the left slope `z′` along, to the right wall or the
//! right slope as far short: it keeps `10 − 2·max(z′, 0)`. The flat's
//! section, `d²/(2√2)`, holds above the eave a triangle whose `z′` sums to
//! `(√2 − 1)d³/12`: it takes `5d²/√2 − (√2 − 1)d³/6`, 663.2001683544628
//! kept at 1. The round's, `S = r²(√2 − 1 − π/8)`, holds above the eave the
//! part between the slope and the arc, from the eave to the slope's tangent
//! point, whose `z′` sums to `r³((√2 − 1)/12 − A + (√2 − 1)B)`, with
//! `A = ((2√2 − 2)^(3/2) − 2^(−3/2))/3` and
//! `B = (1/2 + π/4)/2 − ((√2 − 1)√(2√2 − 2) + asin(√2 − 1))/2`: it takes
//! `10S` less twice that, 666.4529467923597 kept. The arc crosses the eave's
//! level `r(1 − √(2√2 − 2))` out from the wall.
//!
//! A flat takes from a convex part the half-space past its plane, so the
//! body two or three flats leave is the part cut by their planes: a convex
//! polyhedron whose corners have their coordinates in ℚ(√2, √3). Clipped
//! exactly in that field, apart from the kernel, its volume at 1 is, at a
//! hip's foot on `pyramid()`, `4639/7 − 61√2/28 − √3/14 + 25√6/224` with the
//! hip and an eave, 659.782983090225, the issue's figure; `2641/4 + 5√6/32`
//! with the hip and the upright, 660.6327327723099; `2000/3 − 29√2/6` with
//! the two eaves, 659.8313011151967; `1996/3 − 19√2/6` with an eave and the
//! upright, 660.8549903858186. On the pyramid standing on its own base, three
//! hips leave `1865/12 + √6/2`, 156.64141153805826: of the
//! `3(15/4 − 3√6/32)` their three lone cuts take, `7√6/32` is where two or
//! three of them take the same matter.
//!
//! Two rounds at a hip's foot each take their lone cut, above, and give back
//! once what both take: at 1, 0.00707233083665 for the hip and an eave,
//! 0.01968716375627 for the hip and the upright, 0.00116391439218 for the two
//! eaves, and 0.01425718365517 for an eave and the upright, as the booleans
//! measure the cut of both prisms. A second way, apart from the kernel:
//! Gauss–Legendre quadrature, over the section of one round's prism, of the
//! stretch of each line along its edge that lies in the part and in the
//! other's prism, whose section is a kite less a disc; it gives the same
//! within 6e-9. The axes of the four rounds at a foot all cross at one
//! point, one radius in from each of the four faces round it: (1, 1, 6 − √2)
//! at the front left foot.

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it: each is
/// placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// The numbers a raised block's faces answer to: its floor nought, and its
/// sides from the front round to the left, two to five.
const FLOOR: usize = 0;
const FRONT: usize = 2;
const RIGHT: usize = 3;
const BACK: usize = 4;
const LEFT: usize = 5;

/// The numbers a pyramid's slopes answer to: each cut by a slab of six faces,
/// the slab's floor first, the front, the back, the left and the right in
/// turn.
const FRONT_SLOPE: usize = 6;
const BACK_SLOPE: usize = 12;
const LEFT_SLOPE: usize = 18;
const RIGHT_SLOPE: usize = 24;

/// The slopes in the order the slabs cut them.
const SLOPES: [usize; 4] = [FRONT_SLOPE, BACK_SLOPE, LEFT_SLOPE, RIGHT_SLOPE];

/// The side of both pyramids' square base, seen from above.
const SIDE: f64 = 10.0;

/// The apex of `pyramid()`, over the middle of the 10-cube's top.
const ON_A_BOX: DVec3 = DVec3::new(5.0, 5.0, 10.0);

/// The apex of the pyramid standing on its own base.
const ON_ITS_BASE: DVec3 = DVec3::new(5.0, 5.0, 5.0);

/// What `pyramid()` holds: the box 10 by 10 by 5 and the pyramid 5 high on it.
const ON_A_BOX_VOLUME: f64 = 500.0 + 500.0 / 3.0;

/// What the pyramid on its own base holds: a third of its base times its
/// height.
const ON_ITS_BASE_VOLUME: f64 = 500.0 / 3.0;

/// The value every cut here is asked, and takes whole.
const VALUE: f64 = 1.0;

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

/// The way slope `face` faces, out of the matter: each leans 45 degrees.
fn slope_facing(face: usize) -> DVec3 {
    match face {
        FRONT_SLOPE => DVec3::new(0.0, -1.0, 1.0),
        BACK_SLOPE => DVec3::new(0.0, 1.0, 1.0),
        LEFT_SLOPE => DVec3::new(-1.0, 0.0, 1.0),
        RIGHT_SLOPE => DVec3::new(1.0, 0.0, 1.0),
        _ => panic!("face {face} is no slope"),
    }
    .normalize()
}

/// `block` cut by the four planes through `apex` that lean 45 degrees, one
/// over each side of the square base, numbered as [`SLOPES`] lists them.
fn cut_down_to(block: Body, apex: DVec3) -> Body {
    SLOPES.into_iter().fold(block, |part, face| {
        part.difference(&beyond(&part, apex, slope_facing(face), 50.0))
            .expect("a plane through the apex cuts the block")
    })
}

/// The 10-cube `pyramid()` is cut from.
fn cube() -> Body {
    raised(Contour::rectangle(DVec2::ZERO, DVec2::splat(SIDE)), SIDE)
}

/// The block the pyramid on its own base is cut from: one wider than the
/// base all round and one higher than its apex, so that every slope takes
/// the block's walls and top away whole and only its floor stays, the base.
fn block_under() -> Body {
    raised(
        Contour::rectangle(DVec2::splat(-1.0), DVec2::splat(SIDE + 1.0)),
        ON_ITS_BASE.z + 1.0,
    )
}

/// #699's pyramid on a box: the 10-cube cut by four planes through the middle
/// of its top, down to its walls' halfway, a square pyramid 5 high on a box 5
/// high. Its apex and each hip's foot are vertices of four edges: at a foot,
/// the hip, the two eaves and the box's upright.
fn pyramid() -> Body {
    cut_down_to(cube(), ON_A_BOX)
}

/// The square pyramid standing on its own base, 10 by 10, its apex at
/// (5, 5, 5): its apex alone is a vertex of four edges, each corner of its
/// base a corner of three.
fn pyramid_on_its_base() -> Body {
    cut_down_to(block_under(), ON_ITS_BASE)
}

/// The largest coordinate a body reaches, on either side of the origin: the
/// reach it is drawn at, its own since #824, whatever the slabs that cut it
/// reached.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
}

/// How far the triangles of `body` may enclose apart from its matter, its
/// rounds `rounds`: the drawing's tolerance at the body's own reach, over
/// each round's area, carried a radius past both ends of its edge.
fn drawn_within(body: &Body, rounds: &[Round]) -> f64 {
    let area: f64 = rounds
        .iter()
        .map(|round| {
            round
                .part(-round.radius, round.length + round.radius)
                .area()
        })
        .sum();
    DRAWN * reach_of(body) * area
}

/// A point turned a quarter about the pyramids' axis, anticlockwise seen
/// from above: the front left corner of the base onto the front right.
fn turned(point: DVec3) -> DVec3 {
    DVec3::new(SIDE - point.y, point.x, point.z)
}

/// A way turned a quarter, as [`turned`] turns a point.
fn turned_way(way: DVec3) -> DVec3 {
    DVec3::new(-way.y, way.x, way.z)
}

/// The face a quarter turn carries face `face` onto: the front onto the
/// right, the right onto the back, the back onto the left, walls and slopes
/// alike; the floor stays.
fn turned_face(face: usize) -> usize {
    match face {
        FRONT => RIGHT,
        RIGHT => BACK,
        BACK => LEFT,
        LEFT => FRONT,
        FRONT_SLOPE => RIGHT_SLOPE,
        RIGHT_SLOPE => BACK_SLOPE,
        BACK_SLOPE => LEFT_SLOPE,
        LEFT_SLOPE => FRONT_SLOPE,
        other => other,
    }
}

/// A plane a lone cut's fill ends on: the number of the face lying on it, a
/// point of it, and the way it faces out of the matter.
#[derive(Clone, Copy, Debug)]
struct Bound {
    face: usize,
    through: DVec3,
    outward: DVec3,
}

impl Bound {
    /// Whether a point lies on the plane.
    fn holds(&self, point: DVec3) -> bool {
        (point - self.through).dot(self.outward).abs() < PLACE
    }

    /// Whether a point stands on the matter's side of the plane, or on it.
    fn keeps(&self, point: DVec3) -> bool {
        (point - self.through).dot(self.outward) <= PLACE
    }

    /// The plane a quarter turn carries this one onto.
    fn turned(self) -> Bound {
        Bound {
            face: turned_face(self.face),
            through: turned(self.through),
            outward: turned_way(self.outward),
        }
    }
}

/// The plane of a wall of the box, or of its floor.
fn wall(face: usize) -> Bound {
    let (through, outward) = match face {
        FLOOR => (DVec3::ZERO, -DVec3::Z),
        FRONT => (DVec3::ZERO, -DVec3::Y),
        LEFT => (DVec3::ZERO, -DVec3::X),
        RIGHT => (DVec3::new(SIDE, 0.0, 0.0), DVec3::X),
        BACK => (DVec3::new(0.0, SIDE, 0.0), DVec3::Y),
        _ => panic!("face {face} is no wall of the box"),
    };
    Bound {
        face,
        through,
        outward,
    }
}

/// The plane of slope `face` of the pyramid whose apex is `apex`.
fn slope(face: usize, apex: DVec3) -> Bound {
    Bound {
        face,
        through: apex,
        outward: slope_facing(face),
    }
}

/// Whether a lone cut lays a flat or a round.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Cut {
    Flat,
    Round,
}

impl Cut {
    /// What the step is called.
    fn called(self) -> &'static str {
        match self {
            Cut::Flat => "the chamfer",
            Cut::Round => "the fillet",
        }
    }
}

/// A straight edge cut alone: what it is called, the numbers of its two
/// faces and the ways they face, where it starts and ends, and, at each of
/// its two ends, the planes of the faces round that end that the fill ends
/// on — two at a vertex of four edges, one at a corner of three.
#[derive(Clone, Debug)]
struct Lone {
    called: &'static str,
    faces: [usize; 2],
    outward: [DVec3; 2],
    start: DVec3,
    end: DVec3,
    ends: [Vec<Bound>; 2],
}

impl Lone {
    /// The way the edge runs, from its start to its end.
    fn along(&self) -> DVec3 {
        (self.end - self.start).normalize()
    }

    /// How long the edge is.
    fn length(&self) -> f64 {
        self.start.distance(self.end)
    }

    /// The edge's name on the part.
    fn name(&self, part: &Body) -> EdgeName {
        let [one, other] = self.faces;
        edge_between(part, one, other)
    }

    /// The faces a cut on the edge meets: its two, and every face its ends lie
    /// on.
    fn met(&self) -> Vec<usize> {
        let mut met = self.faces.to_vec();
        met.extend(self.ends.iter().flatten().map(|bound| bound.face));
        met
    }

    /// The way from the edge into each of its faces, square to the edge.
    fn ways_in(&self) -> [DVec3; 2] {
        let along = self.along();
        let [one, other] = self.outward;
        let away = |own: DVec3, beside: DVec3| {
            let into = along.cross(own).normalize();
            if into.dot(beside) > 0.0 { -into } else { into }
        };
        [away(one, other), away(other, one)]
    }

    /// The angle its two faces meet at, inside the matter.
    fn angle(&self) -> f64 {
        let [one, other] = self.ways_in();
        one.dot(other).acos()
    }

    /// How far along the edge, from `point`, the line through `point` parallel
    /// to the edge meets the plane `bound`.
    fn reach_on(&self, point: DVec3, bound: &Bound) -> f64 {
        (bound.through - point).dot(bound.outward) / self.along().dot(bound.outward)
    }

    /// Where the line through `point` parallel to the edge leaves the part at
    /// end `side`, nought the start: by the face it meets last among those of
    /// that end, the part being convex.
    fn line_end(&self, point: DVec3, side: usize) -> DVec3 {
        let reaches = self.ends[side]
            .iter()
            .map(|bound| self.reach_on(point, bound));
        let reach = if side == 0 {
            reaches.fold(f64::NEG_INFINITY, f64::max)
        } else {
            reaches.fold(f64::INFINITY, f64::min)
        };
        point + self.along() * reach
    }

    /// Where the section's boundary `boundary`, from nought to one, crosses
    /// from one face of end `side` to the other: the point of that end that
    /// lies on both. None at an end of one face.
    fn kink(&self, side: usize, boundary: impl Fn(f64) -> DVec3) -> Option<DVec3> {
        let [one, other] = match self.ends[side].as_slice() {
            [one, other] => [one, other],
            _ => return None,
        };
        let gap = |share: f64| {
            let point = boundary(share);
            self.reach_on(point, one) - self.reach_on(point, other)
        };
        let (mut low, mut high) = (0.0, 1.0);
        assert!(
            gap(low) * gap(high) < 0.0,
            "the fixture: the faces at the end {side} of {} trade places across its section",
            self.called
        );
        for _ in 0..200 {
            let middle = (low + high) / 2.0;
            if gap(middle) * gap(low) > 0.0 {
                low = middle;
            } else {
                high = middle;
            }
        }
        Some(self.line_end(boundary((low + high) / 2.0), side))
    }

    /// The corners of the flat a chamfer of `distance` lays on the edge alone:
    /// at each end, where the lines along the edge through its two set-backs
    /// leave the part, and, at an end of two faces, where its plane crosses
    /// the edge those two meet along.
    fn flat_corners(&self, distance: f64) -> Vec<DVec3> {
        let [one, other] = self.ways_in().map(|into| self.start + into * distance);
        let chord = |share: f64| one.lerp(other, share);
        let mut corners = Vec::new();
        for side in 0..2 {
            corners.push(self.line_end(one, side));
            corners.push(self.line_end(other, side));
            corners.extend(self.kink(side, chord));
        }
        corners
    }

    /// The round a fillet of `radius` lays on the edge, over the edge's own
    /// length: its axis on the line halfway between the two faces, the radius
    /// in from each.
    fn round(&self, radius: f64) -> Round {
        let [one, other] = self.ways_in();
        let halfway = (one + other).normalize();
        Round {
            start: self.start + halfway * radius / (self.angle() / 2.0).sin(),
            along: self.along(),
            length: self.length(),
            radius,
            facing: self.outward,
        }
    }

    /// The curves a round of `radius` on the edge alone ends along at end
    /// `side`, one on each face of that end: the face's plane, and the two
    /// points the curve runs between — where the lines along the edge through
    /// the round's two tangent lines leave the part, and, at an end of two
    /// faces, where the round crosses the edge those two meet along.
    fn round_ends(&self, radius: f64, side: usize) -> Vec<(Bound, Vec<DVec3>)> {
        let round = self.round(radius);
        let arc = |share: f64| round.start + round.facing_at(share) * radius;
        let mut points = vec![self.line_end(arc(0.0), side), self.line_end(arc(1.0), side)];
        points.extend(self.kink(side, arc));
        self.ends[side]
            .iter()
            .map(|bound| {
                let on: Vec<DVec3> = points
                    .iter()
                    .copied()
                    .filter(|point| bound.holds(*point))
                    .collect();
                (*bound, on)
            })
            .collect()
    }

    /// The edge a quarter turn carries this one onto, called `called`.
    fn turned(&self, called: &'static str) -> Lone {
        Lone {
            called,
            faces: self.faces.map(turned_face),
            outward: self.outward.map(turned_way),
            start: turned(self.start),
            end: turned(self.end),
            ends: self
                .ends
                .clone()
                .map(|side| side.into_iter().map(Bound::turned).collect()),
        }
    }

    /// The prism a cut of `value` on this edge alone takes away, raised along
    /// the edge's whole line past both its ends and out past its faces: cut
    /// from the part by the booleans, it takes what the arithmetic says the
    /// cut takes, the part being convex.
    fn tool(&self, part: &Body, cut: Cut, value: f64) -> Body {
        let along = self.along();
        let into = self.ways_in();
        let u = into[0];
        let v = along.cross(u);
        let flat = |point: DVec3| DVec2::new(point.dot(u), point.dot(v));
        let [mut into_first, mut into_second] = into.map(flat);
        let [mut out_first, mut out_second] = self.outward.map(flat);
        if into_first.perp_dot(into_second) > 0.0 {
            std::mem::swap(&mut into_first, &mut into_second);
            std::mem::swap(&mut out_first, &mut out_second);
        }
        let halfway = (into_first + into_second).normalize();
        let far = 2.0 * value + 2.0;
        let set_back = match cut {
            Cut::Flat => value,
            Cut::Round => value / (self.angle() / 2.0).tan(),
        };
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
        let [before, past] = [-3.0, self.length() + 3.0];
        part.tool_raised(
            &profile(Contour { corners, runs }),
            Frame {
                origin: self.start + along * before,
                u,
                v,
            },
            along * (past - before),
        )
        .expect("the prism raises")
    }
}

/// `first` and the three edges a quarter turn after another carries it onto,
/// called as `called` lists them.
fn four_of(first: Lone, called: [&'static str; 4]) -> Vec<Lone> {
    let mut edges = vec![Lone {
        called: called[0],
        ..first
    }];
    for name in &called[1..] {
        let next = edges[edges.len() - 1].turned(name);
        edges.push(next);
    }
    edges
}

/// The hips of a pyramid, round from the front left.
const HIPS: [&str; 4] = [
    "the front left hip",
    "the front right hip",
    "the back right hip",
    "the back left hip",
];

/// The uprights of `pyramid()`'s box, round from the front left.
const UPRIGHTS: [&str; 4] = [
    "the front left upright",
    "the front right upright",
    "the back right upright",
    "the back left upright",
];

/// The eaves of `pyramid()`, round from the front.
const EAVES: [&str; 4] = [
    "the front eave",
    "the right eave",
    "the back eave",
    "the left eave",
];

/// `pyramid()`'s front left hip, from its foot on the box's corner up to the
/// apex: its fill ends at the foot on the box's front and left walls, and at
/// the apex on the right and back slopes.
fn hip_on_a_box() -> Lone {
    Lone {
        called: HIPS[0],
        faces: [FRONT_SLOPE, LEFT_SLOPE],
        outward: [slope_facing(FRONT_SLOPE), slope_facing(LEFT_SLOPE)],
        start: DVec3::new(0.0, 0.0, 5.0),
        end: ON_A_BOX,
        ends: [
            vec![wall(FRONT), wall(LEFT)],
            vec![slope(RIGHT_SLOPE, ON_A_BOX), slope(BACK_SLOPE, ON_A_BOX)],
        ],
    }
}

/// The front left hip of the pyramid on its own base, from the base's corner
/// up to the apex: its fill ends at the foot on the base alone, and at the
/// apex on the right and back slopes.
fn hip_on_its_base() -> Lone {
    Lone {
        called: HIPS[0],
        faces: [FRONT_SLOPE, LEFT_SLOPE],
        outward: [slope_facing(FRONT_SLOPE), slope_facing(LEFT_SLOPE)],
        start: DVec3::ZERO,
        end: ON_ITS_BASE,
        ends: [
            vec![wall(FLOOR)],
            vec![
                slope(RIGHT_SLOPE, ON_ITS_BASE),
                slope(BACK_SLOPE, ON_ITS_BASE),
            ],
        ],
    }
}

/// The front left upright of `pyramid()`'s box, from the floor up to the
/// front left hip's foot: its fill ends on the floor below, and on the front
/// and left slopes above.
fn upright() -> Lone {
    Lone {
        called: UPRIGHTS[0],
        faces: [FRONT, LEFT],
        outward: [-DVec3::Y, -DVec3::X],
        start: DVec3::ZERO,
        end: DVec3::new(0.0, 0.0, 5.0),
        ends: [
            vec![wall(FLOOR)],
            vec![slope(FRONT_SLOPE, ON_A_BOX), slope(LEFT_SLOPE, ON_A_BOX)],
        ],
    }
}

/// `pyramid()`'s front eave, from the front left hip's foot to the front
/// right's: its fill ends at each foot on the wall below the eave there and on
/// the slope above it.
fn eave() -> Lone {
    Lone {
        called: EAVES[0],
        faces: [FRONT, FRONT_SLOPE],
        outward: [-DVec3::Y, slope_facing(FRONT_SLOPE)],
        start: DVec3::new(0.0, 0.0, 5.0),
        end: DVec3::new(SIDE, 0.0, 5.0),
        ends: [
            vec![wall(LEFT), slope(LEFT_SLOPE, ON_A_BOX)],
            vec![wall(RIGHT), slope(RIGHT_SLOPE, ON_A_BOX)],
        ],
    }
}

/// The section of a hip's flat at `distance`, the triangle between the hip
/// and its two set-backs: its area, and its moments in from the hip along the
/// line halfway between the slopes and across that line either way, as the
/// module's documentation derives them.
fn hip_flat_section(distance: f64) -> [f64; 3] {
    let d = distance;
    [
        3f64.sqrt() * d * d / 4.0,
        3f64.sqrt() * d.powi(3) / 12.0,
        d.powi(3) / 8.0,
    ]
}

/// The section of a hip's round at `radius`, between the slopes and the arc,
/// read as [`hip_flat_section`] reads the flat's.
fn hip_round_section(radius: f64) -> [f64; 3] {
    let r = radius;
    let root3 = 3f64.sqrt();
    [
        r * r * (1.0 / root3 - PI / 6.0),
        r.powi(3) * (11.0 / 18.0 - PI / (3.0 * root3)),
        r.powi(3) * (1.0 / (6.0 * root3) + 1.0 / root3 - 2.0 / 3.0),
    ]
}

/// What a lone cut on a hip of `pyramid()` takes, over a section of area `s`
/// and moments `m_in` and `m_across`: each line of its prism kept from the
/// wall it enters by to the slope it leaves by.
fn taken_on_a_hip_on_a_box([s, m_in, m_across]: [f64; 3]) -> f64 {
    let (root2, root3) = (2f64.sqrt(), 3f64.sqrt());
    5.0 * root3 * s + 3.0 * m_in / (2.0 * root2) - 3.0 * root3 * m_across / (2.0 * root2)
}

/// What a lone cut on a hip of the pyramid on its own base takes, each line
/// of its prism kept from the base to the slope it leaves by.
fn taken_on_a_hip_on_its_base([s, m_in, m_across]: [f64; 3]) -> f64 {
    let (root2, root3) = (2f64.sqrt(), 3f64.sqrt());
    5.0 * root3 * s - (3.0 * m_in + root3 * m_across) / (2.0 * root2)
}

/// What a round of `radius` on an upright of `pyramid()`'s box alone takes,
/// each line of its prism kept from the floor up to the lower of the two
/// slopes over it.
fn taken_by_an_upright_s_round(radius: f64) -> f64 {
    let r = radius;
    5.0 * r * r * (1.0 - PI / 4.0) + r.powi(3) * ((2f64.sqrt() + 1.0) / 3.0 - PI / 4.0)
}

/// What a flat at `distance` on an upright alone takes, read as
/// [`taken_by_an_upright_s_round`] reads the round.
fn taken_by_an_upright_s_flat(distance: f64) -> f64 {
    let d = distance;
    5.0 * d * d / 2.0 + d.powi(3) / 12.0
}

/// What a round of `radius` on an eave of `pyramid()` alone takes, each line
/// of its prism kept from wall to wall, and above the eave from slope to
/// slope.
fn taken_by_an_eave_s_round(radius: f64) -> f64 {
    let r = radius;
    let c = 2f64.sqrt() - 1.0;
    let w = (2.0 * 2f64.sqrt() - 2.0).sqrt();
    let a = (w.powi(3) - 0.5f64.powf(1.5)) / 3.0;
    let b = (0.5 + PI / 4.0) / 2.0 - (c * w + c.asin()) / 2.0;
    let above = c / 12.0 - a + c * b;
    10.0 * r * r * (c - PI / 8.0) - 2.0 * r.powi(3) * above
}

/// What a flat at `distance` on an eave alone takes, read as
/// [`taken_by_an_eave_s_round`] reads the round.
fn taken_by_an_eave_s_flat(distance: f64) -> f64 {
    let d = distance;
    5.0 * d * d / 2f64.sqrt() - (2f64.sqrt() - 1.0) * d.powi(3) / 6.0
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides || name.between == [other.clone(), one.clone()])
        .collect();
    assert_eq!(
        named.len(),
        1,
        "one edge between {first} and {second}: {named:?}"
    );
    named[0].clone()
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

/// The sides of the faces the face numbered `face` meets, one for each edge
/// it has, in order.
fn sides_met(body: &Body, face: usize) -> Vec<Vec<u32>> {
    let own = vec![face as u32];
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

/// Holds that each edge's prism, raised whole and cut from the part by the
/// booleans, keeps what the arithmetic promises the cut on it alone keeps.
fn kept_by_the_booleans(part: &Body, edges: &[Lone], cut: Cut, promised: f64) {
    for edge in edges {
        let kept = part
            .difference(&edge.tool(part, cut, VALUE))
            .expect("the prism cuts the part")
            .volume();
        assert!(
            (kept - promised).abs() <= EXACT * promised,
            "the prism of {} at {VALUE} on {}, cut from the part by the booleans, keeps {kept}, \
             where arithmetic promised {promised}",
            cut.called(),
            edge.called
        );
    }
}

/// What a step of `cut` at [`VALUE`] on the edges `names` finds leaves on
/// `part`: the value asked, the value taken and the body, or why it declined.
fn laid(part: &Body, names: &[EdgeName], cut: Cut) -> Result<(f64, f64, Body), Declined> {
    match cut {
        Cut::Flat => part
            .chamfer_edges(names, VALUE)
            .map(|chamfered| (chamfered.asked, chamfered.taken, chamfered.body)),
        Cut::Round => part
            .fillet_edges(names, VALUE)
            .map(|filleted| (filleted.asked, filleted.taken, filleted.body)),
    }
}

/// Holds that a step reports the value it was asked, and took it whole.
fn taken_whole(asked: f64, taken: f64, what: &str) {
    assert!(
        (asked - VALUE).abs() < PLACE && (taken - VALUE).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {VALUE} fits"
    );
}

/// What a step of `cut` at [`VALUE`] on each of `edges`, each alone in its
/// list, leaves on `part`: held laid on every one of them, the value taken
/// whole, and the bodies handed back.
fn laid_on_each_alone(part: &Body, edges: &[Lone], cut: Cut) -> Vec<Body> {
    let laid: Vec<Result<(f64, f64, Body), Declined>> = edges
        .iter()
        .map(|edge| laid(part, &[edge.name(part)], cut))
        .collect();
    let declined: Vec<String> = edges
        .iter()
        .zip(&laid)
        .filter_map(|(edge, laid)| {
            laid.as_ref()
                .err()
                .map(|declined| format!("{}: {declined:?}", edge.called))
        })
        .collect();
    assert!(
        declined.is_empty(),
        "{} at {VALUE} on each edge alone is built, its ends at a vertex of four edges cut by \
         the faces there as at a corner of three; declined:\n{}",
        cut.called(),
        declined.join("\n")
    );
    edges
        .iter()
        .zip(laid)
        .map(|(edge, laid)| {
            let (asked, taken, body) = laid.expect("declines are held above");
            taken_whole(
                asked,
                taken,
                &format!("{} on {} alone", cut.called(), edge.called),
            );
            body
        })
        .collect()
}

/// Holds that the step laid `count` faces, numbered from where the part's
/// numbers ended, the next step numbering past them, and kept every face of
/// the part; hands back the first one's number.
fn faces_laid(part: &Body, body: &Body, count: usize, what: &str) -> usize {
    let first = part.faces_end();
    assert_eq!(
        faces_made(part, body),
        (first..first + count).collect::<Vec<_>>(),
        "{what}: one face laid for each edge, numbered from where the steps' numbers ended: {:?}",
        numbers(body)
    );
    assert_eq!(
        body.faces_end(),
        first + count,
        "{what}: the next step numbers its own faces past those laid"
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );
    first
}

/// Holds that face `face` of the body is the round a fillet of [`VALUE`]
/// lays on `edge` alone: curved, drawn on its cylinder and on the matter's
/// side of every face its ends lie on, ending on each of them along the curve
/// it cuts from the cylinder between the points the arithmetic gives, met
/// first from outside, and meeting its two faces and the faces its ends lie
/// on, one edge each.
fn holds_the_round(body: &Body, face: usize, edge: &Lone, what: &str) {
    let r = VALUE;
    let round = edge.round(r);
    assert!(
        !body.is_flat(face) && body.plane_of(face).is_none(),
        "{what}: the round is curved, and offers no plane to draw on"
    );
    let drawn: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!drawn.is_empty(), "{what}: the round is drawn");
    let whole = round.part(-2.0 * r, round.length + 2.0 * r);
    for point in &drawn {
        assert!(
            whole.holds(*point),
            "{what}: the round is drawn through {point}, off its cylinder of {r} about the line \
             one radius in from its two faces"
        );
        for bound in edge.ends.iter().flatten() {
            assert!(
                bound.keeps(*point),
                "{what}: the round is drawn through {point}, past face {}, which an end of it \
                 lies on",
                bound.face
            );
        }
    }
    for side in 0..2 {
        for (bound, ends) in edge.round_ends(r, side) {
            assert_eq!(
                ends.len(),
                2,
                "the fixture: the round's end on face {} runs between two points",
                bound.face
            );
            let on: Vec<&DVec3> = drawn.iter().filter(|point| bound.holds(**point)).collect();
            for end in &ends {
                assert!(
                    on.iter().any(|point| point.distance(*end) < PLACE),
                    "{what}: the round's end on face {} runs to {end}, among {on:?}",
                    bound.face
                );
            }
            assert!(
                on.len() > 2,
                "{what}: the round's end on face {} is drawn along the curve the face cuts from \
                 its cylinder: {on:?}",
                bound.face
            );
        }
    }
    met_from_outside(body, face, &round.part(1.5, round.length - 1.5), what);
    assert_eq!(
        sides_met(body, face),
        sides(&edge.met()),
        "{what}: the round meets its two faces along its sides and each face its ends lie on, \
         by one edge each"
    );
}

/// Holds that face `face` of the body is the flat a chamfer of [`VALUE`]
/// lays on `edge` alone: its corners where the arithmetic puts them, facing
/// halfway between the edge's two faces, and meeting those two and the faces
/// its ends lie on, one edge each.
fn holds_the_flat(body: &Body, face: usize, edge: &Lone, what: &str) {
    corners_are(
        body,
        face,
        &edge.flat_corners(VALUE),
        &format!(
            "{what}: the flat, its corners where its set-backs' lines leave the part and where \
             its plane crosses the edges round each end"
        ),
    );
    let [one, other] = edge.outward;
    faces_toward(
        body,
        face,
        (one + other).normalize(),
        &format!("{what}: the flat, halfway between the edge's two faces"),
    );
    assert_eq!(
        sides_met(body, face),
        sides(&edge.met()),
        "{what}: the flat meets the edge's two faces along its set-backs and each face its ends \
         lie on, by one edge each"
    );
}

/// Holds that the body a fillet of [`VALUE`] on `edge` alone left on `part`
/// holds `promised` and lays the round of `edge`, numbered where the part's
/// numbers ended, no corner left where the edge ended.
fn holds_a_lone_round(part: &Body, body: &Body, edge: &Lone, promised: f64, what: &str) {
    sound_drawn_within(
        body,
        promised,
        drawn_within(body, &[edge.round(VALUE)]),
        what,
    );
    let face = faces_laid(part, body, 1, what);
    holds_the_round(body, face, edge, what);
    no_corner_left_at(body, &[edge.start, edge.end], what);
}

/// Holds that the body a chamfer of [`VALUE`] on `edge` alone left on
/// `part` holds `promised` and lays the flat of `edge`, numbered where the
/// part's numbers ended, no corner left where the edge ended.
fn holds_a_lone_flat(part: &Body, body: &Body, edge: &Lone, promised: f64, what: &str) {
    sound_drawn_within(body, promised, EXACT * promised, what);
    let face = faces_laid(part, body, 1, what);
    holds_the_flat(body, face, edge, what);
    no_corner_left_at(body, &[edge.start, edge.end], what);
}

/// Holds that a fillet of [`VALUE`] on each of the four edges `first` and its
/// quarter turns, each alone, lays its round, the part keeping `promised`.
fn lays_each_round_alone(part: &Body, first: Lone, called: [&'static str; 4], promised: f64) {
    let edges = four_of(first, called);
    kept_by_the_booleans(part, &edges, Cut::Round, promised);
    let bodies = laid_on_each_alone(part, &edges, Cut::Round);
    for (edge, body) in edges.iter().zip(&bodies) {
        let what = format!("the fillet at {VALUE} on {} alone", edge.called);
        holds_a_lone_round(part, body, edge, promised, &what);
    }
}

/// Holds that a chamfer of [`VALUE`] on each of the four edges `first` and its
/// quarter turns, each alone, lays its flat, a polygon of `corners` corners,
/// the part keeping `promised`.
fn lays_each_flat_alone(
    part: &Body,
    first: Lone,
    called: [&'static str; 4],
    promised: f64,
    corners: usize,
) {
    let edges = four_of(first, called);
    for edge in &edges {
        assert_eq!(
            edge.flat_corners(VALUE).len(),
            corners,
            "the fixture: the flat on {} alone has {corners} corners",
            edge.called
        );
    }
    kept_by_the_booleans(part, &edges, Cut::Flat, promised);
    let bodies = laid_on_each_alone(part, &edges, Cut::Flat);
    for (edge, body) in edges.iter().zip(&bodies) {
        let what = format!("the chamfer at {VALUE} on {} alone", edge.called);
        holds_a_lone_flat(part, body, edge, promised, &what);
    }
}

#[test]
fn a_fillet_on_a_lone_hip_of_a_pyramid_on_a_box_ends_on_the_box_s_two_walls_and_on_the_two_other_slopes_at_the_apex()
 {
    lays_each_round_alone(
        &pyramid(),
        hip_on_a_box(),
        HIPS,
        ON_A_BOX_VOLUME - taken_on_a_hip_on_a_box(hip_round_section(VALUE)),
    );
}

#[test]
fn a_chamfer_on_a_lone_hip_of_a_pyramid_on_a_box_lays_a_hexagon_ending_on_the_box_s_two_walls_and_on_the_two_other_slopes_at_the_apex()
 {
    lays_each_flat_alone(
        &pyramid(),
        hip_on_a_box(),
        HIPS,
        ON_A_BOX_VOLUME - taken_on_a_hip_on_a_box(hip_flat_section(VALUE)),
        6,
    );
}

#[test]
fn a_fillet_on_a_lone_upright_of_the_box_under_a_pyramid_ends_on_the_two_slopes_over_it() {
    lays_each_round_alone(
        &pyramid(),
        upright(),
        UPRIGHTS,
        ON_A_BOX_VOLUME - taken_by_an_upright_s_round(VALUE),
    );
}

#[test]
fn a_chamfer_on_a_lone_upright_of_the_box_under_a_pyramid_lays_a_pentagon_ending_on_the_two_slopes_over_it()
 {
    lays_each_flat_alone(
        &pyramid(),
        upright(),
        UPRIGHTS,
        ON_A_BOX_VOLUME - taken_by_an_upright_s_flat(VALUE),
        5,
    );
}

#[test]
fn a_fillet_on_a_lone_eave_of_a_pyramid_on_a_box_ends_on_the_wall_and_the_slope_at_each_hip_s_foot()
{
    lays_each_round_alone(
        &pyramid(),
        eave(),
        EAVES,
        ON_A_BOX_VOLUME - taken_by_an_eave_s_round(VALUE),
    );
}

#[test]
fn a_chamfer_on_a_lone_eave_of_a_pyramid_on_a_box_lays_a_hexagon_ending_on_the_wall_and_the_slope_at_each_hip_s_foot()
 {
    lays_each_flat_alone(
        &pyramid(),
        eave(),
        EAVES,
        ON_A_BOX_VOLUME - taken_by_an_eave_s_flat(VALUE),
        6,
    );
}

#[test]
fn a_fillet_on_a_lone_hip_of_a_pyramid_on_its_own_base_ends_on_the_base_and_on_the_two_other_slopes_at_the_apex()
 {
    lays_each_round_alone(
        &pyramid_on_its_base(),
        hip_on_its_base(),
        HIPS,
        ON_ITS_BASE_VOLUME - taken_on_a_hip_on_its_base(hip_round_section(VALUE)),
    );
}

#[test]
fn a_chamfer_on_a_lone_hip_of_a_pyramid_on_its_own_base_lays_a_pentagon_ending_on_the_base_and_on_the_two_other_slopes_at_the_apex()
 {
    lays_each_flat_alone(
        &pyramid_on_its_base(),
        hip_on_its_base(),
        HIPS,
        ON_ITS_BASE_VOLUME - taken_on_a_hip_on_its_base(hip_flat_section(VALUE)),
        5,
    );
}

/// The numbers of the plate's walls the pyramid on its own base stands flush
/// with, the plate raised past the pyramid's numbers: its floor thirty, its
/// top thirty-one, then its walls from the front round to the left.
const PLATE_FRONT: usize = 32;
const PLATE_LEFT: usize = 35;

/// What the plate holds: 20 by 20 by 2.
const PLATE_VOLUME: f64 = 800.0;

/// The pyramid on its own base standing on a plate 20 by 20 by 2 under its
/// base, flush with the plate's front and left walls. The front left hip's
/// foot is a vertex of four edges all meeting outwards, as on `pyramid()`'s
/// box; the front right hip's and the back left hip's are vertices of four
/// where a slope meets the plate's top inwards; the back right hip's is a
/// corner of three, the hip and two feet meeting inwards.
fn pyramid_on_a_plate() -> Body {
    let pyramid = pyramid_on_its_base();
    let plate = pyramid
        .tool_raised(
            &profile(Contour::rectangle(DVec2::ZERO, DVec2::splat(2.0 * SIDE))),
            Frame {
                origin: -DVec3::Z * 2.0,
                ..ground()
            },
            DVec3::Z * 2.0,
        )
        .expect("the plate raises");
    pyramid
        .union(&plate)
        .expect("the pyramid stands on the plate")
}

/// The front left hip of [`pyramid_on_a_plate`], from the plate's corner up
/// to the apex: its fill ends at the foot on the plate's front and left walls,
/// and at the apex on the right and back slopes.
fn hip_on_a_plate() -> Lone {
    Lone {
        called: HIPS[0],
        faces: [FRONT_SLOPE, LEFT_SLOPE],
        outward: [slope_facing(FRONT_SLOPE), slope_facing(LEFT_SLOPE)],
        start: DVec3::ZERO,
        end: ON_ITS_BASE,
        ends: [
            vec![
                Bound {
                    face: PLATE_FRONT,
                    through: DVec3::ZERO,
                    outward: -DVec3::Y,
                },
                Bound {
                    face: PLATE_LEFT,
                    through: DVec3::ZERO,
                    outward: -DVec3::X,
                },
            ],
            vec![
                slope(RIGHT_SLOPE, ON_ITS_BASE),
                slope(BACK_SLOPE, ON_ITS_BASE),
            ],
        ],
    }
}

#[test]
fn a_lone_hip_or_eave_of_a_pyramid_on_a_plate_is_declined_as_unsupported_where_its_vertex_of_four_edges_has_a_concave_one_and_cut_where_it_has_none()
 {
    let part = pyramid_on_a_plate();
    let convex = hip_on_a_plate();
    for (cut, alone) in [
        (Cut::Flat, taken_on_a_hip_on_a_box(hip_flat_section(VALUE))),
        (
            Cut::Round,
            taken_on_a_hip_on_a_box(hip_round_section(VALUE)),
        ),
    ] {
        let promised = PLATE_VOLUME + ON_ITS_BASE_VOLUME - alone;
        let what = format!(
            "{} at {VALUE} on the front left hip alone, its foot on the plate's corner, a vertex \
             of four edges meeting outwards",
            cut.called()
        );
        let edges = [convex.clone()];
        kept_by_the_booleans(&part, &edges, cut, promised);
        let body = laid_on_each_alone(&part, &edges, cut).remove(0);
        match cut {
            Cut::Flat => holds_a_lone_flat(&part, &body, &convex, promised, &what),
            Cut::Round => holds_a_lone_round(&part, &body, &convex, promised, &what),
        }
    }

    let of_three = four_of(hip_on_its_base(), HIPS).remove(2);
    for (cut, alone) in [
        (
            Cut::Flat,
            taken_on_a_hip_on_its_base(hip_flat_section(VALUE)),
        ),
        (
            Cut::Round,
            taken_on_a_hip_on_its_base(hip_round_section(VALUE)),
        ),
    ] {
        let promised = PLATE_VOLUME + ON_ITS_BASE_VOLUME - alone;
        let what = format!(
            "{} at {VALUE} on the back right hip alone, its foot a corner of three with the two \
             feet beside it, which meet inwards",
            cut.called()
        );
        let (asked, taken, body) = laid(&part, &[of_three.name(&part)], cut)
            .unwrap_or_else(|declined| panic!("{what}: declined {declined:?}"));
        taken_whole(asked, taken, &what);
        let drawn = match cut {
            Cut::Flat => EXACT * promised,
            Cut::Round => drawn_within(&body, &[of_three.round(VALUE)]),
        };
        sound_drawn_within(&body, promised, drawn, &what);
        faces_laid(&part, &body, 1, &what);
    }

    for (called, [one, other]) in [
        ("the front right hip", [FRONT_SLOPE, RIGHT_SLOPE]),
        ("the back left hip", [BACK_SLOPE, LEFT_SLOPE]),
        ("the front eave", [FRONT_SLOPE, PLATE_FRONT]),
        ("the left eave", [LEFT_SLOPE, PLATE_LEFT]),
    ] {
        for cut in [Cut::Flat, Cut::Round] {
            assert_eq!(
                laid(&part, &[edge_between(&part, one, other)], cut).err(),
                Some(Declined::Unsupported),
                "{} at {VALUE} on {called} alone, ending at a vertex of four edges where a slope \
                 meets the plate inwards: a broken step and its reason, and no body, until #1025",
                cut.called()
            );
        }
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_two_eaves_of_a_pyramid_on_a_box_sharing_no_vertex_lay_each_as_alone_in_one_step_whatever_the_order()
 {
    let part = pyramid();
    let eaves = four_of(eave(), EAVES);
    for [one, other] in [[0, 2], [1, 3]] {
        for order in [[one, other], [other, one]] {
            for (cut, alone) in [
                (Cut::Flat, taken_by_an_eave_s_flat(VALUE)),
                (Cut::Round, taken_by_an_eave_s_round(VALUE)),
            ] {
                let edges = order.map(|index| &eaves[index]);
                let what = format!(
                    "{} at {VALUE} on {}, then {}, in one step",
                    cut.called(),
                    edges[0].called,
                    edges[1].called
                );
                let promised = ON_A_BOX_VOLUME - 2.0 * alone;
                let names = edges.map(|edge| edge.name(&part));
                let (asked, taken, body) = laid(&part, &names, cut).unwrap_or_else(|declined| {
                    panic!("{what}: declined {declined:?}, where each eave alone is built")
                });
                taken_whole(asked, taken, &what);
                let drawn = match cut {
                    Cut::Flat => EXACT * promised,
                    Cut::Round => drawn_within(&body, &edges.map(|edge| edge.round(VALUE))),
                };
                sound_drawn_within(&body, promised, drawn, &what);
                let first = faces_laid(&part, &body, 2, &what);
                for (rank, edge) in edges.iter().enumerate() {
                    let which = format!("{what}: {}, laid as alone", edge.called);
                    match cut {
                        Cut::Flat => holds_the_flat(&body, first + rank, edge, &which),
                        Cut::Round => holds_the_round(&body, first + rank, edge, &which),
                    }
                    no_corner_left_at(&body, &[edge.start, edge.end], &which);
                }
            }
        }
    }
}

/// How high the slab [`pyramid_on_a_slab`] stands on is: under a fifth of
/// how far below a hip's foot the face a cut of [`VALUE`] on it alone would
/// cross the box's upright, the round's sooner than the flat's.
const SLAB: f64 = 0.1;

/// `pyramid()` with its box cut down to a slab [`SLAB`] high: the square
/// pyramid 5 high standing on a slab 10 by 10, flush with its four walls,
/// numbered as `pyramid()`'s faces are. Each hip's foot is a vertex of four
/// edges meeting outwards, the slab's upright there [`SLAB`] long.
fn pyramid_on_a_slab() -> Body {
    cut_down_to(
        raised(
            Contour::rectangle(DVec2::ZERO, DVec2::splat(SIDE)),
            SLAB + 5.0,
        ),
        DVec3::new(5.0, 5.0, SLAB + 5.0),
    )
}

/// The front left hip of [`pyramid_on_a_slab`], from its foot on the slab's
/// corner up to the apex, its fill read as on `pyramid()`'s box: ending at the
/// foot on the slab's front and left walls, and at the apex on the right and
/// back slopes.
fn hip_on_a_slab() -> Lone {
    let apex = DVec3::new(5.0, 5.0, SLAB + 5.0);
    Lone {
        called: HIPS[0],
        faces: [FRONT_SLOPE, LEFT_SLOPE],
        outward: [slope_facing(FRONT_SLOPE), slope_facing(LEFT_SLOPE)],
        start: DVec3::new(0.0, 0.0, SLAB),
        end: apex,
        ends: [
            vec![wall(FRONT), wall(LEFT)],
            vec![slope(RIGHT_SLOPE, apex), slope(BACK_SLOPE, apex)],
        ],
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_a_lone_hip_whose_face_would_end_past_the_foot_of_the_slab_s_upright_are_declined_as_unsupported()
 {
    let part = pyramid_on_a_slab();
    for edge in four_of(hip_on_a_slab(), HIPS) {
        let round = edge.round(VALUE);
        let [one, other] = edge.ways_in().map(|into| edge.start + into * VALUE);
        let across = [
            (Cut::Flat, edge.kink(0, |share: f64| one.lerp(other, share))),
            (
                Cut::Round,
                edge.kink(0, |share: f64| round.start + round.facing_at(share) * VALUE),
            ),
        ];
        for (cut, on_the_upright) in across {
            let crossing = on_the_upright.expect("two walls round the foot");
            assert!(
                crossing.z < -PLACE,
                "the fixture: {} at {VALUE} on {} alone, carried past its foot as on the box, \
                 would cross the line of the slab's upright at {crossing}, below the slab's \
                 floor, its face ending across the floor on edges that do not end at the foot",
                cut.called(),
                edge.called
            );
            assert!(
                part.difference(&edge.tool(&part, cut, VALUE)).is_ok(),
                "the fixture: the booleans cut the prism of {} on {} from the slab",
                cut.called(),
                edge.called
            );
            assert_eq!(
                laid(&part, &[edge.name(&part)], cut).err(),
                Some(Declined::Unsupported),
                "{} at {VALUE} on {} alone, its face ending past the foot of the slab's upright, \
                 a corner of it on no face of the hip and on no edge ending at the foot: a \
                 broken step and its reason, and no body",
                cut.called(),
                edge.called
            );
        }
    }
}

/// The four edges ending at the foot of the hip `quarters` quarter turns
/// round from the front left one on `pyramid()`: the hip, the eave leaving
/// the foot anticlockwise round the base seen from above, the eave arriving
/// there, and the box's upright under the foot, in that order. At the front
/// left foot: the hip, the front eave, the left eave and the upright.
fn at_the_foot(quarters: usize) -> [Lone; 4] {
    let hips = four_of(hip_on_a_box(), HIPS);
    let eaves = four_of(eave(), EAVES);
    let uprights = four_of(upright(), UPRIGHTS);
    [
        hips[quarters].clone(),
        eaves[quarters].clone(),
        eaves[(quarters + 3) % 4].clone(),
        uprights[quarters].clone(),
    ]
}

/// What two rounds of [`VALUE`] at a hip's foot both take, given back once
/// by the cut of the two together, as the module's documentation measures
/// it: the hip and an eave, the hip and the upright, the two eaves, an eave
/// and the upright.
const SHARED_BY_TWO_ROUNDS: [f64; 4] = [
    0.00707233083665,
    0.01968716375627,
    0.00116391439218,
    0.01425718365517,
];

/// What `pyramid()` keeps once two of the four edges ending at a hip's foot,
/// `pair` naming them as indices into [`at_the_foot`]'s, are cut at
/// [`VALUE`] together, as the module's documentation derives it: two flats,
/// the part clipped by their two planes; two rounds, each taking its lone
/// cut, what both take given back once.
fn kept_by_two_at_a_foot(cut: Cut, pair: [usize; 2]) -> f64 {
    let (root2, root3, root6) = (2f64.sqrt(), 3f64.sqrt(), 6f64.sqrt());
    let [one, other] = [pair[0].min(pair[1]), pair[0].max(pair[1])];
    let which = match (one, other) {
        (0, 3) => 1,
        (0, _) => 0,
        (1, 2) => 2,
        _ => 3,
    };
    match cut {
        Cut::Flat => [
            4639.0 / 7.0 - 61.0 * root2 / 28.0 - root3 / 14.0 + 25.0 * root6 / 224.0,
            2641.0 / 4.0 + 5.0 * root6 / 32.0,
            2000.0 / 3.0 - 29.0 * root2 / 6.0,
            1996.0 / 3.0 - 19.0 * root2 / 6.0,
        ][which],
        Cut::Round => {
            let alone = |edge: usize| match edge {
                0 => taken_on_a_hip_on_a_box(hip_round_section(VALUE)),
                3 => taken_by_an_upright_s_round(VALUE),
                _ => taken_by_an_eave_s_round(VALUE),
            };
            ON_A_BOX_VOLUME - alone(one) - alone(other) + SHARED_BY_TWO_ROUNDS[which]
        }
    }
}

/// The way `edge` leaves `at`, one of its two ends.
fn leaving(edge: &Lone, at: DVec3) -> DVec3 {
    if edge.start.distance(at) < PLACE {
        edge.along()
    } else {
        -edge.along()
    }
}

/// The point of `one`'s axis nearest `other`'s, and how far apart the two
/// axes pass there: where they cross, when they do.
fn axes_crossing(one: &Round, other: &Round) -> (DVec3, f64) {
    let gap = one.start - other.start;
    let across = one.along.dot(other.along);
    let (on_one, on_other) = (one.along.dot(gap), other.along.dot(gap));
    let square = 1.0 - across * across;
    let along_one = (across * on_other - on_one) / square;
    let along_other = (on_other - across * on_one) / square;
    let nearest = one.start + one.along * along_one;
    (
        nearest,
        nearest.distance(other.start + other.along * along_other),
    )
}

/// The booleans' cut from `part` of the prisms a cut of `cut` at [`VALUE`]
/// on each of `edges` takes away, one after the other in the list's order,
/// and the number the face each prism leaves answers to there, by its
/// edge's rank.
fn cut_by_the_booleans(part: &Body, edges: &[&Lone], cut: Cut) -> (Body, Vec<usize>) {
    let mut body = part.clone();
    let mut left = Vec::new();
    for edge in edges {
        let next = body
            .difference(&edge.tool(&body, cut, VALUE))
            .unwrap_or_else(|declined| {
                panic!(
                    "the fixture: the booleans cut the prism of {} on {}: {declined:?}",
                    cut.called(),
                    edge.called
                )
            });
        let made = faces_made(&body, &next);
        assert_eq!(
            made.len(),
            1,
            "the fixture: the prism of {} on {} leaves one face of its own: {made:?}",
            cut.called(),
            edge.called
        );
        left.push(made[0]);
        body = next;
    }
    (body, left)
}

/// The names of every edge between the faces numbered `first` and `second`.
fn edges_between(body: &Body, first: usize, second: usize) -> Vec<EdgeName> {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    body.edge_names()
        .into_iter()
        .filter(|name| name.between == sides || name.between == [other.clone(), one.clone()])
        .collect()
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

/// The corners of the flat face numbered `face`, each once.
fn corners_of(body: &Body, face: usize) -> Vec<DVec3> {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("face {face} is flat"));
    let mut corners: Vec<DVec3> = Vec::new();
    for corner in plane.corners {
        if corners.iter().all(|seen| seen.distance(corner) > PLACE) {
            corners.push(corner);
        }
    }
    corners
}

/// Holds that `body` is the booleans' cut `fixture` face for face: the face
/// laid on each edge, numbered from `start` by its rank, answering to the
/// face its prism leaves there, `left`, and every other number to itself;
/// each face meeting what it meets there, and each flat face flat there too
/// and keeping its corners.
fn as_the_booleans_cut(body: &Body, fixture: &Body, start: usize, left: &[usize], what: &str) {
    let to_fixture = |number: usize| {
        if (start..start + left.len()).contains(&number) {
            left[number - start]
        } else {
            number
        }
    };
    assert_eq!(
        numbers(body)
            .into_iter()
            .map(to_fixture)
            .collect::<BTreeSet<_>>(),
        numbers(fixture),
        "{what}: the faces the booleans' cut has, the face laid on each edge for its prism's: {:?}",
        numbers(body)
    );
    for face in numbers(body) {
        assert_eq!(
            sides_renumbered(body, face, to_fixture),
            sides_met(fixture, to_fixture(face)),
            "{what}: face {face} meets what it meets on the part the booleans cut the prisms from"
        );
        match (body.plane_of(face), fixture.plane_of(to_fixture(face))) {
            (Some(_), Some(expected)) => corners_are(
                body,
                face,
                &expected.corners,
                &format!("{what}: face {face}, as the booleans cut it"),
            ),
            (None, None) => {}
            (flat, _) => panic!(
                "{what}: face {face} is {} where the booleans' cut has it {}",
                if flat.is_some() { "flat" } else { "curved" },
                if flat.is_some() { "curved" } else { "flat" }
            ),
        }
    }
}

/// Holds that the two faces `faces` meet along one edge, and no more.
fn meet_along_one_edge(body: &Body, faces: [usize; 2], what: &str) {
    assert_eq!(
        edges_between(body, faces[0], faces[1]).len(),
        1,
        "{what}: the two faces laid meet along one edge, the mitre"
    );
}

/// Holds that the flats `faces` a chamfer of [`VALUE`] laid on `edges` are
/// mitred where they meet: each facing halfway between its edge's two
/// faces, and the two meeting along one straight edge, sharing its two ends
/// as corners and no other.
fn holds_mitred_flats(body: &Body, faces: [usize; 2], edges: [&Lone; 2], what: &str) {
    for (&face, edge) in faces.iter().zip(edges) {
        let [one, other] = edge.outward;
        faces_toward(
            body,
            face,
            (one + other).normalize(),
            &format!(
                "{what}: the flat of {}, halfway between its two faces",
                edge.called
            ),
        );
    }
    meet_along_one_edge(body, faces, what);
    let [first, second] = faces.map(|face| corners_of(body, face));
    let shared = first
        .iter()
        .filter(|corner| second.iter().any(|other| other.distance(**corner) < PLACE))
        .count();
    assert_eq!(
        shared, 2,
        "{what}: the two flats share the two ends of the straight mitre as corners, and no \
         other: {first:?} and {second:?}"
    );
}

/// Holds that the rounds `faces` a fillet of [`VALUE`] laid on `edges`, both
/// ending at `at`, are mitred where they meet: each curved, drawn on its own
/// cylinder and met first from outside, and the two meeting along one edge,
/// every point both are drawn through on the plane through the crossing of
/// their axes halfway between the ways the two edges leave `at`, where the
/// two cylinders cut each other.
fn holds_mitred_rounds(body: &Body, faces: [usize; 2], edges: [&Lone; 2], at: DVec3, what: &str) {
    let rounds = edges.map(|edge| edge.round(VALUE));
    let (crossing, gap) = axes_crossing(&rounds[0], &rounds[1]);
    assert!(
        gap < PLACE,
        "the fixture: the axes of the rounds on {} and {} cross, where they pass {gap} apart",
        edges[0].called,
        edges[1].called
    );
    let halfway = (leaving(edges[0], at) - leaving(edges[1], at)).normalize();
    let drawn = faces.map(|face| body.triangles_of(face).flatten().collect::<Vec<DVec3>>());
    for ((&face, edge), (round, points)) in faces.iter().zip(edges).zip(rounds.iter().zip(&drawn)) {
        let which = format!("{what}: the round of {}", edge.called);
        assert!(
            !body.is_flat(face) && body.plane_of(face).is_none(),
            "{which} is curved, and offers no plane to draw on"
        );
        assert!(!points.is_empty(), "{which} is drawn");
        let whole = round.part(-round.length, 2.0 * round.length);
        for point in points {
            assert!(
                whole.holds(*point),
                "{which} is drawn through {point}, off its cylinder of {VALUE} about the line one \
                 radius in from its two faces"
            );
        }
        met_from_outside(body, face, &round.part(1.5, round.length - 1.5), &which);
    }
    meet_along_one_edge(body, faces, what);
    let shared: Vec<DVec3> = drawn[0]
        .iter()
        .copied()
        .filter(|point| drawn[1].iter().any(|other| other.distance(*point) < PLACE))
        .collect();
    assert!(
        shared.len() > 2,
        "{what}: the two rounds meet along a curve both are drawn through: {shared:?}"
    );
    for point in &shared {
        let off = (*point - crossing).dot(halfway);
        assert!(
            off.abs() < PLACE,
            "{what}: the two rounds meet at {point}, {off} off the plane through their axes' \
             crossing {crossing} halfway between the two edges, where their cylinders cut each \
             other"
        );
    }
}

/// Holds that each of `edges` is still one sharp edge between its two faces.
fn left_sharp(body: &Body, edges: &[&Lone], what: &str) {
    for edge in edges {
        let [one, other] = edge.faces;
        assert_eq!(
            edges_between(body, one, other).len(),
            1,
            "{what}: {}, left out of the list, is still one sharp edge",
            edge.called
        );
    }
}

/// Holds that the faces laid on the two edges `pair` names among `edges`,
/// all four ending at `at`, are mitred where they meet as a cut of `cut`
/// lays them, `faces` numbering them, and the two edges left out sharp.
fn mitred_at_the_foot(
    body: &Body,
    cut: Cut,
    faces: [usize; 2],
    (edges, pair): (&[Lone; 4], [usize; 2]),
    at: DVec3,
    what: &str,
) {
    let named = pair.map(|index| &edges[index]);
    match cut {
        Cut::Flat => holds_mitred_flats(body, faces, named, what),
        Cut::Round => holds_mitred_rounds(body, faces, named, at, what),
    }
    let others: Vec<&Lone> = (0..4)
        .filter(|index| !pair.contains(index))
        .map(|index| &edges[index])
        .collect();
    left_sharp(body, &others, what);
}

/// Holds that a step of `cut` at [`VALUE`] on every two of the four edges
/// ending at each hip's foot on `pyramid()`, named in either order, lays the
/// booleans' cut of both prisms, the two faces mitred where they meet and
/// the two edges left out sharp, each reading held on the booleans' cut
/// first.
fn mitres_every_two_at_each_hip_s_foot(cut: Cut) {
    let part = pyramid();
    let start = part.faces_end();
    let feet: Vec<[Lone; 4]> = (0..4).map(at_the_foot).collect();
    let mut cases = Vec::new();
    for (quarters, edges) in feet.iter().enumerate() {
        for first in 0..4 {
            for second in (0..4).filter(|&second| second != first) {
                let what = format!(
                    "{} at {VALUE} on {}, then {}, both ending at the foot of {}",
                    cut.called(),
                    edges[first].called,
                    edges[second].called,
                    HIPS[quarters]
                );
                cases.push((edges, [first, second], what));
            }
        }
    }

    let mut fixtures = Vec::new();
    for (edges, pair, what) in &cases {
        let foot = edges[0].start;
        let promised = kept_by_two_at_a_foot(cut, *pair);
        let (fixture, left) = cut_by_the_booleans(&part, &pair.map(|index| &edges[index]), cut);
        let which = format!("the fixture: {what}, the booleans' cut of both prisms");
        assert!(
            (fixture.volume() - promised).abs() <= EXACT * promised,
            "{which} keeps {}, where arithmetic promised {promised}",
            fixture.volume()
        );
        let laid = [left[0], left[1]];
        mitred_at_the_foot(&fixture, cut, laid, (edges, *pair), foot, &which);
        no_corner_left_at(&fixture, &[foot], &which);
        fixtures.push((fixture, left));
    }

    let laid_all: Vec<Result<(f64, f64, Body), Declined>> = cases
        .iter()
        .map(|(edges, pair, _)| laid(&part, &pair.map(|index| edges[index].name(&part)), cut))
        .collect();
    let declined: Vec<String> = cases
        .iter()
        .zip(&laid_all)
        .filter_map(|((_, _, what), laid)| {
            laid.as_ref()
                .err()
                .map(|declined| format!("{what}: {declined:?}"))
        })
        .collect();
    assert!(
        declined.is_empty(),
        "two edges ending at a hip's foot, a vertex of four edges, the two others left sharp, are \
         built, mitred as two edges at a corner; declined:\n{}",
        declined.join("\n")
    );

    for (((edges, pair, what), (fixture, left)), laid) in cases.iter().zip(&fixtures).zip(laid_all)
    {
        let foot = edges[0].start;
        let promised = kept_by_two_at_a_foot(cut, *pair);
        let (asked, taken, body) = laid.expect("declines are held above");
        taken_whole(asked, taken, what);
        let drawn = match cut {
            Cut::Flat => EXACT * promised,
            Cut::Round => drawn_within(&body, &pair.map(|index| edges[index].round(VALUE))),
        };
        sound_drawn_within(&body, promised, drawn, what);
        let first = faces_laid(&part, &body, 2, what);
        as_the_booleans_cut(&body, fixture, start, left, what);
        mitred_at_the_foot(&body, cut, [first, first + 1], (edges, *pair), foot, what);
        no_corner_left_at(&body, &[foot], what);
    }
}

#[test]
fn a_chamfer_on_two_edges_ending_at_one_hip_s_foot_on_a_box_mitres_their_flats_and_leaves_the_two_others_sharp_in_either_order()
 {
    mitres_every_two_at_each_hip_s_foot(Cut::Flat);
}

#[test]
fn a_fillet_on_two_edges_ending_at_one_hip_s_foot_on_a_box_mitres_their_rounds_and_leaves_the_two_others_sharp_in_either_order()
 {
    mitres_every_two_at_each_hip_s_foot(Cut::Round);
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

/// What the pyramid on its own base keeps once three of its four hips are
/// chamfered at [`VALUE`] together, the part clipped by their three flats'
/// planes, as the module's documentation derives it.
fn kept_by_three_flats_at_the_apex() -> f64 {
    1865.0 / 12.0 + 6f64.sqrt() / 2.0
}

/// How far down a hip of the pyramid on its own base, from the apex, the
/// flats of [`VALUE`] on the two hips beside it both cross it.
fn crossed_by_the_flats_beside() -> f64 {
    3.0 * 2f64.sqrt() / 4.0
}

/// Holds that `hip`, left out of the list, is still one sharp edge between
/// its two slopes, from its foot up to where the flats of the two hips
/// beside it cross it, each of the two slopes with a corner at both ends.
fn hip_left_sharp(body: &Body, hip: &Lone, what: &str) {
    left_sharp(body, &[hip], what);
    let top = hip.end - hip.along() * crossed_by_the_flats_beside();
    for face in hip.faces {
        let corners = corners_of(body, face);
        for (end, at) in [("its foot", hip.start), ("its top", top)] {
            assert!(
                corners.iter().any(|corner| corner.distance(at) < PLACE),
                "{what}: {} runs sharp from its foot up to {top}, where the flats of the two hips \
                 beside it cross it, 3√2/4 from the apex: no corner of face {face} at {end}, {at}, \
                 among {corners:?}",
                hip.called
            );
        }
    }
}

#[test]
fn a_chamfer_on_three_of_the_four_hips_at_a_pyramid_s_apex_cuts_the_part_by_each_hip_s_flat_and_leaves_the_fourth_hip_sharp_whatever_the_order()
 {
    let part = pyramid_on_its_base();
    let start = part.faces_end();
    let hips = four_of(hip_on_its_base(), HIPS);
    let promised = kept_by_three_flats_at_the_apex();
    let mut cases = Vec::new();
    for left_out in 0..4 {
        let three: Vec<usize> = (0..4).filter(|&hip| hip != left_out).collect();
        for order in ORDERS_OF_THREE {
            let list = order.map(|at| three[at]);
            let what = format!(
                "the chamfer at {VALUE} on {}, {} left out",
                list.map(|hip| hips[hip].called).join(", then "),
                hips[left_out].called
            );
            cases.push((left_out, list, what));
        }
    }

    let mut fixtures = Vec::new();
    for (left_out, list, what) in &cases {
        let (fixture, left) = cut_by_the_booleans(&part, &list.map(|hip| &hips[hip]), Cut::Flat);
        let which = format!("the fixture: {what}, the booleans' cut of the three prisms");
        assert!(
            (fixture.volume() - promised).abs() <= EXACT * promised,
            "{which} keeps {}, where arithmetic promised {promised}",
            fixture.volume()
        );
        hip_left_sharp(&fixture, &hips[*left_out], &which);
        fixtures.push((fixture, left));
    }

    let laid_all: Vec<Result<(f64, f64, Body), Declined>> = cases
        .iter()
        .map(|(_, list, _)| laid(&part, &list.map(|hip| hips[hip].name(&part)), Cut::Flat))
        .collect();
    let declined: Vec<String> = cases
        .iter()
        .zip(&laid_all)
        .filter_map(|((_, _, what), laid)| {
            laid.as_ref()
                .err()
                .map(|declined| format!("{what}: {declined:?}"))
        })
        .collect();
    assert!(
        declined.is_empty(),
        "three of the four hips at a regular apex chamfered at {VALUE}, the fourth left sharp, \
         are built as the booleans cut each flat; declined:\n{}",
        declined.join("\n")
    );

    for (((left_out, list, what), (fixture, left)), laid) in
        cases.iter().zip(&fixtures).zip(laid_all)
    {
        let (asked, taken, body) = laid.expect("declines are held above");
        taken_whole(asked, taken, what);
        sound_drawn_within(&body, promised, EXACT * promised, what);
        let first = faces_laid(&part, &body, 3, what);
        as_the_booleans_cut(&body, fixture, start, left, what);
        for (rank, &hip) in list.iter().enumerate() {
            let [one, other] = hips[hip].outward;
            faces_toward(
                &body,
                first + rank,
                (one + other).normalize(),
                &format!(
                    "{what}: the flat of {}, halfway between its two slopes",
                    hips[hip].called
                ),
            );
        }
        hip_left_sharp(&body, &hips[*left_out], what);
        no_corner_left_at(&body, &[ON_ITS_BASE], what);
    }
}
