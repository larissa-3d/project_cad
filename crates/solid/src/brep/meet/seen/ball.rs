//! The curve a cylinder meets a sphere along (#805), seen in the sphere's
//! `(θ, φ)`: the latitude, and the angle about the sphere's axis unwrapped
//! along the component, which may wind round a pole or not.
//!
//! No closed form tells how many turns the angle has made at a place of the
//! component, so it is read once along the component, [`SAMPLES`] places a
//! turn of the parameter, each angle taken within half a turn of the one
//! before; at any parameter the angle is then the one within half a turn of
//! the nearest place read. That holds wherever the angle turns by less than
//! [`STEP`] between two places and the component stands at least [`POLE`] of
//! the radius off the sphere's axis, where nothing swings it round between
//! two places: a component that does not is not read, and a face of the
//! sphere it bounds is declined.

use std::cell::RefCell;
use std::f64::consts::{PI, TAU};
use std::rc::Rc;

use glam::{DVec2, DVec3};

use super::super::pair::Pair;
use super::{Around, angles};
use crate::brep::curve::{Meet, Wall};
use crate::brep::surface::Sphere;

/// Places a turn of the parameter the angle is read at.
const SAMPLES: usize = 512;

/// The most the angle may turn between two places read.
const STEP: f64 = PI / 8.0;

/// How near a pole, as a share of the radius off the sphere's axis, a
/// component may come and still be read.
const POLE: f64 = 1e-3;

/// How many components the reading is kept for, the last asked first, as
/// the curves of two walls are kept in `meet.rs`.
const KEPT: usize = 64;

/// A component, and its angles read, or the want of them.
type Kept = (Meet, Option<Rc<Vec<f64>>>);

thread_local! {
    static READ: RefCell<Vec<Kept>> = const { RefCell::new(Vec::new()) };
}

impl Meet {
    /// Whether the component, the curve a cylinder meets a sphere along, can
    /// be seen in the sphere's `(θ, φ)`: it stands off both poles, and its
    /// angle about the axis is read all along it.
    pub(in crate::brep) fn charted_on_the_ball(&self) -> bool {
        self.unwrapped().is_some()
    }

    /// The angles about the sphere's axis at even places along the first
    /// period, its end included, each within half a turn of the one before.
    fn unwrapped(&self) -> Option<Rc<Vec<f64>>> {
        READ.with(|read| {
            let mut read = read.borrow_mut();
            if let Some(rank) = read.iter().position(|(meet, _)| meet == self) {
                read[..=rank].rotate_right(1);
                return read[0].1.clone();
            }
            let found = self.read_round().map(Rc::new);
            read.truncate(KEPT - 1);
            read.insert(0, (*self, found.clone()));
            found
        })
    }

    fn read_round(&self) -> Option<Vec<f64>> {
        let Wall::Sphere(ball) = self.second else {
            return None;
        };
        let pair = self.pair();
        if self.component >= pair.components() {
            return None;
        }
        let period = pair.period();
        let count = SAMPLES * (period / TAU).round() as usize;
        let signs = pair.signs(self.component);
        let mut angles: Vec<f64> = Vec::with_capacity(count + 1);
        for step in 0..=count {
            let t = period * step as f64 / count as f64;
            let from = pair.world(pair.local(signs, t)[0]) - ball.middle();
            let flat = DVec2::new(from.dot(ball.u), from.dot(ball.v));
            if flat.length() < POLE * ball.radius {
                return None;
            }
            let raw = flat.y.atan2(flat.x);
            let angle = match angles.last() {
                Some(&last) => {
                    let angle = raw + TAU * ((last - raw) / TAU).round();
                    if (angle - last).abs() > STEP {
                        return None;
                    }
                    angle
                }
                None => raw,
            };
            angles.push(angle);
        }
        Some(angles)
    }

    /// The point at `t` in the `(θ, φ)` of `ball`, the pair's second wall,
    /// with its first and second derivatives with respect to `t`; not a
    /// number where the component cannot be read so.
    pub(super) fn seen_on_the_ball(&self, pair: &Pair, ball: &Sphere, t: f64) -> [DVec2; 3] {
        let Some(read) = self.unwrapped() else {
            return [DVec2::NAN; 3];
        };
        let count = read.len() - 1;
        let period = pair.period();
        let turns = (t / period).floor();
        let rank = (((t - turns * period) / period * count as f64).round() as usize).min(count);
        let reference = read[rank] + turns * (read[count] - read[0]);
        let flat = |v: DVec3| DVec2::new(v.dot(ball.u), v.dot(ball.v));
        let [point, speed, bend] = pair.local(pair.signs(self.component), t);
        let from = pair.world(point) - ball.middle();
        let along = [from, pair.direction(speed), pair.direction(bend)];
        let [angle, turn, sway] = angles(along.map(flat), Around::Winding(reference), 0.0);
        let [r, r1, r2] = along.map(flat);
        let [h, h1, h2] = along.map(|v| v.dot(ball.axis));
        let rho = r.length();
        let out = r.dot(r1) / rho;
        let out_sway = (r1.length_squared() + r.dot(r2)) / rho - out * out / rho;
        let whole = rho * rho + h * h;
        let lift = rho * h1 - h * out;
        let climb_sway =
            (rho * h2 - h * out_sway) / whole - lift * 2.0 * (rho * out + h * h1) / (whole * whole);
        [
            DVec2::new(angle, h.atan2(rho)),
            DVec2::new(turn, lift / whole),
            DVec2::new(sway, climb_sway),
        ]
    }
}
