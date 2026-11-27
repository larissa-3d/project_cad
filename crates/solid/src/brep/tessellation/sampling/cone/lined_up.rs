//! A round's grid of angles lined up with the grids of the coaxial walls
//! standing beside it (#644, #1123), so that the strip of the round along
//! such a wall stands on the wall's columns; a sphere's read about another
//! line than its own, lined up so only beside a ring a wall of that line
//! leaves thinner than its chords sag (#1144, [`About`]); and whether a
//! grid stands on every step of a circle's ([`lined_with`]), which a level
//! laid at a circle just inside a sphere asks too.

use std::cmp::Ordering;
use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::super::divisions_in;
use super::super::round::{Rounded, SHARE, midway};
use super::super::steps::most;
use crate::brep::curve::{Circle, Curve};
use crate::brep::surface::{Cylinder, Sphere, Surface};
use crate::brep::tessellation::contact::lies_on;
use crate::brep::topology::{Body, Edge, Face, FaceId, SurfaceId};

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
/// post's (#1123).
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
    let mut walls: Vec<(Beside, usize)> = Vec::new();
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
            walls.push((near, theirs));
        }
    }
    walls.sort_by(|(one, _), (other, _)| one.order(other));
    let mut every = 1;
    for (_, theirs) in walls {
        let both = every / greatest_common(every, theirs) * theirs;
        if steps.div_ceil(both) * both <= cap {
            every = both;
        }
    }
    steps.div_ceil(every) * every
}

/// Whether a grid of `steps` a turn on `round` stands on every step of the
/// grid `circle`, a circle about the round's axis, is sampled on at
/// `fineness`: a multiple of it, starting the same way round, so that every
/// sample the circle takes on its grid has a lattice point on its ray.
pub(in crate::brep::tessellation) fn lined_with(
    round: &Rounded,
    steps: usize,
    circle: &Circle,
    (tolerance, span): (f64, f64),
) -> bool {
    let theirs = divisions_in(circle.radius, tolerance, span);
    let turn = circle.u.dot(round.v()).atan2(circle.u.dot(round.u()));
    let ranks = turn / TAU * steps as f64;
    steps.is_multiple_of(theirs) && (ranks - ranks.round()).abs() <= ALIGNED
}

/// The grid of a sphere read about the line through its centre along
/// another axis than its own ([`read_about`]).
pub(in crate::brep::tessellation) struct About {
    /// The sphere so read.
    round: Rounded,
    /// The steps a turn it is cut into for the sphere's own radius: the grid
    /// of its own axis is cut for the widest its faces reach about that one,
    /// and a face of a slanted cut reaches no circle of it.
    plain: usize,
    /// Those steps lined up with the walls of the line standing beside the
    /// sphere, as [`kept_by_walls`] lines up a round's own grid.
    lined: usize,
    /// The tolerance it is cut for, and the span of the body.
    fineness: (f64, f64),
}

/// The grid of `sphere` read about the line through its centre along
/// `axis`, its rows `rows` a turn, at `fineness`.
pub(in crate::brep::tessellation) fn read_about(
    body: &Body,
    sphere: &Sphere,
    axis: DVec3,
    rows: usize,
    fineness: (f64, f64),
) -> About {
    let (tolerance, span) = fineness;
    let round = Rounded::Sphere(sphere.about(axis));
    let plain = divisions_in(sphere.radius, tolerance * SHARE, span);
    let cap = most(round.widest(), span);
    let eps = body.scale().eps();
    let lined = kept_by_walls(body, &round, (plain, rows, cap), fineness, eps);
    About {
        round,
        plain,
        lined,
        fineness,
    }
}

impl About {
    /// The steps a turn the face `id` of `sphere`, read about `axis`, is cut
    /// in strips on: lined up beside a ring ([`About::beside_a_ring`]), and
    /// plain anywhere else, as before #1144.
    pub(in crate::brep::tessellation) fn steps_of(
        &self,
        body: &Body,
        id: FaceId,
        sphere: &Sphere,
        axis: DVec3,
    ) -> usize {
        if self.beside_a_ring(body, id, sphere, axis) {
            self.lined
        } else {
            self.plain
        }
    }

    /// Whether the face `id` of `sphere`, read about `axis`, runs from one of
    /// its rims towards the sphere's equator about that line beside a face of
    /// a wall the lined grid stands on ([`lined_with`]), running from that
    /// rim the same way, the wall standing under the sphere's radius by more
    /// than the kernel's tolerance and by no more than the plain grid's
    /// chords sag: the ring a ball less a shaft a little under its radius
    /// leaves, through it or ending in the band. Cut on the plain grid, the
    /// strip from the rim to the next row reached from a sample of the rim
    /// across to lattice points a part of a step round, and sagged through the
    /// wall's chords over a ring thinner than they sag (#1144). A face running
    /// from its rim away from the equator — a cap beyond the band, the end a
    /// ball leaves on a shaft (#1138) — or beside a wall running away from it —
    /// the ball joined with the shaft — and a ring thicker than that sag keep
    /// the plain grid, drawn as before.
    pub(in crate::brep::tessellation) fn beside_a_ring(
        &self,
        body: &Body,
        id: FaceId,
        sphere: &Sphere,
        axis: DVec3,
    ) -> bool {
        let face = body.face(id);
        let eps = body.scale().eps();
        let sag = sphere.radius * (1.0 - (PI / self.plain as f64).cos());
        let middle = sphere.middle();
        let towards = |face: &Face, forward: bool, edge: &Edge, circle: &Circle| {
            let at = midway(edge);
            let surface = body.surface(face.surface);
            let normal = surface.normal(surface.parameters(circle.point(at)));
            let outward = if face.flipped { -normal } else { normal };
            let way = if forward { 1.0 } else { -1.0 } * (edge.to - edge.from).signum();
            let side = outward.cross(circle.derivative(at) * way).dot(axis);
            side * (circle.center - middle).dot(axis) < 0.0
        };
        let beside = |wall: &Face, circle: &Circle| match body.surface(wall.surface) {
            Surface::Cylinder(cylinder) => {
                let gap = sphere.radius - cylinder.radius;
                lined_with(&self.round, self.lined, circle, self.fineness)
                    && lies_on(circle, cylinder, eps)
                    && gap > eps
                    && gap <= sag
            }
            _ => false,
        };
        (face.loops.iter().flatten()).any(|coedge| {
            let edge = body.edge(coedge.edge);
            let Curve::Circle(circle) = body.curve(edge.curve) else {
                return false;
            };
            towards(face, coedge.forward, edge, circle)
                && (body.uses(coedge.edge).into_iter()).any(|(wall, forward)| {
                    let wall = body.face(wall);
                    beside(wall, circle) && towards(wall, forward, edge, circle)
                })
        })
    }
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
