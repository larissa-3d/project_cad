//! A circle about a sphere's axis standing just inside it, off it, closer
//! than twice the sag of a cell of its grid (#1144): the end plane of a shaft
//! a little under a ball's radius, ending deep inside the band where its
//! wall and the sphere stand that close. No edge of the sphere's face runs
//! there, and a triangle of its lattice from a row on one side of the
//! circle's height to the next on the other sags inside the circle, through
//! the face the circle bounds: a shaft of 5 − 1e-5 ending 1e-3 inside a
//! ball of 5 has its end plane's rim 2e-6 under the sphere, where the chord
//! between two rows a step apart along the meridian sags some 4e-4. The
//! face is cut at the circle's height as at a row of its grid instead: its
//! lattice points there stand on the sphere, outside the circle, on the
//! rays of the circle's samples wherever the round's grid is lined up with
//! the circle's wall.

use std::f64::consts::PI;

use super::super::round::Rounded;
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::topology::Body;

/// The angles along the meridian of `round`, a sphere whose grid has `steps`
/// angles and `rows` rows a turn, at the height of every circle of `body`
/// about its axis that stands inside it by more than `eps` and by no more
/// than twice the sag of a cell of that grid, its chord about the axis and
/// its chord along the meridian together; none on a torus.
pub(in crate::brep::tessellation) fn latitudes(
    body: &Body,
    round: &Rounded,
    (steps, rows): (usize, usize),
    eps: f64,
) -> Vec<f64> {
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
            (off <= eps && inside > eps && inside <= near)
                .then(|| (height / sphere.radius).clamp(-1.0, 1.0).asin() + 0.0)
        })
        .collect()
}
