//! The section view's numbered steps, as its panel lays them out: the face it
//! cuts along, the distance, the two tilts, the two sides. Which one is
//! current, how each reads, and where `Entrée` goes.

use std::cmp::Ordering;

use cao_part::PartDocument;

use super::Section;
use crate::lang::Catalogue;
use crate::screens::sketch::StepState;
use crate::wording::section::{self as said, Asked};

/// A numbered step of the section view's panel, top to bottom. « Enregistrer
/// la coupe » is the panel's main button, not a step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SectionStep {
    Face,
    Distance,
    Tilts,
    Sides,
}

const STEPS: [SectionStep; 4] = [
    SectionStep::Face,
    SectionStep::Distance,
    SectionStep::Tilts,
    SectionStep::Sides,
];

/// Where `Entrée` stands, top to bottom: a step, or one of the two fields of
/// the tilts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum SectionField {
    #[default]
    Face,
    Distance,
    TiltAboutU,
    TiltAboutV,
    Sides,
}

impl SectionField {
    pub(crate) fn step(self) -> SectionStep {
        match self {
            Self::Face => SectionStep::Face,
            Self::Distance => SectionStep::Distance,
            Self::TiltAboutU | Self::TiltAboutV => SectionStep::Tilts,
            Self::Sides => SectionStep::Sides,
        }
    }

    fn below(self) -> Option<Self> {
        match self {
            Self::Face => Some(Self::Distance),
            Self::Distance => Some(Self::TiltAboutU),
            Self::TiltAboutU => Some(Self::TiltAboutV),
            Self::TiltAboutV => Some(Self::Sides),
            Self::Sides => None,
        }
    }
}

impl Section {
    /// Each step, top to bottom, with how its dot reads: current where
    /// `Entrée` stands, done above it once it holds what it asks for, to come
    /// otherwise.
    pub(crate) fn steps(&self) -> Vec<(SectionStep, StepState)> {
        let current = self.field.step() as usize;
        STEPS
            .into_iter()
            .map(|step| {
                let state = match (step as usize).cmp(&current) {
                    Ordering::Equal => StepState::Current,
                    Ordering::Less if self.holds(step) => StepState::Done,
                    _ => StepState::ToCome,
                };
                (step, state)
            })
            .collect()
    }

    /// Every step holds what it asks for, but the face while one is still to
    /// be clicked.
    fn holds(&self, step: SectionStep) -> bool {
        step != SectionStep::Face || !self.waiting_for_a_face
    }

    /// A step clicked in the panel becomes current, on its first field.
    pub(crate) fn activate(&mut self, step: SectionStep) {
        self.field = match step {
            SectionStep::Face => SectionField::Face,
            SectionStep::Distance => SectionField::Distance,
            SectionStep::Tilts => SectionField::TiltAboutU,
            SectionStep::Sides => SectionField::Sides,
        };
    }

    /// A field reached in the panel becomes the one `Entrée` stands on.
    pub(crate) fn reach(&mut self, field: SectionField) {
        self.field = field;
    }

    pub(crate) fn field(&self) -> SectionField {
        self.field
    }

    /// `Entrée`: the field below becomes current, and on the last step the
    /// section is kept, as « Enregistrer la coupe » keeps it. True when it
    /// was.
    pub(crate) fn enter(&mut self, document: &mut PartDocument) -> bool {
        match self.field.below() {
            Some(below) => {
                self.field = below;
                false
            }
            None => {
                self.keep_in(document);
                true
            }
        }
    }

    /// What the message pill says while the section waits on its current
    /// step.
    pub(crate) fn said(&self, lang: &Catalogue) -> String {
        let step = self.field.step();
        let asked = match step {
            SectionStep::Face => Asked::Face,
            SectionStep::Distance => Asked::Distance,
            SectionStep::Tilts => Asked::Tilts,
            SectionStep::Sides => Asked::Sides,
        };
        said::step_asks(lang, step as usize + 1, asked)
    }
}
