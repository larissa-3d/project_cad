//! A ball and a slab whose face touches it at one point off its poles —
//! where the plane is square to the ball's radius and no pole of its
//! sphere's chart stands — are joined and cut as the slab a hair clear of it
//! is: the plane is read as touching the sphere at that point, as a plane
//! resting on a pole is, rather than as a circle of radius nought, which no
//! face is built along.
//!
//! Closes #939.
//! - A block whose side touches a ball at a point off its poles, joined and
//!   cut: built, sound, of the right volume, as a hair apart gives —
//!   `a_ball_and_a_slab_touching_it_off_its_poles_are_joined_and_cut_as_the_slab_a_hair_clear_of_it_is`
//!
//! The touch is built as well where it meets an edge: the block's own, along
//! which the ball's great circle on the block's floor touches it too; the
//! rim of a half ball, or its dome a little way from it. And from inside:
//! a ball hollowed out of a block whose side it touches.
//!
//! Beneath the body, the relation of a plane and a sphere reads that touch
//! within half the tolerance, at the point, in either order, misses the
//! sphere twice the tolerance beyond and cuts its small circle twice within
//! (`brep/relation/tests/round.rs`); the exact kernel's join and cuts hold
//! without the drawing (`brep/combine/tests/round.rs`); and the drawing lays
//! on the plane the samples of the sphere's face it added, and no other
//! (`brep/tessellation/sampling/tests/touching.rs`).
//!
//! Just past the tolerance, a hair inside, the plane cuts the sphere along a
//! small circle of radius √(2r·δ), δ how far inside it stands: about 4e-4 on
//! the ball of 5 at δ = 1.5e-8. Two bodies lay that circle across an edge.
//! The block from x = 5, 20 across, whose floor z = 0 holds the ball's
//! centre and whose side touches the ball where the floor does: the floor
//! halves the circle, which crosses the edge of the block's side. And the
//! half ball under z = 0, against a slab whose face touches its dome 1e-5 or
//! 3e-5 rad under its rim — 5e-5 or 1.5e-4 from it, so near that the rim
//! crosses the plane on either side of the touch, and the circle crosses the
//! rim of the sphere's face. Either builds as the touch a hair further out
//! does.
//!
//! Closes #960.
//! - The two bodies above, at every offset from 5 − 2·eps to 5 + 2·eps,
//!   joined and cut: built, sound, of the right volume, drawn uncrossed —
//!   the block,
//!   `a_block_a_hair_inside_a_ball_whose_floor_halves_their_small_circle_is_joined_and_cut_as_the_block_touching_it_is`;
//!   the half ball,
//!   `a_slab_a_hair_inside_a_half_ball_just_under_its_rim_is_joined_and_cut_as_the_slab_touching_it_is`
//! - #939's, #938's and #805's tests still pass — no test: they are the tests
//!   closing #939 above and beneath the body, `through_a_pole.rs`'s for #938,
//!   and those `tests.rs`, the relation's files and
//!   `tests/a_chamfer_and_a_fillet_fill_a_concave_edge_up_to_a_curved_end.rs`
//!   name for #805, run as they stand
//!
//! Read as: "every offset", each quarter of the tolerance from 5 − 2·eps to
//! 5 + 2·eps — ±2·eps, ±eps, ±eps/2 and the touch itself among them, 5 − eps
//! too, whose gap rounding sets at −1.50000004e-8 against the block's eps of
//! 1.5e-8, just past the band the touch is read in — a thousandth of the
//! tolerance either side of that band's two edges, and 1 1/16, 1 11/16 and
//! 1 13/16 of it inside, between the quarters, where a sweep by sixteenths
//! found the half ball declined as well. eps is the tolerance the boolean of
//! the two decides at, 1.5e-8 for the block, about 2e-8 for the slab; the
//! offset is the block's side x = offset, or the slab's face that far from
//! the half ball's centre towards the touch, at either angle. "Joined and
//! cut": the union and both differences. "Built, sound, drawn uncrossed":
//! what `built_whole` holds — built by the exact kernel, listed sound, its
//! triangles closed, uncrossed and enclosing the volume within what the
//! drawing promises, every corner within the tolerance of its face's
//! surface. "Of the right volume": the volume the arithmetic gives the
//! touch, within a billionth of it, the ratio of the kernel's tolerance to
//! its reach. The cap a plane δ inside takes, πδ²(15 − δ)/3, is under 3e-14
//! at 2·eps, seven orders below that, so a body that cuts the small circle
//! exactly and one that reads it out as the touch meet it alike.
//!
//! Beyond the criteria, a pin on what #960 changed in the refinement on the
//! sphere's own `(θ, φ)`, where a triangle facing the centre is now cut as
//! one sagging is: a boundary side there bows off the chart's straight line
//! between its ends, and a corner of a fraction of a degree, or a curve
//! within half the drawing's tolerance of another, could leave a triangle
//! inside that bow which no cut turns round. A ball cut by two flats whose
//! circles meet at 45° of latitude, running east and west there, crossing at
//! half a degree down to a twentieth or apart by half the drawing's
//! tolerance down to a hundredth of it, laid out by `pierced.rs` and by
//! `charted.rs`, is built and drawn whole as before #960 —
//! `a_ball_cut_by_two_flats_whose_circles_cross_at_a_fraction_of_a_degree_or_pass_within_half_the_drawing_s_tolerance_is_built_and_drawn_whole`.
//! What the two caps share, under 1.2e-8 at half a degree, is left out of
//! the volume: under a tenth of the billionth of it the volume is held to.
//!
//! The same flats turned a quarter about the ball's axis, their normals in
//! the plane of its meridian of −Z rather than of +X, were drawn crossed,
//! dozens to hundreds of triangles facing the centre, where the meridians of
//! +X and −X draw them clean: caps of 30° and 50° — flats 5·cos 30° and
//! 5·cos 50° from the centre — meeting at 60° of latitude, and caps of 45°
//! and 45° meeting at 45°, their circles crossing at 0.1° and at 0.05°. Each
//! step was accepted all the same, since a face drawn whole is all it asks.
//!
//! Closes #1110.
//! - The four placements above, the flats cut in either order: built, sound,
//!   drawn uncrossed, of the right volume (the caps' shared sliver is under
//!   the tolerance) —
//!   `a_ball_cut_by_two_flats_whose_circles_cross_at_a_tenth_of_a_degree_on_its_meridian_of_minus_z_is_drawn_uncrossed_whichever_is_cut_first`
//! - `touching_a_ball.rs`'s
//!   `a_ball_cut_by_two_flats_whose_circles_cross_at_a_fraction_of_a_degree_or_pass_within_half_the_drawing_s_tolerance_is_built_and_drawn_whole`
//!   (#960) keeps passing — the test of that name above, run as it stands
//!
//! Read as: "built, sound, drawn uncrossed, of the right volume", what
//! `built_whole` holds, as for #960's flats; "the right volume", the ball
//! less both caps whole, the sliver the two share left out of it as #960's
//! test leaves it: some 1e-11 for the caps of 30° and 50° crossing at a
//! tenth of a degree, four orders under the billionth of the volume it is
//! held to.
//!
//! Beyond the criteria, the review's placements: when the face is drawn
//! crossed hangs on how the first circle's samples stand against the grid's
//! angles beside the plane its cap tilts in. The same four, either flat cut
//! first, on the mirror meridian of +Z and on the meridians 10° and 45° from
//! +X, whose plane holds no angle of the grid, are built and drawn whole as
//! on −Z —
//! `a_ball_cut_by_two_flats_whose_circles_cross_at_a_tenth_of_a_degree_on_its_meridian_of_plus_z_or_between_the_grid_s_angles_is_drawn_uncrossed_whichever_is_cut_first`.
//!
//! One of them, older than #1110 and declined under #960's cut order as
//! well, was declined `Unverified` at the second cut, the kernel's own
//! listing failing before anything was drawn: on +Z, the caps of 45°
//! crossing at 0.05°, the flat about the equator cut first. The other order
//! built it sound, closed and uncrossed. #1110 held it declined, never
//! accepted crossed; it is held built now, the same body either way, as
//! every other placement is.
//!
//! Closes #1117.
//! - That placement, both orders: built, sound, drawn uncrossed, of the
//!   right volume — the case
//!   `a_ball_cut_by_two_flats_whose_circles_cross_at_a_tenth_of_a_degree_on_its_meridian_of_plus_z_or_between_the_grid_s_angles_is_drawn_uncrossed_whichever_is_cut_first`
//!   (#1110's branch, `touching_a_ball.rs`) leaves aside holds — that test,
//!   the case no longer left aside
//! - #1110's and #960's tests keep passing — the tests closing them above,
//!   run as they stand:
//!   `a_ball_cut_by_two_flats_whose_circles_cross_at_a_tenth_of_a_degree_on_its_meridian_of_minus_z_is_drawn_uncrossed_whichever_is_cut_first`,
//!   `a_ball_cut_by_two_flats_whose_circles_cross_at_a_fraction_of_a_degree_or_pass_within_half_the_drawing_s_tolerance_is_built_and_drawn_whole`,
//!   `a_block_a_hair_inside_a_ball_whose_floor_halves_their_small_circle_is_joined_and_cut_as_the_block_touching_it_is`,
//!   `a_slab_a_hair_inside_a_half_ball_just_under_its_rim_is_joined_and_cut_as_the_slab_touching_it_is`;
//!   and the one above, holding every other placement it held
//!
//! Read as: "built, sound, drawn uncrossed, of the right volume", what
//! `built_whole` holds, as for #1110's placements; "the right volume", the
//! ball less both caps of 45° whole, the sliver they share at 0.05° left out
//! of it as #1110's test leaves it. "Both orders" give the same body as
//! #822 has it: each is held to that one volume, by the same checks.
//!
//! Beyond the criteria, the review's case: the place the parity is read
//! from, where a trace runs most steeply across the meridian, is never on a
//! slit, an edge the face runs both ways with the face on either side of
//! it, which runs across the meridian squarely along a latitude and bounds
//! nothing. The ball a half band touches along half its equator, its face
//! slit there (5585606550), cut by a flat whose cap takes in the pole
//! towards −Y or +Y, joined then cut or cut then joined, the band turned
//! either way, holds the ball and the band less the cap —
//! `a_ball_a_half_band_touches_along_half_its_equator_cut_by_a_flat_about_either_pole_holds_the_ball_and_the_band_less_the_cap_whichever_comes_first`.
//! It is held built, listed sound, of the arithmetic's volume and drawn
//! whole, as the other balls of this file are.
//!
//! Until #1140 it was held drawn closed but not uncrossed: wherever the ball
//! was cut as well, a triangle of the sphere with two corners on the half
//! of the equator the band's inner wall touches, its third a hair off it,
//! crossed a triangle of the wall on the same two corners, in all sixteen
//! placements, before #1117 as after it — the family
//! `tessellation/sampling/cone.rs` names, a chord of the round along a
//! circle another surface only touches. They are drawn uncrossed since
//! #1112's strip between a touched equator and the next row inside it
//! (`tessellation/outline/touched.rs`).
//!
//! Closes #1140.
//! - #1117's half-band test holds `uncrossed` too, all 16 placements —
//!   `a_ball_a_half_band_touches_along_half_its_equator_cut_by_a_flat_about_either_pole_holds_the_ball_and_the_band_less_the_cap_whichever_comes_first`
//! - #644's and #1117's tests keep their bodies — no test: they are
//!   `tests/the_largest_failure_families.rs`'s for #644, and for #1117 the
//!   tests closing it above and those `brep/domain/tests/round.rs` holds
//!   the poles' reading to, run as they stand
//!
//! Read as: "holds `uncrossed` too", the test goes through `built_whole`
//! as the other balls of this file do, which holds as well every corner of
//! its triangles on its face's surface and the volume they enclose within
//! what the drawing promises — both held already where the triangles
//! crossed.
//!
//! Beyond the criteria, the same ball and band cut about both poles by the
//! half-band test's flats, the zone between the caps a whole turn, which
//! `pierced.rs` lays as a band —
//! `a_ball_a_half_band_touches_along_half_its_equator_cut_about_both_poles_is_built_whole_whichever_comes_first`
//! — and that zone cut short of a turn by a third flat 2 from the centre
//! along (0.1, 0.05, ±1), which `charted.rs` lays —
//! `a_ball_a_half_band_touches_along_half_its_equator_cut_about_both_poles_and_short_of_a_turn_is_built_whole_whichever_comes_first`.
//! Both are drawn along the touch by that strip too. The third flat's cap
//! meets both of the others, and takes a part of the band where the band
//! lies on its side: the volume held is the arithmetic's by slices, each a
//! disc of the ball or of the band clipped by straight lines, exactly, then
//! summed by Simpson's rule between the heights where a slice changes how
//! it is read.
//!
//! And a case another review found: a shaft of radius 4.9 along Z, from
//! z = −10 to the centre of a ball of 5 or 2 past it, less the half ball
//! turned half a turn about an axis through that centre at a slant, held
//! 166.2 and 224.9 about (1, 0, 1)/√2 where its own triangles enclosed 689.5
//! and 748.2, and is built and listed sound holding what the slices give
//! once the poles are read as #1117 reads them —
//! `a_shaft_less_a_half_ball_turned_at_a_slant_about_its_centre_holds_what_the_slices_give`.
//! The other slants and the shaft's own axis held it already.

use super::*;
use crate::soundness::listed;

/// Directions from the centre of [`ball_about_y`] to points of its sphere
/// off its poles, at y = ±5: two on its equator, one 30° from it towards
/// one pole, one 60° towards the other.
fn off_the_poles() -> [DVec3; 4] {
    let at = |latitude: f64, longitude: f64| {
        let (up, out) = latitude.to_radians().sin_cos();
        let (z, x) = longitude.to_radians().sin_cos();
        DVec3::new(out * x, up, out * z)
    };
    [
        at(0.0, 0.0),
        at(0.0, 225.0),
        at(30.0, 0.0),
        at(-60.0, 110.0),
    ]
}

/// What [`slab_along`] holds: 40 by 40, 10 thick.
const SLAB: f64 = 40.0 * 40.0 * 10.0;

/// The tolerance the boolean of `one` and `other` decides at.
fn eps_of(one: &Body, other: &Body) -> f64 {
    let [one, other] = [one, other].map(|body| body.brep().expect("an exact body").scale());
    one.joined(other).eps()
}

/// Holds that `result` is built by the exact kernel, listed sound, holds
/// `kept`, and that its triangles are closed, uncrossed and enclose that
/// within what the drawing promises over its curved faces, `curved` in all,
/// and the rounding of a sum of that much matter; and that every corner of a
/// face's triangles stands within the tolerance of the face's own surface,
/// the most a sample of a sphere laid on a plane touching it moves.
fn built_whole(result: Result<Body, Declined>, kept: f64, curved: f64, what: &str) {
    let body = result.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    let exact = body.brep().expect("the exact kernel computed the step");
    assert_eq!(
        listed(&exact.listing(), exact.scale().reach()),
        Ok(()),
        "{what}: listed sound"
    );
    let cut = exact.triangles_by_face(super::super::drawn::tolerance(exact));
    let within = exact.scale().eps() + 1e-12 * exact.scale().reach();
    for (triangle, face) in cut.triangles.iter().zip(&cut.faces) {
        let surface = exact.surface(exact.face(*face).surface);
        for corner in triangle {
            assert!(
                surface.distance(*corner).abs() <= within,
                "{what}: a corner {corner} of face {face:?} stands {} off its surface",
                surface.distance(*corner)
            );
        }
    }
    assert!(
        (body.volume() - kept).abs() <= 1e-9 * kept,
        "{what}: holds {}, where the arithmetic gives {kept}",
        body.volume()
    );
    let triangles = body.triangles();
    assert_eq!(crate::soundness::closed(&triangles), Ok(()), "{what}");
    assert_eq!(crate::soundness::uncrossed(&triangles), Ok(()), "{what}");
    let (low, high) = body.bounds().expect("a body has a box");
    let reach = crate::brep::Scale::of(low.abs().max(high.abs()).max_element()).reach();
    let promised = super::super::drawn::DRAWN * reach * curved + 1e-9 * kept;
    let enclosed = crate::soundness::enclosed(&triangles);
    assert!(
        (enclosed - kept).abs() <= promised,
        "{what}: its triangles enclose {enclosed}, where it holds {kept}, more than the \
         {promised} the drawing's tolerance gives"
    );
}

/// The slab twice the tolerance clear of the ball, at a point off its poles,
/// then within half the tolerance of touching it there: the union holds
/// both, the ball less the slab is the whole ball, the slab less the ball
/// the whole slab — each built, listed sound and drawn whole, as the slab
/// clear of it gives them.
#[test]
fn a_ball_and_a_slab_touching_it_off_its_poles_are_joined_and_cut_as_the_slab_a_hair_clear_of_it_is()
 {
    let ball = ball_about_y();
    let sphere = 4.0 * PI * 25.0;
    for towards in off_the_poles() {
        let eps = eps_of(&ball, &slab_along(towards, 5.0));
        for offset in [5.0 + 2.0 * eps, 5.0 + eps / 2.0, 5.0, 5.0 - eps / 2.0] {
            let slab = slab_along(towards, offset);
            let what = |how: &str| {
                format!("{how}, the slab's face {offset} from the centre towards {towards}")
            };

            built_whole(
                ball.union(&slab),
                BALL_OF_5 + SLAB,
                sphere,
                &what("the ball joined to the slab"),
            );
            built_whole(
                ball.difference(&slab),
                BALL_OF_5,
                sphere,
                &what("the ball cut by the slab"),
            );
            built_whole(
                slab.difference(&ball),
                SLAB,
                0.0,
                &what("the slab cut by the ball"),
            );
        }
    }
}

/// The block from x = `low` to 15 above the ball's equatorial plane, z = 0,
/// 20 across: its floor holds the ball's centre, and where its side touches
/// the ball, at x = 5, so does its edge along the floor, which the ball's
/// great circle on the floor touches there too.
fn block_from(low: f64) -> Body {
    boxed(DVec3::new(low, -10.0, 0.0), DVec3::new(15.0, 10.0, 10.0))
}

/// The block's edge twice the tolerance clear of the ball, then within half
/// the tolerance of touching it where the block's side and floor meet: the
/// union holds both, the ball less the block is the whole ball, the block
/// less the ball the whole block, as the block clear of it gives them.
#[test]
fn a_block_whose_edge_touches_a_ball_where_its_side_and_floor_do_is_joined_and_cut_as_the_block_a_hair_clear_of_it_is()
 {
    let ball = ball_about_y();
    let sphere = 4.0 * PI * 25.0;
    let eps = eps_of(&ball, &block_from(5.0));
    for low in [5.0 + 2.0 * eps, 5.0 + eps / 2.0, 5.0, 5.0 - eps / 2.0] {
        let block = block_from(low);
        let holds = (15.0 - low) * 200.0;
        let what = |how: &str| format!("{how}, the block from x = {low}");

        built_whole(
            ball.union(&block),
            BALL_OF_5 + holds,
            sphere,
            &what("the ball joined to the block"),
        );
        built_whole(
            ball.difference(&block),
            BALL_OF_5,
            sphere,
            &what("the ball cut by the block"),
        );
        built_whole(
            block.difference(&ball),
            holds,
            0.0,
            &what("the block cut by the ball"),
        );
    }
}

/// The block from x = −10 to `high`, 20 across: from `high` = 5 on, it holds
/// the whole ball, whose sphere touches its side x = `high` from inside.
fn block_to(high: f64) -> Body {
    boxed(
        DVec3::new(-10.0, -10.0, -10.0),
        DVec3::new(high, 10.0, 10.0),
    )
}

/// The ball twice the tolerance inside the block's side, then within half
/// the tolerance of touching it from inside: the block less the ball is
/// hollow, the hollow touching its side at a point, and the block joined to
/// the ball is the block, as the ball a hair inside it gives them.
#[test]
fn a_ball_touching_the_side_of_a_block_from_inside_hollows_it_and_joins_it_as_the_ball_a_hair_inside_it_does()
 {
    let ball = ball_about_y();
    let sphere = 4.0 * PI * 25.0;
    let eps = eps_of(&block_to(5.0), &ball);
    for high in [5.0 + 2.0 * eps, 5.0 + eps / 2.0, 5.0, 5.0 - eps / 2.0] {
        let block = block_to(high);
        let holds = (high + 10.0) * 400.0;
        let what = |how: &str| format!("{how}, the block's side at x = {high}");

        built_whole(
            block.difference(&ball),
            holds - BALL_OF_5,
            sphere,
            &what("the block hollowed by the ball"),
        );
        built_whole(
            block.union(&ball),
            holds,
            0.0,
            &what("the block joined to the ball"),
        );
    }
}

/// The half ball under z = 0, its rim the great circle there, and a slab
/// twice the tolerance clear of its dome, then within half the tolerance of
/// touching it: at a point of its rim, a thousandth of a radian below it, or
/// half a radian. The union holds both, the half ball less the slab is the
/// whole half ball, the slab less the half ball the whole slab, as the slab
/// clear of it gives them.
#[test]
fn a_half_ball_touched_on_its_rim_or_on_its_dome_is_joined_and_cut_as_the_slab_a_hair_clear_of_it_is()
 {
    let half = ball_turned_about_y(PI);
    let (_, high) = half.bounds().expect("a body has a box");
    assert!(
        high.z.abs() <= 1e-9,
        "the half ball lies under z = 0, up to {high}"
    );
    let dome = 2.0 * PI * 25.0;
    for below in [0.0f64, 1e-3, 0.5] {
        let towards = DVec3::new(below.cos(), 0.0, -below.sin());
        let eps = eps_of(&half, &slab_along(towards, 5.0));
        for offset in [5.0 + 2.0 * eps, 5.0 + eps / 2.0, 5.0, 5.0 - eps / 2.0] {
            let slab = slab_along(towards, offset);
            let what = |how: &str| {
                format!("{how}, the slab's face {offset} from the centre, {below} below the rim")
            };

            built_whole(
                half.union(&slab),
                BALL_OF_5 / 2.0 + SLAB,
                dome,
                &what("the half ball joined to the slab"),
            );
            built_whole(
                half.difference(&slab),
                BALL_OF_5 / 2.0,
                dome,
                &what("the half ball cut by the slab"),
            );
            built_whole(
                slab.difference(&half),
                SLAB,
                0.0,
                &what("the slab cut by the half ball"),
            );
        }
    }
}

/// The offsets across the band a plane is read as touching the ball of 5
/// in, at the tolerance `eps`: each quarter of it from 5 − 2·eps to
/// 5 + 2·eps, a thousandth of it either side of 5 − eps and 5 + eps, and the
/// three sixteenths inside, between the quarters, where a sweep by
/// sixteenths found the half ball declined.
fn across_the_touch(eps: f64) -> impl Iterator<Item = f64> {
    (-8..=8)
        .map(move |quarter| 5.0 + f64::from(quarter) * eps / 4.0)
        .chain([-1.001, -0.999, 0.999, 1.001].map(move |edge| 5.0 + edge * eps))
        .chain([-29.0, -27.0, -17.0].map(move |sixteenths| 5.0 + sixteenths * eps / 16.0))
}

/// The block's side from twice the tolerance inside the ball to twice clear
/// of it, where the block's floor holds the ball's centre and halves the
/// small circle the side cuts a hair inside: the union holds both, the ball
/// less the block is the whole ball, the block less the ball the whole
/// block, as the block touching it gives them.
#[test]
fn a_block_a_hair_inside_a_ball_whose_floor_halves_their_small_circle_is_joined_and_cut_as_the_block_touching_it_is()
 {
    let ball = ball_about_y();
    let sphere = 4.0 * PI * 25.0;
    let eps = eps_of(&ball, &block_from(5.0));
    for low in across_the_touch(eps) {
        let block = block_from(low);
        let holds = (15.0 - low) * 200.0;
        let what = |how: &str| {
            format!(
                "{how}, the block from x = {low}, {} of the tolerance off the touch",
                (low - 5.0) / eps
            )
        };

        built_whole(
            ball.union(&block),
            BALL_OF_5 + holds,
            sphere,
            &what("the ball joined to the block"),
        );
        built_whole(
            ball.difference(&block),
            BALL_OF_5,
            sphere,
            &what("the ball cut by the block"),
        );
        built_whole(
            block.difference(&ball),
            holds,
            0.0,
            &what("the block cut by the ball"),
        );
    }
}

/// The slab's face from twice the tolerance inside the half ball to twice
/// clear of it, towards a point of its dome 1e-5 or 3e-5 rad under its rim,
/// which the small circle the face cuts a hair inside crosses: the union
/// holds both, the half ball less the slab is the whole half ball, the slab
/// less the half ball the whole slab, as the slab touching it gives them.
#[test]
fn a_slab_a_hair_inside_a_half_ball_just_under_its_rim_is_joined_and_cut_as_the_slab_touching_it_is()
 {
    let half = ball_turned_about_y(PI);
    let dome = 2.0 * PI * 25.0;
    for below in [1e-5f64, 3e-5] {
        let towards = DVec3::new(below.cos(), 0.0, -below.sin());
        let eps = eps_of(&half, &slab_along(towards, 5.0));
        for offset in across_the_touch(eps) {
            let slab = slab_along(towards, offset);
            let what = |how: &str| {
                format!(
                    "{how}, the slab's face {offset} from the centre, {} of the tolerance off \
                     the touch, {below} below the rim",
                    (offset - 5.0) / eps
                )
            };

            built_whole(
                half.union(&slab),
                BALL_OF_5 / 2.0 + SLAB,
                dome,
                &what("the half ball joined to the slab"),
            );
            built_whole(
                half.difference(&slab),
                BALL_OF_5 / 2.0,
                dome,
                &what("the half ball cut by the slab"),
            );
            built_whole(
                slab.difference(&half),
                SLAB,
                0.0,
                &what("the slab cut by the half ball"),
            );
        }
    }
}

/// The two directions, in the plane of the meridian of +X, of two flats
/// `offsets` from the centre of [`ball_about_y`] whose caps' centres stand
/// `apart` on the sphere, the place between their circles at `latitude`.
fn two_flats_about(offsets: [f64; 2], apart: f64, latitude: f64) -> [DVec3; 2] {
    let [first, second] = offsets.map(|offset| (offset / 5.0).acos());
    let overlap = first + second - apart;
    let along = |at: f64| DVec3::new(at.cos(), at.sin(), 0.0);
    [
        along(latitude + first - overlap / 2.0),
        along(latitude - second + overlap / 2.0),
    ]
}

/// How far apart the centres of two caps reaching `offsets` from the centre
/// of the ball of 5 stand when their circles cross at `angle` on the sphere
/// outside both: the law of cosines on the triangle of a crossing and the two
/// centres, whose angle at the crossing is π − `angle`.
fn apart_crossing_at(offsets: [f64; 2], angle: f64) -> f64 {
    let [first, second] = offsets.map(|offset| (offset / 5.0).acos());
    (first.cos() * second.cos() - first.sin() * second.sin() * angle.cos()).acos()
}

/// The ball cut by two flats whose circles meet at 45° of latitude, running
/// east and west there on the sphere's own `(θ, φ)`: crossing at half a
/// degree down to a twentieth, or a gap of half the drawing's tolerance down
/// to a hundredth of it apart. Flats 2 and 4 from the centre leave the south
/// pole on the sphere left, which `pierced.rs` lays out; flats 1.25 from it
/// take a pole each, and the sphere left is `charted.rs`'s where they cross
/// and `pierced.rs`'s band where they do not. Each is built, listed sound,
/// drawn whole and closed, uncrossed, as the refinement before #960 drew it.
#[test]
fn a_ball_cut_by_two_flats_whose_circles_cross_at_a_fraction_of_a_degree_or_pass_within_half_the_drawing_s_tolerance_is_built_and_drawn_whole()
 {
    let latitude = 45f64.to_radians();
    for offsets in [[2.0, 4.0], [1.25, 1.25]] {
        let crossing = [0.5f64, 0.2, 0.1, 0.05].map(|degrees| {
            (
                format!("crossing at {degrees}°"),
                apart_crossing_at(offsets, degrees.to_radians()),
            )
        });
        let reach = offsets.map(|offset| (offset / 5.0).acos());
        let gaps = [5e-4f64, 1e-4, 1e-5]
            .map(|gap| (format!("{gap} apart"), reach[0] + reach[1] + gap / 5.0));
        for (how, apart) in crossing.into_iter().chain(gaps) {
            let [one, other] = two_flats_about(offsets, apart, latitude);
            let cut = ball_about_y()
                .difference(&slab_along(one, offsets[0]))
                .and_then(|cut| cut.difference(&slab_along(other, offsets[1])));
            let [first, second] = offsets.map(|offset| 5.0 - offset);
            built_whole(
                cut,
                BALL_OF_5 - cap_of_5(first) - cap_of_5(second),
                10.0 * PI * (10.0 - first - second),
                &format!(
                    "the ball cut by two flats {offsets:?} from its centre, their circles {how}"
                ),
            );
        }
    }
}

/// The four placements of #1110 — caps of 30° and 50° meeting at 60° of
/// latitude, or of 45° each meeting at 45°, their circles crossing at a tenth
/// of a degree or a twentieth — with [`two_flats_about`]'s directions, in the
/// plane of the meridian of +X, `turned` about Y into that of the meridian
/// `named`. Cut by either flat first, each is built, listed sound, drawn
/// whole and closed, uncrossed, holding the ball less both caps.
fn cut_by_two_flats_on_the_meridian(turned: impl Fn(DVec3) -> DVec3, named: &str) {
    for (caps, latitude) in [([30f64, 50.0], 60f64), ([45.0, 45.0], 45.0)] {
        let offsets = caps.map(|cap| 5.0 * cap.to_radians().cos());
        let [first, second] = offsets.map(|offset| 5.0 - offset);
        for degrees in [0.1f64, 0.05] {
            let apart = apart_crossing_at(offsets, degrees.to_radians());
            let flats = two_flats_about(offsets, apart, latitude.to_radians()).map(&turned);
            for [one, other] in [[0, 1], [1, 0]] {
                let cut = ball_about_y()
                    .difference(&slab_along(flats[one], offsets[one]))
                    .and_then(|cut| cut.difference(&slab_along(flats[other], offsets[other])));
                let what = format!(
                    "the ball cut by the flat of the cap of {}° about {:.1}° of latitude, then \
                     of {}° about {:.1}°, their circles crossing at {degrees}° at {latitude}° \
                     of latitude on the meridian of {named}",
                    caps[one],
                    flats[one].y.asin().to_degrees(),
                    caps[other],
                    flats[other].y.asin().to_degrees()
                );
                built_whole(
                    cut,
                    BALL_OF_5 - cap_of_5(first) - cap_of_5(second),
                    10.0 * PI * (10.0 - first - second),
                    &what,
                );
            }
        }
    }
}

/// The ball cut by two flats whose circles meet in the plane of its
/// meridian of −Z: caps of 30° and 50° meeting at 60° of latitude, or of 45°
/// each meeting at 45°, their circles crossing at a tenth of a degree or a
/// twentieth, the first cap's centre under half a millionth of a radian off
/// the pole it takes in. Cut by either flat first, each is built, listed
/// sound, drawn whole and closed, uncrossed, as on the meridians of +X and
/// −X.
#[test]
fn a_ball_cut_by_two_flats_whose_circles_cross_at_a_tenth_of_a_degree_on_its_meridian_of_minus_z_is_drawn_uncrossed_whichever_is_cut_first()
 {
    cut_by_two_flats_on_the_meridian(|way| DVec3::new(0.0, way.y, -way.x), "−Z");
}

/// The same placements on the mirror meridian, +Z, whose plane holds the
/// grid angles −Z's does, and on the meridians 10° and 45° from +X towards
/// −Z, as the ball's chart turns, whose plane — the one the first cap tilts
/// in, a hair off the pole — holds none of the grid's angles, a step of
/// some 1.5° here: the first circle's samples stand otherwise against the
/// angle the face is cut open at. Each is built, listed sound, drawn whole
/// and closed, uncrossed, as on the meridian of −Z — #1117's among them, on
/// +Z the caps of 45° crossing at 0.05°, the flat about the equator cut
/// first, which #1117 found declined `Unverified` at its second cut.
#[test]
fn a_ball_cut_by_two_flats_whose_circles_cross_at_a_tenth_of_a_degree_on_its_meridian_of_plus_z_or_between_the_grid_s_angles_is_drawn_uncrossed_whichever_is_cut_first()
 {
    cut_by_two_flats_on_the_meridian(|way| DVec3::new(0.0, way.y, way.x), "+Z");
    for longitude in [10f64, 45.0] {
        let (across, along) = longitude.to_radians().sin_cos();
        cut_by_two_flats_on_the_meridian(
            |way| DVec3::new(way.x * along, way.y, -way.x * across),
            &format!("{longitude}° from +X towards −Z"),
        );
    }
}

/// The band from radius 5 to 6, from y = −0.5 to 0.5, turned half a turn
/// about Y `way`: its inner wall touches [`ball_about_y`] along half its
/// equator, the ball's face slit there (5585606550).
fn half_band_about_y(way: f64) -> Body {
    let corners = Contour::rectangle(DVec2::new(5.0, -0.5), DVec2::new(6.0, 0.5)).corners;
    let triangles = fan(&corners);
    let profile = straight(&corners, &triangles);
    Body::default()
        .tool_turned(&profile, FLAT, &about_v(&profile, way * PI))
        .expect("the exact kernel turns the band")
}

/// The ball joined to a half band its equator touches, the ball's face slit
/// along that half, and cut by a flat 4 from its centre whose cap takes in
/// the pole towards −Y or +Y — its normal (0.3, ±1, 0.18) or (0, ±1, 0.18)
/// — joined then cut, or cut then joined, the band turned either way: each
/// is built, listed sound, drawn whole and closed, uncrossed, holding the
/// ball and the band less the cap, which misses the band. The ball's face
/// holds the pole the cap leaves it, the slit bounding nothing, and is drawn
/// clear of the band's inner wall along the half of its equator the two
/// only touch.
#[test]
fn a_ball_a_half_band_touches_along_half_its_equator_cut_by_a_flat_about_either_pole_holds_the_ball_and_the_band_less_the_cap_whichever_comes_first()
 {
    let ball = ball_about_y();
    let curved = 10.0 * PI * (10.0 - 1.0) + 11.0 * PI;
    for way in [1.0, -1.0] {
        let half = half_band_about_y(way);
        for up in [-1.0, 1.0] {
            for tilt in [0.3, 0.0] {
                let towards = DVec3::new(tilt, up, 0.18);
                let flat = slab_along(towards, 4.0);
                for (how, built) in [
                    (
                        "joined then cut",
                        ball.union(&half)
                            .and_then(|joined| joined.difference(&flat)),
                    ),
                    (
                        "cut then joined",
                        ball.difference(&flat).and_then(|cut| cut.union(&half)),
                    ),
                ] {
                    built_whole(
                        built,
                        BALL_OF_5 + 5.5 * PI - cap_of_5(1.0),
                        curved,
                        &format!(
                            "the ball and the half band turned {way} of a half turn, {how} by \
                             the flat along {towards}"
                        ),
                    );
                }
            }
        }
    }
}

/// The half disc of radius 5 about the origin, on the side of
/// `axis.any_orthonormal_vector()` in the plane it spans with `axis`, turned
/// half a turn about `axis` — and the normal of that plane pointing away
/// from the half ball, which [`about_v`] lays on the side it lays
/// [`ball_turned_about_y`]'s, under z = 0.
fn half_ball_about(axis: DVec3) -> (Body, DVec3) {
    let sampled: &'static [DVec2] = Box::leak(
        (0..=32)
            .map(|step| DVec2::from_angle(-PI / 2.0 + PI * f64::from(step) / 32.0) * 5.0)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let half = Contour {
        corners: vec![DVec2::new(0.0, -5.0), DVec2::new(0.0, 5.0)],
        runs: vec![
            Run::Round {
                center: DVec2::ZERO,
                turn: PI,
            },
            Run::Straight,
        ],
    };
    let profile = Profile {
        exact: Some((half, Vec::new())),
        sampled: Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let u = axis.any_orthonormal_vector();
    let body = Body::default()
        .tool_turned(
            &profile,
            Frame {
                origin: DVec3::ZERO,
                u,
                v: axis,
            },
            &about_v(&profile, PI),
        )
        .expect("the exact kernel turns the half ball");
    (body, u.cross(axis))
}

/// A shaft of `radius` along Z, from z = `from` to z = `to`.
fn shaft_along_z(radius: f64, from: f64, to: f64) -> Body {
    let round = Contour {
        corners: vec![DVec2::X * radius],
        runs: vec![Run::Round {
            center: DVec2::ZERO,
            turn: TAU,
        }],
    };
    Body::default()
        .tool_raised(
            &exact(round, vec![]),
            Frame {
                origin: DVec3::Z * from,
                ..FLAT
            },
            DVec3::Z * (to - from),
        )
        .expect("the kernel raises the shaft")
}

/// What a shaft of `radius` along Z from `from` to `to` keeps outside the
/// half of the ball of 5 about the origin that lies where `away` points
/// away from: the shaft less, slice by slice along Z, the disc the shaft and
/// the ball share — of the shaft's radius or the ball's, the smaller — on
/// the side of the chord the half's flat face cuts it along. The slices are
/// summed by Simpson's rule between the heights where the disc's radius or
/// the chord's reach changes how a slice is read, where the area runs as a
/// power three halves: a tenth of the steps comes within 1e-11 of all of
/// them.
fn kept_beside_a_half_ball(radius: f64, [from, to]: [f64; 2], away: DVec3) -> f64 {
    let across = DVec2::new(away.x, away.y).length();
    let lean = -away.z / across;
    let disc = |z: f64| radius.min((25.0 - z * z).max(0.0).sqrt());
    let beyond = |reach: f64, chord: f64| {
        if chord >= reach {
            0.0
        } else if chord <= -reach {
            PI * reach * reach
        } else {
            reach * reach * (chord / reach).acos() - chord * (reach * reach - chord * chord).sqrt()
        }
    };
    let taken = |z: f64| {
        let reach = disc(z);
        PI * reach * reach - beyond(reach, lean * z)
    };
    let (low, high) = (from.max(-5.0), to.min(5.0));
    let wall = (25.0 - radius * radius).sqrt();
    let sphere = 5.0 / (1.0 + lean * lean).sqrt();
    let mut cuts = vec![low, high, -wall, wall, -sphere, sphere, 0.0];
    if lean != 0.0 {
        cuts.extend([-radius / lean.abs(), radius / lean.abs()]);
    }
    cuts.retain(|cut| (low..=high).contains(cut));
    cuts.sort_by(f64::total_cmp);
    cuts.dedup();
    let steps = 20_000;
    let simpson = |a: f64, b: f64| {
        let step = (b - a) / f64::from(steps);
        let inner: f64 = (1..steps)
            .map(|rank| {
                let weight = if rank % 2 == 1 { 4.0 } else { 2.0 };
                weight * taken(a + step * f64::from(rank))
            })
            .sum();
        (taken(a) + inner + taken(b)) * step / 3.0
    };
    let shared: f64 = cuts
        .windows(2)
        .map(|piece| simpson(piece[0], piece[1]))
        .sum();
    PI * radius * radius * (to - from) - shared
}

/// A shaft of radius 4.9 along Z, from z = −10 to the centre of the ball of
/// 5 or 2 past it, less the half ball turned half a turn about an axis
/// through that centre — at a slant, (1, 0, 1)/√2, (−1, 0, 1)/√2 or
/// (0, 1, 1)/√2, or along the shaft: built, listed sound, holding what the
/// slices give. About (1, 0, 1)/√2, before #1117 read the poles its faces
/// hold from the squarest crossing, the body held 166.2 and 224.9 where the
/// slices give 689.8 and 748.5.
#[test]
fn a_shaft_less_a_half_ball_turned_at_a_slant_about_its_centre_holds_what_the_slices_give() {
    let slant = std::f64::consts::FRAC_1_SQRT_2;
    for axis in [
        DVec3::new(slant, 0.0, slant),
        DVec3::new(-slant, 0.0, slant),
        DVec3::new(0.0, slant, slant),
        DVec3::Z,
    ] {
        let (half, away) = half_ball_about(axis);
        for to in [0.0, 2.0] {
            let what = format!("the shaft of 4.9 from −10 to {to} less the half ball about {axis}");
            let body = shaft_along_z(4.9, -10.0, to)
                .difference(&half)
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
            let exact = body.brep().expect("the exact kernel computed the cut");
            assert_eq!(
                listed(&exact.listing(), exact.scale().reach()),
                Ok(()),
                "{what}: listed sound"
            );
            let kept = kept_beside_a_half_ball(4.9, [-10.0, to], away);
            assert!(
                (body.volume() - kept).abs() <= 1e-9 * kept,
                "{what}: holds {}, where the slices give {kept}",
                body.volume()
            );
        }
    }
}

/// The signed area the disc of `radius` about the origin shares with the
/// triangle of the origin, `from` and `to`: the stretches of the edge inside
/// the disc with the origin as triangles, those outside as sectors.
fn shared_with_the_disc(radius: f64, from: DVec2, to: DVec2) -> f64 {
    let run = to - from;
    let (square, half) = (run.length_squared(), from.dot(run));
    let gap = half * half - square * (from.length_squared() - radius * radius);
    let mut cuts = vec![0.0];
    if square > 0.0 && gap > 0.0 {
        for along in [(-half - gap.sqrt()) / square, (-half + gap.sqrt()) / square] {
            if along > 0.0 && along < 1.0 {
                cuts.push(along);
            }
        }
    }
    cuts.push(1.0);
    cuts.windows(2)
        .map(|piece| {
            let (one, other) = (from + run * piece[0], from + run * piece[1]);
            if ((one + other) / 2.0).length() <= radius {
                one.perp_dot(other) / 2.0
            } else {
                radius * radius * one.perp_dot(other).atan2(one.dot(other)) / 2.0
            }
        })
        .sum()
}

/// The area of the disc of `radius` about the origin on the side of each
/// line of `sides`, a normal and how far along it the line stands: a square
/// about the disc clipped by each line, then the disc's share of the
/// triangles the clipped square's edges make with the centre.
fn disc_beyond(radius: f64, sides: &[(DVec2, f64)]) -> f64 {
    let reach = 2.0 * radius;
    let mut corners = vec![
        DVec2::new(-reach, -reach),
        DVec2::new(reach, -reach),
        DVec2::new(reach, reach),
        DVec2::new(-reach, reach),
    ];
    for &(normal, at) in sides {
        let past = |corner: DVec2| normal.dot(corner) - at;
        let mut clipped = Vec::new();
        for rank in 0..corners.len() {
            let (from, to) = (corners[rank], corners[(rank + 1) % corners.len()]);
            let (one, other) = (past(from), past(to));
            if one >= 0.0 {
                clipped.push(from);
            }
            if (one >= 0.0) != (other >= 0.0) {
                clipped.push(from + (to - from) * (one / (one - other)));
            }
        }
        corners = clipped;
    }
    (0..corners.len())
        .map(|rank| {
            shared_with_the_disc(radius, corners[rank], corners[(rank + 1) % corners.len()])
        })
        .sum()
}

/// `along` summed from each of `cuts` to the next by Simpson's rule, in
/// twenty thousand steps a piece.
fn summed(along: impl Fn(f64) -> f64, cuts: &[f64]) -> f64 {
    let steps = 20_000;
    cuts.windows(2)
        .map(|piece| {
            let step = (piece[1] - piece[0]) / f64::from(steps);
            let inner: f64 = (1..steps)
                .map(|rank| {
                    let weight = if rank % 2 == 1 { 4.0 } else { 2.0 };
                    weight * along(piece[0] + step * f64::from(rank))
                })
                .sum();
            (along(piece[0]) + inner + along(piece[1])) * step / 3.0
        })
        .sum()
}

/// What the ball of 5 about the origin holds past both planes `first` and
/// `second` — each a unit normal and how far along it the plane stands —
/// and the area of its sphere there: in slices square to both planes, each a
/// disc of the ball cut by the two planes' lines, and on the sphere the arc
/// of each slice's circle both reach, Archimedes' measure along their line.
/// The slices are summed between the heights where a line or the corner of
/// the two leaves the slice's disc.
fn ball_past_two(first: (DVec3, f64), second: (DVec3, f64)) -> (f64, f64) {
    let (across, up) = (
        first.0,
        (second.0 - first.0 * second.0.dot(first.0)).normalize(),
    );
    let normal = DVec2::new(second.0.dot(across), second.0.dot(up));
    let sides = [(DVec2::X, first.1), (normal, second.1)];
    let corner = DVec2::new(first.1, (second.1 - normal.x * first.1) / normal.y);
    let radius = |height: f64| (25.0 - height * height).max(0.0).sqrt();
    let mut cuts = vec![-5.0, 5.0];
    for reach in [first.1, second.1, corner.length()] {
        if reach < 5.0 {
            cuts.extend([-radius(reach), radius(reach)]);
        }
    }
    cuts.sort_by(f64::total_cmp);
    cuts.dedup();
    let held = summed(|height| disc_beyond(radius(height), &sides), &cuts);
    let apart = normal.y.atan2(normal.x);
    let arc = |height: f64| {
        let reach = radius(height);
        if reach <= first.1 || reach <= second.1 {
            return 0.0;
        }
        let [one, other] = [first.1, second.1].map(|at| (at / reach).acos());
        (one.min(apart + other) - (-one).max(apart - other)).max(0.0)
    };
    (held, 5.0 * summed(arc, &cuts))
}

/// What [`half_band_about_y`] turned `way` holds past the plane `flat`, a
/// unit normal and how far along it the plane stands: in slices along Y, the
/// half annulus from 5 to 6 on its side of z = 0 beyond the plane's line.
fn band_past(way: f64, flat: (DVec3, f64)) -> f64 {
    let (normal, at) = flat;
    let beyond = |height: f64| {
        let sides = [
            (DVec2::new(normal.x, normal.z), at - normal.y * height),
            (DVec2::new(0.0, -way), 0.0),
        ];
        disc_beyond(6.0, &sides) - disc_beyond(5.0, &sides)
    };
    summed(beyond, &[-0.5, 0.5])
}

/// A case of [`the_zone`] or [`the_zone_cut_short`]: how it was built,
/// what it gave, what the arithmetic gives it, and the area of its curved
/// faces.
type Case = (String, Result<Body, Declined>, f64, f64);

/// The ball joined to [`half_band_about_y`] turned either way and cut about
/// both poles by the half-band test's flats, 4 from its centre along
/// (tilt, ±1, 0.18), joined then cut or cut then joined: the zone left
/// between the two caps winds a whole turn about the ball's axis.
fn the_zone(ball: &Body) -> Vec<Case> {
    let kept = BALL_OF_5 + 5.5 * PI - 2.0 * cap_of_5(1.0);
    let curved = 100.0 * PI - 20.0 * PI + 11.0 * PI;
    let mut cases = Vec::new();
    for way in [1.0, -1.0] {
        let half = half_band_about_y(way);
        for tilt in [0.3, 0.0] {
            let [low, high] = [-1.0, 1.0].map(|up| slab_along(DVec3::new(tilt, up, 0.18), 4.0));
            let what = |how: &str| {
                format!(
                    "the ball and the half band turned {way}, {how} about both poles, tilt {tilt}"
                )
            };
            cases.push((
                what("joined then cut"),
                ball.union(&half)
                    .and_then(|joined| joined.difference(&low))
                    .and_then(|cut| cut.difference(&high)),
                kept,
                curved,
            ));
            cases.push((
                what("cut then joined"),
                ball.difference(&low)
                    .and_then(|cut| cut.difference(&high))
                    .and_then(|cut| cut.union(&half)),
                kept,
                curved,
            ));
        }
    }
    cases
}

/// [`the_zone`] at the tilt of 0.3, cut by a third flat 2 from the centre
/// along (0.1, 0.05, ±1), whose cap meets both of the others and leaves the
/// zone short of a turn — taking a part of the band too where the band lies
/// on its side, the band always joined before the third flat cuts — in two
/// orders. The arithmetic is the ball and the band less the three caps, plus
/// what the third shares with each of the others, less what it takes of the
/// band; the curved faces count the band's walls whole, the most the third
/// flat leaves of them.
fn the_zone_cut_short(ball: &Body) -> Vec<Case> {
    let pole = |up: f64| (DVec3::new(0.3, up, 0.18).normalize(), 4.0);
    let poles = [pole(-1.0), pole(1.0)];
    let [low, high] = poles.map(|(towards, at)| slab_along(towards, at));
    let mut cases = Vec::new();
    for way in [1.0, -1.0] {
        let half = half_band_about_y(way);
        for side in [1.0, -1.0] {
            let third = (DVec3::new(0.1, 0.05, side).normalize(), 2.0);
            let flat = slab_along(third.0, third.1);
            let [(low_held, low_area), (high_held, high_area)] =
                poles.map(|pole| ball_past_two(pole, third));
            let kept =
                BALL_OF_5 + 5.5 * PI - 2.0 * cap_of_5(1.0) - cap_of_5(3.0) + low_held + high_held
                    - band_past(way, third);
            let curved = 100.0 * PI - 20.0 * PI - 30.0 * PI + low_area + high_area + 11.0 * PI;
            let what = |how: &str| {
                format!(
                    "the ball and the half band turned {way}, {how}, the third flat along {}",
                    third.0
                )
            };
            cases.push((
                what("joined then cut about both poles and by the third flat"),
                ball.union(&half)
                    .and_then(|joined| joined.difference(&low))
                    .and_then(|cut| cut.difference(&high))
                    .and_then(|cut| cut.difference(&flat)),
                kept,
                curved,
            ));
            cases.push((
                what("cut about both poles, joined, then cut by the third flat"),
                ball.difference(&high)
                    .and_then(|cut| cut.difference(&low))
                    .and_then(|cut| cut.union(&half))
                    .and_then(|joined| joined.difference(&flat)),
                kept,
                curved,
            ));
        }
    }
    cases
}

/// [`the_zone`]: each is built, listed sound, drawn whole and closed,
/// uncrossed, holding the ball and the band less both caps — the zone a
/// whole turn, laid as a band by `pierced.rs`, clear of the band's inner
/// wall along the half of its equator the two only touch.
#[test]
fn a_ball_a_half_band_touches_along_half_its_equator_cut_about_both_poles_is_built_whole_whichever_comes_first()
 {
    for (what, built, kept, curved) in the_zone(&ball_about_y()) {
        built_whole(built, kept, curved, &what);
    }
}

/// [`the_zone_cut_short`]: each is built, listed sound, drawn whole and
/// closed, uncrossed, holding what the arithmetic gives it — the zone short
/// of a turn, laid by `charted.rs`, clear of the band's inner wall along
/// what is left of the half of its equator the two only touch.
#[test]
fn a_ball_a_half_band_touches_along_half_its_equator_cut_about_both_poles_and_short_of_a_turn_is_built_whole_whichever_comes_first()
 {
    for (what, built, kept, curved) in the_zone_cut_short(&ball_about_y()) {
        built_whole(built, kept, curved, &what);
    }
}
