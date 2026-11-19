//! Two points held mirror images of each other across an axis: what keeps a
//! mirrored copy the image of what it was made from.

use serde::{Deserialize, Serialize};

use crate::axis::ChosenAxis;
use crate::constraints::Constraint;
use crate::sketch::{Element, PointId, Sketch};

/// A point and its twin, each the reflection of the other across the axis.
///
/// Neither drives the other: the rule pulls both, and a trait serving as the
/// axis with them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symmetry {
    pub point: PointId,
    pub twin: PointId,
    pub axis: ChosenAxis,
}

impl Sketch {
    /// The piece a mirror laid as the image of this one, or the one this is the
    /// image of: what goes with it when it is erased.
    ///
    /// Nothing for a piece no symmetry speaks of, nor for one that is its own
    /// image, lying on the axis.
    pub fn twin_of(&self, element: Element) -> Option<Element> {
        let points = self.points_it_leans_on(element);
        self.symmetries()
            .filter(|pair| points.contains(&pair.point) || points.contains(&pair.twin))
            .find_map(|pair| {
                let image: Vec<PointId> = points
                    .iter()
                    .map(|point| self.image_of(*point, pair.axis))
                    .collect();
                self.standing_on(element, &image)
            })
    }

    fn symmetries(&self) -> impl Iterator<Item = Symmetry> + '_ {
        self.constraints().iter().filter_map(|rule| match rule {
            Constraint::Symmetric(pair) => Some(*pair),
            _ => None,
        })
    }

    /// A point's twin across the axis, or the point itself when no symmetry
    /// about that axis names it: a point shared by both halves.
    fn image_of(&self, point: PointId, axis: ChosenAxis) -> PointId {
        self.symmetries()
            .filter(|pair| pair.axis == axis)
            .find_map(|pair| match (pair.point == point, pair.twin == point) {
                (true, _) => Some(pair.twin),
                (_, true) => Some(pair.point),
                _ => None,
            })
            .unwrap_or(point)
    }

    /// Another live piece of the same kind standing on exactly these points.
    fn standing_on(&self, element: Element, points: &[PointId]) -> Option<Element> {
        let candidates: Vec<Element> = match element {
            Element::Point(_) => points.iter().copied().map(Element::Point).collect(),
            Element::Segment(_) => self
                .live_segments()
                .map(|(id, _)| Element::Segment(id))
                .collect(),
            Element::Circle(_) => self
                .live_circles()
                .map(|(id, _)| Element::Circle(id))
                .collect(),
            Element::Arc(_) => self.live_arcs().map(|(id, _)| Element::Arc(id)).collect(),
            Element::Ellipse(_) => self
                .live_ellipses()
                .map(|(id, _)| Element::Ellipse(id))
                .collect(),
        };
        candidates.into_iter().find(|candidate| {
            let stands_on = self.points_it_leans_on(*candidate);
            *candidate != element
                && stands_on.len() == points.len()
                && stands_on.iter().all(|point| points.contains(point))
                && points.iter().all(|point| stands_on.contains(point))
        })
    }
}
