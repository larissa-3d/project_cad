use crate::axis::ChosenAxis;
use crate::constraints::Constraint;
use crate::duplicating::Duplicated;
use crate::element::Element;
use crate::holding::Support;
use crate::sketch::{PointId, SegmentId, Sketch};
use crate::symmetry::Symmetry;

impl Sketch {
    /// Copies the elements given to the other side of an axis, each copied
    /// point held symmetric to its original about it.
    ///
    /// A point lying on the axis is not copied: both halves end on it, and it
    /// is held on the axis, its own mirror image wherever it slides along it.
    ///
    /// Nothing when the axis names no direction.
    pub fn mirror(&mut self, of: &[Element], axis: ChosenAxis) -> Option<Duplicated> {
        let (through, along) = self.axis_line(axis)?;
        let (made, pairs) = self.duplicate_paired(
            of,
            |at| {
                let offset = at - through;
                through + along * 2.0 * offset.dot(along) - offset
            },
            true,
        );
        for (point, twin) in pairs {
            let rule = match (point == twin, axis) {
                (false, _) => Constraint::Symmetric(Symmetry { point, twin, axis }),
                (true, ChosenAxis::Sketch(on)) => Support::Axis(on).holding(point),
                (true, ChosenAxis::Trait(on)) if self.is_an_end_of(point, on) => continue,
                (true, ChosenAxis::Trait(on)) => Support::Segment(on).holding(point),
            };
            self.add_constraint(rule);
        }
        Some(made)
    }

    fn is_an_end_of(&self, point: PointId, segment: SegmentId) -> bool {
        let line = self.segments()[segment.0];
        line.start == point || line.end == point
    }
}

#[cfg(test)]
mod tests;
