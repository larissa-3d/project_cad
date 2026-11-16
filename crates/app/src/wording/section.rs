//! What the section view's panel says, and what the message pill says while
//! the section waits on one of its steps.

use cao_prefs::Command;

use crate::lang::Catalogue;

/// Which side of the cutting plane a row is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Behind,
    InFront,
}

/// A side's name, which the state its matter is drawn in stands under.
pub fn side(lang: &Catalogue, side: Side) -> String {
    lang.t(match side {
        Side::Behind => "section.behind",
        Side::InFront => "section.in_front",
    })
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
