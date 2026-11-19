//! A piece made entirely constrained: what it is, and what it takes to turn
//! green — attached to the origin, its shape values, which way round it lies
//! — and nothing more. Of what it could be given, only what takes some freedom
//! away from it is kept, so the sketch's own count of freedom decides what is
//! already held.

use cao_sketch::{
    ArcId, CircleId, EllipseId, PointId, Rule, SegmentId, Sketch, SketchAxis, Support,
};

use super::asking::{Ask, Asker, Target};
use super::holding::SCALE;
use super::laying::reach;

/// What each curve of the drawing joins: its own points, and those held on
/// it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Curve {
    Segment(SegmentId),
    Circle(CircleId),
    Arc(ArcId),
    Ellipse(EllipseId),
}

pub(super) fn curves_and_points(sketch: &Sketch) -> Vec<(Curve, Vec<PointId>)> {
    let mut curves: Vec<(Curve, Vec<PointId>)> = Vec::new();
    curves.extend(
        sketch
            .live_segments()
            .map(|(id, side)| (Curve::Segment(id), vec![side.start, side.end])),
    );
    curves.extend(
        sketch
            .live_circles()
            .map(|(id, round)| (Curve::Circle(id), vec![round.center])),
    );
    curves.extend(
        sketch
            .live_arcs()
            .map(|(id, arc)| (Curve::Arc(id), vec![arc.center, arc.start, arc.end])),
    );
    curves.extend(
        sketch
            .live_ellipses()
            .map(|(id, _)| (Curve::Ellipse(id), sketch.ellipse_points(id).to_vec())),
    );
    for (point, _) in sketch.drawn_points() {
        for support in sketch.holds_on(point) {
            let on = match support {
                Support::Segment(side) | Support::Middle(side) => Curve::Segment(side),
                Support::Circle(circle) => Curve::Circle(circle),
                Support::Arc(arc) => Curve::Arc(arc),
                Support::Ellipse(ellipse) => Curve::Ellipse(ellipse),
                _ => continue,
            };
            if let Some((_, points)) = curves.iter_mut().find(|(curve, _)| *curve == on) {
                points.push(point);
            }
        }
    }
    curves
}

/// The groups of points joined by the curves they belong to and by what holds
/// them on a curve, the origin joining none: each a piece.
pub(super) fn pieces(sketch: &Sketch) -> Vec<Vec<PointId>> {
    let mut parent: Vec<usize> = (0..sketch.points().len()).collect();
    fn root(parent: &mut [usize], mut at: usize) -> usize {
        while parent[at] != at {
            parent[at] = parent[parent[at]];
            at = parent[at];
        }
        at
    }
    for (_, points) in curves_and_points(sketch) {
        let mut joined = points.into_iter().filter(|point| !sketch.is_origin(*point));
        if let Some(first) = joined.next() {
            for point in joined {
                let (one, other) = (root(&mut parent, first.0), root(&mut parent, point.0));
                parent[one] = other;
            }
        }
    }
    let mut pieces: Vec<(usize, Vec<PointId>)> = Vec::new();
    for (point, _) in sketch.drawn_points() {
        let at = root(&mut parent, point.0);
        match pieces.iter_mut().find(|(of, _)| *of == at) {
            Some((_, piece)) => piece.push(point),
            None => pieces.push((at, vec![point])),
        }
    }
    pieces.into_iter().map(|(_, piece)| piece).collect()
}

/// Which traits meet at a corner: each pair of traits one after the other
/// round a point they both end at.
pub(super) fn corners(sketch: &Sketch, among: &[SegmentId]) -> Vec<(SegmentId, SegmentId)> {
    let mut corners = Vec::new();
    for (point, _) in sketch.live_points() {
        let meeting: Vec<SegmentId> = among
            .iter()
            .copied()
            .filter(|side| {
                let line = sketch.segments()[side.0];
                line.start == point || line.end == point
            })
            .collect();
        corners.extend(meeting.windows(2).map(|pair| (pair[0], pair[1])));
    }
    corners
}

fn is_square(sketch: &Sketch, one: SegmentId, other: SegmentId) -> bool {
    let (a, b) = sketch.endpoints(one);
    let (c, d) = sketch.endpoints(other);
    let (first, second) = (b - a, d - c);
    first.dot(second).abs() <= 1e-9 * first.length() * second.length()
}

impl Asker<'_> {
    /// Gives a piece what it takes to turn green, each in turn: kept when it
    /// takes some freedom away, left out otherwise. Its values are what it
    /// measures, so that it turns green where it stands.
    pub(super) fn make_green(&mut self, piece: &[PointId]) {
        for ask in self.what_green_takes(piece) {
            let settled = self.sketch.settled_points(SCALE);
            let is_settled = |point: PointId| settled.get(point.0).copied().unwrap_or(false);
            if self.already_held(ask, &is_settled) {
                continue;
            }
            let Some(gesture) = self.gesture(ask, true) else {
                continue;
            };
            let mut trial = self.sketch.clone();
            gesture.lay(&mut trial);
            if trial.freedom(SCALE).degrees_of_freedom
                < self.sketch.freedom(SCALE).degrees_of_freedom
            {
                self.sketch = trial;
                self.gestures.push(gesture);
            }
        }
    }

    fn what_green_takes(&mut self, piece: &[PointId]) -> Vec<Ask> {
        let sketch = &self.sketch;
        let curves: Vec<(Curve, Vec<PointId>)> = curves_and_points(sketch)
            .into_iter()
            .filter(|(_, points)| points.iter().any(|point| piece.contains(point)))
            .collect();
        let attached = curves
            .iter()
            .any(|(_, points)| points.iter().any(|point| sketch.is_origin(*point)));
        let sides: Vec<SegmentId> = curves
            .iter()
            .filter_map(|(curve, _)| match curve {
                Curve::Segment(side) => Some(*side),
                _ => None,
            })
            .collect();
        let mut asks = Vec::new();
        if !attached && let Some(nearest) = nearest_the_origin(sketch, piece) {
            let by_values = [
                Ask::FromOrigin(nearest, SketchAxis::U),
                Ask::FromOrigin(nearest, SketchAxis::V),
            ];
            match self.random.chance(0.5) {
                true => {
                    asks.push(Ask::OntoOrigin(nearest));
                    asks.extend(by_values);
                }
                false => {
                    asks.extend(by_values);
                    asks.push(Ask::OntoOrigin(nearest));
                }
            }
        }
        for (curve, _) in &curves {
            match curve {
                Curve::Circle(circle) => asks.push(Ask::Diameter(*circle)),
                Curve::Arc(arc) => asks.extend([Ask::ArcSweep(*arc), Ask::ArcRadius(*arc)]),
                _ => {}
            }
        }
        asks.extend(sides.iter().map(|side| Ask::Length(*side)));
        for (one, other) in corners(sketch, &sides) {
            asks.push(match is_square(sketch, one, other) {
                true => Ask::Ruled(
                    Rule::Perpendicular,
                    Target::Segment(one),
                    Target::Segment(other),
                ),
                false => Ask::Corner(one, other),
            });
        }
        asks.extend(
            sides
                .iter()
                .map(|side| Ask::AgainstAxis(*side, SketchAxis::U)),
        );
        for point in piece {
            asks.extend([
                Ask::FromOrigin(*point, SketchAxis::U),
                Ask::FromOrigin(*point, SketchAxis::V),
            ]);
            for reference in slanted_from(sketch, *point) {
                asks.extend([
                    Ask::FromPoint(*point, reference, Some(SketchAxis::U)),
                    Ask::FromPoint(*point, reference, Some(SketchAxis::V)),
                    Ask::FromPoint(*point, reference, None),
                ]);
            }
        }
        asks
    }

    /// Whether every point an ask would act on is settled already, so that it
    /// could take nothing away. A size is never taken for settled: a circle
    /// whose centre is held still has its radius free.
    fn already_held(&self, ask: Ask, settled: &dyn Fn(PointId) -> bool) -> bool {
        let sketch = &self.sketch;
        let ends = |side: SegmentId| {
            let line = sketch.segments()[side.0];
            settled(line.start) && settled(line.end)
        };
        match ask {
            Ask::Length(side) | Ask::AgainstAxis(side, _) => ends(side),
            Ask::Corner(one, other)
            | Ask::Ruled(_, Target::Segment(one), Target::Segment(other)) => {
                ends(one) && ends(other)
            }
            Ask::ArcRadius(arc) | Ask::ArcSweep(arc) => {
                let drawn = sketch.arcs()[arc.0];
                settled(drawn.center) && settled(drawn.start) && settled(drawn.end)
            }
            Ask::FromOrigin(point, _) | Ask::FromPoint(point, ..) | Ask::OntoOrigin(point) => {
                settled(point)
            }
            _ => false,
        }
    }
}

/// The four points nearest `point` that stand off both its axes, for its
/// width and its height to be read from where the origin reads only one.
fn slanted_from(sketch: &Sketch, point: PointId) -> Vec<PointId> {
    let at = sketch.point(point);
    let apart = reach(at, 1e-6);
    let mut others: Vec<(PointId, f64)> = sketch
        .live_points()
        .filter(|(other, place)| {
            *other != point && (place.x - at.x).abs() > apart && (place.y - at.y).abs() > apart
        })
        .map(|(other, place)| (other, place.distance(at)))
        .collect();
    others.sort_by(|one, other| one.1.total_cmp(&other.1));
    others.into_iter().take(4).map(|(other, _)| other).collect()
}

fn nearest_the_origin(sketch: &Sketch, piece: &[PointId]) -> Option<PointId> {
    piece.iter().copied().min_by(|one, other| {
        sketch
            .point(*one)
            .length()
            .total_cmp(&sketch.point(*other).length())
    })
}

/// Whether a point stands on its own: no curve passes through it, nor holds
/// it.
pub(super) fn alone(sketch: &Sketch, point: PointId) -> bool {
    !curves_and_points(sketch)
        .iter()
        .any(|(_, points)| points.contains(&point))
}
