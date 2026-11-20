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

use cao_part::history::PointRef;
use cao_part::{Operation, PartDocument};
use cao_sketch::{Constraint, Element, EllipseId, PointId, Sketch, ToolState, WorkPlane};
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
        let (centre, _) = self.centre_and_reach();
        let pressed = on_the_screen(&self.state, centre);
        let dropped = pressed + THE_HAND;
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
    bench.document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(centre),
        end: PointRef::Existing(start),
        construction: true,
    });
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
