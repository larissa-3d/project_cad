//! How far a drag may throw a point: read on the drawing as the press found
//! it, and judged at every try of the drag. A landing may hold every rule; a
//! point sent far beyond the drawing is still not what the hand asked for, and
//! the try is refused as a cut corner turned inside out is (#597). Nor may it
//! bring a curve to nothing whose centre slides along a trait to its pinned
//! end (#1065).

use glam::DVec2;

use super::PointPull;
use crate::length::LengthOutcome;
use crate::sketch::settling::Kept;
use crate::sketch::{PointId, SegmentId, Sketch};

/// What a curve's centre slid along a trait to its pinned end may leave of
/// the curve, against the size the press found it at: an order of magnitude
/// down, and no further. Below it the hand stops short; a second
/// drag, read from the smaller curve, takes it on.
const LEFT_OF_A_CURVE: f64 = 0.1;

/// Where the press found every point, and how far from there any point may
/// end, the dragged one held to it like any other: twice the sum of the hand's
/// way and the drawing's size. A shape stretched after a corner taken far
/// carries its points as far as the hand went, never farther; a shape turned
/// about a point near the hand swings its far side by up to twice the
/// drawing's size.
#[derive(Clone, Debug)]
pub(crate) struct Throw {
    press: Vec<DVec2>,
    /// Each centre and pinned end a trait from the dragged centre runs to, with
    /// how far apart the press found them, and the axis such an end ends.
    radii: Vec<(PointId, PointId, f64)>,
    axes: Vec<SegmentId>,
    bound: f64,
}

impl Sketch {
    /// Lays one place of the drag down: the point taken towards `position`
    /// and the drawing settled around it, as `pull` allows.
    ///
    /// `Exact` when the drawing settled, the point wherever it could go;
    /// `BestEffort` when nothing could be had and the drawing is given back
    /// as it was. Every frame is laid on the drawing as the press found it,
    /// which is where the bound of the drag's throw is read.
    pub fn settle_pulled(
        &mut self,
        pull: &PointPull,
        position: DVec2,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let ends = self.ends_slid_towards(pull.point);
        let outer = self.held.throw.replace(Throw {
            press: self.points().to_vec(),
            radii: ends
                .iter()
                .map(|(end, _)| (pull.point, *end, pull.from.distance(self.point(*end))))
                .collect(),
            axes: ends.iter().filter_map(|(_, axis)| *axis).collect(),
            bound: 2.0 * (position.distance(pull.from) + self.drawing_size()),
        });
        let outcome = self.lay_pulled(pull, position, millimeters_per_unit);
        self.held.throw = outer;
        outcome
    }

    /// Whether the drag under way has thrown a point, the dragged one
    /// included, past its bound. Never outside a drag.
    pub(crate) fn throws_a_point(&self) -> bool {
        let Some(throw) = &self.held.throw else {
            return false;
        };
        self.live_points().any(|(point, place)| {
            throw
                .press
                .get(point.0)
                .is_some_and(|was| place.distance(*was) > throw.bound)
        })
    }

    /// Whether the drag under way has slid a curve's centre so near a pinned
    /// end it runs to that the curve keeps less than it may of its size — an
    /// arc's radius, an ellipse's half-axis, read against the press. Never
    /// outside a drag.
    pub(crate) fn shrinks_a_curve(&self) -> bool {
        self.held.throw.as_ref().is_some_and(|throw| {
            throw.radii.iter().any(|(centre, end, was)| {
                self.point(*centre).distance(self.point(*end)) < was * LEFT_OF_A_CURVE
            })
        })
    }

    /// Whether one of `lines` keeps an axis such an end ends, and came out the
    /// other way round: the ellipse went through nothing to get there.
    pub(crate) fn turns_an_axis_round(&self, lines: &[Kept]) -> bool {
        self.held.throw.as_ref().is_some_and(|throw| {
            lines.iter().any(|line| {
                line.segment.is_some_and(|axis| throw.axes.contains(&axis))
                    && line.turned_round(self)
            })
        })
    }

    /// The pinned ends — fixed, or the origin — of the curves centred on
    /// `dragged` that a trait from `dragged` runs to: the centre slides along
    /// that trait towards the end, which the curve gives way around (#1057).
    /// Each with the axis it ends, on an ellipse.
    fn ends_slid_towards(&self, dragged: PointId) -> Vec<(PointId, Option<SegmentId>)> {
        let pinned = self.points_that_stay();
        let joined = |end: PointId| {
            self.live_segments().any(|(_, line)| {
                (line.start, line.end) == (dragged, end) || (line.start, line.end) == (end, dragged)
            })
        };
        let arcs = self
            .live_arcs()
            .filter(|(_, arc)| arc.center == dragged)
            .flat_map(|(_, arc)| [(arc.start, None), (arc.end, None)]);
        let ellipses = self
            .live_ellipses()
            .filter(|(_, ellipse)| ellipse.center == dragged)
            .flat_map(|(id, ellipse)| {
                let [_, ends @ ..] = self.ellipse_points(id);
                let axes = [ellipse.first, ellipse.first, ellipse.second, ellipse.second];
                ends.into_iter().zip(axes.map(Some))
            });
        arcs.chain(ellipses)
            .filter(|(end, _)| pinned.get(end.0).copied().unwrap_or(false) && joined(*end))
            .collect()
    }
}
