//! A round's grid of angles lined up with the grids of the coaxial walls
//! standing beside it (#644, #1123), so that the strip of the round along
//! such a wall stands on the wall's columns.

use std::cmp::Ordering;
use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::super::divisions_in;
use super::super::round::Rounded;
use crate::brep::curve::{Circle, Curve};
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::tessellation::contact::lies_on;
use crate::brep::topology::{Body, SurfaceId};

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
/// round first, then the nearest ([`Beside::order`]), and one whose multiple
/// with those taken would pass the most a turn is cut into is left out
/// alone: leaving them all out undid a half ball's shaft beside a pin's
/// bore, and taking a wall apart before the post a ring touches undid the
/// post's (#1123). A wall touching it by its radius alone, its faces
/// nowhere near the equators' plane, is taken after every other: taken
/// first, a rod of a ring's inner radius standing above it left out the
/// shaft a hair inside its outer equator, whose rim crossed (#1130).
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
    let mut walls: Vec<(bool, Beside, usize)> = Vec::new();
    for (id, surface) in body.surfaces.iter().enumerate() {
        let Surface::Cylinder(wall) = surface else {
            continue;
        };
        let theirs = divisions_in(wall.radius, tolerance, span);
        let turn = wall.u.dot(round.v()).atan2(wall.u.dot(round.u()));
        let ranks = turn / TAU * theirs as f64;
        let reach = 2.0 * wall.radius * (1.0 - (PI / theirs as f64).cos());
        let window = (2.0 * round.radius() * reach).sqrt();
        let reaching = || reaches_the_equators(body, SurfaceId(id as u32), round, window);
        if (ranks - ranks.round()).abs() <= ALIGNED
            && let Some(near) = beside(round, wall, &rims, rows, (eps, reach), reaching)
        {
            let far = matches!(near, Beside::Touching(_)) && !reaching();
            walls.push((far, near, theirs));
        }
    }
    walls.sort_by(|(far, one, _), (other_far, other, _)| far.cmp(other_far).then(one.order(other)));
    let mut every = 1;
    for (_, _, theirs) in walls {
        let both = every / greatest_common(every, theirs) * theirs;
        if steps.div_ceil(both) * both <= cap {
            every = both;
        }
    }
    steps.div_ceil(every) * every
}

/// Under this share of a step, two grids start at one angle.
const ALIGNED: f64 = 1e-9;

/// How a wall stands beside a round, which sets the order the walls are
/// taken in.
#[derive(Clone, Copy, Debug)]
enum Beside {
    /// Along an equator within the kernel's tolerance, this far from it:
    /// #644's touch.
    Touching(f64),
    /// Within twice the sag of its chords of the circle it stands nearest,
    /// this far from it.
    Apart(f64),
}

impl Beside {
    /// A wall touching the round before any standing apart from it, and the
    /// nearer first among either.
    fn order(&self, other: &Beside) -> Ordering {
        match (self, other) {
            (Beside::Touching(_), Beside::Apart(_)) => Ordering::Less,
            (Beside::Apart(_), Beside::Touching(_)) => Ordering::Greater,
            (Beside::Touching(one), Beside::Touching(other))
            | (Beside::Apart(one), Beside::Apart(other)) => one.total_cmp(other),
        }
    }
}

/// How `wall`, on the axis of `round`, stands beside it, if it does.
/// Touching it, its radius an equator's within `eps`, wherever it stands
/// along the axis, as #644 counts it: a ring round a post of its inner
/// radius, the post stopping a hair short of the equator or not. Apart,
/// within `reach` of an equator's circle — twice the sag of the wall's
/// chords, as two walls are told close — where its faces stand beside the
/// equator along the axis, `reaching` its plane: a bore a hair narrower
/// than a ring's inner equator (956002773). Or apart, sharing one of
/// `rims`, the round's latitude circles, within `reach` of the circle of
/// either row about it of the round's grid of `rows` a turn. A wall a hair
/// off an equator's circle or a row's far along the axis from it — a rod a
/// hair outside a ring's inner equator above its tube, a rod on the end of
/// a post a ring touches — stands beside it nowhere.
fn beside(
    round: &Rounded,
    wall: &Cylinder,
    rims: &[&Circle],
    rows: usize,
    (eps, reach): (f64, f64),
    reaching: impl Fn() -> bool,
) -> Option<Beside> {
    let from = round.origin() - wall.origin;
    let off = (from - wall.axis * wall.axis.dot(from)).length();
    let leaning = wall.axis.cross(round.axis()).length() * wall.radius;
    if off > eps || leaning > eps {
        return None;
    }
    let (outer, inner) = (round.widest(), round.widest() - 2.0 * round.radius());
    let touch = (outer - wall.radius).abs().min((inner - wall.radius).abs());
    if touch <= eps {
        return Some(Beside::Touching(touch));
    }
    let along = touch <= reach && reaching();
    let step = TAU / rows as f64;
    rims.iter()
        .filter(|rim| lies_on(rim, wall, eps))
        .flat_map(|rim| {
            let under = (round.latitude(rim) / step).floor();
            [under, under + 1.0]
        })
        .map(|rank| (outer - round.radius() * (1.0 - (rank * step).cos()) - wall.radius).abs())
        .chain(along.then_some(touch))
        .filter(|gap| *gap <= reach)
        .min_by(f64::total_cmp)
        .map(Beside::Apart)
}

/// How many stretches each edge of a wall is read across, along the axis of
/// a round: a wall's extent along its axis is its edges', which a curve it
/// meets another wall along reaches between its ends.
const PLACES: usize = 8;

/// Whether a face of the wall `surface` stands within `window` of the plane
/// of the equators of `round`, along its axis: its edges read at their ends
/// and at [`PLACES`] places between. A round stays within a wall's reach of
/// it, `h`, over √(2 r h) either side of the equator it stands beside, the
/// band `window` is given.
fn reaches_the_equators(body: &Body, surface: SurfaceId, round: &Rounded, window: f64) -> bool {
    let height = |point: DVec3| (point - round.origin()).dot(round.axis());
    let level = height(round.point(DVec2::ZERO));
    let (low, high) = body
        .face_ids()
        .map(|id| body.face(id))
        .filter(|face| face.surface == surface)
        .flat_map(|face| face.loops.iter().flatten())
        .flat_map(|coedge| {
            let edge = body.edge(coedge.edge);
            let curve = body.curve(edge.curve);
            (0..=PLACES).map(move |place| {
                let at = edge.from + (edge.to - edge.from) * place as f64 / PLACES as f64;
                height(curve.point(at))
            })
        })
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), at| {
            (low.min(at), high.max(at))
        });
    low - window <= level && level <= high + window
}

fn greatest_common(one: usize, other: usize) -> usize {
    if other == 0 {
        one
    } else {
        greatest_common(other, one % other)
    }
}

#[cfg(test)]
mod tests;
