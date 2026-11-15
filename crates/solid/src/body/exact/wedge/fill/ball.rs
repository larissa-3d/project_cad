//! What a fill is trimmed by at a sphere end (#805): the box standing past
//! where the prism's long edges leave the sphere, less the ball, so that the
//! fill ends flush on it. The ball is turned about the sphere's own axis, so
//! that the fill's end lies on the very surface of the face it merges into.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::Prism;
use super::round::long_edges;
use crate::body::exact::Exact;
use crate::body::exact::wedge::end::beyond;
use crate::brep::{Declined, Plane, Sphere};
use crate::profile::{Contour, Frame, Run};
use crate::turning::{Axis, ON_THE_AXIS, Straight, Turn};

/// The solid holding what of the prism stands outside `sphere` past where
/// its long edges, running on `past`, leave it: the box [`beyond`] raises
/// past the plane square to `past` where every long edge that meets the
/// sphere stands inside it — clear short of where the first leaves it, or
/// the plane through the centre where that is nearer — less the ball. An
/// edge that misses the sphere is cut where the box starts, as a cylinder's
/// rectangle cuts it. Declined `Unsupported` where no edge meets the sphere
/// and where one leaves it past the prism's end; as [`beyond`] declines; a
/// ball the kernel cannot turn or a box it cannot cut, as those say.
pub(super) fn outside_sphere(
    prism: &Prism,
    sphere: &Sphere,
    past: DVec3,
    reach: f64,
) -> Result<Exact, Declined> {
    let towards = past.normalize();
    let middle = sphere.middle();
    let (edges, clear) = long_edges(prism, past);
    let mut leaving = f64::INFINITY;
    for [_, far] in &edges {
        let from = *far - middle;
        let along = from.dot(towards);
        let half = (sphere.radius.powi(2) - (from - towards * along).length_squared()).sqrt();
        if half > 0.0 {
            if half >= along {
                return Err(Declined::Unsupported);
            }
            leaving = leaving.min(half);
        }
    }
    if !leaving.is_finite() {
        return Err(Declined::Unsupported);
    }
    let (start, _) = Plane::through(middle + towards * (leaving - clear).max(0.0), towards);
    beyond(prism, &start, towards)?.cut_by(&ball(sphere, reach)?)
}

/// The whole ball of `sphere`, its half disc turned about the sphere's own
/// axis.
fn ball(sphere: &Sphere, reach: f64) -> Result<Exact, Declined> {
    let (low, high) = (sphere.center - sphere.radius, sphere.center + sphere.radius);
    let half_disc = Contour {
        corners: vec![DVec2::new(low, 0.0), DVec2::new(high, 0.0)],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: DVec2::new(sphere.center, 0.0),
                turn: PI,
            },
        ],
    };
    let frame = Frame {
        origin: sphere.origin,
        u: sphere.axis,
        v: sphere.u,
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::X,
        },
        angle: TAU,
        resolution: 0.0,
        on_the_axis: ON_THE_AXIS * sphere.radius,
    };
    let straight =
        Straight::of(&half_disc, &[], frame, &turn, reach).ok_or(Declined::Unsupported)?;
    Exact::turned(&straight, frame, &turn, straight.numbers(true))
}
