//! The rays a circle of a cone takes besides its grid: its cone's grid, and,
//! where the cone stands close to another wall, the rays of every wall
//! standing with it. A circle lying on a sphere about another line takes
//! the places it crosses the steps of the sphere's grid at (#940): a face of
//! the sphere going round its axis between two such circles is cut open at
//! one of those angles, which every loop must pass at a sample. Not at the
//! angles of the sphere's vertices, which open nothing: a circle ending at
//! one, square to the rim it ends on, touches that angle's half-plane there.
//! Nor where it only touches a step's half-plane, which rounding reads as
//! two crossings a hair apart; nor on a circle standing in a group, whose
//! other circles would not take them, nor where a ray of the group would
//! not be taken. Where its wall stands close to another, as [`contact`]
//! pairs two parallel walls it samples on common rays, they are handed to
//! contact as the wall's own, and every circle of both takes them: a shaft
//! laid onto a ball's touch, its end a hair inside the ball, leaves a ring a
//! hair wide on its end plane between the ball's circle and its own rim, and
//! the rim, sampled on its grid alone, had its chords sag inside the
//! circle's crossings, the ring left open along it (#1109). They come back
//! to the circle among the pair's rays, which it takes at a step its contact
//! withholds too. A wall close to none keeps them on the circle alone:
//! handed to its other circles, they cost a shaft through a ball turned at a
//! slant up to half again its triangles, for nothing.
//!
//! A circle lying on a cone takes the cone's grid and the angles of its
//! vertices ([`cone`]). That leaves a cone's narrow rim with angles of its
//! own grid the wide one has not, and the strip between them, at two
//! consecutive angles, a pentagon rather than a flat trapezoid: which way
//! it is cut into triangles is the sweep's choice. Alone, that costs nothing.
//! A second wall a hair inside the cone — a coaxial cone or cylinder cut from
//! it — has its own pentagons cut the other way, and the skin between the two
//! is thinner than the gap between those choices: the triangles cross. A ring
//! a hair wide between two rims, each sampled on angles the other has not,
//! has the outer one's chords sag inside the inner one's samples, and is left
//! open along it.
//!
//! So a cone with a rim within twice a chord's sag of a parallel wall — as
//! [`contact`] tells two cylinders close — stands in a group with every other
//! rim of it and every coaxial wall within that sag of one of them, joined
//! one through the next. So does a cone with a coaxial circle standing within
//! that sag of its face, read across the axis at the circle's height, between
//! the heights of its rims: a post's rim a hair inside a funnel. The cone's
//! triangles reach from rim to rim, and at the circle's height they are the
//! chords between the rays their rims share; a sample of the circle off those
//! rays stands outside such a chord, and its face pokes through the cone's.
//! Every circle of the group is sampled on every ray any of them takes: its
//! cones' grids and vertices, every wall's own grid, and what the walls take in
//! contact. Every strip of those walls is then flat between the same two rays,
//! and walls a hair apart keep their order: what [`contact`] gives cylinders
//! close to each other, given to the walls of a cone, decided once for them. A
//! cone standing close to nothing keeps its rims as they were: two grids merged
//! leave samples a fraction of a step apart, and a plane all but lying on a
//! cone's face, a slope a hair from square, cut into a sliver between two of
//! them lies on the cone's triangles. Such a plane stands close to the cone
//! along the axis, not across it, and its circles are left out of the group;
//! so is a circle beyond the cone's rims, where the cone has no triangle for it
//! to poke through: a second cone all but square to the axis, a hair long,
//! ending the first.
//!
//! A group's rays are handed to [`contact`] as each wall's own before it
//! shares them: a wall a hair off the axis, close to one of the group, takes
//! them too through its own axis, and the two keep their order. What the
//! walls of the group then take in contact is gathered back onto all of
//! them, and handed to [`contact`] once more when that grew the group: a ray
//! one wall of it took in contact reaches every other wall of it, and a wall
//! a hair off the axis, close to one of those, must take it too. Left out, a
//! slot's end a hair off a cut's wall, which a cone's rim stands on, has its
//! arc sampled apart from the wall's and the sliver between them crossed.
//!
//! [`cone`]: super::cone
//! [`contact`]: super::super::contact

mod joined;
mod spheres;

use std::collections::BTreeMap;
use std::f64::consts::TAU;

use glam::DVec3;

use super::super::contact::{self, Contact, Wall};
use super::cone::{Grid, Spun};
use super::divisions_in;
use super::round::Rounded;
use crate::brep::curve::{Circle, Curve};
use crate::brep::surface::Cylinder;
use crate::brep::topology::{Body, SurfaceId};
use joined::{Joined, along, beside};

/// Walls of one axis standing together, and the directions from it, square
/// to it, every circle of them is sampled along.
struct Group {
    axis: DVec3,
    u: DVec3,
    v: DVec3,
    rays: Vec<DVec3>,
}

impl Group {
    fn angle(&self, way: DVec3) -> f64 {
        way.dot(self.v).atan2(way.dot(self.u))
    }

    /// The rays taken, each square to the axis and of unit length, in the
    /// order of their angle and one of each.
    fn settle(&mut self) {
        let flat = |way: DVec3| (way - self.axis * self.axis.dot(way)).normalize_or_zero();
        let mut rays: Vec<DVec3> = self.rays.iter().map(|way| flat(*way)).collect();
        rays.retain(|way| way.length_squared() > 0.0);
        rays.sort_by(|one, other| self.angle(*one).total_cmp(&self.angle(*other)));
        rays.dedup_by(|later, earlier| self.angle(*later) - self.angle(*earlier) <= ROUNDING);
        self.rays = rays;
    }
}

/// The grid of every cone, sphere and torus, and the walls standing together
/// with one, each in its group, with every group's rays: the grid of each of
/// its cones and the angles of their vertices, the grid of each of its walls,
/// and, once [`Axes::gathered`], the rays its walls take in contact.
pub(super) struct Axes {
    cones: Vec<(Spun, Vec<DVec3>)>,
    /// Each round with the directions from its axis of its grid's steps, but
    /// a sphere whose faces want none of the places its circles of another
    /// line cross them at ([`spheres::opened`]).
    steps: Vec<(Rounded, Vec<DVec3>)>,
    groups: Vec<Group>,
    of_wall: BTreeMap<SurfaceId, usize>,
}

impl Axes {
    pub(super) fn of(
        body: &Body,
        grids: &BTreeMap<SurfaceId, Grid>,
        walls: &[Wall],
        fineness: (f64, f64),
    ) -> Axes {
        let eps = body.scale().eps();
        let (tolerance, span) = fineness;
        let circles: Vec<&Circle> = body
            .edge_ids()
            .filter_map(|id| match body.curve(body.edge(id).curve) {
                Curve::Circle(circle) => Some(circle),
                Curve::Line(_) | Curve::Meet(_) | Curve::Conic(_) => None,
            })
            .collect();
        let mut cones = Vec::new();
        let mut steps = Vec::new();
        let mut rims: Vec<(Vec<SurfaceId>, Vec<SurfaceId>, &Grid)> = Vec::new();
        for (id, grid) in grids {
            cones.push((grid.spun, grid.rays().collect()));
            if let Spun::Round(round) = grid.spun
                && spheres::opened(body, *id, &round, grid.rows, fineness)
            {
                steps.push((round, grid.on_steps().collect()));
            }
            let walls_of = |within: &dyn Fn(&Circle) -> bool| -> Vec<SurfaceId> {
                circles
                    .iter()
                    .filter(|circle| within(circle))
                    .filter_map(|circle| contact::wall_of(circle, walls, eps))
                    .map(|(id, _)| *id)
                    .collect()
            };
            let height =
                |circle: &Circle| (circle.center - grid.spun.origin()).dot(grid.spun.axis());
            let (low, high) = circles
                .iter()
                .filter(|circle| grid.spun.holds(circle, eps))
                .map(|circle| height(circle))
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), at| {
                    (low.min(at), high.max(at))
                });
            let held = walls_of(&|circle| grid.spun.holds(circle, eps));
            let near = walls_of(&|circle| {
                low + eps < height(circle)
                    && height(circle) < high - eps
                    && !grid.spun.holds(circle, eps)
                    && beside(circle, grid, fineness)
            });
            if !held.is_empty() {
                rims.push((held, near, grid));
            }
        }
        let about: Vec<&Wall> = walls
            .iter()
            .filter(|wall| {
                rims.iter()
                    .any(|(_, _, grid)| along(grid.spun.origin(), grid.spun.axis(), wall, eps))
            })
            .collect();
        let (mut joined, close) = Joined::close(&about, walls, eps, fineness);
        rims.retain(|(held, near, _)| !near.is_empty() || held.iter().any(|id| close.contains(id)));
        // A round's rims are kept apart by its rows, and the strip from a rim
        // to the next stands on the round's own columns: a rim given the rays
        // of a wall it stands close to through another rim of the round, not
        // itself, had the strip reach across them where it was a hair thin,
        // at a shaft's rim under a ring's equator beside a rod at its inner
        // one (#1123). Each rim of a round standing close to a wall takes the
        // round's rays in that wall's group alone.
        for (held, _, grid) in &mut rims {
            if let Spun::Round(_) = grid.spun {
                held.retain(|id| close.contains(id));
            }
        }
        for (held, near, grid) in &rims {
            if let Spun::Cone(_) = grid.spun {
                for id in held.iter().chain(near) {
                    joined.join(held[0], *id);
                }
            }
        }
        let mut axes = Axes {
            cones,
            steps,
            groups: Vec::new(),
            of_wall: BTreeMap::new(),
        };
        for (held, _, grid) in rims {
            let rooted = match grid.spun {
                Spun::Cone(_) => &held[..1],
                Spun::Round(_) => &held[..],
            };
            for id in rooted {
                let group = axes.group(joined.root(*id), grid);
                axes.groups[group].rays.extend(grid.rays());
            }
        }
        for (id, wall) in about {
            let Some(&group) = axes.of_wall.get(&joined.root(*id)) else {
                continue;
            };
            axes.of_wall.insert(*id, group);
            let steps = divisions_in(wall.radius, tolerance, span);
            axes.groups[group]
                .rays
                .extend((0..steps).map(|step| wall.radial(TAU * step as f64 / steps as f64)));
        }
        for group in &mut axes.groups {
            group.settle();
        }
        axes
    }

    /// The group whose walls `root` stands for, made for `grid`'s axis when
    /// it is the first.
    fn group(&mut self, root: SurfaceId, grid: &Grid) -> usize {
        if let Some(&group) = self.of_wall.get(&root) {
            return group;
        }
        self.groups.push(Group {
            axis: grid.spun.axis(),
            u: grid.spun.u(),
            v: grid.spun.v(),
            rays: Vec::new(),
        });
        self.of_wall.insert(root, self.groups.len() - 1);
        self.groups.len() - 1
    }

    /// For each wall of a group, the group's rays: what it takes on its own
    /// in contact, besides its vertices.
    pub(super) fn given(&self) -> BTreeMap<SurfaceId, Vec<DVec3>> {
        self.of_wall
            .iter()
            .map(|(id, group)| (*id, self.groups[*group].rays.clone()))
            .collect()
    }

    /// For each wall standing in no group, the places its circles lying on
    /// a sphere about another line cross the steps of the sphere's grid at,
    /// where `kept` says a sample stands on the wall; none for a wall with
    /// no such place.
    pub(super) fn crossed(
        &self,
        body: &Body,
        walls: &[Wall],
        eps: f64,
        kept: impl Fn(&Cylinder, DVec3) -> bool,
    ) -> BTreeMap<SurfaceId, Vec<DVec3>> {
        let mut crossed: BTreeMap<SurfaceId, Vec<DVec3>> = BTreeMap::new();
        for id in body.edge_ids() {
            let Curve::Circle(circle) = body.curve(body.edge(id).curve) else {
                continue;
            };
            let Some((wall, cylinder)) = contact::wall_of(circle, walls, eps) else {
                continue;
            };
            if self.of_wall.contains_key(wall) {
                continue;
            }
            let kept = |way: &DVec3| kept(cylinder, circle.center + *way * circle.radius);
            for way in self.across_spheres(circle, eps).filter(kept) {
                crossed.entry(*wall).or_default().push(way);
            }
        }
        crossed
    }

    /// Every group's rays and every ray one of its walls takes in contact.
    pub(super) fn gathered(mut self, contacts: &BTreeMap<SurfaceId, Contact>) -> Axes {
        for (id, group) in &self.of_wall {
            if let Some(contact) = contacts.get(id) {
                self.groups[*group]
                    .rays
                    .extend(contact.rays.iter().copied());
            }
        }
        for group in &mut self.groups {
            group.settle();
        }
        self
    }

    /// What a circle of `wall` takes besides its grid, `contact`: the rays
    /// of every cone, sphere or torus it lies on, and those of the wall's
    /// group if it stands in one, or else the places it crosses the steps of
    /// a sphere it lies on about another line. None for a circle lying on
    /// none and standing in no group, which takes `contact` alone. A ray of
    /// the group or a crossing is left out where `kept` says no sample
    /// stands, and at a step of the circle's grid `contact` withholds there.
    pub(super) fn widened(
        &self,
        contact: &Contact,
        (circle, wall): (&Circle, SurfaceId),
        (tolerance, span): (f64, f64),
        eps: f64,
        kept: impl Fn(DVec3) -> bool,
    ) -> Option<Contact> {
        let on_cones = self
            .cones
            .iter()
            .filter(|(spun, _)| spun.holds(circle, eps))
            .flat_map(|(_, rays)| rays.iter().copied());
        let group = self.of_wall.get(&wall).map(|group| &self.groups[*group]);
        let steps = divisions_in(circle.radius, tolerance, span);
        let step = TAU / steps as f64;
        let level = circle.center.dot(circle.axis);
        let withheld = |way: &DVec3| {
            let angle = way.dot(circle.v).atan2(way.dot(circle.u));
            let rank = (angle / step).round();
            (angle - rank * step).abs() <= eps / circle.radius
                && contact.withheld.iter().any(|(at, [low, high])| {
                    *at == (rank as i64).rem_euclid(steps as i64) as usize
                        && *low - eps <= level
                        && level <= *high + eps
                })
        };
        let in_group = group
            .into_iter()
            .flat_map(|group| group.rays.iter())
            .filter(|way| !withheld(way) && kept(circle.center + **way * circle.radius))
            .copied();
        let across_spheres = (group.is_none().then(|| self.across_spheres(circle, eps)))
            .into_iter()
            .flatten()
            .filter(|way| !withheld(way) && kept(circle.center + *way * circle.radius));
        let rays: Vec<DVec3> = contact
            .rays
            .iter()
            .copied()
            .chain(on_cones)
            .chain(across_spheres)
            .chain(in_group)
            .collect();
        if rays.len() == contact.rays.len() {
            return None;
        }
        Some(Contact {
            rays,
            withheld: contact.withheld.clone(),
            anchors: contact.anchors.clone(),
            beside: contact.beside.clone(),
            beneath: contact.beneath.clone(),
            oval: contact.oval,
        })
    }
}

/// How close two rays of a group stand, in angle, to be one: far under any
/// gap a circle tells apart, so that two rays a circle would sample apart
/// are both kept.
const ROUNDING: f64 = 1e-14;

#[cfg(test)]
mod tests;
