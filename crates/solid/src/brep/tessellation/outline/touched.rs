//! A face of a sphere laid out in its `(θ, φ)` and refined there (#805),
//! along a latitude a wall touches it along: a ball zone in a bore of its
//! own radius, its equator on the bore's wall (#1112).
//!
//! The two part there only as the square of the distance from the
//! latitude. The refinement holds a triangle along the boundary by the
//! chord there, once a cut has brought its corner inside the circle on that
//! chord as its diameter: such a corner, a hair off the latitude between
//! the chord's ends, stands outside the chord, and so outside the wall's
//! triangles running from that very chord to the wall's far rim. The strip
//! between the latitude and the next row of the grid inside the face is
//! laid as the strips lay it instead, and kept from the refinement: the
//! lattice points along that row, one at each angle of the grid the
//! latitude is sampled at, joined to each other and, at the strip's two
//! ends, to the latitude, so that every triangle along the latitude
//! reaches the row at an end of its chord and leans inside the wall's
//! chords by half the row's angle, as two cylinders touching take common
//! rays ([`super::super::contact`]). Only a stretch between two angles of
//! the grid no other boundary of the face comes into is laid so; the
//! latitude's last step at either end of a stretch is the refinement's.

use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::TAU;

use glam::DVec2;

use super::super::sampling::{Rounded, Samples, rows};
use super::Outline;
use super::cylinder::CLEAR;
use super::strips::Lattice;
use crate::brep::curve::Curve;
use crate::brep::surface::{Sphere, Surface};
use crate::brep::topology::{Body, Face};

/// The angles along the meridian of `sphere`'s chart at which a wall
/// touches `face`: its edges that are circles of the sphere's own radius
/// about its axis along which a face of a cylinder runs, whose circle the
/// edge then is, so that the cylinder stands on the sphere's axis at its
/// radius.
pub(super) fn touched(body: &Body, face: &Face, sphere: &Sphere, eps: f64) -> Vec<f64> {
    let round = Rounded::Sphere(*sphere);
    let mut latitudes: Vec<f64> = (face.loops.iter().flatten())
        .filter_map(|coedge| {
            let Curve::Circle(circle) = body.curve(body.edge(coedge.edge).curve) else {
                return None;
            };
            let walled = body.uses(coedge.edge).iter().any(|(id, _)| {
                matches!(body.surface(body.face(*id).surface), Surface::Cylinder(_))
            });
            let equator = (circle.radius - sphere.radius).abs() <= eps;
            (walled && equator && round.holds(circle, eps)).then(|| round.latitude(circle))
        })
        .collect();
    latitudes.sort_by(f64::total_cmp);
    latitudes.dedup();
    latitudes
}

impl Outline {
    /// Lays the strip between each of `latitudes` and the next row of
    /// `lattice`'s grid on the face's side of it, wherever the face runs
    /// along it with no other boundary coming into the strip: the lattice
    /// points of that row, at each angle of the grid the latitude is sampled
    /// at, joined to each other and at the stretch's ends to the latitude,
    /// both ways, so that the sweep cuts the strip apart and the refinement
    /// keeps every side of it.
    pub(super) fn beside_walls(
        &mut self,
        samples: &mut Samples,
        lattice: &mut Lattice,
        latitudes: &[f64],
    ) {
        let step = TAU / lattice.steps as f64;
        let (low, high) = (self.points.iter())
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), at| {
                (low.min(at.y), high.max(at.y))
            });
        for &latitude in latitudes {
            for above in [true, false] {
                let (from, to) = if above {
                    (latitude, high)
                } else {
                    (low, latitude)
                };
                let rows = rows(lattice.rows, from, to, &lattice.levels, lattice.room);
                let row = if above { rows.first() } else { rows.last() };
                let Some(&row) = row else {
                    continue;
                };
                for stretch in self.stretches(latitude, row, above, step) {
                    self.strip(samples, lattice, &stretch, row, step);
                }
            }
        }
    }

    /// The stretches of the face along `latitude`, the face on the side of
    /// it `row` stands, each as the points of the latitude at the angles of
    /// the grid along it, in order of the angle about the axis: between two
    /// consecutive ones, the latitude runs unbroken and no other boundary
    /// comes between it and `row`. Neither end of a run along the latitude
    /// is ever one of them.
    fn stretches(&self, latitude: f64, row: f64, above: bool, step: f64) -> Vec<Vec<usize>> {
        let next: BTreeMap<usize, usize> = (self.segments.iter())
            .filter(|&&[from, to]| {
                let (from, to) = (self.points[from], self.points[to]);
                from.y == latitude && to.y == latitude && (to.x > from.x) == above
            })
            .map(|&[from, to]| (from, to))
            .collect();
        let reached: BTreeSet<usize> = next.values().copied().collect();
        let mut inside: Vec<usize> = (next.keys().copied())
            .filter(|point| reached.contains(point))
            .filter(|&point| {
                let x = self.points[point].x;
                (x - (x / step).round() * step).abs() <= CLEAR
            })
            .collect();
        inside.sort_by(|&one, &other| self.points[one].x.total_cmp(&self.points[other].x));
        let way = if above { 1.0 } else { -1.0 };
        let runs = |from: usize, to: usize| {
            let mut at = from;
            while let Some(&then) = next.get(&at) {
                if then == to {
                    return true;
                }
                if (self.points[then].x - self.points[to].x) * way >= 0.0 {
                    return false;
                }
                at = then;
            }
            false
        };
        let into = |at: DVec2| (at.y - latitude) * way;
        let clear = |left: DVec2, right: DVec2| {
            self.segments.iter().all(|&[one, other]| {
                let (one, other) = (self.points[one], self.points[other]);
                into(one).max(into(other)) <= 0.0
                    || into(one).min(into(other)) > (row - latitude) * way
                    || one.x.max(other.x) < left.x
                    || one.x.min(other.x) > right.x
            })
        };
        let mut stretches: Vec<Vec<usize>> = Vec::new();
        let mut open = false;
        for pair in inside.windows(2) {
            let [left, right] = [pair[0], pair[1]];
            let unbroken = if above {
                runs(left, right)
            } else {
                runs(right, left)
            };
            if !unbroken || !clear(self.points[left], self.points[right]) {
                open = false;
                continue;
            }
            match stretches.last_mut() {
                Some(stretch) if open => stretch.push(right),
                _ => stretches.push(vec![left, right]),
            }
            open = true;
        }
        stretches
    }

    /// The strip along `stretch`, points of a latitude at angles of the
    /// grid, up to `row`: the lattice point of the row at each of their
    /// angles, the row's chords between them and the two ends' rungs to the
    /// latitude, each both ways.
    fn strip(
        &mut self,
        samples: &mut Samples,
        lattice: &mut Lattice,
        stretch: &[usize],
        row: f64,
        step: f64,
    ) {
        let (Some(&first), Some(&last)) = (stretch.first(), stretch.last()) else {
            return;
        };
        let mut chain = vec![first];
        for &point in stretch {
            let x = self.points[point].x;
            let sample = lattice.sample(samples, (x / step).round() as i64, row);
            chain.push(self.point(DVec2::new(x, row), sample));
        }
        chain.push(last);
        for pair in chain.windows(2) {
            self.segments.push([pair[0], pair[1]]);
            self.segments.push([pair[1], pair[0]]);
        }
    }
}
