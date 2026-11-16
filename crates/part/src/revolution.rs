//! Turning a sketch's areas around an axis lying in its plane, into matter or
//! matter taken away.

use cao_sketch::{Area, Region, Sketch};
use cao_solid::profile::{Frame, Profile};
use cao_solid::turning::{Axis, Lie, Turn};
use cao_solid::{Body, Declined};
use glam::DVec2;

use crate::broken::Broken;
use crate::extrusion::frame_of;
use crate::formula::Formula;
use crate::history::{ExtrusionMode, RevolutionAxis};
use crate::profile::profile;
use crate::state::PartState;

impl PartState {
    /// Sweeps the chosen areas around an axis of the sketch and joins the
    /// result to the part, or takes it out.
    ///
    /// An area across the axis would sweep through itself: it is cut along
    /// the axis first, and each side is turned on its own, as if the axis had
    /// been drawn across it. A turn of straight runs and arcs is the exact
    /// kernel's (#576); one with an ellipse, or a profile the kernel cannot lay
    /// against its axis, is a broken step (#632).
    pub(crate) fn revolve(
        &mut self,
        index: usize,
        areas: &[Area],
        axis: RevolutionAxis,
        degrees: &Formula,
        mode: ExtrusionMode,
    ) {
        let Some(degrees) = self.size(degrees, turns) else {
            return;
        };
        let Some(turning) = self.turning(index, areas, axis, degrees) else {
            return;
        };
        let profiles = turning.profiles();
        let past_turns = |body: &mut Body| {
            for (profile, turn) in &profiles {
                body.count_past_turned(profile, turning.frame, turn);
            }
        };
        let tool = match turning.tool(&self.body, &profiles) {
            Ok(tool) => tool,
            Err(reason) => return self.declined_with(reason, past_turns),
        };

        // An area the drawing no longer encloses raises nothing, which is a
        // size that no longer holds.
        if turning.lost {
            self.broke(Broken::Operation(self.replaying));
        }
        if !self.shows(&tool, mode) {
            return past_turns(&mut self.body);
        }
        if let Err(reason) = self.combine(tool, mode) {
            self.declined_with(reason, past_turns);
        }
    }

    /// The matter a revolution would turn, alone, with nothing noted.
    pub(crate) fn turned_tool(
        &self,
        index: usize,
        areas: &[Area],
        axis: RevolutionAxis,
        degrees: &Formula,
    ) -> Option<Body> {
        let degrees = degrees.value(&self.values).filter(|turn| turns(*turn))?;
        let turning = self.turning(index, areas, axis, degrees)?;
        turning.tool(&self.body, &turning.profiles()).ok()
    }

    /// The areas of a sketch a turn of that many degrees about that axis
    /// stands on, each as the turn takes it.
    fn turning(
        &self,
        index: usize,
        areas: &[Area],
        axis: RevolutionAxis,
        degrees: f64,
    ) -> Option<Turning> {
        let sketch = self.sketches.get(index)?;
        let axis = axis_in_sketch(sketch, axis)?;
        let regions = sketch.regions();
        let (standing, lost) = self.all_standing_on(index, areas, &regions);
        Some(Turning {
            sides: sides_of(sketch, &standing, axis, degrees.to_radians()),
            lost,
            frame: frame_of(sketch),
        })
    }
}

/// Whether a turn of that many degrees turns anything, a whole turn at most.
fn turns(degrees: f64) -> bool {
    degrees.abs() > 1e-6 && degrees.abs() <= 360.0
}

/// What a turn is built from: each area as the turn takes it, with its
/// triangles and its turn, whether an area is lost, and the frame they stand
/// in.
struct Turning {
    sides: Vec<(Region, Vec<[DVec2; 3]>, Turn)>,
    lost: bool,
    frame: Frame,
}

impl Turning {
    fn profiles(&self) -> Vec<(Profile<'_>, Turn)> {
        self.sides
            .iter()
            .map(|(region, triangles, turn)| (profile(region, triangles), *turn))
            .collect()
    }

    /// Every side turned against the part's matter and joined into one tool.
    fn tool(&self, part: &Body, profiles: &[(Profile, Turn)]) -> Result<Body, Declined> {
        profiles
            .iter()
            .try_fold(Body::default(), |tool, (profile, turn)| {
                tool.union(&part.tool_turned(profile, self.frame, turn)?)
            })
    }
}

/// Where a revolution's axis lies, in the sketch's own coordinates.
fn axis_in_sketch(sketch: &Sketch, axis: RevolutionAxis) -> Option<Axis> {
    match axis {
        RevolutionAxis::Sketch(axis) => Some(Axis {
            origin: DVec2::ZERO,
            direction: axis.direction(),
        }),
        RevolutionAxis::Segment(segment) => {
            if segment.0 >= sketch.segments().len() {
                return None;
            }
            let (start, end) = sketch.endpoints(segment);
            ((end - start).length() > 1e-6).then_some(Axis {
                origin: start,
                direction: end - start,
            })
        }
    }
}

/// Each area as a turn takes it, with its triangles: whole when it lies on
/// one side of the axis, and cut along the axis into its sides when it lies
/// across it. Every side keeps the band its whole area was read at, so that
/// a corner the drawing left a hair off the axis lands on it whichever side
/// it ended up in. Turned whole, a side the other side's turn holds already
/// is left out.
fn sides_of(
    sketch: &Sketch,
    standing: &[&Region],
    axis: Axis,
    angle: f64,
) -> Vec<(Region, Vec<[DVec2; 3]>, Turn)> {
    let resolution = sketch.resolution();
    let mut sides = Vec::new();
    for &region in standing {
        let triangles = region.face_triangles();
        let whole = profile(region, &triangles);
        let turn = Turn::of(axis, angle, resolution, &whole);
        if turn.lie(&whole) != Lie::Across {
            sides.push((region.clone(), triangles, turn));
            continue;
        }
        let pieces: Vec<(Region, Vec<[DVec2; 3]>)> = sketch
            .pieces_across(region, axis.origin, axis.direction)
            .into_iter()
            .map(|side| {
                let triangles = side.face_triangles();
                (side, triangles)
            })
            .collect();
        let profiles: Vec<Profile> = pieces
            .iter()
            .map(|(side, triangles)| profile(side, triangles))
            .collect();
        let needed = turn.sides_needed(&profiles);
        for (index, (side, triangles)) in pieces.into_iter().enumerate() {
            if needed.contains(&index) {
                sides.push((side, triangles, turn));
            }
        }
    }
    sides
}
