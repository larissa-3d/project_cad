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
//! Beneath the body, the relation of a plane and a sphere reads that touch
//! within half the tolerance, at the point, in either order, misses the
//! sphere twice the tolerance beyond and cuts its small circle twice within
//! (`brep/relation/tests/round.rs`); and the exact kernel's join and cuts
//! hold without the drawing (`brep/combine/tests/round.rs`).

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

/// Holds that `result` is built by the exact kernel, listed sound, holds
/// `kept`, and that its triangles are closed, uncrossed and enclose that
/// within what the drawing promises over its curved faces, `curved` in all,
/// and the rounding of a sum of that much matter.
fn built_whole(result: Result<Body, Declined>, kept: f64, curved: f64, what: &str) {
    let body = result.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    let exact = body.brep().expect("the exact kernel computed the step");
    assert_eq!(
        listed(&exact.listing(), exact.scale().reach()),
        Ok(()),
        "{what}: listed sound"
    );
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
        let eps = ball
            .brep()
            .expect("the exact ball")
            .scale()
            .joined(
                slab_along(towards, 5.0)
                    .brep()
                    .expect("the exact slab")
                    .scale(),
            )
            .eps();
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
