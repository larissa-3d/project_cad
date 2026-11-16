//! Two bosses stand on one block, as high as each other, their tops in one
//! plane and apart. A chamfer or a fillet on a top edge of one of them is laid
//! on that edge as on a boss standing alone, and the other boss keeps every
//! face and every edge it had, whether the bosses stand back from the block's
//! sides or flush with them. Two tops that touch or overlap are one face, a
//! different body, and out of this issue's reach.
//!
//! Closes #924.
//! - A chamfer and a fillet on a top edge of one of two bosses whose tops share
//!   a plane are built, the other boss untouched, the body closed and sound,
//!   the volume the closed form —
//!   `a_chamfer_on_a_top_edge_of_one_of_two_bosses_level_with_each_other_is_cut_leaving_the_other_untouched`,
//!   `a_fillet_on_a_top_edge_of_one_of_two_bosses_level_with_each_other_is_laid_leaving_the_other_untouched`,
//!   each on the first boss's top front edge with the second boss's front in
//!   line with it and set back by 1, the issue's table; on its top back edge;
//!   on the second boss's top front edge, the first boss the one left
//!   untouched; on the first boss's top edge facing the second; on two blocks
//!   apart in line with nothing under them; on a first boss whose end wall
//!   leans out in plan; with the bosses flush with the block's sides, whose
//!   walls the join merges with the block's into one face: both fronts flush
//!   with the block's front, the first's flush and the second's set back by
//!   1, and both backs flush with the block's back; on a first boss shaped as
//!   an L, whose top runs on past the edge's end behind the second boss; and
//!   on the two halves of a block's top either side of a slot across it,
//!   their front one face; and, the bosses only 2 apart, the issue's two rows
//!   and the fronts flush, on the part drawn along X and turned in plan by 30
//!   and 45 degrees. Where the first boss's end wall leans out over the
//!   second, the second is never touched: the step is declined `Unsupported`,
//!   or laid with the second boss as it was —
//!   `a_chamfer_and_a_fillet_on_a_boss_whose_end_wall_leans_out_past_a_second_boss_never_touch_the_second`.
//!   Named in one step, the two bosses' top front edges are each laid as its
//!   own step would lay it, in either order —
//!   `a_chamfer_and_a_fillet_on_the_top_front_edges_of_two_level_bosses_in_one_step_lay_each_as_it_alone_would`.
//!   At the part's level, the issue's own probe, a boss put in before the
//!   chamfered one as high as it —
//!   `a_step_put_in_before_a_chamfer_while_the_part_is_read_earlier_leaves_it_on_its_edge`
//!   in `crates/part/tests/a_chamfer_moves_nothing_raised_or_laid_after_it.rs`
//! - Existing chamfer and fillet tests still pass — no test: their own suites,
//!   `a_chamfer_cuts_one_straight_edge.rs`, `a_fillet_rounds_one_straight_edge.rs`,
//!   the other `a_chamfer_and_a_fillet_` files and `src/body/tests/`, are the
//!   answer, all of them passing, and changed in the chamfer's suite by three
//!   tests. One went:
//!   `a_chamfer_whose_cut_carried_along_its_edge_would_reach_a_second_block_in_line_is_declined_as_unsupported`
//!   declined two blocks in line, the very limit this issue lifts, and its two
//!   blocks are a case of the two tests above, built. One was renamed: the
//!   tab beside the edge's end, which the cut no longer takes away whole, now
//!   `a_chamfer_whose_cut_would_end_inside_a_tab_beside_its_edge_s_end_is_declined_as_unsupported`,
//!   the cut's end left a face of its own inside the tab. One was added, for
//!   the guard that declines a cut taking a face away whole, which the tab no
//!   longer reaches:
//!   `a_chamfer_whose_cut_would_take_a_blind_hole_in_the_wall_at_its_end_away_whole_is_declined_as_unsupported`

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Chamfered, Declined, EdgeName, Filleted, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it, on a plane
/// or on the round: each is placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// The block both bosses stand on: 30 along X, 10 along Y, 20 high.
const BLOCK: [f64; 3] = [30.0, 10.0, 20.0];

/// How high both bosses stand, raised from the ground through the block.
const HIGH: f64 = 30.0;

/// Where each boss stands along X, and where the second stands when only 2
/// lie between them.
const FIRST: [f64; 2] = [5.0, 10.0];
const SECOND: [f64; 2] = [20.0, 25.0];
const CLOSE: [f64; 2] = [12.0, 17.0];

/// The value every chamfer and fillet here is asked: a distance or a radius
/// both of an edge's faces give whole.
const VALUE: f64 = 2.0;

/// The numbers each boss's faces answer to, joined after the block's six:
/// the first's floor, which the join takes in, six, then its top, and its
/// walls from the front round to the left; the second's six on from there.
/// A wall flush with a side of the block answers to the block's number too.
const FIRST_FACES: [u32; 5] = [7, 8, 9, 10, 11];
const SECOND_FACES: [u32; 5] = [13, 14, 15, 16, 17];
const FIRST_TOP: u32 = 7;
const FIRST_FRONT: u32 = 8;
const FIRST_RIGHT: u32 = 9;
const FIRST_BACK: u32 = 10;
const SECOND_TOP: u32 = 13;
const SECOND_FRONT: u32 = 14;

/// The box round a boss standing `across` along X, a unit wider on either
/// side, and holding the block's depth and the bosses' height.
fn round_the(across: [f64; 2]) -> [DVec3; 2] {
    [
        DVec3::new(across[0] - 1.0, -1.0, -1.0),
        DVec3::new(across[1] + 1.0, BLOCK[1] + 1.0, HIGH + 1.0),
    ]
}

/// An outline with its runs, as the drawing hands it to the exact kernel.
fn profile(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

/// The part with an outline drawn on the ground raised up by `height`,
/// joined to it as `cao_part` joins a step: the tool's faces numbered past
/// the part's.
fn joined_to(part: &Body, outline: Contour, height: f64) -> Body {
    let ground = Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    };
    let tool = part
        .tool_raised(&profile(outline), ground, DVec3::Z * height)
        .expect("the tool raises");
    part.union(&tool).expect("the join is built")
}

/// The corners of a rectangle across `x` and `y`, from its front left round
/// to its back left: its walls numbered from the front round to the left.
fn rectangle(x: [f64; 2], y: [f64; 2]) -> Vec<DVec2> {
    vec![
        DVec2::new(x[0], y[0]),
        DVec2::new(x[1], y[0]),
        DVec2::new(x[1], y[1]),
        DVec2::new(x[0], y[1]),
    ]
}

/// The area an outline encloses, its corners counter-clockwise.
fn area(corners: &[DVec2]) -> f64 {
    let next = corners.iter().cycle().skip(1);
    corners
        .iter()
        .zip(next)
        .map(|(here, there)| here.perp_dot(*there))
        .sum::<f64>()
        / 2.0
}

/// The [`BLOCK`], and on it two bosses [`HIGH`] raised from the ground as a
/// sketch on the ground raises them, of the outlines `first` and `second`;
/// and the matter the part holds. Their tops lie in one plane, apart.
fn two_bosses(first: &[DVec2], second: &[DVec2]) -> (Body, f64) {
    two_bosses_turned(first, second, DVec2::X)
}

/// The [`two_bosses`] of the outlines `first` and `second`, the whole part
/// drawn turned about the vertical through the origin by the turn `turn`
/// holds, its cosine and its sine.
fn two_bosses_turned(first: &[DVec2], second: &[DVec2], turn: DVec2) -> (Body, f64) {
    let [length, breadth, height] = BLOCK;
    let drawn = |outline: &[DVec2]| {
        Contour::straight(outline.iter().map(|corner| turn.rotate(*corner)).collect())
    };
    let block = joined_to(
        &Body::default(),
        drawn(&rectangle([0.0, length], [0.0, breadth])),
        height,
    );
    let with_the_first = joined_to(&block, drawn(first), HIGH);
    let part = joined_to(&with_the_first, drawn(second), HIGH);
    let volume = length * breadth * height + (area(first) + area(second)) * (HIGH - height);
    (part, volume)
}

/// A point or a way turned about the vertical by the turn `turn` holds.
fn spun(turn: DVec2, point: DVec3) -> DVec3 {
    turn.rotate(point.truncate()).extend(point.z)
}

/// Two blocks 15 by 10 by 10 on the ground with nothing under them, the
/// second from x = 20, its front and its top in line with the first's: two
/// bosses as high as each other with no block to stand on. The first's faces
/// are numbered nought to five, its top one and its front two, the second's
/// six to eleven.
fn two_blocks_in_line() -> Body {
    let first = joined_to(
        &Body::default(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(15.0, 10.0)),
        10.0,
    );
    joined_to(
        &first,
        Contour::rectangle(DVec2::new(20.0, 0.0), DVec2::new(35.0, 10.0)),
        10.0,
    )
}

/// A block 30 by 10 by [`HIGH`] on the ground with a slot 4 wide across its
/// top from x = 13 to 17, 10 deep: two bosses as high as each other on the
/// block under the slot, their fronts and their backs one face with the
/// block's. The block's faces are numbered nought to five, its top one,
/// both pieces of it, and its front two; the slot's bottom six, its right
/// wall nine and its left eleven.
fn slotted() -> Body {
    let block = joined_to(
        &Body::default(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 10.0)),
        HIGH,
    );
    let tool = block
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::new(13.0, -1.0),
                DVec2::new(17.0, 11.0),
            )),
            Frame {
                origin: DVec3::Z * 20.0,
                u: DVec3::X,
                v: DVec3::Y,
            },
            DVec3::Z * 11.0,
        )
        .expect("the tool raises");
    block.difference(&tool).expect("the slot is cut")
}

/// A top edge of one boss: the part it stands on and the matter that part
/// holds; a number the boss's top answers to and one the wall the edge lies
/// between answers to, and, where two edges lie between those, one a face
/// at its end answers to; the edge's two ends, the way the wall faces, and
/// the way the face at each end faces, out of the boss; and the box round
/// the other boss, drawn as the part was before it was turned by `turn`,
/// with the numbers its faces answer to, where nothing may change.
struct TopEdge {
    what: String,
    part: Body,
    volume: f64,
    sides: [u32; 2],
    ending_on: Option<u32>,
    ends: [DVec3; 2],
    outward: DVec3,
    end_faces: [DVec3; 2],
    other: [DVec3; 2],
    turn: DVec2,
    other_faces: Vec<u32>,
}

impl TopEdge {
    fn length(&self) -> f64 {
        self.ends[0].distance(self.ends[1])
    }

    fn along(&self) -> DVec3 {
        (self.ends[1] - self.ends[0]).normalize()
    }

    /// Where the line through the edge's end `end` moved by `off` crosses the
    /// plane of the face at that end.
    fn crossing(&self, end: usize, off: DVec3) -> DVec3 {
        let facing = self.end_faces[end];
        let from = self.ends[end] + off;
        from - self.along() * (facing.dot(off) / facing.dot(self.along()))
    }

    /// How much longer the line through the edge moved by `off` runs between
    /// the planes of its two end faces than the edge does.
    fn longer_by(&self, off: DVec3) -> f64 {
        (self.crossing(1, off) - self.crossing(0, off)).dot(self.along()) - self.length()
    }

    /// The matter a cut takes off along the edge, when its section, in the
    /// edge's corner, holds `section` and has its centre `centre` from both
    /// faces: the section times its length between the two end faces, which
    /// grows by how much it grows along the top and down the wall, at the
    /// centre.
    fn taken(&self, section: f64, centre: f64) -> f64 {
        let longer = self.longer_by(-self.outward * centre) + self.longer_by(-DVec3::Z * centre);
        section * (self.length() + longer)
    }

    /// The matter a chamfer of [`VALUE`] takes off along the edge.
    fn chamfered_off(&self) -> f64 {
        self.taken(VALUE * VALUE / 2.0, VALUE / 3.0)
    }

    /// The matter a fillet of [`VALUE`] takes off along the edge: the corner
    /// of the square on its radius a quarter disc leaves, whose centre stands
    /// `r (5/6 − π/4) / (1 − π/4)` from both faces.
    fn rounded_off(&self) -> f64 {
        let section = (1.0 - PI / 4.0) * VALUE * VALUE;
        self.taken(section, VALUE * (5.0 / 6.0 - PI / 4.0) / (1.0 - PI / 4.0))
    }

    /// Whether a point stands between the planes of the edge's two end faces.
    fn between_its_ends(&self, point: DVec3) -> bool {
        (0..2).all(|end| self.end_faces[end].dot(point - self.ends[end]) <= PLACE)
    }

    /// The same edge on its part drawn turned by `turn`, the part given
    /// turned already: its ends and the ways its faces face turned with it.
    fn turned(self, turn: DVec2) -> TopEdge {
        TopEdge {
            ends: self.ends.map(|end| spun(turn, end)),
            outward: spun(turn, self.outward),
            end_faces: self.end_faces.map(|facing| spun(turn, facing)),
            turn,
            ..self
        }
    }
}

/// The first boss's top front edge, its front at `front`, on the part and
/// the matter it holds given, the second boss standing [`SECOND`] along X.
fn first_s_front(what: &str, (part, volume): (Body, f64), front: f64) -> TopEdge {
    let at = |x: f64| DVec3::new(x, front, HIGH);
    TopEdge {
        what: what.to_string(),
        part,
        volume,
        sides: [FIRST_TOP, FIRST_FRONT],
        ending_on: None,
        ends: [at(FIRST[0]), at(FIRST[1])],
        outward: -DVec3::Y,
        end_faces: [-DVec3::X, DVec3::X],
        other: round_the(SECOND),
        turn: DVec2::X,
        other_faces: SECOND_FACES.to_vec(),
    }
}

/// The second boss's top front edge, its front at `front`, on the part and
/// the matter it holds given, the first boss standing [`FIRST`] along X.
fn second_s_front(what: &str, (part, volume): (Body, f64), front: f64) -> TopEdge {
    TopEdge {
        sides: [SECOND_TOP, SECOND_FRONT],
        ends: [SECOND[0], SECOND[1]].map(|x| DVec3::new(x, front, HIGH)),
        other: round_the(FIRST),
        other_faces: FIRST_FACES.to_vec(),
        ..first_s_front(what, (part, volume), front)
    }
}

/// The edges each test takes off: the issue's two cases first, the first
/// boss's top front edge with the second boss's front in line with it and
/// set back by 1; then its top back edge, the backs in line; the second
/// boss's top front edge, the first boss the one beside it; the first boss's
/// top edge facing the second, which runs across the other's line rather
/// than along it; the first of [`two_blocks_in_line`]'s top front edge, two
/// bosses with no block under them; the first boss's top front edge where
/// its right wall leans out in plan, a third along X for every unit back;
/// the bosses flush with a side of the block, whose wall the join merges
/// with the block's; the first boss an L whose arm runs on behind the
/// second, its top reaching past the edge's end, the second's faces
/// numbered two further on; and the top front edge left of a [`slotted`]
/// block's slot, told from the one right of it by the left side it ends on.
fn top_edges() -> Vec<TopEdge> {
    let at = |x: f64, y: f64| DVec3::new(x, y, HIGH);
    let bosses = |first: [f64; 2], second: [f64; 2]| {
        two_bosses(&rectangle(FIRST, first), &rectangle(SECOND, second))
    };
    let first_s_back = |what: &str, (part, volume): (Body, f64), back: f64| TopEdge {
        sides: [FIRST_TOP, FIRST_BACK],
        ends: [at(FIRST[0], back), at(FIRST[1], back)],
        outward: DVec3::Y,
        ..first_s_front(what, (part, volume), back)
    };
    let leaning = vec![
        DVec2::new(FIRST[0], 2.0),
        DVec2::new(FIRST[1], 2.0),
        DVec2::new(FIRST[1] + 2.0, 8.0),
        DVec2::new(FIRST[0], 8.0),
    ];
    let ell = vec![
        DVec2::new(FIRST[0], 2.0),
        DVec2::new(FIRST[1], 2.0),
        DVec2::new(FIRST[1], 6.0),
        DVec2::new(28.0, 6.0),
        DVec2::new(28.0, 8.0),
        DVec2::new(FIRST[0], 8.0),
    ];
    vec![
        first_s_front(
            "the first boss's top front edge, the second's front in line with it",
            bosses([2.0, 8.0], [2.0, 8.0]),
            2.0,
        ),
        first_s_front(
            "the first boss's top front edge, the second's front set back by 1",
            bosses([2.0, 8.0], [3.0, 8.0]),
            2.0,
        ),
        first_s_back(
            "the first boss's top back edge, the backs in line",
            bosses([2.0, 8.0], [2.0, 8.0]),
            8.0,
        ),
        second_s_front(
            "the second boss's top front edge, the first's front in line with it",
            bosses([2.0, 8.0], [2.0, 8.0]),
            2.0,
        ),
        TopEdge {
            sides: [FIRST_TOP, FIRST_RIGHT],
            ends: [at(FIRST[1], 2.0), at(FIRST[1], 8.0)],
            outward: DVec3::X,
            end_faces: [-DVec3::Y, DVec3::Y],
            ..first_s_front(
                "the first boss's top edge facing the second",
                bosses([2.0, 8.0], [2.0, 8.0]),
                2.0,
            )
        },
        TopEdge {
            what: "the top front edge of a block standing apart from a second in line with it, \
                   nothing under either"
                .to_string(),
            part: two_blocks_in_line(),
            volume: 2.0 * 15.0 * 10.0 * 10.0,
            sides: [1, 2],
            ending_on: None,
            ends: [DVec3::new(0.0, 0.0, 10.0), DVec3::new(15.0, 0.0, 10.0)],
            outward: -DVec3::Y,
            end_faces: [-DVec3::X, DVec3::X],
            other: [DVec3::new(19.0, -1.0, -1.0), DVec3::new(36.0, 11.0, 11.0)],
            turn: DVec2::X,
            other_faces: (6..12).collect(),
        },
        TopEdge {
            end_faces: [-DVec3::X, DVec3::new(6.0, -2.0, 0.0).normalize()],
            ..first_s_front(
                "the first boss's top front edge, its right wall leaning out in plan",
                two_bosses(&leaning, &rectangle(SECOND, [2.0, 8.0])),
                2.0,
            )
        },
        first_s_front(
            "the first boss's top front edge, both fronts flush with the block's",
            bosses([0.0, 8.0], [0.0, 8.0]),
            0.0,
        ),
        first_s_front(
            "the first boss's top front edge, its front flush with the block's, the second's \
             set back by 1",
            bosses([0.0, 8.0], [1.0, 8.0]),
            0.0,
        ),
        first_s_back(
            "the first boss's top back edge, both backs flush with the block's",
            bosses([2.0, 10.0], [2.0, 10.0]),
            10.0,
        ),
        TopEdge {
            other: [
                DVec3::new(SECOND[0] - 1.0, -1.0, -1.0),
                DVec3::new(SECOND[1] + 1.0, 5.5, HIGH + 1.0),
            ],
            other_faces: (15..20).collect(),
            ..first_s_front(
                "the first boss's top front edge, its top running on past the edge's end in an \
                 L behind the second",
                two_bosses(&ell, &rectangle(SECOND, [2.0, 5.0])),
                2.0,
            )
        },
        TopEdge {
            what: "the top front edge left of a slot across a block's top".to_string(),
            part: slotted(),
            volume: 30.0 * 10.0 * HIGH - 4.0 * 10.0 * 10.0,
            sides: [1, 2],
            ending_on: Some(5),
            ends: [at(0.0, 0.0), at(13.0, 0.0)],
            outward: -DVec3::Y,
            end_faces: [-DVec3::X, DVec3::X],
            other: [
                DVec3::new(16.0, -1.0, -1.0),
                DVec3::new(31.0, 11.0, HIGH + 1.0),
            ],
            turn: DVec2::X,
            other_faces: vec![1, 2, 3, 4, 9],
        },
    ]
}

/// The issue's two rows and the fronts flush with the block's, the bosses
/// only 2 apart, the second across [`CLOSE`], on the part drawn along X, and
/// turned in plan by 30 and 45 degrees: there the box round an end wall
/// square to the edge reaches past the wall along it, by half the wall's
/// depth at 45 degrees, further than the gap.
fn turned_edges() -> Vec<TopEdge> {
    let rows = [
        (
            "the first boss's top front edge, the second's front in line with it",
            [2.0, 8.0],
            [2.0, 8.0],
            2.0,
        ),
        (
            "the first boss's top front edge, the second's front set back by 1",
            [2.0, 8.0],
            [3.0, 8.0],
            2.0,
        ),
        (
            "the first boss's top front edge, both fronts flush with the block's",
            [0.0, 8.0],
            [0.0, 8.0],
            0.0,
        ),
    ];
    [0.0_f64, 30.0, 45.0]
        .into_iter()
        .flat_map(|degrees| {
            let turn = DVec2::from_angle(degrees.to_radians());
            rows.map(|(what, first, second, front)| {
                let part =
                    two_bosses_turned(&rectangle(FIRST, first), &rectangle(CLOSE, second), turn);
                let what = format!("{what}, 2 apart, the part turned by {degrees} degrees");
                TopEdge {
                    other: round_the(CLOSE),
                    ..first_s_front(&what, part, front)
                }
                .turned(turn)
            })
        })
        .collect()
}

/// The first boss drawn through (5, 2), (10, 2), (22, 8) and (5, 8), its
/// right wall leaning out in plan by 2 along X for every unit back, and the
/// second from x = 16.5 to 18.5 and y = 2 to 5, in front of that wall and
/// half a unit clear of it at y = 5: the chamfer's set-back lines meet the
/// wall at x = 10 and 14, short of the second boss, but the wall's own box
/// reaches x = 22, past it.
fn leaning_over_the_second() -> TopEdge {
    let first = [(5.0, 2.0), (10.0, 2.0), (22.0, 8.0), (5.0, 8.0)].map(DVec2::from);
    let second = rectangle([16.5, 18.5], [2.0, 5.0]);
    TopEdge {
        end_faces: [-DVec3::X, DVec3::new(6.0, -12.0, 0.0).normalize()],
        other: [
            DVec3::new(16.0, 1.5, -1.0),
            DVec3::new(19.0, 5.25, HIGH + 1.0),
        ],
        ..first_s_front(
            "the first boss's top front edge, its right wall leaning out over the second",
            two_bosses(&first, &second),
            2.0,
        )
    }
}

/// The name of the one edge between a face answering to `first` and one
/// answering to `second`, and ending on one answering to `ending_on` where
/// it is given, as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, [first, second]: [u32; 2], ending_on: Option<u32>) -> EdgeName {
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| {
            let [one, other] = &name.between;
            (one.contains(&first) && other.contains(&second))
                || (one.contains(&second) && other.contains(&first))
        })
        .filter(|name| {
            ending_on.is_none_or(|end| name.ends.iter().flatten().any(|side| side.contains(&end)))
        })
        .collect();
    assert_eq!(
        named.len(),
        1,
        "one edge between {first} and {second}: {named:?}"
    );
    named[0].clone()
}

/// Every number some face of the body answers to: a step's, below where the
/// steps' numbers end, and a piece's, past them.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&number| body.has_face(number))
        .collect()
}

/// Holds that a step left every face of the part it was given and made one
/// face more, and gives back the number that face answers to.
fn one_face_more(part: &Body, body: &Body, what: &str) -> usize {
    let (before, after) = (numbers(part), numbers(body));
    assert!(
        before.is_subset(&after),
        "{what}: a face went: {before:?} became {after:?}"
    );
    let made: Vec<usize> = after.difference(&before).copied().collect();
    assert_eq!(
        made.len(),
        1,
        "{what}: one face more: {before:?} became {after:?}"
    );
    made[0]
}

/// Holds that each point of `found` is one of `expected`, and each of
/// `expected` one of `found`.
fn same_points(found: &[DVec3], expected: &[DVec3], what: &str) {
    for point in found {
        assert!(
            expected.iter().any(|other| other.distance(*point) < PLACE),
            "{what}: a corner at {point}, where the corners are {expected:?}"
        );
    }
    for point in expected {
        assert!(
            found.iter().any(|other| other.distance(*point) < PLACE),
            "{what}: no corner at {point}, among {found:?}"
        );
    }
}

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    same_points(&plane.corners, expected, what);
}

/// Holds that a body is exact and keeps the rules every solid must: its
/// surface closed, no face through another, the matter it holds what
/// arithmetic promised, and the matter its triangles enclose within `drawn`
/// of it.
fn sound_drawn_within(body: &Body, promised: f64, drawn: f64, what: &str) {
    assert!(
        body.is_exact(),
        "{what}: the exact kernel computed the body"
    );
    let triangles = body.triangles();
    assert_eq!(closed(&triangles), Ok(()), "{what}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
    assert!(
        (body.volume() - promised).abs() <= EXACT * promised,
        "{what}: the body holds {}, where arithmetic promised {promised}",
        body.volume()
    );
    let enclosed = enclosed(&triangles);
    assert!(
        (enclosed - promised).abs() <= drawn,
        "{what}: its triangles enclose {enclosed}, where arithmetic promised {promised}"
    );
}

/// Holds that nothing changed round the other boss: every face of the part
/// has, within the box round it, the very corners it had, a face the step
/// made is drawn nowhere in that box, and every edge between two faces
/// answering to the other boss's numbers, but the one `taken` off, is found
/// by its name after.
fn untouched(part: &Body, body: &Body, edge: &TopEdge, taken: &EdgeName, what: &str) {
    let [low, high] = edge.other;
    let back = DVec2::new(edge.turn.x, -edge.turn.y);
    let inside = |point: &DVec3| {
        let drawn = spun(back, *point);
        drawn.cmpge(low - PLACE).all() && drawn.cmple(high + PLACE).all()
    };
    let before = numbers(part);
    for &face in &before {
        let corners_inside = |body: &Body| -> Vec<DVec3> {
            body.plane_of(face)
                .map(|plane| plane.corners.into_iter().filter(inside).collect())
                .unwrap_or_default()
        };
        same_points(
            &corners_inside(body),
            &corners_inside(part),
            &format!("{what}: face {face}, round the other boss"),
        );
    }
    for made in numbers(body).difference(&before) {
        assert!(
            !body
                .triangles_of(*made)
                .flatten()
                .any(|corner| inside(&corner)),
            "{what}: the face {made} the step made reaches the other boss"
        );
    }
    let its_own = |side: &Vec<u32>| side.iter().any(|number| edge.other_faces.contains(number));
    let edges: Vec<EdgeName> = (part.edge_names().into_iter())
        .filter(|name| name != taken && name.between.iter().all(its_own))
        .collect();
    assert!(!edges.is_empty(), "{what}: the other boss has edges");
    for name in edges {
        assert_eq!(
            body.lost(&name),
            None,
            "{what}: the other boss's edge between {:?} is no longer found",
            name.between
        );
    }
}

/// Holds that a chamfer was given the distance it was asked.
fn chamfer_taken_whole(chamfered: &Chamfered, what: &str) {
    assert!(
        (chamfered.asked - VALUE).abs() < PLACE && (chamfered.taken - VALUE).abs() < PLACE,
        "{what}: asked {} and took {}, where {VALUE} fits",
        chamfered.asked,
        chamfered.taken
    );
}

/// Holds that a fillet was given the radius it was asked.
fn fillet_taken_whole(filleted: &Filleted, what: &str) {
    assert!(
        (filleted.asked - VALUE).abs() < PLACE && (filleted.taken - VALUE).abs() < PLACE,
        "{what}: asked {} and took {}, where {VALUE} fits",
        filleted.asked,
        filleted.taken
    );
}

/// Holds that the face numbered `face` lies on the quarter of a cylinder of
/// [`VALUE`] tangent to the edge's top and wall: every corner of its
/// triangles at that radius from the line set back by it under the top and
/// behind the wall, between the planes of the edge's two end faces, and on
/// the side of the line facing out of both.
fn on_the_round(body: &Body, face: usize, edge: &TopEdge, what: &str) {
    let axis = edge.ends[0] - edge.outward * VALUE - DVec3::Z * VALUE;
    let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the round is drawn");
    for corner in corners {
        let off = corner - axis;
        let across = off - edge.along() * off.dot(edge.along());
        assert!(
            (across.length() - VALUE).abs() < PLACE
                && edge.between_its_ends(corner)
                && across.dot(edge.outward) >= -PLACE
                && across.dot(DVec3::Z) >= -PLACE,
            "{what}: the round is drawn through {corner}, off the quarter cylinder of {VALUE} \
             about the line from {axis} along {}, between the edge's end faces",
            edge.along()
        );
    }
}

/// Holds that the face numbered `face` is the flat a chamfer of [`VALUE`]
/// lays on `edge`: set back by it along the top and down the wall, from one
/// end face to the other.
fn flat_laid_on(body: &Body, face: usize, edge: &TopEdge, what: &str) {
    assert!(
        body.is_flat(face),
        "{what}: the face the chamfer lays is flat"
    );
    let (on_top, on_wall) = (-edge.outward * VALUE, -DVec3::Z * VALUE);
    corners_are(
        body,
        face,
        &[
            edge.crossing(0, on_top),
            edge.crossing(1, on_top),
            edge.crossing(1, on_wall),
            edge.crossing(0, on_wall),
        ],
        &format!("{what}: the chamfer, set back by {VALUE} along the top and the wall"),
    );
}

/// Holds that the face numbered `face` is the round a fillet of [`VALUE`]
/// lays on `edge`.
fn round_laid_on(body: &Body, face: usize, edge: &TopEdge, what: &str) {
    assert!(
        !body.is_flat(face) && body.plane_of(face).is_none(),
        "{what}: the face the fillet lays is curved, and offers no plane to draw on"
    );
    on_the_round(body, face, edge, what);
}

/// What the triangles of a body may enclose apart from its matter, where a
/// step on `part` laid rounds of [`VALUE`] along edges `length` long in all:
/// the drawing's tolerance at the part's reach, over their area.
fn drawn_within(part: &Body, length: f64) -> f64 {
    let (low, high) = part.bounds().expect("a part has a box");
    let reach = low.abs().max(high.abs()).max_element();
    DRAWN * reach * PI / 2.0 * VALUE * length
}

/// Holds that `chamfered` is the chamfer of [`VALUE`] on `edge`, named
/// `name`, alone on its part: taken whole, the body sound and holding the
/// matter arithmetic leaves, the other boss untouched, and one flat more,
/// set back along the edge's two faces.
fn cut_as_it_should_be(edge: &TopEdge, name: &EdgeName, chamfered: &Chamfered) {
    let (what, part) = (&edge.what[..], &edge.part);
    chamfer_taken_whole(chamfered, what);
    let body = &chamfered.body;
    let promised = edge.volume - edge.chamfered_off();
    sound_drawn_within(body, promised, EXACT * promised, what);
    untouched(part, body, edge, name, what);
    let chamfer = one_face_more(part, body, what);
    flat_laid_on(body, chamfer, edge, what);
}

/// Holds that `filleted` is the fillet of [`VALUE`] on `edge`, named `name`,
/// alone on its part, as [`cut_as_it_should_be`] holds a chamfer: one round
/// more, tangent to the edge's two faces.
fn rounded_as_it_should_be(edge: &TopEdge, name: &EdgeName, filleted: &Filleted) {
    let (what, part) = (&edge.what[..], &edge.part);
    fillet_taken_whole(filleted, what);
    let body = &filleted.body;
    let promised = edge.volume - edge.rounded_off();
    sound_drawn_within(body, promised, drawn_within(part, edge.length()), what);
    untouched(part, body, edge, name, what);
    let fillet = one_face_more(part, body, what);
    round_laid_on(body, fillet, edge, what);
}

#[test]
fn a_chamfer_on_a_top_edge_of_one_of_two_bosses_level_with_each_other_is_cut_leaving_the_other_untouched()
 {
    for edge in top_edges().into_iter().chain(turned_edges()) {
        let name = edge_between(&edge.part, edge.sides, edge.ending_on);

        let chamfered = edge.part.chamfer(&name, VALUE).unwrap_or_else(|declined| {
            panic!("{}: the chamfer is declined, {declined:?}", edge.what)
        });

        cut_as_it_should_be(&edge, &name, &chamfered);
    }
}

#[test]
fn a_fillet_on_a_top_edge_of_one_of_two_bosses_level_with_each_other_is_laid_leaving_the_other_untouched()
 {
    for edge in top_edges().into_iter().chain(turned_edges()) {
        let name = edge_between(&edge.part, edge.sides, edge.ending_on);

        let filleted = edge.part.fillet(&name, VALUE).unwrap_or_else(|declined| {
            panic!("{}: the fillet is declined, {declined:?}", edge.what)
        });

        rounded_as_it_should_be(&edge, &name, &filleted);
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_a_boss_whose_end_wall_leans_out_past_a_second_boss_never_touch_the_second()
 {
    let edge = leaning_over_the_second();
    let name = edge_between(&edge.part, edge.sides, edge.ending_on);
    let what = &edge.what;

    match edge.part.chamfer(&name, VALUE) {
        Err(Declined::Unsupported) => {}
        Err(declined) => panic!("{what}: the chamfer is declined, {declined:?}"),
        Ok(chamfered) => cut_as_it_should_be(&edge, &name, &chamfered),
    }
    match edge.part.fillet(&name, VALUE) {
        Err(Declined::Unsupported) => {}
        Err(declined) => panic!("{what}: the fillet is declined, {declined:?}"),
        Ok(filleted) => rounded_as_it_should_be(&edge, &name, &filleted),
    }
}

/// Holds that a step on the edges `named`, `edges` in the order named, laid
/// each as its own step would: its body sound and holding the matter both
/// take off, every face of the part kept and one more for each edge,
/// numbered from where the steps' numbers ended in the order named and each
/// laid on its edge by `laid_on`, and every other edge of the part found by
/// its name after.
fn laid_each_as_alone(
    part: &Body,
    body: &Body,
    (named, edges): (&[EdgeName], &[&TopEdge]),
    (promised, drawn): (f64, f64),
    laid_on: fn(&Body, usize, &TopEdge, &str),
    what: &str,
) {
    sound_drawn_within(body, promised, drawn, what);
    let (before, after) = (numbers(part), numbers(body));
    let made: Vec<usize> = (part.faces_end()..part.faces_end() + edges.len()).collect();
    assert!(
        before.is_subset(&after) && after.difference(&before).eq(made.iter()),
        "{what}: every face kept and one more for each edge, from {}: {before:?} became \
         {after:?}",
        part.faces_end()
    );
    for (face, edge) in made.into_iter().zip(edges) {
        laid_on(body, face, edge, &format!("{what}, {}", edge.what));
    }
    for name in part.edge_names() {
        if !named.contains(&name) {
            assert_eq!(
                body.lost(&name),
                None,
                "{what}: the edge between {:?} is no longer found",
                name.between
            );
        }
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_the_top_front_edges_of_two_level_bosses_in_one_step_lay_each_as_it_alone_would()
 {
    let bosses = || {
        two_bosses(
            &rectangle(FIRST, [2.0, 8.0]),
            &rectangle(SECOND, [2.0, 8.0]),
        )
    };
    let first = first_s_front("the first boss's top front edge", bosses(), 2.0);
    let second = second_s_front("the second boss's top front edge", bosses(), 2.0);
    let part = &first.part;
    let both = |edges: [&TopEdge; 2]| edges.map(|edge| edge_between(part, edge.sides, None));

    for edges in [[&first, &second], [&second, &first]] {
        let named = both(edges);
        let what = format!("both top front edges, {} named first", edges[0].what);

        let chamfered = part
            .chamfer_edges(&named, VALUE)
            .unwrap_or_else(|declined| panic!("{what}: the chamfer is declined, {declined:?}"));
        let filleted = part
            .fillet_edges(&named, VALUE)
            .unwrap_or_else(|declined| panic!("{what}: the fillet is declined, {declined:?}"));

        chamfer_taken_whole(&chamfered, &what);
        let promised = first.volume - first.chamfered_off() - second.chamfered_off();
        laid_each_as_alone(
            part,
            &chamfered.body,
            (&named, &edges),
            (promised, EXACT * promised),
            flat_laid_on,
            &format!("{what}, chamfered"),
        );
        fillet_taken_whole(&filleted, &what);
        let promised = first.volume - first.rounded_off() - second.rounded_off();
        let drawn = drawn_within(part, first.length() + second.length());
        laid_each_as_alone(
            part,
            &filleted.body,
            (&named, &edges),
            (promised, drawn),
            round_laid_on,
            &format!("{what}, filleted"),
        );
    }
}
