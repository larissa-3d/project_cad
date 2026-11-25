//! A round's grid of angles lined up with the grids of the coaxial walls
//! standing beside it (#644, #1123), so that the strip of the round along
//! such a wall stands on the wall's columns.

use std::f64::consts::{PI, TAU};

use super::super::divisions_in;
use super::super::round::Rounded;
use crate::brep::curve::{Circle, Curve};
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::tessellation::contact::lies_on;
use crate::brep::topology::Body;

/// A round's `steps` a turn, made a multiple of the grid of every cylinder
/// of the body that stands on its axis beside it ([`beside`]), where the two
/// grids start the same way round. A wall touching a round along an equator
/// — a ring round a post of its inner radius or in a bore of its outer one,
/// a ball in a bore of its own — touches it along a circle that parts
/// neither face (#576), so neither face samples it, and a chord of the
/// round along it cut across the cylinder's between two of its steps: the
/// two crossed beside the circle they share (5576855166, 5585606550). A
/// wall crossing the round a hair from a row leaves between them, along the
/// strip from the rim to that row, a ring thinner than their chords sag,
/// and the strip, on columns of its own, reached across from a step of the
/// wall's to the next, through the wall's chords: a half ball cut by a
/// shaft 2.1e-7 to 4e-4 under its radius, its rim a hair under the equator,
/// or 1.9e-3 and 2.2e-3, a hair under the row below, was crossed there
/// (#1123). On a multiple of the wall's grid each column of the strip
/// stands where the rim has a sample, and the strip stays outside the
/// wall's chords. The walls are taken one at a time, those touching the
/// round first, then the nearest a row, and one whose multiple with those
/// taken would pass the most a turn is cut into is left out alone: leaving
/// them all out undid a half ball's shaft beside a pin's bore (#1123).
pub(super) fn kept_by_walls(
    body: &Body,
    round: &Rounded,
    (steps, rows, cap): (usize, usize, usize),
    (tolerance, span): (f64, f64),
    eps: f64,
) -> usize {
    let rims: Vec<&Circle> = body
        .edge_ids()
        .filter_map(|id| match body.curve(body.edge(id).curve) {
            Curve::Circle(circle) if round.holds(circle, eps) => Some(circle),
            Curve::Line(_) | Curve::Circle(_) | Curve::Meet(_) | Curve::Conic(_) => None,
        })
        .collect();
    let mut walls: Vec<((bool, f64), usize)> = Vec::new();
    for surface in &body.surfaces {
        let Surface::Cylinder(wall) = surface else {
            continue;
        };
        let theirs = divisions_in(wall.radius, tolerance, span);
        let turn = wall.u.dot(round.v()).atan2(wall.u.dot(round.u()));
        let ranks = turn / TAU * theirs as f64;
        let sag = wall.radius * (1.0 - (PI / theirs as f64).cos());
        if (ranks - ranks.round()).abs() <= ALIGNED
            && let Some(near) = beside(round, wall, &rims, rows, (eps, 2.0 * sag))
        {
            walls.push((near, theirs));
        }
    }
    walls.sort_by(|(one, _), (other, _)| one.0.cmp(&other.0).then(one.1.total_cmp(&other.1)));
    let mut every = 1;
    for (_, theirs) in walls {
        let both = every / greatest_common(every, theirs) * theirs;
        if steps.div_ceil(both) * both <= cap {
            every = both;
        }
    }
    steps.div_ceil(every) * every
}

/// Under this share of a step, two grids start at one angle.
const ALIGNED: f64 = 1e-9;

/// Whether `wall`, on the axis of `round`, stands apart from it rather than
/// touching it along an equator within `eps`, and how far from the circle
/// it stands nearest; or none. Apart, it stands within `reach` — twice the
/// sag of its chords, as two walls are told close — of an equator, a bore
/// a hair narrower than a ring's inner one (956002773), or shares one of
/// `rims`, the round's latitude circles, within `reach` of the circle of
/// either row about it of the round's grid of `rows` a turn. A wall a hair
/// from a row's circle far along the axis from it — a rod on the end of a
/// post a ring touches — shares no rim with the round, and stands beside
/// it nowhere.
fn beside(
    round: &Rounded,
    wall: &Cylinder,
    rims: &[&Circle],
    rows: usize,
    (eps, reach): (f64, f64),
) -> Option<(bool, f64)> {
    let from = round.origin() - wall.origin;
    let off = (from - wall.axis * wall.axis.dot(from)).length();
    let leaning = wall.axis.cross(round.axis()).length() * wall.radius;
    if off > eps || leaning > eps {
        return None;
    }
    let (outer, inner) = (round.widest(), round.widest() - 2.0 * round.radius());
    let touch = (outer - wall.radius).abs().min((inner - wall.radius).abs());
    if touch <= eps {
        return Some((false, touch));
    }
    let step = TAU / rows as f64;
    rims.iter()
        .filter(|rim| lies_on(rim, wall, eps))
        .flat_map(|rim| {
            let under = (round.latitude(rim) / step).floor();
            [under, under + 1.0]
        })
        .map(|rank| (outer - round.radius() * (1.0 - (rank * step).cos()) - wall.radius).abs())
        .chain([touch])
        .filter(|gap| *gap <= reach)
        .min_by(f64::total_cmp)
        .map(|gap| (true, gap))
}

fn greatest_common(one: usize, other: usize) -> usize {
    if other == 0 {
        one
    } else {
        greatest_common(other, one % other)
    }
}
