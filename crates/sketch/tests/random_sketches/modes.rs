//! The four ways a campaign gives its drawings dimensions and rules, chosen
//! when it is launched.

use super::Gesture;
use super::asking::{Shares, asked};
use super::drawing::drawn;
use super::random::Random;

/// What each drawing of a campaign gets besides what it draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    /// No dimension and no rule: #500's campaign as it is.
    Free,
    /// Each drawing draws its own shares, from 0 to 100 %: of the dimensions
    /// it could take, of the rules it could take, and of its pieces made
    /// entirely constrained.
    Mix,
    /// The same three percentages for every drawing.
    Shares {
        dimensions: u8,
        rules: u8,
        constrained: u8,
    },
    /// Every piece made entirely constrained.
    AllConstrained,
}

impl Mode {
    /// The mode written at launch, `free`, `mix`, `shares` or `constrained`,
    /// and for `shares` its three percentages written `dimensions,rules,constrained`.
    /// Nothing written is the free mode.
    pub fn read(mode: Option<&str>, shares: Option<&str>) -> Result<Mode, String> {
        match mode.map(str::trim) {
            None | Some("free") => Ok(Mode::Free),
            Some("mix") => Ok(Mode::Mix),
            Some("constrained") => Ok(Mode::AllConstrained),
            Some("shares") => {
                let written = shares.ok_or("the shares mode takes its three percentages")?;
                let shares = written
                    .split(',')
                    .map(|share| match share.trim().parse::<u8>() {
                        Ok(share) if share <= 100 => Ok(share),
                        _ => Err(format!("{share:?} is no percentage from 0 to 100")),
                    })
                    .collect::<Result<Vec<u8>, String>>()?;
                match shares[..] {
                    [dimensions, rules, constrained] => Ok(Mode::Shares {
                        dimensions,
                        rules,
                        constrained,
                    }),
                    _ => Err(format!(
                        "{written:?} is not three percentages: dimensions,rules,constrained"
                    )),
                }
            }
            Some(other) => Err(format!(
                "{other:?} is no mode: free, mix, shares or constrained"
            )),
        }
    }

    /// What tells one mode's random numbers from another's, so that a seed
    /// stands for an unrelated case in each.
    fn salt(&self) -> u64 {
        match self {
            Mode::Free => 0,
            Mode::Mix => 0x6D69_7800,
            Mode::Shares {
                dimensions,
                rules,
                constrained,
            } => {
                0x7368_6172_0000_0000
                    ^ (u64::from(*dimensions) << 16)
                    ^ (u64::from(*rules) << 8)
                    ^ u64::from(*constrained)
            }
            Mode::AllConstrained => 0x636F_6E73,
        }
    }
}

/// How a mixed drawing draws each of its shares: nought and the whole as
/// often as anything between, so that some drawings take nothing and some
/// take everything.
const MIXED: [f64; 8] = [0.0, 0.0, 0.0, 0.1, 0.25, 0.5, 0.75, 1.0];

/// The case a seed stands for in a mode: the same seed in the same mode always
/// gives the same case, and in the free mode the drawing `drawn` gives.
///
/// What a mode adds is drawn from a stream of its own and laid after the
/// drawing, so that the drawing itself is the free mode's.
pub fn drawn_in(seed: u64, mode: &Mode) -> Vec<Gesture> {
    let mut gestures = drawn(seed);
    if *mode == Mode::Free {
        return gestures;
    }
    let mut random = Random::seeded(Random::seeded(seed).number() ^ mode.salt());
    let percent = |share: u8| f64::from(share) / 100.0;
    let shares = match mode {
        Mode::Free => return gestures,
        Mode::Mix => Shares {
            dimensions: *random.pick(&MIXED),
            rules: *random.pick(&MIXED),
            constrained: *random.pick(&MIXED),
            drags: random.below(3),
        },
        Mode::Shares {
            dimensions,
            rules,
            constrained,
        } => Shares {
            dimensions: percent(*dimensions),
            rules: percent(*rules),
            constrained: percent(*constrained),
            drags: random.below(3),
        },
        Mode::AllConstrained => Shares {
            dimensions: 0.0,
            rules: 0.0,
            constrained: 1.0,
            drags: 0,
        },
    };
    let added = asked(&mut random, &gestures, shares);
    gestures.extend(added);
    gestures
}
