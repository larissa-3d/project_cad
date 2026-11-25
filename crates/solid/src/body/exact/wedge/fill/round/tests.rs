//! What a fill is trimmed by at a round it runs into, its matter inside —
//! a pillar (#908), a bored boss (#915) — read on [`inside_cylinder`]
//! itself. The cylinder's disc is cut from the prism where it stands inside
//! the round, so a disc raised taller than the fill cuts the same matter, and
//! the body a fill leaves carries its part's reach whatever the disc's
//! (#1023): only the disc's own extent along the axis, and the reach it
//! carries into the booleans that cut the fill, tell its height.

use glam::{DVec2, DVec3};

use super::inside_cylinder;
use crate::body::exact::wedge::end::CLEAR;
use crate::brep::Cylinder;
use crate::profile::{Contour, Frame, Run};

/// How far a place may stand from where the arithmetic puts it: each is a
/// sum of eighths, so what is left is rounding.
const PLACE: f64 = 1e-12;

/// Where the plate's top stands: 200 up, so that a disc raised past the fill
/// along the upright axis reaches further than anything else of it.
const TOP: f64 = 200.0;

/// The prism of a fill of 1 at the foot of a wall's front, at y = 12 on the
/// plate's top, [`TOP`] up, running along X into a pillar of 3 about the
/// upright line through (25, 13): a right triangle, its legs 1 out along −Y
/// on the plate and 1 up the wall, raised 30 along X from x = 2. Every
/// corner of the section stands within 1 of the edge, so the trim stands
/// clear of the prism by [`CLEAR`].
fn running_into_a_pillar_far_up() -> (Cylinder, (Contour, Frame, DVec3)) {
    let section = Contour {
        corners: vec![DVec2::ZERO, DVec2::X, DVec2::Y],
        runs: vec![Run::Straight; 3],
    };
    let frame = Frame {
        origin: DVec3::new(2.0, 12.0, TOP),
        u: -DVec3::Y,
        v: DVec3::Z,
    };
    (
        Cylinder::about(DVec3::new(25.0, 13.0, 0.0), DVec3::Z, 3.0),
        (section, frame, DVec3::X * 30.0),
    )
}

#[test]
fn the_disc_a_fill_is_trimmed_by_inside_a_pillar_stands_a_clearance_under_and_over_what_the_fill_spans_along_its_axis()
 {
    let (cylinder, prism) = running_into_a_pillar_far_up();

    let trims = inside_cylinder(&prism, &cylinder, DVec3::X)
        .unwrap_or_else(|declined| panic!("the trim inside the pillar: {declined:?}"));

    let [_, disc] = trims.as_slice() else {
        panic!(
            "the trim inside the pillar is the box past its axis and the disc, {} solids",
            trims.len()
        );
    };
    let promised = [TOP - CLEAR, TOP + 1.0 + CLEAR];
    let (low, high) = disc.bounds().expect("the disc has a box");
    assert!(
        (low.z - promised[0]).abs() < PLACE && (high.z - promised[1]).abs() < PLACE,
        "the disc a fill of 1 on the plate's top {TOP} up is trimmed by inside a pillar stands from \
         {} to {}, where the fill spans {TOP} to {} along the axis and the disc stands a clearance \
         of {CLEAR} under and over that, from {} to {}",
        low.z,
        high.z,
        TOP + 1.0,
        promised[0],
        promised[1]
    );
    let reach = disc.reach();
    assert!(
        (reach - promised[1]).abs() < PLACE,
        "the disc carries a reach of {reach} into the booleans that cut the fill, where its top, \
         {}, is the furthest it stands",
        promised[1]
    );
}
