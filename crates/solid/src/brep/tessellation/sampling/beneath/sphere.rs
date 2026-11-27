//! A circle about a sphere's axis standing just inside it, off it, closer
//! than twice the sag of a cell of its grid (#1144): the end plane of a shaft
//! a little under a ball's radius, ending deep inside the band where its
//! wall and the sphere stand that close. No edge of the sphere's face runs
//! there, and a triangle of its lattice from a row on one side of the
//! circle's height to the next on the other sags inside the circle, through
//! the face the circle bounds: a shaft of 5 − 1e-5 ending 1e-3 inside a
//! ball of 5 has its end plane's rim 2e-6 under the sphere, where the chord
//! between two rows a step apart along the meridian sags some 4e-4. The
//! face is cut at the circle's height as at a row of its grid instead, its
//! lattice points there on the sphere, outside the circle — but only where
//! its grid stands on every step of the circle's ([`lined_with`]), so that
//! each of those points stands on the ray of one of the circle's samples.
//! On a grid of its own, the row's chords between two lattice points sag
//! inside a sample of the circle between them: a ball cut by a flat across
//! z less a shaft ending a few thousandths above the flat, its wall a few
//! ten-thousandths inside the sphere's circle at its end, drawn sound
//! before, was crossed along that row.

use std::f64::consts::PI;

use super::super::cone::lined_with;
use super::super::round::Rounded;
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::topology::Body;

/// The angles along the meridian of `round`, a sphere whose grid has `steps`
/// angles and `rows` rows a turn at `fineness`, at the height of every
/// circle of `body` about its axis that stands inside it by more than the
/// kernel's tolerance and by no more than twice the sag of a cell of that
/// grid, its chord about the axis and its chord along the meridian
/// together, and whose grid that one stands on; none on a torus.
pub(in crate::brep::tessellation) fn latitudes(
    body: &Body,
    round: &Rounded,
    (steps, rows): (usize, usize),
    fineness: (f64, f64),
) -> Vec<f64> {
    let eps = body.scale().eps();
    let Rounded::Sphere(sphere) = round else {
        return Vec::new();
    };
    let sag = |count: usize| sphere.radius * (1.0 - (PI / count as f64).cos());
    let near = 2.0 * (sag(steps) + sag(rows));
    body.edge_ids()
        .filter_map(|id| match body.curve(body.edge(id).curve) {
            Curve::Circle(circle) => Some(circle),
            Curve::Line(_) | Curve::Meet(_) | Curve::Conic(_) => None,
        })
        .filter(|circle| circle.axis.cross(sphere.axis).length() <= Scale::RELATIVE)
        .filter_map(|circle| {
            let from = circle.center - sphere.middle();
            let height = from.dot(sphere.axis);
            let off = (from - sphere.axis * height).length();
            let across = (sphere.radius * sphere.radius - height * height).max(0.0);
            let inside = across.sqrt() - circle.radius;
            (off <= eps
                && inside > eps
                && inside <= near
                && lined_with(round, steps, circle, fineness))
            .then(|| (height / sphere.radius).clamp(-1.0, 1.0).asin() + 0.0)
        })
        .collect()
}
