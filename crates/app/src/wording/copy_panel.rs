//! What the steps of the copy tools' panel say, and what the drawing says the
//! panel's current step asks for.

use cao_prefs::Command;
use cao_sketch::{ChosenAxis, Element, SketchAxis};

use crate::lang::Catalogue;
use crate::wording::command;
use crate::wording::dimension::short;

/// An element held, by the name its row in the part goes by; a point, which
/// has no row, by its own number.
pub fn element(lang: &Catalogue, element: Element) -> String {
    let (key, rank) = match element {
        Element::Point(point) => ("copy_panel.point", point.0),
        Element::Segment(segment) => ("part_tree.segment", segment.0),
        Element::Circle(circle) => ("part_tree.circle", circle.0),
        Element::Arc(arc) => ("part_tree.arc", arc.0),
        Element::Ellipse(ellipse) => ("part_tree.ellipse", ellipse.0),
    };
    lang.t_with(key, &[("rank", &(rank + 1).to_string())])
}

/// The centre chosen, by where it stands in millimetres.
pub fn centre_at(lang: &Catalogue, (x, y): (f64, f64)) -> String {
    lang.t_with(
        "copy_panel.centre_at",
        &[("x", &short(x)), ("y", &short(y))],
    )
}

/// The axis or the direction chosen: a trait by the name its row in the part
/// goes by, or one of the sketch's own two axes.
pub fn axis(lang: &Catalogue, axis: ChosenAxis) -> String {
    match axis {
        ChosenAxis::Trait(segment) => element(lang, Element::Segment(segment)),
        ChosenAxis::Sketch(SketchAxis::U) => lang.t("copy_panel.horizontal_axis"),
        ChosenAxis::Sketch(SketchAxis::V) => lang.t("copy_panel.vertical_axis"),
    }
}

/// What a numbered step of a copy tool's panel asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asked {
    Elements,
    Centre,
    Axis,
    Direction,
    CountAndStep,
    Rotation,
}

/// The message over the drawing while a step of a copy tool's panel is
/// current: the tool, the step's number, and what it asks for. The circular
/// pattern names its elements and its number at more length.
pub fn step_asks(lang: &Catalogue, tool: Command, number: usize, asked: Asked) -> String {
    let circular = tool == Command::ToolCircularPattern;
    let asked = lang.t(match asked {
        Asked::Elements if circular => "copy_panel.asks_elements_to_copy",
        Asked::Elements => "copy_panel.asks_elements",
        Asked::Centre => "copy_panel.asks_centre",
        Asked::Axis => "copy_panel.asks_axis",
        Asked::Direction => "copy_panel.asks_direction",
        Asked::CountAndStep if circular => "copy_panel.asks_count",
        Asked::CountAndStep => "copy_panel.asks_count_and_step",
        Asked::Rotation => "copy_panel.asks_rotation",
    });
    let tool = command::label(lang, tool);
    lang.t_with(
        "copy_panel.step_asks",
        &[
            ("tool", &tool),
            ("number", &number.to_string()),
            ("asked", &asked),
        ],
    )
}
