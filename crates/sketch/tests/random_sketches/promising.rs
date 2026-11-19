//! What a played case has promised after each of its gestures: the values and
//! rules it was given and the drawing took, read again on the drawing as it
//! now stands; the pieces it made entirely constrained; and whether the solver
//! has moved anything yet.
//!
//! How far a rule stands from holding is worked out here from the drawing's
//! places, never asked of the sketch that is being checked. A fixed element
//! promises nothing: it gives way when nothing else can (`docs/sketch.md`).

use cao_sketch::{
    ChosenAxis, CircleSymmetry, Constraint, DimensionTarget, PointId, Sketch, SketchAxis, Symmetry,
};
use glam::DVec2;

use super::Gesture;
use super::checking::Promised;
use super::greening::{pieces, radius_free};
use super::holding::{SCALE, Taken, dimension_aimed, lay_rule, measured, type_value};
use super::settling::{Piece, Promise};

/// A value or a rule the drawing took.
#[derive(Clone, Debug)]
enum Kept {
    Value {
        number: usize,
        target: DimensionTarget,
        near: DVec2,
    },
    Rule {
        rule: Constraint,
        near: DVec2,
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
    values_kept: usize,
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
                        (_, Some(Kept::Value { number, .. })) => values.push(number),
                        (taken, _) => all_taken &= taken,
                    }
                }
                if all_taken
                    && let Some(named) = named
                    && is_green(sketch, &named)
                {
                    let green_without = self.green_without(sketch, &named, &values);
                    self.claims.push(Claim {
                        points: named,
                        green_without,
                    });
                }
            }
            Gesture::Mirror { .. } => {
                let laid_before = sketch.constraints().len();
                gesture.lay(sketch);
                let symmetries: Vec<Constraint> = sketch.constraints()[laid_before..]
                    .iter()
                    .copied()
                    .filter(|rule| {
                        matches!(
                            rule,
                            Constraint::Symmetric(_) | Constraint::SymmetricCircles(_)
                        )
                    })
                    .collect();
                for rule in symmetries {
                    let near = match rule {
                        Constraint::Symmetric(Symmetry { point, .. }) => sketch.point(point),
                        Constraint::SymmetricCircles(CircleSymmetry { circle, .. }) => {
                            sketch.point(sketch.circle(circle).center)
                        }
                        _ => continue,
                    };
                    self.kept.push(Kept::Rule { rule, near });
                }
            }
            other => other.lay(sketch),
        }
        if gesture.constrains() || gesture.drags() {
            self.moved |= moved(&before, sketch.points());
        }
        self.follow_joins(sketch);
        self.promised(sketch, refused)
    }

    /// For each value a piece was given that still drives the drawing, whether
    /// the piece would still be green without it: every point settled, and
    /// the size of every circle held.
    fn green_without(&self, sketch: &Sketch, named: &[PointId], values: &[usize]) -> Vec<bool> {
        let points = grouped(sketch, named);
        self.kept
            .iter()
            .filter_map(|kept| match kept {
                Kept::Value { number, target, .. } if values.contains(number) => Some(*target),
                _ => None,
            })
            .filter(|target| {
                sketch
                    .dimension_of(*target)
                    .is_some_and(|laid| !laid.driven)
            })
            .map(|target| {
                let mut without = sketch.clone();
                without.erase_dimension(target);
                let settled = without.settled_points(SCALE);
                points.iter().all(|point| settled[point.0]) && !radius_free(&without, &points)
            })
            .collect()
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
                    Taken::Held => {
                        self.values_kept += 1;
                        Kept::Value {
                            number: self.values_kept,
                            target,
                            near: DVec2::from(clicks[0]),
                        }
                    }
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
                    self.follow_joins(sketch);
                    return (taken == Taken::Joined, None);
                };
                Kept::Rule {
                    rule,
                    near: DVec2::from(clicks[0]),
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

    /// Two points joined into one carry their values over to the point that
    /// stays: a value kept is followed onto the one now laid in its place, a
    /// distance between the same things but for a point that is gone.
    fn follow_joins(&mut self, sketch: &Sketch) {
        let alive: Vec<PointId> = sketch.live_points().map(|(point, _)| point).collect();
        for index in 0..self.kept.len() {
            let Kept::Value { target, .. } = self.kept[index] else {
                continue;
            };
            if sketch.dimension_of(target).is_some() {
                continue;
            }
            let followed = sketch
                .dimensions()
                .iter()
                .map(|laid| laid.target)
                .find(|laid| {
                    redirected_from(target, *laid, &alive)
                    && !self.kept.iter().any(
                        |other| matches!(other, Kept::Value { target: kept, .. } if kept == laid),
                    )
                });
            if let (Some(laid), Kept::Value { target, .. }) = (followed, &mut self.kept[index]) {
                *target = laid;
            }
        }
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
        let whole =
            (every_piece && sketch.holds_a_drawing()).then(|| sketch.is_fully_constrained(SCALE));
        (held, whole)
    }
}

/// Whether a piece is green as the drawing now stands: every point of it
/// settled, and the size of every circle of it held. A piece whose values
/// lean on a value of the case that is gone, shrunk away, is not.
fn is_green(sketch: &Sketch, named: &[PointId]) -> bool {
    let points = grouped(sketch, named);
    let settled = sketch.settled_points(SCALE);
    points.iter().all(|point| settled[point.0]) && !radius_free(sketch, &points)
}

/// Whether `laid` is `kept` with some of its points, gone since, replaced by
/// others: what joining two points does to a value on one of them.
fn redirected_from(kept: DimensionTarget, laid: DimensionTarget, alive: &[PointId]) -> bool {
    let follows = |was: PointId, now: PointId| was == now || !alive.contains(&was);
    match (kept, laid) {
        (
            DimensionTarget::Distance { from, to },
            DimensionTarget::Distance {
                from: now_from,
                to: now_to,
            },
        ) => follows(from, now_from) && follows(to, now_to),
        (
            DimensionTarget::PointToSegment { point, segment },
            DimensionTarget::PointToSegment {
                point: now,
                segment: now_segment,
            },
        ) => segment == now_segment && follows(point, now),
        (
            DimensionTarget::Projected { from, to, axis },
            DimensionTarget::Projected {
                from: now_from,
                to: now_to,
                axis: now_axis,
            },
        ) => axis == now_axis && follows(from, now_from) && follows(to, now_to),
        _ => false,
    }
}

/// The points of the one piece of the drawing that stands wholly at a
/// piece's places, with one point at every place and no more: a case shrunk
/// out of the drawing a piece was made green on names none, rather than a
/// piece it never gave what green takes. Two points laid on top of each other
/// are told apart by the piece each belongs to.
fn named_piece(sketch: &Sketch, places: &[[f64; 2]]) -> Option<Vec<PointId>> {
    let on = |at: [f64; 2], place: DVec2| {
        let at = DVec2::from(at);
        at.distance(place) <= 1e-9 * (1.0 + at.abs().max_element())
    };
    let standing: Vec<Vec<PointId>> = pieces(sketch)
        .into_iter()
        .filter(|piece| {
            piece
                .iter()
                .all(|point| places.iter().any(|at| on(*at, sketch.point(*point))))
        })
        .collect();
    let [piece] = standing.as_slice() else {
        return None;
    };
    let every_place = places
        .iter()
        .all(|at| piece.iter().any(|point| on(*at, sketch.point(*point))));
    (every_place && piece.len() == places.len()).then(|| piece.clone())
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

/// The arm an angle is read on: the shorter of its traits, or an arc's
/// radius. A turn of so many radians moves its end by that much times the arm.
fn arm(sketch: &Sketch, target: DimensionTarget) -> Option<f64> {
    let length = |side: cao_sketch::SegmentId| sketch.segment_length(side);
    match target {
        DimensionTarget::Angle { first, second, .. }
        | DimensionTarget::AngleBetween { first, second, .. } => {
            Some(length(first).min(length(second)))
        }
        DimensionTarget::AxisAngle { segment, .. } => Some(length(segment)),
        DimensionTarget::ArcSweep(arc) => Some(sketch.arc_radius(arc)),
        _ => None,
    }
}

/// What a value or a rule kept says of the drawing as it now stands, or
/// nothing once it is no longer on it.
fn reread(sketch: &Sketch, kept: &Kept) -> Option<Promise> {
    match kept {
        Kept::Value { target, near, .. } => {
            let laid = sketch.dimension_of(*target).filter(|laid| !laid.driven)?;
            let measure = measured(sketch, *target)?;
            Some(match target.is_angle() {
                true => Promise::Turned {
                    near: *near,
                    degrees: laid.value,
                    measured: measure,
                    arm: arm(sketch, *target)?,
                },
                false => Promise::Measured {
                    near: *near,
                    value: laid.value,
                    measured: measure,
                },
            })
        }
        Kept::Rule { rule, near } => {
            if !sketch.carries(*rule) {
                return None;
            }
            Some(Promise::Ruled {
                near: *near,
                off_by: off_by(sketch, *rule)?,
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

/// A place a mirror's axis runs through, and its direction, of length one.
fn axis_line(sketch: &Sketch, axis: ChosenAxis) -> (DVec2, DVec2) {
    match axis {
        ChosenAxis::Sketch(axis) => (DVec2::ZERO, axis_way(axis)),
        ChosenAxis::Trait(side) => {
            let (from, way) = direction(sketch, side);
            (from, way.normalize_or_zero())
        }
    }
}

/// How far a sketch stands from a rule, read the way a promise reads it:
/// nought when it holds, nothing for a rule that makes no promise.
pub fn how_far_from(sketch: &Sketch, rule: Constraint) -> Option<f64> {
    off_by(sketch, rule)
}

/// How far the drawing stands from a rule, in its own lengths: nought when it
/// holds. Nothing for the rules a cut carries, whose places are the cut's own,
/// nor for a fixed element.
fn off_by(sketch: &Sketch, rule: Constraint) -> Option<f64> {
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
        Constraint::Symmetric(Symmetry { point, twin, axis }) => {
            let (through, way) = axis_line(sketch, axis);
            let (one, other) = (sketch.point(point), sketch.point(twin));
            let middle_off = way.perp_dot((one + other) / 2.0 - through).abs();
            let pair_along = way.dot(other - one).abs();
            middle_off.max(pair_along)
        }
        Constraint::SymmetricCircles(CircleSymmetry { circle, twin, .. }) => {
            (sketch.circle(circle).radius - sketch.circle(twin).radius).abs()
        }
        Constraint::Fixed { .. }
        | Constraint::AtCutPoint { .. }
        | Constraint::OnCut { .. }
        | Constraint::CutTangent { .. } => {
            return None;
        }
    })
}
