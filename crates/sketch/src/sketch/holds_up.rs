//! Whether a rule still speaks of a drawing that has it.
//!
//! Erasing a piece of the drawing takes with it every rule that named it: a
//! rule left behind would hold a number against something nobody can see, and
//! the solver would go on satisfying it. What each kind of rule needs is one
//! question — is what it names still there — asked of the kinds it names.

use super::{CircleId, Constraint, Element, PointId, SegmentId, Sketch};
use crate::arc::ArcId;
use crate::axis::ChosenAxis;
use crate::corner_cut::CutPoint;
use crate::ellipse::EllipseId;

impl Sketch {
    /// Whether everything a rule speaks of is still drawn.
    pub(super) fn holds_up(&self, constraint: Constraint) -> bool {
        let segment = |id: SegmentId| id.0 < self.segments.len() && !self.is_erased_segment(id);
        let circle = |id: CircleId| id.0 < self.circles.len() && !self.is_erased_circle(id);
        let point = |id: PointId| id.0 < self.points.len() && !self.is_erased_point(id);
        let axis_stands = |axis: ChosenAxis| match axis {
            ChosenAxis::Trait(axis) => segment(axis),
            ChosenAxis::Sketch(_) => true,
        };
        match constraint {
            Constraint::Perpendicular { first, second }
            | Constraint::Parallel { first, second }
            | Constraint::Equal { first, second }
            | Constraint::Collinear { first, second } => {
                first != second && segment(first) && segment(second)
            }
            Constraint::EqualRadius { first, second } => {
                first != second && circle(first) && circle(second)
            }
            Constraint::EqualRadiusArc { .. }
            | Constraint::EqualRadiusArcCircle { .. }
            | Constraint::ArcTangent { .. } => self.arc_rule_holds_up(constraint),
            Constraint::EllipseTangent {
                ellipse,
                segment: line,
                ..
            } => {
                ellipse.0 < self.ellipses().len()
                    && !self.is_erased_ellipse(ellipse)
                    && segment(line)
            }
            Constraint::OnSegment { .. }
            | Constraint::OnCircle { .. }
            | Constraint::OnArc { .. }
            | Constraint::OnEllipse { .. }
            | Constraint::OnAxis { .. }
            | Constraint::AtCutPoint { .. }
            | Constraint::OnCut { .. } => self.hold_holds_up(constraint),
            Constraint::Midpoint {
                point: held,
                segment: on,
            } => point(held) && segment(on),
            Constraint::Tangent {
                circle: round,
                segment: line,
                ..
            } => circle(round) && segment(line),
            // A fillet's curve has a circle to brush; a chamfer's has none.
            Constraint::CutTangent {
                cut, segment: line, ..
            } => segment(line) && self.cut_offers(cut, CutPoint::Centre),
            Constraint::AxisCollinear { segment: on, .. }
            | Constraint::AxisParallel { segment: on, .. }
            | Constraint::AxisPerpendicular { segment: on, .. } => segment(on),
            Constraint::Symmetric(pair) => {
                point(pair.point) && point(pair.twin) && axis_stands(pair.axis)
            }
            Constraint::SymmetricCircles(pair) => {
                pair.circle != pair.twin
                    && circle(pair.circle)
                    && circle(pair.twin)
                    && axis_stands(pair.axis)
            }
            Constraint::Fixed { element } => match element {
                Element::Point(held) => point(held),
                Element::Segment(held) => segment(held),
                Element::Circle(held) => circle(held),
                Element::Arc(held) => held.0 < self.arcs.len() && !self.is_erased_arc(held),
                Element::Ellipse(held) => !self.is_erased_ellipse(held),
            },
        }
    }

    /// Whether a rule puts a curve's own centre on the curve, or has the curve
    /// touch something there or touch a trait running through it — one that
    /// ends there, or that the centre is held on or in the middle of: it could
    /// only hold with the curve shrunk to nothing, and the solver writes no row
    /// for it. `same` says which points are one, so that a merge can ask it of
    /// the drawing it would leave.
    pub(super) fn on_its_own_centre(
        &self,
        rule: Constraint,
        same: impl Fn(PointId, PointId) -> bool,
    ) -> bool {
        let circle = |id: CircleId| self.circles.get(id.0).map(|circle| circle.center);
        let arc = |id: ArcId| self.arcs.get(id.0).copied();
        let ellipse = |id: EllipseId| self.ellipses.get(id.0).map(|ellipse| ellipse.center);
        let (point, centre, segment) = match rule {
            Constraint::OnCircle {
                point, circle: on, ..
            } => (Some(point), circle(on), None),
            Constraint::OnArc { point, arc: on, .. } => match arc(on) {
                Some(curve) if same(curve.center, curve.start) || same(curve.center, curve.end) => {
                    return false;
                }
                curve => (Some(point), curve.map(|curve| curve.center), None),
            },
            Constraint::OnEllipse {
                point, ellipse: on, ..
            } => (Some(point), ellipse(on), None),
            Constraint::Tangent {
                circle: on,
                segment,
                at,
                ..
            } => (at, circle(on), Some(segment)),
            Constraint::ArcTangent {
                arc: on,
                segment,
                at,
                ..
            } => (at, arc(on).map(|curve| curve.center), Some(segment)),
            Constraint::EllipseTangent {
                ellipse: on,
                segment,
                at,
                ..
            } => (at, ellipse(on), Some(segment)),
            _ => return false,
        };
        let Some(centre) = centre else {
            return false;
        };
        point.is_some_and(|point| same(point, centre))
            || segment.is_some_and(|segment| self.held_through(segment, centre, &same))
    }

    /// Whether a trait runs through a point: ends there, or holds it on itself
    /// or at its middle by a rule.
    fn held_through(
        &self,
        segment: SegmentId,
        point: PointId,
        same: &impl Fn(PointId, PointId) -> bool,
    ) -> bool {
        let ends = self
            .segments
            .get(segment.0)
            .is_some_and(|line| same(line.start, point) || same(line.end, point));
        ends || self.constraints.iter().any(|rule| match *rule {
            Constraint::OnSegment {
                point: held,
                segment: on,
                ..
            }
            | Constraint::Midpoint {
                point: held,
                segment: on,
            } => on == segment && same(held, point),
            _ => false,
        })
    }
}
