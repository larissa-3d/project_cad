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
