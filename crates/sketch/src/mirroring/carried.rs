//! The rules a mirrored copy takes from its original.

use crate::arc::ArcId;
use crate::axis::ChosenAxis;
use crate::constraints::Constraint;
use crate::duplicating::Pairing;
use crate::ellipse::EllipseId;
use crate::sketch::{CircleId, Element, PointId, SegmentId};

impl Pairing {
    /// The rule the copy holds where the original held `rule`, when every
    /// element the rule speaks of has an image.
    ///
    /// A rule to the sketch's own axes comes along only across one of them,
    /// which each map onto itself: across a trait, a horizontal's image is
    /// horizontal only while the trait stands upright. A fixed element is left
    /// behind, the copy being held by the symmetry already, and a symmetry
    /// laid by an earlier mirror leans on an axis that has no image here.
    pub(super) fn carried(&self, rule: Constraint, across: ChosenAxis) -> Option<Constraint> {
        let to_the_axes = matches!(across, ChosenAxis::Sketch(_));
        Some(match rule {
            Constraint::Perpendicular { first, second } => Constraint::Perpendicular {
                first: self.segment(first)?,
                second: self.segment(second)?,
            },
            Constraint::Parallel { first, second } => Constraint::Parallel {
                first: self.segment(first)?,
                second: self.segment(second)?,
            },
            Constraint::Equal { first, second } => Constraint::Equal {
                first: self.segment(first)?,
                second: self.segment(second)?,
            },
            Constraint::Collinear { first, second } => Constraint::Collinear {
                first: self.segment(first)?,
                second: self.segment(second)?,
            },
            Constraint::EqualRadius { first, second } => Constraint::EqualRadius {
                first: self.circle(first)?,
                second: self.circle(second)?,
            },
            Constraint::EqualRadiusArc { first, second } => Constraint::EqualRadiusArc {
                first: self.arc(first)?,
                second: self.arc(second)?,
            },
            Constraint::EqualRadiusArcCircle { arc, circle } => Constraint::EqualRadiusArcCircle {
                arc: self.arc(arc)?,
                circle: self.circle(circle)?,
            },
            Constraint::OnSegment {
                point,
                segment,
                from,
            } => Constraint::OnSegment {
                point: self.point(point)?,
                segment: self.segment(segment)?,
                from,
            },
            Constraint::OnCircle {
                point,
                circle,
                from,
            } => Constraint::OnCircle {
                point: self.point(point)?,
                circle: self.circle(circle)?,
                from,
            },
            Constraint::OnArc { point, arc, from } => Constraint::OnArc {
                point: self.point(point)?,
                arc: self.arc(arc)?,
                from,
            },
            Constraint::OnEllipse {
                point,
                ellipse,
                from,
            } => Constraint::OnEllipse {
                point: self.point(point)?,
                ellipse: self.ellipse(ellipse)?,
                from,
            },
            Constraint::Midpoint { point, segment } => Constraint::Midpoint {
                point: self.point(point)?,
                segment: self.segment(segment)?,
            },
            // A contact left out of the copy is laid afresh with the tangency.
            Constraint::Tangent {
                circle,
                segment,
                at,
                from,
            } => Constraint::Tangent {
                circle: self.circle(circle)?,
                segment: self.segment(segment)?,
                at: at.and_then(|at| self.point(at)),
                from,
            },
            Constraint::ArcTangent {
                arc,
                segment,
                at,
                from,
            } => Constraint::ArcTangent {
                arc: self.arc(arc)?,
                segment: self.segment(segment)?,
                at: at.and_then(|at| self.point(at)),
                from,
            },
            Constraint::EllipseTangent {
                ellipse,
                segment,
                at,
                from,
            } => Constraint::EllipseTangent {
                ellipse: self.ellipse(ellipse)?,
                segment: self.segment(segment)?,
                at: at.and_then(|at| self.point(at)),
                from,
            },
            Constraint::OnAxis { point, axis } if to_the_axes => Constraint::OnAxis {
                point: self.point(point)?,
                axis,
            },
            Constraint::AxisCollinear { segment, axis } if to_the_axes => {
                Constraint::AxisCollinear {
                    segment: self.segment(segment)?,
                    axis,
                }
            }
            Constraint::AxisParallel { segment, axis } if to_the_axes => Constraint::AxisParallel {
                segment: self.segment(segment)?,
                axis,
            },
            Constraint::AxisPerpendicular { segment, axis } if to_the_axes => {
                Constraint::AxisPerpendicular {
                    segment: self.segment(segment)?,
                    axis,
                }
            }
            Constraint::OnAxis { .. }
            | Constraint::AxisCollinear { .. }
            | Constraint::AxisParallel { .. }
            | Constraint::AxisPerpendicular { .. }
            | Constraint::AtCutPoint { .. }
            | Constraint::OnCut { .. }
            | Constraint::CutTangent { .. }
            | Constraint::Fixed { .. }
            | Constraint::Symmetric(_) => return None,
        })
    }

    fn point(&self, of: PointId) -> Option<PointId> {
        self.points
            .iter()
            .find(|(was, _)| *was == of)
            .map(|(_, now)| *now)
    }

    fn image(&self, of: Element) -> Option<Element> {
        self.elements
            .iter()
            .find(|(was, _)| *was == of)
            .map(|(_, now)| *now)
    }

    fn segment(&self, of: SegmentId) -> Option<SegmentId> {
        match self.image(Element::Segment(of))? {
            Element::Segment(now) => Some(now),
            _ => None,
        }
    }

    fn circle(&self, of: CircleId) -> Option<CircleId> {
        match self.image(Element::Circle(of))? {
            Element::Circle(now) => Some(now),
            _ => None,
        }
    }

    fn arc(&self, of: ArcId) -> Option<ArcId> {
        match self.image(Element::Arc(of))? {
            Element::Arc(now) => Some(now),
            _ => None,
        }
    }

    fn ellipse(&self, of: EllipseId) -> Option<EllipseId> {
        match self.image(Element::Ellipse(of))? {
            Element::Ellipse(now) => Some(now),
            _ => None,
        }
    }
}
