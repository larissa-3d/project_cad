//! What holds two points mirror images of each other across an axis.

use glam::DVec2;

use crate::axis::ChosenAxis;
use crate::equation::Equation;
use crate::sketch::Sketch;
use crate::symmetry::Symmetry;

impl Sketch {
    /// Two statements: the middle of the pair lies on the axis, and the pair
    /// stands square across it. A trait serving as the axis is pulled by both,
    /// as the line a point is held on is.
    pub(super) fn symmetric_equations(&self, pair: Symmetry, into: &mut Vec<Equation>) {
        if pair.point.0 >= self.points().len() || pair.twin.0 >= self.points().len() {
            return;
        }
        let (through, towards, ends) = match pair.axis {
            ChosenAxis::Sketch(axis) => (DVec2::ZERO, axis.direction(), None),
            ChosenAxis::Trait(segment) => {
                let Some(line) = self.segments().get(segment.0).copied() else {
                    return;
                };
                let ends = Some((line.start, line.end));
                (self.point(line.start), self.point(line.end), ends)
            }
        };
        let span = towards - through;
        let length = span.length();
        if length < 1e-9 {
            return;
        }
        let unit = span / length;
        let normal = unit.perp();
        let (point, twin) = (self.point(pair.point), self.point(pair.twin));
        let reach = (point + twin) * 0.5 - through;
        let off = span.perp_dot(reach) / length;
        let gap = twin - point;
        let along = gap.dot(unit);

        let mut middle = Equation::new(self.variables());
        middle.error = off;
        middle.add(pair.point, normal * 0.5);
        middle.add(pair.twin, normal * 0.5);
        let mut square = Equation::new(self.variables());
        square.error = along;
        square.add(pair.twin, unit);
        square.add(pair.point, -unit);

        if let Some((start, end)) = ends {
            let turn_middle = (DVec2::new(reach.y, -reach.x) - unit * off) / length;
            middle.add(end, turn_middle);
            middle.add(start, -turn_middle - normal);
            let turn_square = (gap - unit * along) / length;
            square.add(end, turn_square);
            square.add(start, -turn_square);
        }
        into.extend([middle, square]);
    }
}

#[cfg(test)]
mod tests;
