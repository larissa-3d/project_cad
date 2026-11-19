//! What a drawing given dimensions and rules must keep once the solver has
//! settled it: every promise it accepted, the green of every piece made
//! entirely constrained, and the rules of its areas.
//!
//! Like the rules of the areas, these read plain data and never a `Sketch`,
//! so that each can be caught out by a drawing broken on purpose.

use glam::DVec2;

use super::checking::keeps_every_rule;
use super::rules::{Area, Flaw, Place, lowest, reach};

/// How far a settled drawing may stand from what it promised, against how far
/// it reaches: the tenth of a micron on a metre that the sketch takes as one at
/// the use, a hundred times what its solver settles to.
const HELD: f64 = 1e-7;

/// What a dimension or a rule laid on a drawing promised, and what the
/// drawing does with it once settled.
#[derive(Clone, Debug, PartialEq)]
pub enum Promise {
    /// An accepted dimension laid near `near`: the value it was given, and
    /// what the settled drawing measures there.
    Measured {
        near: DVec2,
        value: f64,
        measured: f64,
    },
    /// An accepted angle laid near `near`: the degrees it was given, what the
    /// settled drawing measures there, and the arm it is read on.
    Turned {
        near: DVec2,
        degrees: f64,
        measured: f64,
        arm: f64,
    },
    /// An accepted rule laid near `near`, and how far the settled drawing
    /// stands from it: nought when it holds.
    Ruled { near: DVec2, off_by: f64 },
    /// A dimension or a rule the sketch refused, and where the drawing's
    /// points stood before it was laid and after.
    Refused {
        before: Vec<DVec2>,
        after: Vec<DVec2>,
    },
}

/// A piece made entirely constrained, and what the sketch says of it.
#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    /// Where each of its points stands, and whether the sketch says it is
    /// settled (`Sketch::settled_points`).
    pub points: Vec<(DVec2, bool)>,
    /// For each of the dimensions it was given, taken away alone, whether
    /// every one of its points is still said settled.
    pub green_without: Vec<bool>,
}

/// Every accepted dimension measures its value, every accepted rule holds,
/// and every refused one leaves the drawing as it was. An angle may stray by
/// what moves the end of its arm as far as a length may stray.
pub fn tells_the_truth(promises: &[Promise]) -> Result<(), Flaw> {
    for promise in promises {
        match promise {
            Promise::Measured {
                near,
                value,
                measured,
            } => {
                let allowed = HELD * reach(&[*near]).max(value.abs());
                if (measured - value).abs() > allowed {
                    return Err(Flaw::Untrue {
                        near: *near,
                        value: *value,
                        measured: *measured,
                    });
                }
            }
            Promise::Turned {
                near,
                degrees,
                measured,
                arm,
            } => {
                let allowed = (HELD * reach(&[*near]) / arm.max(f64::MIN_POSITIVE)).to_degrees();
                if (measured - degrees).abs() > allowed {
                    return Err(Flaw::Untrue {
                        near: *near,
                        value: *degrees,
                        measured: *measured,
                    });
                }
            }
            Promise::Ruled { near, off_by } => {
                if off_by.abs() > HELD * reach(&[*near]) {
                    return Err(Flaw::Unkept {
                        near: *near,
                        off_by: *off_by,
                    });
                }
            }
            Promise::Refused { before, after } => {
                if before.len() != after.len() {
                    return Err(Flaw::Recounted {
                        before: before.len(),
                        after: after.len(),
                    });
                }
                let allowed = HELD * reach(before).max(reach(after));
                if let Some((from, to)) = before
                    .iter()
                    .zip(after)
                    .find(|(from, to)| from.distance(**to) > allowed)
                {
                    return Err(Flaw::Budged {
                        from: *from,
                        to: *to,
                    });
                }
            }
        }
    }
    Ok(())
}

/// Every piece made entirely constrained is said to be, point by point, and
/// the whole drawing too when `whole` says what `is_fully_constrained` gave
/// for a drawing whose every piece was made so; none is still said to be with
/// one of its dimensions taken away.
pub fn green_is_fixed(pieces: &[Piece], whole: Option<bool>) -> Result<(), Flaw> {
    for piece in pieces {
        if let Some((at, _)) = piece.points.iter().find(|(_, settled)| !settled) {
            return Err(Flaw::NotGreen { at: Some(*at) });
        }
        if let Some(without) = piece.green_without.iter().position(|green| *green) {
            let places: Vec<DVec2> = piece.points.iter().map(|(at, _)| *at).collect();
            return Err(Flaw::StillGreen {
                near: lowest(&places),
                without,
            });
        }
    }
    match whole {
        Some(false) => Err(Flaw::NotGreen { at: None }),
        _ => Ok(()),
    }
}

/// The rules every area keeps, read on a drawing the solver has moved: what
/// breaks one is named as broken once moved.
pub fn areas_hold(areas: &[Area], places: &[Place], closed: Option<usize>) -> Result<(), Flaw> {
    keeps_every_rule(areas, places, closed).map_err(|flaw| Flaw::Moved(Box::new(flaw)))
}
