//! The gestures that make the solver move a drawing — a value typed on what
//! the dimension tool picked, a rule the constraint tool laid, a drag — laid
//! the way the application and `cao_part` lay them.
//!
//! A unit of the drawing stands for a millimetre: the first value a part is
//! given fixes its scale in `cao_part`, out of reach here, and a value that
//! only set the scale would move nothing.

use cao_sketch::{
    CurveDrag, DimensionMode, DimensionPick, DimensionPicks, DimensionTarget, Element,
    LengthOutcome, PointId, Pulled, Rule, RuleIntent, RulePick, SideDrag, Sketch, SnapSettings,
    Support, axis_under, measure_pick, rule_intent,
};
use glam::DVec2;

use super::laying::{ON_A_POINT, UNDER_A_PLACE, reach};

pub(super) const SCALE: f64 = 1.0;

/// No magnet pulls a gesture of a campaign: its places are where it acts.
const NO_MAGNET: SnapSettings = SnapSettings {
    point_reach: 0.0,
    curve_reach: 0.0,
    grid_step: None,
    grid_reach: 0.0,
};

fn near(at: DVec2) -> f64 {
    reach(at, UNDER_A_PLACE)
}

/// What the drawing measures for a dimension now, a unit for a millimetre.
pub(super) fn measured(sketch: &Sketch, target: DimensionTarget) -> Option<f64> {
    let alive = |point: PointId| point.0 < sketch.points().len();
    match target {
        DimensionTarget::Length(segment) => {
            (segment.0 < sketch.segments().len()).then(|| sketch.segment_length(segment))
        }
        DimensionTarget::Distance { from, to } => {
            (alive(from) && alive(to)).then(|| sketch.point(from).distance(sketch.point(to)))
        }
        DimensionTarget::Radius(circle) => {
            (circle.0 < sketch.circles().len()).then(|| sketch.circle(circle).radius)
        }
        DimensionTarget::Diameter(circle) => {
            (circle.0 < sketch.circles().len()).then(|| sketch.circle(circle).radius * 2.0)
        }
        DimensionTarget::Angle { .. }
        | DimensionTarget::AngleBetween { .. }
        | DimensionTarget::AxisAngle { .. } => sketch.opening(target),
        DimensionTarget::PointToSegment { point, segment } => {
            sketch.point_to_segment(point, segment)
        }
        DimensionTarget::Projected { from, to, axis } => sketch.projected_gap(from, to, axis),
        DimensionTarget::ArcRadius(arc) => {
            (arc.0 < sketch.arcs().len()).then(|| sketch.arc_radius(arc))
        }
        DimensionTarget::ArcSweep(arc) => {
            (arc.0 < sketch.arcs().len()).then(|| sketch.arc_sweep(arc).to_degrees())
        }
    }
}

/// What the smart dimension tool holds once clicked at each place in turn,
/// the last click putting it down (`measure.rs` in the application): the
/// dimension to type a value into, a new one or one already there.
///
/// A click on a point a fillet or a chamfer offers, where the drawing has
/// none, hangs one there first, as the tool does.
pub(super) fn dimension_aimed(sketch: &mut Sketch, clicks: &[[f64; 2]]) -> Option<DimensionTarget> {
    let mut picks = DimensionPicks::default();
    let mut placing: Option<DimensionTarget> = None;
    let mut opened: Option<DimensionTarget> = None;
    for click in clicks {
        let at = DVec2::from(*click);
        let snap = near(at);
        if let Some(target) = placing {
            if let Some(refined) = sketch.refine(target, at, snap) {
                placing = Some(refined);
                continue;
            }
            return Some(sketch.oriented(target, at));
        }
        hang_on_a_cut_point(sketch, at, snap);
        let (now, pick) = measure_pick(sketch, DimensionMode::Auto, picks, at, snap);
        picks = now;
        if let DimensionPick::Target(target) = pick {
            let laid_already = !target.is_angle()
                && !sketch.is_slanted(target)
                && sketch.dimension_of(target).is_some();
            match laid_already {
                true => {
                    opened = Some(target);
                    if let DimensionTarget::Length(segment) = target {
                        picks = DimensionPicks {
                            first_angle_segment: Some(segment),
                            ..DimensionPicks::default()
                        };
                    }
                }
                false => placing = Some(target),
            }
        }
    }
    opened
}

fn hang_on_a_cut_point(sketch: &mut Sketch, at: DVec2, snap: f64) {
    if sketch.nearest_point(at, snap * 0.8).is_some() {
        return;
    }
    let Some((cut, offered)) = sketch.nearest_cut_point(at, snap * 0.8) else {
        return;
    };
    let Some(place) = sketch.cut_point(cut, offered) else {
        return;
    };
    let point = sketch.add_point(place);
    sketch.hold(point, Support::CutPoint(cut, offered));
}

/// A value typed for a dimension, as `cao_part` applies it once its scale is
/// fixed: nought or less refused but for an angle to an axis, a value saying
/// nothing new kept as a readout, and one the drawing cannot fully honour
/// refused, the drawing as it was.
pub(super) fn type_value(sketch: &mut Sketch, target: DimensionTarget, value: f64) {
    if !target.takes(value) {
        return;
    }
    if value <= 0.0 && !matches!(target, DimensionTarget::AxisAngle { .. }) {
        return;
    }
    let Some(measure) = measured(sketch, target) else {
        return;
    };
    let readout = match sketch.dimension_of(target) {
        Some(laid) => laid.driven,
        None => sketch.would_be_redundant(target, SCALE),
    };
    if readout {
        sketch.set_dimension(target, measure, true);
        return;
    }
    let before = sketch.clone();
    sketch.set_dimension(target, value, false);
    if sketch.land_value(target, SCALE) != LengthOutcome::Exact {
        *sketch = before;
    }
}

/// What a click of the constraint tool picks (`constrain.rs` in the
/// application): a point, the origin only for a coincidence; a point a cut
/// offers, for a coincidence; a fillet's curve, for a tangent; a trait, a
/// circle, an arc, an ellipse; one of the axes, for the three rules that take
/// one.
fn rule_pick(sketch: &Sketch, at: DVec2, rule: Rule) -> Option<RulePick> {
    let snap = near(at);
    let point = sketch
        .nearest_point(at, snap * 0.8)
        .filter(|point| !sketch.is_origin(*point) || rule == Rule::Coincident);
    if point.is_none()
        && rule == Rule::Coincident
        && let Some((cut, offered)) = sketch.nearest_cut_point(at, snap * 0.8)
    {
        return Some(RulePick::CutPoint(cut, offered));
    }
    if point.is_none()
        && rule == Rule::Tangent
        && let Some(cut) = sketch.cut_before_traits(at, snap)
    {
        return Some(RulePick::CutCurve(cut));
    }
    point
        .map(Element::Point)
        .or_else(|| sketch.nearest_segment(at, snap).map(Element::Segment))
        .or_else(|| sketch.nearest_circle(at, snap).map(Element::Circle))
        .or_else(|| sketch.nearest_arc(at, snap).map(Element::Arc))
        .or_else(|| sketch.nearest_ellipse(at, snap).map(Element::Ellipse))
        .map(RulePick::Element)
        .or_else(|| {
            matches!(rule, Rule::Collinear | Rule::Parallel | Rule::Perpendicular)
                .then(|| axis_under(at, snap).map(RulePick::Axis))
                .flatten()
        })
}

/// What the constraint tool holds once clicked at each place in turn: a
/// click on nothing, or on something already picked, adds nothing, and the
/// tool lays its rule as soon as it has enough.
pub(super) fn rule_picks(sketch: &Sketch, rule: Rule, clicks: &[[f64; 2]]) -> Vec<RulePick> {
    let mut picks = Vec::new();
    for click in clicks {
        if picks.len() == rule.arity() {
            break;
        }
        if let Some(picked) = rule_pick(sketch, DVec2::from(*click), rule)
            && !picks.contains(&picked)
        {
            picks.push(picked);
        }
    }
    picks
}

/// The rule laid as the constraint tool and `cao_part` lay it: a rule already
/// carried is not laid again, one that cannot hold is refused with the
/// drawing as it was, and two points brought together are joined.
pub(super) fn lay_rule(sketch: &mut Sketch, rule: Rule, clicks: &[[f64; 2]]) {
    let picks = rule_picks(sketch, rule, clicks);
    if picks.len() < rule.arity() {
        return;
    }
    match rule_intent(rule, &picks, sketch) {
        Some(RuleIntent::Constrain(constraint)) if !sketch.carries(constraint) => {
            let before = sketch.clone();
            if sketch.lay_rule(constraint, SCALE) == LengthOutcome::BestEffort {
                *sketch = before;
            }
        }
        Some(RuleIntent::Merge { kept, dropped }) => {
            sketch.join_points(kept, dropped, SCALE);
        }
        _ => {}
    }
}

/// A press at `from` dragged to `to` and let go, with nothing selected and no
/// key held (`dragging.rs` in the application): a point is moved, the centre
/// of an arc or an ellipse carries its curve, and a press on no point pulls
/// the side or the curve under it — across, a side travels and a curve is
/// drawn to another size; along, the shape turns.
pub(super) fn drag(sketch: &mut Sketch, from: DVec2, to: DVec2) {
    let snap = near(from);
    let settled = sketch.settled_points(SCALE);
    let grip = sketch
        .point_to_drag_at(from, snap)
        .filter(|grip| !sketch.is_origin(grip.point))
        .filter(|grip| {
            !settled.get(grip.point.0).copied().unwrap_or(false)
                && sketch.holds_on(grip.point).len() <= 1
        });
    match grip {
        Some(grip) => {
            let mut group = sketch.arc_ends_around(grip.point);
            group.extend(sketch.ellipse_ends_around(grip.point));
            match group.is_empty() {
                true => {
                    let held_off = match grip.from_afar {
                        true => sketch.point(grip.point) - from,
                        false => DVec2::ZERO,
                    };
                    move_point(sketch, grip.point, to + held_off);
                }
                false => {
                    group.insert(0, grip.point);
                    move_many(sketch, &group, to - from);
                }
            }
        }
        None => match sketch.pulled_at(from, snap, SCALE) {
            Some(Pulled::Side(side)) => {
                let pulled = sketch.side_drag(side, from, to, &NO_MAGNET, SCALE);
                let mut settling = sketch.clone();
                let outcome = match &pulled {
                    SideDrag::Across { by } if by.length() > 1e-9 => {
                        settling.move_side(side, *by, SCALE)
                    }
                    SideDrag::Along(turn) if turn.angle.abs() > 1e-12 => {
                        settling.turn_shape(&turn.points, turn.about, turn.angle, SCALE)
                    }
                    _ => return,
                };
                if outcome == LengthOutcome::Exact {
                    *sketch = settling;
                }
            }
            Some(Pulled::Curve(curve)) => {
                match sketch.curve_drag(curve, from, to, to, &NO_MAGNET, SCALE) {
                    CurveDrag::Resize { reach } => {
                        sketch.resize(curve, reach, SCALE);
                    }
                    CurveDrag::Along { turn, reach } => {
                        let mut settling = sketch.clone();
                        let turned =
                            settling.turn_shape(&turn.points, turn.about, turn.angle, SCALE);
                        let resized = reach
                            .map(|reach| (reach, settling.resize_in_place(curve, reach, SCALE)));
                        if turned == LengthOutcome::Exact && turn.angle.abs() > 1e-12 {
                            sketch.turn_shape(&turn.points, turn.about, turn.angle, SCALE);
                        }
                        if let Some((reach, LengthOutcome::Exact)) = resized {
                            sketch.resize(curve, reach, SCALE);
                        }
                    }
                }
            }
            None => {}
        },
    }
}

/// A point let go of at `hand`, as the drop writes it and `cao_part` replays
/// it: settled the way its pull allows, held on what it was dropped on, or
/// joined to the point it was dropped onto.
fn move_point(sketch: &mut Sketch, point: PointId, hand: DVec2) {
    let pull = sketch.pull(point, SCALE);
    let landing = pull.landing(sketch, hand, hand, false, &NO_MAGNET);
    let mut settling = sketch.clone();
    settling.settle_pulled(&pull, landing, SCALE);
    let arrived = pull.arrived(&settling, landing);
    let joined = pull.joins(&settling, landing, reach(landing, ON_A_POINT), SCALE);
    let on = match joined.is_some() || !arrived {
        true => Vec::new(),
        false => held_at_drop(sketch, &settling, point, landing),
    };
    if joined.is_none() && on.is_empty() && !reshaped(sketch, &settling) {
        return;
    }
    let pull = sketch.pull(point, SCALE);
    sketch.settle_pulled(&pull, landing, SCALE);
    for support in on {
        sketch.hold(point, support);
    }
    if let Some(kept) = joined {
        sketch.join_points(kept, point, SCALE);
    }
}

fn move_many(sketch: &mut Sketch, points: &[PointId], by: DVec2) {
    let dropped: Vec<(PointId, DVec2)> = points
        .iter()
        .map(|point| (*point, sketch.point(*point) + by))
        .collect();
    let mut settling = sketch.clone();
    settling.settle_around_all(&dropped, SCALE);
    if by.length() >= 1e-9 && reshaped(sketch, &settling) {
        sketch.settle_around_all(&dropped, SCALE);
    }
}

/// `landing.rs` in the application: what ran through the place before the
/// drag and still does once settled, at most a crossing's two, never what
/// held the point already but a middle.
fn held_at_drop(before: &Sketch, settled: &Sketch, point: PointId, landing: DVec2) -> Vec<Support> {
    let dropped_on = |sketch: &Sketch| {
        let mut on = sketch.supports_for(point, landing);
        on.truncate(2);
        on
    };
    let held = before.holds_on(point);
    let was_there = dropped_on(before);
    dropped_on(settled)
        .into_iter()
        .filter(|support| was_there.contains(support))
        .filter(|support| held.is_empty() || matches!(support, Support::Middle(_)))
        .filter(|support| !held.contains(support))
        .collect()
}

fn reshaped(before: &Sketch, after: &Sketch) -> bool {
    before.points() != after.points()
        || before
            .circles()
            .iter()
            .zip(after.circles())
            .any(|(one, other)| one.radius != other.radius)
}
