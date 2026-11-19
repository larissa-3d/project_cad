//! What a played case has promised after each of its gestures: the values and
//! rules it was given and the drawing took, read again on the drawing as it
//! now stands; the pieces it made entirely constrained; and whether the solver
//! has moved anything yet.
//!
//! How far a rule stands from holding is worked out here from the drawing's
//! places, never asked of the sketch that is being checked.

use cao_sketch::{Constraint, DimensionTarget, Element, PointId, Sketch, SketchAxis};
use glam::DVec2;

use super::Gesture;
use super::checking::Promised;
use super::greening::pieces;
use super::holding::{SCALE, Taken, dimension_aimed, lay_rule, measured, type_value};
use super::settling::{Piece, Promise};

/// A value or a rule the drawing took.
#[derive(Clone, Debug)]
enum Kept {
    Value {
        target: DimensionTarget,
        near: DVec2,
    },
    Rule {
        rule: Constraint,
        near: DVec2,
        /// For a fixed element, where its points stood when it was fixed.
        stood: Vec<(PointId, DVec2)>,
    },
}

/// A piece made entirely constrained, by the points it was named by.
#[derive(Clone, Debug)]
struct Claim {
    points: Vec<PointId>,
    green_without: Vec<bool>,
}

/// What a case has promised so far, carried from one gesture to the next.
#[derive(Default)]
pub(super) struct Promises {
    kept: Vec<Kept>,
    claims: Vec<Claim>,
    moved: bool,
}

impl Promises {
    /// Lays a gesture and says what the case has promised once it is laid.
    pub(super) fn lay(&mut self, gesture: &Gesture, sketch: &mut Sketch) -> Promised {
        let before: Vec<DVec2> = sketch.points().to_vec();
        let mut refused = Vec::new();
        match gesture {
            Gesture::Dimension { .. } | Gesture::Rule { .. } => {
                self.take(gesture, sketch, &mut refused);
            }
            Gesture::Green { piece, given } => {
                let named = named_piece(sketch, piece);
                let mut values = Vec::new();
                let mut all_taken = true;
                for one in given {
                    match self.take(one, sketch, &mut refused) {
                        (_, Some(Kept::Value { target, .. })) => values.push(target),
                        (taken, _) => all_taken &= taken,
                    }
                }
                if all_taken && let Some(named) = named {
                    let points = grouped(sketch, &named);
                    let green_without = values
                        .iter()
                        .map(|target| {
                            let mut without = sketch.clone();
                            without.erase_dimension(*target);
                            let settled = without.settled_points(SCALE);
                            points.iter().all(|point| settled[point.0])
                        })
                        .collect();
                    self.claims.push(Claim {
                        points: named,
                        green_without,
                    });
                }
            }
            other => other.lay(sketch),
        }
        if gesture.constrains() || gesture.drags() {
            self.moved |= moved(&before, sketch.points());
        }
        self.promised(sketch, refused)
    }

    /// Lays a dimension or a rule, and keeps it when the drawing took it. A
    /// refusal is written down with where the drawing's points stood before
    /// it and after. Two points joined are taken, and keep nothing to read.
    fn take(
        &mut self,
        gesture: &Gesture,
        sketch: &mut Sketch,
        refused: &mut Vec<Promise>,
    ) -> (bool, Option<Kept>) {
        let kept = match gesture {
            Gesture::Dimension { clicks, value } => {
                let Some(target) = dimension_aimed(sketch, clicks) else {
                    return (false, None);
                };
                let before = live_places(sketch);
                match type_value(sketch, target, *value) {
                    Taken::Held => Kept::Value {
                        target,
                        near: DVec2::from(clicks[0]),
                    },
                    taken => {
                        if taken == Taken::Refused {
                            refused.push(Promise::Refused {
                                before,
                                after: live_places(sketch),
                            });
                        }
                        return (false, None);
                    }
                }
            }
            Gesture::Rule { rule, clicks } => {
                let before = live_places(sketch);
                let (taken, laid) = lay_rule(sketch, *rule, clicks);
                if taken == Taken::Refused {
                    refused.push(Promise::Refused {
                        before,
                        after: live_places(sketch),
                    });
                }
                let Some(rule) = laid.filter(|_| taken == Taken::Held) else {
                    return (taken == Taken::Joined, None);
                };
                Kept::Rule {
                    rule,
                    near: DVec2::from(clicks[0]),
                    stood: stood(sketch, rule),
                }
            }
            _ => return (false, None),
        };
        if let Kept::Value { target, .. } = &kept {
            self.kept.retain(
                |earlier| !matches!(earlier, Kept::Value { target: laid, .. } if laid == target),
            );
        }
        self.kept.push(kept.clone());
        (true, Some(kept))
    }

    fn promised(&self, sketch: &Sketch, refused: Vec<Promise>) -> Promised {
        let mut promises: Vec<Promise> = self
            .kept
            .iter()
            .filter_map(|kept| reread(sketch, kept))
            .collect();
        promises.extend(refused);
        let (pieces, whole) = match self.claims.is_empty() {
            true => (Vec::new(), None),
            false => self.pieces(sketch),
        };
        Promised {
            promises,
            pieces,
            whole,
            moved: self.moved,
        }
    }

    fn pieces(&self, sketch: &Sketch) -> (Vec<Piece>, Option<bool>) {
        let settled = sketch.settled_points(SCALE);
        let alive: Vec<PointId> = sketch.live_points().map(|(point, _)| point).collect();
        let held: Vec<Piece> = self
            .claims
            .iter()
            .filter_map(|claim| {
                let named: Vec<PointId> = claim
                    .points
                    .iter()
                    .copied()
                    .filter(|point| alive.contains(point))
                    .collect();
                let points = grouped(sketch, &named);
                (!points.is_empty()).then(|| Piece {
                    points: points
                        .iter()
                        .map(|point| (sketch.point(*point), settled[point.0]))
                        .collect(),
                    green_without: claim.green_without.clone(),
                })
            })
            .collect();
        let claimed: Vec<PointId> = self
            .claims
            .iter()
            .flat_map(|claim| claim.points.iter().copied())
            .collect();
        let every_piece = pieces(sketch)
            .iter()
            .all(|piece| piece.iter().any(|point| claimed.contains(point)));
        let whole = every_piece.then(|| sketch.is_fully_constrained(SCALE));
        (held, whole)
    }
}

/// The points of the pieces of the drawing that stand wholly at a piece's
/// places, when there is one at every place and no more: a case shrunk out of the
/// drawing a piece was made green on names none, rather than a piece it never
/// gave what green takes. Two points laid on top of each other are told apart
/// by the piece each belongs to.
fn named_piece(sketch: &Sketch, places: &[[f64; 2]]) -> Option<Vec<PointId>> {
    let at_a_place = |point: PointId| {
        let place = sketch.point(point);
        places.iter().any(|at| {
            let at = DVec2::from(*at);
            at.distance(place) <= 1e-9 * (1.0 + at.abs().max_element())
        })
    };
    let named: Vec<PointId> = pieces(sketch)
        .into_iter()
        .filter(|piece| piece.iter().all(|point| at_a_place(*point)))
        .flatten()
        .collect();
    let every_place = places.iter().all(|at| {
        let at = DVec2::from(*at);
        named
            .iter()
            .any(|point| sketch.point(*point).distance(at) <= 1e-9 * (1.0 + at.abs().max_element()))
    });
    (!named.is_empty() && every_place && named.len() == places.len()).then_some(named)
}

/// The points of the pieces the named points belong to, as the drawing now
/// joins them.
fn grouped(sketch: &Sketch, named: &[PointId]) -> Vec<PointId> {
    pieces(sketch)
        .into_iter()
        .filter(|piece| piece.iter().any(|point| named.contains(point)))
        .flatten()
        .collect()
}

fn live_places(sketch: &Sketch) -> Vec<DVec2> {
    sketch.live_points().map(|(_, at)| at).collect()
}

/// Whether a point moved, or one came or went: more than the rounding a
/// value the drawing already measures leaves behind.
fn moved(before: &[DVec2], after: &[DVec2]) -> bool {
    before.len() != after.len()
        || before
            .iter()
            .zip(after)
            .any(|(from, to)| from.distance(*to) > 1e-9 * (1.0 + from.abs().max_element()))
}

fn stood(sketch: &Sketch, rule: Constraint) -> Vec<(PointId, DVec2)> {
    let Constraint::Fixed { element } = rule else {
        return Vec::new();
    };
    let points = match element {
        Element::Point(point) => vec![point],
        Element::Segment(side) => {
            let line = sketch.segments()[side.0];
            vec![line.start, line.end]
        }
        Element::Circle(circle) => vec![sketch.circle(circle).center],
        Element::Arc(arc) => {
            let drawn = sketch.arcs()[arc.0];
            vec![drawn.center, drawn.start, drawn.end]
        }
        Element::Ellipse(ellipse) => sketch.ellipse_points(ellipse).to_vec(),
    };
    points
        .into_iter()
        .map(|point| (point, sketch.point(point)))
        .collect()
}

/// What a value or a rule kept says of the drawing as it now stands, or
/// nothing once it is no longer on it.
fn reread(sketch: &Sketch, kept: &Kept) -> Option<Promise> {
    match kept {
        Kept::Value { target, near } => {
            let laid = sketch.dimension_of(*target).filter(|laid| !laid.driven)?;
            Some(Promise::Measured {
                near: *near,
                value: laid.value,
                measured: measured(sketch, *target)?,
            })
        }
        Kept::Rule { rule, near, stood } => {
            if !sketch.carries(*rule) {
                return None;
            }
            Some(Promise::Ruled {
                near: *near,
                off_by: off_by(sketch, *rule, stood)?,
            })
        }
    }
}

fn direction(sketch: &Sketch, side: cao_sketch::SegmentId) -> (DVec2, DVec2) {
    let (from, to) = sketch.endpoints(side);
    (from, to - from)
}

fn off_the_line(sketch: &Sketch, side: cao_sketch::SegmentId, at: DVec2) -> f64 {
    let (from, way) = direction(sketch, side);
    way.normalize_or_zero().perp_dot(at - from).abs()
}

fn axis_way(axis: SketchAxis) -> DVec2 {
    axis.direction()
}

/// How far the drawing stands from a rule, in its own lengths: nought when it
/// holds. Nothing for the rules a cut carries, whose places are the cut's own.
fn off_by(sketch: &Sketch, rule: Constraint, stood: &[(PointId, DVec2)]) -> Option<f64> {
    let length = |side| sketch.segment_length(side);
    let round = |at: DVec2, centre: DVec2, radius: f64| (at.distance(centre) - radius).abs();
    Some(match rule {
        Constraint::Perpendicular { first, second } => {
            let (one, other) = (direction(sketch, first).1, direction(sketch, second).1);
            one.normalize_or_zero().dot(other.normalize_or_zero()).abs()
                * length(first).min(length(second))
        }
        Constraint::Parallel { first, second } => {
            let (one, other) = (direction(sketch, first).1, direction(sketch, second).1);
            one.normalize_or_zero()
                .perp_dot(other.normalize_or_zero())
                .abs()
                * length(first).min(length(second))
        }
        Constraint::Equal { first, second } => (length(first) - length(second)).abs(),
        Constraint::EqualRadius { first, second } => {
            (sketch.circle(first).radius - sketch.circle(second).radius).abs()
        }
        Constraint::EqualRadiusArc { first, second } => {
            (sketch.arc_radius(first) - sketch.arc_radius(second)).abs()
        }
        Constraint::EqualRadiusArcCircle { arc, circle } => {
            (sketch.arc_radius(arc) - sketch.circle(circle).radius).abs()
        }
        Constraint::OnSegment { point, segment, .. } => {
            off_the_line(sketch, segment, sketch.point(point))
        }
        Constraint::Collinear { first, second } => {
            let (from, to) = sketch.endpoints(second);
            off_the_line(sketch, first, from).max(off_the_line(sketch, first, to))
        }
        Constraint::Tangent {
            circle, segment, ..
        } => {
            let round = sketch.circle(circle);
            (off_the_line(sketch, segment, sketch.point(round.center)) - round.radius).abs()
        }
        Constraint::ArcTangent { arc, segment, .. } => {
            let centre = sketch.point(sketch.arcs()[arc.0].center);
            (off_the_line(sketch, segment, centre) - sketch.arc_radius(arc)).abs()
        }
        Constraint::OnCircle { point, circle, .. } => {
            let held = sketch.circle(circle);
            round(sketch.point(point), sketch.point(held.center), held.radius)
        }
        Constraint::OnArc { point, arc, .. } => {
            let centre = sketch.point(sketch.arcs()[arc.0].center);
            round(sketch.point(point), centre, sketch.arc_radius(arc))
        }
        Constraint::EllipseTangent {
            ellipse, segment, ..
        } => {
            let drawn = sketch.ellipse_draft(ellipse);
            let (from, way) = direction(sketch, segment);
            let normal = way.normalize_or_zero().perp();
            let across = drawn.first.normalize_or_zero().perp() * drawn.second;
            let reach = (normal.dot(drawn.first).powi(2) + normal.dot(across).powi(2)).sqrt();
            let gap = normal.dot(drawn.centre - from).abs();
            (gap - reach).abs()
        }
        Constraint::OnEllipse { point, ellipse, .. } => {
            let drawn = sketch.ellipse_draft(ellipse);
            let (long, short) = (drawn.first.length(), drawn.second);
            let way = drawn.first.normalize_or_zero();
            let at = sketch.point(point) - drawn.centre;
            let (x, y) = (way.dot(at) / long, way.perp_dot(at) / short);
            ((x * x + y * y).sqrt() - 1.0).abs() * long.min(short)
        }
        Constraint::OnAxis { point, axis } => axis_way(axis).perp_dot(sketch.point(point)).abs(),
        Constraint::Midpoint { point, segment } => {
            let (from, to) = sketch.endpoints(segment);
            sketch.point(point).distance((from + to) / 2.0)
        }
        Constraint::AxisCollinear { segment, axis } => {
            let (from, to) = sketch.endpoints(segment);
            let off = |at: DVec2| axis_way(axis).perp_dot(at).abs();
            off(from).max(off(to))
        }
        Constraint::AxisParallel { segment, axis } => {
            axis_way(axis).perp_dot(direction(sketch, segment).1).abs()
        }
        Constraint::AxisPerpendicular { segment, axis } => {
            axis_way(axis).dot(direction(sketch, segment).1).abs()
        }
        Constraint::Fixed { .. } => stood
            .iter()
            .map(|(point, at)| sketch.point(*point).distance(*at))
            .fold(0.0, f64::max),
        Constraint::AtCutPoint { .. }
        | Constraint::OnCut { .. }
        | Constraint::CutTangent { .. } => {
            return None;
        }
    })
}
