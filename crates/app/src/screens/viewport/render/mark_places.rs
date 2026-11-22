//! Where the marks of the rules stand on screen: each in its box, clear of
//! the traits and the corner it marks, set side by side where two would
//! overlap. The painter and the pick both read it, so a mark is taken where it
//! is seen.

use cao_sketch::{Constraint, Sketch};
use glam::DVec2;

use super::overlays::{MARK_BOX, MARK_CLEARS, MARK_REACH, MARK_SPACING, to_screen};

/// Where each rule's mark is written on screen, one entry per mark: its box
/// clear of the traits through the place it marks, so that a click on a trait
/// is the trait's.
///
/// Several rules can hold the same corner — a middle and a right angle, say —
/// so marks that would land on each other are set side by side.
pub(crate) fn placed_marks(
    sketch: &Sketch,
    view_projection: glam::Mat4,
    rect: egui::Rect,
) -> Vec<(Constraint, egui::Pos2)> {
    let screen = |at: DVec2| to_screen(sketch.plane.to_world(at), view_projection, rect);
    let mut placed: Vec<(Constraint, egui::Pos2)> = Vec::new();
    for constraint in sketch.constraints() {
        for at in sketch.rule_marks(*constraint) {
            let Some(position) = screen(at) else {
                continue;
            };
            let clear = match corner_of(sketch, *constraint) {
                Some(corner) => screen(corner)
                    .map(|here| here + clear_of(&arms_at(sketch, corner, here, screen))),
                None => Some(position + clear_of(&arms_at(sketch, at, position, screen))),
            };
            let Some(mut position) = clear else {
                continue;
            };
            while placed.iter().any(|(_, held)| {
                (held.x - position.x).abs() < MARK_SPACING
                    && (held.y - position.y).abs() < MARK_SPACING
            }) {
                position.x += MARK_SPACING;
            }
            placed.push((*constraint, position));
        }
    }
    placed
}

/// The corner a right angle is written in, when its two traits meet: its box
/// stands in it as near as clearing the corner and both its sides allows,
/// rather than where the drawing marks it, further in.
fn corner_of(sketch: &Sketch, rule: Constraint) -> Option<DVec2> {
    let Constraint::Perpendicular { first, second } = rule else {
        return None;
    };
    let (corner, _, _) = sketch.corner_points(first, second)?;
    Some(corner)
}

/// The ways the traits through `at` leave it on screen, `here`: one for a
/// trait ending there, two for one running through.
fn arms_at(
    sketch: &Sketch,
    at: DVec2,
    here: egui::Pos2,
    screen: impl Fn(DVec2) -> Option<egui::Pos2>,
) -> Vec<egui::Vec2> {
    let mut arms = Vec::new();
    for (id, _) in sketch.live_segments() {
        let (start, end) = sketch.endpoints(id);
        let span = end - start;
        if span.length_squared() <= 0.0 {
            continue;
        }
        let along = (at - start).dot(span) / span.length_squared();
        let near = 1e-9 * span.length().max(1.0);
        if (start + span * along).distance(at) > near || !(-1e-9..=1.0 + 1e-9).contains(&along) {
            continue;
        }
        for (far, reach) in [(start, along), (end, 1.0 - along)] {
            if let Some(far) = screen(far).filter(|_| reach > 1e-9) {
                arms.push((far - here).normalized());
            }
        }
    }
    arms
}

/// Where a mark's box stands from the place it marks, on screen: beside a
/// trait running through it, inside the corner two traits make there, behind
/// the end of one — clear of each, every way the box can lie against it — and
/// up and to the right where no trait runs.
fn clear_of(arms: &[egui::Vec2]) -> egui::Vec2 {
    let half = MARK_BOX / 2.0;
    let around = half * std::f32::consts::SQRT_2 + MARK_CLEARS;
    let up_and_right = egui::vec2(1.0, -1.0).normalized() * around;
    match arms {
        [] => up_and_right,
        [one, other] if one.dot(*other) < -0.999 => {
            let normal = one.rot90();
            let normal = match normal.y > 0.0 || (normal.y == 0.0 && normal.x > 0.0) {
                true => -normal,
                false => normal,
            };
            normal * room(normal)
        }
        // Wide enough for the box to sit in the corner, as the redesign draws
        // a right angle's.
        [one, other] if (*one - *other).length() >= 1.0 => {
            let sine_of_half = (*one - *other).length() / 2.0;
            let inside = (*one + *other).normalized();
            inside * (room(one.rot90()).max(room(other.rot90())) / sine_of_half)
        }
        _ => {
            let behind = -arms.iter().fold(egui::Vec2::ZERO, |sum, arm| sum + *arm);
            match behind.length() > 1e-3 {
                true => behind.normalized() * around,
                false => up_and_right,
            }
        }
    }
}

/// How far from a line square to `normal` a box's centre has to stand for the
/// box to keep clear of it.
fn room(normal: egui::Vec2) -> f32 {
    MARK_BOX / 2.0 * (normal.x.abs() + normal.y.abs()) + MARK_CLEARS
}

/// The rule whose mark is under `pointer`: on the surface the mark is drawn
/// on, and no further, so that just beside it is still the trait.
pub(crate) fn mark_under(
    placed: &[(Constraint, egui::Pos2)],
    pointer: egui::Pos2,
) -> Option<Constraint> {
    placed
        .iter()
        .find(|(_, at)| {
            (at.x - pointer.x).abs() <= MARK_REACH && (at.y - pointer.y).abs() <= MARK_REACH
        })
        .map(|(rule, _)| *rule)
}
