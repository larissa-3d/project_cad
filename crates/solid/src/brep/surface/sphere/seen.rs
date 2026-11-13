//! A circle the sphere carries neither as a latitude nor as a meridian —
//! the section of a plane at a slant to its axis, or holding a direction of
//! it off it (#805) — seen in its `(θ, φ)`, with `θ` unwrapped so that it
//! never jumps by a turn along the circle.
//!
//! Seen down the axis the circle is an ellipse, whose angle about the axis
//! runs round once a turn, the way the circle's axis points along the
//! sphere's, where the axis pierces the circle's disc, and turns back within
//! half a turn of its centre's angle where it does not. Inside, the angle is
//! read in two steps, each within half a turn of the one before it: the
//! circle's own angle about the pierce point, within a quarter turn of the
//! circle's parameter, then that angle carried down the axis, which an
//! affine map of the plane shifts by less than half a turn all round. A
//! circle through a pole has no angle there, and is no circle this reads.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::Sphere;
use crate::brep::curve::Circle;

impl Sphere {
    /// Whether the circle passes within `eps` of a pole.
    pub(in crate::brep) fn through_a_pole(&self, circle: &Circle, eps: f64) -> bool {
        [-1.0, 1.0].into_iter().any(|side| {
            let from = self.middle() + self.axis * (side * self.radius) - circle.center;
            let h = from.dot(circle.axis);
            DVec2::new((from - circle.axis * h).length() - circle.radius, h).length() <= eps
        })
    }

    /// The point of `circle` at `t` in the sphere's `(θ, φ)`, with its first
    /// and second derivatives with respect to `t`, for a circle the sphere
    /// carries through neither pole.
    pub(in crate::brep) fn seen_along(&self, circle: &Circle, t: f64) -> [DVec2; 3] {
        let (sin, cos) = t.sin_cos();
        let radial = circle.u * cos + circle.v * sin;
        let point = circle.center + radial * circle.radius;
        let speed = (circle.v * cos - circle.u * sin) * circle.radius;
        let bend = -radial * circle.radius;
        let from = point - self.middle();
        let seen = |v: DVec3| DVec3::new(v.dot(self.u), v.dot(self.v), v.dot(self.axis));
        let [at, run, sway] = [seen(from), seen(speed), seen(bend)];
        let square = at.x * at.x + at.y * at.y;
        let rho = square.sqrt();
        let turn = (at.x * run.y - at.y * run.x) / square;
        let out = (at.x * run.x + at.y * run.y) / rho;
        let sway_turn = (at.x * sway.y - at.y * sway.x) / square - 2.0 * turn * out / rho;
        let out_sway =
            (run.x * run.x + run.y * run.y + at.x * sway.x + at.y * sway.y) / rho - out * out / rho;
        let whole = square + at.z * at.z;
        let lift = rho * run.z - at.z * out;
        let climb = lift / whole;
        let climb_sway = (rho * sway.z - at.z * out_sway) / whole
            - lift * 2.0 * (rho * out + at.z * run.z) / (whole * whole);
        [
            DVec2::new(self.angle(circle, t, at), at.z.atan2(rho)),
            DVec2::new(turn, climb),
            DVec2::new(sway_turn, climb_sway),
        ]
    }

    /// The angle about the axis of the circle's point at `t`, `at` read in
    /// the sphere's frame from its centre, unwrapped as the module says.
    fn angle(&self, circle: &Circle, t: f64, at: DVec3) -> f64 {
        let raw = at.y.atan2(at.x);
        let pierce = circle.axis.dot(self.axis);
        let off = self.middle() - circle.center;
        let to_the_axis = off * pierce + self.axis * circle.axis.dot(-off);
        let inside = to_the_axis.length_squared() < (circle.radius * pierce).powi(2);
        if !inside {
            let centre = -off;
            let across = centre.dot(self.v).atan2(centre.dot(self.u));
            return across + wrapped(raw - across);
        }
        let pierced = to_the_axis / pierce;
        let (x, y) = (pierced.dot(circle.u), pierced.dot(circle.v));
        let (sin, cos) = t.sin_cos();
        let own = t + wrapped((circle.radius * sin - y).atan2(circle.radius * cos - x) - t);
        let way = circle.u.cross(circle.v).dot(self.axis).signum();
        let shift = circle.u.dot(self.v).atan2(circle.u.dot(self.u));
        let carried = way * own + shift;
        carried + wrapped(raw - carried)
    }
}

/// An angle moved by whole turns into `[-π, π)`.
fn wrapped(angle: f64) -> f64 {
    (angle + PI).rem_euclid(TAU) - PI
}
