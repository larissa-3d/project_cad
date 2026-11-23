//! A face of a cone, checked before its body is handed back: the apex it
//! holds within it is read off the traces of its loops, as the poles of a
//! sphere's or a torus's face are, so a face whose loops its cone cannot
//! trace is declined as the trace declines, as theirs is since #938, rather
//! than read as holding no apex and given a flux short of the apex's share.
//!
//! Closes #957.
//! - A cone face with an untraceable loop (a test-only construction if no
//!   builder reaches it) is declined, not given a volume —
//!   `a_tip_whose_ellipse_its_cone_cannot_trace_is_declined_as_the_trace_declines_not_given_a_volume`
//! - Every cone test still passes — no test: it is the suite as it stands,
//!   every test that builds a cone — in `brep/turned`, `brep/volume`,
//!   `brep/tessellation/tests/cones.rs` and `conics.rs`,
//!   `brep/combine/tests/conical.rs`, `soundness/listed`, and the points
//!   `cao_part` turns — passing unedited under `cargo test --workspace`
//!
//! No step reaches such a face today, and the test builds it by hand. A conic
//! a cone carries is always traced on it, whichever cone it was cut on: as its
//! own, or as the conic that cone cuts the conic's plane along; and a curve a
//! cone meets another wall along names the cone first, and is read on either.
//! What a cone cannot trace is a loop it does not carry, so the tip's wall is
//! laid again on a cone of the same apex its ellipse does not lie on — which
//! no step lays: a boolean lays a face on the surface its curves were cut on,
//! and a step laying a face of its own checks it on its surface first
//! (`replaced_by`).

use super::*;
use crate::brep::surface::Cone;
use crate::brep::tessellation::tests::slanted::{
    BASE, TIP, tip_cut_at_a_slant, tip_cut_at_a_slant_volume,
};

/// How steeply the plane cutting the point's tip off leans along X.
const LEAN: f64 = 0.3;

/// The rank among a body's surfaces of the one cone it lies on.
fn the_cone_of(body: &Body) -> usize {
    let cones: Vec<usize> = (body.surfaces.iter().enumerate())
        .filter(|(_, surface)| matches!(surface, Surface::Cone(_)))
        .map(|(rank, _)| rank)
        .collect();
    let [cone] = cones[..] else {
        panic!("the tip lies on one cone: {cones:?}");
    };
    cone
}

/// The point's tip cut off by `z = 12 + 0.3·x`, its wall laid again on the
/// cone of the same apex and axis opening wider, 12 across at the point's
/// foot rather than 10: the ellipse bounding the wall still goes round that
/// apex, and lies on the plane alone. Taken as built, the tip holds what the
/// arithmetic says, its wall holding the apex; laid again, its wall's loop is
/// one its cone does not trace, and the body is declined as the trace
/// declines — never handed back holding no apex, its flux anchored away from
/// it.
#[test]
fn a_tip_whose_ellipse_its_cone_cannot_trace_is_declined_as_the_trace_declines_not_given_a_volume()
{
    let tip = tip_cut_at_a_slant(LEAN);
    let kept = tip_cut_at_a_slant_volume(LEAN);
    let as_built = tip.rejoined_at(&[]).expect("the tip as built is taken");
    assert!(
        (as_built.volume() - kept).abs() <= 1e-9 * kept,
        "the tip as built holds {}, where the arithmetic says {kept}",
        as_built.volume()
    );

    let mut laid = tip.clone();
    let cone = the_cone_of(&laid);
    laid.surfaces[cone] = Surface::Cone(Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, BASE + 2.0), DVec2::new(TIP, 0.0)],
    ));
    let wall = (laid.faces.iter())
        .find(|face| face.surface.0 as usize == cone)
        .expect("the tip's wall lies on its cone");
    let declined = laid
        .traces_of(wall)
        .expect_err("the wider cone does not carry the tip's ellipse, and does not trace it");

    assert_eq!(
        laid.rejoined_at(&[]).map(|body| body.volume()),
        Err(declined),
        "the tip's wall laid again on the wider cone of its apex, the tip as built holding {kept}"
    );
}
