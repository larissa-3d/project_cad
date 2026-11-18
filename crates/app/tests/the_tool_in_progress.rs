//! The tool in progress as the whole application paints it over the canvas:
//! what egui writes beside the drawing while a line is drawn, read back off
//! the shapes of the frame.
//!
//! Closes #742 — the whole transcription is in
//! `crates/app/src/screens/viewport/render/drawing/tests.rs`; these bullets
//! are what egui paints of the tool in progress.
//! - a snap ring with its name (« Coïncidence », « Milieu », « Tangence »…) —
//!   the name, beside the ring, in a pill bordered amber: a point
//!   « Coïncidence », a middle « Milieu », a crossing « Intersection », a
//!   centre « Centre », a place on a curve « Sur la courbe »; each place the
//!   fixture hovers is checked first to pull the cursor as it says, which
//!   also holds that which snap wins is untouched —
//!   `each_snap_is_named_beside_its_ring`
//! - an angle arc with its value on the line (today only on the arc) — the
//!   angle the arc spans, written by its start in a pill bordered amber, the
//!   way every other angle is written, « 90° » for a line square to the
//!   horizontal and « 45° » for one halfway — `a_line_in_progress_writes_its_angle_on_its_arc`
//! - the length field by the cursor with an amber border — a frame bordered in
//!   the tool's amber stands round the length field the cursor carries —
//!   `the_length_field_by_the_cursor_has_an_amber_border`; typing in it, and
//!   in the angle field, is held as it was by
//!   `a_number_typed_while_drawing_goes_into_the_field` and
//!   `a_name_typed_straight_into_the_field_lays_the_line_to_it` in
//!   `crates/app/tests/variables.rs`
//! - found in review: a snap's name stood half hidden under the fields the
//!   cursor carries once a line was begun — its pill and their popup do not
//!   overlap — `a_snap_s_name_stands_clear_of_the_fields_the_cursor_carries`
//! - found in review: a snap's ring and name outlived the cursor, the last
//!   place it was pulled onto still named with the pointer off the canvas —
//!   no snap is named once the pointer has left it, whichever snap it was —
//!   `no_snap_is_named_once_the_pointer_has_left_the_canvas`; the ring, in
//!   `no_ring_is_drawn_once_the_cursor_has_left_the_canvas` in
//!   `crates/app/src/screens/viewport/render/drawing/tests.rs`
//! - found in the real window: the angle on the arc was written to four
//!   places, « 10.008° » — it is written to a tenth of a degree, whole when
//!   whole, and the angle field keeps what is typed in it to the last figure —
//!   `the_angle_on_the_arc_is_written_to_a_tenth_of_a_degree`
//! - found in the real window: the angle stood under the fields the cursor
//!   carries, the cursor just by the start of the line — its pill and their
//!   popup do not overlap with the cursor above, below, left or right of the
//!   start — `the_angle_s_pill_stands_clear_of_the_fields_the_cursor_carries`
//! - found in the real window: the fillet tool, picking a corner, named its
//!   snap « Coïncidence » — the fillet and the dimension tools, which pick
//!   what is drawn, name no snap — `a_tool_that_picks_what_is_drawn_names_no_snap`
//!
//! The amber is the profile's tool colour, `tool_in_progress`, whose default
//! is the design file's amber. The tests open on a profile that sets it to a
//! teal no token of the chrome wears: a focused field is already bordered in
//! the chrome's amber, and that border is not the tool's. A border may wear
//! the tool's colour at any opacity, as the amber pills of the circular
//! pattern do.

mod driver;

use cao_part::history::PointRef;
use cao_part::{Operation, PartDocument};
use cao_prefs::{Rgba, Settings};
use cao_sketch::{Corner, SegmentId, Snap, SnapSettings, WorkPlane};
use egui_kittest::kittest::Queryable;
use glam::DVec2;

/// The tool's colour the tests' profile sets, as bytes.
const TOOL: [u8; 3] = [40, 180, 160];

/// Settings whose tool colour is [`TOOL`].
fn a_profile_with_its_own_tool_colour() -> Settings {
    let mut settings = Settings::default();
    let [r, g, b] = TOOL.map(|channel| f32::from(channel) / 255.0);
    settings.theme.tool_in_progress = Rgba::opaque(r, g, b);
    settings
}

/// A plate a hundred by sixty on XY, its top-left corner rounded by 20, and a
/// trait standing upright across it at 60, from below its foot to above its
/// top.
fn a_rounded_plate_crossed_by_a_trait() -> PartDocument {
    let mut part = PartDocument::new("Plaque", "2026-11-18T09:00:00Z".parse().expect("a date"));
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    part.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(100.0, 60.0)),
        construction: false,
    });
    part.apply(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(2).into(), SegmentId(3).into())],
        radius: 20.0.into(),
        construction: false,
    });
    part.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(60.0, -20.0)),
        end: PointRef::New(DVec2::new(60.0, 80.0)),
        construction: false,
    });
    part
}

/// Each place the fixture hovers, the snap it pulls the cursor onto, and the
/// name that snap is written under.
const PLACES: [(DVec2, &str, &str); 5] = [
    (DVec2::new(100.0, 60.0), "a point", "Coïncidence"),
    (DVec2::new(100.0, 30.0), "a middle", "Milieu"),
    (DVec2::new(60.0, 60.0), "a crossing", "Intersection"),
    (DVec2::new(20.0, 40.0), "a centre", "Centre"),
    (
        DVec2::new(100.0, 45.0),
        "a place on a curve",
        "Sur la courbe",
    ),
];

/// Which snap the drawing pulls a cursor at `at` onto, by the words of
/// [`PLACES`]: read off the sketch itself, so that the fixture says what it
/// is before the screen is asked anything.
fn pulled_onto(document: &PartDocument, at: DVec2) -> &'static str {
    let reach = SnapSettings {
        point_reach: 0.5,
        curve_reach: 0.5,
        grid_step: None,
        grid_reach: 0.0,
    };
    match document.sketches()[0].magnetise(at, &reach).1 {
        Some(Snap::Point) => "a point",
        Some(Snap::Midpoint(_)) => "a middle",
        Some(Snap::Crossing(_)) => "a crossing",
        Some(Snap::Centre(_)) => "a centre",
        Some(Snap::OnCurve(_)) => "a place on a curve",
        None => "nothing",
    }
}

/// The plate opened from the start menu, its sketch open, the line tool in
/// hand and nothing begun.
fn the_line_tool_over_the_plate(test: &str) -> driver::App {
    let mut app = driver::open_with(
        test,
        egui::vec2(1920.0, 800.0),
        a_profile_with_its_own_tool_colour(),
    );
    driver::create_a_part(&mut app, "Plaque");
    driver::the_part_put_away(&mut app, test, "Plaque");
    a_rounded_plate_crossed_by_a_trait()
        .save(
            &cao_app::adapters::files::DiskFiles,
            &driver::the_part_s_file(test, "Plaque"),
            "2026-11-18T10:00:00Z".parse().expect("a date"),
        )
        .expect("the part is written");
    app.query_all_by_label("Plaque")
        .last()
        .expect("the start menu lists the part")
        .click();
    app.run();
    driver::double_click(&mut app, "Esquisse 1");
    driver::click(&mut app, "Ligne (L)");
    app
}

/// Every shape the last frame painted, in the order it was painted: egui hands
/// a layer back as one shape holding the rest.
fn painted(app: &driver::App) -> Vec<egui::Shape> {
    fn flatten(shape: egui::Shape, out: &mut Vec<egui::Shape>) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.into_iter().for_each(|shape| flatten(shape, out)),
            shape => out.push(shape),
        }
    }
    let mut out = Vec::new();
    for clipped in &app.output().shapes {
        flatten(clipped.shape.clone(), &mut out);
    }
    out
}

/// Whether a colour is the tool's, at whatever opacity: a channel or two
/// either way is what a premultiplied opacity costs.
fn is_the_tool_s(colour: egui::Color32) -> bool {
    let [r, g, b, _] = colour.to_srgba_unmultiplied();
    [r, g, b]
        .iter()
        .zip(TOOL)
        .all(|(worn, tool)| worn.abs_diff(tool) <= 3)
}

/// Where the last frame wrote exactly `text` in a pill bordered in the tool's
/// colour — a box painted before it, around it, with a stroke of that colour —
/// on screen.
fn pills_of_the_tool_reading(app: &driver::App, text: &str) -> Vec<egui::Rect> {
    let shapes = painted(app);
    shapes
        .iter()
        .enumerate()
        .filter_map(|(rank, shape)| match shape {
            egui::Shape::Text(written) if written.galley.text() == text => {
                let at = egui::Rect::from_min_size(written.pos, written.galley.size());
                shapes[..rank]
                    .iter()
                    .any(|shape| {
                        matches!(shape, egui::Shape::Rect(pill)
                            if pill.rect.expand(0.5).contains_rect(at)
                                && at.expand(16.0).contains_rect(pill.rect)
                                && pill.stroke.width > 0.0
                                && is_the_tool_s(pill.stroke.color))
                    })
                    .then_some(at)
            }
            _ => None,
        })
        .collect()
}

/// Every piece of text the last frame painted within `reach` of `at`.
fn written_near(app: &driver::App, at: egui::Pos2, reach: f32) -> Vec<String> {
    painted(app)
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Text(text)
                if egui::Rect::from_min_size(text.pos, text.galley.size()).distance_to_pos(at)
                    <= reach =>
            {
                Some(text.galley.text().to_string())
            }
            _ => None,
        })
        .collect()
}

#[test]
fn each_snap_is_named_beside_its_ring() {
    let test = "each_snap_is_named_beside_its_ring";
    let fixture = a_rounded_plate_crossed_by_a_trait();
    for (at, snap, _) in PLACES {
        assert_eq!(
            pulled_onto(&fixture, at),
            snap,
            "the fixture is meant to pull a cursor at {at:?} onto {snap}",
        );
    }
    let mut app = the_line_tool_over_the_plate(test);

    let mut unnamed = Vec::new();
    for (at, snap, name) in PLACES {
        driver::hover_at_sketch_point(&mut app, at);
        app.run();
        let ring = driver::sketch_point_on_screen(&app, at);
        let beside = pills_of_the_tool_reading(&app, name)
            .into_iter()
            .any(|pill| pill.distance_to_pos(ring) <= 40.0);
        if !beside {
            unnamed.push(format!(
                "{snap} at {at:?}, on screen at {ring:?}, is not named « {name} » beside it; \
                 the frame writes near it {:?}",
                written_near(&app, ring, 80.0),
            ));
        }
    }

    assert!(
        unnamed.is_empty(),
        "a snap is named, in a pill bordered in the tool's colour, within 40 px of its \
         ring:\n  {}",
        unnamed.join("\n  "),
    );
}

#[test]
fn a_line_in_progress_writes_its_angle_on_its_arc() {
    let mut app = driver::open_held_with(
        "a_line_in_progress_writes_its_angle_on_its_arc",
        a_profile_with_its_own_tool_colour(),
    );
    driver::create_a_part(&mut app, "Equerre");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Ligne (L)");
    driver::click_at_sketch_point(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);

    let mut unwritten = Vec::new();
    for (cursor, angle) in [
        (DVec2::new(-25.0, -62.5), "90°"),
        (DVec2::new(87.5, -25.0), "45°"),
    ] {
        driver::hover_at_sketch_point(&mut app, cursor);
        app.run();
        let start = driver::sketch_point_on_screen(&app, driver::A_POINT_ABOVE_THE_ORIGIN);
        let hand = driver::sketch_point_on_screen(&app, cursor);
        assert!(
            start.distance(hand) > 250.0,
            "the fixture keeps the cursor well away from the start, where the fields it \
             carries cannot be taken for the arc's pill: {start:?} and {hand:?}",
        );
        let by_the_start = pills_of_the_tool_reading(&app, angle)
            .into_iter()
            .any(|pill| pill.distance_to_pos(start) <= 120.0);
        if !by_the_start {
            unwritten.push(format!(
                "a line from {:?} to {cursor:?} does not write « {angle} » within 120 px of \
                 its start, at {start:?}; the frame writes near it {:?}",
                driver::A_POINT_ABOVE_THE_ORIGIN,
                written_near(&app, start, 160.0),
            ));
        }
    }

    assert!(
        unwritten.is_empty(),
        "a line in progress writes the angle its arc spans by its start, in a pill bordered \
         in the tool's colour:\n  {}",
        unwritten.join("\n  "),
    );
}

#[test]
fn the_length_field_by_the_cursor_has_an_amber_border() {
    let mut app = driver::open_held_with(
        "the_length_field_by_the_cursor_has_an_amber_border",
        a_profile_with_its_own_tool_colour(),
    );
    driver::create_a_part(&mut app, "Regle");
    driver::start_a_sketch(&mut app);
    driver::start_a_line(&mut app);
    let hand = driver::sketch_point_on_screen(&app, driver::A_POINT_FURTHER_ALONG);
    let carried = egui::Rect::from_min_size(hand, egui::vec2(480.0, 120.0));
    let mut fields: Vec<egui::Rect> = app
        .query_all_by_role(egui::accesskit::Role::TextInput)
        .map(|field| field.rect())
        .filter(|field| carried.contains_rect(*field))
        .collect();
    fields.sort_by(|one, other| one.min.x.total_cmp(&other.min.x));
    assert_eq!(
        fields.len(),
        2,
        "a line in progress carries its length and its angle by the cursor, at {hand:?}: \
         {fields:?}",
    );
    let length = fields[0];

    let framed = painted(&app).iter().any(|shape| {
        matches!(shape, egui::Shape::Rect(frame)
            if frame.rect.expand(0.5).contains_rect(length)
                && frame.rect.height() <= length.height() + 40.0
                && frame.rect.width() <= 600.0
                && frame.stroke.width >= 1.0
                && is_the_tool_s(frame.stroke.color))
    });

    let borders: Vec<(egui::Rect, egui::Stroke)> = painted(&app)
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(frame) if frame.rect.expand(0.5).contains_rect(length) => {
                Some((frame.rect, frame.stroke))
            }
            _ => None,
        })
        .collect();
    assert!(
        framed,
        "no border of the tool's colour, {TOOL:?}, a point wide or more, stands round the \
         length field at {length:?}; what stands round it: {borders:?}",
    );
}

/// The pills the last frame wrote exactly `text` in: each box painted before
/// the text, around it, with a stroke of the tool's colour.
fn the_pills_around(app: &driver::App, text: &str) -> Vec<egui::Rect> {
    let shapes = painted(app);
    shapes
        .iter()
        .enumerate()
        .filter_map(|(rank, shape)| match shape {
            egui::Shape::Text(written) if written.galley.text() == text => {
                let at = egui::Rect::from_min_size(written.pos, written.galley.size());
                shapes[..rank].iter().find_map(|shape| match shape {
                    egui::Shape::Rect(pill)
                        if pill.rect.expand(0.5).contains_rect(at)
                            && at.expand(16.0).contains_rect(pill.rect)
                            && pill.stroke.width > 0.0
                            && is_the_tool_s(pill.stroke.color) =>
                    {
                        Some(pill.rect)
                    }
                    _ => None,
                })
            }
            _ => None,
        })
        .collect()
}

#[test]
fn a_snap_s_name_stands_clear_of_the_fields_the_cursor_carries() {
    let mut app = driver::open_held_with(
        "a_snap_s_name_stands_clear_of_the_fields_the_cursor_carries",
        a_profile_with_its_own_tool_colour(),
    );
    driver::create_a_part(&mut app, "Equerre");
    driver::start_a_sketch(&mut app);
    driver::start_a_line(&mut app);

    driver::hover_at_sketch_point(&mut app, DVec2::ZERO);
    app.run();

    let hand = driver::sketch_point_on_screen(&app, DVec2::ZERO);
    let carried = egui::Rect::from_min_size(hand, egui::vec2(480.0, 120.0));
    let fields: Vec<egui::Rect> = app
        .query_all_by_role(egui::accesskit::Role::TextInput)
        .map(|field| field.rect())
        .filter(|field| carried.contains_rect(*field))
        .collect();
    assert_eq!(
        fields.len(),
        2,
        "a line under way carries its length and its angle by the cursor, at {hand:?}: \
         {fields:?}",
    );
    let popup = painted(&app)
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(frame)
                if fields
                    .iter()
                    .all(|field| frame.rect.expand(0.5).contains_rect(*field)) =>
            {
                Some(frame.rect)
            }
            _ => None,
        })
        .min_by(|one, other| one.area().total_cmp(&other.area()))
        .unwrap_or_else(|| fields[0].union(fields[1]));
    let pills = the_pills_around(&app, "Coïncidence");
    assert!(
        !pills.is_empty(),
        "the cursor on the sketch's origin, a point, is named « Coïncidence »: the frame \
         writes near it {:?}",
        written_near(&app, hand, 120.0),
    );

    let under: Vec<&egui::Rect> = pills.iter().filter(|pill| pill.intersects(popup)).collect();
    assert!(
        under.is_empty(),
        "the name « Coïncidence », in {under:?}, stands under the fields the cursor carries, \
         in {popup:?}",
    );
}

#[test]
fn no_snap_is_named_once_the_pointer_has_left_the_canvas() {
    let test = "no_snap_is_named_once_the_pointer_has_left_the_canvas";
    let mut app = the_line_tool_over_the_plate(test);
    let off_the_canvas = app
        .query_all_by_label("Ligne (L)")
        .next()
        .expect("the ribbon offers the line tool, outside the canvas")
        .rect()
        .center();

    let mut outlived = Vec::new();
    for (at, snap, name) in PLACES {
        driver::hover_at_sketch_point(&mut app, at);
        app.run();
        assert!(
            !the_pills_around(&app, name).is_empty(),
            "the fixture names {snap} « {name} » while the cursor is on it",
        );

        app.hover_at(off_the_canvas);
        app.run();
        app.run();

        if !the_pills_around(&app, name).is_empty() {
            outlived.push(format!("{snap} at {at:?}, « {name} »"));
        }
    }

    assert!(
        outlived.is_empty(),
        "with the pointer off the canvas, at {off_the_canvas:?}, the last snap is still \
         named:\n  {}",
        outlived.join("\n  "),
    );
}

/// Every text the last frame wrote in a pill bordered in the tool's colour,
/// with the pill it sits in.
fn the_tool_s_pills(app: &driver::App) -> Vec<(String, egui::Rect)> {
    let shapes = painted(app);
    shapes
        .iter()
        .enumerate()
        .filter_map(|(rank, shape)| match shape {
            egui::Shape::Text(written) => {
                let at = egui::Rect::from_min_size(written.pos, written.galley.size());
                shapes[..rank]
                    .iter()
                    .find_map(|shape| match shape {
                        egui::Shape::Rect(pill)
                            if pill.rect.expand(0.5).contains_rect(at)
                                && at.expand(16.0).contains_rect(pill.rect)
                                && pill.stroke.width > 0.0
                                && is_the_tool_s(pill.stroke.color) =>
                        {
                            Some(pill.rect)
                        }
                        _ => None,
                    })
                    .map(|pill| (written.galley.text().to_string(), pill))
            }
            _ => None,
        })
        .collect()
}

/// The angles written in the tool's pills within 120 points of `start`.
fn angles_written_by(app: &driver::App, start: egui::Pos2) -> Vec<(String, egui::Rect)> {
    the_tool_s_pills(app)
        .into_iter()
        .filter(|(text, pill)| text.ends_with('°') && pill.distance_to_pos(start) <= 120.0)
        .collect()
}

/// The two fields the cursor carries while a line is drawn, left to right, as
/// the cursor stands at `hand` on screen.
fn the_fields_carried(app: &driver::App, hand: egui::Pos2) -> Vec<egui::Rect> {
    let carried = egui::Rect::from_min_size(hand, egui::vec2(480.0, 120.0));
    let mut fields: Vec<egui::Rect> = app
        .query_all_by_role(egui::accesskit::Role::TextInput)
        .map(|field| field.rect())
        .filter(|field| carried.contains_rect(*field))
        .collect();
    fields.sort_by(|one, other| one.min.x.total_cmp(&other.min.x));
    fields
}

/// The popup holding `fields`: the smallest box painted round both.
fn the_popup_round(app: &driver::App, fields: &[egui::Rect]) -> egui::Rect {
    painted(app)
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(frame)
                if fields
                    .iter()
                    .all(|field| frame.rect.expand(0.5).contains_rect(*field)) =>
            {
                Some(frame.rect)
            }
            _ => None,
        })
        .min_by(|one, other| one.area().total_cmp(&other.area()))
        .unwrap_or_else(|| fields[0].union(fields[1]))
}

/// A line begun at `A_POINT_ABOVE_THE_ORIGIN` on YZ, in a part opened on the
/// tests' profile.
fn a_line_begun(test: &str) -> driver::App {
    let mut app = driver::open_held_with(test, a_profile_with_its_own_tool_colour());
    driver::create_a_part(&mut app, "Equerre");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Ligne (L)");
    driver::click_at_sketch_point(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    app
}

#[test]
fn the_angle_on_the_arc_is_written_to_a_tenth_of_a_degree() {
    let mut app = a_line_begun("the_angle_on_the_arc_is_written_to_a_tenth_of_a_degree");
    let start = driver::sketch_point_on_screen(&app, driver::A_POINT_ABOVE_THE_ORIGIN);

    driver::hover_at_sketch_point(&mut app, driver::A_POINT_FURTHER_ALONG);
    app.run();
    let read: Vec<String> = angles_written_by(&app, start)
        .into_iter()
        .map(|(text, _)| text)
        .collect();
    assert_eq!(
        read,
        ["26.6°"],
        "a line from {:?} to {:?} leans 26.565…° off the horizontal: its arc reads it to a \
         tenth of a degree",
        driver::A_POINT_ABOVE_THE_ORIGIN,
        driver::A_POINT_FURTHER_ALONG,
    );

    let hand = driver::sketch_point_on_screen(&app, driver::A_POINT_FURTHER_ALONG);
    let fields = the_fields_carried(&app, hand);
    assert_eq!(
        fields.len(),
        2,
        "the line carries its length and its angle: {fields:?}"
    );
    if let Some(angle) = app
        .query_all_by_role(egui::accesskit::Role::TextInput)
        .find(|field| field.rect() == fields[1])
    {
        angle.focus();
    }
    app.run();
    app.event(egui::Event::Text("26.56".to_owned()));
    app.run();
    app.run();

    assert!(
        driver::fields(&app).iter().any(|held| held == "26.56"),
        "the angle field keeps what is typed in it, to the last figure: {:?}",
        driver::fields(&app),
    );
    let read: Vec<String> = angles_written_by(&app, start)
        .into_iter()
        .map(|(text, _)| text)
        .collect();
    assert_eq!(
        read,
        ["26.6°"],
        "26.56 typed in the angle field, the arc reads it to a tenth of a degree",
    );
}

#[test]
fn the_angle_s_pill_stands_clear_of_the_fields_the_cursor_carries() {
    let mut app = a_line_begun("the_angle_s_pill_stands_clear_of_the_fields_the_cursor_carries");
    let start = driver::sketch_point_on_screen(&app, driver::A_POINT_ABOVE_THE_ORIGIN);
    let from = driver::A_POINT_ABOVE_THE_ORIGIN;

    let mut under = Vec::new();
    for (side, cursor) in [
        ("above", from + DVec2::new(0.0, 12.5)),
        ("below", from + DVec2::new(0.0, -12.5)),
        ("left", from + DVec2::new(-12.5, 0.0)),
        ("right", from + DVec2::new(12.5, 0.0)),
    ] {
        driver::hover_at_sketch_point(&mut app, cursor);
        app.run();
        let hand = driver::sketch_point_on_screen(&app, cursor);
        let fields = the_fields_carried(&app, hand);
        if fields.len() != 2 {
            under.push(format!(
                "the cursor {side} the start carries no two fields: {fields:?}"
            ));
            continue;
        }
        let popup = the_popup_round(&app, &fields);
        let pills = angles_written_by(&app, start);
        if pills.is_empty() {
            under.push(format!(
                "the cursor {side} the start, no angle is written by it"
            ));
        }
        for (text, pill) in pills {
            if pill.intersects(popup) {
                under.push(format!(
                    "the cursor {side} the start: « {text} », in {pill:?}, stands under the \
                     fields, in {popup:?}"
                ));
            }
        }
    }

    assert!(
        under.is_empty(),
        "the angle by the start of a line stands clear of the fields its cursor carries:\n  {}",
        under.join("\n  "),
    );
}

#[test]
fn a_tool_that_picks_what_is_drawn_names_no_snap() {
    let test = "a_tool_that_picks_what_is_drawn_names_no_snap";
    let mut app = the_line_tool_over_the_plate(test);

    let mut named = Vec::new();
    for tool in ["Congé", "Cote (D)"] {
        driver::click_in_the_ribbon(&mut app, tool);
        for (at, snap, name) in PLACES {
            driver::hover_at_sketch_point(&mut app, at);
            app.run();
            if !the_pills_around(&app, name).is_empty() {
                named.push(format!(
                    "« {tool} » over {snap} at {at:?} names it « {name} »"
                ));
            }
        }
    }

    assert!(
        named.is_empty(),
        "a tool that takes what is drawn, rather than laying a point, names no snap — the \
         ring and its name say where a point is about to be laid:\n  {}",
        named.join("\n  "),
    );
}
