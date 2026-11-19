//! A pocket's round bottom meeting the fills of the floor edges at its
//! pointed or notched end. The block is 30 by 10 by 20, standing on the
//! ground's origin, raised as `cao_part` builds a step: its floor numbered
//! nought, its top one, its sides two to five. A pocket 6 wide, y from 2 to 8,
//! and 6 deep, its floor at z = 14, is cut into its top from its left wall at
//! x = 5 to its shoulders at (25, 2) and (25, 8), past which its right end is
//! a point or a notch turned inwards: its floor numbered six, its walls —
//! front, the end's front, the end's back, back and left — eight to twelve.
//! The floor's front and back edges, filled at half its width, meet across the
//! floor in one round bottom, as #698 lays it: a hollow half cylinder of
//! radius 3 about the line y = 5, z = 17. Each floor edge of the end, named
//! with them, is filled as it would be alone, about the line 3 in from its
//! wall and over the floor. At a symmetric end the three axes cross at one
//! point over the middle line, and each end edge's fill meets the round bottom
//! in a valley on the upright plane through its shoulder and that crossing, as
//! #838's end edge does; the two end edges' fills meet on the plane y = 5. An
//! operation of the kernel on [`Body`], as #838's valley is; the step of the
//! history that calls it comes with the application, in #701.
//!
//! Closes #921.
//! - **The part.** `narrow_pocket`'s block, 30 × 10 × 20, with a pocket 6 wide
//!   and 6 deep, its floor at z = 14, running from x = 5. Its right end is
//!   either: a point at (28, 5); a point at (27, 5); a notch turned inwards to
//!   (22, 5), from shoulders at (25, 2) and (25, 8) — no test: the part every
//!   test below lays its list on, [`End::part`]. Read as: the points stand on
//!   the notch's shoulders too, (25, 2) and (25, 8), the pocket's outline
//!   (5, 2), (25, 2), the tip, (25, 8), (5, 8)
//! - **The list.** The fillet at 3 on the round bottom's pair and the end's two
//!   floor edges — no test: the list the three tests below name, in every
//!   order
//! - **The points.** Each is built in every order as one body, exact, closed
//!   and uncrossed: each end edge's fill meets the round bottom in a valley;
//!   the two end edges' fills meet at the tip in a valley, as two fills meet
//!   at a pocket's bottom corner beside the upright left out (#801); at the
//!   shoulders, every edge ending there meets inwards, which is the corner
//!   #919 lets in; the volume is the union of the four fills' prisms, by
//!   arithmetic and by the booleans —
//!   `a_fillet_on_a_pocket_s_round_bottom_and_the_two_floor_edges_of_its_pointed_end_meets_each_end_fill_in_a_valley_and_the_two_meet_in_a_valley_at_the_tip_whatever_the_order`,
//!   the point at (28, 5), and
//!   `a_fillet_on_a_pocket_s_round_bottom_and_the_two_floor_edges_of_a_blunter_point_meets_each_end_fill_in_a_valley_and_the_two_meet_in_a_valley_at_the_tip_whatever_the_order`,
//!   the point at (27, 5). Read as: on the part, the upright at each shoulder
//!   and at the tip meets inwards, each wall there facing the far corners of
//!   the other, the floor's edges ending there a pocket's, meeting inwards; in
//!   each of the 24 orders, the value taken whole;
//!   the body exact, closed, uncrossed, holding the volume the arithmetic
//!   below gives, which the booleans give too, and its triangles enclosing
//!   that within the drawing's tolerance; the pocket's floor gone, the round
//!   bottom one face answering to both its edges' ranks and each end edge's
//!   fill to its own, nothing laid at a corner, the next step numbering its
//!   faces past them; the round bottom drawn on its half cylinder only and
//!   never past either valley, each end edge's fill on its quarter cylinder
//!   only, never short of its valley and on its own side of y = 5, each met
//!   first from its axis; each valley drawn by both its faces from its
//!   shoulder, at z = 17, down to where the three axes stand over the floor's
//!   plane, the round bottom and an end edge's fill sharing no point off it;
//!   the tip's valley drawn by both end fills from that point up to the tip,
//!   at z = 17, the two sharing no point off y = 5; one vertex at each
//!   shoulder, where the side wall, the end's wall there, the round bottom and
//!   that end's fill meet, one at the tip, where the end's two walls and its
//!   two fills meet, and one where the round bottom and both end fills meet;
//!   each end wall flat, facing back into the pocket, kept from z = 17 up to
//!   the top, meeting the side wall at its shoulder and the other end wall at
//!   the tip along uprights left sharp, one edge each; the front and the back
//!   walls kept from z = 17 up, the left wall above the half circle of 3 about
//!   (5, 5, 17); every face meeting the faces it bounds by one edge each; and
//!   every order laying the body the listed order lays. The tip of (27, 5) is
//!   blunter than the other, 112.6°, its valley on y = 5 all the same
//! - **The notch.** The same, in every order, at the volume the arithmetic and
//!   the booleans give. Its tip is different: there, the two end edges' fills
//!   meet beside the notch's convex upright, left out, as two foot fills meet
//!   round a boss's corner. That is the kind of `beside` corner #919 keeps
//!   away from a round bottom's uprights, but here neither fill is a pair
//!   member —
//!   `a_fillet_on_a_pocket_s_round_bottom_and_the_two_floor_edges_of_its_notched_end_meets_each_end_fill_in_a_valley_and_mitres_the_two_outside_the_notch_s_upright_whatever_the_order`.
//!   Read as the points are read, the upright at the notch's tip meeting
//!   outwards on the part, the two notch walls facing away from each other's
//!   far corners; and the tip read as #801 reads a standing block's foot
//!   corner beside its upright left sharp: the two notch edges' fills run on
//!   past the tip, past the plane of the other's wall, and meet in a mitre on
//!   y = 5 outside the upright, from its foot at (22, 5, 17) down to where the
//!   three axes stand over the floor's plane, (22 − 3√2, 5, 14); each fill met
//!   first from its axis there too, past the plane of the other's wall
//! - **Sub-lists.** The pair with ONE end floor edge, at each point and at the
//!   notch, is probed too. Whatever it does, built or declined, it does it in
//!   every order (#822), and the tests pin it —
//!   `a_fillet_on_a_pocket_s_round_bottom_and_one_floor_edge_of_its_pointed_end_stops_where_that_edge_s_fill_would_meet_the_far_floor_edge_s_on_the_other_end_wall_whatever_the_order`,
//!   `a_fillet_on_a_pocket_s_round_bottom_and_one_floor_edge_of_its_notched_end_alone_or_with_the_upright_at_its_shoulder_is_declined_as_unsupported_whatever_the_order`.
//!   Read as what the probe of the branch's base, 473d9835, found, each end's
//!   front floor edge and its back one in turn, in each of the six orders. At
//!   each point the list is built, but short of the value asked: it takes
//!   `6 − 3√2` at (28, 5) and `16 − 4√13` at (27, 5), where the end edge's
//!   fill, its end trimmed flush on the far end wall, and the far floor edge's
//!   fill, trimmed flush on that wall too, would first meet on it; the pair's
//!   two fills, short of half the width, never meet, and no round bottom
//!   forms. Pinned so: in every order, asked 3, taking that value, the body
//!   exact, closed, uncrossed, holding what the arithmetic gives at that
//!   value, which the booleans give too, the floor kept, a face answering to
//!   each edge's rank, the pair's two fills two faces, and the same body
//!   whatever the order. At the notch the list is declined `Unsupported` in
//!   every order, on the base and on the change that closes this issue, and
//!   is pinned so after the review (2026-11-19), as the criterion has the
//!   tests pin it: a broken step and its reason in every order, each notch
//!   edge in turn, alone with the pair and with the upright at its shoulder
//!   too; what it would hold, 5412.104023 by the arithmetic below and the
//!   booleans, kept for whoever builds it
//! - **What must not break:** #838's square end, #843's full round on a
//!   pointed end, #801's concave corners, #919's pocket end filled all round
//!   and its pit's decline, and every list at 6.5 wide — no test: each is held
//!   where it was built. #838's square end, by
//!   `a_fillet_on_a_pocket_s_round_bottom_and_the_floor_edge_at_its_end_meets_that_edge_s_fill_in_a_valley_whatever_the_order`
//!   and
//!   `a_fillet_on_a_pocket_s_round_bottom_and_the_floor_edges_at_both_its_ends_meets_each_end_s_fill_in_a_valley_whatever_the_order`;
//!   #919's end filled all round, by
//!   `a_fillet_on_a_pocket_s_round_bottom_the_floor_edge_at_its_end_and_both_uprights_there_closes_its_floor_corners_with_one_hollow_quarter_of_a_ball_whatever_the_order`
//!   and
//!   `a_chamfer_on_a_pocket_s_round_bottom_s_pair_the_floor_edge_at_its_end_and_both_uprights_there_closes_that_edge_s_flat_to_nothing_between_the_corners_triangles_whatever_the_order`,
//!   and its pit's decline, by
//!   `a_fillet_on_a_pit_s_upright_beside_a_floor_that_drops_over_a_convex_edge_with_the_round_bottom_s_pair_stays_declined_as_unsupported_whatever_the_order`;
//!   every list at 6.5 wide, by
//!   `the_lists_main_builds_on_a_pocket_6_or_6_5_wide_keep_their_bodies_in_every_order`
//!   — all in `a_round_bottom_meets_the_fill_of_the_edge_at_its_pocket_s_end_in_a_valley.rs`;
//!   #843's full round on a pointed end, by the tests of
//!   `a_full_round_ends_on_a_pointed_end_mitred_against_each_edge_there.rs`,
//!   the first of them
//!   `a_fillet_on_a_full_round_and_the_two_edges_of_its_pointed_end_mitres_each_point_round_against_its_neighbours_up_to_the_crown`;
//!   #801's concave corners, the boss's foot corner among them, by the tests
//!   of `a_chamfer_and_a_fillet_fill_concave_edges_meeting_at_a_corner.rs`.
//!   Read as: "every list at 6.5 wide" is the table of #838 and #919, the
//!   pocket 6.5 wide with its square end, as it is in #919's own list
//! - Not in the issue's wording: the chamfer at 3 on the same lists, which the
//!   branch's base builds in every order on each end — the pair alone, the
//!   pair and both the end's floor edges, the pair and either one — keeps its
//!   body, so that what lets the fillet in changes none of them unseen, as
//!   #919 holds the lists `main` builds —
//!   `a_chamfer_at_3_on_the_round_bottom_s_pair_and_any_floor_edge_of_a_pointed_or_notched_end_keeps_its_body_in_every_order`.
//!   Read as: in every order, asked 3, the value the base took — 3, or at
//!   each point with one end edge the value the fillet takes there — the body
//!   exact, closed, uncrossed, holding what the arithmetic below gives. After
//!   the review (2026-11-19) the test holds its topology too, which the
//!   change that closes this issue moved unseen on the notch with both end
//!   edges, two edges between each floor edge's flat and its end edge's
//!   becoming one: in every order, the edges and vertices of the body the
//!   change lays — 29 and 19 for the pair, 32 and 20 with both end edges, 34
//!   and 22 with one at a point and 32 and 21 at the notch — no two faces
//!   meeting by more than one edge, each floor edge's flat meeting its end
//!   edge's by one, and the same body the listed order lays
//! - **If the derivation disagrees.** If it shows that the notch's tip, or any
//!   of these shapes, is not what the fills give, the test-writer reports it,
//!   and it is recorded here before any test. A shape that cannot be built in
//!   every order is pinned declined in every order, and gets its follow-up —
//!   no test: the derivation below gives every shape the criteria name, the
//!   notch's tip the mitre a boss's foot corner has, and the booleans give
//!   every volume it writes
//! - Decided after the review (2026-11-19): a stepped end, its outline (5, 2),
//!   (25, 2), (25, 5), (22, 5), (22, 8), (5, 8), the floor's front edge ending
//!   on the wall at x = 25 and its back edge on the wall at x = 22, two walls
//!   that share no edge. The fillet at 3 on the two floor edges is built in
//!   every order at 5405.462412; the change declined it with the front edge
//!   named first — the shape of the end read from the first-named edge's wall
//!   and the other's corner — and built it the other way round. Held, and
//!   its mirror across y = 5, whose other order is the one built —
//!   `a_fillet_on_the_round_bottom_of_a_pocket_whose_floor_edges_end_on_the_two_walls_of_a_stepped_end_is_built_in_either_order`.
//!   Read as: in both orders, on each outline, the value taken whole; the
//!   body exact, closed, uncrossed, holding `5667 − 333π/4`, which the
//!   booleans give too; the floor gone, the round bottom one face answering
//!   to both its edges' ranks, drawn on its half cylinder, each half from the
//!   left wall to its own end, and met first from its axis; each half's end
//!   wall facing back along X, kept above the quarter circle its half ends
//!   on; the step kept whole, the longer half's round meeting it along its
//!   foot, where that round touches the floor's plane; the round bottom
//!   meeting the side walls, both end walls, the step and the left wall, the
//!   step the top, both end walls and the round bottom, and the left wall the
//!   top, the side walls and the round bottom, one edge each; one vertex at
//!   the foot of each upright at the step; no two faces meeting by more than
//!   one edge; and both orders laying one body
//! - Decided after the review (2026-11-19): what the change builds beside the
//!   criteria, each declined on the base, held so that none changes unseen,
//!   each built in every order, the value taken whole, exact, closed,
//!   uncrossed, at the volume the arithmetic below gives, which the booleans
//!   give too, no two faces meeting by more than one edge, and one body
//!   whatever the order. The pair alone at each end, each half of the round
//!   bottom ending flush on the end's wall on its side —
//!   `a_fillet_on_a_pocket_s_round_bottom_alone_ends_each_half_of_it_flush_on_its_wall_at_a_pointed_or_notched_end_whatever_the_order`.
//!   Read as: 5305.845160, 5322.982327 and 5408.668162; the round bottom one
//!   face on its half cylinder, never past the end's wall on its side; each
//!   end wall kept above where its half ends on it, from the shoulder down to
//!   the tip's foot; the round bottom meeting the side walls, the left wall
//!   and both end walls, one edge each, and the two end walls at one vertex
//!   with it, the tip's foot, the upright at the tip one sharp edge. The pair
//!   and the upright at either shoulder of each point, its fill meeting the
//!   round bottom in a valley, as #801 mitres a floor edge and the upright it
//!   meets —
//!   `a_fillet_on_a_pocket_s_round_bottom_and_the_upright_at_one_shoulder_of_its_pointed_end_meets_that_upright_s_fill_in_a_valley_whatever_the_order`.
//!   Read as: 5306.602762 and 5324.606534; the round bottom one face on its
//!   half cylinder, the upright's fill one face on its round about the
//!   vertical through the crossing of the axes, between the side wall and the
//!   end's wall, the two sharing points only on the plane x − cx = 17 − z
//!   their cylinders meet on, `cx` the crossing's X. The pair, both end edges
//!   and the upright at the tip of each point, the tip closed by a hollow
//!   piece of sphere —
//!   `a_fillet_on_a_pocket_s_round_bottom_the_two_floor_edges_of_its_pointed_end_and_the_upright_at_its_tip_closes_the_tip_with_a_hollow_piece_of_sphere_whatever_the_order`.
//!   Read as: in each of the 120 orders, 5324.569474 and 5334.227875; the
//!   patch one face numbered after every edge's, drawn on the sphere of 3
//!   about the crossing of the axes, past both end edges' planes through it
//!   square to them and under it only, met first from its centre, facing it,
//!   and meeting the end edges' two fills and the upright's, one edge each;
//!   the upright's fill on its round about the vertical through the crossing,
//!   over z = 17; the end edges' fills never past their planes, no longer
//!   meeting each other
//! - Decided after the review (2026-11-19): the declines the change's
//!   documentation names, each a broken step and its reason, `Unsupported`,
//!   in every order, as #822 has it. One notch edge with the pair, above. A
//!   point with the pair, both end edges and the upright at either shoulder,
//!   at each point, and the notch with the pair, both notch edges and the
//!   upright at its tip, each in its 120 orders —
//!   `a_fillet_on_a_pocket_s_round_bottom_both_floor_edges_of_its_end_and_an_upright_at_a_point_s_shoulder_or_at_a_notch_s_tip_is_declined_as_unsupported_whatever_the_order`.
//!   Not the notch's pair with the upright at its tip alone, nor its pair,
//!   both notch edges and the upright at a shoulder: they split between
//!   orders on `main` already, and #1020 holds them
//!
//! The volumes are arithmetic. The block holds 6000 and the pocket takes its
//! area `S` times its depth 6: the 120 of the strip from x = 5 to the
//! shoulders, and the triangle the end makes with the line x = 25, added at a
//! point and taken away at the notch, 9, 6 and 9. Each fill holds, at a depth
//! `t` under z = 17, what lies within `δ = r − √(r² − t²)` of its wall, `r` the
//! radius 3: what the fills of the list hold together at that depth is what
//! the pocket's outline loses when its named edges are offset inwards by `δ`,
//! meeting where their offsets cross. Offset so, an outline loses its named
//! edges' length `L` times `δ`, less, at each corner between two of them, `δ²`
//! times the tangent of half the turn the outline makes there, which a turn
//! the other way makes negative. Over the depth `δ` integrates to
//! `r²(1 − π/4)` and `δ²` to `r³(5/3 − π/2)`: the body holds
//! `6000 − 6S + L·r²(1 − π/4) − K·r³(5/3 − π/2)`, `K` the sum of those
//! tangents — #838's arithmetic, its square end turning 90° at each shoulder.
//! Here every corner is offset until the depth 3, where the outline closes
//! onto the middle line from x = 5 to the crossing of the axes, and no edge of
//! it closes to nothing on the way.
//!
//! The point at (28, 5): `S` 129, the end's edges `3√2` each, `L = 40 + 6√2`;
//! the outline turns 45° at each shoulder and 90° at the tip, `K = 2(√2 − 1) + 1`.
//! The body holds `5631 − 36√2 − 207π/2 + 27√2π/2`, 5314.912392. The axes
//! cross at (28 − 3√2, 5, 17).
//!
//! The point at (27, 5): `S` 126, the end's edges `√13` each,
//! `L = 40 + 2√13`; the outline turns `atan(3/2)` at each shoulder, the
//! tangent of half of which is `(√13 − 2)/3`, and `π − 2·atan(3/2)` at the
//! tip, `2/3`: `K = (2√13 − 2)/3`. The body holds
//! `5634 − 12√13 − 99π + 9√13π/2`, 5330.687992. The axes cross at
//! (27 − √13, 5, 17).
//!
//! The notch at (22, 5): `S` 111, `L = 40 + 6√2`; the outline turns 135° at
//! each shoulder, `√2 + 1`, and back the other way by 90° at the tip, `−1`:
//! `K = 2√2 + 1`. The body holds `5649 − 36√2 − 153π/2 + 27√2π/2`,
//! 5417.735393. Offset by `δ`, the notch's two edges cross at (22 − √2δ, 5),
//! on the middle line, and each meets the offset side edge at its shoulder.
//! Each notch edge's fill, carried past the tip, runs on past the plane of the
//! other's wall, and what lies past both planes within `δ` of both walls is
//! held by both, the piece #801 joins at a boss's foot corner; past the depth
//! where `δ` is `6 − 3√2`, the shoulder's valley itself lies past that plane,
//! seen from above. The axes cross at (22 − 3√2, 5, 17).
//!
//! The booleans give each: the tool that cuts the pocket, the fills' prisms
//! taken off it first, cut from the block — the pocketed block joined to what
//! the prisms hold in the pocket's air. The round bottom's prism is the pair's
//! two fills' prisms joined, raised along the whole pocket; each end edge's
//! prism is raised along its edge's whole line, through the crossing of the
//! axes. At a point every one of them holds in the pocket only what its fill
//! does, the outline being convex. At the notch an end edge's prism carried
//! past the tip would hold air in front of it that its fill does not, so the
//! two are taken off as one, what both hold, each reaching out past its own
//! wall as far as the other's shoulder: in the pocket that is each fill where
//! it lies alone, the other's prism holding it too from past its own wall,
//! and past both walls the piece both fills hold.
//!
//! The sub-lists take another arithmetic. At the point (28, 5), the pair and
//! its end's front floor edge: the front edge's fill, trimmed flush at the tip
//! on the plane of the end's back wall, covers `δ` of that wall from the tip,
//! and the back floor edge's fill, trimmed flush on it too, `√2δ` from the
//! back shoulder; the wall being `3√2` long, the two meet on it at
//! `δ = 6 − 3√2`, which the step takes as its value, the floor's two fills
//! then short of meeting across it. At the value `v` taken the body holds
//! `5226 + (40 + 3√2)·v²(1 − π/4) + (3/2 − √2)·v³(5/3 − π/2)`, 5255.366772: the
//! three edges' length times the section; and times the cube's share, less
//! the tangent of half the turn at the front shoulder, plus half the
//! cotangent of the turn at the back shoulder and at the tip, where a fill
//! ends flush on a wall left out. At (27, 5) they meet
//! at `δ = √13/(13/12 + √13/3)`, `16 − 4√13`, and the body holds
//! `5244 + (40 + √13)·v²(1 − π/4) + (29/24 − √13/3)·v³(5/3 − π/2)`,
//! 5267.298170. The back floor edge with the pair is the mirror image across
//! y = 5. The booleans give them as they give the whole lists, each of the
//! three fills' prisms raised at the value taken.
//!
//! At the notch, the pair and its front floor edge, were it built, hold at
//! the depth `δ` the round bottom's two halves, each trimmed flush on a notch
//! wall at 45°, `20δ − δ²/2`; the notch edge's fill, trimmed flush on the
//! front wall and on the plane of the back notch wall past the tip,
//! `3√2δ − δ²/2`; less the rhombus the first and the last share at the front
//! shoulder, `√2δ²`, given back past `δ* = 6 − 3√2`, where it crosses the
//! plane that trims the notch edge's fill, by `(3/2 + √2)(δ − δ*)²`. With
//! `ρ* = 3√2 − 3` and `t* = √(r² − ρ*²)`, where `δ` reaches `δ*`, the last
//! integrates to `(3/2 + √2)·∫ (ρ* − √(r² − t²))² dt` from `t*` to `r`. The
//! body holds 5412.104023, which the booleans give, the notch edge's prism
//! raised from its line's crossing with the back notch wall's plane.
//!
//! Chamfered at `d`, a flat holds at the depth `t` what lies within `t` of its
//! wall: `δ = t`, which integrates to `d²/2` and `δ²` to `d³/3`, and every
//! sum above holds with those. The pair alone ends flush on the end's walls
//! left out, half the cotangent of the turn at each shoulder: 5415, 5430 and
//! 5505. The pair and both end edges: `5415 + 9√2`, `5430 + 3√13` and
//! `5505 + 9√2`. The pair and one end edge, at a point, at the value the
//! fillet takes there: 5294.472729 and 5298.285119; at the notch, at 3, the
//! rhombus given back past `δ*` by `(3/2 + √2)(d − δ*)³/3`: `5496 + 9√2`.
//!
//! The stepped end's pocket is 20 long on its front half and 17 on its back
//! half, 3 wide each: it holds `6000 − 6·111`, 5334. Each floor edge's fill
//! adds its section along its length, trimmed flush on its end wall square to
//! it, and the two meet along the middle line from x = 5 to 22 without
//! overlapping; past x = 22 the longer half's round alone, touching the
//! floor's plane at the step's foot: `5334 + 37r²(1 − π/4)`, `5667 − 333π/4`,
//! 5405.462412. The booleans take the round bottom's prism, raised along the
//! whole block, off the tool: past the shorter half's end its other half
//! lies in the step's matter.
//!
//! The pair alone: each half of the round bottom ends flush on the end's wall
//! on its side, which the list leaves out, and the body holds
//! `6000 − 6S + 40r²(1 − π/4) + r³(5/3 − π/2)/tan τ`, `τ` the turn at each
//! shoulder: `5631 − 207π/2`, `5634 − 99π` and `5649 − 153π/2`, the notch's
//! turn of 135° giving back what the points' add.
//!
//! The pair and the upright at a shoulder: the upright's fill holds, seen
//! from above, the region `F` between the side wall, the end's wall and the
//! circle of 3 about the vertical through the crossing of the axes, which
//! touches both, `r²(tan(τ/2) − τ/2)`, up the pocket's depth 6. The round
//! bottom holds of it, at each point of `F` standing `b` across from the
//! middle line, the depth `r − √(r² − b²)`: over `F`, from the side wall to
//! where the circle touches the end's wall, that is
//! `r³(tan(τ/2) − (τ/2)(1 + 1/sin τ) + 2/3 − cos τ/6)`, counted once. The
//! body holds `5586 + 117√2/4 − 855π/8 + 27√2π/8`, 5306.602762, at (28, 5),
//! and `5598 − 99π + 126√13/13 + (9√13 − 27)·atan(3/2)/2`, 5324.606534, at
//! (27, 5). The booleans take the round bottom's prism and the upright's — the
//! region outside that circle, between its two touching points and out into
//! the matter — off the tool.
//!
//! The pair, both end edges and the upright at the tip: the three edges at
//! the tip are closed as #801 closes a pocket's bottom corner, by a hollow
//! piece of the sphere of 3 about the crossing of the axes, one radius from
//! the floor and from both end walls. Past the planes through that centre
//! square to the end's edges and under it — the cell — the body now holds all
//! but the ball, where the whole list without the upright left air of the
//! quadrilateral `ρ² tan(τ/2)` at each depth `t`, `ρ = √(r² − t²)`: the cell
//! adds `2r³/3·tan(τ/2) − r³τ/3` to it, `τ` the turn at the tip, and the
//! upright's fill over z = 17 the region `F` of the tip, `r²(tan(τ/2) − τ/2)`,
//! up the 3 left. The body holds `5676 − 36√2 − 459π/4 + 27√2π/2`,
//! 5324.569474, at (28, 5), and
//! `5664 − 12√13 − 243π/2 + 9√13π/2 + 45·atan(3/2)`, 5334.227875, at
//! (27, 5). The booleans take the fills' prisms, the upright's and the cell's
//! box off the tool, then the ball's share of the cell, `r³τ/3`, from the
//! volume.

use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Chamfered, Declined, EdgeName, FacePlane, Filleted, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it: each is
/// placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// How far from a valley a ray aims, on either side of it, to meet the
/// triangles of the fill it aims at rather than the other's: well over the
/// drawing's tolerance on the block, [`DRAWN`] times its reach.
const CLEAR_OF_THE_VALLEY: f64 = 0.1;

/// The number a raised block's top answers to.
const TOP: usize = 1;

/// The numbers the pocket's faces answer to: its floor six, its walls — front,
/// the end's front, the end's back, back and left — eight to twelve, the
/// tool's top, above the block, gone.
const POCKET_FLOOR: usize = 6;
const POCKET_FRONT: usize = 8;
const END_FRONT: usize = 9;
const END_BACK: usize = 10;
const POCKET_BACK: usize = 11;
const POCKET_LEFT: usize = 12;

/// The block the pocket is cut into: 30 along X, 10 across, 20 high.
const LENGTH: f64 = 30.0;
const BREADTH: f64 = 10.0;
const HEIGHT: f64 = 20.0;

/// Where the pocket's walls stand: the left at x = 5, the front at y = 2, the
/// back its width further on; its shoulders, where the end begins, at x = 25.
const LEFT_WALL_AT: f64 = 5.0;
const SHOULDERS_AT: f64 = 25.0;
const FRONT_WALL_AT: f64 = 2.0;

/// The pocket's floor, 6 under the block's top.
const FLOOR_AT: f64 = 14.0;

/// The pocket's width, twice [`RADIUS`]: the fills of its floor's front and
/// back edges meet across the floor in one round bottom.
const WIDTH: f64 = 6.0;

/// The line the pocket's end is symmetric about, half its width from its
/// front wall.
const MIDDLE: f64 = FRONT_WALL_AT + WIDTH / 2.0;

/// The value every list is filleted at: half [`WIDTH`].
const RADIUS: f64 = WIDTH / 2.0;

/// The height the fills reach up the walls: [`RADIUS`] over the floor.
const ROUND_TOP: f64 = FLOOR_AT + RADIUS;

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

/// The tool an outline raises from `base` by `height`, for `part` to number
/// its faces after its own, as `cao_part` raises a step: its floor first, then
/// its top, then its walls, one for each run of the outline.
fn raised_on(part: &Body, outline: Contour, base: f64, height: f64) -> Body {
    part.tool_raised(
        &profile(outline),
        Frame {
            origin: DVec3::Z * base,
            ..ground()
        },
        DVec3::Z * height,
    )
    .expect("the tool raises")
}

/// The block 30 by 10 by 20, standing on the ground's origin.
fn block() -> Body {
    Body::default()
        .union(&raised_on(
            &Body::default(),
            Contour::rectangle(DVec2::ZERO, DVec2::new(LENGTH, BREADTH)),
            0.0,
            HEIGHT,
        ))
        .expect("the block is built")
}

/// One of the pocket's two side walls, and the floor edge and the end's wall
/// on its side of the middle line.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Side {
    Front,
    Back,
}

impl Side {
    /// The side wall.
    fn wall(self) -> usize {
        match self {
            Side::Front => POCKET_FRONT,
            Side::Back => POCKET_BACK,
        }
    }

    /// The end's wall on this side of the middle line.
    fn end_wall(self) -> usize {
        match self {
            Side::Front => END_FRONT,
            Side::Back => END_BACK,
        }
    }

    /// The other side.
    fn other(self) -> Side {
        match self {
            Side::Front => Side::Back,
            Side::Back => Side::Front,
        }
    }

    /// Where the side wall stands along Y.
    fn wall_at(self) -> f64 {
        match self {
            Side::Front => FRONT_WALL_AT,
            Side::Back => FRONT_WALL_AT + WIDTH,
        }
    }

    /// A way seen from above as it runs on the front side, or mirrored across
    /// the middle line onto the back.
    fn way_seen(self, way: DVec2) -> DVec2 {
        match self {
            Side::Front => way,
            Side::Back => DVec2::new(way.x, -way.y),
        }
    }

    /// How far a point stands on this side of the middle line.
    fn across(self, point: DVec3) -> f64 {
        match self {
            Side::Front => MIDDLE - point.y,
            Side::Back => point.y - MIDDLE,
        }
    }

    /// What the floor's edge on this side's wall is called.
    fn floor_edge(self) -> ([usize; 2], &'static str) {
        match self {
            Side::Front => (
                [POCKET_FLOOR, POCKET_FRONT],
                "the floor's edge on the front wall",
            ),
            Side::Back => (
                [POCKET_FLOOR, POCKET_BACK],
                "the floor's edge on the back wall",
            ),
        }
    }

    /// What the floor's edge on the end's wall on this side is called.
    fn end_edge(self) -> ([usize; 2], &'static str) {
        match self {
            Side::Front => (
                [POCKET_FLOOR, END_FRONT],
                "the floor's edge on the end's front wall",
            ),
            Side::Back => (
                [POCKET_FLOOR, END_BACK],
                "the floor's edge on the end's back wall",
            ),
        }
    }
}

/// The pocket's right end: a point, or a notch turned inwards, its tip on the
/// middle line, its shoulders at (25, 2) and (25, 8).
#[derive(Clone, Copy, Debug)]
struct End {
    tip: DVec2,
    called: &'static str,
}

impl End {
    /// The point at (28, 5), its tip square.
    const POINTED: End = End {
        tip: DVec2::new(28.0, MIDDLE),
        called: "the pocket 6 wide pointed at (28, 5)",
    };

    /// The point at (27, 5), its tip blunter, 112.6°.
    const BLUNTER: End = End {
        tip: DVec2::new(27.0, MIDDLE),
        called: "the pocket 6 wide pointed at (27, 5)",
    };

    /// The notch turned inwards to (22, 5), its tip square.
    const NOTCHED: End = End {
        tip: DVec2::new(22.0, MIDDLE),
        called: "the pocket 6 wide notched in to (22, 5)",
    };

    /// The pocket's outline seen from above: front, the end's front, the
    /// end's back, back and left, its walls eight to twelve.
    fn outline(self) -> Contour {
        Contour::straight(vec![
            DVec2::new(LEFT_WALL_AT, FRONT_WALL_AT),
            self.shoulder(Side::Front),
            self.tip,
            self.shoulder(Side::Back),
            DVec2::new(LEFT_WALL_AT, FRONT_WALL_AT + WIDTH),
        ])
    }

    /// The block with this pocket cut into its top down to the floor, as
    /// `cao_part` cuts a step: the tool's faces numbered past the block's, its
    /// floor six, its top, above the block, seven and gone, its walls from
    /// eight.
    fn part(self) -> Body {
        let block = block();
        block
            .difference(&self.tool(&block))
            .expect("the pocket is cut")
    }

    /// The tool that cuts the pocket out of `block`.
    fn tool(self, block: &Body) -> Body {
        raised_on(block, self.outline(), FLOOR_AT, HEIGHT - FLOOR_AT + 1.0)
    }

    /// Whether the end is a notch turned inwards rather than a point.
    fn notched(self) -> bool {
        self.tip.x < SHOULDERS_AT
    }

    /// The shoulder on `side`, where the side wall meets the end's wall.
    fn shoulder(self, side: Side) -> DVec2 {
        DVec2::new(SHOULDERS_AT, side.wall_at())
    }

    /// The way the end's wall on `side` runs, seen from above, from its
    /// shoulder to the tip.
    fn way(self, side: Side) -> DVec2 {
        (self.tip - self.shoulder(side)).normalize()
    }

    /// The way out of the pocket through the end's wall on `side`, square to
    /// it, seen from above: into the matter behind it.
    fn outward(self, side: Side) -> DVec2 {
        let front = self.way(Side::Front);
        side.way_seen(DVec2::new(front.y, -front.x))
    }

    /// How far out a point stands from the axis of the fill of the end's floor
    /// edge on `side`, toward its wall, square to it: nought on the axis,
    /// [`RADIUS`] on the wall.
    fn reach(self, side: Side, point: DVec3) -> f64 {
        RADIUS + (point.truncate() - self.tip).dot(self.outward(side))
    }

    /// Where the three axes cross: [`RADIUS`] over the floor's middle line,
    /// and as far from each end wall.
    fn crossing(self) -> DVec3 {
        DVec3::new(
            self.tip.x - RADIUS / self.outward(Side::Front).x,
            MIDDLE,
            ROUND_TOP,
        )
    }

    /// Where the three fills touch the floor's plane together, under the
    /// crossing of their axes: the valleys' and the tip's lowest point.
    fn deepest(self) -> DVec3 {
        self.crossing() - DVec3::Z * RADIUS
    }

    /// Where the end's two walls meet at the fills' top: the tip's foot.
    fn tip_top(self) -> DVec3 {
        self.tip.extend(ROUND_TOP)
    }

    /// Where the round bottom leaves the side wall at the shoulder on `side`.
    fn shoulder_top(self, side: Side) -> DVec3 {
        self.shoulder(side).extend(ROUND_TOP)
    }

    /// How far a point stands past the valley on its side of the middle line,
    /// on the side of the end edge's fill: how far out from that fill's axis,
    /// less how far across from the middle line. Nought on the valley's plane,
    /// through the shoulder and the crossing of the axes.
    fn past_the_valley(self, point: DVec3) -> f64 {
        let side = if point.y <= MIDDLE {
            Side::Front
        } else {
            Side::Back
        };
        self.reach(side, point) - side.across(point)
    }

    /// The fill of the end's floor edge on `side`, whole: a hollow quarter
    /// round about the line [`RADIUS`] in from its wall and over the floor,
    /// from level with its shoulder to level with the tip, or at the notch
    /// on to the crossing of the axes, from facing the floor's way to facing
    /// back from the wall.
    fn fill(self, side: Side) -> Hollow {
        let start = self.shoulder(side) - self.outward(side) * RADIUS;
        let way = self.way(side);
        let to_the_crossing = (self.crossing().truncate() - start).dot(way);
        Hollow {
            start: start.extend(ROUND_TOP),
            along: way.extend(0.0),
            length: to_the_crossing.max(self.tip.distance(self.shoulder(side))),
            radius: RADIUS,
            facing: [DVec3::Z, -self.outward(side).extend(0.0)],
        }
    }

    /// The part of [`End::fill`] whose axis runs through the pocket's air, a
    /// half unit past its side wall: where a ray from the axis starts in the
    /// air. At a point all of it; at the notch, whose shoulders are sharp, the
    /// axis enters the pocket only some way past them.
    fn fill_seen_from_its_axis(self, side: Side) -> Hollow {
        let whole = self.fill(side);
        let in_the_air = side.wall_at() + (MIDDLE - side.wall_at()).signum() * 0.5;
        let skipped = ((in_the_air - whole.start.y) / whole.along.y).max(0.0);
        Hollow {
            start: whole.axis_at(skipped / whole.length),
            length: whole.length - skipped,
            ..whole
        }
    }

    /// The round bottom, whole, as far along X as either half may run where
    /// it ends flush on the end's wall on its side: to the tip at a point, to
    /// the shoulders at a notch.
    fn round_bottom_running_to_its_walls(self) -> [Hollow; 2] {
        round_bottom_along(self.tip.x.max(SHOULDERS_AT) - LEFT_WALL_AT)
    }

    /// The pocket's area seen from above: the strip from the left wall to the
    /// shoulders, and the triangle the end makes with the line x = 25, added
    /// at a point and taken away at a notch.
    fn area(self) -> f64 {
        (SHOULDERS_AT - LEFT_WALL_AT) * WIDTH + WIDTH / 2.0 * (self.tip.x - SHOULDERS_AT)
    }

    /// The length of each of the end's two floor edges.
    fn edge_length(self) -> f64 {
        self.tip.distance(self.shoulder(Side::Front))
    }

    /// The turns the outline makes, left positive, at the front shoulder and
    /// at the tip.
    fn turns(self) -> [f64; 2] {
        let turn = |from: DVec2, to: DVec2| from.perp_dot(to).atan2(from.dot(to));
        let front = self.way(Side::Front);
        let back = (self.shoulder(Side::Back) - self.tip).normalize();
        [turn(DVec2::X, front), turn(front, back)]
    }

    /// What the pocket holds once the round bottom's pair and both the end's
    /// floor edges are filleted at [`RADIUS`], as the module's documentation
    /// derives it: the block less the pocket, plus the named edges' length
    /// times the round's section, less at each corner between two of them the
    /// tangent of half the turn there times the cube's share.
    fn kept_all_round(self) -> f64 {
        let r = RADIUS;
        let [shoulder, tip] = self.turns();
        let length = 2.0 * (SHOULDERS_AT - LEFT_WALL_AT) + 2.0 * self.edge_length();
        let tangents = 2.0 * (shoulder / 2.0).tan() + (tip / 2.0).tan();
        LENGTH * BREADTH * HEIGHT - self.area() * (HEIGHT - FLOOR_AT)
            + length * r * r * (1.0 - PI / 4.0)
            - tangents * r.powi(3) * (5.0 / 3.0 - PI / 2.0)
    }

    /// The prism of the end edge's fill on `side` at `radius`: what lies
    /// beyond the quarter round about the line `radius` in from the wall and
    /// over the floor, a unit past the wall and under the floor, raised along
    /// that line by `along` from its point `through`, seen from above, ahead
    /// toward the shoulder on its side.
    fn end_prism(self, side: Side, radius: f64, through: DVec2, along: [f64; 2]) -> Prism {
        let [from, to] = match side {
            Side::Front => along,
            Side::Back => [-along[1], -along[0]],
        };
        Prism {
            through,
            outward: self.outward(side),
            radius,
            along: [from, to],
            beyond: 1.0,
            below: 1.0,
            top: FLOOR_AT + radius,
        }
    }

    /// The prisms the round bottom and both end edges' fills at [`RADIUS`]
    /// take off the tool that cuts the pocket, each raised for `part` to
    /// number its faces after its own: the round bottom's along the whole
    /// pocket; at a point each end edge's along its edge's whole line; at the
    /// notch the two end edges' as one, what both hold, each raised out past
    /// the other's wall. The end edges' are raised through the crossing of the
    /// axes, where the three rounds touch the floor's plane together.
    fn prisms(self, part: &Body) -> Vec<Body> {
        let through = self.crossing().truncate();
        let round_bottom = round_bottom_prism(part);
        let [front, back] = [Side::Front, Side::Back].map(|side| {
            if self.notched() {
                Prism {
                    beyond: 5.0,
                    ..self.end_prism(side, RADIUS, through, [-RADIUS, 10.0])
                }
            } else {
                self.end_prism(side, RADIUS, through, [-8.0, 12.0])
            }
        });
        let top = if self.notched() {
            ROUND_TOP + 0.5
        } else {
            ROUND_TOP
        };
        let front = Prism { top, ..front }.raised_on(&round_bottom);
        let back = Prism { below: 1.5, ..back }.raised_on(&front);
        if !self.notched() {
            return vec![round_bottom, front, back];
        }
        let alone = front
            .difference(&back)
            .expect("the front end prism less the back one");
        let both = front.difference(&alone).expect("what both end prisms hold");
        vec![round_bottom, both]
    }

    /// What the pocket holds once the round bottom and both end edges' fills
    /// are laid at [`RADIUS`], computed apart from the step: the tool that cuts
    /// the pocket, [`End::prisms`] taken off it, cut from the block.
    fn by_booleans(self) -> f64 {
        let block = block();
        let tool = self.tool(&block);
        cut_with_its_fills(&block, &tool, &self.prisms(&tool))
    }
}

/// The block less the pocket's tool, `prisms` taken off the tool first: the
/// pocketed block joined to what the prisms hold in the pocket's air.
fn cut_with_its_fills(block: &Body, tool: &Body, prisms: &[Body]) -> f64 {
    let air = prisms.iter().fold(tool.clone(), |air, prism| {
        air.difference(prism)
            .expect("a fill's prism comes off the tool")
    });
    block.difference(&air).expect("the pocket is cut").volume()
}

/// A fill's prism: what lies beyond the quarter round of `radius` about the
/// line through `through`, seen from above, `radius` over the floor, the
/// round facing from the floor's way to back from the wall that `outward`
/// points to; raised along the line from `along[0]` to `along[1]`, measured
/// along the way `outward` turned a quarter clockwise, seen from above; out
/// `beyond` past the wall, `below` under the floor, and up to `top` past the
/// wall.
#[derive(Clone, Copy, Debug)]
struct Prism {
    through: DVec2,
    outward: DVec2,
    radius: f64,
    along: [f64; 2],
    beyond: f64,
    below: f64,
    top: f64,
}

impl Prism {
    /// The prism raised, for `part` to number its faces after its own.
    fn raised_on(self, part: &Body) -> Body {
        let r = self.radius;
        let round_top = FLOOR_AT + r;
        let mut corners = vec![
            DVec2::new(0.0, FLOOR_AT - self.below),
            DVec2::new(r + self.beyond, FLOOR_AT - self.below),
            DVec2::new(r + self.beyond, self.top),
        ];
        if self.top > round_top {
            corners.push(DVec2::new(r, self.top));
        }
        corners.push(DVec2::new(r, round_top));
        let round = corners.len() - 1;
        corners.push(DVec2::new(0.0, FLOOR_AT));
        let mut runs = vec![Run::Straight; corners.len()];
        runs[round] = Run::Round {
            center: DVec2::new(0.0, round_top),
            turn: -PI / 2.0,
        };
        let way = DVec2::new(self.outward.y, -self.outward.x);
        let [from, to] = self.along;
        let start = self.through + way * from;
        part.tool_raised(
            &profile(Contour { corners, runs }),
            Frame {
                origin: start.extend(0.0),
                u: self.outward.extend(0.0),
                v: DVec3::Z,
            },
            way.extend(0.0) * (to - from),
        )
        .expect("the fill's prism raises")
    }
}

/// The round bottom's prism, the pair's two fills' joined: what lies under the
/// half round about the line [`RADIUS`] over the floor's middle line, out a
/// unit past the front and back walls and under the floor, raised along X
/// across the whole block and past it.
fn round_bottom_prism(part: &Body) -> Body {
    let r = RADIUS;
    let [near, far] = [FRONT_WALL_AT, FRONT_WALL_AT + WIDTH];
    let corners = vec![
        DVec2::new(near - 1.0, FLOOR_AT - r),
        DVec2::new(far + 1.0, FLOOR_AT - r),
        DVec2::new(far + 1.0, ROUND_TOP),
        DVec2::new(far, ROUND_TOP),
        DVec2::new(near, ROUND_TOP),
        DVec2::new(near - 1.0, ROUND_TOP),
    ];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[3] = Run::Round {
        center: DVec2::new(MIDDLE, ROUND_TOP),
        turn: -PI,
    };
    part.tool_raised(
        &profile(Contour { corners, runs }),
        Frame {
            origin: DVec3::X * -1.0,
            u: DVec3::Y,
            v: DVec3::Z,
        },
        DVec3::X * (LENGTH + 2.0),
    )
    .expect("the round bottom's prism raises")
}

/// The fill at `radius` of the floor's edge on `side`, alone: its prism raised
/// along X across the whole block and past it.
fn floor_edge_prism(side: Side, radius: f64) -> Prism {
    let outward = side.way_seen(-DVec2::Y);
    let [from, to] = match side {
        Side::Front => [-LENGTH - 1.0, 1.0],
        Side::Back => [-1.0, LENGTH + 1.0],
    };
    Prism {
        through: DVec2::new(0.0, side.wall_at()) - outward * radius,
        outward,
        radius,
        along: [from, to],
        beyond: 1.0,
        below: 1.0,
        top: FLOOR_AT + radius,
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
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    body.edge_names()
        .into_iter()
        .filter(|name| name.between == sides || name.between == [other.clone(), one.clone()])
        .collect()
}

/// Every number some face of the body answers to: a step's, below where the
/// steps' numbers end, and a piece's, past them.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&number| body.has_face(number))
        .collect()
}

/// Whether some point of `points` stands at `point`.
fn drawn_through(points: &[DVec3], point: DVec3) -> bool {
    points.iter().any(|other| other.distance(point) < PLACE)
}

/// Whether every point of `some` stands at a point of `others`, and every
/// point of `others` at one of `some`.
fn same_points(some: &[DVec3], others: &[DVec3]) -> bool {
    some.iter().all(|point| drawn_through(others, *point))
        && others.iter().all(|point| drawn_through(some, *point))
}

/// The plane the face numbered `face` lies on, its normal and its corners.
fn plane(body: &Body, face: usize, what: &str) -> FacePlane {
    body.plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"))
}

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    let corners = plane(body, face, what).corners;
    assert!(
        same_points(&corners, expected),
        "{what}: face {face} has its corners at {corners:?}, where they are {expected:?}"
    );
}

/// Holds that a face one of whose edges runs along an arc has its corners at
/// the points expected and along that arc: each point expected a corner, and
/// every other corner — the points the arc is drawn through — one `on_the_arc`
/// takes.
fn corners_are_along_an_arc(
    body: &Body,
    face: usize,
    expected: &[DVec3],
    on_the_arc: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let corners = plane(body, face, what).corners;
    let off: Vec<&DVec3> = corners
        .iter()
        .filter(|&&corner| !drawn_through(expected, corner) && !on_the_arc(corner))
        .collect();
    let missing: Vec<&DVec3> = expected
        .iter()
        .filter(|&&point| !drawn_through(&corners, point))
        .collect();
    assert!(
        off.is_empty() && missing.is_empty(),
        "{what}: face {face} has corners at {off:?}, neither one of {expected:?} nor on its \
         arc, and none at {missing:?}"
    );
}

/// Holds that the face numbered `face` is flat and faces the way `facing`
/// points.
fn faces_toward(body: &Body, face: usize, facing: DVec3, what: &str) {
    let normal = plane(body, face, what).normal;
    assert!(
        normal.distance(facing) < PLACE,
        "{what}: face {face} faces {normal}, not {facing}"
    );
}

/// Whether the flat face numbered `one` faces every corner of the flat face
/// numbered `other` that does not lie on its plane, and some corner does not:
/// the air on the side of `one` the other face stands on.
fn faces_the_corners_of(part: &Body, one: usize, other: usize) -> bool {
    let what = format!("the faces {one} and {other} of the part");
    let [facing, faced] = [one, other].map(|face| plane(part, face, &what));
    let heights: Vec<f64> = faced
        .corners
        .iter()
        .map(|&corner| (corner - facing.corners[0]).dot(facing.normal))
        .collect();
    heights.iter().all(|&height| height >= -PLACE) && heights.iter().any(|&height| height > PLACE)
}

/// Whether the edge between the flat faces numbered `one` and `other` meets
/// inwards: each facing the far corners of the other, the air between them.
fn meets_inwards(part: &Body, one: usize, other: usize) -> bool {
    faces_the_corners_of(part, one, other) && faces_the_corners_of(part, other, one)
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
/// [`sides_met_by`] lists them, each side every number the face across it
/// answers to: a round bottom answers to both its edges'.
fn sides_answering(faces: &[&[usize]]) -> Vec<Vec<u32>> {
    let mut sorted: Vec<Vec<u32>> = faces
        .iter()
        .map(|numbers| numbers.iter().map(|&number| number as u32).collect())
        .collect();
    sorted.sort();
    sorted
}

/// How many vertices of the body stand where exactly the faces `faces` meet,
/// each given as every number it answers to.
fn vertices_among(body: &Body, faces: &[&[usize]]) -> usize {
    let sides = sides_answering(faces);
    body.vertex_names()
        .into_iter()
        .filter(|vertex| vertex.faces == sides)
        .count()
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

/// Holds that a step reports the value it was asked, and took `value`.
fn takes(asked: f64, taken: f64, value: f64, what: &str) {
    assert!(
        (asked - RADIUS).abs() < PLACE && (taken - value).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {RADIUS} is asked and {value} taken"
    );
}

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
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

/// The round a fillet lays in a concave corner: a cylinder of `radius` about
/// the line from `start` along `along` for `length`, standing in the air the
/// two faces leave between them, the round facing its axis, from the way the
/// edge's first face faces to the way its second does.
#[derive(Clone, Copy, Debug)]
struct Hollow {
    start: DVec3,
    along: DVec3,
    length: f64,
    radius: f64,
    facing: [DVec3; 2],
}

impl Hollow {
    /// The point of the axis `share` of the way along it.
    fn axis_at(&self, share: f64) -> DVec3 {
        self.start + self.along * self.length * share
    }

    /// The way the round faces `share` of the way round it, from its first
    /// face's way to its second's: toward its axis.
    fn facing_at(&self, share: f64) -> DVec3 {
        let [first, second] = self.facing;
        let turn = first.dot(second).acos() * share;
        let across = (second - first * first.dot(second)).normalize();
        first * turn.cos() + across * turn.sin()
    }

    /// Whether a point lies on the round: on its cylinder, between its two
    /// ends, and on the edge's side of its axis, between its two sides.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.start;
        let along = off.dot(self.along);
        let across = off - self.along * along;
        let [first, second] = self.facing;
        let least = self.radius * first.dot(second);
        (across.length() - self.radius).abs() < PLACE
            && (-PLACE..=self.length + PLACE).contains(&along)
            && across.dot(-first) >= least - PLACE
            && across.dot(-second) >= least - PLACE
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// The round bottom, whole: two hollow quarter rounds about the line
/// [`RADIUS`] over the floor's middle line, from the left wall `length` along
/// X, from facing the front wall's way to the floor's, and from the floor's to
/// the back wall's.
fn round_bottom_along(length: f64) -> [Hollow; 2] {
    [[DVec3::Y, DVec3::Z], [DVec3::Z, -DVec3::Y]].map(|facing| Hollow {
        start: DVec3::new(LEFT_WALL_AT, MIDDLE, ROUND_TOP),
        along: DVec3::X,
        length,
        radius: RADIUS,
        facing,
    })
}

/// Holds that every ray aimed from the round's axis out to it, over a grid
/// along it and round it, meets the face numbered `face` first, facing back
/// along the ray, toward the axis, wherever `shows` says the round shows at
/// the point the ray aims at; and that some ray was aimed.
fn met_from_its_axis_where(
    body: &Body,
    face: usize,
    hollow: &Hollow,
    shows: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let mut aimed = 0;
    for tenth in 0..10 {
        let along = (f64::from(tenth) + 0.5) / 10.0;
        for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let facing = hollow.facing_at(share);
            let from = hollow.axis_at(along);
            let aim = from - facing * hollow.radius;
            if !shows(aim) {
                continue;
            }
            aimed += 1;
            let hit = body
                .ray_hit(from, -facing)
                .unwrap_or_else(|| panic!("{what}: a ray from {from} on the axis meets nothing"));
            assert_eq!(
                hit.face, face,
                "{what}: a ray from {from} on the axis toward {aim} meets face {} before the \
                 round",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{what}: where a ray from {from} meets it, the round faces {}, not {facing}, \
                 toward its axis",
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

/// The ranks the faces laid on the list's edges `edges` answer to, named in
/// `order`, from `start`, ascending.
fn ranks_of(start: usize, order: &[usize], edges: &[usize]) -> Vec<usize> {
    let mut ranks: Vec<usize> = edges
        .iter()
        .map(|edge| {
            start
                + order
                    .iter()
                    .position(|named| named == edge)
                    .expect("every edge is named")
        })
        .collect();
    ranks.sort();
    ranks
}

/// Each face of a body and the sides of the faces it meets, one for each edge
/// it has, every face laid on the edges named in `order` read by the edge it
/// was laid on, as the rank the listed order gives that edge: what tells two
/// bodies the same face for face, whatever the order their edges were named
/// in.
fn meetings(body: &Body, start: usize, order: &[usize]) -> BTreeMap<Vec<u32>, Vec<Vec<u32>>> {
    let as_listed = |side: Vec<u32>| -> Vec<u32> {
        let mut read: Vec<u32> = side
            .into_iter()
            .map(|number| {
                let number = number as usize;
                let read = if (start..start + order.len()).contains(&number) {
                    start + order[number - start]
                } else {
                    number
                };
                read as u32
            })
            .collect();
        read.sort();
        read
    };
    let mut met: BTreeMap<Vec<u32>, Vec<Vec<u32>>> = BTreeMap::new();
    for name in body.edge_names() {
        let [first, second] = name.between.map(as_listed);
        met.entry(first.clone()).or_default().push(second.clone());
        met.entry(second).or_default().push(first);
    }
    for sides in met.values_mut() {
        sides.sort();
    }
    met
}

/// What the faces of `body`, laid on the edges named in `order`, meet
/// otherwise than those of `reference`, laid on them in the listed order, each
/// laid face read by the edge it was laid on: one line for each face that
/// does.
fn met_otherwise(start: usize, order: &[usize], body: &Body, reference: &Body) -> Vec<String> {
    let listed_order: Vec<usize> = (0..order.len()).collect();
    let own = meetings(body, start, order);
    let theirs = meetings(reference, start, &listed_order);
    let faces: BTreeSet<&Vec<u32>> = own.keys().chain(theirs.keys()).collect();
    faces
        .into_iter()
        .filter(|face| own.get(*face) != theirs.get(*face))
        .map(|face| {
            format!(
                "face {face:?} meets {:?}, where the listed order lays it meeting {:?}",
                own.get(face),
                theirs.get(face)
            )
        })
        .collect()
}

/// A list of the part's edges, each as the two faces it lies between, and
/// what it is called.
type Listed = Vec<([usize; 2], &'static str)>;

/// What the fillet at [`RADIUS`] on a list laid in each order: the order, and
/// the step's outcome.
type LaidInEachOrder = Vec<(Vec<usize>, Result<Filleted, Declined>)>;

/// The fillet at [`RADIUS`] on `listed`'s edges of `part`, in every order the
/// list can name them: every order laid before any body is read, so that a
/// decline names every order that declines.
fn filleted_in_every_order(part: &Body, listed: &[([usize; 2], &str)]) -> LaidInEachOrder {
    let named: Vec<EdgeName> = listed
        .iter()
        .map(|&([one, other], _)| edge_between(part, one, other))
        .collect();
    every_order(named.len())
        .into_iter()
        .map(|order| {
            let names: Vec<EdgeName> = order.iter().map(|&edge| named[edge].clone()).collect();
            let laid = part.fillet_edges(&names, RADIUS);
            (order, laid)
        })
        .collect()
}

/// What the fillet at [`RADIUS`] on `listed`, named in `order`, on the part
/// `on` is called.
fn called(listed: &[([usize; 2], &str)], order: &[usize], on: &str) -> String {
    let called: Vec<&str> = order.iter().map(|&edge| listed[edge].1).collect();
    format!(
        "the fillet at {RADIUS} on {}, on {on}",
        called.join(", then ")
    )
}

/// Holds that every order of `laid` built a body, naming every order that
/// declined; and hands each order's body back with what it is called, the
/// listed order's first.
fn built_in_every_order<'a>(
    laid: &'a LaidInEachOrder,
    listed: &[([usize; 2], &str)],
    on: &str,
) -> Vec<(&'a [usize], &'a Filleted, String)> {
    let declined: Vec<String> =
        laid.iter()
            .filter_map(|(order, outcome)| {
                outcome.as_ref().err().map(|declined| {
                    format!("{}: declined, {declined:?}", called(listed, order, on))
                })
            })
            .collect();
    let listed_order: Vec<usize> = (0..listed.len()).collect();
    assert!(
        declined.is_empty(),
        "{} is built whichever edge the list names first; declined in {} of {} orders:\n{}",
        called(listed, &listed_order, on),
        declined.len(),
        laid.len(),
        declined.join("\n")
    );
    let mut built: Vec<(&[usize], &Filleted, String)> = laid
        .iter()
        .map(|(order, outcome)| {
            let filleted = outcome.as_ref().expect("declines are held above");
            (order.as_slice(), filleted, called(listed, order, on))
        })
        .collect();
    built.sort_by_key(|(order, _, _)| *order != listed_order.as_slice());
    built
}

/// Holds that each order's body is the one the listed order lays, its faces
/// meeting alike, each laid face read by the edge it was laid on.
fn the_same_body_in_every_order(start: usize, built: &[(&[usize], &Filleted, String)]) {
    let (_, reference, _) = &built[0];
    for (order, filleted, what) in built {
        let otherwise = met_otherwise(start, order, &filleted.body, &reference.body);
        assert!(
            otherwise.is_empty(),
            "{what}: the body is the one the listed order lays, its faces meeting alike; not \
             so:\n{}",
            otherwise.join("\n")
        );
    }
}

/// The faces the fillet laid on the round bottom's pair and both the end's
/// floor edges, as the body numbers them: the round bottom, answering to every
/// number in `bottom`, and each end edge's fill, the front's then the back's.
struct Laid {
    bottom: Vec<usize>,
    fills: [usize; 2],
}

impl Laid {
    /// The fill of the end's floor edge on `side`.
    fn fill(&self, side: Side) -> usize {
        match side {
            Side::Front => self.fills[0],
            Side::Back => self.fills[1],
        }
    }
}

/// Holds that `body`, `part` with the round bottom's pair and both the end's
/// floor edges filleted at [`RADIUS`], their faces numbered as `laid` says,
/// meets each end edge's fill in a valley and the two end edges' fills at the
/// tip: the floor gone; the round bottom drawn on its half cylinder and never
/// past either valley, each end edge's fill on its quarter cylinder, never
/// short of its valley and on its own side of the middle line, each met there
/// from its axis; each valley drawn by both its faces from its shoulder down
/// to the deepest point, the tip's from there up to the tip, no point shared
/// off them, one vertex at each end; the end walls flat, kept above the fills,
/// meeting at the shoulders and the tip along uprights left sharp; the side
/// walls kept above the round bottom, the left wall above its half circle;
/// and every face meeting the faces it bounds, one edge each.
fn meets_each_end_fill_in_a_valley(end: End, part: &Body, body: &Body, laid: &Laid, what: &str) {
    let r = RADIUS;
    let bottom = &laid.bottom;
    assert!(
        !body.has_face(POCKET_FLOOR),
        "{what}: the pocket's floor is gone, wholly under the round bottom and the end's fills, \
         which touch its plane only along the middle line, from the left wall to {}",
        end.deepest()
    );
    one_curved_face_answers_to(body, bottom, &format!("{what}: the round bottom"));

    let [front_half, back_half] = round_bottom_along(SHOULDERS_AT - LEFT_WALL_AT);
    let bottom_drawn: Vec<DVec3> = body.triangles_of(bottom[0]).flatten().collect();
    for &point in &bottom_drawn {
        assert!(
            front_half.holds(point) || back_half.holds(point),
            "{what}: the round bottom is drawn through {point}, off its half cylinder of {r} \
             about the line {r} over the floor's middle line"
        );
        assert!(
            end.past_the_valley(point) <= PLACE,
            "{what}: the round bottom is drawn through {point}, past the valley where an end \
             edge's fill meets it"
        );
    }
    let in_the_air = if end.notched() {
        end.tip.x - 1.0
    } else {
        SHOULDERS_AT
    };
    for half in round_bottom_along(in_the_air - LEFT_WALL_AT) {
        met_from_its_axis_where(
            body,
            bottom[0],
            &half,
            |aim| end.past_the_valley(aim) < -CLEAR_OF_THE_VALLEY,
            &format!("{what}: the round bottom, short of the valleys"),
        );
    }

    let drawn_of =
        |side: Side| -> Vec<DVec3> { body.triangles_of(laid.fill(side)).flatten().collect() };
    for side in [Side::Front, Side::Back] {
        let fill = laid.fill(side);
        let other = laid.fill(side.other());
        let which = format!("the fill of {}", side.end_edge().1);
        let round = end.fill(side);
        assert!(
            !body.is_flat(fill) && body.plane_of(fill).is_none(),
            "{what}: {which}, face {fill}, is curved, and offers no plane to draw on"
        );
        let fill_drawn = drawn_of(side);
        assert!(!fill_drawn.is_empty(), "{what}: {which} is drawn");
        for &point in &fill_drawn {
            assert!(
                round.holds(point),
                "{what}: {which} is drawn through {point}, off its quarter cylinder of {r} about \
                 the line {r} in from its wall and over the floor"
            );
            assert!(
                end.past_the_valley(point) >= -PLACE,
                "{what}: {which} is drawn through {point}, short of the valley where the round \
                 bottom meets it"
            );
            assert!(
                side.across(point) >= -PLACE,
                "{what}: {which} is drawn through {point}, past the middle line, where the other \
                 end edge's fill meets it"
            );
        }
        met_from_its_axis_where(
            body,
            fill,
            &end.fill_seen_from_its_axis(side),
            |aim| {
                end.past_the_valley(aim) > CLEAR_OF_THE_VALLEY
                    && side.across(aim) > CLEAR_OF_THE_VALLEY
            },
            &format!("{what}: {which}, past the valley and short of the middle line"),
        );

        let [shoulder, deepest, tip] = [end.shoulder_top(side), end.deepest(), end.tip_top()];
        for (point, at) in [
            (
                shoulder,
                "where the round bottom leaves the side wall at the shoulder",
            ),
            (deepest, "where the three fills touch the floor's plane"),
        ] {
            for (drawn, of) in [(&bottom_drawn, "the round bottom"), (&fill_drawn, &*which)] {
                assert!(
                    drawn_through(drawn, point),
                    "{what}: {of} is drawn up to {at}, {point}, where the valley ends"
                );
            }
        }
        for (point, at) in [(deepest, "the deepest point"), (tip, "the tip")] {
            assert!(
                drawn_through(&fill_drawn, point),
                "{what}: {which} is drawn up to {at}, {point}, where the curve the two end \
                 edges' fills meet along ends"
            );
        }
        for &point in bottom_drawn
            .iter()
            .filter(|&&point| drawn_through(&fill_drawn, point))
        {
            assert!(
                end.past_the_valley(point).abs() < PLACE,
                "{what}: the round bottom and {which} meet at {point}, off the valley's plane"
            );
        }
        assert_eq!(
            vertices_among(body, &[&[side.wall()], &[side.end_wall()], bottom, &[fill]]),
            1,
            "{what}: the side wall, the end's wall, the round bottom and {which} meet at one \
             vertex, {shoulder}, where the valley begins"
        );
        assert_eq!(
            sides_met_by(body, &[fill]),
            sides_answering(&[&[side.end_wall()], bottom, &[other]]),
            "{what}: {which} meets its wall along its top, the round bottom along the valley and \
             the other end edge's fill at the tip, by one edge each, and no other face"
        );

        let wall = side.end_wall();
        faces_toward(
            body,
            wall,
            -end.outward(side).extend(0.0),
            &format!("{what}: the end's wall over {which}"),
        );
        corners_are(
            body,
            wall,
            &[
                shoulder,
                tip,
                end.tip.extend(HEIGHT),
                end.shoulder(side).extend(HEIGHT),
            ],
            &format!(
                "{what}: the end's wall over {which}, kept from where the fill leaves it up to \
                 the top"
            ),
        );
        assert_eq!(
            sides_met_by(body, &[wall]),
            sides_answering(&[&[TOP], &[side.wall()], &[side.other().end_wall()], &[fill]]),
            "{what}: the end's wall over {which} meets the top, the side wall at the shoulder \
             and the end's other wall at the tip along uprights left sharp, and its fill, by one \
             edge each"
        );
        assert_eq!(
            edges_between(body, side.wall(), wall).len(),
            1,
            "{what}: the upright at the shoulder is one sharp edge, from the top down to {shoulder}"
        );

        let across = side.wall_at();
        corners_are(
            body,
            side.wall(),
            &[
                DVec3::new(LEFT_WALL_AT, across, ROUND_TOP),
                shoulder,
                end.shoulder(side).extend(HEIGHT),
                DVec3::new(LEFT_WALL_AT, across, HEIGHT),
            ],
            &format!(
                "{what}: the side wall, kept from where the round bottom leaves it up to the top"
            ),
        );
        assert_eq!(
            sides_met_by(body, &[side.wall()]),
            sides_answering(&[&[TOP], &[wall], &[POCKET_LEFT], bottom]),
            "{what}: the side wall meets the top, the end's wall and the left wall along its \
             uprights, and the round bottom, by one edge each"
        );
    }

    let [front_drawn, back_drawn] = [Side::Front, Side::Back].map(drawn_of);
    for &point in front_drawn
        .iter()
        .filter(|&&point| drawn_through(&back_drawn, point))
    {
        assert!(
            (point.y - MIDDLE).abs() < PLACE,
            "{what}: the end edges' two fills meet at {point}, off the middle line"
        );
    }
    let fills: [&[usize]; 2] = [&laid.fills[..1], &laid.fills[1..]];
    assert_eq!(
        vertices_among(body, &[bottom, fills[0], fills[1]]),
        1,
        "{what}: the round bottom and the end edges' two fills meet at one vertex, {}",
        end.deepest()
    );
    assert_eq!(
        vertices_among(body, &[&[END_FRONT], &[END_BACK], fills[0], fills[1]]),
        1,
        "{what}: the end's two walls and their fills meet at one vertex, {}",
        end.tip_top()
    );
    assert_eq!(
        edges_between(body, END_FRONT, END_BACK).len(),
        1,
        "{what}: the upright at the tip is one sharp edge, from the top down to {}",
        end.tip_top()
    );

    let half_circle_center = DVec3::new(LEFT_WALL_AT, MIDDLE, ROUND_TOP);
    corners_are_along_an_arc(
        body,
        POCKET_LEFT,
        &[
            DVec3::new(LEFT_WALL_AT, FRONT_WALL_AT, ROUND_TOP),
            DVec3::new(LEFT_WALL_AT, FRONT_WALL_AT + WIDTH, ROUND_TOP),
            DVec3::new(LEFT_WALL_AT, FRONT_WALL_AT + WIDTH, HEIGHT),
            DVec3::new(LEFT_WALL_AT, FRONT_WALL_AT, HEIGHT),
        ],
        |corner| {
            let off = corner - half_circle_center;
            (off.length() - r).abs() < PLACE && off.z <= PLACE
        },
        &format!(
            "{what}: the left wall, kept above the half circle of {r} about {half_circle_center} \
             the round bottom leaves on it"
        ),
    );
    assert_eq!(
        sides_met_by(body, &[POCKET_LEFT]),
        sides_answering(&[&[TOP], &[POCKET_FRONT], &[POCKET_BACK], bottom]),
        "{what}: the left wall meets the top, the front and the back walls along its uprights \
         and the round bottom along the half circle, by one edge each"
    );
    assert_eq!(
        sides_met_by(body, bottom),
        sides_answering(&[
            &[POCKET_FRONT],
            &[POCKET_BACK],
            &[POCKET_LEFT],
            fills[0],
            fills[1]
        ]),
        "{what}: the round bottom meets the front and the back walls along its sides, the left \
         wall along its half circle, and each end edge's fill along its valley, by one edge each"
    );
    assert_eq!(
        sides_met_by(body, &[TOP]),
        sides_met_by(part, &[TOP]),
        "{what}: the block's top meets what it met before the step"
    );
}

/// The round bottom's pair, then the end's front and back floor edges.
fn pair_and_both_end_edges() -> Listed {
    [Side::Front, Side::Back]
        .map(Side::floor_edge)
        .into_iter()
        .chain([Side::Front, Side::Back].map(Side::end_edge))
        .collect()
}

/// Holds, for `end`, that the arithmetic gives the volume the module's
/// documentation writes, the booleans what the arithmetic does, and that the
/// fillet at [`RADIUS`] on the round bottom's pair and both the end's floor
/// edges is laid in every order as [`meets_each_end_fill_in_a_valley`] holds
/// it, the value taken whole, the body exact, closed, uncrossed and drawn,
/// each laid face answering to its edge's rank, nothing laid at a corner, and
/// every order laying the body the listed order lays.
fn meets_each_end_fill_in_a_valley_whatever_the_order(end: End, volume: f64, written: &str) {
    let promised = end.kept_all_round();
    assert!(
        (promised - volume).abs() <= EXACT * volume,
        "the arithmetic gives {promised}, where the documentation writes {written}"
    );
    let by_booleans = end.by_booleans();
    assert!(
        (by_booleans - promised).abs() <= EXACT * promised,
        "the round bottom's prism and the end edges' fill prisms, taken off the tool that cuts \
         {}, leave {by_booleans}, where arithmetic promises {promised}",
        end.called
    );

    let part = end.part();
    for side in [Side::Front, Side::Back] {
        assert!(
            meets_inwards(&part, side.wall(), side.end_wall()),
            "on {}, the upright at the shoulder {} meets inwards, as the floor's two edges \
             ending there do: a pocket's bottom corner",
            end.called,
            end.shoulder(side)
        );
    }
    assert_eq!(
        meets_inwards(&part, END_FRONT, END_BACK),
        !end.notched(),
        "on {}, the upright at the tip meets inwards at a point and outwards at a notch",
        end.called
    );

    let listed = pair_and_both_end_edges();
    let start = part.faces_end();
    let laid = filleted_in_every_order(&part, &listed);
    let built = built_in_every_order(&laid, &listed, end.called);
    let area = round_bottom_along(SHOULDERS_AT - LEFT_WALL_AT)
        .iter()
        .chain(&[end.fill(Side::Front), end.fill(Side::Back)])
        .map(Hollow::area)
        .sum::<f64>();
    for (order, filleted, what) in &built {
        takes(filleted.asked, filleted.taken, RADIUS, what);
        let body = &filleted.body;
        sound_drawn_within(body, promised, DRAWN * reach_of(&part) * area, what);
        let mut kept_and_laid = numbers(&part);
        kept_and_laid.remove(&POCKET_FLOOR);
        kept_and_laid.extend(start..start + order.len());
        assert_eq!(
            numbers(body),
            kept_and_laid,
            "{what}: every face of the pocketed block but the pocket's floor, the round bottom \
             answering to both its edges' ranks, each end edge's fill to its own, and no face \
             laid at a corner"
        );
        assert_eq!(
            body.faces_end(),
            start + order.len(),
            "{what}: the next step numbers its faces past the {} the list gave its edges",
            order.len()
        );
        let faces = Laid {
            bottom: ranks_of(start, order, &[0, 1]),
            fills: [2, 3].map(|edge| ranks_of(start, order, &[edge])[0]),
        };
        meets_each_end_fill_in_a_valley(end, &part, body, &faces, what);
    }
    the_same_body_in_every_order(start, &built);
}

#[test]
fn a_fillet_on_a_pocket_s_round_bottom_and_the_two_floor_edges_of_its_pointed_end_meets_each_end_fill_in_a_valley_and_the_two_meet_in_a_valley_at_the_tip_whatever_the_order()
 {
    let root = 2f64.sqrt();
    meets_each_end_fill_in_a_valley_whatever_the_order(
        End::POINTED,
        5631.0 - 36.0 * root - 207.0 * PI / 2.0 + 27.0 * root * PI / 2.0,
        "5631 − 36√2 − 207π/2 + 27√2π/2",
    );
}

#[test]
fn a_fillet_on_a_pocket_s_round_bottom_and_the_two_floor_edges_of_a_blunter_point_meets_each_end_fill_in_a_valley_and_the_two_meet_in_a_valley_at_the_tip_whatever_the_order()
 {
    let root = 13f64.sqrt();
    meets_each_end_fill_in_a_valley_whatever_the_order(
        End::BLUNTER,
        5634.0 - 12.0 * root - 99.0 * PI + 9.0 * root * PI / 2.0,
        "5634 − 12√13 − 99π + 9√13π/2",
    );
}

#[test]
fn a_fillet_on_a_pocket_s_round_bottom_and_the_two_floor_edges_of_its_notched_end_meets_each_end_fill_in_a_valley_and_mitres_the_two_outside_the_notch_s_upright_whatever_the_order()
 {
    let root = 2f64.sqrt();
    meets_each_end_fill_in_a_valley_whatever_the_order(
        End::NOTCHED,
        5649.0 - 36.0 * root - 153.0 * PI / 2.0 + 27.0 * root * PI / 2.0,
        "5649 − 36√2 − 153π/2 + 27√2π/2",
    );
}

/// The round bottom's pair, then the end's floor edge on `side`.
fn pair_and_end_edge(side: Side) -> Listed {
    [Side::Front, Side::Back]
        .map(Side::floor_edge)
        .into_iter()
        .chain([side.end_edge()])
        .collect()
}

/// The value the fillet on the round bottom's pair and one floor edge of a
/// pointed end takes, as the module's documentation derives it: where that
/// edge's fill, trimmed flush at the tip on the far end wall, and the far
/// floor edge's fill, trimmed flush on it too, first meet on it.
fn where_the_far_fills_meet(end: End) -> f64 {
    let [shoulder, tip] = end.turns();
    end.edge_length() / (1.0 / tip.sin() + 1.0 / shoulder.sin())
}

/// What a pointed end's pocket holds once the round bottom's pair and one of
/// its end's floor edges are filleted at `value`, short of half the width, as
/// the module's documentation derives it: the three edges' length times the
/// round's section; at the shoulder between two of them, less the tangent of
/// half its turn times the cube's share; at the far shoulder and at the tip,
/// where a fill ends flush on the wall left out, half the cotangent of the
/// turn there.
fn kept_with_one_end_edge_at(end: End, value: f64) -> f64 {
    let [shoulder, tip] = end.turns();
    let length = 2.0 * (SHOULDERS_AT - LEFT_WALL_AT) + end.edge_length();
    let corners = -(shoulder / 2.0).tan() + 0.5 / shoulder.tan() + 0.5 / tip.tan();
    LENGTH * BREADTH * HEIGHT - end.area() * (HEIGHT - FLOOR_AT)
        + length * value * value * (1.0 - PI / 4.0)
        + corners * value.powi(3) * (5.0 / 3.0 - PI / 2.0)
}

#[test]
fn a_fillet_on_a_pocket_s_round_bottom_and_one_floor_edge_of_its_pointed_end_stops_where_that_edge_s_fill_would_meet_the_far_floor_edge_s_on_the_other_end_wall_whatever_the_order()
 {
    let [root2, root13] = [2f64.sqrt(), 13f64.sqrt()];
    for (end, value, written, volume) in [
        (End::POINTED, 6.0 - 3.0 * root2, "6 − 3√2", 5255.366772),
        (End::BLUNTER, 16.0 - 4.0 * root13, "16 − 4√13", 5267.298170),
    ] {
        let derived = where_the_far_fills_meet(end);
        assert!(
            (derived - value).abs() < PLACE,
            "on {}: the far fills meet on the far end wall at {derived}, where the documentation \
             writes {written}",
            end.called
        );
        let promised = kept_with_one_end_edge_at(end, value);
        assert!(
            (promised - volume).abs() < 1e-6,
            "on {}: the arithmetic gives {promised}, where the documentation writes {volume}",
            end.called
        );
        let part = end.part();
        let start = part.faces_end();
        for side in [Side::Front, Side::Back] {
            let block = block();
            let prisms: Vec<Body> = [
                floor_edge_prism(Side::Front, value),
                floor_edge_prism(Side::Back, value),
                end.end_prism(
                    side,
                    value,
                    end.tip - end.outward(side) * value,
                    [-15.0, 15.0],
                ),
            ]
            .map(|prism| prism.raised_on(&block))
            .into();
            let by_booleans = cut_with_its_fills(&block, &end.tool(&block), &prisms);
            assert!(
                (by_booleans - promised).abs() <= EXACT * promised,
                "on {}: the three fills' prisms at {value}, {} among them, taken off the tool, \
                 leave {by_booleans}, where arithmetic promises {promised}",
                end.called,
                side.end_edge().1
            );

            let listed = pair_and_end_edge(side);
            let laid = filleted_in_every_order(&part, &listed);
            let built = built_in_every_order(&laid, &listed, end.called);
            for (order, filleted, what) in &built {
                takes(filleted.asked, filleted.taken, value, what);
                let body = &filleted.body;
                let drawn = DRAWN
                    * reach_of(&part)
                    * (2.0 * (SHOULDERS_AT - LEFT_WALL_AT) + end.edge_length())
                    * value
                    * PI
                    / 2.0;
                sound_drawn_within(body, promised, drawn, what);
                assert!(
                    body.has_face(POCKET_FLOOR),
                    "{what}: the pocket's floor is kept between the fills, which never meet \
                     across it short of half its width"
                );
                let mut kept_and_laid = numbers(&part);
                kept_and_laid.extend(start..start + order.len());
                assert_eq!(
                    numbers(body),
                    kept_and_laid,
                    "{what}: every face of the pocketed block, a face for each edge's rank, and \
                     no face laid at a corner"
                );
                let [front, back] = [0, 1].map(|edge| ranks_of(start, order, &[edge])[0]);
                assert!(
                    body.triangles_of(front).collect::<Vec<_>>()
                        != body.triangles_of(back).collect::<Vec<_>>(),
                    "{what}: the pair's two fills are two faces, no round bottom formed"
                );
            }
            the_same_body_in_every_order(start, &built);
        }
    }
}

/// What the notched end's pocket holds once the round bottom's pair and one of
/// its end's floor edges are filleted at [`RADIUS`], as the module's
/// documentation derives it: the three fills' sections along their edges, the
/// rhombus the round bottom's half and the notch edge's fill share at the
/// shoulder counted once, and what of it lies past the plane that trims the
/// notch edge's fill, below the depth where the rhombus reaches it, given
/// back.
fn kept_with_one_notch_edge() -> f64 {
    let r = RADIUS;
    let root = 2f64.sqrt();
    let rho = 3.0 * root - 3.0;
    let deep = (r * r - rho * rho).sqrt();
    let under_the_round = r * r * PI / 4.0 - deep * rho / 2.0 - r * r / 2.0 * (deep / r).asin();
    let under_the_parabola = r * r * (r - deep) - (r.powi(3) - deep.powi(3)) / 3.0;
    let past_the_plane = rho * rho * (r - deep) - 2.0 * rho * under_the_round + under_the_parabola;
    let crossing = 1.5 + root;
    LENGTH * BREADTH * HEIGHT - End::NOTCHED.area() * (HEIGHT - FLOOR_AT)
        + (40.0 + 3.0 * root) * r * r * (1.0 - PI / 4.0)
        - crossing * r.powi(3) * (5.0 / 3.0 - PI / 2.0)
        + crossing * past_the_plane
}

/// The upright at the shoulder on `side`, where the side wall meets the end's
/// wall there, and what it is called.
fn shoulder_upright(side: Side) -> ([usize; 2], &'static str) {
    match side {
        Side::Front => (
            [POCKET_FRONT, END_FRONT],
            "the upright at the front shoulder",
        ),
        Side::Back => ([POCKET_BACK, END_BACK], "the upright at the back shoulder"),
    }
}

/// The upright at the end's tip, where its two walls meet, and what it is
/// called.
const TIP_UPRIGHT: ([usize; 2], &str) = ([END_FRONT, END_BACK], "the upright at the tip");

/// `listed` followed by `others`.
fn and(listed: Listed, others: &[([usize; 2], &'static str)]) -> Listed {
    listed.into_iter().chain(others.iter().copied()).collect()
}

/// The lists of `part` the fillet at [`RADIUS`] declines in some order, each
/// order and its reason, and those it lays: what `on` is called and each list
/// that is not a broken step and its reason, `Unsupported`, in every order.
fn not_declined_in_every_order(part: &Body, on: &str, lists: &[Listed]) -> Vec<String> {
    let mut not_declined = Vec::new();
    for listed in lists {
        for (order, outcome) in filleted_in_every_order(part, listed) {
            let what = called(listed, &order, on);
            match outcome {
                Err(Declined::Unsupported) => {}
                Err(declined) => not_declined.push(format!("{what}: declined, {declined:?}")),
                Ok(laid) => not_declined.push(format!(
                    "{what}: a body of {}, taking {}",
                    laid.body.volume(),
                    laid.taken
                )),
            }
        }
    }
    not_declined
}

#[test]
fn a_fillet_on_a_pocket_s_round_bottom_and_one_floor_edge_of_its_notched_end_alone_or_with_the_upright_at_its_shoulder_is_declined_as_unsupported_whatever_the_order()
 {
    let end = End::NOTCHED;
    let promised = kept_with_one_notch_edge();
    assert!(
        (promised - 5412.104023).abs() < 1e-6,
        "the arithmetic gives {promised}, where the documentation writes 5412.104023"
    );
    let part = end.part();
    let mut lists = Vec::new();
    for side in [Side::Front, Side::Back] {
        let block = block();
        let prisms = vec![
            round_bottom_prism(&block),
            end.end_prism(
                side,
                RADIUS,
                end.tip - end.outward(side) * RADIUS,
                [0.0, 10.0],
            )
            .raised_on(&block),
        ];
        let by_booleans = cut_with_its_fills(&block, &end.tool(&block), &prisms);
        assert!(
            (by_booleans - promised).abs() <= EXACT * promised,
            "the round bottom's prism and the fill prism of {}, raised from the plane of the \
             other notch wall, taken off the tool, leave {by_booleans}, where arithmetic \
             promises {promised}",
            side.end_edge().1
        );
        lists.push(pair_and_end_edge(side));
        lists.push(and(pair_and_end_edge(side), &[shoulder_upright(side)]));
    }
    let not_declined = not_declined_in_every_order(&part, end.called, &lists);
    assert!(
        not_declined.is_empty(),
        "the round bottom's pair with one floor edge of the notched end, alone or with the \
         upright at that edge's shoulder, is a broken step and its reason, Unsupported, whatever \
         the order (#822); not so:\n{}",
        not_declined.join("\n")
    );
}

/// What a pointed end's pocket holds once the round bottom's pair alone is
/// chamfered at [`RADIUS`], as the module's documentation derives it: the
/// floor's two edges' length times the flat's section, and at each shoulder,
/// where the flat ends flush on the end's wall left out, half the cotangent of
/// the turn there times the cube's share.
fn chamfered_pair(end: End) -> f64 {
    let d = RADIUS;
    let [shoulder, _] = end.turns();
    LENGTH * BREADTH * HEIGHT - end.area() * (HEIGHT - FLOOR_AT)
        + 2.0 * (SHOULDERS_AT - LEFT_WALL_AT) * d * d / 2.0
        + 1.0 / shoulder.tan() * d.powi(3) / 3.0
}

/// What the pocket holds once the round bottom's pair and both the end's floor
/// edges are chamfered at [`RADIUS`]: [`End::kept_all_round`] with the flat's
/// section and the cube's share.
fn chamfered_all_round(end: End) -> f64 {
    let d = RADIUS;
    let [shoulder, tip] = end.turns();
    let length = 2.0 * (SHOULDERS_AT - LEFT_WALL_AT) + 2.0 * end.edge_length();
    let tangents = 2.0 * (shoulder / 2.0).tan() + (tip / 2.0).tan();
    LENGTH * BREADTH * HEIGHT - end.area() * (HEIGHT - FLOOR_AT) + length * d * d / 2.0
        - tangents * d.powi(3) / 3.0
}

/// What a pointed end's pocket holds once the round bottom's pair and one of
/// its end's floor edges are chamfered at `value`: [`kept_with_one_end_edge_at`]
/// with the flat's section and the cube's share.
fn chamfered_with_one_end_edge_at(end: End, value: f64) -> f64 {
    let [shoulder, tip] = end.turns();
    let length = 2.0 * (SHOULDERS_AT - LEFT_WALL_AT) + end.edge_length();
    let corners = -(shoulder / 2.0).tan() + 0.5 / shoulder.tan() + 0.5 / tip.tan();
    LENGTH * BREADTH * HEIGHT - end.area() * (HEIGHT - FLOOR_AT)
        + length * value * value / 2.0
        + corners * value.powi(3) / 3.0
}

/// What the notched end's pocket holds once the round bottom's pair and one
/// of its end's floor edges are chamfered at [`RADIUS`]:
/// [`kept_with_one_notch_edge`] with the flat's depth, `δ = t`.
fn chamfered_with_one_notch_edge() -> f64 {
    let d = RADIUS;
    let root = 2f64.sqrt();
    let crossing = 1.5 + root;
    let past = d - (6.0 - 3.0 * root);
    LENGTH * BREADTH * HEIGHT - End::NOTCHED.area() * (HEIGHT - FLOOR_AT)
        + (40.0 + 3.0 * root) * d * d / 2.0
        - crossing * d.powi(3) / 3.0
        + crossing * past.powi(3) / 3.0
}

#[test]
fn a_chamfer_at_3_on_the_round_bottom_s_pair_and_any_floor_edge_of_a_pointed_or_notched_end_keeps_its_body_in_every_order()
 {
    let [root2, root13] = [2f64.sqrt(), 13f64.sqrt()];
    let pair: Listed = [Side::Front, Side::Back].map(Side::floor_edge).into();
    let mut cases: Vec<(End, Listed, f64, f64, [usize; 2])> = Vec::new();
    for (end, short, pair_alone, all_round, one_edge) in [
        (
            End::POINTED,
            6.0 - 3.0 * root2,
            5415.0,
            5415.0 + 9.0 * root2,
            5294.472729,
        ),
        (
            End::BLUNTER,
            16.0 - 4.0 * root13,
            5430.0,
            5430.0 + 3.0 * root13,
            5298.285119,
        ),
        (
            End::NOTCHED,
            RADIUS,
            5505.0,
            5505.0 + 9.0 * root2,
            5496.0 + 9.0 * root2,
        ),
    ] {
        let with_one = if end.notched() {
            chamfered_with_one_notch_edge()
        } else {
            chamfered_with_one_end_edge_at(end, short)
        };
        for (promised, written, called) in [
            (chamfered_pair(end), pair_alone, "the pair alone"),
            (
                chamfered_all_round(end),
                all_round,
                "the pair and both end edges",
            ),
            (with_one, one_edge, "the pair and one end edge"),
        ] {
            assert!(
                (promised - written).abs() < 1e-6,
                "{}, {called}: the arithmetic gives {promised}, where the documentation writes \
                 {written}",
                end.called
            );
        }
        cases.push((end, pair.clone(), RADIUS, chamfered_pair(end), [29, 19]));
        cases.push((
            end,
            pair_and_both_end_edges(),
            RADIUS,
            chamfered_all_round(end),
            [32, 20],
        ));
        let one_edge_counts = if end.notched() { [32, 21] } else { [34, 22] };
        for side in [Side::Front, Side::Back] {
            cases.push((
                end,
                pair_and_end_edge(side),
                short,
                with_one,
                one_edge_counts,
            ));
        }
    }

    let mut not_kept = Vec::new();
    for (end, listed, value, promised, [edges, vertices]) in &cases {
        let part = end.part();
        let start = part.faces_end();
        let named: Vec<EdgeName> = listed
            .iter()
            .map(|&([one, other], _)| edge_between(&part, one, other))
            .collect();
        let laid: Vec<(Vec<usize>, Result<Chamfered, Declined>)> = every_order(named.len())
            .into_iter()
            .map(|order| {
                let names: Vec<EdgeName> = order.iter().map(|&edge| named[edge].clone()).collect();
                let chamfered = part.chamfer_edges(&names, RADIUS);
                (order, chamfered)
            })
            .collect();
        let listed_order: Vec<usize> = (0..named.len()).collect();
        let reference = laid
            .iter()
            .find(|(order, _)| *order == listed_order)
            .and_then(|(_, chamfered)| chamfered.as_ref().ok());
        for (order, chamfered) in &laid {
            let what = called(listed, order, end.called).replace("fillet", "chamfer");
            let chamfered = match chamfered {
                Ok(chamfered) => chamfered,
                Err(declined) => {
                    not_kept.push(format!("{what}: declined, {declined:?}"));
                    continue;
                }
            };
            if (chamfered.asked - RADIUS).abs() >= PLACE || (chamfered.taken - value).abs() >= PLACE
            {
                not_kept.push(format!(
                    "{what}: asked {} and took {}, where {value} is taken",
                    chamfered.asked, chamfered.taken
                ));
                continue;
            }
            let body = &chamfered.body;
            let triangles = body.triangles();
            if !body.is_exact() {
                not_kept.push(format!("{what}: not computed by the exact kernel"));
            } else if let Err(fault) = closed(&triangles) {
                not_kept.push(format!("{what}: not closed, {fault:?}"));
            } else if let Err(fault) = uncrossed(&triangles) {
                not_kept.push(format!("{what}: crossed, {fault:?}"));
            } else if (body.volume() - promised).abs() > EXACT * promised {
                not_kept.push(format!(
                    "{what}: holds {}, where the arithmetic gives {promised}",
                    body.volume()
                ));
            }
            let counted = [body.edge_names().len(), body.vertex_names().len()];
            if counted != [*edges, *vertices] {
                not_kept.push(format!(
                    "{what}: {} edges and {} vertices, where the body pinned has {edges} and \
                     {vertices}",
                    counted[0], counted[1]
                ));
            }
            for several in met_by_several_edges(body) {
                not_kept.push(format!("{what}: {several}, where they meet by one"));
            }
            if listed.len() == 4 {
                for (floor_edge, end_edge) in [(0, 2), (1, 3)] {
                    let [floor_flat, end_flat] =
                        [floor_edge, end_edge].map(|edge| ranks_of(start, order, &[edge])[0]);
                    let met = edges_between(body, floor_flat, end_flat).len();
                    if met != 1 {
                        not_kept.push(format!(
                            "{what}: the flats on {} and {} meet by {met} edges, where the valley \
                             between them is one",
                            listed[floor_edge].1, listed[end_edge].1
                        ));
                    }
                }
            }
            if let Some(reference) = reference {
                for otherwise in met_otherwise(start, order, body, &reference.body) {
                    not_kept.push(format!("{what}: {otherwise}"));
                }
            }
        }
    }
    assert!(
        not_kept.is_empty(),
        "the chamfer at {RADIUS} on the pair, alone or with either or both of the end's floor \
         edges, is built in every order on each end, the value it took on the branch's base, \
         exact, closed and uncrossed, holding what the arithmetic gives, with the edges and \
         vertices it has, every face meeting each other by one edge at most, and the same body \
         the listed order lays; not so:\n{}",
        not_kept.join("\n")
    );
}

/// Every two faces of `body` that meet by more than one edge, each given as
/// every number it answers to: none where every face meets each face it
/// bounds by one edge.
fn met_by_several_edges(body: &Body) -> Vec<String> {
    let mut met: BTreeMap<[Vec<u32>; 2], usize> = BTreeMap::new();
    for name in body.edge_names() {
        let mut sides = name.between;
        sides.sort();
        *met.entry(sides).or_insert(0) += 1;
    }
    met.into_iter()
        .filter(|&(_, count)| count > 1)
        .map(|(sides, count)| format!("faces {sides:?} meet by {count} edges"))
        .collect()
}

/// Holds that `body` has no two faces meeting by more than one edge.
fn every_face_meets_each_by_one_edge(body: &Body, what: &str) {
    let several = met_by_several_edges(body);
    assert!(
        several.is_empty(),
        "{what}: every face meets each face it bounds by one edge; not so:\n{}",
        several.join("\n")
    );
}

/// The round bottom's pair on the pocket the end walls of `end` close.
fn the_pair() -> Listed {
    [Side::Front, Side::Back].map(Side::floor_edge).into()
}

/// What the pocket holds once the round bottom's pair alone is filleted at
/// [`RADIUS`], as the module's documentation derives it: the floor's two
/// edges' length times the round's section, and at each shoulder, where a
/// half of the round bottom ends flush on the end's wall left out, half the
/// cotangent of the turn there times the cube's share.
fn kept_with_the_pair_alone(end: End) -> f64 {
    let r = RADIUS;
    let [shoulder, _] = end.turns();
    LENGTH * BREADTH * HEIGHT - end.area() * (HEIGHT - FLOOR_AT)
        + 2.0 * (SHOULDERS_AT - LEFT_WALL_AT) * r * r * (1.0 - PI / 4.0)
        + r.powi(3) * (5.0 / 3.0 - PI / 2.0) / shoulder.tan()
}

#[test]
fn a_fillet_on_a_pocket_s_round_bottom_alone_ends_each_half_of_it_flush_on_its_wall_at_a_pointed_or_notched_end_whatever_the_order()
 {
    let r = RADIUS;
    for (end, volume, written) in [
        (End::POINTED, 5631.0 - 207.0 * PI / 2.0, "5631 − 207π/2"),
        (End::BLUNTER, 5634.0 - 99.0 * PI, "5634 − 99π"),
        (End::NOTCHED, 5649.0 - 153.0 * PI / 2.0, "5649 − 153π/2"),
    ] {
        let promised = kept_with_the_pair_alone(end);
        assert!(
            (promised - volume).abs() <= EXACT * volume,
            "on {}: the arithmetic gives {promised}, where the documentation writes {written}",
            end.called
        );
        let block = block();
        let tool = end.tool(&block);
        let by_booleans = cut_with_its_fills(&block, &tool, &[round_bottom_prism(&tool)]);
        assert!(
            (by_booleans - promised).abs() <= EXACT * promised,
            "on {}: the round bottom's prism, taken off the tool, leaves {by_booleans}, where \
             arithmetic promises {promised}",
            end.called
        );

        let part = end.part();
        let start = part.faces_end();
        let listed = the_pair();
        let laid = filleted_in_every_order(&part, &listed);
        let built = built_in_every_order(&laid, &listed, end.called);
        let [front_half, back_half] = end.round_bottom_running_to_its_walls();
        let area = front_half.area() + back_half.area();
        for (order, filleted, what) in &built {
            takes(filleted.asked, filleted.taken, RADIUS, what);
            let body = &filleted.body;
            sound_drawn_within(body, promised, DRAWN * reach_of(&part) * area, what);
            let mut kept_and_laid = numbers(&part);
            kept_and_laid.remove(&POCKET_FLOOR);
            kept_and_laid.extend(start..start + order.len());
            assert_eq!(
                numbers(body),
                kept_and_laid,
                "{what}: every face of the pocketed block but its floor, and the round bottom \
                 answering to both its edges' ranks"
            );
            assert_eq!(body.faces_end(), start + order.len(), "{what}");
            let bottom = ranks_of(start, order, &[0, 1]);
            one_curved_face_answers_to(body, &bottom, &format!("{what}: the round bottom"));
            for point in body.triangles_of(bottom[0]).flatten() {
                let side = if point.y <= MIDDLE {
                    Side::Front
                } else {
                    Side::Back
                };
                assert!(
                    (front_half.holds(point) || back_half.holds(point))
                        && end.reach(side, point) <= r + PLACE,
                    "{what}: the round bottom is drawn through {point}, off its half cylinder or \
                     past the end's wall on its side"
                );
            }
            for side in [Side::Front, Side::Back] {
                let wall = side.end_wall();
                let toward = side.way_seen(-DVec2::Y).extend(0.0);
                corners_are_along_an_arc(
                    body,
                    wall,
                    &[
                        end.shoulder_top(side),
                        end.shoulder(side).extend(HEIGHT),
                        end.tip.extend(HEIGHT),
                        end.tip.extend(FLOOR_AT),
                    ],
                    |corner| {
                        let off = corner - DVec3::new(corner.x, MIDDLE, ROUND_TOP);
                        (off.length() - r).abs() < PLACE
                            && off.dot(toward) >= -PLACE
                            && off.z <= PLACE
                    },
                    &format!(
                        "{what}: the end's wall on the {side:?} side, kept above where the round \
                         bottom's half ends on it, from the shoulder down to the tip's foot"
                    ),
                );
            }
            assert_eq!(
                sides_met_by(body, &bottom),
                sides_answering(&[
                    &[POCKET_FRONT],
                    &[POCKET_BACK],
                    &[POCKET_LEFT],
                    &[END_FRONT],
                    &[END_BACK]
                ]),
                "{what}: the round bottom meets the side walls, the left wall and each of the \
                 end's walls, by one edge each"
            );
            assert_eq!(
                vertices_among(body, &[&[END_FRONT], &[END_BACK], &bottom]),
                1,
                "{what}: the end's two walls and the round bottom meet at one vertex, the tip's \
                 foot, {}",
                end.tip.extend(FLOOR_AT)
            );
            assert_eq!(
                edges_between(body, END_FRONT, END_BACK).len(),
                1,
                "{what}: the upright at the tip is one sharp edge, the floor's height up"
            );
            every_face_meets_each_by_one_edge(body, what);
        }
        the_same_body_in_every_order(start, &built);
    }
}

/// What a pointed end's pocket holds once the round bottom's pair and the
/// upright at one shoulder are filleted at [`RADIUS`], as the module's
/// documentation derives it: [`kept_with_the_pair_alone`], with the upright's
/// fill, its section seen from above times the pocket's depth, less what the
/// round bottom holds of it.
fn kept_with_a_shoulder_upright(end: End) -> f64 {
    let r = RADIUS;
    let [turn, _] = end.turns();
    let half = turn / 2.0;
    kept_with_the_pair_alone(end) + (HEIGHT - FLOOR_AT) * r * r * (half.tan() - half)
        - r.powi(3) * (half.tan() - half * (1.0 + 1.0 / turn.sin()) + 2.0 / 3.0 - turn.cos() / 6.0)
}

/// The prism the fill of the upright at the shoulder on `side` adds, raised
/// from under the floor to over the tool's top for `part` to number its faces
/// after its own: what lies outside the circle of [`RADIUS`] about the
/// vertical through the crossing of the axes, between where it touches the
/// side wall and where it touches the end's wall, out into the matter past
/// both.
fn shoulder_upright_prism(end: End, side: Side, part: &Body) -> Body {
    let [turn, _] = end.turns();
    let center = end.crossing().truncate();
    let on_the_side = DVec2::new(center.x, side.wall_at());
    let on_the_end = center + end.outward(side) * RADIUS;
    let out = side.way_seen(-DVec2::Y);
    let corners = match side {
        Side::Front => vec![
            on_the_side,
            on_the_side + out,
            DVec2::new(SHOULDERS_AT + 1.0, side.wall_at()) + out,
            on_the_end + end.outward(side),
            on_the_end,
        ],
        Side::Back => vec![
            on_the_end,
            on_the_end + end.outward(side),
            DVec2::new(SHOULDERS_AT + 1.0, side.wall_at()) + out,
            on_the_side + out,
            on_the_side,
        ],
    };
    arc_closed_prism(part, corners, center, -turn)
}

/// The prism an outline raises from under the floor to over the tool's top,
/// for `part` to number its faces after its own, its last run an arc about
/// `center` turning by `turn` back to its first corner.
fn arc_closed_prism(part: &Body, corners: Vec<DVec2>, center: DVec2, turn: f64) -> Body {
    let mut runs = vec![Run::Straight; corners.len()];
    let last = corners.len() - 1;
    runs[last] = Run::Round { center, turn };
    raised_on(
        part,
        Contour { corners, runs },
        FLOOR_AT - 1.0,
        HEIGHT - FLOOR_AT + 3.0,
    )
}

#[test]
fn a_fillet_on_a_pocket_s_round_bottom_and_the_upright_at_one_shoulder_of_its_pointed_end_meets_that_upright_s_fill_in_a_valley_whatever_the_order()
 {
    let r = RADIUS;
    let [root2, root13] = [2f64.sqrt(), 13f64.sqrt()];
    for (end, volume, written) in [
        (
            End::POINTED,
            5586.0 + 117.0 * root2 / 4.0 - 855.0 * PI / 8.0 + 27.0 * root2 * PI / 8.0,
            "5586 + 117√2/4 − 855π/8 + 27√2π/8",
        ),
        (
            End::BLUNTER,
            5598.0 - 99.0 * PI
                + 126.0 * root13 / 13.0
                + (9.0 * root13 - 27.0) * 1.5f64.atan() / 2.0,
            "5598 − 99π + 126√13/13 + (9√13 − 27)·atan(3/2)/2",
        ),
    ] {
        let promised = kept_with_a_shoulder_upright(end);
        assert!(
            (promised - volume).abs() <= EXACT * volume,
            "on {}: the arithmetic gives {promised}, where the documentation writes {written}",
            end.called
        );
        let part = end.part();
        let start = part.faces_end();
        let center = end.crossing();
        let [front_half, back_half] = end.round_bottom_running_to_its_walls();
        for side in [Side::Front, Side::Back] {
            let block = block();
            let tool = end.tool(&block);
            let round_bottom = round_bottom_prism(&tool);
            let upright = shoulder_upright_prism(end, side, &round_bottom);
            let by_booleans = cut_with_its_fills(&block, &tool, &[round_bottom, upright]);
            assert!(
                (by_booleans - promised).abs() <= EXACT * promised,
                "on {}: the round bottom's prism and the {side:?} shoulder upright's, taken off \
                 the tool, leave {by_booleans}, where arithmetic promises {promised}",
                end.called
            );

            let listed = and(the_pair(), &[shoulder_upright(side)]);
            let laid = filleted_in_every_order(&part, &listed);
            let built = built_in_every_order(&laid, &listed, end.called);
            let round = Hollow {
                start: DVec3::new(center.x, MIDDLE, FLOOR_AT),
                along: DVec3::Z,
                length: HEIGHT - FLOOR_AT,
                radius: r,
                facing: [
                    side.way_seen(DVec2::Y).extend(0.0),
                    -end.outward(side).extend(0.0),
                ],
            };
            let area = front_half.area() + back_half.area() + round.area();
            for (order, filleted, what) in &built {
                takes(filleted.asked, filleted.taken, RADIUS, what);
                let body = &filleted.body;
                sound_drawn_within(body, promised, DRAWN * reach_of(&part) * area, what);
                let mut kept_and_laid = numbers(&part);
                kept_and_laid.remove(&POCKET_FLOOR);
                kept_and_laid.extend(start..start + order.len());
                assert_eq!(
                    numbers(body),
                    kept_and_laid,
                    "{what}: every face of the pocketed block but its floor, the round bottom \
                     answering to both its edges' ranks, the upright's fill to its own, and no \
                     face laid at a corner"
                );
                assert_eq!(body.faces_end(), start + order.len(), "{what}");
                let bottom = ranks_of(start, order, &[0, 1]);
                let fill = ranks_of(start, order, &[2])[0];
                one_curved_face_answers_to(body, &bottom, &format!("{what}: the round bottom"));
                let bottom_drawn: Vec<DVec3> = body.triangles_of(bottom[0]).flatten().collect();
                for &point in &bottom_drawn {
                    assert!(
                        front_half.holds(point) || back_half.holds(point),
                        "{what}: the round bottom is drawn through {point}, off its half \
                         cylinder"
                    );
                }
                let fill_drawn: Vec<DVec3> = body.triangles_of(fill).flatten().collect();
                assert!(
                    !fill_drawn.is_empty() && !body.is_flat(fill),
                    "{what}: the upright's fill is a curved face, drawn"
                );
                for &point in &fill_drawn {
                    assert!(
                        round.holds(point),
                        "{what}: the upright's fill is drawn through {point}, off its round of \
                         {r} about the vertical through {}, between the side wall and the end's \
                         wall",
                        center.truncate()
                    );
                }
                for &point in fill_drawn
                    .iter()
                    .filter(|&&point| drawn_through(&bottom_drawn, point))
                {
                    assert!(
                        ((point.x - center.x) - (ROUND_TOP - point.z)).abs() < PLACE,
                        "{what}: the round bottom and the upright's fill meet at {point}, off \
                         the plane x − {} = 17 − z their two cylinders meet on",
                        center.x
                    );
                }
                every_face_meets_each_by_one_edge(body, what);
            }
            the_same_body_in_every_order(start, &built);
        }
    }
}

/// What a pointed end's pocket holds once the round bottom's pair, both the
/// end's floor edges and the upright at the tip are filleted at [`RADIUS`], as
/// the module's documentation derives it: [`End::kept_all_round`], with the
/// upright's fill over the round bottom's top, its section seen from above
/// times the depth left, and under it the cell past both end edges' planes
/// through the crossing of the axes filled but for the ball of [`RADIUS`]
/// about that crossing.
fn kept_with_the_tip_upright(end: End) -> f64 {
    let r = RADIUS;
    let [_, turn] = end.turns();
    let half = turn / 2.0;
    end.kept_all_round()
        + (HEIGHT - ROUND_TOP) * r * r * (half.tan() - half)
        + 2.0 * r.powi(3) / 3.0 * half.tan()
        - r.powi(3) / 3.0 * turn
}

/// The prism the fill of the upright at a point's tip adds, for `part` to
/// number its faces after its own: what lies outside the circle of [`RADIUS`]
/// about the vertical through the crossing of the axes, between where it
/// touches the end's two walls, out into the matter past the tip.
fn tip_upright_prism(end: End, part: &Body) -> Body {
    let [_, turn] = end.turns();
    let center = end.crossing().truncate();
    let [front, back] = [Side::Front, Side::Back].map(|side| end.outward(side));
    let [on_the_front, on_the_back] = [front, back].map(|out| center + out * RADIUS);
    arc_closed_prism(
        part,
        vec![
            on_the_front,
            on_the_front + front,
            end.tip + (end.tip - center).normalize(),
            on_the_back + back,
            on_the_back,
        ],
        center,
        -turn,
    )
}

/// The box the corner a point's tip closes fills, for `part` to number its
/// faces after its own: past the two planes through the crossing of the axes
/// square to the end's floor edges, from under the floor up to the round
/// bottom's top, out into the matter past the tip.
fn tip_cell_prism(end: End, part: &Body) -> Body {
    let center = end.crossing().truncate();
    let [front, back] = [Side::Front, Side::Back].map(|side| end.outward(side));
    raised_on(
        part,
        Contour::straight(vec![
            center,
            center + front * 4.0,
            center + DVec2::X * 6.0,
            center + back * 4.0,
        ]),
        FLOOR_AT - 1.0,
        RADIUS + 1.0,
    )
}

#[test]
fn a_fillet_on_a_pocket_s_round_bottom_the_two_floor_edges_of_its_pointed_end_and_the_upright_at_its_tip_closes_the_tip_with_a_hollow_piece_of_sphere_whatever_the_order()
 {
    let r = RADIUS;
    let [root2, root13] = [2f64.sqrt(), 13f64.sqrt()];
    for (end, volume, written) in [
        (
            End::POINTED,
            5676.0 - 36.0 * root2 - 459.0 * PI / 4.0 + 27.0 * root2 * PI / 2.0,
            "5676 − 36√2 − 459π/4 + 27√2π/2",
        ),
        (
            End::BLUNTER,
            5664.0 - 12.0 * root13 - 243.0 * PI / 2.0
                + 9.0 * root13 * PI / 2.0
                + 45.0 * 1.5f64.atan(),
            "5664 − 12√13 − 243π/2 + 9√13π/2 + 45·atan(3/2)",
        ),
    ] {
        let promised = kept_with_the_tip_upright(end);
        assert!(
            (promised - volume).abs() <= EXACT * volume,
            "on {}: the arithmetic gives {promised}, where the documentation writes {written}",
            end.called
        );
        let [_, turn] = end.turns();
        let block = block();
        let tool = end.tool(&block);
        let upright = tip_upright_prism(end, &round_bottom_prism(&tool));
        let mut prisms = end.prisms(&upright);
        prisms.push(tip_cell_prism(end, &upright));
        prisms.push(upright);
        let filled = cut_with_its_fills(&block, &tool, &prisms);
        let by_booleans = filled - r.powi(3) / 3.0 * turn;
        assert!(
            (by_booleans - promised).abs() <= EXACT * promised,
            "on {}: the fills' prisms and the tip's cell, taken off the tool, leave {filled}, and \
             less the ball's share of the cell {by_booleans}, where arithmetic promises \
             {promised}",
            end.called
        );

        let part = end.part();
        let start = part.faces_end();
        let center = end.crossing();
        let listed = and(pair_and_both_end_edges(), &[TIP_UPRIGHT]);
        let laid = filleted_in_every_order(&part, &listed);
        let built = built_in_every_order(&laid, &listed, end.called);
        let ways = [Side::Front, Side::Back].map(|side| end.way(side).extend(0.0));
        let round = Hollow {
            start: center,
            along: DVec3::Z,
            length: HEIGHT - ROUND_TOP,
            radius: r,
            facing: [Side::Front, Side::Back].map(|side| -end.outward(side).extend(0.0)),
        };
        let area = round_bottom_along(SHOULDERS_AT - LEFT_WALL_AT)
            .iter()
            .chain(&[end.fill(Side::Front), end.fill(Side::Back), round])
            .map(Hollow::area)
            .sum::<f64>()
            + 4.0 * PI * r * r;
        for (order, filleted, what) in &built {
            takes(filleted.asked, filleted.taken, RADIUS, what);
            let body = &filleted.body;
            sound_drawn_within(body, promised, DRAWN * reach_of(&part) * area, what);
            let patch = start + order.len();
            let mut kept_and_laid = numbers(&part);
            kept_and_laid.remove(&POCKET_FLOOR);
            kept_and_laid.extend(start..=patch);
            assert_eq!(
                numbers(body),
                kept_and_laid,
                "{what}: every face of the pocketed block but its floor, a face for each edge's \
                 rank, the round bottom answering to both its edges', and the tip's patch \
                 numbered after them, {patch}"
            );
            assert_eq!(
                body.faces_end(),
                patch + 1,
                "{what}: the next step numbers past the patch"
            );
            let bottom = ranks_of(start, order, &[0, 1]);
            let [front_fill, back_fill, upright] =
                [2, 3, 4].map(|edge| ranks_of(start, order, &[edge])[0]);
            one_curved_face_answers_to(body, &bottom, &format!("{what}: the round bottom"));

            let patch_drawn: Vec<DVec3> = body.triangles_of(patch).flatten().collect();
            assert!(
                !patch_drawn.is_empty() && !body.is_flat(patch),
                "{what}: the tip's patch is a curved face, drawn"
            );
            for &point in &patch_drawn {
                let off = point - center;
                assert!(
                    (off.length() - r).abs() < PLACE
                        && ways.iter().all(|way| off.dot(*way) >= -PLACE)
                        && off.z <= PLACE,
                    "{what}: the tip's patch is drawn through {point}, off the sphere of {r} \
                     about {center} in the cell past both end edges' planes through it and \
                     under it"
                );
            }
            let [front, back] = [Side::Front, Side::Back].map(|side| end.outward(side).extend(0.0));
            for share in [0.25, 0.5, 0.75] {
                let across = (front * share + back * (1.0 - share)).normalize();
                for drop in [0.2, 0.6, 1.0, 1.4] {
                    let way = across * f64::cos(drop) - DVec3::Z * f64::sin(drop);
                    let hit = body.ray_hit(center, way).unwrap_or_else(|| {
                        panic!("{what}: a ray from {center} along {way} meets nothing")
                    });
                    assert!(
                        hit.face == patch && hit.normal.distance(-way) < PLACE,
                        "{what}: a ray from {center} along {way} meets face {} facing {}, where \
                         it meets the tip's patch first, facing back to its centre",
                        hit.face,
                        hit.normal
                    );
                }
            }
            assert_eq!(
                sides_met_by(body, &[patch]),
                sides_answering(&[&[front_fill], &[back_fill], &[upright]]),
                "{what}: the tip's patch meets the end edges' two fills and the upright's, by one \
                 edge each, and no other face"
            );
            let upright_drawn: Vec<DVec3> = body.triangles_of(upright).flatten().collect();
            assert!(
                !upright_drawn.is_empty(),
                "{what}: the upright's fill is drawn"
            );
            for &point in &upright_drawn {
                assert!(
                    round.holds(point),
                    "{what}: the upright's fill is drawn through {point}, off its round of {r} \
                     about the vertical through {} between the end's walls, over z = 17",
                    center.truncate()
                );
            }
            for (fill, way) in [front_fill, back_fill].into_iter().zip(ways) {
                for point in body.triangles_of(fill).flatten() {
                    assert!(
                        (point - center).dot(way) <= PLACE,
                        "{what}: the end edge's fill, face {fill}, is drawn through {point}, past \
                         the plane through {center} square to its edge, where the patch takes over"
                    );
                }
            }
            assert!(
                edges_between(body, front_fill, back_fill).is_empty(),
                "{what}: the end edges' two fills no longer meet, the patch between them"
            );
            every_face_meets_each_by_one_edge(body, what);
        }
        the_same_body_in_every_order(start, &built);
    }
}

#[test]
fn a_fillet_on_a_pocket_s_round_bottom_both_floor_edges_of_its_end_and_an_upright_at_a_point_s_shoulder_or_at_a_notch_s_tip_is_declined_as_unsupported_whatever_the_order()
 {
    for end in [End::POINTED, End::BLUNTER] {
        let lists: Vec<Listed> = [Side::Front, Side::Back]
            .map(|side| and(pair_and_both_end_edges(), &[shoulder_upright(side)]))
            .into();
        let not_declined = not_declined_in_every_order(&end.part(), end.called, &lists);
        assert!(
            not_declined.is_empty(),
            "on {}: the pair, both the point's floor edges and the upright at either shoulder are \
             a broken step and its reason, Unsupported, whatever the order (#822); not so:\n{}",
            end.called,
            not_declined.join("\n")
        );
    }
    let end = End::NOTCHED;
    let lists = [and(pair_and_both_end_edges(), &[TIP_UPRIGHT])];
    let not_declined = not_declined_in_every_order(&end.part(), end.called, &lists);
    assert!(
        not_declined.is_empty(),
        "on {}: the pair, both the notch's floor edges and the upright at its tip are a broken \
         step and its reason, Unsupported, whatever the order (#822); not so:\n{}",
        end.called,
        not_declined.join("\n")
    );
}

/// The numbers of a stepped pocket's walls past its front one: the front
/// half's end, the step, the back half's end, back and left.
const STEPPED_FRONT_END: usize = 9;
const STEP: usize = 10;
const STEPPED_BACK_END: usize = 11;
const STEPPED_BACK: usize = 12;
const STEPPED_LEFT: usize = 13;

/// A pocket 6 wide whose right end is a step: one half of it, either side of
/// the middle line, runs on to x = 25 and the other stops at x = 22. Its
/// outline is (5, 2), the front half's end at y = 2 and y = 5, the back
/// half's at y = 5 and y = 8, (5, 8): its walls front, the front half's end,
/// the step, the back half's end, back and left, eight to thirteen. The
/// floor's front and back edges end on two walls that share no edge.
#[derive(Clone, Copy, Debug)]
struct Stepped {
    front_to: f64,
    back_to: f64,
    called: &'static str,
}

impl Stepped {
    /// The front half running on to x = 25, past the back half's end.
    const FRONT_RUNS_ON: Stepped = Stepped {
        front_to: SHOULDERS_AT,
        back_to: 22.0,
        called: "the pocket 6 wide whose front half runs on to x = 25, its back half stopping \
                 at x = 22",
    };

    /// The same mirrored across the middle line: the back half running on.
    const BACK_RUNS_ON: Stepped = Stepped {
        front_to: 22.0,
        back_to: SHOULDERS_AT,
        called: "the pocket 6 wide whose back half runs on to x = 25, its front half stopping \
                 at x = 22",
    };

    /// Where the half on `side` ends along X.
    fn reach(self, side: Side) -> f64 {
        match side {
            Side::Front => self.front_to,
            Side::Back => self.back_to,
        }
    }

    /// The wall the half on `side` ends on.
    fn end_wall(self, side: Side) -> usize {
        match side {
            Side::Front => STEPPED_FRONT_END,
            Side::Back => STEPPED_BACK_END,
        }
    }

    /// The pocket's outline seen from above.
    fn outline(self) -> Contour {
        let [front, back] = [self.front_to, self.back_to];
        Contour::straight(vec![
            DVec2::new(LEFT_WALL_AT, FRONT_WALL_AT),
            DVec2::new(front, FRONT_WALL_AT),
            DVec2::new(front, MIDDLE),
            DVec2::new(back, MIDDLE),
            DVec2::new(back, FRONT_WALL_AT + WIDTH),
            DVec2::new(LEFT_WALL_AT, FRONT_WALL_AT + WIDTH),
        ])
    }

    /// The tool that cuts the pocket out of `block`.
    fn tool(self, block: &Body) -> Body {
        raised_on(block, self.outline(), FLOOR_AT, HEIGHT - FLOOR_AT + 1.0)
    }

    /// The block with this pocket cut into its top, as [`End::part`] cuts it.
    fn part(self) -> Body {
        let block = block();
        block
            .difference(&self.tool(&block))
            .expect("the pocket is cut")
    }

    /// What it holds once the floor's front and back edges are filleted at
    /// [`RADIUS`], as the module's documentation derives it: the block less
    /// the pocket, plus each edge's length times the round's section, each
    /// fill trimmed flush on a wall square to it and the two meeting along
    /// the middle line where both run.
    fn kept(self) -> f64 {
        let r = RADIUS;
        let [front, back] = [self.front_to, self.back_to].map(|to| to - LEFT_WALL_AT);
        LENGTH * BREADTH * HEIGHT - (front + back) * WIDTH / 2.0 * (HEIGHT - FLOOR_AT)
            + (front + back) * r * r * (1.0 - PI / 4.0)
    }
}

#[test]
fn a_fillet_on_the_round_bottom_of_a_pocket_whose_floor_edges_end_on_the_two_walls_of_a_stepped_end_is_built_in_either_order()
 {
    let r = RADIUS;
    for stepped in [Stepped::FRONT_RUNS_ON, Stepped::BACK_RUNS_ON] {
        let promised = stepped.kept();
        let written = 5667.0 - 333.0 * PI / 4.0;
        assert!(
            (promised - written).abs() <= EXACT * written,
            "on {}: the arithmetic gives {promised}, where the documentation writes \
             5667 − 333π/4",
            stepped.called
        );
        let block = block();
        let tool = stepped.tool(&block);
        let by_booleans = cut_with_its_fills(&block, &tool, &[round_bottom_prism(&tool)]);
        assert!(
            (by_booleans - promised).abs() <= EXACT * promised,
            "on {}: the round bottom's prism, taken off the tool, leaves {by_booleans}, where \
             arithmetic promises {promised}",
            stepped.called
        );

        let part = stepped.part();
        let start = part.faces_end();
        let listed: Listed = vec![
            (
                [POCKET_FLOOR, POCKET_FRONT],
                "the floor's edge on the front wall",
            ),
            (
                [POCKET_FLOOR, STEPPED_BACK],
                "the floor's edge on the back wall",
            ),
        ];
        let laid = filleted_in_every_order(&part, &listed);
        let built = built_in_every_order(&laid, &listed, stepped.called);
        let quarters = [Side::Front, Side::Back].map(|side| {
            let [front_half, back_half] = round_bottom_along(stepped.reach(side) - LEFT_WALL_AT);
            match side {
                Side::Front => front_half,
                Side::Back => back_half,
            }
        });
        let area = quarters.iter().map(Hollow::area).sum::<f64>();
        let [short, long] = [22.0, SHOULDERS_AT];
        let step_facing = if stepped.front_to > stepped.back_to {
            -DVec3::Y
        } else {
            DVec3::Y
        };
        for (order, filleted, what) in &built {
            takes(filleted.asked, filleted.taken, RADIUS, what);
            let body = &filleted.body;
            sound_drawn_within(body, promised, DRAWN * reach_of(&part) * area, what);
            let mut kept_and_laid = numbers(&part);
            kept_and_laid.remove(&POCKET_FLOOR);
            kept_and_laid.extend(start..start + order.len());
            assert_eq!(
                numbers(body),
                kept_and_laid,
                "{what}: every face of the pocketed block but its floor, and the round bottom \
                 answering to both its edges' ranks"
            );
            assert_eq!(body.faces_end(), start + order.len(), "{what}");
            let bottom = ranks_of(start, order, &[0, 1]);
            one_curved_face_answers_to(body, &bottom, &format!("{what}: the round bottom"));
            for point in body.triangles_of(bottom[0]).flatten() {
                assert!(
                    quarters.iter().any(|quarter| quarter.holds(point)),
                    "{what}: the round bottom is drawn through {point}, off its half cylinder \
                     about the line {r} over the middle line, each half from the left wall to \
                     its own end"
                );
            }
            for half in round_bottom_along(short - 1.0 - LEFT_WALL_AT) {
                met_from_its_axis_where(
                    body,
                    bottom[0],
                    &half,
                    |_| true,
                    &format!("{what}: the round bottom"),
                );
            }
            for side in [Side::Front, Side::Back] {
                let wall = stepped.end_wall(side);
                let x = stepped.reach(side);
                let across = side.wall_at();
                let toward = DVec3::Y * (across - MIDDLE).signum();
                faces_toward(
                    body,
                    wall,
                    -DVec3::X,
                    &format!("{what}: the {side:?} half's end wall"),
                );
                corners_are_along_an_arc(
                    body,
                    wall,
                    &[
                        DVec3::new(x, across, ROUND_TOP),
                        DVec3::new(x, MIDDLE, FLOOR_AT),
                        DVec3::new(x, MIDDLE, HEIGHT),
                        DVec3::new(x, across, HEIGHT),
                    ],
                    |corner| {
                        let off = corner - DVec3::new(x, MIDDLE, ROUND_TOP);
                        (off.length() - r).abs() < PLACE
                            && off.dot(toward) >= -PLACE
                            && off.z <= PLACE
                    },
                    &format!(
                        "{what}: the {side:?} half's end wall, kept above the quarter circle its \
                         half of the round bottom ends on"
                    ),
                );
            }
            faces_toward(body, STEP, step_facing, &format!("{what}: the step"));
            corners_are(
                body,
                STEP,
                &[
                    DVec3::new(short, MIDDLE, FLOOR_AT),
                    DVec3::new(long, MIDDLE, FLOOR_AT),
                    DVec3::new(long, MIDDLE, HEIGHT),
                    DVec3::new(short, MIDDLE, HEIGHT),
                ],
                &format!(
                    "{what}: the step, kept whole, the longer half's round meeting it along its \
                     foot, where it touches the floor's plane"
                ),
            );
            assert_eq!(
                sides_met_by(body, &bottom),
                sides_answering(&[
                    &[POCKET_FRONT],
                    &[STEPPED_FRONT_END],
                    &[STEP],
                    &[STEPPED_BACK_END],
                    &[STEPPED_BACK],
                    &[STEPPED_LEFT]
                ]),
                "{what}: the round bottom meets the side walls, both halves' end walls, the step \
                 and the left wall, by one edge each"
            );
            assert_eq!(
                sides_met_by(body, &[STEP]),
                sides_answering(&[&[TOP], &[STEPPED_FRONT_END], &[STEPPED_BACK_END], &bottom]),
                "{what}: the step meets the top, both halves' end walls along the uprights left \
                 sharp, and the round bottom along its foot, by one edge each"
            );
            assert_eq!(
                sides_met_by(body, &[STEPPED_LEFT]),
                sides_answering(&[&[TOP], &[POCKET_FRONT], &[STEPPED_BACK], &bottom]),
                "{what}: the left wall meets the top, the side walls and the round bottom along \
                 its half circle, by one edge each"
            );
            for wall in [STEPPED_FRONT_END, STEPPED_BACK_END] {
                assert_eq!(
                    vertices_among(body, &[&bottom, &[STEP], &[wall]]),
                    1,
                    "{what}: the round bottom, the step and face {wall} meet at one vertex, the \
                     upright's foot"
                );
            }
            every_face_meets_each_by_one_edge(body, what);
        }
        the_same_body_in_every_order(start, &built);
    }
}
