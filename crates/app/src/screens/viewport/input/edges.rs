//! The edge of the part under the hand while the edge panel is open: lit as
//! the hand passes, taken by a click.

use std::borrow::Cow;

use cao_render::MatterInFront;
use cao_solid::{Body, DrawnEdge, EdgeName};
use glam::DVec3;

use crate::screens::blend::BlendState;

use super::super::render::to_screen;
use super::super::section::Section;
use super::super::{EDGE_REACH, ViewportState, to_ndc};

/// How much nearer the eye than a point of an edge, as a share of how far it
/// stands, the matter has to be met to hide it: the faces along the edge are
/// met at the edge itself, give or take their triangles.
const IN_FRONT: f64 = 1e-3;

/// Lights the nearest edge in sight within reach of the hand, and hands it to
/// the panel on a click; a click reaching none leaves the panel's edge as it
/// was.
pub(crate) fn pick_an_edge(
    state: &mut ViewportState,
    response: &egui::Response,
    rect: egui::Rect,
    body: &Body,
    blend: &mut BlendState,
) {
    let under = response
        .hover_pos()
        .and_then(|at| edge_under(state, body, at, rect));
    if response.clicked()
        && let Some(name) = &under
    {
        blend.choose(name.clone());
    }
    state.edge_under_the_hand = under;
}

fn edge_under(
    state: &ViewportState,
    body: &Body,
    at: egui::Pos2,
    rect: egui::Rect,
) -> Option<EdgeName> {
    let seen = |point: DVec3| {
        let view_projection = state.camera.view_projection(rect.aspect_ratio());
        to_screen(point, view_projection, rect)
    };
    let section = state.section_shown();
    let offered = match section {
        None => Offered::Everywhere,
        Some(section) => Offered::by(section, body)?,
    };
    let edges = state.edges.of(body);
    let mut within: Vec<(f32, &DrawnEdge, DVec3)> = edges
        .iter()
        .filter_map(|edge| {
            let segments = edge
                .points
                .windows(2)
                .filter_map(|pair| offered.part_of(pair[0], pair[1]));
            let (distance, point) = nearest_on_screen(seen, segments, at)?;
            (distance <= EDGE_REACH).then_some((distance, edge, point))
        })
        .collect();
    if within.is_empty() {
        return None;
    }
    within.sort_by(|one, other| one.0.total_cmp(&other.0));
    let matter = match section {
        None => Cow::Borrowed(body),
        Some(section) => state.sections.sectioned(section, body).opaque,
    };
    within
        .into_iter()
        .find(|(_, _, point)| in_sight(state, &matter, seen(*point), *point, rect))
        .map(|(_, edge, _)| edge.name.clone())
}

/// Where the points of an edge are offered to the hand: on a side of the
/// section drawn opaque, a place lying on the plane going behind it as the
/// kernel parts the body; everywhere while no section is shown.
enum Offered {
    Everywhere,
    /// Where `towards · p <= offset`.
    Below {
        towards: DVec3,
        offset: f64,
    },
}

impl Offered {
    /// `None` when no side of the section is drawn opaque.
    fn by(section: &Section, body: &Body) -> Option<Self> {
        let plane = section.cutting_plane();
        let towards = plane.normal();
        let offset = towards.dot(plane.origin);
        let offset = offset + body.on_a_plane(offset);
        match (section.behind, section.in_front) {
            (MatterInFront::Opaque, MatterInFront::Opaque) => Some(Self::Everywhere),
            (MatterInFront::Opaque, _) => Some(Self::Below { towards, offset }),
            (_, MatterInFront::Opaque) => Some(Self::Below {
                towards: -towards,
                offset: -offset,
            }),
            _ => None,
        }
    }

    /// The part of the segment from `start` to `end` offered, if any.
    fn part_of(&self, start: DVec3, end: DVec3) -> Option<(DVec3, DVec3)> {
        let Self::Below { towards, offset } = *self else {
            return Some((start, end));
        };
        let (from, to) = (towards.dot(start) - offset, towards.dot(end) - offset);
        let crossing = || start.lerp(end, from / (from - to));
        match (from <= 0.0, to <= 0.0) {
            (true, true) => Some((start, end)),
            (true, false) => Some((start, crossing())),
            (false, true) => Some((crossing(), end)),
            (false, false) => None,
        }
    }
}

/// How far from `at` the edge made of these segments passes on screen, and
/// the point of it that comes nearest.
fn nearest_on_screen(
    seen: impl Fn(DVec3) -> Option<egui::Pos2>,
    segments: impl Iterator<Item = (DVec3, DVec3)>,
    at: egui::Pos2,
) -> Option<(f32, DVec3)> {
    segments
        .filter_map(|(from, to)| {
            let start = seen(from)?;
            let end = seen(to)?;
            let along = end - start;
            let length = along.length_sq();
            let t = match length > 0.0 {
                true => ((at - start).dot(along) / length).clamp(0.0, 1.0),
                false => 0.0,
            };
            let distance = (start + along * t).distance(at);
            Some((distance, from.lerp(to, f64::from(t))))
        })
        .min_by(|one, other| one.0.total_cmp(&other.0))
}

/// Whether nothing of `matter` stands between the eye and this point.
fn in_sight(
    state: &ViewportState,
    matter: &Body,
    on_screen: Option<egui::Pos2>,
    point: DVec3,
    rect: egui::Rect,
) -> bool {
    let Some(on_screen) = on_screen else {
        return false;
    };
    let (origin, direction) = state
        .camera
        .ray(to_ndc(on_screen, rect), rect.aspect_ratio());
    let (origin, direction) = (origin.as_dvec3(), direction.as_dvec3().normalize());
    let along = (point - origin).dot(direction);
    matter
        .ray_hit(origin, direction)
        .is_none_or(|hit| hit.distance >= along - IN_FRONT * along.abs().max(1.0))
}
