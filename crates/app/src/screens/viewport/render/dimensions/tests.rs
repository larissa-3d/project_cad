//! What app · screens/viewport/render/dimensions.rs is held to.
//!
//! Closes #724.
//! - found in review: every number in JetBrains Mono, as "To try it" promises —
//!   the value a dimension carries is written in the monospace family —
//!   `a_dimension_s_value_is_written_in_the_monospace_family`
//!
//! Closes #742 — the whole transcription is in
//! `crates/app/src/screens/viewport/render/drawing/tests.rs`; these bullets
//! are the values a dimension writes, which egui paints.
//! - **Dimensions**: the value in a cartouche — a small rounded box, filled and
//!   bordered in the dimension colour, painted behind the value and around it —
//!   `a_dimension_s_value_sits_in_a_cartouche`; its text in the dimension
//!   colour, which a profile sets, opacity and all, and a readout's in the
//!   read-only one's — `a_dimension_s_value_is_written_in_the_dimension_colour`
//! - must not break: a read-only dimension still looks read-only — with the
//!   « Par défaut » profile it is still written between brackets, and its
//!   colour stands apart from a driving one's by more than the 1.26:1 the issue
//!   calls "a hair apart" — `a_read_only_dimension_still_reads_apart_from_a_driving_one`
//! - found in review: a fillet's and a chamfer's values kept the chrome's
//!   amber, which is the tool in progress's alone — a cut's value is written
//!   as a dimension's is, in the profile's dimension colour, opacity and all,
//!   in a cartouche bordered in it —
//!   `a_cut_s_value_is_written_like_a_dimension_s_in_its_colour_and_its_cartouche`

use cao_part::history::PointRef;
use cao_part::{Operation, PartDocument};
use cao_prefs::theme::{Background, Rgba, Theme};
use cao_sketch::{DimensionTarget, SegmentId, WorkPlane};
use glam::DVec2;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

/// A plate forty by twenty whose bottom side is held at forty.
fn a_dimensioned_plate() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 40.0.into(),
        placement: None,
    });
    document
}

/// The text one pass of `egui` paints for the plate's dimensions, its sketch
/// open, each with the family it is written in.
fn the_plate_s_dimensions_written() -> Vec<(String, egui::FontFamily)> {
    let mut document = a_dimensioned_plate();
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let state = ViewportState::default();
    let egui = egui::Context::default();
    let mut output = egui::FullOutput::default();
    for _ in 0..2 {
        output.textures_delta.clear();
        egui.begin_pass(egui::RawInput {
            screen_rect: Some(SCREEN),
            ..Default::default()
        });
        egui::Area::new(egui::Id::new("the canvas"))
            .fixed_pos(SCREEN.min)
            .fade_in(false)
            .show(&egui, |ui| {
                paint_dimension_labels(ui, &state, SCREEN, &context, None);
            });
        output = egui.end_pass();
    }
    output.textures_delta.clear();
    let mut written = Vec::new();
    let mut pending: Vec<egui::Shape> = output
        .shapes
        .into_iter()
        .map(|clipped| clipped.shape)
        .collect();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(shapes) => pending.extend(shapes),
            egui::Shape::Text(text) => {
                for section in &text.galley.job.sections {
                    written.push((
                        text.galley.text().to_string(),
                        section.format.font_id.family.clone(),
                    ));
                }
            }
            _ => {}
        }
    }
    written
}

#[test]
fn a_dimension_s_value_is_written_in_the_monospace_family() {
    let written = the_plate_s_dimensions_written();

    assert!(
        written.iter().any(|(text, _)| holds_a_figure(text))
            && written
                .iter()
                .filter(|(text, _)| holds_a_figure(text))
                .all(|(_, family)| *family == egui::FontFamily::Monospace),
        "a dimension's value is written {written:?}: the figures go in the monospace family",
    );
}

/// Whether a piece of text holds a figure: a label may set a word beside it,
/// « Distance » or « R », and the word may stay in the proportional family.
fn holds_a_figure(text: &str) -> bool {
    text.chars().any(|character| character.is_ascii_digit())
}

/// A right triangle from (10, 10), its base forty, its side thirty and the
/// right angle between them given, then its hypotenuse given fifty: nothing
/// was left for that length to set, so it only reads what the drawing holds.
fn a_triangle_with_a_readout() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 10.0)),
        end: PointRef::New(DVec2::new(50.0, 10.0)),
        construction: false,
    });
    let base = document.sketches()[0].segments()[0];
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(base.end),
        end: PointRef::New(DVec2::new(50.0, 40.0)),
        construction: false,
    });
    let top = document.sketches()[0].segments()[1].end;
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(top),
        end: PointRef::Existing(base.start),
        construction: false,
    });
    for (target, value) in [
        (DimensionTarget::Length(SegmentId(0)), 40.0),
        (DimensionTarget::Length(SegmentId(1)), 30.0),
        (DimensionTarget::corner(SegmentId(0), SegmentId(1)), 90.0),
        (DimensionTarget::Length(SegmentId(2)), 50.0),
    ] {
        document.apply(Operation::SetDimension {
            sketch: 0,
            target,
            value: value.into(),
            placement: None,
        });
    }
    document
}

/// Every shape one pass of `egui` paints for the values of a part's first
/// sketch, open, under `theme`, in the order they were painted.
fn the_values_painted(mut document: PartDocument, theme: &Theme) -> Vec<egui::Shape> {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let mut state = ViewportState::default();
    state.theme = theme.clone();
    let egui = egui::Context::default();
    let mut output = egui::FullOutput::default();
    for _ in 0..2 {
        output.textures_delta.clear();
        egui.begin_pass(egui::RawInput {
            screen_rect: Some(SCREEN),
            ..Default::default()
        });
        egui::Area::new(egui::Id::new("the canvas"))
            .fixed_pos(SCREEN.min)
            .fade_in(false)
            .show(&egui, |ui| {
                paint_dimension_labels(ui, &state, SCREEN, &context, None);
            });
        output = egui.end_pass();
    }
    output.textures_delta.clear();
    let mut painted = Vec::new();
    for clipped in output.shapes {
        flatten(clipped.shape, &mut painted);
    }
    painted
}

/// egui hands a layer back as one shape holding the rest, so what a painter
/// added is a leaf of that tree rather than an entry of the list.
fn flatten(shape: egui::Shape, out: &mut Vec<egui::Shape>) {
    match shape {
        egui::Shape::Vec(shapes) => shapes.into_iter().for_each(|shape| flatten(shape, out)),
        shape => out.push(shape),
    }
}

/// Each value painted, by its text, with the rank it was painted at and the
/// colour it reaches the screen in: the shape is tessellated, so whatever egui
/// does to it on the way is already applied.
fn values_written(painted: &[egui::Shape]) -> Vec<(String, usize, egui::Color32)> {
    painted
        .iter()
        .enumerate()
        .filter_map(|(rank, shape)| match shape {
            egui::Shape::Text(text) if holds_a_figure(text.galley.text()) => {
                let mut tessellated = Default::default();
                egui::epaint::Tessellator::new(1.0, Default::default(), [1, 1], Vec::new())
                    .tessellate_text(text, &mut tessellated);
                let ink = tessellated.vertices.first().map(|vertex| vertex.color)?;
                Some((text.galley.text().to_string(), rank, ink))
            }
            _ => None,
        })
        .collect()
}

/// Whether two colours are one to the rounding a premultiplied opacity costs:
/// a channel or two either way.
fn near(painted: egui::Color32, wanted: egui::Color32) -> bool {
    let [painted, wanted] = [painted, wanted].map(|colour| colour.to_srgba_unmultiplied());
    painted
        .iter()
        .zip(wanted)
        .all(|(painted, wanted)| painted.abs_diff(wanted) <= 2)
}

#[test]
fn a_dimension_s_value_sits_in_a_cartouche() {
    let theme = Theme::default();
    let painted = the_values_painted(a_dimensioned_plate(), &theme);
    let Some((said, rank)) = painted
        .iter()
        .enumerate()
        .find_map(|(rank, shape)| match shape {
            egui::Shape::Text(text) if holds_a_figure(text.galley.text()) => Some((text, rank)),
            _ => None,
        })
    else {
        panic!("the plate's value is not written at all: {painted:?}");
    };
    let value = egui::Rect::from_min_size(said.pos, said.galley.size());

    let around: Vec<&egui::epaint::RectShape> = painted[..rank]
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(shaped)
                if shaped.rect.expand(0.5).contains_rect(value)
                    && value.expand(12.0).contains_rect(shaped.rect) =>
            {
                Some(shaped)
            }
            _ => None,
        })
        .collect();

    assert!(
        !around.is_empty(),
        "nothing is painted behind « {} » and around it: the value stands bare on the canvas",
        said.galley.text(),
    );
    assert!(
        around
            .iter()
            .any(|shaped| shaped.corner_radius != egui::CornerRadius::ZERO),
        "the box behind the value has square corners, where a cartouche is rounded: {around:?}",
    );
    assert!(
        around.iter().any(|shaped| shaped.fill.a() > 0),
        "the box behind the value is not filled: {around:?}",
    );
    let border = to_color_of(theme.dimension);
    assert!(
        around
            .iter()
            .any(|shaped| shaped.stroke.width > 0.0 && near(shaped.stroke.color, border)),
        "the box behind the value is bordered in no dimension colour, {border:?}: {around:?}",
    );
}

#[test]
fn a_dimension_s_value_is_written_in_the_dimension_colour() {
    let theme = Theme {
        dimension: Rgba::new(0.20, 0.80, 0.40, 0.70),
        dimension_driven: Rgba::new(0.85, 0.30, 0.60, 0.90),
        ..Theme::default()
    };
    let document = a_triangle_with_a_readout();
    let dimensions = document.sketches()[0].dimensions();
    assert!(
        dimensions.iter().any(|dimension| dimension.driven)
            && dimensions.iter().any(|dimension| !dimension.driven),
        "the fixture is meant to hold a value and a readout: {dimensions:?}",
    );

    let written = values_written(&the_values_painted(document, &theme));

    let (driving, read_only) = (
        to_color_of(theme.dimension),
        to_color_of(theme.dimension_driven),
    );
    let wrong: Vec<String> = written
        .iter()
        .filter_map(|(text, _, ink)| {
            let wanted = match text.starts_with('(') {
                true => read_only,
                false => driving,
            };
            (!near(*ink, wanted)).then(|| format!("« {text} » is written {ink:?}, not {wanted:?}"))
        })
        .collect();
    assert!(
        written.len() >= 2 && wrong.is_empty(),
        "a value is written in the profile's dimension colour and a readout in its read-only \
         one, opacity and all:\n  {}\nwritten: {written:?}",
        wrong.join("\n  "),
    );
}

/// The issue calls the old free and settled strokes, 1.26:1 apart, "a hair
/// apart in brightness": a readout has to stand further off than that.
const MORE_THAN_A_HAIR: f32 = 1.3;

/// How bright `ink` reads laid over `behind`: egui blends a premultiplied
/// colour in gamma space, and what comes out is weighed by its relative
/// luminance — the sRGB definition contrast is measured with.
fn brightness_over(ink: egui::Color32, behind: Rgba) -> f32 {
    let [r, g, b, alpha] = ink.to_normalized_gamma_f32();
    let linear = |channel: f32| match channel <= 0.04045 {
        true => channel / 12.92,
        false => ((channel + 0.055) / 1.055).powf(2.4),
    };
    let seen = |ink: f32, behind: f32| linear(ink + (1.0 - alpha) * behind);
    0.2126 * seen(r, behind.r) + 0.7152 * seen(g, behind.g) + 0.0722 * seen(b, behind.b)
}

#[test]
fn a_read_only_dimension_still_reads_apart_from_a_driving_one() {
    let theme = Theme::default();
    let Background::Solid(canvas) = theme.sketch_background else {
        panic!("the default sketch background is no plain colour to read against");
    };
    let document = a_triangle_with_a_readout();
    assert!(
        document.sketches()[0]
            .dimensions()
            .iter()
            .any(|dimension| dimension.driven),
        "the fixture is meant to hold a readout",
    );

    let written = values_written(&the_values_painted(document, &theme));

    let readouts: Vec<_> = written
        .iter()
        .filter(|(text, _, _)| text.starts_with('(') && text.ends_with(')'))
        .collect();
    let values: Vec<_> = written
        .iter()
        .filter(|(text, _, _)| !text.starts_with('('))
        .collect();
    assert!(
        !readouts.is_empty() && !values.is_empty(),
        "a readout is written between brackets beside a value written bare: {written:?}",
    );
    for (readout, _, quiet) in &readouts {
        for (value, _, ink) in &values {
            let (seen, quiet_seen) = (
                brightness_over(*ink, canvas),
                brightness_over(*quiet, canvas),
            );
            let apart = (seen.max(quiet_seen) + 0.05) / (seen.min(quiet_seen) + 0.05);
            assert!(
                apart > MORE_THAN_A_HAIR,
                "« {readout} » ({quiet:?}) reads {apart:.2}:1 from « {value} » ({ink:?}) on the \
                 sketch background: a readout no longer looks apart from a value",
            );
        }
    }
}

/// A plate a hundred by sixty, the corner of its foot and its right side
/// chamfered 20 along the first and 30 along the second, and its top-left
/// corner rounded by 20: two cuts, each writing its value, and no dimension.
fn a_plate_with_a_chamfer_and_a_fillet() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(100.0, 60.0)),
        construction: false,
    });
    document.apply(Operation::Chamfer {
        sketch: 0,
        corners: vec![cao_sketch::Corner::Between(
            SegmentId(0).into(),
            SegmentId(1).into(),
        )],
        mode: cao_part::history::ChamferAsked::from(cao_sketch::Chamfer::Sided {
            first: 20.0,
            second: 30.0,
        }),
        construction: false,
    });
    document.apply(Operation::Fillet {
        sketch: 0,
        corners: vec![cao_sketch::Corner::Between(
            SegmentId(2).into(),
            SegmentId(3).into(),
        )],
        radius: 20.0.into(),
        construction: false,
    });
    document
}

#[test]
fn a_cut_s_value_is_written_like_a_dimension_s_in_its_colour_and_its_cartouche() {
    let theme = Theme {
        dimension: Rgba::new(0.20, 0.80, 0.40, 0.70),
        ..Theme::default()
    };
    let painted = the_values_painted(a_plate_with_a_chamfer_and_a_fillet(), &theme);
    let written = values_written(&painted);
    assert!(
        written.iter().any(|(text, _, _)| text.starts_with('R'))
            && written.iter().any(|(text, _, _)| text.contains('×')),
        "the fillet's and the chamfer's values are both written: {written:?}",
    );

    let colour = to_color_of(theme.dimension);
    let mut wrong = Vec::new();
    for (text, rank, ink) in &written {
        if !near(*ink, colour) {
            wrong.push(format!("« {text} » is written {ink:?}, not {colour:?}"));
        }
        let egui::Shape::Text(said) = &painted[*rank] else {
            unreachable!("a value written is a text");
        };
        let value = egui::Rect::from_min_size(said.pos, said.galley.size());
        let around: Vec<&egui::epaint::RectShape> = painted[..*rank]
            .iter()
            .filter_map(|shape| match shape {
                egui::Shape::Rect(shaped)
                    if shaped.rect.expand(0.5).contains_rect(value)
                        && value.expand(12.0).contains_rect(shaped.rect) =>
                {
                    Some(shaped)
                }
                _ => None,
            })
            .collect();
        let in_a_cartouche = around
            .iter()
            .any(|shaped| shaped.corner_radius != egui::CornerRadius::ZERO)
            && around.iter().any(|shaped| shaped.fill.a() > 0)
            && around
                .iter()
                .any(|shaped| shaped.stroke.width > 0.0 && near(shaped.stroke.color, colour));
        if !in_a_cartouche {
            wrong.push(format!(
                "« {text} » sits in no cartouche bordered {colour:?}: {around:?}"
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "a cut's value is written as a dimension's is, in the profile's dimension colour, \
         opacity and all, in a cartouche — amber is the tool in progress's alone:\n  {}",
        wrong.join("\n  "),
    );
}
