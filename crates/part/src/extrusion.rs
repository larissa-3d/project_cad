//! Turning a sketch's areas into matter, or taking matter away, by a prism
//! along the plane's normal; and what every step of matter shares with the
//! sweep around an axis, in `revolution.rs`.

use cao_sketch::{Area, Region, Sketch};
use cao_solid::profile::{Frame, Profile};
use cao_solid::{Body, Declined};
use glam::{DVec2, DVec3};

use crate::broken::Broken;
use crate::formula::Formula;
use crate::history::{ExtrusionMode, Operation};
use crate::profile::profile;
use crate::state::PartState;

impl PartState {
    /// Raises a step of matter, or cuts it, noting the faces of the part it
    /// made under the number of the operation replayed: a tool's faces are
    /// numbered above the part's, so they are the ones numbered from where
    /// the part's ended. Noted even when the step made nothing, so that every
    /// step of matter has its note.
    pub(crate) fn raising(&mut self, raise: impl FnOnce(&mut Self)) {
        let from = self.body.faces_end();
        raise(self);
        self.made
            .push((self.replaying, from..self.body.faces_end()));
    }

    /// The faces the part still holds of those the step of matter of that
    /// number made, and the pieces a later cut parted from them, which answer
    /// to numbers of their own: a cut makes none of the pieces it leaves of
    /// another step's face.
    pub(crate) fn faces_made(&self, step: u32) -> Vec<usize> {
        let made = self.made.iter().find(|(number, _)| *number == step);
        made.map_or_else(Vec::new, |(_, made)| {
            let root = |piece: usize| {
                std::iter::successors(Some(piece), |&face| self.body.parted_from(face))
                    .last()
                    .unwrap_or(piece)
            };
            // Walked number by number, which holds only while `profile` hands
            // exact runs: on the flats, an exact body's pieces would carry the
            // count past 2^31 and stretch the range with it.
            made.clone()
                .filter(|face| self.body.has_face(*face))
                .chain(
                    self.body
                        .pieces()
                        .into_iter()
                        .filter(|&piece| made.contains(&root(piece))),
                )
                .collect()
        })
    }

    /// Joins a tool to the part, or takes it out. A step the kernel declines
    /// leaves the part as it was; one it built though it reads it unstably is
    /// noted with its lean. The tool's own pieces stand square or along each
    /// other, and are not read.
    pub(crate) fn combine(&mut self, tool: Body, mode: ExtrusionMode) -> Result<(), Declined> {
        if tool.is_empty() {
            return Ok(());
        }
        self.body = match mode {
            ExtrusionMode::Add => self.body.union(&tool),
            ExtrusionMode::Cut => self.body.difference(&tool),
        }?;
        if let Some(unstable) = self.body.unstable() {
            self.unstable.insert(self.replaying, unstable);
        }
        Ok(())
    }

    /// Turns the chosen areas of a sketch into a prism and joins it to the
    /// part, or takes it out.
    ///
    /// Every area is turned into matter first and the lot is applied in one go:
    /// two areas extruded together must behave as one shape, not as two that
    /// happen to be cut one after the other.
    pub(crate) fn extrude(
        &mut self,
        index: usize,
        areas: &[Area],
        distance: &Formula,
        mode: ExtrusionMode,
    ) {
        let Some(distance) = self.size(distance, raises) else {
            return;
        };
        self.fix_a_unit_at_a_millimetre();
        let Some(prism) = self.prism(index, areas, distance) else {
            return;
        };
        let profiles = prism.profiles();
        let tool = match prism.tool(&self.body, &profiles) {
            Ok(tool) => tool,
            Err(reason) => return self.declined_with(reason, past_raises(&profiles)),
        };

        // An area the drawing no longer encloses raises nothing, which is a
        // size that no longer holds.
        if prism.lost {
            self.broke(Broken::Operation(self.replaying));
        }
        if !self.shows(&tool, mode) {
            return past_raises(&profiles)(&mut self.body);
        }
        if let Err(reason) = self.combine(tool, mode) {
            self.declined_with(reason, past_raises(&profiles));
        }
    }

    /// The matter a step would raise or take away, alone: the same for an
    /// addition and a cut, neither joined to the part nor cut from it, and
    /// nothing noted. Nothing for a size that raises nothing, or a tool the
    /// kernel declines.
    pub(crate) fn tool_of(&self, operation: &Operation) -> Option<Body> {
        match operation {
            Operation::Extrude {
                sketch,
                areas,
                distance,
                ..
            } => self.raised_tool(*sketch, areas, distance),
            Operation::Revolve {
                sketch,
                areas,
                axis,
                angle,
                ..
            } => self.turned_tool(*sketch, areas, *axis, angle),
            _ => None,
        }
    }

    /// The matter an extrusion would raise, alone, with nothing noted.
    fn raised_tool(&self, index: usize, areas: &[Area], distance: &Formula) -> Option<Body> {
        let distance = distance
            .value(&self.values)
            .filter(|travel| raises(*travel))?;
        let prism = self.prism(index, areas, distance)?;
        prism.tool(&self.body, &prism.profiles()).ok()
    }

    /// The areas of a sketch a raise of that distance stands on, and the way
    /// it travels. The scale is read as a step fixes it: a unit at a
    /// millimetre when the part has none yet.
    fn prism(&self, index: usize, areas: &[Area], distance: f64) -> Option<Prism> {
        let sketch = self.sketches.get(index)?;
        let regions = sketch.regions();
        let (standing, lost) = self.all_standing_on(index, areas, &regions);
        let standing: Vec<Region> = standing.into_iter().cloned().collect();
        Some(Prism {
            triangles: standing.iter().map(Region::face_triangles).collect(),
            standing,
            lost,
            frame: frame_of(sketch),
            travel: sketch.plane.normal() * (distance / self.scale()),
        })
    }

    /// A step the kernel declined: broken, named among the declined ones with
    /// the reason it gave, and counted past the numbers it would have given its
    /// faces, so that the steps after it number theirs as if it had stood.
    pub(crate) fn declined_with(&mut self, reason: Declined, count_past: impl FnOnce(&mut Body)) {
        self.broke(Broken::Operation(self.replaying));
        self.declined.insert(self.replaying, reason);
        count_past(&mut self.body);
    }

    /// The areas of a drawing these places fall in, each named by the curves
    /// that bound it.
    ///
    /// This is what a click means, and it is worked out at the moment of the
    /// click and never again: snapping and what lies under the cursor depend
    /// on the view at the time, the same reason `PointRef` records the point
    /// a click landed on.
    pub fn areas_at(&self, sketch: usize, places: &[DVec2]) -> Vec<Area> {
        let Some(drawing) = self.sketches.get(sketch) else {
            return Vec::new();
        };
        let regions = drawing.regions();
        places
            .iter()
            .filter_map(|place| {
                let rank = cao_sketch::area_under(&regions, *place)?;
                Some(Area::of(&regions[rank], *place))
            })
            .collect()
    }

    /// The areas a step of matter stands on, as the drawing holds them now,
    /// and whether one of them is lost.
    pub(crate) fn all_standing_on<'a>(
        &self,
        sketch: usize,
        areas: &[Area],
        regions: &'a [Region],
    ) -> (Vec<&'a Region>, bool) {
        let mut lost = false;
        let standing = areas
            .iter()
            .filter_map(|area| {
                let region = self.standing_on(sketch, area, regions);
                lost |= region.is_none();
                region
            })
            .collect();
        (standing, lost)
    }

    /// The area a step of matter stands on, as the drawing holds it now.
    ///
    /// The name it was given at the click is followed through every cut made
    /// since, and then asked of the drawing. Nothing when the drawing no
    /// longer encloses it — which is what the tree says, rather than raising
    /// matter somewhere the user never pointed at.
    fn standing_on<'a>(
        &self,
        sketch: usize,
        area: &Area,
        regions: &'a [Region],
    ) -> Option<&'a Region> {
        regions.get(self.area_rank(sketch, area, regions)?)
    }
}

/// Whether a step travels far enough to raise anything.
fn raises(travel: f64) -> bool {
    travel.abs() >= 1e-6
}

/// What a raise is built from: the areas it stands on, with their triangles,
/// whether one of them is lost, and the way it travels.
struct Prism {
    standing: Vec<Region>,
    triangles: Vec<Vec<[DVec2; 3]>>,
    lost: bool,
    frame: Frame,
    travel: DVec3,
}

impl Prism {
    fn profiles(&self) -> Vec<Profile<'_>> {
        self.standing
            .iter()
            .zip(&self.triangles)
            .map(|(region, triangles)| profile(region, triangles))
            .collect()
    }

    /// Every area raised against the part's matter and joined into one tool.
    fn tool(&self, part: &Body, profiles: &[Profile]) -> Result<Body, Declined> {
        profiles.iter().try_fold(Body::default(), |tool, profile| {
            tool.union(&part.tool_raised(profile, self.frame, self.travel)?)
        })
    }
}

/// Counts a body past the numbers a raise of each profile would have named.
fn past_raises<'a>(profiles: &'a [Profile<'a>]) -> impl FnOnce(&mut Body) + 'a {
    move |body| profiles.iter().for_each(|profile| body.count_past(profile))
}

/// The frame a sketch's areas stand in, for the kernels.
pub(crate) fn frame_of(sketch: &Sketch) -> Frame {
    Frame {
        origin: sketch.plane.origin,
        u: sketch.plane.u,
        v: sketch.plane.v,
    }
}
