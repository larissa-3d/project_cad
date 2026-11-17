//! What the step panel shows while it is open: what it holds, as named cases
//! the view puts into words, and the step whose volume it previews.

use cao_part::{ExtrusionMode, Formula, Operation, PartDocument, RevolutionAxis};

use super::{ExtrusionState, Shape};

/// What the panel of a step of matter holds.
pub struct Panel {
    pub shape: Shape,
    pub sketch: usize,
    /// The areas chosen, as ranks among the sketch's, in the order chosen.
    pub areas: Vec<usize>,
    pub matter: ExtrusionMode,
    pub extent: Extent,
    pub around_an_axis: bool,
    /// Whether « Autour d'un axe » may change the shape: a step reopened
    /// keeps its own.
    pub shape_can_change: bool,
    pub reversed: bool,
    pub applies: bool,
}

/// How far the step goes, by what the value comes to as typed: nothing when
/// it does not read.
#[derive(Debug, PartialEq)]
pub enum Extent {
    Depth {
        millimetres: Option<f64>,
    },
    Turn {
        degrees: Option<f64>,
        axis: RevolutionAxis,
    },
}

/// A step of matter as applying would record it, and whether it adds matter
/// or takes it away.
pub struct Preview {
    pub operation: Operation,
    pub mode: ExtrusionMode,
}

impl ExtrusionState {
    /// The panel of the step being set, once the tool is armed on a sketch.
    pub fn panel(&self, document: &PartDocument) -> Option<Panel> {
        let (Some(sketch), Some(matter)) = (self.sketch, self.mode) else {
            return None;
        };
        let variables = document.variables();
        let typed = |input: &str| variables.size_of(input).ok().map(|(_, value)| value);
        let extent = match self.shape {
            Shape::Straight => Extent::Depth {
                millimetres: typed(&self.distance_input),
            },
            Shape::Revolution => Extent::Turn {
                degrees: typed(&self.angle_input),
                axis: self.axis,
            },
        };
        Some(Panel {
            shape: self.shape,
            sketch,
            areas: document.areas_standing(sketch, &self.chosen_areas(document, sketch)),
            matter,
            extent,
            around_an_axis: self.is_revolving(),
            shape_can_change: self.reopened.is_none(),
            reversed: self.reversed,
            applies: self.is_ready(variables),
        })
    }

    /// The step applying would record now, as the view draws it while the
    /// panel is open: nothing until the tool is armed on a sketch and ready.
    pub fn preview(&self, document: &PartDocument) -> Option<Preview> {
        let (Some(sketch), Some(mode)) = (self.sketch, self.mode) else {
            return None;
        };
        let variables = document.variables();
        if !self.is_ready(variables) {
            return None;
        }
        let areas = self.chosen_areas(document, sketch);
        let operation = if self.is_revolving() {
            Operation::Revolve {
                sketch,
                areas,
                axis: self.axis,
                angle: self
                    .angle(variables)
                    .map_or(Formula::Number(0.0), |(written, _)| written),
                mode,
            }
        } else {
            Operation::Extrude {
                sketch,
                areas,
                distance: self
                    .distance(variables)
                    .map_or(Formula::Number(0.0), |(written, _)| written),
                mode,
            }
        };
        Some(Preview { operation, mode })
    }
}
