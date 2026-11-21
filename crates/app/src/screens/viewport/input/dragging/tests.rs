//! What app · screens/viewport/input/dragging.rs is held to, driven through
//! the input pass a frame at a time, the way the canvas calls it.
//!
//! Closes #1057.
//! - Dragging an arc's centre leaves a fixed end where it stood; the same for
//!   an ellipse's centre and a fixed end of its axes —
//!   `an_arc_s_centre_dragged_leaves_its_fixed_end_where_it_stood`,
//!   `an_ellipse_s_centre_dragged_leaves_a_fixed_end_of_its_first_axis_where_it_stood`,
//!   `an_ellipse_s_centre_dragged_leaves_a_fixed_end_of_its_second_axis_where_it_stood`
//! - An arc with nothing fixed still travels whole with its centre —
//!   `an_arc_with_nothing_fixed_still_travels_whole_with_its_centre`
//! - A centre two arcs share, one holding a fixed end, still carries the other
//!   arc whole (surfaced by review) —
//!   `a_centre_shared_with_an_arc_holding_a_fixed_end_still_carries_the_free_arc_whole`
//! - A centre that also ends a trait to the fixed end is dragged as an ordinary
//!   point, and is shown where it is dropped (surfaced by review) —
//!   `a_centre_ending_a_trait_to_its_fixed_end_leaves_that_end_and_drops_where_it_is_shown`
//!
//! Closes #1065.
//! - Dragging that centre past the fixed end stops the centre short of it; the
//!   arc keeps a radius well above the drawing's tolerance —
//!   `a_centre_ending_a_trait_to_its_fixed_end_leaves_that_end_and_drops_where_it_is_shown`,
//!   `a_centre_ending_a_trait_dragged_far_past_its_fixed_end_stops_short_of_it`,
//!   and the same for an ellipse's centre ending a trait to a fixed end of its
//!   first axis —
//!   `an_ellipse_s_centre_ending_a_trait_to_a_fixed_end_of_its_axis_stops_short_of_it`
//! - The test
//!   `a_centre_ending_a_trait_to_its_fixed_end_leaves_that_end_and_drops_where_it_is_shown`
//!   gains that assertion — `assert_short_of`, before release and after the drop
//! - What the stop implies, surfaced by review: the centre stops near the
//!   fixed end, and a hand going further never brings it back —
//!   `an_arc_s_centre_dragged_ever_further_along_its_trait_stops_near_its_fixed_end_and_never_back`,
//!   `an_ellipse_s_centre_dragged_ever_further_along_its_trait_stops_near_its_fixed_end_and_never_back`
//! - What the stop must not break, surfaced by review: a curve's own handle
//!   still brings its size down with the hand —
//!   `an_arc_s_end_dragged_near_its_centre_with_the_key_that_lets_go_brings_the_radius_down_with_the_hand`,
//!   `an_ellipse_s_axis_end_dragged_near_its_centre_brings_that_axis_down_with_the_hand`;
//!   no test for a circle's rim point: the hand slides it round its circle,
//!   and the key lets it off the circle rather than resizing it;
//!   and a circle's centre slid along a trait through its fixed rim point,
//!   with no inside to turn out, follows the hand —
//!   `a_circle_s_centre_slid_along_a_trait_through_its_fixed_rim_point_follows_the_hand`
//! - Nor, surfaced by the second review: an ellipse's axis end dragged
//!   through its centre and beyond passes through and mirrors —
//!   `an_ellipse_s_axis_end_dragged_through_its_centre_comes_out_the_other_side_with_the_hand`;
//!   and an end of one of two arcs held equal, dragged with the key that lets
//!   go towards its centre, brings both radii down with the hand —
//!   `an_end_of_one_of_two_equal_arcs_dragged_near_its_centre_brings_both_radii_down_with_the_hand`
//! - Nor, surfaced by the third review: a centre ending no trait, dragged as
//!   #1057 drags it, follows the hand near its curve's fixed end and past it —
//!   `an_arc_s_bare_centre_dragged_near_and_past_its_fixed_end_follows_the_hand`,
//!   `an_ellipse_s_bare_centre_dragged_near_a_fixed_end_of_its_axis_follows_the_hand`
//! - Nor, surfaced by the last review: a centre ending a trait to a free point
//!   elsewhere, dragged near its arc's fixed end, follows the hand —
//!   `an_arc_s_centre_ending_a_trait_to_a_free_point_dragged_near_its_fixed_end_follows_the_hand`
//!
//! Closes #1072.
//! - Also: a merge the curve goes with still takes it — an arc's start
//!   dragged and dropped on its own centre is the merge #1043 and #1063
//!   decided, and takes the arc away rather than stopping a hair short of
//!   the centre —
//!   `an_arc_s_start_dropped_on_its_own_centre_takes_the_arc_as_the_merge_does`

use cao_part::history::PointRef;
use cao_part::{Operation, PartDocument};
use cao_prefs::Modifier;
use cao_sketch::{ArcId, Constraint, Element, EllipseId, PointId, Sketch, ToolState, WorkPlane};
use chrono::Utc;
use glam::DVec2;

use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, SketchPhase, Tool};
use crate::screens::viewport::input::handle_sketch_input;
use crate::screens::viewport::{ViewScale, ViewportState, to_ndc};

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

const AFTER_THE_SOLVER: f64 = 1e-6;

const THE_CENTRE: egui::Pos2 = egui::pos2(820.0, 260.0);

const THE_HAND: egui::Vec2 = egui::vec2(90.0, 40.0);

const FAR_PAST_THE_END: egui::Vec2 = egui::vec2(180.0, 0.0);

/// What is left of a curve's size, against what it had, below which it no
/// longer reads as the curve it was: a hundredth, a hundred thousand times the
/// drawing's tolerance.
const A_REAL_SIZE: f64 = 0.01;

const WIDE_CENTRE: egui::Pos2 = egui::pos2(400.0, 550.0);

const WIDE_REACH: f32 = 400.0;

const NEAR_ITS_CENTRE: f32 = 30.0;

const EVER_FURTHER: [f32; 7] = [0.5, 0.85, 0.905, 0.95, 1.1, 1.5, 2.0];

const A_HAIR: f64 = 0.01;

fn on_the_plane(state: &ViewportState, at: egui::Pos2) -> DVec2 {
    let (origin, direction) = state
        .camera
        .ray(to_ndc(at, SCREEN), SCREEN.width() / SCREEN.height());
    WorkPlane::XY
        .ray_intersection(origin.as_dvec3(), direction.as_dvec3())
        .expect("the place is on the drawing's plane")
}

fn on_the_screen(state: &ViewportState, place: DVec2) -> egui::Pos2 {
    let view_projection = state
        .camera
        .view_projection(SCREEN.width() / SCREEN.height());
    let clip = view_projection * WorkPlane::XY.to_world(place).as_vec3().extend(1.0);
    let ndc = clip.truncate() / clip.w;
    egui::pos2(
        SCREEN.center().x + ndc.x * SCREEN.width() * 0.5,
        SCREEN.center().y - ndc.y * SCREEN.height() * 0.5,
    )
}

fn frame(
    egui: &egui::Context,
    state: &mut ViewportState,
    context: &mut SketchContext<'_>,
    events: Vec<egui::Event>,
) {
    egui.begin_pass(egui::RawInput {
        screen_rect: Some(SCREEN),
        events,
        ..Default::default()
    });
    egui::Area::new(egui::Id::new("the canvas"))
        .fixed_pos(SCREEN.min)
        .show(egui, |ui| {
            ui.set_min_size(SCREEN.size());
            let response = ui.interact(
                SCREEN,
                egui::Id::new("the canvas's surface"),
                egui::Sense::click_and_drag(),
            );
            let scale = ViewScale::of(&state.camera, SCREEN, 1.0, &state.config, 1.0);
            handle_sketch_input(ui, state, &response, SCREEN, scale, context);
        });
    egui.end_pass().textures_delta.clear();
}

fn moved(to: egui::Pos2) -> Vec<egui::Event> {
    vec![egui::Event::PointerMoved(to)]
}

fn button(at: egui::Pos2, pressed: bool) -> Vec<egui::Event> {
    vec![egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }]
}

/// A drawing seen straight on, with the selection tool in hand.
struct FaceOn {
    state: ViewportState,
    document: PartDocument,
    editor: SketchEditor,
    extrusion: ExtrusionState,
    lang: Catalogue,
    egui: egui::Context,
}

impl FaceOn {
    fn empty() -> Self {
        let mut state = ViewportState::default();
        state
            .camera
            .set_view_angles(0.0, std::f32::consts::FRAC_PI_2);
        let mut document = PartDocument::new("part", Utc::now());
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        Self {
            state,
            document,
            editor: SketchEditor {
                phase: SketchPhase::Editing(0),
                tool: Tool::Select,
                tool_state: ToolState::Select(Box::default()),
                ..Default::default()
            },
            extrusion: ExtrusionState::default(),
            lang: Catalogue::french(),
            egui: egui::Context::default(),
        }
    }

    /// The centre on the plane, under `THE_CENTRE`, and how long sixty pixels
    /// are there.
    fn centre_and_reach(&self) -> (DVec2, f64) {
        let centre = on_the_plane(&self.state, THE_CENTRE);
        let reach = on_the_plane(&self.state, THE_CENTRE + egui::vec2(60.0, 0.0)).distance(centre);
        (centre, reach)
    }

    fn sketch(&self) -> &Sketch {
        &self.document.sketches()[0]
    }

    fn point_at(&self, place: DVec2) -> PointId {
        self.sketch()
            .nearest_point(place, 1e-6)
            .expect("a point stands there")
    }

    fn fix(&mut self, point: PointId) {
        self.document.apply(Operation::Constrain {
            sketch: 0,
            constraint: Constraint::Fixed {
                element: Element::Point(point),
            },
        });
    }

    /// An arc about `THE_CENTRE`, from a quarter turn to the next; its centre,
    /// start and end.
    fn with_an_arc() -> (Self, [PointId; 3]) {
        let mut bench = Self::empty();
        let (centre, reach) = bench.centre_and_reach();
        let (start, end) = (
            centre + DVec2::new(reach, 0.0),
            centre + DVec2::new(0.0, reach),
        );
        bench.document.apply(Operation::AddArc {
            sketch: 0,
            center: PointRef::New(centre),
            start: PointRef::New(start),
            end: PointRef::New(end),
            construction: false,
        });
        let points = [
            bench.point_at(centre),
            bench.point_at(start),
            bench.point_at(end),
        ];
        (bench, points)
    }

    /// The centre pressed on, carried `THE_HAND` across, and let go of; what
    /// was shown just before the release, when anything was.
    fn drag_the_centre(&mut self) -> Option<Sketch> {
        self.drag_the_centre_by(THE_HAND)
    }

    fn drag_the_centre_by(&mut self, hand: egui::Vec2) -> Option<Sketch> {
        let (centre, _) = self.centre_and_reach();
        self.drag_from(on_the_screen(&self.state, centre), hand)
    }

    fn drag_point_by(&mut self, point: PointId, hand: egui::Vec2) -> Option<Sketch> {
        let pressed = on_the_screen(&self.state, self.sketch().point(point));
        self.drag_from(pressed, hand)
    }

    fn drag_from(&mut self, pressed: egui::Pos2, hand: egui::Vec2) -> Option<Sketch> {
        let dropped = pressed + hand;
        self.frames(vec![
            vec![],
            moved(pressed),
            button(pressed, true),
            moved(pressed + egui::vec2(12.0, 12.0)),
            moved(dropped),
            vec![],
        ]);
        let shown = self
            .editor
            .select_state()
            .and_then(|state| state.drag_preview.clone());
        self.frames(vec![button(dropped, false), vec![]]);
        shown
    }

    fn hold_the_key_that_lets_go(&mut self) {
        let key = match self.state.let_go {
            Modifier::Command => egui::Modifiers::COMMAND,
            Modifier::Shift => egui::Modifiers::SHIFT,
            Modifier::Alt => egui::Modifiers::ALT,
        };
        self.frames(vec![vec![egui::Event::ModifiersChanged(key)]]);
    }

    fn add_a_construction_trait(&mut self, from: PointId, to: PointId) {
        self.document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(from),
            end: PointRef::Existing(to),
            construction: true,
        });
    }

    fn the_hand_on_the_plane(&self) -> DVec2 {
        let (centre, _) = self.centre_and_reach();
        let pressed = on_the_screen(&self.state, centre);
        on_the_plane(&self.state, pressed + THE_HAND) - on_the_plane(&self.state, pressed)
    }

    fn frames(&mut self, frames: Vec<Vec<egui::Event>>) {
        let mut context = SketchContext {
            document: &mut self.document,
            editor: &mut self.editor,
            extrusion: &mut self.extrusion,
            lang: &self.lang,
        };
        for events in frames {
            frame(&self.egui, &mut self.state, &mut context, events);
        }
    }
}

/// Every drawing the user sees the gesture leave: the one shown while the hand
/// still holds it, and the one it is dropped as.
fn seen(shown: &Option<Sketch>, dropped: &Sketch) -> Vec<(&'static str, Sketch)> {
    let mut seen = vec![("dropped", dropped.clone())];
    if let Some(shown) = shown {
        seen.push(("shown while dragged", shown.clone()));
    }
    seen
}

impl FaceOn {
    /// A quarter of a circle `radius` long about the arc's centre, laid on
    /// `centre`; its centre, start and end.
    fn add_an_arc(&mut self, centre: PointId, radius: f64) -> [PointId; 3] {
        let place = self.sketch().point(centre);
        let (start, end) = (
            place + DVec2::new(radius, 0.0),
            place + DVec2::new(0.0, radius),
        );
        self.document.apply(Operation::AddArc {
            sketch: 0,
            center: PointRef::Existing(centre),
            start: PointRef::New(start),
            end: PointRef::New(end),
            construction: false,
        });
        [centre, self.point_at(start), self.point_at(end)]
    }

    /// An ellipse about `THE_CENTRE`, its first axis level and twice as long
    /// as its second; its centre and the ends of its axes, first then second.
    fn with_an_ellipse() -> (Self, [PointId; 5]) {
        let mut bench = Self::empty();
        let (centre, reach) = bench.centre_and_reach();
        let (along, across) = (DVec2::new(reach, 0.0), DVec2::new(0.0, reach * 0.5));
        bench.document.apply(Operation::AddEllipse {
            sketch: 0,
            center: PointRef::New(centre),
            first: [PointRef::New(centre - along), PointRef::New(centre + along)],
            second: [
                PointRef::New(centre - across),
                PointRef::New(centre + across),
            ],
            construction: false,
            drawn: None,
        });
        let points = bench.sketch().ellipse_points(EllipseId(0));
        (bench, points)
    }
}

fn assert_stands(sketch: &Sketch, point: PointId, stood: DVec2, what: &str) {
    assert!(
        sketch.point(point).distance(stood) < AFTER_THE_SOLVER,
        "{what} stands at {}, not where it stood, {stood}",
        sketch.point(point)
    );
}

fn assert_round(sketch: &Sketch, [centre, start, end]: [PointId; 3], what: &str) {
    let (to_start, to_end) = (
        sketch.point(centre).distance(sketch.point(start)),
        sketch.point(centre).distance(sketch.point(end)),
    );
    assert!(
        (to_start - to_end).abs() < AFTER_THE_SOLVER,
        "{what} still runs from its centre through both its ends: \
         {to_start} to the start, {to_end} to the end"
    );
}

fn assert_a_true_ellipse(sketch: &Sketch, points: [PointId; 5], when: &str) {
    let [centre, first_start, first_end, second_start, second_end] =
        points.map(|point| sketch.point(point));
    for (axis, (one, other)) in [
        ("first", (first_start, first_end)),
        ("second", (second_start, second_end)),
    ] {
        assert!(
            ((one + other) / 2.0).distance(centre) < AFTER_THE_SOLVER,
            "{when}: the centre {centre} is still the middle of the {axis} axis, \
             from {one} to {other}"
        );
    }
    let (first, second) = (
        (first_end - first_start).normalize(),
        (second_end - second_start).normalize(),
    );
    assert!(
        first.dot(second).abs() < AFTER_THE_SOLVER,
        "{when}: the two axes still stand square: {first} against {second}"
    );
}

/// A curve about `WIDE_CENTRE`, `WIDE_REACH` pixels across, so that its
/// handles go well under a tenth of its size beyond the reach of any magnet.
impl FaceOn {
    fn wide_centre_and_reach(&self) -> (DVec2, f64) {
        let centre = on_the_plane(&self.state, WIDE_CENTRE);
        let reach =
            on_the_plane(&self.state, WIDE_CENTRE + egui::vec2(WIDE_REACH, 0.0)).distance(centre);
        (centre, reach)
    }

    fn near_its_centre(&self) -> f64 {
        on_the_plane(&self.state, WIDE_CENTRE + egui::vec2(NEAR_ITS_CENTRE, 0.0))
            .distance(on_the_plane(&self.state, WIDE_CENTRE))
    }

    fn with_a_wide_arc() -> (Self, [PointId; 3]) {
        let mut bench = Self::empty();
        let (centre, reach) = bench.wide_centre_and_reach();
        let (start, end) = (
            centre + DVec2::new(reach, 0.0),
            centre + DVec2::new(0.0, reach),
        );
        bench.document.apply(Operation::AddArc {
            sketch: 0,
            center: PointRef::New(centre),
            start: PointRef::New(start),
            end: PointRef::New(end),
            construction: false,
        });
        let points = [
            bench.point_at(centre),
            bench.point_at(start),
            bench.point_at(end),
        ];
        (bench, points)
    }

    /// A circle drawn by a point clicked on its rim; its centre and that point.
    fn with_a_wide_circle() -> (Self, [PointId; 2]) {
        let mut bench = Self::empty();
        let (centre, reach) = bench.wide_centre_and_reach();
        let rim = centre + DVec2::new(reach, 0.0);
        bench.document.apply(Operation::AddCircle {
            sketch: 0,
            center: PointRef::New(centre),
            radius: reach,
            rim: vec![PointRef::New(rim)],
            construction: false,
        });
        let points = [bench.point_at(centre), bench.point_at(rim)];
        (bench, points)
    }

    fn with_a_wide_ellipse() -> (Self, [PointId; 5]) {
        let mut bench = Self::empty();
        let (centre, reach) = bench.wide_centre_and_reach();
        let (along, across) = (DVec2::new(reach, 0.0), DVec2::new(0.0, reach * 0.5));
        bench.document.apply(Operation::AddEllipse {
            sketch: 0,
            center: PointRef::New(centre),
            first: [PointRef::New(centre - along), PointRef::New(centre + along)],
            second: [
                PointRef::New(centre - across),
                PointRef::New(centre + across),
            ],
            construction: false,
            drawn: None,
        });
        let points = bench.sketch().ellipse_points(EllipseId(0));
        (bench, points)
    }
}

fn assert_follows_the_hand(size: f64, asked: f64, was: f64, when: &str) {
    assert!(
        (size - asked).abs() < asked * 0.25,
        "{when}: the curve's size follows the hand down to {asked}, from {was}: it is {size}"
    );
}

/// How far along its trait to the fixed end the centre stops, as a share of
/// the way, for a hand going each of `EVER_FURTHER` of the way.
fn stops_along_the_trait(build: fn() -> (FaceOn, PointId, PointId)) -> Vec<(f32, f64)> {
    EVER_FURTHER
        .into_iter()
        .map(|fraction| {
            let (mut bench, centre, fixed) = build();
            let (was, stood) = (bench.sketch().point(centre), bench.sketch().point(fixed));
            bench.drag_point_by(centre, egui::vec2(WIDE_REACH * fraction, 0.0));
            let way = stood - was;
            let share = (bench.sketch().point(centre) - was).dot(way) / way.length_squared();
            (fraction, share)
        })
        .collect()
}

fn assert_stops_near_the_end_and_never_back(stops: &[(f32, f64)]) {
    for pair in stops.windows(2) {
        let [(nearer, before), (further, after)] = [pair[0], pair[1]];
        assert!(
            after > before - A_HAIR,
            "the centre never comes back as the hand goes further: a hand {nearer} of the way \
             to the fixed end leaves it {before} of the way, one {further} of the way {after} \
             — all of them: {stops:?}"
        );
    }
    for (fraction, share) in stops.iter().filter(|(fraction, _)| *fraction >= 0.85) {
        assert!(
            *share > 0.5 && *share < 1.0,
            "a hand {fraction} of the way to the fixed end stops the centre short of it, \
             nearer it than where it was pressed: {share} of the way — all of them: {stops:?}"
        );
    }
}

/// The centre, slid along its trait towards the fixed end it stood `was` from,
/// stays on its own side of that end and far enough from it for the curve to
/// keep a real size.
fn assert_short_of(sketch: &Sketch, centre: PointId, end: DVec2, was: DVec2, when: &str) {
    let (now, then) = (sketch.point(centre) - end, was - end);
    assert!(
        now.dot(then) > 0.0,
        "{when}: the centre stops short of the fixed end {end}, on the side it came \
         from: it stands at {}",
        sketch.point(centre)
    );
    assert!(
        now.length() > then.length() * A_REAL_SIZE,
        "{when}: the curve keeps a real size: {} from its centre to the fixed end, \
         against {} before the drag",
        now.length(),
        then.length()
    );
}

/// The centre lands where the hand let it go, give or take what the magnets
/// take of the cursor.
fn assert_at_the_hand(travelled: DVec2, hand: DVec2, when: &str) {
    assert!(
        travelled.distance(hand) < hand.length() * 0.05,
        "{when}: the centre follows the hand: it travelled {travelled}, the hand {hand}"
    );
}

#[test]
fn an_arc_s_centre_dragged_leaves_its_fixed_end_where_it_stood() {
    let (mut bench, arc) = FaceOn::with_an_arc();
    let [centre, start, _] = arc;
    bench.fix(start);
    let (stood, was) = (bench.sketch().point(start), bench.sketch().point(centre));
    let hand = bench.the_hand_on_the_plane();

    let shown = bench.drag_the_centre();

    for (when, sketch) in seen(&shown, bench.sketch()) {
        assert_stands(&sketch, start, stood, &format!("{when}: the fixed end"));
        assert_round(&sketch, arc, &format!("{when}: the arc"));
        assert_at_the_hand(sketch.point(centre) - was, hand, when);
    }
}

fn an_ellipse_s_centre_dragged_leaves_a_fixed_end_where_it_stood(fixed_at: usize) {
    let (mut bench, points) = FaceOn::with_an_ellipse();
    let fixed = points[fixed_at];
    bench.fix(fixed);
    let (stood, was) = (bench.sketch().point(fixed), bench.sketch().point(points[0]));
    let hand = bench.the_hand_on_the_plane();

    let shown = bench.drag_the_centre();

    for (when, sketch) in seen(&shown, bench.sketch()) {
        assert_stands(&sketch, fixed, stood, &format!("{when}: the fixed end"));
        assert_a_true_ellipse(&sketch, points, when);
        assert_at_the_hand(sketch.point(points[0]) - was, hand, when);
    }
}

#[test]
fn an_ellipse_s_centre_dragged_leaves_a_fixed_end_of_its_first_axis_where_it_stood() {
    an_ellipse_s_centre_dragged_leaves_a_fixed_end_where_it_stood(2);
}

#[test]
fn an_ellipse_s_centre_dragged_leaves_a_fixed_end_of_its_second_axis_where_it_stood() {
    an_ellipse_s_centre_dragged_leaves_a_fixed_end_where_it_stood(4);
}

#[test]
fn an_arc_with_nothing_fixed_still_travels_whole_with_its_centre() {
    let (mut bench, points) = FaceOn::with_an_arc();
    let before = points.map(|point| bench.sketch().point(point));
    let hand = bench.the_hand_on_the_plane();

    bench.drag_the_centre();

    let travelled = bench.sketch().point(points[0]) - before[0];
    assert_at_the_hand(travelled, hand, "dropped");
    for (what, (point, was)) in ["the start", "the end"]
        .into_iter()
        .zip(points[1..].iter().zip(&before[1..]))
    {
        assert!(
            (bench.sketch().point(*point) - *was).distance(travelled) < AFTER_THE_SOLVER,
            "{what} travels as its centre did, by {travelled}: it travelled {}",
            bench.sketch().point(*point) - *was
        );
    }
}

#[test]
fn a_centre_shared_with_an_arc_holding_a_fixed_end_still_carries_the_free_arc_whole() {
    let (mut bench, outer) = FaceOn::with_an_arc();
    let [centre, fixed, _] = outer;
    let (_, reach) = bench.centre_and_reach();
    let inner = bench.add_an_arc(centre, reach * 0.5);
    bench.fix(fixed);
    let before = inner.map(|point| bench.sketch().point(point));
    let stood = bench.sketch().point(fixed);
    let hand = bench.the_hand_on_the_plane();

    let shown = bench.drag_the_centre();

    for (when, sketch) in seen(&shown, bench.sketch()) {
        assert_stands(
            &sketch,
            fixed,
            stood,
            &format!("{when}: the outer arc's fixed end"),
        );
        assert_round(&sketch, outer, &format!("{when}: the outer arc"));
        let travelled = sketch.point(centre) - before[0];
        assert_at_the_hand(travelled, hand, when);
        for (what, (point, was)) in ["start", "end"]
            .into_iter()
            .zip(inner[1..].iter().zip(&before[1..]))
        {
            assert!(
                (sketch.point(*point) - *was).distance(travelled) < AFTER_THE_SOLVER,
                "{when}: the inner arc's {what} travels as the centre did, by {travelled}: \
                 it travelled {}",
                sketch.point(*point) - *was
            );
        }
    }
}

#[test]
fn a_centre_ending_a_trait_to_its_fixed_end_leaves_that_end_and_drops_where_it_is_shown() {
    let (mut bench, arc) = FaceOn::with_an_arc();
    let [centre, start, _] = arc;
    bench.add_a_construction_trait(centre, start);
    bench.fix(start);
    let (stood, was) = (bench.sketch().point(start), bench.sketch().point(centre));

    let shown = bench
        .drag_the_centre()
        .expect("the centre is dragged, and shown where it goes");

    for (when, sketch) in seen(&Some(shown.clone()), bench.sketch()) {
        assert_stands(&sketch, start, stood, &format!("{when}: the fixed end"));
        assert_round(&sketch, arc, &format!("{when}: the arc"));
        let off_the_trait = (sketch.point(centre) - stood).perp_dot((was - stood).normalize());
        assert!(
            off_the_trait.abs() < AFTER_THE_SOLVER,
            "{when}: the centre slides along its trait to the fixed end, an ordinary point \
             the trait holds: it stands {off_the_trait} off it"
        );
        assert_short_of(&sketch, centre, stood, was, when);
    }
    for point in arc {
        assert!(
            shown.point(point).distance(bench.sketch().point(point)) < AFTER_THE_SOLVER,
            "the arc is dropped where it was shown: {point:?} shown at {}, dropped at {}",
            shown.point(point),
            bench.sketch().point(point)
        );
    }
}

#[test]
fn a_centre_ending_a_trait_dragged_far_past_its_fixed_end_stops_short_of_it() {
    let (mut bench, arc) = FaceOn::with_an_arc();
    let [centre, start, _] = arc;
    bench.add_a_construction_trait(centre, start);
    bench.fix(start);
    let (stood, was) = (bench.sketch().point(start), bench.sketch().point(centre));

    let shown = bench.drag_the_centre_by(FAR_PAST_THE_END);

    for (when, sketch) in seen(&shown, bench.sketch()) {
        assert_stands(&sketch, start, stood, &format!("{when}: the fixed end"));
        assert_round(&sketch, arc, &format!("{when}: the arc"));
        assert_short_of(&sketch, centre, stood, was, when);
    }
}

#[test]
fn an_ellipse_s_centre_ending_a_trait_to_a_fixed_end_of_its_axis_stops_short_of_it() {
    let (mut bench, points) = FaceOn::with_an_ellipse();
    let (centre, fixed) = (points[0], points[2]);
    bench.add_a_construction_trait(centre, fixed);
    bench.fix(fixed);
    let (stood, was) = (bench.sketch().point(fixed), bench.sketch().point(centre));

    let shown = bench.drag_the_centre();

    for (when, sketch) in seen(&shown, bench.sketch()) {
        assert_stands(&sketch, fixed, stood, &format!("{when}: the fixed end"));
        assert_a_true_ellipse(&sketch, points, when);
        assert_short_of(&sketch, centre, stood, was, when);
    }
}

#[test]
fn an_arc_s_centre_dragged_ever_further_along_its_trait_stops_near_its_fixed_end_and_never_back() {
    let stops = stops_along_the_trait(|| {
        let (mut bench, [centre, start, _]) = FaceOn::with_a_wide_arc();
        bench.add_a_construction_trait(centre, start);
        bench.fix(start);
        (bench, centre, start)
    });

    assert_stops_near_the_end_and_never_back(&stops);
}

#[test]
fn an_ellipse_s_centre_dragged_ever_further_along_its_trait_stops_near_its_fixed_end_and_never_back()
 {
    let stops = stops_along_the_trait(|| {
        let (mut bench, points) = FaceOn::with_a_wide_ellipse();
        let (centre, fixed) = (points[0], points[2]);
        bench.add_a_construction_trait(centre, fixed);
        bench.fix(fixed);
        (bench, centre, fixed)
    });

    assert_stops_near_the_end_and_never_back(&stops);
}

#[test]
fn an_arc_s_end_dragged_near_its_centre_with_the_key_that_lets_go_brings_the_radius_down_with_the_hand()
 {
    let (mut bench, arc) = FaceOn::with_a_wide_arc();
    let [centre, start, _] = arc;
    let was = bench
        .sketch()
        .point(centre)
        .distance(bench.sketch().point(start));
    let asked = bench.near_its_centre();
    bench.hold_the_key_that_lets_go();

    let shown = bench.drag_point_by(start, egui::vec2(NEAR_ITS_CENTRE - WIDE_REACH, 0.0));

    for (when, sketch) in seen(&shown, bench.sketch()) {
        assert_round(&sketch, arc, &format!("{when}: the arc"));
        let radius = sketch.point(centre).distance(sketch.point(start));
        assert_follows_the_hand(radius, asked, was, when);
    }
}

#[test]
fn an_ellipse_s_axis_end_dragged_near_its_centre_brings_that_axis_down_with_the_hand() {
    let (mut bench, points) = FaceOn::with_a_wide_ellipse();
    let (centre, end) = (points[0], points[2]);
    let was = bench
        .sketch()
        .point(centre)
        .distance(bench.sketch().point(end));
    let asked = bench.near_its_centre();

    let shown = bench.drag_point_by(end, egui::vec2(NEAR_ITS_CENTRE - WIDE_REACH, 0.0));

    for (when, sketch) in seen(&shown, bench.sketch()) {
        assert_a_true_ellipse(&sketch, points, when);
        let half = sketch.point(centre).distance(sketch.point(end));
        assert_follows_the_hand(half, asked, was, when);
    }
}

#[test]
fn a_circle_s_centre_slid_along_a_trait_through_its_fixed_rim_point_follows_the_hand() {
    for fraction in [0.925, 1.075] {
        let (mut bench, [centre, rim]) = FaceOn::with_a_wide_circle();
        bench.add_a_construction_trait(centre, rim);
        bench.fix(rim);
        let was = bench.sketch().point(centre);
        let hand = on_the_plane(
            &bench.state,
            WIDE_CENTRE + egui::vec2(WIDE_REACH * fraction, 0.0),
        ) - on_the_plane(&bench.state, WIDE_CENTRE);

        bench.drag_point_by(centre, egui::vec2(WIDE_REACH * fraction, 0.0));

        assert_at_the_hand(
            bench.sketch().point(centre) - was,
            hand,
            &format!("a hand {fraction} of the way to the rim point"),
        );
    }
}

#[test]
fn an_ellipse_s_axis_end_dragged_through_its_centre_comes_out_the_other_side_with_the_hand() {
    for beyond in [0.1f32, 0.5, 0.9] {
        let (mut bench, points) = FaceOn::with_a_wide_ellipse();
        let (centre, end) = (points[0], points[2]);
        let (stood, was) = (bench.sketch().point(centre), bench.sketch().point(end));
        let asked = on_the_plane(
            &bench.state,
            WIDE_CENTRE - egui::vec2(WIDE_REACH * beyond, 0.0),
        )
        .distance(on_the_plane(&bench.state, WIDE_CENTRE));

        let shown = bench.drag_point_by(end, egui::vec2(-WIDE_REACH * (1.0 + beyond), 0.0));

        for (when, sketch) in seen(&shown, bench.sketch()) {
            let when = format!("a hand {beyond} past the centre, {when}");
            assert_a_true_ellipse(&sketch, points, &when);
            let (now, then) = (sketch.point(end) - stood, was - stood);
            assert!(
                now.dot(then) < 0.0,
                "{when}: the end comes out on the other side of the centre {stood}: \
                 it stands at {}",
                sketch.point(end)
            );
            assert_follows_the_hand(now.length(), asked, then.length(), &when);
        }
    }
}

#[test]
fn an_end_of_one_of_two_equal_arcs_dragged_near_its_centre_brings_both_radii_down_with_the_hand() {
    let (mut bench, first) = FaceOn::with_a_wide_arc();
    let [centre, start, _] = first;
    let (place, reach) = bench.wide_centre_and_reach();
    let other = place + DVec2::new(reach * 1.125, 0.0);
    bench.document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(other),
        start: PointRef::New(other + DVec2::new(reach, 0.0)),
        end: PointRef::New(other + DVec2::new(0.0, reach)),
        construction: false,
    });
    bench.document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::EqualRadiusArc {
            first: ArcId(0),
            second: ArcId(1),
        },
    });
    let was = bench
        .sketch()
        .point(centre)
        .distance(bench.sketch().point(start));
    let asked = bench.near_its_centre();
    bench.hold_the_key_that_lets_go();

    let shown = bench.drag_point_by(start, egui::vec2(NEAR_ITS_CENTRE - WIDE_REACH, 0.0));

    for (when, sketch) in seen(&shown, bench.sketch()) {
        for (which, arc) in [("first", ArcId(0)), ("second", ArcId(1))] {
            assert_follows_the_hand(
                sketch.arc_radius(arc),
                asked,
                was,
                &format!("{when}: the {which} arc"),
            );
        }
    }
}

fn the_wide_hand_on_the_plane(bench: &FaceOn, hand: egui::Vec2) -> DVec2 {
    on_the_plane(&bench.state, WIDE_CENTRE + hand) - on_the_plane(&bench.state, WIDE_CENTRE)
}

#[test]
fn an_arc_s_bare_centre_dragged_near_and_past_its_fixed_end_follows_the_hand() {
    for hand in [
        egui::vec2(WIDE_REACH * 0.92, -WIDE_REACH * 0.02),
        egui::vec2(WIDE_REACH * 1.1, 0.0),
    ] {
        let (mut bench, arc) = FaceOn::with_a_wide_arc();
        let [centre, start, _] = arc;
        bench.fix(start);
        let (stood, was) = (bench.sketch().point(start), bench.sketch().point(centre));
        let on_the_plane = the_wide_hand_on_the_plane(&bench, hand);

        let shown = bench.drag_point_by(centre, hand);

        for (when, sketch) in seen(&shown, bench.sketch()) {
            let when = format!("a hand {hand:?} across, {when}");
            assert_stands(&sketch, start, stood, &format!("{when}: the fixed end"));
            assert_round(&sketch, arc, &format!("{when}: the arc"));
            assert_at_the_hand(sketch.point(centre) - was, on_the_plane, &when);
        }
    }
}

#[test]
fn an_ellipse_s_bare_centre_dragged_near_a_fixed_end_of_its_axis_follows_the_hand() {
    for hand in [
        egui::vec2(WIDE_REACH * 0.92, 0.0),
        egui::vec2(WIDE_REACH * 0.92, -WIDE_REACH * 0.03),
    ] {
        let (mut bench, points) = FaceOn::with_a_wide_ellipse();
        let (centre, fixed) = (points[0], points[2]);
        bench.fix(fixed);
        let (stood, was) = (bench.sketch().point(fixed), bench.sketch().point(centre));
        let on_the_plane = the_wide_hand_on_the_plane(&bench, hand);

        let shown = bench.drag_point_by(centre, hand);

        for (when, sketch) in seen(&shown, bench.sketch()) {
            let when = format!("a hand {hand:?} across, {when}");
            assert_stands(&sketch, fixed, stood, &format!("{when}: the fixed end"));
            assert_a_true_ellipse(&sketch, points, &when);
            assert_at_the_hand(sketch.point(centre) - was, on_the_plane, &when);
        }
    }
}

#[test]
fn an_arc_s_centre_ending_a_trait_to_a_free_point_dragged_near_its_fixed_end_follows_the_hand() {
    let (mut bench, arc) = FaceOn::with_a_wide_arc();
    let [centre, start, _] = arc;
    let (place, reach) = bench.wide_centre_and_reach();
    bench.document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(centre),
        end: PointRef::New(place - DVec2::new(reach, reach * 0.5)),
        construction: false,
    });
    bench.fix(start);
    bench.state.config.grid_snap = false;
    let (stood, was) = (bench.sketch().point(start), bench.sketch().point(centre));
    let hand = egui::vec2(WIDE_REACH * 0.92, 0.0);
    let to = was + the_wide_hand_on_the_plane(&bench, hand);
    let two_pixels = 2.0 * reach / f64::from(WIDE_REACH);

    let shown = bench.drag_point_by(centre, hand);

    for (when, sketch) in seen(&shown, bench.sketch()) {
        assert_stands(&sketch, start, stood, &format!("{when}: the fixed end"));
        assert_round(&sketch, arc, &format!("{when}: the arc"));
        assert!(
            sketch.point(centre).distance(to) < two_pixels,
            "{when}: the centre lands under the hand, nothing pulling the cursor: \
             it stands at {}, the hand at {to}, a radius of {} left against {} under the hand",
            sketch.point(centre),
            sketch.point(centre).distance(stood),
            to.distance(stood)
        );
    }
}

#[test]
fn an_arc_s_start_dropped_on_its_own_centre_takes_the_arc_as_the_merge_does() {
    let (mut bench, [centre, start, _]) = FaceOn::with_an_arc();
    let pressed = on_the_screen(&bench.state, bench.sketch().point(start));
    let dropped = on_the_screen(&bench.state, bench.sketch().point(centre));

    bench.drag_from(pressed, dropped - pressed);

    let arcs: Vec<f64> = bench
        .sketch()
        .live_arcs()
        .map(|(arc, _)| bench.sketch().arc_radius(arc))
        .collect();
    assert!(arcs.is_empty(), "the arc is still drawn, {arcs:?} round");
}
