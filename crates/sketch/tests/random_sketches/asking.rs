//! Dimensions, rules and drags drawn for a drawing, under the shares a mode
//! gives it.
//!
//! Each is aimed at a piece of the drawing as it stands when it is laid and
//! written as the clicks that reach it, so that it shrinks and prints like
//! any other gesture. A value is drawn around what the drawing measures at
//! that moment; a piece made entirely constrained is `greening.rs`'s.

use cao_sketch::{
    ArcId, CircleId, EllipseId, PointId, Rule, SegmentId, Sketch, SketchAxis, precision,
};
use glam::DVec2;

use super::Gesture;
use super::greening::{alone, corners, pieces};
use super::holding::{dimension_aimed, measured, rule_picks};
use super::laying::{UNDER_A_PLACE, laid, reach};
use super::random::Random;

/// How much of what a drawing could take it is given, each from nought to
/// one, and how many times it is dragged.
#[derive(Clone, Copy, Debug)]
pub(super) struct Shares {
    pub dimensions: f64,
    pub rules: f64,
    pub constrained: f64,
    pub drags: usize,
}

/// How many dimensions, and how many rules, a drawing is offered at most out
/// of all it could take: each one laid on a large drawing that already holds
/// many costs the solver more, and the gate runs these drawings too.
const OFFERED: usize = 8;

/// What a dimension, a rule or a drag is aimed at, by the drawing's own
/// names: read again at the moment it is laid, and dropped when what it names
/// is gone.
#[derive(Clone, Copy, Debug)]
pub(super) enum Ask {
    Length(SegmentId),
    Corner(SegmentId, SegmentId),
    AgainstAxis(SegmentId, SketchAxis),
    Diameter(CircleId),
    Radius(CircleId),
    ArcRadius(ArcId),
    ArcSweep(ArcId),
    FromOrigin(PointId, SketchAxis),
    FromPoint(PointId, PointId, Option<SketchAxis>),
    Ruled(Rule, Target, Target),
    Fixed(Target),
    OntoOrigin(PointId),
    Dragged(Target),
}

/// One thing a rule or a drag is aimed at.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Target {
    Point(PointId),
    Segment(SegmentId),
    Circle(CircleId),
    Arc(ArcId),
    Ellipse(EllipseId),
    Axis(SketchAxis),
}

pub(super) struct Asker<'a> {
    pub(super) random: &'a mut Random,
    pub(super) sketch: Sketch,
    pub(super) gestures: Vec<Gesture>,
}

/// The dimensions, rules and drags a drawing is given, to be laid after its
/// own gestures.
pub(super) fn asked(random: &mut Random, drawing: &[Gesture], shares: Shares) -> Vec<Gesture> {
    let sketch = laid(drawing);
    let mut asker = Asker {
        random,
        sketch,
        gestures: Vec::new(),
    };
    let mut asks: Vec<Ask> = Vec::new();
    let dimensions = asker.dimensions_it_could_take();
    for ask in asker.offered(dimensions) {
        if asker.random.chance(shares.dimensions) {
            asks.push(ask);
        }
    }
    let rules = asker.rules_it_could_take();
    for ask in asker.offered(rules) {
        if asker.random.chance(shares.rules) {
            asks.push(ask);
        }
    }
    for _ in 0..shares.drags {
        if let Some(ask) = asker.drag() {
            asks.push(ask);
        }
    }
    for index in (1..asks.len()).rev() {
        asks.swap(index, asker.random.below(index + 1));
    }
    let made_green: Vec<Vec<PointId>> = pieces(&asker.sketch)
        .into_iter()
        .filter(|_| asker.random.chance(shares.constrained))
        .collect();
    for ask in asks {
        if let Some(gesture) = asker.gesture(ask, false) {
            gesture.lay(&mut asker.sketch);
            asker.gestures.push(gesture);
        }
    }
    for piece in made_green {
        asker.make_green(&piece);
    }
    asker.gestures
}

pub(super) fn origin_of(sketch: &Sketch) -> Option<PointId> {
    sketch
        .live_points()
        .map(|(point, _)| point)
        .find(|point| sketch.is_origin(*point))
}

impl Asker<'_> {
    pub(super) fn size(&self) -> f64 {
        precision::drawing_size(&self.sketch).max(1.0)
    }

    /// At most `OFFERED` of what a drawing could take, chosen at random and
    /// kept in the order they came.
    fn offered(&mut self, mut could: Vec<Ask>) -> Vec<Ask> {
        while could.len() > OFFERED {
            could.remove(self.random.below(could.len()));
        }
        could
    }

    fn dimensions_it_could_take(&mut self) -> Vec<Ask> {
        let sketch = &self.sketch;
        let sides: Vec<SegmentId> = sketch.live_segments().map(|(id, _)| id).collect();
        let mut asks: Vec<Ask> = sides.iter().map(|side| Ask::Length(*side)).collect();
        asks.extend(
            corners(sketch, &sides)
                .into_iter()
                .map(|(one, other)| Ask::Corner(one, other)),
        );
        for side in &sides {
            let axis = *self.random.pick(&[SketchAxis::U, SketchAxis::V]);
            asks.push(Ask::AgainstAxis(*side, axis));
        }
        for (circle, _) in sketch.live_circles() {
            asks.push(match self.random.chance(0.7) {
                true => Ask::Diameter(circle),
                false => Ask::Radius(circle),
            });
        }
        for (arc, _) in sketch.live_arcs() {
            asks.extend([Ask::ArcRadius(arc), Ask::ArcSweep(arc)]);
        }
        for (point, _) in sketch.drawn_points() {
            if alone(sketch, point) {
                let axis = *self.random.pick(&[SketchAxis::U, SketchAxis::V]);
                asks.push(Ask::FromOrigin(point, axis));
            }
        }
        asks
    }

    fn rules_it_could_take(&mut self) -> Vec<Ask> {
        let sketch = &self.sketch;
        let sides: Vec<Target> = sketch
            .live_segments()
            .map(|(id, _)| Target::Segment(id))
            .collect();
        let rounds: Vec<Target> = sketch
            .live_circles()
            .map(|(id, _)| Target::Circle(id))
            .chain(sketch.live_arcs().map(|(id, _)| Target::Arc(id)))
            .collect();
        let ellipses: Vec<Target> = sketch
            .live_ellipses()
            .map(|(id, _)| Target::Ellipse(id))
            .collect();
        let points: Vec<Target> = sketch
            .live_points()
            .map(|(id, _)| Target::Point(id))
            .collect();
        let curves: Vec<Target> = rounds.iter().chain(&ellipses).copied().collect();
        let random = &mut *self.random;
        let mut asks = Vec::new();
        for side in &sides {
            let axis = Target::Axis(*random.pick(&[SketchAxis::U, SketchAxis::V]));
            let with_axis = *random.pick(&[Rule::Perpendicular, Rule::Parallel, Rule::Collinear]);
            asks.push(Ask::Ruled(with_axis, *side, axis));
            let other = *random.pick(&sides);
            if other != *side {
                let rule = *random.pick(&[
                    Rule::Perpendicular,
                    Rule::Parallel,
                    Rule::Equal,
                    Rule::Collinear,
                ]);
                asks.push(Ask::Ruled(rule, *side, other));
            }
            if !curves.is_empty() && random.chance(0.5) {
                asks.push(Ask::Ruled(Rule::Tangent, *side, *random.pick(&curves)));
            }
        }
        for point in points.iter().filter(|point| match point {
            Target::Point(id) => !sketch.is_origin(*id),
            _ => false,
        }) {
            let onto: Vec<Target> = sides
                .iter()
                .chain(&curves)
                .chain(&points)
                .copied()
                .filter(|onto| onto != point)
                .collect();
            if onto.is_empty() {
                continue;
            }
            let ask = match random.chance(0.2) && !sides.is_empty() {
                true => Ask::Ruled(Rule::Midpoint, *point, *random.pick(&sides)),
                false => Ask::Ruled(Rule::Coincident, *point, *random.pick(&onto)),
            };
            asks.push(ask);
        }
        for round in &rounds {
            let other = *random.pick(&rounds);
            if other != *round {
                let rule = match (round, other) {
                    (Target::Circle(_), Target::Circle(_)) if random.chance(0.4) => {
                        Rule::Concentric
                    }
                    _ => Rule::Equal,
                };
                asks.push(Ask::Ruled(rule, *round, other));
            }
        }
        let all: Vec<Target> = points
            .iter()
            .filter(|point| !matches!(point, Target::Point(id) if sketch.is_origin(*id)))
            .chain(&sides)
            .chain(&curves)
            .copied()
            .collect();
        if !all.is_empty() {
            asks.push(Ask::Fixed(*random.pick(&all)));
        }
        asks
    }

    fn drag(&mut self) -> Option<Ask> {
        let sketch = &self.sketch;
        let mut targets: Vec<Target> = sketch
            .drawn_points()
            .map(|(id, _)| Target::Point(id))
            .collect();
        targets.extend(sketch.live_segments().map(|(id, _)| Target::Segment(id)));
        targets.extend(sketch.live_circles().map(|(id, _)| Target::Circle(id)));
        targets.extend(sketch.live_arcs().map(|(id, _)| Target::Arc(id)));
        targets.extend(sketch.live_ellipses().map(|(id, _)| Target::Ellipse(id)));
        (!targets.is_empty()).then(|| Ask::Dragged(*self.random.pick(&targets)))
    }

    /// The gesture that lays an ask on the drawing as it stands, or nothing
    /// when what it names is gone or cannot be clicked apart from what lies
    /// around it. `exact` types what the drawing measures.
    pub(super) fn gesture(&mut self, ask: Ask, exact: bool) -> Option<Gesture> {
        match ask {
            Ask::Ruled(rule, one, other) => {
                let mut clicks = vec![self.click_on(one)?, self.click_on(other)?];
                let laid_from_either = !exact && self.random.chance(0.5);
                if laid_from_either {
                    clicks.reverse();
                }
                self.rule_reaches(rule, clicks, &[one, other])
            }
            Ask::Fixed(target) => {
                let clicks = vec![self.click_on(target)?];
                self.rule_reaches(Rule::Fixed, clicks, &[target])
            }
            Ask::OntoOrigin(point) => {
                let origin = origin_of(&self.sketch)?;
                let clicks = vec![
                    self.click_on(Target::Point(point))?,
                    self.click_on(Target::Point(origin))?,
                ];
                let reached = rule_picks(&self.sketch, Rule::Coincident, &clicks).len() == 2;
                reached.then_some(Gesture::Rule {
                    rule: Rule::Coincident,
                    clicks,
                })
            }
            Ask::Dragged(target) => self.dragged(target),
            _ => self.dimension(ask, exact),
        }
    }

    fn rule_reaches(&self, rule: Rule, clicks: Vec<[f64; 2]>, on: &[Target]) -> Option<Gesture> {
        let picks = rule_picks(&self.sketch, rule, &clicks);
        let reached = picks.len() == on.len()
            && on.iter().all(|target| {
                picks.iter().any(|pick| match (pick, target) {
                    (cao_sketch::RulePick::Axis(axis), Target::Axis(wanted)) => axis == wanted,
                    (cao_sketch::RulePick::Element(element), _) => element_is(*element, *target),
                    _ => false,
                })
            });
        reached.then_some(Gesture::Rule { rule, clicks })
    }

    fn dimension(&mut self, ask: Ask, exact: bool) -> Option<Gesture> {
        let sketch = &self.sketch;
        let (clicks, about) = match ask {
            Ask::Length(side) => {
                let on = self.click_on(Target::Segment(side))?;
                (vec![on], DVec2::from(on))
            }
            Ask::Corner(one, other) => {
                let corner = shared_end(sketch, one, other)?;
                let clicks = vec![
                    self.click_on(Target::Segment(one))?,
                    self.click_on(Target::Segment(other))?,
                ];
                (clicks, corner)
            }
            Ask::AgainstAxis(side, axis) => {
                let on = self.click_on(Target::Segment(side))?;
                (
                    vec![on, self.click_on(Target::Axis(axis))?],
                    DVec2::from(on),
                )
            }
            Ask::Diameter(circle) => {
                let on = self.click_on(Target::Circle(circle))?;
                (vec![on], DVec2::from(on))
            }
            Ask::Radius(circle) => {
                let centre = sketch.circles().get(circle.0)?.center;
                let on = self.click_on(Target::Circle(circle))?;
                (
                    vec![on, self.click_on(Target::Point(centre))?],
                    DVec2::from(on),
                )
            }
            Ask::ArcRadius(arc) => {
                let on = self.click_on(Target::Arc(arc))?;
                (vec![on], DVec2::from(on))
            }
            Ask::ArcSweep(arc) => {
                let start = sketch.arcs().get(arc.0)?.start;
                let on = self.click_on(Target::Arc(arc))?;
                (
                    vec![on, self.click_on(Target::Point(start))?],
                    DVec2::from(on),
                )
            }
            Ask::FromOrigin(point, axis) => {
                let origin = origin_of(sketch)?;
                return self.dimension(Ask::FromPoint(point, origin, Some(axis)), exact);
            }
            Ask::FromPoint(point, reference, axis) => {
                let clicks = vec![
                    self.click_on(Target::Point(point))?,
                    self.click_on(Target::Point(reference))?,
                ];
                let (from, to) = (sketch.point(point), sketch.point(reference));
                let (low, high) = (from.min(to), from.max(to));
                let out = (self.size() * 0.05).max(reach(high, 1e-3));
                let reading = match axis {
                    Some(SketchAxis::U) => DVec2::new((low.x + high.x) / 2.0, low.y - out),
                    Some(SketchAxis::V) => DVec2::new(low.x - out, (low.y + high.y) / 2.0),
                    None => (low + high) / 2.0,
                };
                return self.put_down(clicks, &[reading], exact);
            }
            _ => return None,
        };
        let size = self.size();
        let places: Vec<DVec2> = [0.03, 0.08, 0.2]
            .into_iter()
            .flat_map(|share| {
                (0..8).map(move |step| {
                    about
                        + DVec2::from_angle(0.4 + step as f64 * std::f64::consts::TAU / 8.0)
                            * share
                            * size
                })
            })
            .collect();
        self.put_down(clicks, &places, exact)
    }

    /// The clicks that pick a dimension, put down at the first of `places`
    /// that puts it down rather than picking something more, and the value
    /// typed into it.
    fn put_down(
        &mut self,
        clicks: Vec<[f64; 2]>,
        places: &[DVec2],
        exact: bool,
    ) -> Option<Gesture> {
        for place in places {
            let mut all = clicks.clone();
            all.push(place.to_array());
            let mut trial = self.sketch.clone();
            let Some(target) = dimension_aimed(&mut trial, &all) else {
                continue;
            };
            let mut shorter = self.sketch.clone();
            if dimension_aimed(&mut shorter, &clicks) == Some(target) {
                continue;
            }
            let measure = measured(&trial, target)?;
            let value = match exact {
                true => measure,
                false => self.around(measure, target.is_angle()),
            };
            return Some(Gesture::Dimension { clicks: all, value });
        }
        None
    }

    /// A value drawn around what is measured: most of the time near it, so
    /// that the drawing moves more or less; now and then one it cannot take.
    fn around(&mut self, measure: f64, angle: bool) -> f64 {
        let random = &mut *self.random;
        let thousandth = |value: f64| (value * 1000.0).round() / 1000.0;
        match random.below(12) {
            0..=2 => measure,
            3..=6 => thousandth(measure * (1.0 + random.pick(&[-0.1, -0.02, 0.01, 0.05, 0.2]))),
            7..=9 => thousandth(measure * random.pick(&[0.5, 0.75, 1.5, 2.0])),
            _ => match angle {
                true => *random.pick(&[0.0, 180.0, 359.0, -30.0, measure + 180.0]),
                false => *random.pick(&[0.0, -measure, measure * 40.0, 1e-4]),
            },
        }
    }

    fn dragged(&mut self, target: Target) -> Option<Gesture> {
        let from = DVec2::from(self.click_on(target)?);
        let size = self.size();
        let way = *self.random.pick(&[
            DVec2::X,
            DVec2::Y,
            DVec2::NEG_X,
            DVec2::NEG_Y,
            DVec2::ONE.normalize(),
            DVec2::new(1.0, -1.0).normalize(),
        ]);
        let far = *self.random.pick(&[0.05, 0.1, 0.25, 0.5]) * size;
        Some(Gesture::Drag {
            from: from.to_array(),
            to: (from + way * far).to_array(),
        })
    }

    /// A place a click lands on the target at, and on nothing else that would
    /// take the click first.
    fn click_on(&self, target: Target) -> Option<[f64; 2]> {
        let sketch = &self.sketch;
        let clear_of_points =
            |at: DVec2| sketch.nearest_point(at, reach(at, UNDER_A_PLACE)).is_none();
        let picks = |at: DVec2| -> Option<cao_sketch::Element> {
            let near = reach(at, UNDER_A_PLACE);
            sketch
                .nearest_segment(at, near)
                .map(cao_sketch::Element::Segment)
                .or_else(|| {
                    sketch
                        .nearest_circle(at, near)
                        .map(cao_sketch::Element::Circle)
                })
                .or_else(|| sketch.nearest_arc(at, near).map(cao_sketch::Element::Arc))
                .or_else(|| {
                    sketch
                        .nearest_ellipse(at, near)
                        .map(cao_sketch::Element::Ellipse)
                })
        };
        let lands = |at: DVec2| {
            clear_of_points(at) && picks(at).is_some_and(|element| element_is(element, target))
        };
        let places: Vec<DVec2> = match target {
            Target::Point(point) => {
                let at = sketch.points().get(point.0).copied()?;
                return (sketch.nearest_point(at, reach(at, UNDER_A_PLACE)) == Some(point))
                    .then(|| at.to_array());
            }
            Target::Segment(side) => {
                sketch.live_segments().find(|(id, _)| *id == side)?;
                let (from, to) = sketch.endpoints(side);
                [0.5, 0.4, 0.6, 0.3, 0.7, 0.2, 0.8]
                    .map(|along| from.lerp(to, along))
                    .to_vec()
            }
            Target::Circle(circle) => {
                let round = sketch.live_circles().find(|(id, _)| *id == circle)?.1;
                let centre = sketch.point(round.center);
                (0..8)
                    .map(|step| {
                        centre
                            + DVec2::from_angle(0.3 + step as f64 * std::f64::consts::TAU / 8.0)
                                * round.radius
                    })
                    .collect()
            }
            Target::Arc(arc) => {
                sketch.live_arcs().find(|(id, _)| *id == arc)?;
                vec![sketch.arc_midpoint(arc)]
            }
            Target::Ellipse(ellipse) => {
                sketch.live_ellipses().find(|(id, _)| *id == ellipse)?;
                let (from, sweep) = sketch.ellipse_run(ellipse);
                let drawn = sketch.ellipse_draft(ellipse);
                [0.3, 0.6, 0.45, 0.8]
                    .map(|share| drawn.at(from + sweep * share))
                    .to_vec()
            }
            Target::Axis(axis) => {
                let out = sketch
                    .live_points()
                    .map(|(_, at)| at)
                    .fold(DVec2::ZERO, |low, at| low.min(at))
                    - DVec2::splat(self.size() * 0.5);
                let at = match axis {
                    SketchAxis::U => DVec2::new(out.x, 0.0),
                    SketchAxis::V => DVec2::new(0.0, out.y),
                };
                return (clear_of_points(at) && picks(at).is_none()).then(|| at.to_array());
            }
        };
        places
            .into_iter()
            .find(|at| lands(*at))
            .map(|at| at.to_array())
    }
}

fn element_is(element: cao_sketch::Element, target: Target) -> bool {
    use cao_sketch::Element;
    matches!(
        (element, target),
        (Element::Point(one), Target::Point(other)) if one == other
    ) || matches!(
        (element, target),
        (Element::Segment(one), Target::Segment(other)) if one == other
    ) || matches!(
        (element, target),
        (Element::Circle(one), Target::Circle(other)) if one == other
    ) || matches!(
        (element, target),
        (Element::Arc(one), Target::Arc(other)) if one == other
    ) || matches!(
        (element, target),
        (Element::Ellipse(one), Target::Ellipse(other)) if one == other
    )
}

fn shared_end(sketch: &Sketch, one: SegmentId, other: SegmentId) -> Option<DVec2> {
    let (first, second) = (sketch.segments()[one.0], sketch.segments()[other.0]);
    [first.start, first.end]
        .into_iter()
        .find(|end| *end == second.start || *end == second.end)
        .map(|end| sketch.point(end))
}
