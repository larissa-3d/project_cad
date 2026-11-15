//! What the section view's panel says, and what the message pill says while
//! the section waits on one of its steps.

use cao_prefs::Command;
use cao_render::MatterInFront;

use crate::lang::Catalogue;

/// Which side of the cutting plane a row is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Behind,
    InFront,
}

/// A side's row: which side, and how its matter is drawn.
pub fn side(lang: &Catalogue, side: Side, state: MatterInFront) -> String {
    let state = super::matter::state(lang, state);
    let key = match side {
        Side::Behind => "section.behind",
        Side::InFront => "section.in_front",
    };
    lang.t_with(key, &[("state", &state)])
}

/// What a numbered step of the section view's panel asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asked {
    Face,
    Distance,
    Tilts,
    Sides,
}

/// The message over the view while a step of the section is current.
pub fn step_asks(lang: &Catalogue, number: usize, asked: Asked) -> String {
    let asked = lang.t(match asked {
        Asked::Face => "section.asks_face",
        Asked::Distance => "section.asks_distance",
        Asked::Tilts => "section.asks_tilts",
        Asked::Sides => "section.asks_sides",
    });
    super::steps::asks(lang, Command::SectionView, number, &asked)
}
