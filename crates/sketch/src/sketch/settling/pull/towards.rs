//! The point taken as near to where the hand asks as its shape follows: held
//! there, let go of along the hand's way, or the way halved.

use glam::DVec2;

use super::{Holding, PointPull};
use crate::sketch::Sketch;
use crate::sketch::settling::Settled;
use crate::sketch::settling::kept::Kept;

/// How close to the last place a shape can still follow to the way there is
/// halved, as a share of the drawing's size — a hundred-thousandth, the
/// solver's own precision — and how many halvings that may take at the most.
const REACHED_WITHIN: f64 = 1e-5;
const HALVINGS: usize = 24;
/// The least way along the hand's line, as a share of the drawing's size,
/// that counts as having followed the hand at all.
const PROGRESS: f64 = 1e-3;

impl Sketch {
    /// The point held at `landing`, or as near as the shape follows. A curve
    /// brought to or through nothing there leaves no place of the drawing's
    /// own worth resting at: the way is halved up to where it still has a size.
    pub(super) fn towards(
        &mut self,
        pull: &PointPull,
        holding: &Holding,
        landing: DVec2,
        millimeters_per_unit: f64,
    ) -> bool {
        match self.stretch(pull.point, holding, landing, millimeters_per_unit) {
            Settled::Whole => true,
            refused => {
                (refused == Settled::Broken
                    && self.slide_back(pull, holding, landing, millimeters_per_unit))
                    || self.stretch_towards(pull, holding, landing, millimeters_per_unit)
                    || self.retract(pull, holding, landing, millimeters_per_unit)
            }
        }
    }

    /// The point placed at `landing` and let go of along the hand's way: the
    /// drawing pulls it back along that line to the last place its shape can
    /// follow to — one settling, where halving the way takes a dozen. Taken
    /// only when it comes to rest between the press and the hand.
    fn slide_back(
        &mut self,
        pull: &PointPull,
        holding: &Holding,
        landing: DVec2,
        millimeters_per_unit: f64,
    ) -> bool {
        let travel = landing - pull.from;
        let Some(way) = travel.try_normalize() else {
            return false;
        };
        let kept = self.shapes_now();
        self.move_point(pull.point, landing);
        let mut lines = holding.lines.clone();
        lines.push(Kept::level(self, pull.point, way.perp(), 0.0));
        let settled = self.settle_held(holding.pins.clone(), lines, millimeters_per_unit);
        let gone = (self.point(pull.point) - pull.from).dot(way);
        let size = self.drawing_size();
        if settled && gone > size * PROGRESS && gone <= travel.length() + size * REACHED_WITHIN {
            return true;
        }
        self.give_back(kept);
        false
    }

    /// The point held as far towards `landing` as the shape can stretch: the
    /// way there halved until the last place it still follows to.
    fn stretch_towards(
        &mut self,
        pull: &PointPull,
        holding: &Holding,
        landing: DVec2,
        millimeters_per_unit: f64,
    ) -> bool {
        let (mut reached, mut missed) = (0.0, 1.0);
        let within = self.drawing_size() * REACHED_WITHIN / pull.from.distance(landing).max(1e-12);
        for _ in 0..HALVINGS {
            if missed - reached <= within {
                break;
            }
            let halfway = (reached + missed) / 2.0;
            let place = pull.from.lerp(landing, halfway);
            let kept = self.shapes_now();
            match self.stretch(pull.point, holding, place, millimeters_per_unit) {
                Settled::Whole => reached = halfway,
                _ => missed = halfway,
            }
            self.give_back(kept);
        }
        // A hair's breadth past the press is no way at all: the hand's way
        // runs off a curve the point is held to, and letting it go is what
        // brings it round.
        reached * pull.from.distance(landing) > self.drawing_size() * PROGRESS
            && self.stretch(
                pull.point,
                holding,
                pull.from.lerp(landing, reached),
                millimeters_per_unit,
            ) == Settled::Whole
    }
}
