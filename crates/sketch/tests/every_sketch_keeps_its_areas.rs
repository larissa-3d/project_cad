//! Drawings made at random with every tool of the sketch mode, held after
//! every gesture to the rules every area must keep.
//!
//! Closes #500.
//! - a generator draws, from a seed, with every gesture the issue lists, and
//!   the same seed gives the same drawing — `the_same_seed_draws_the_same_drawing`,
//!   `the_generator_draws_with_every_gesture`,
//!   `every_change_the_generator_draws_changes_a_drawing_now_and_then`
//! - tint is measure, caught out by areas broken on purpose —
//!   `a_tint_reaching_outside_its_outline_breaks_the_tint`,
//!   `a_tint_leaving_part_of_its_outline_bare_breaks_the_tint`
//! - nothing missing, caught out by areas broken on purpose —
//!   `a_closed_place_lying_in_no_area_is_missing`
//! - nothing extra, caught out by areas broken on purpose —
//!   `a_place_in_two_areas_is_extra`, `an_area_where_nothing_closes_is_extra`
//! - order does not matter, caught out by areas broken on purpose —
//!   `another_count_of_areas_laid_again_breaks_the_order`,
//!   `a_place_in_an_area_of_another_size_laid_again_breaks_the_order`
//! - answers, caught out by walks broken on purpose —
//!   `a_walk_that_panics_answers_with_the_panic`,
//!   `a_walk_that_never_comes_back_gives_no_answer`
//! - a broken case shrinks to the smallest drawing that still breaks the same
//!   rule — `a_failing_drawing_shrinks_to_the_gesture_that_fails`,
//!   `shrinking_any_drawn_drawing_comes_to_an_end`,
//!   `a_campaign_shrinks_what_it_finds_and_counts_the_rest`,
//!   `a_case_is_never_shrunk_into_breaking_another_rule`
//! - and is printed as Rust that pastes into a named test —
//!   `a_drawing_prints_as_the_rust_that_lays_it`,
//!   `the_printed_rust_lays_the_drawing_it_was_printed_from`
//! - the campaign is ignored by the gate and bounded by a deadline —
//!   `a_campaign_of_random_drawings_keeps_every_rule`
//! - the gate is no slower — no test: measured, the tests this adds to the
//!   gate take under a second of the test run; no campaign runs there
//! - `docs/sketch-soundness.md` says how to run the campaign and how to read a
//!   finding — no test: it is prose, held by `language.rs` and by nothing that
//!   asserts
//! - what the first campaigns find is written down — no test: the cases are
//!   in `what_random_sketches_found.rs`, each ignored under its issue, and on
//!   the issues themselves
//! - no file of #498's branch is touched, except one line in
//!   `docs/code-map.md` — no test: read off `git diff --stat origin/main`
//!   before the pull request was opened
//!
//! Closes #507.
//! - a drawing laid again mirrored gives the areas of its mirror image: a
//!   drawing holding an arc and half an ellipse keeps every rule, mirrored as
//!   well as drawn — `a_drawing_with_an_arc_and_half_an_ellipse_holds_mirrored`,
//!   `the_same_areas_mirrored_keep_the_order`
//! - a list of areas broken on purpose, mirrored wrong, is caught by
//!   `laid_alike` under `Relaying::Mirrored` —
//!   `an_area_of_another_size_mirrored_breaks_the_order`
//! - a campaign has run with it, and what it found is written down — no test:
//!   the findings live on the issues and in `what_random_sketches_found.rs`
//! - `docs/sketch-soundness.md` lists the mirrored laying, and drops it from
//!   *What the rules cannot see* — no test: it is prose
//!
//! Closes #508.
//! - the matter is the measure, caught out by matter broken on purpose: a hole
//!   left full, matter reaching outside its outline, a frame short of its
//!   matter — `an_area_raised_as_its_measure_keeps_the_matter`,
//!   `a_hole_left_full_breaks_the_matter`,
//!   `matter_reaching_outside_its_outline_breaks_the_matter`,
//!   `a_frame_whose_matter_misses_its_jamb_breaks_the_matter`
//! - every closed place is in the matter of one area, caught out by matter
//!   broken on purpose: a place in the matter of two areas, matter where
//!   nothing closes, a closed place in none —
//!   `a_rod_where_a_tube_is_drawn_puts_a_place_in_two_matters`,
//!   `matter_where_nothing_closes_is_untiled`,
//!   `a_closed_place_in_no_matter_is_untiled`
//! - #503's drawing breaks the second rule — no test: #524 fixed what it broke
//!   on before this net landed; the rod it would have shown is laid by hand in
//!   `a_rod_where_a_tube_is_drawn_puts_a_place_in_two_matters`, and the drawing
//!   itself now holds both rules in `what_random_sketches_found.rs`
//! - #540's twin windows break the first rule — no test: #543 fixed them
//!   meanwhile; their 325 mm² of matter for a measure of 500 is laid by hand in
//!   `a_frame_whose_matter_misses_its_jamb_breaks_the_matter`, and the three
//!   frames and the owner's notch hold both rules in
//!   `what_random_sketches_found.rs`
//! - a campaign has run with both rules, and what it found is written down —
//!   no test: the report is a comment on #508; the dominant finding, a hole
//!   touching its outline whose matter comes out short or empty, waits on an
//!   issue of its own
//! - `docs/sketch-soundness.md` lists both rules, and drops the matter from
//!   *What the rules cannot see* — no test: it is prose
//!
//! Closes #509.
//! - caught out by areas merged on purpose: two areas given as one, tinted and
//!   measured right, break the rule — `two_areas_given_as_one_break_the_count`
//! - the diagonal drawn before the square on its ends breaks it, and is kept
//!   under #494 — no test: not in this file. `what_random_sketches_found.rs`
//!   holds it, and its own `Closes #509` block names the test. #494 is closed,
//!   so the drawing holds today: what that test rejects is #494's old answer,
//!   the square alone as one area, against the reckoning's count of two
//! - the faces of no surface are told apart: #493's kept cases break this rule
//!   only where they already broke another, or not at all — no test: not in
//!   this file. `what_random_sketches_found.rs` holds #493's cases, and its own
//!   `Closes #509` block names the test
//! - a campaign has run with it, and what it found is written down: added to
//!   an open issue, or given one of its own — no test: the report lives on the
//!   issues, and any drawing it keeps in `what_random_sketches_found.rs`
//! - `docs/sketch-soundness.md` lists the rule, and drops one area where two
//!   are drawn from *What the rules cannot see* — no test: it is prose
//!
//! Closes #501.
//! - the four modes are chosen at launch, and the same seed in the same mode
//!   gives the same case — `a_mode_is_chosen_at_launch_as_it_is_written`,
//!   `the_same_seed_in_the_same_mode_draws_the_same_case`,
//!   `each_mode_gives_its_drawings_what_the_issue_says`
//! - three rules, each caught out on purpose — a dimension tells the truth:
//!   `a_drawing_settled_on_what_it_was_promised_tells_the_truth`,
//!   `a_dimension_measuring_other_than_its_value_breaks_the_truth`,
//!   `an_accepted_rule_that_does_not_hold_breaks_the_truth`,
//!   `a_refused_dimension_that_moves_the_drawing_breaks_the_truth`,
//!   `a_refused_rule_that_adds_or_takes_away_a_point_breaks_the_truth`; green is
//!   fixed: `pieces_made_entirely_constrained_and_said_to_be_keep_green_fixed`,
//!   `a_point_of_a_piece_made_entirely_constrained_not_said_settled_breaks_green`,
//!   `a_drawing_made_entirely_constrained_not_said_to_be_breaks_green`,
//!   `a_piece_still_green_with_one_dimension_taken_away_breaks_green`; the
//!   areas hold: `areas_keeping_their_rules_once_moved_hold`,
//!   `a_tint_left_bare_once_the_solver_moved_the_drawing_breaks_the_areas`,
//!   `a_closed_place_left_out_once_the_solver_moved_the_drawing_breaks_the_areas`,
//!   `fewer_areas_than_closed_once_the_solver_moved_the_drawing_break_the_areas`;
//!   and asked after every gesture of a played case:
//!   `a_value_typed_on_a_side_is_promised_and_held_once_settled`,
//!   `a_value_the_drawing_cannot_take_is_promised_to_leave_it_as_it_was`,
//!   `a_drag_that_moves_the_drawing_counts_as_moving_it`,
//!   `pieces_an_all_constrained_drawing_makes_green_are_held_to_their_green`,
//!   `the_check_of_a_played_case_asks_the_three_rules`
//! - a broken case shrinks with its dimensions and rules, and is printed as
//!   Rust that pastes into a named test —
//!   `a_broken_case_shrinks_keeping_the_dimensions_and_rules_it_breaks_on`,
//!   `dimensions_and_rules_print_as_the_rust_that_lays_them`,
//!   `the_printed_rust_lays_the_dimensions_and_rules_it_was_printed_from`,
//!   `a_case_shrunk_never_claims_a_piece_it_no_longer_made_green`
//! - the campaign is ignored by the gate and bounded by a deadline; the gate
//!   is no slower — `a_campaign_of_random_drawings_keeps_every_rule`; that the
//!   gate is no slower is measured on the test run, not asserted
//! - `docs/sketch-soundness.md` gains the modes and the three rules — no test:
//!   it is prose, held by `language.rs` and by nothing that asserts
//! - what the first campaigns find is written down, one issue per distinct
//!   failure. Nothing is fixed here — no test: not in this file.
//!   `what_random_sketches_found.rs` keeps each case ignored under its issue,
//!   and its own `Closes #501` block names the tests
//!
//! The campaign is run by hand:
//!
//! ```text
//! CAO_FUZZ_SECONDS=300 cargo test --release -p cao_sketch \
//!     --test every_sketch_keeps_its_areas -- --ignored --nocapture
//! ```
//!
//! `CAO_FUZZ_SEED` starts it from a given seed rather than from the clock, and
//! `CAO_FUZZ_PATIENCE` is how many seconds one drawing may take before it
//! counts as no answer.
//!
//! `CAO_FUZZ_MODE` chooses what each drawing is given besides what it draws:
//! `free` (the default, #500's campaign), `mix`, `shares` or `constrained`. In
//! the `shares` mode, `CAO_FUZZ_SHARES=30,50,20` gives the percentages of
//! dimensions, of rules and of pieces made entirely constrained.

mod random_sketches;

use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use glam::DVec2;
use random_sketches::{
    Area, Axis, Check, Drawing, Flaw, Gesture, Matter, Mode, Piece, Place, Promise, Promised,
    Random, Relaying, Report, Rule, Silence, answer, areas_hold, as_many_as_closed, campaign,
    drawn, drawn_in, green_is_fixed, keeps_its_promises, laid_alike, matter_is_measure,
    matter_tiles, nothing_extra, nothing_missing, played, promises_of, smaller, tells_the_truth,
    tint_is_measure,
};

fn square(low: [f64; 2], high: [f64; 2]) -> Area {
    let (low, high) = (DVec2::from(low), DVec2::from(high));
    let corners = [
        low,
        DVec2::new(high.x, low.y),
        high,
        DVec2::new(low.x, high.y),
    ];
    let triangles = vec![
        [corners[0], corners[1], corners[2]],
        [corners[0], corners[2], corners[3]],
    ];
    let measure = (high.x - low.x) * (high.y - low.y);
    Area {
        outline: corners.to_vec(),
        holes: Vec::new(),
        triangles: triangles.clone(),
        measure,
        sampling: 0.0,
        matter: Matter {
            triangles,
            measure,
            sampling: 0.0,
        },
    }
}

/// Cuts a `square()` hole out of a `square()` area, matter included — the
/// bands laid by hand hold only for axis-aligned rectangles.
fn hollowed(mut area: Area, hole: &Area) -> Area {
    let (low, high) = (area.outline[0], area.outline[2]);
    let (hole_low, hole_high) = (hole.outline[0], hole.outline[2]);
    area.holes.push(hole.outline.clone());
    area.matter.measure -= hole.measure;
    area.matter.triangles = [
        [low, DVec2::new(high.x, hole_low.y)],
        [DVec2::new(low.x, hole_high.y), high],
        [
            DVec2::new(low.x, hole_low.y),
            DVec2::new(hole_low.x, hole_high.y),
        ],
        [
            DVec2::new(hole_high.x, hole_low.y),
            DVec2::new(high.x, hole_high.y),
        ],
    ]
    .iter()
    .flat_map(|band| square([band[0].x, band[0].y], [band[1].x, band[1].y]).triangles)
    .collect();
    area
}

fn closed(at: [f64; 2]) -> Place {
    Place {
        at: DVec2::from(at),
        enclosed: true,
    }
}

fn open(at: [f64; 2]) -> Place {
    Place {
        at: DVec2::from(at),
        enclosed: false,
    }
}

#[test]
fn the_same_seed_draws_the_same_numbers() {
    let (mut first, mut second) = (Random::seeded(500), Random::seeded(500));
    assert!((0..100).all(|_| first.number() == second.number()));
    assert_ne!(Random::seeded(1).number(), Random::seeded(2).number());
}

#[test]
fn a_number_on_the_lattice_stays_between_its_bounds() {
    let mut random = Random::seeded(7);
    for _ in 0..1000 {
        let drawn = random.on_lattice(-2.0, 10.0, 0.5);
        assert!((-2.0..=10.0).contains(&drawn), "{drawn}");
        assert_eq!((drawn * 2.0).fract(), 0.0, "{drawn} is on the half lattice");
    }
}

#[test]
fn the_same_seed_draws_the_same_drawing() {
    for seed in 0..200 {
        assert_eq!(drawn(seed), drawn(seed), "seed {seed}");
    }
    let distinct: std::collections::BTreeSet<String> = (0..200)
        .map(|seed| Drawing(&drawn(seed)).to_string())
        .collect();
    assert!(distinct.len() > 190, "{} distinct drawings", distinct.len());
}

#[test]
fn a_mode_is_chosen_at_launch_as_it_is_written() {
    assert_eq!(Mode::read(None, None), Ok(Mode::Free));
    assert_eq!(Mode::read(Some("free"), None), Ok(Mode::Free));
    assert_eq!(Mode::read(Some("mix"), None), Ok(Mode::Mix));
    assert_eq!(
        Mode::read(Some("constrained"), None),
        Ok(Mode::AllConstrained)
    );
    assert_eq!(
        Mode::read(Some("shares"), Some("30,50,20")),
        Ok(Mode::Shares {
            dimensions: 30,
            rules: 50,
            constrained: 20,
        })
    );
    assert_eq!(
        Mode::read(Some("shares"), Some("0,100,0")),
        Ok(Mode::Shares {
            dimensions: 0,
            rules: 100,
            constrained: 0,
        })
    );
    for (mode, shares) in [
        (Some("everything"), None),
        (Some("shares"), None),
        (Some("shares"), Some("30,50")),
        (Some("shares"), Some("30,50,20,10")),
        (Some("shares"), Some("30,50,101")),
        (Some("shares"), Some("30,fifty,20")),
    ] {
        assert!(
            Mode::read(mode, shares).is_err(),
            "{mode:?} with shares {shares:?} was read as a mode"
        );
    }
}

fn every_mode() -> [Mode; 4] {
    [
        Mode::Free,
        Mode::Mix,
        Mode::Shares {
            dimensions: 30,
            rules: 50,
            constrained: 20,
        },
        Mode::AllConstrained,
    ]
}

#[test]
fn the_same_seed_in_the_same_mode_draws_the_same_case() {
    for mode in every_mode() {
        for seed in 0..40 {
            assert_eq!(
                drawn_in(seed, &mode),
                drawn_in(seed, &mode),
                "seed {seed} in {mode:?}"
            );
        }
    }
    for seed in 0..200 {
        assert_eq!(drawn_in(seed, &Mode::Free), drawn(seed), "seed {seed}");
    }
    let constrained_otherwise = (0..40)
        .filter(|seed| drawn_in(*seed, &Mode::AllConstrained) != drawn(*seed))
        .count();
    assert!(
        constrained_otherwise > 30,
        "only {constrained_otherwise} of 40 seeds draw another case all constrained than free"
    );
}

fn holds_a_trait(drawing: &[Gesture]) -> bool {
    random_sketches::laid(drawing)
        .live_segments()
        .next()
        .is_some()
}

#[test]
fn each_mode_gives_its_drawings_what_the_issue_says() {
    let in_mode =
        |mode: &Mode| -> Vec<Vec<Gesture>> { (0..60).map(|seed| drawn_in(seed, mode)).collect() };
    let constraining = |drawing: &[Gesture]| {
        drawing
            .iter()
            .filter(|gesture| gesture.constrains())
            .count()
    };

    assert!(
        in_mode(&Mode::Free)
            .iter()
            .flatten()
            .all(|gesture| !gesture.constrains() && !gesture.drags()),
        "a free drawing took a dimension, a rule or a drag"
    );

    let nought = Mode::Shares {
        dimensions: 0,
        rules: 0,
        constrained: 0,
    };
    assert!(
        in_mode(&nought)
            .iter()
            .flatten()
            .all(|gesture| !gesture.constrains()),
        "shares of nought gave a drawing a dimension or a rule"
    );

    let full = Mode::Shares {
        dimensions: 100,
        rules: 100,
        constrained: 100,
    };
    for drawing in in_mode(&full)
        .iter()
        .filter(|drawing| holds_a_trait(drawing))
    {
        assert!(
            constraining(drawing) > 0,
            "full shares gave nothing to {}",
            Drawing(drawing)
        );
    }

    let mixed: Vec<Vec<Gesture>> = in_mode(&Mode::Mix)
        .into_iter()
        .filter(|drawing| holds_a_trait(drawing))
        .collect();
    assert!(
        mixed.iter().any(|drawing| constraining(drawing) == 0),
        "no mixed drawing drew a share of nought"
    );
    assert!(
        mixed.iter().any(|drawing| constraining(drawing) > 2),
        "no mixed drawing drew a large share"
    );
    assert!(
        mixed.iter().flatten().any(Gesture::drags),
        "no mixed drawing was dragged"
    );

    let sketches: Vec<_> = in_mode(&Mode::AllConstrained)
        .iter()
        .map(|drawing| random_sketches::laid(drawing))
        .filter(|sketch| sketch.holds_a_drawing())
        .collect();
    let green = sketches
        .iter()
        .filter(|sketch| sketch.is_fully_constrained(1.0))
        .count();
    assert!(
        green * 4 > sketches.len() * 3,
        "only {green} of {} drawings all constrained are fully constrained",
        sketches.len()
    );
}

fn kind(gesture: &Gesture) -> &'static str {
    match gesture {
        Gesture::Chain { .. } => "a chain of traits",
        Gesture::Rectangle { .. } => "a rectangle",
        Gesture::Circle { .. } => "a circle",
        Gesture::Arc { .. } => "an arc",
        Gesture::Ellipse { .. } => "an ellipse",
        Gesture::HalfEllipse { .. } => "half an ellipse",
        Gesture::Point { .. } => "a lone point",
        Gesture::Fillet { .. } => "a fillet",
        Gesture::Chamfer { .. } => "a chamfer",
        Gesture::Mirror { .. } => "a mirror",
        Gesture::PatternAround { .. } => "a circular pattern",
        Gesture::PatternAlong { .. } => "a rectangular pattern",
        Gesture::Divide { .. } => "a division at a crossing",
        Gesture::Trim { .. } => "a trim",
        Gesture::Erase { .. } => "an erasure",
        Gesture::Dimension { .. } => "a dimension",
        Gesture::Rule { .. } => "a rule",
        Gesture::Drag { .. } => "a drag",
        Gesture::Green { .. } => "a piece made entirely constrained",
    }
}

fn is_construction(gesture: &Gesture) -> bool {
    matches!(
        gesture,
        Gesture::Chain {
            construction: true,
            ..
        } | Gesture::Rectangle {
            construction: true,
            ..
        } | Gesture::Circle {
            construction: true,
            ..
        } | Gesture::Arc {
            construction: true,
            ..
        } | Gesture::Ellipse {
            construction: true,
            ..
        } | Gesture::HalfEllipse {
            construction: true,
            ..
        }
    )
}

/// The places a gesture is drawn through or acts at.
fn places(gesture: &Gesture) -> Vec<DVec2> {
    let pairs: Vec<[f64; 2]> = match gesture {
        Gesture::Chain { through, .. } => through.clone(),
        Gesture::Rectangle {
            corner, opposite, ..
        } => vec![*corner, *opposite],
        Gesture::Circle { centre, .. } => vec![*centre],
        Gesture::Arc { centre, start, .. } => vec![*centre, *start],
        Gesture::Ellipse { centre, reach, .. } => vec![*centre, *reach],
        Gesture::HalfEllipse { from, to, .. } => vec![*from, *to],
        Gesture::Point { at }
        | Gesture::Fillet { at, .. }
        | Gesture::Chamfer { at, .. }
        | Gesture::Divide { at }
        | Gesture::Trim { at }
        | Gesture::Erase { at } => vec![*at],
        Gesture::Mirror { of, axis } | Gesture::PatternAlong { of, axis, .. } => {
            let mut places = of.clone();
            if let Axis::Trait(at) = axis {
                places.push(*at);
            }
            places
        }
        Gesture::PatternAround { of, centre, .. } => {
            let mut places = of.clone();
            places.push(*centre);
            places
        }
        Gesture::Dimension { clicks, .. } | Gesture::Rule { clicks, .. } => clicks.clone(),
        Gesture::Drag { from, to } => vec![*from, *to],
        Gesture::Green { piece, given } => piece
            .iter()
            .copied()
            .map(DVec2::from)
            .chain(given.iter().flat_map(places))
            .map(|at| at.to_array())
            .collect(),
    };
    pairs.into_iter().map(DVec2::from).collect()
}

/// The sizes a gesture is drawn with: radii, reaches, lengths and steps.
fn sizes(gesture: &Gesture) -> Vec<f64> {
    match gesture {
        Gesture::Circle { radius, .. } | Gesture::Fillet { radius, .. } => vec![*radius],
        Gesture::Ellipse { across, .. } => vec![*across],
        Gesture::HalfEllipse { rise, .. } => vec![*rise],
        Gesture::Chamfer { length, .. } => vec![*length],
        Gesture::PatternAlong { along, across, .. } => vec![along.0, across.0],
        _ => Vec::new(),
    }
}

/// The two corners of a rectangle, lowest first.
fn corners(gesture: &Gesture) -> Option<(DVec2, DVec2)> {
    match gesture {
        Gesture::Rectangle {
            corner, opposite, ..
        } => {
            let (corner, opposite) = (DVec2::from(*corner), DVec2::from(*opposite));
            Some((corner.min(opposite), corner.max(opposite)))
        }
        _ => None,
    }
}

/// Whether some gesture of some drawing is drawn from a rectangle drawn
/// before it, as `drawn_from` says.
fn drawn_from_a_rectangle(
    drawings: &[Vec<Gesture>],
    drawn_from: impl Fn(&Gesture, DVec2, DVec2) -> bool,
) -> bool {
    drawings.iter().any(|drawing| {
        drawing.iter().enumerate().any(|(index, gesture)| {
            drawing[..index]
                .iter()
                .filter_map(corners)
                .any(|(low, high)| drawn_from(gesture, low, high))
        })
    })
}

#[test]
fn the_generator_draws_with_every_gesture() {
    let drawings: Vec<Vec<Gesture>> = (0..3000).map(drawn).collect();
    let gestures = || drawings.iter().flatten();
    let seen = |what: &str, found: bool| assert!(found, "no drawing holds {what}");

    for what in [
        "a chain of traits",
        "a rectangle",
        "a circle",
        "an arc",
        "an ellipse",
        "half an ellipse",
        "a lone point",
        "a fillet",
        "a chamfer",
        "a mirror",
        "a circular pattern",
        "a rectangular pattern",
        "a division at a crossing",
        "a trim",
        "an erasure",
    ] {
        seen(what, gestures().any(|gesture| kind(gesture) == what));
    }
    for what in [
        "a chain of traits",
        "a rectangle",
        "a circle",
        "an arc",
        "an ellipse",
        "half an ellipse",
    ] {
        seen(
            &format!("{what} as construction geometry"),
            gestures().any(|gesture| kind(gesture) == what && is_construction(gesture)),
        );
    }
    seen(
        "a closed chain",
        gestures().any(|gesture| matches!(gesture, Gesture::Chain { closed: true, .. })),
    );
    seen(
        "an open chain",
        gestures().any(|gesture| matches!(gesture, Gesture::Chain { closed: false, .. })),
    );
    let hairs: Vec<(f64, f64)> = drawings
        .iter()
        .flat_map(|drawing| {
            let places: Vec<DVec2> = drawing.iter().flat_map(places).collect();
            let mut gaps = Vec::new();
            for (index, one) in places.iter().enumerate() {
                for other in &places[index + 1..] {
                    let gap = one.distance(*other);
                    if gap > 1e-12 && gap < 1e-2 {
                        gaps.push((gap, one.abs().max(other.abs()).max_element()));
                    }
                }
            }
            gaps
        })
        .collect();
    seen(
        "a chain a hair short of closing",
        gestures().any(|gesture| {
            matches!(gesture, Gesture::Chain { through, closed: false, .. }
            if through.len() > 3 && {
                let gap = DVec2::from(through[0]).distance(DVec2::from(through[through.len() - 1]));
                gap > 0.0 && gap < 1e-2
            })
        }),
    );
    for (gap, far) in &hairs {
        assert!(
            *gap >= 5.0 * 1e-7 * (1.0 + far),
            "a coincidence missed by {gap}, finer than five times one place at {far}"
        );
    }
    seen(
        "a drawing thirty times larger",
        gestures()
            .flat_map(places)
            .any(|at| at.abs().max_element() >= 250.0),
    );
    seen(
        "a rectangle sharing a whole side with one drawn before it",
        drawn_from_a_rectangle(&drawings, |gesture, low, high| {
            corners(gesture)
                .is_some_and(|(near, far)| near == DVec2::new(high.x, low.y) && far.y == high.y)
        }),
    );
    seen(
        "a circle inscribed in a rectangle drawn before it",
        drawn_from_a_rectangle(&drawings, |gesture, low, high| {
            matches!(gesture, Gesture::Circle { centre, radius, .. }
                if DVec2::from(*centre) == (low + high) / 2.0
                    && *radius == (high - low).min_element() / 2.0)
        }),
    );
    seen(
        "an ellipse inscribed in a rectangle drawn before it",
        drawn_from_a_rectangle(&drawings, |gesture, low, high| {
            let (middle, half) = ((low + high) / 2.0, (high - low) / 2.0);
            matches!(gesture, Gesture::Ellipse { centre, reach, across, .. }
                if DVec2::from(*centre) == middle
                    && DVec2::from(*reach) == middle + DVec2::X * half.x
                    && *across == half.y)
        }),
    );
    seen(
        "an arc drawn along a circle drawn before it",
        drawings.iter().any(|drawing| {
            drawing.iter().enumerate().any(|(index, gesture)| {
                let Gesture::Arc { centre, start, .. } = gesture else {
                    return false;
                };
                drawing[..index].iter().any(|earlier| {
                    matches!(earlier, Gesture::Circle { centre: middle, radius, .. }
                        if middle == centre
                            && DVec2::from(*start).distance(DVec2::from(*centre)) == *radius)
                })
            })
        }),
    );
    seen(
        "a chain landing on the corner of a rectangle drawn before it",
        drawn_from_a_rectangle(&drawings, |gesture, low, high| {
            let corners = [
                low,
                high,
                DVec2::new(low.x, high.y),
                DVec2::new(high.x, low.y),
            ];
            matches!(gesture, Gesture::Chain { through, .. }
                if through.iter().any(|at| corners.contains(&DVec2::from(*at))))
        }),
    );
    seen(
        "a drawing that closes nothing",
        drawings
            .iter()
            .any(|drawing| random_sketches::laid(drawing).regions().is_empty()),
    );
}

#[test]
fn every_change_the_generator_draws_changes_a_drawing_now_and_then() {
    let mut changed: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    let count = |sketch: &cao_sketch::Sketch| {
        (
            sketch.live_points().count(),
            sketch.live_segments().count(),
            sketch.live_circles().count(),
            sketch.live_arcs().count(),
            sketch.live_ellipses().count(),
            sketch.live_corner_cuts().count(),
            sketch
                .ellipses()
                .iter()
                .map(|ellipse| ellipse.drawn)
                .collect::<Vec<_>>(),
        )
    };
    for seed in 0..3000 {
        let mut sketch = cao_sketch::Sketch::new(cao_sketch::WorkPlane::XY);
        for gesture in drawn(seed) {
            let before = count(&sketch);
            gesture.lay(&mut sketch);
            if !gesture.draws() && count(&sketch) != before {
                changed.insert(kind(&gesture));
            }
        }
    }
    for what in [
        "a fillet",
        "a chamfer",
        "a mirror",
        "a circular pattern",
        "a rectangular pattern",
        "a division at a crossing",
        "a trim",
        "an erasure",
    ] {
        assert!(changed.contains(what), "{what} never changed a drawing");
    }
}

#[test]
fn a_drawing_made_with_every_tool_keeps_every_rule() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [0.0, 0.0],
            opposite: [10.0, 6.0],
            construction: false,
        },
        Gesture::Fillet {
            at: [10.0, 6.0],
            radius: 1.0,
        },
        Gesture::Chamfer {
            at: [0.0, 0.0],
            length: 1.0,
        },
        Gesture::Circle {
            centre: [3.0, 3.0],
            radius: 1.0,
            construction: false,
        },
        Gesture::Ellipse {
            centre: [7.0, 3.0],
            reach: [8.0, 3.0],
            across: 0.5,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[5.0, -2.0], [5.0, 8.0]],
            closed: false,
            construction: false,
        },
        Gesture::Divide { at: [5.0, 0.0] },
        Gesture::Divide { at: [5.0, 6.0] },
        Gesture::Trim { at: [5.0, -1.0] },
        Gesture::Trim { at: [5.0, 7.0] },
        Gesture::Chain {
            through: vec![[20.0, 0.0], [24.0, 0.0], [22.0, 3.0]],
            closed: true,
            construction: false,
        },
        Gesture::Mirror {
            of: vec![[22.0, 0.0], [23.0, 1.5], [21.0, 1.5]],
            axis: Axis::V,
        },
        Gesture::PatternAlong {
            of: vec![[22.0, 0.0], [23.0, 1.5], [21.0, 1.5]],
            axis: Axis::U,
            along: (6.0, 2),
            across: (5.0, 2),
        },
        Gesture::Arc {
            centre: [15.0, 15.0],
            start: [17.0, 15.0],
            degrees: 180.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[13.0, 15.0], [17.0, 15.0]],
            closed: false,
            construction: false,
        },
        Gesture::HalfEllipse {
            from: [12.0, 20.0],
            to: [18.0, 20.0],
            rise: 2.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[12.0, 20.0], [18.0, 20.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rectangle {
            corner: [30.0, 0.0],
            opposite: [32.0, 2.0],
            construction: false,
        },
        Gesture::PatternAround {
            of: vec![[31.0, 0.0], [32.0, 1.0], [31.0, 2.0], [30.0, 1.0]],
            centre: [0.0, 0.0],
            degrees: 90.0,
            count: 3,
        },
        Gesture::Rectangle {
            corner: [40.0, 0.0],
            opposite: [42.0, 2.0],
            construction: true,
        },
        Gesture::Point { at: [41.0, 1.0] },
        Gesture::Erase { at: [41.0, 0.0] },
    ]);
}

#[test]
fn an_element_named_twice_for_a_copy_is_let_go_as_the_tool_lets_it_go() {
    let circle = Gesture::Circle {
        centre: [3.0, 1.0],
        radius: 1.0,
        construction: false,
    };
    let mirrored = |of: Vec<[f64; 2]>| {
        random_sketches::laid(&[circle.clone(), Gesture::Mirror { of, axis: Axis::V }])
            .live_circles()
            .count()
    };
    assert_eq!(mirrored(vec![[3.0, 2.0]]), 2);
    assert_eq!(mirrored(vec![[3.0, 2.0], [3.0, 0.0]]), 1);
}

#[test]
fn a_flat_rectangle_is_laid_as_the_tool_lays_it() {
    let sketch = random_sketches::laid(&[Gesture::Rectangle {
        corner: [3.0, -2.0],
        opposite: [3.0, 6.0],
        construction: false,
    }]);
    assert_eq!(sketch.live_segments().count(), 4);
}

#[test]
fn a_number_clippy_takes_for_a_constant_is_written_so_that_it_pastes() {
    let cut_short = std::f64::consts::SQRT_2.next_down();
    let printed = Gesture::Point {
        at: [cut_short, 0.7071067811865475],
    }
    .to_string();
    assert_eq!(
        printed,
        format!(
            "Gesture::Point {{ at: [f64::from_bits({:#018x}), 0.7071067811865475] }}",
            cut_short.to_bits()
        )
    );
    assert_eq!(f64::from_bits(0x3ff6a09e667f3bcc), cut_short);
}

#[test]
fn an_area_tinted_as_its_outline_encloses_keeps_the_tint() {
    assert_eq!(tint_is_measure(&[square([0.0, 0.0], [2.0, 3.0])]), Ok(()));
}

#[test]
fn a_tint_reaching_outside_its_outline_breaks_the_tint() {
    let mut reaching = square([0.0, 0.0], [2.0, 2.0]);
    reaching.triangles.push([
        DVec2::new(2.0, 0.0),
        DVec2::new(3.0, 0.0),
        DVec2::new(2.0, 3.0),
    ]);
    assert_eq!(
        tint_is_measure(&[square([5.0, 5.0], [6.0, 6.0]), reaching]),
        Err(Flaw::Tint {
            near: DVec2::ZERO,
            tinted: 5.5,
            measured: 4.0,
        })
    );
}

#[test]
fn a_tint_leaving_part_of_its_outline_bare_breaks_the_tint() {
    let mut bare = square([0.0, 0.0], [2.0, 2.0]);
    bare.triangles.pop();
    assert!(matches!(
        tint_is_measure(&[bare]),
        Err(Flaw::Tint { tinted: 2.0, .. })
    ));
}

#[test]
fn a_tint_off_by_less_than_its_sampling_keeps_the_tint() {
    let mut sampled = square([0.0, 0.0], [2.0, 2.0]);
    sampled.measure = 4.01;
    assert!(tint_is_measure(&[sampled.clone()]).is_err());
    sampled.sampling = 0.02;
    assert_eq!(tint_is_measure(&[sampled]), Ok(()));
}

#[test]
fn a_small_piece_cut_from_a_big_circle_may_stray_only_by_the_steps_it_walks() {
    let sketch = random_sketches::laid(&[
        Gesture::Circle {
            centre: [0.0, 0.0],
            radius: 5.0,
            construction: false,
        },
        Gesture::Rectangle {
            corner: [4.5, -0.5],
            opposite: [5.5, 0.5],
            construction: false,
        },
    ]);
    let mut areas = random_sketches::areas::areas_of(&sketch);
    assert_eq!(tint_is_measure(&areas), Ok(()));
    let piece = areas
        .iter_mut()
        .find(|area| (area.measure - 0.4917).abs() < 1e-3)
        .expect("the piece of the square inside the circle");
    piece
        .triangles
        .sort_by(|a, b| random_sketches::surface(a).total_cmp(&random_sketches::surface(b)));
    let mut taken = 0.0;
    while taken < 0.01 {
        taken += random_sketches::surface(&piece.triangles.remove(0));
    }
    assert!(
        tint_is_measure(&areas).is_err(),
        "{taken} of tint went unseen"
    );
}

#[test]
fn an_area_raised_as_its_measure_keeps_the_matter() {
    let ring = hollowed(
        square([0.0, 0.0], [4.0, 4.0]),
        &square([1.0, 1.0], [2.0, 2.0]),
    );
    assert_eq!(
        matter_is_measure(&[square([5.0, 5.0], [6.0, 6.0]), ring]),
        Ok(())
    );
}

#[test]
fn a_hole_left_full_breaks_the_matter() {
    let mut full = hollowed(
        square([0.0, 0.0], [4.0, 4.0]),
        &square([1.0, 1.0], [2.0, 2.0]),
    );
    full.matter.triangles = full.triangles.clone();
    assert_eq!(
        matter_is_measure(&[full]),
        Err(Flaw::Matter {
            near: DVec2::ZERO,
            raised: 16.0,
            measured: 15.0,
        })
    );
}

#[test]
fn matter_reaching_outside_its_outline_breaks_the_matter() {
    let mut reaching = square([0.0, 0.0], [2.0, 2.0]);
    reaching.matter.triangles.push([
        DVec2::new(2.0, 0.0),
        DVec2::new(3.0, 0.0),
        DVec2::new(2.0, 3.0),
    ]);
    assert_eq!(
        matter_is_measure(&[reaching]),
        Err(Flaw::Matter {
            near: DVec2::ZERO,
            raised: 5.5,
            measured: 4.0,
        })
    );
}

#[test]
fn a_frame_whose_matter_misses_its_jamb_breaks_the_matter() {
    let mut frame = square([0.0, 0.0], [30.0, 30.0]);
    frame.holes.push(square([5.0, 5.0], [15.0, 25.0]).outline);
    frame.holes.push(square([15.0, 5.0], [25.0, 25.0]).outline);
    frame.matter.measure = 500.0;
    frame.matter.triangles = [
        square([25.0, 0.0], [30.0, 30.0]).triangles,
        square([5.0, 25.0], [25.0, 30.0]).triangles,
        square([10.0, 0.0], [25.0, 5.0]).triangles,
    ]
    .concat();
    assert_eq!(
        matter_is_measure(&[frame]),
        Err(Flaw::Matter {
            near: DVec2::ZERO,
            raised: 325.0,
            measured: 500.0,
        })
    );
}

#[test]
fn a_rod_where_a_tube_is_drawn_puts_a_place_in_two_matters() {
    let inner = square([1.0, 1.0], [2.0, 2.0]);
    let tube = hollowed(square([0.0, 0.0], [4.0, 4.0]), &inner);
    let mut rod = tube.clone();
    rod.matter.triangles = rod.triangles.clone();
    let places = [closed([1.5, 1.5]), closed([0.5, 0.5]), open([5.0, 5.0])];

    assert_eq!(matter_tiles(&[tube, inner.clone()], &places), Ok(()));
    assert_eq!(
        matter_tiles(&[rod, inner], &places),
        Err(Flaw::Untiled {
            at: DVec2::splat(1.5),
            areas: 2,
            enclosed: true,
        })
    );
}

#[test]
fn matter_where_nothing_closes_is_untiled() {
    let areas = [square([0.0, 0.0], [2.0, 2.0])];
    assert_eq!(matter_tiles(&areas, &[open([3.0, 1.0])]), Ok(()));
    assert_eq!(
        matter_tiles(&areas, &[open([1.0, 1.0])]),
        Err(Flaw::Untiled {
            at: DVec2::ONE,
            areas: 1,
            enclosed: false,
        })
    );
}

#[test]
fn a_closed_place_in_no_matter_is_untiled() {
    let mut bare = square([0.0, 0.0], [2.0, 2.0]);
    bare.matter.triangles.clear();
    assert_eq!(
        matter_tiles(&[bare], &[closed([1.0, 1.0])]),
        Err(Flaw::Untiled {
            at: DVec2::ONE,
            areas: 0,
            enclosed: true,
        })
    );
}

#[test]
fn a_closed_place_lying_in_no_area_is_missing() {
    let inner = square([1.0, 2.0], [3.0, 4.0]);
    let outer = hollowed(square([0.0, 0.0], [4.0, 4.0]), &inner);
    let places = [closed([0.5, 0.5]), closed([2.0, 3.0]), open([5.0, 5.0])];

    assert_eq!(nothing_missing(&[outer.clone(), inner], &places), Ok(()));
    assert_eq!(
        nothing_missing(&[outer], &places),
        Err(Flaw::Missing {
            at: DVec2::new(2.0, 3.0)
        })
    );
}

#[test]
fn a_place_in_two_areas_is_extra() {
    let places = [closed([1.5, 1.5])];
    let apart = [
        square([0.0, 0.0], [1.0, 1.0]),
        square([1.0, 1.0], [2.0, 2.0]),
    ];
    let overlapping = [
        square([0.0, 0.0], [2.0, 2.0]),
        square([1.0, 1.0], [3.0, 3.0]),
    ];

    assert_eq!(nothing_extra(&apart, &places), Ok(()));
    assert_eq!(
        nothing_extra(&overlapping, &places),
        Err(Flaw::Extra {
            at: DVec2::splat(1.5),
            areas: 2,
            enclosed: true,
        })
    );
}

#[test]
fn an_area_where_nothing_closes_is_extra() {
    let areas = [square([0.0, 0.0], [2.0, 2.0])];
    assert_eq!(nothing_extra(&areas, &[open([3.0, 1.0])]), Ok(()));
    assert_eq!(
        nothing_extra(&areas, &[open([1.0, 1.0])]),
        Err(Flaw::Extra {
            at: DVec2::ONE,
            areas: 1,
            enclosed: false,
        })
    );
}

#[test]
fn two_areas_given_as_one_break_the_count() {
    let places = [closed([0.5, 0.5]), closed([1.5, 0.5]), open([3.0, 0.5])];
    let apart = [
        square([0.0, 0.0], [1.0, 1.0]),
        square([1.0, 0.0], [2.0, 1.0]),
    ];
    let merged = [square([0.0, 0.0], [2.0, 1.0])];

    assert_eq!(tint_is_measure(&merged), Ok(()));
    assert_eq!(matter_is_measure(&merged), Ok(()));
    assert_eq!(nothing_missing(&merged, &places), Ok(()));
    assert_eq!(nothing_extra(&merged, &places), Ok(()));
    assert_eq!(matter_tiles(&merged, &places), Ok(()));
    assert_eq!(as_many_as_closed(&apart, 2), Ok(()));
    let flaw = as_many_as_closed(&merged, 2);
    assert_eq!(
        flaw,
        Err(Flaw::Miscounted {
            found: 1,
            closed: 2
        })
    );
    assert_eq!(flaw.map_err(|flaw| flaw.rule()), Err(Rule::AsManyAsClosed));
}

#[test]
fn a_hole_takes_its_place_out_of_the_area_around_it() {
    let inner = square([1.0, 1.0], [2.0, 2.0]);
    let outer = hollowed(square([0.0, 0.0], [3.0, 3.0]), &inner);
    let places = [closed([1.5, 1.5]), closed([0.5, 0.5])];
    let areas = [outer, inner];

    assert_eq!(nothing_extra(&areas, &places), Ok(()));
    assert_eq!(nothing_missing(&areas, &places), Ok(()));
}

#[test]
fn a_place_on_the_seam_between_two_triangles_lies_in_their_area() {
    let corner = DVec2::new(4.000028991378029, 2.0000289913780285);
    let [a, b, c, d] = [
        corner,
        DVec2::new(1.0, corner.y),
        DVec2::new(1.0, -3.0),
        DVec2::new(corner.x, -3.0),
    ];
    let area = Area {
        outline: vec![a, b, c, d],
        holes: Vec::new(),
        triangles: vec![[a, b, c], [a, c, d]],
        measure: (a.x - c.x) * (a.y - c.y),
        sampling: 0.0,
        matter: Matter {
            triangles: vec![[a, b, c], [a, c, d]],
            measure: (a.x - c.x) * (a.y - c.y),
            sampling: 0.0,
        },
    };
    let seam = (d + a + b) / 3.0;
    assert_eq!(random_sketches::holding(&[area], seam).len(), 1);
}

#[test]
fn a_sliver_of_tint_along_a_line_holds_nothing_past_its_ends() {
    let turn = DVec2::from_angle(30f64.to_radians());
    let [a, b, c] = [[6.5, -0.5], [6.5, -1.25], [6.5, -2.0]].map(|at| turn.rotate(DVec2::from(at)));
    let sliver = Area {
        outline: vec![a, b, c],
        holes: Vec::new(),
        triangles: vec![[a, b, c]],
        measure: 0.0,
        sampling: 0.0,
        matter: Matter {
            triangles: vec![[a, b, c]],
            measure: 0.0,
            sampling: 0.0,
        },
    };
    let beyond = turn.rotate(DVec2::new(6.5, 1.0));
    assert!(random_sketches::holding(&[sliver], beyond).is_empty());
}

fn moved(area: &Area, by: DVec2) -> Area {
    let mut moved = area.clone();
    for place in moved.outline.iter_mut() {
        *place += by;
    }
    for corner in moved.triangles.iter_mut().flatten() {
        *corner += by;
    }
    for corner in moved.matter.triangles.iter_mut().flatten() {
        *corner += by;
    }
    moved
}

#[test]
fn the_same_areas_laid_again_keep_the_order() {
    let by = DVec2::new(10.0, -3.0);
    let first = [
        square([0.0, 0.0], [1.0, 1.0]),
        square([2.0, 0.0], [4.0, 2.0]),
    ];
    let second: Vec<Area> = first.iter().rev().map(|area| moved(area, by)).collect();
    let places = [DVec2::splat(0.5), DVec2::new(3.0, 1.0), DVec2::splat(9.0)];

    assert_eq!(
        laid_alike(Relaying::Moved(by), &first, &second, &places, |at| at + by),
        Ok(())
    );
}

#[test]
fn an_area_laid_again_off_by_what_a_weld_moves_keeps_the_order() {
    let first = [square([0.0, 0.0], [4.0, 5.0])];
    let mut second = first.clone();
    second[0].measure += 1.78e-6;
    assert_eq!(
        laid_alike(
            Relaying::Moved(DVec2::X),
            &first,
            &second,
            &[DVec2::ONE],
            |at| at
        ),
        Ok(())
    );
}

#[test]
fn an_area_laid_again_off_by_more_than_a_weld_moves_breaks_the_order() {
    let first = [square([0.0, 0.0], [4.0, 5.0])];
    let mut second = first.clone();
    second[0].measure += 1e-3;
    assert_eq!(
        laid_alike(
            Relaying::Moved(DVec2::X),
            &first,
            &second,
            &[DVec2::ONE],
            |at| at
        ),
        Err(Flaw::Relaid {
            how: Relaying::Moved(DVec2::X),
            areas: (1, 1),
            at: None,
        })
    );
}

#[test]
fn a_small_area_laid_again_a_fifth_larger_breaks_the_order_however_far_the_drawing_reaches() {
    let first = [
        square([0.0, 0.0], [0.1, 0.1]),
        square([2000.0, 2000.0], [3000.0, 3000.0]),
    ];
    let second = [square([0.0, 0.0], [0.11, 0.11]), first[1].clone()];
    assert_eq!(
        laid_alike(
            Relaying::Reordered,
            &first,
            &second,
            &[DVec2::splat(2500.0)],
            |at| at
        ),
        Err(Flaw::Relaid {
            how: Relaying::Reordered,
            areas: (2, 2),
            at: None,
        })
    );
}

#[test]
fn a_hair_the_walk_welds_only_once_the_drawing_is_moved_out_is_not_held_against_it() {
    random_sketches::holds(&[
        Gesture::Rectangle {
            corner: [100.0, 100.0],
            opposite: [110.0, 110.0],
            construction: false,
        },
        Gesture::Chain {
            through: vec![[1.0, 1.0], [3.0, 1.0], [2.0, 3.0], [1.5, 1.000002]],
            closed: false,
            construction: false,
        },
    ]);
}

#[test]
fn another_count_of_areas_laid_again_breaks_the_order() {
    let first = [
        square([0.0, 0.0], [1.0, 1.0]),
        square([2.0, 0.0], [4.0, 2.0]),
    ];
    assert_eq!(
        laid_alike(Relaying::Reordered, &first, &first[..1], &[], |at| at),
        Err(Flaw::Relaid {
            how: Relaying::Reordered,
            areas: (2, 1),
            at: None,
        })
    );
}

#[test]
fn a_place_in_an_area_of_another_size_laid_again_breaks_the_order() {
    let first = [
        square([0.0, 0.0], [1.0, 1.0]),
        square([2.0, 0.0], [4.0, 2.0]),
    ];
    let second = [
        square([0.0, 0.0], [2.0, 2.0]),
        square([3.0, 0.0], [4.0, 1.0]),
    ];
    let places = [DVec2::new(3.5, 0.5), DVec2::splat(0.5)];

    assert_eq!(
        laid_alike(Relaying::Turned(90.0), &first, &second, &places, |at| at),
        Err(Flaw::Relaid {
            how: Relaying::Turned(90.0),
            areas: (2, 2),
            at: Some(DVec2::new(3.5, 0.5)),
        })
    );
}

#[test]
fn a_drawing_with_an_arc_and_half_an_ellipse_holds_mirrored() {
    random_sketches::holds(&[
        Gesture::Arc {
            centre: [2.0, 0.0],
            start: [4.0, 0.0],
            degrees: 180.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[4.0, 0.0], [0.0, 0.0]],
            closed: false,
            construction: false,
        },
        Gesture::HalfEllipse {
            from: [5.0, 0.0],
            to: [9.0, 0.0],
            rise: 2.0,
            construction: false,
        },
        Gesture::Chain {
            through: vec![[5.0, 0.0], [9.0, 0.0]],
            closed: false,
            construction: false,
        },
    ]);
}

fn mirrored(area: &Area) -> Area {
    let mut mirrored = area.clone();
    for place in mirrored.outline.iter_mut() {
        place.x = -place.x;
    }
    for corner in mirrored.triangles.iter_mut().flatten() {
        corner.x = -corner.x;
    }
    for place in mirrored.holes.iter_mut().flatten() {
        place.x = -place.x;
    }
    for corner in mirrored.matter.triangles.iter_mut().flatten() {
        corner.x = -corner.x;
    }
    mirrored
}

#[test]
fn the_same_areas_mirrored_keep_the_order() {
    let first = [
        square([0.0, 0.0], [1.0, 1.0]),
        square([2.0, 0.0], [4.0, 2.0]),
    ];
    let second: Vec<Area> = first.iter().map(mirrored).collect();
    let places = [DVec2::splat(0.5), DVec2::new(3.0, 1.0), DVec2::splat(9.0)];

    assert_eq!(
        laid_alike(Relaying::Mirrored, &first, &second, &places, |at| {
            DVec2::new(-at.x, at.y)
        }),
        Ok(())
    );
}

#[test]
fn an_area_of_another_size_mirrored_breaks_the_order() {
    let first = [
        square([0.0, 0.0], [1.0, 1.0]),
        square([2.0, 0.0], [4.0, 2.0]),
    ];
    let second = [
        mirrored(&square([0.0, 0.0], [2.0, 2.0])),
        mirrored(&square([3.0, 0.0], [4.0, 1.0])),
    ];
    let places = [DVec2::new(3.5, 0.5), DVec2::splat(0.5)];

    assert_eq!(
        laid_alike(Relaying::Mirrored, &first, &second, &places, |at| {
            DVec2::new(-at.x, at.y)
        }),
        Err(Flaw::Relaid {
            how: Relaying::Mirrored,
            areas: (2, 2),
            at: Some(DVec2::new(3.5, 0.5)),
        })
    );
}

const PATIENCE: Duration = Duration::from_secs(5);

#[test]
fn a_walk_that_panics_answers_with_the_panic() {
    let check: Check<u64> = Arc::new(|_| panic!("the walk never closes"));
    assert_eq!(
        answer(1, &check, PATIENCE),
        Err(Flaw::NoAnswer(Silence::Panicked(Some(
            "the walk never closes".to_string()
        ))))
    );
}

#[test]
fn a_walk_that_never_comes_back_gives_no_answer() {
    let check: Check<u64> = Arc::new(|_| {
        std::thread::sleep(Duration::from_secs(2));
        Ok(())
    });
    let verdict = answer(1, &check, Duration::from_millis(20));
    assert!(
        matches!(verdict, Err(Flaw::NoAnswer(Silence::Late(_)))),
        "{verdict:?}"
    );
}

fn at(x: f64, y: f64) -> DVec2 {
    DVec2::new(x, y)
}

fn measured(near: [f64; 2], value: f64, measured: f64) -> Promise {
    Promise::Measured {
        near: DVec2::from(near),
        value,
        measured,
    }
}

fn ruled(near: [f64; 2], off_by: f64) -> Promise {
    Promise::Ruled {
        near: DVec2::from(near),
        off_by,
    }
}

#[test]
fn a_drawing_settled_on_what_it_was_promised_tells_the_truth() {
    let corners = vec![at(0.0, 0.0), at(10.0, 0.0), at(10.0, 5.0)];
    assert_eq!(
        tells_the_truth(&[
            measured([5.0, 0.0], 10.0, 10.0),
            ruled([10.0, 2.5], 0.0),
            Promise::Refused {
                before: corners.clone(),
                after: corners,
            },
        ]),
        Ok(())
    );
}

#[test]
fn a_dimension_measuring_other_than_its_value_breaks_the_truth() {
    let verdict = tells_the_truth(&[
        measured([5.0, 0.0], 10.0, 10.0),
        measured([1.0, 2.0], 10.0, 12.0),
    ]);
    assert_eq!(
        verdict,
        Err(Flaw::Untrue {
            near: at(1.0, 2.0),
            value: 10.0,
            measured: 12.0,
        })
    );
    assert_eq!(
        verdict.map_err(|flaw| flaw.rule()),
        Err(Rule::TellsTheTruth)
    );
}

#[test]
fn an_accepted_rule_that_does_not_hold_breaks_the_truth() {
    assert_eq!(
        tells_the_truth(&[ruled([10.0, 2.5], 0.0), ruled([3.0, 4.0], 1.5)]),
        Err(Flaw::Unkept {
            near: at(3.0, 4.0),
            off_by: 1.5,
        })
    );
}

#[test]
fn a_refused_dimension_that_moves_the_drawing_breaks_the_truth() {
    assert_eq!(
        tells_the_truth(&[Promise::Refused {
            before: vec![at(0.0, 0.0), at(10.0, 0.0), at(10.0, 5.0)],
            after: vec![at(0.0, 0.0), at(12.0, 0.0), at(10.0, 5.0)],
        }]),
        Err(Flaw::Budged {
            from: at(10.0, 0.0),
            to: at(12.0, 0.0),
        })
    );
}

#[test]
fn a_refused_rule_that_adds_or_takes_away_a_point_breaks_the_truth() {
    let corners = vec![at(0.0, 0.0), at(10.0, 0.0), at(10.0, 5.0)];
    let mut grown = corners.clone();
    grown.push(at(5.0, 0.0));
    assert_eq!(
        tells_the_truth(&[Promise::Refused {
            before: corners.clone(),
            after: grown,
        }]),
        Err(Flaw::Recounted {
            before: 3,
            after: 4,
        })
    );
    assert_eq!(
        tells_the_truth(&[Promise::Refused {
            before: corners.clone(),
            after: corners[..2].to_vec(),
        }]),
        Err(Flaw::Recounted {
            before: 3,
            after: 2,
        })
    );
}

fn piece(points: &[([f64; 2], bool)], green_without: &[bool]) -> Piece {
    Piece {
        points: points
            .iter()
            .map(|(place, settled)| (DVec2::from(*place), *settled))
            .collect(),
        green_without: green_without.to_vec(),
    }
}

fn settled_rectangle(low: [f64; 2], high: [f64; 2]) -> Piece {
    piece(
        &[
            (low, true),
            ([high[0], low[1]], true),
            (high, true),
            ([low[0], high[1]], true),
        ],
        &[false, false, false],
    )
}

#[test]
fn pieces_made_entirely_constrained_and_said_to_be_keep_green_fixed() {
    let pieces = [
        settled_rectangle([0.0, 0.0], [10.0, 5.0]),
        settled_rectangle([20.0, 0.0], [24.0, 3.0]),
    ];
    assert_eq!(green_is_fixed(&pieces, Some(true)), Ok(()));
    assert_eq!(green_is_fixed(&pieces, None), Ok(()));
}

#[test]
fn a_point_of_a_piece_made_entirely_constrained_not_said_settled_breaks_green() {
    let mut loose = settled_rectangle([0.0, 0.0], [10.0, 5.0]);
    loose.points[2].1 = false;
    let verdict = green_is_fixed(&[settled_rectangle([20.0, 0.0], [24.0, 3.0]), loose], None);
    assert_eq!(
        verdict,
        Err(Flaw::NotGreen {
            at: Some(at(10.0, 5.0)),
        })
    );
    assert_eq!(verdict.map_err(|flaw| flaw.rule()), Err(Rule::GreenIsFixed));
}

#[test]
fn a_drawing_made_entirely_constrained_not_said_to_be_breaks_green() {
    assert_eq!(
        green_is_fixed(&[settled_rectangle([0.0, 0.0], [10.0, 5.0])], Some(false)),
        Err(Flaw::NotGreen { at: None })
    );
}

#[test]
fn a_piece_still_green_with_one_dimension_taken_away_breaks_green() {
    let mut overheld = settled_rectangle([4.0, 3.0], [10.0, 8.0]);
    overheld.green_without = vec![false, true, false];
    assert_eq!(
        green_is_fixed(
            &[settled_rectangle([20.0, 0.0], [24.0, 3.0]), overheld],
            Some(true)
        ),
        Err(Flaw::StillGreen {
            near: at(4.0, 3.0),
            without: 1,
        })
    );
}

#[test]
fn areas_keeping_their_rules_once_moved_hold() {
    assert_eq!(
        areas_hold(
            &[square([0.0, 0.0], [2.0, 2.0])],
            &[closed([1.0, 1.0]), open([5.0, 5.0])],
            Some(1)
        ),
        Ok(())
    );
}

#[test]
fn a_tint_left_bare_once_the_solver_moved_the_drawing_breaks_the_areas() {
    let mut bare = square([0.0, 0.0], [2.0, 2.0]);
    bare.triangles.pop();
    let verdict = areas_hold(&[bare], &[], None);
    assert_eq!(
        verdict,
        Err(Flaw::Moved(Box::new(Flaw::Tint {
            near: DVec2::ZERO,
            tinted: 2.0,
            measured: 4.0,
        })))
    );
    assert_eq!(verdict.map_err(|flaw| flaw.rule()), Err(Rule::AreasHold));
}

#[test]
fn a_closed_place_left_out_once_the_solver_moved_the_drawing_breaks_the_areas() {
    let verdict = areas_hold(
        &[square([0.0, 0.0], [2.0, 2.0])],
        &[closed([1.0, 1.0]), closed([5.0, 5.0])],
        Some(1),
    );
    assert!(
        matches!(&verdict, Err(Flaw::Moved(inner))
            if matches!(**inner, Flaw::Missing { at } | Flaw::Untiled { at, .. } if at == DVec2::new(5.0, 5.0))),
        "{verdict:?}"
    );
}

#[test]
fn fewer_areas_than_closed_once_the_solver_moved_the_drawing_break_the_areas() {
    assert_eq!(
        areas_hold(&[square([0.0, 0.0], [2.0, 2.0])], &[], Some(2)),
        Err(Flaw::Moved(Box::new(Flaw::Miscounted {
            found: 1,
            closed: 2,
        })))
    );
}

/// A right triangle whose two short sides are given the values they measure,
/// and its long side then given `long`.
fn a_triangle_given_its_long_side(long: f64) -> Vec<Gesture> {
    vec![
        Gesture::Chain {
            through: vec![[1.0, 1.0], [5.0, 1.0], [1.0, 4.0]],
            closed: true,
            construction: false,
        },
        Gesture::Dimension {
            clicks: vec![[3.0, 1.0], [3.3, 1.6]],
            value: 4.0,
        },
        Gesture::Dimension {
            clicks: vec![[1.0, 2.5], [1.6, 2.2]],
            value: 3.0,
        },
        Gesture::Dimension {
            clicks: vec![[3.0, 2.5], [3.6, 2.9]],
            value: long,
        },
    ]
}

fn values_measured(promised: &Promised) -> Vec<(f64, f64)> {
    promised
        .promises
        .iter()
        .filter_map(|promise| match promise {
            Promise::Measured {
                value, measured, ..
            } => Some((*value, *measured)),
            _ => None,
        })
        .collect()
}

#[test]
fn a_value_typed_on_a_side_is_promised_and_held_once_settled() {
    let case = a_triangle_given_its_long_side(6.0);
    let promised = promises_of(&case);

    assert_eq!(promised.len(), 4);
    assert!(promised[0].promises.is_empty(), "{:?}", promised[0]);
    assert_eq!(
        promised.iter().map(|after| after.moved).collect::<Vec<_>>(),
        [false, false, false, true],
        "values the triangle already measured move nothing; its long side made 6 does"
    );
    let mut values = values_measured(&promised[3]);
    values.sort_by(|one, other| one.0.total_cmp(&other.0));
    assert_eq!(
        values.iter().map(|(value, _)| *value).collect::<Vec<_>>(),
        [3.0, 4.0, 6.0]
    );
    for (value, measured) in values {
        assert!(
            (measured - value).abs() < 1e-6,
            "a side given {value} measures {measured} once settled"
        );
    }
    assert_eq!(random_sketches::check(&case), Ok(()));
}

#[test]
fn a_value_the_drawing_cannot_take_is_promised_to_leave_it_as_it_was() {
    let promised = promises_of(&a_triangle_given_its_long_side(100.0));

    let refused: Vec<(&Vec<DVec2>, &Vec<DVec2>)> = promised[3]
        .promises
        .iter()
        .filter_map(|promise| match promise {
            Promise::Refused { before, after } => Some((before, after)),
            _ => None,
        })
        .collect();
    assert_eq!(refused.len(), 1, "{:?}", promised[3]);
    let (before, after) = refused[0];
    assert!(!before.is_empty());
    assert_eq!(before, after);
    assert_eq!(values_measured(&promised[3]).len(), 2);
    assert!(!promised[3].moved);
}

#[test]
fn a_drag_that_moves_the_drawing_counts_as_moving_it() {
    let promised = promises_of(&[
        Gesture::Rectangle {
            corner: [1.0, 1.0],
            opposite: [5.0, 3.0],
            construction: false,
        },
        Gesture::Drag {
            from: [5.0, 3.0],
            to: [6.0, 4.0],
        },
    ]);
    assert_eq!(
        promised.iter().map(|after| after.moved).collect::<Vec<_>>(),
        [false, true]
    );
    assert!(promised.iter().all(|after| after.promises.is_empty()));
}

#[test]
fn pieces_an_all_constrained_drawing_makes_green_are_held_to_their_green() {
    let mut drawn_with_a_trait = 0;
    let mut claiming = 0;
    let mut asking_the_whole = 0;
    for seed in 0..10 {
        let case = drawn_in(seed, &Mode::AllConstrained);
        if !holds_a_trait(&case) {
            continue;
        }
        drawn_with_a_trait += 1;
        let promised = promises_of(&case);
        let last = promised.last().expect("a case of some gestures");
        claiming += usize::from(!last.pieces.is_empty());
        asking_the_whole += usize::from(last.whole.is_some());
        assert!(
            last.pieces.iter().all(|piece| !piece.points.is_empty()),
            "seed {seed}: {:?}",
            last.pieces
        );
    }
    assert!(
        claiming >= 7,
        "only {claiming} of {drawn_with_a_trait} drawings all constrained hold a piece to its green"
    );
    assert!(
        asking_the_whole * 2 >= drawn_with_a_trait,
        "only {asking_the_whole} of {drawn_with_a_trait} drawings all constrained ask the whole drawing"
    );
    for seed in 0..10 {
        for after in promises_of(&drawn(seed)) {
            assert_eq!(
                after,
                Promised {
                    promises: Vec::new(),
                    pieces: Vec::new(),
                    whole: None,
                    moved: false,
                },
                "seed {seed}, drawn free"
            );
        }
    }
}

#[test]
fn the_check_of_a_played_case_asks_the_three_rules() {
    let nothing = Promised {
        promises: Vec::new(),
        pieces: Vec::new(),
        whole: None,
        moved: false,
    };
    let bare = Flaw::Tint {
        near: DVec2::ZERO,
        tinted: 2.0,
        measured: 4.0,
    };
    assert_eq!(keeps_its_promises(&nothing, Ok(())), Ok(()));
    assert_eq!(
        keeps_its_promises(&nothing, Err(bare.clone())),
        Err(bare.clone())
    );
    let moved = Promised {
        moved: true,
        ..nothing.clone()
    };
    assert_eq!(
        keeps_its_promises(&moved, Err(bare.clone())),
        Err(Flaw::Moved(Box::new(bare)))
    );
    let untrue = Promised {
        promises: vec![measured([1.0, 2.0], 10.0, 12.0)],
        ..nothing.clone()
    };
    assert_eq!(
        keeps_its_promises(&untrue, Ok(())),
        Err(Flaw::Untrue {
            near: at(1.0, 2.0),
            value: 10.0,
            measured: 12.0,
        })
    );
    let mut loose = settled_rectangle([0.0, 0.0], [10.0, 5.0]);
    loose.points[2].1 = false;
    let ungreen = Promised {
        pieces: vec![loose],
        ..nothing
    };
    assert_eq!(
        keeps_its_promises(&ungreen, Ok(())),
        Err(Flaw::NotGreen {
            at: Some(at(10.0, 5.0)),
        })
    );
}

/// A case is a list of numbers, and leaves a place in no area when it holds a
/// seven.
fn sevens() -> Check<Vec<u64>> {
    Arc::new(|case: &Vec<u64>| {
        if case.contains(&7) {
            Err(Flaw::Missing { at: DVec2::ZERO })
        } else {
            Ok(())
        }
    })
}

fn shorter(case: &[u64]) -> Vec<Vec<u64>> {
    (0..case.len())
        .map(|index| {
            let mut fewer = case.to_vec();
            fewer.remove(index);
            fewer
        })
        .collect()
}

#[test]
fn a_campaign_shrinks_what_it_finds_and_counts_the_rest() {
    let draw = |seed: u64| vec![seed % 10, seed % 7 + 5, seed % 3];
    let report = campaign(
        0..40,
        draw,
        sevens(),
        |case: &Vec<u64>| shorter(case),
        PATIENCE,
        || true,
    );

    assert_eq!(report.tried, 40);
    let broken: usize = report.broken.iter().map(|(_, count)| count).sum();
    assert_eq!(
        broken,
        (0..40).filter(|seed| draw(*seed).contains(&7)).count()
    );
    assert_eq!(report.findings.len(), 1, "{:?}", report.findings);
    assert_eq!(report.findings[0].shrunk, vec![7]);
    assert_eq!(report.findings[0].shrunk_flaw.rule(), Rule::NothingMissing);
}

#[test]
fn a_failing_drawing_shrinks_to_the_gesture_that_fails() {
    let holds_a_circle = |gestures: &Vec<Gesture>| {
        random_sketches::laid(gestures)
            .live_circles()
            .any(|(_, circle)| !circle.construction)
    };
    let drawn = (0..3000)
        .map(drawn)
        .find(|gestures| gestures.len() >= 4 && holds_a_circle(gestures))
        .expect("a drawing of four gestures or more, one of them a circle");

    let shrunk =
        random_sketches::shrink(drawn, |gestures| smaller(gestures), holds_a_circle, || true);

    assert_eq!(shrunk.len(), 1, "{}", Drawing(&shrunk));
    assert!(
        matches!(
            shrunk[0],
            Gesture::Circle {
                construction: false,
                ..
            }
        ),
        "{}",
        Drawing(&shrunk)
    );
    assert!(
        places(&shrunk[0])
            .iter()
            .flat_map(|at| at.to_array())
            .chain(sizes(&shrunk[0]))
            .all(|number| number.fract() == 0.0),
        "{}",
        Drawing(&shrunk)
    );
}

#[test]
fn shrinking_any_drawn_drawing_comes_to_an_end() {
    for seed in 0..200 {
        let mut rounds = 0;
        let shrunk = random_sketches::shrink(
            drawn(seed),
            |gestures| smaller(gestures),
            |gestures| !gestures.is_empty(),
            || {
                rounds += 1;
                rounds < 100_000
            },
        );
        assert!(rounds < 100_000, "seed {seed} was still shrinking");
        assert!(
            smaller(&shrunk).iter().all(Vec::is_empty),
            "seed {seed} stopped short: {}",
            Drawing(&shrunk)
        );
    }
}

fn every_kind() -> Vec<Gesture> {
    vec![
        Gesture::Chain {
            through: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0]],
            closed: true,
            construction: false,
        },
        Gesture::Rectangle {
            corner: [5.0, 0.0],
            opposite: [9.0, 2.5],
            construction: true,
        },
        Gesture::Circle {
            centre: [2.0, 6.0],
            radius: 1.5,
            construction: false,
        },
        Gesture::Arc {
            centre: [6.0, 6.0],
            start: [7.0, 6.0],
            degrees: 135.0,
            construction: false,
        },
        Gesture::Ellipse {
            centre: [10.0, 6.0],
            reach: [12.0, 6.0],
            across: 1.0,
            construction: false,
        },
        Gesture::HalfEllipse {
            from: [0.0, 10.0],
            to: [4.0, 10.0],
            rise: -1.5,
            construction: false,
        },
        Gesture::Point {
            at: [1e-7, 9.0000005],
        },
        Gesture::Fillet {
            at: [4.0, 0.0],
            radius: 0.5,
        },
        Gesture::Chamfer {
            at: [4.0, 3.0],
            length: 0.25,
        },
        Gesture::Mirror {
            of: vec![[2.0, 0.0], [2.0, 6.0]],
            axis: Axis::Trait([7.0, 0.0]),
        },
        Gesture::PatternAround {
            of: vec![[3.5, 6.0]],
            centre: [2.0, 6.0],
            degrees: 90.0,
            count: 3,
        },
        Gesture::PatternAlong {
            of: vec![[2.0, 0.0]],
            axis: Axis::U,
            along: (5.0, 2),
            across: (0.0, 1),
        },
        Gesture::Divide {
            at: [std::f64::consts::SQRT_2, 1.5],
        },
        Gesture::Trim { at: [2.0, 0.0] },
        Gesture::Erase { at: [10.0, 7.0] },
    ]
}

#[test]
fn a_drawing_prints_as_the_rust_that_lays_it() {
    assert_eq!(
        Drawing(&every_kind()).to_string(),
        "&[
    Gesture::Chain { through: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0]], closed: true, construction: false },
    Gesture::Rectangle { corner: [5.0, 0.0], opposite: [9.0, 2.5], construction: true },
    Gesture::Circle { centre: [2.0, 6.0], radius: 1.5, construction: false },
    Gesture::Arc { centre: [6.0, 6.0], start: [7.0, 6.0], degrees: 135.0, construction: false },
    Gesture::Ellipse { centre: [10.0, 6.0], reach: [12.0, 6.0], across: 1.0, construction: false },
    Gesture::HalfEllipse { from: [0.0, 10.0], to: [4.0, 10.0], rise: -1.5, construction: false },
    Gesture::Point { at: [1e-7, 9.0000005] },
    Gesture::Fillet { at: [4.0, 0.0], radius: 0.5 },
    Gesture::Chamfer { at: [4.0, 3.0], length: 0.25 },
    Gesture::Mirror { of: vec![[2.0, 0.0], [2.0, 6.0]], axis: Axis::Trait([7.0, 0.0]) },
    Gesture::PatternAround { of: vec![[3.5, 6.0]], centre: [2.0, 6.0], degrees: 90.0, count: 3 },
    Gesture::PatternAlong { of: vec![[2.0, 0.0]], axis: Axis::U, along: (5.0, 2), across: (0.0, 1) },
    Gesture::Divide { at: [std::f64::consts::SQRT_2, 1.5] },
    Gesture::Trim { at: [2.0, 0.0] },
    Gesture::Erase { at: [10.0, 7.0] },
]"
    );
}

#[test]
fn the_printed_rust_lays_the_drawing_it_was_printed_from() {
    let pasted: &[Gesture] = &[
        Gesture::Chain {
            through: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0]],
            closed: true,
            construction: false,
        },
        Gesture::Rectangle {
            corner: [5.0, 0.0],
            opposite: [9.0, 2.5],
            construction: true,
        },
        Gesture::Circle {
            centre: [2.0, 6.0],
            radius: 1.5,
            construction: false,
        },
        Gesture::Arc {
            centre: [6.0, 6.0],
            start: [7.0, 6.0],
            degrees: 135.0,
            construction: false,
        },
        Gesture::Ellipse {
            centre: [10.0, 6.0],
            reach: [12.0, 6.0],
            across: 1.0,
            construction: false,
        },
        Gesture::HalfEllipse {
            from: [0.0, 10.0],
            to: [4.0, 10.0],
            rise: -1.5,
            construction: false,
        },
        Gesture::Point {
            at: [1e-7, 9.0000005],
        },
        Gesture::Fillet {
            at: [4.0, 0.0],
            radius: 0.5,
        },
        Gesture::Chamfer {
            at: [4.0, 3.0],
            length: 0.25,
        },
        Gesture::Mirror {
            of: vec![[2.0, 0.0], [2.0, 6.0]],
            axis: Axis::Trait([7.0, 0.0]),
        },
        Gesture::PatternAround {
            of: vec![[3.5, 6.0]],
            centre: [2.0, 6.0],
            degrees: 90.0,
            count: 3,
        },
        Gesture::PatternAlong {
            of: vec![[2.0, 0.0]],
            axis: Axis::U,
            along: (5.0, 2),
            across: (0.0, 1),
        },
        Gesture::Divide {
            at: [std::f64::consts::SQRT_2, 1.5],
        },
        Gesture::Trim { at: [2.0, 0.0] },
        Gesture::Erase { at: [10.0, 7.0] },
    ];
    assert_eq!(pasted, every_kind().as_slice());

    for seed in 0..300 {
        let printed = Drawing(&drawn(seed)).to_string();
        assert!(
            !printed.contains("NaN") && !printed.contains("inf"),
            "seed {seed} prints a number Rust cannot read back: {printed}"
        );
    }
}

fn a_constrained_case() -> Vec<Gesture> {
    vec![
        Gesture::Chain {
            through: vec![[1.0, 1.0], [5.0, 1.0], [1.0, 4.0]],
            closed: true,
            construction: false,
        },
        Gesture::Dimension {
            clicks: vec![[3.0, 1.0], [3.3, 1.6]],
            value: 0.1 + 0.2,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Perpendicular,
            clicks: vec![[3.0, 1.0], [1.0, 2.5]],
        },
        Gesture::Drag {
            from: [5.0, 1.0],
            to: [6.0, 1.0],
        },
    ]
}

#[test]
fn dimensions_and_rules_print_as_the_rust_that_lays_them() {
    assert_eq!(
        Drawing(&a_constrained_case()).to_string(),
        "&[
    Gesture::Chain { through: vec![[1.0, 1.0], [5.0, 1.0], [1.0, 4.0]], closed: true, construction: false },
    Gesture::Dimension { clicks: vec![[3.0, 1.0], [3.3, 1.6]], value: 0.30000000000000004 },
    Gesture::Rule { rule: cao_sketch::Rule::Perpendicular, clicks: vec![[3.0, 1.0], [1.0, 2.5]] },
    Gesture::Drag { from: [5.0, 1.0], to: [6.0, 1.0] },
]"
    );
}

#[test]
fn the_printed_rust_lays_the_dimensions_and_rules_it_was_printed_from() {
    let pasted: &[Gesture] = &[
        Gesture::Chain {
            through: vec![[1.0, 1.0], [5.0, 1.0], [1.0, 4.0]],
            closed: true,
            construction: false,
        },
        Gesture::Dimension {
            clicks: vec![[3.0, 1.0], [3.3, 1.6]],
            value: 0.30000000000000004,
        },
        Gesture::Rule {
            rule: cao_sketch::Rule::Perpendicular,
            clicks: vec![[3.0, 1.0], [1.0, 2.5]],
        },
        Gesture::Drag {
            from: [5.0, 1.0],
            to: [6.0, 1.0],
        },
    ];
    assert_eq!(pasted, a_constrained_case().as_slice());

    let mut nested = 0;
    for mode in [Mode::Mix, Mode::AllConstrained] {
        for seed in 0..60 {
            for gesture in drawn_in(seed, &mode) {
                let values = values_within(&gesture);
                if matches!(gesture, Gesture::Green { .. }) {
                    nested += values.len();
                }
                let printed = gesture.to_string();
                let read_back: Vec<Option<u64>> = printed
                    .split("value: ")
                    .skip(1)
                    .map(|rest| rest.split(" }").next().unwrap_or(rest))
                    .map(|written| match written.strip_prefix("f64::from_bits(") {
                        Some(bits) => u64::from_str_radix(
                            bits.trim_end_matches(')').trim_start_matches("0x"),
                            16,
                        )
                        .ok(),
                        None => written.parse::<f64>().ok().map(f64::to_bits),
                    })
                    .collect();
                assert_eq!(
                    read_back,
                    values
                        .iter()
                        .map(|value| Some(value.to_bits()))
                        .collect::<Vec<_>>(),
                    "seed {seed} in {mode:?}: {values:?} printed as {printed}"
                );
            }
        }
    }
    assert!(
        nested > 0,
        "no value given to a piece made green was printed"
    );
}

/// The values a gesture types, those it gives a piece made green included, in
/// the order it types them.
fn values_within(gesture: &Gesture) -> Vec<f64> {
    match gesture {
        Gesture::Dimension { value, .. } => vec![*value],
        Gesture::Green { given, .. } => given.iter().flat_map(values_within).collect(),
        _ => Vec::new(),
    }
}

/// How many gestures of a kind a case holds, those given to a piece made green
/// included.
fn holding_within(gestures: &[Gesture], kind: fn(&Gesture) -> bool) -> usize {
    gestures
        .iter()
        .map(|gesture| match gesture {
            Gesture::Green { given, .. } => holding_within(given, kind),
            _ => usize::from(kind(gesture)),
        })
        .sum()
}

/// Whether a drawing, laid, carries a value that drives it and a rule laid
/// square or parallel.
fn driven_and_ruled(gestures: &[Gesture]) -> bool {
    let sketch = random_sketches::laid(gestures);
    sketch
        .dimensions()
        .iter()
        .any(|dimension| !dimension.driven)
        && sketch.constraints().iter().any(|constraint| {
            matches!(
                constraint,
                cao_sketch::Constraint::Perpendicular { .. }
                    | cao_sketch::Constraint::Parallel { .. }
            )
        })
}

#[test]
fn a_broken_case_shrinks_keeping_the_dimensions_and_rules_it_breaks_on() {
    let case = (0..500)
        .map(|seed| drawn_in(seed, &Mode::Mix))
        .find(|case| case.len() >= 6 && driven_and_ruled(case))
        .expect("a mixed case of six gestures or more, driven and ruled");

    let shrunk = random_sketches::shrink(
        case.clone(),
        |gestures| smaller(gestures),
        |gestures: &Vec<Gesture>| driven_and_ruled(gestures),
        || true,
    );

    assert!(driven_and_ruled(&shrunk), "{}", Drawing(&shrunk));
    assert!(
        holding_within(&shrunk, |gesture| matches!(
            gesture,
            Gesture::Dimension { .. }
        )) >= 1,
        "{}",
        Drawing(&shrunk)
    );
    assert!(
        holding_within(&shrunk, |gesture| matches!(gesture, Gesture::Rule { .. })) >= 1,
        "{}",
        Drawing(&shrunk)
    );
    assert!(shrunk.len() < case.len(), "{}", Drawing(&shrunk));
    for index in 0..shrunk.len() {
        let mut fewer = shrunk.clone();
        fewer.remove(index);
        assert!(
            !driven_and_ruled(&fewer),
            "gesture {index} of the shrunk case is not needed: {}",
            Drawing(&shrunk)
        );
    }
}

#[test]
fn a_case_shrunk_never_claims_a_piece_it_no_longer_made_green() {
    for seed in 0..3 {
        let case = drawn_in(seed, &Mode::AllConstrained);
        let green = |case: &[Gesture]| {
            promises_of(case)
                .last()
                .is_none_or(|last| green_is_fixed(&last.pieces, last.whole).is_ok())
        };
        if !holds_a_trait(&case) || !green(&case) {
            continue;
        }
        for candidate in smaller(&case) {
            assert!(
                promises_of(&candidate).last().is_none_or(|last| !matches!(
                    green_is_fixed(&last.pieces, last.whole),
                    Err(Flaw::NotGreen { .. })
                )),
                "seed {seed}: shrunk into a piece claimed green that was not given what green takes: {}",
                Drawing(&candidate)
            );
        }
    }
}

/// A case holding a seven breaks the tint while anything is left beside the
/// seven, and leaves a place in no area once the seven is alone.
fn sevens_change_their_rule() -> Check<Vec<u64>> {
    Arc::new(|case: &Vec<u64>| match (case.contains(&7), case.len()) {
        (false, _) => Ok(()),
        (true, 1) => Err(Flaw::Missing { at: DVec2::ZERO }),
        (true, _) => Err(Flaw::Tint {
            near: DVec2::ZERO,
            tinted: 1.0,
            measured: 0.0,
        }),
    })
}

#[test]
fn a_case_is_never_shrunk_into_breaking_another_rule() {
    let report = campaign(
        [17],
        |_| vec![3, 7, 1],
        sevens_change_their_rule(),
        |case: &Vec<u64>| shorter(case),
        PATIENCE,
        || true,
    );
    let finding = &report.findings[0];
    assert_eq!(finding.shrunk.len(), 2, "{finding:?}");
    assert_eq!(finding.shrunk_flaw.rule(), Rule::TintIsMeasure);
}

/// The case a seed stands for in the mode chosen at launch, with the seed
/// written where a campaign that ends the program still leaves it to be read.
fn drawn_noisily(seed: u64) -> Vec<Gesture> {
    eprint!("\rseed {seed} ");
    drawn_in(seed, chosen_mode())
}

/// The mode `CAO_FUZZ_MODE` and `CAO_FUZZ_SHARES` choose, read once. One
/// they do not name stops the campaign: a campaign run in a mode nobody asked
/// for reports on cases nobody meant.
fn chosen_mode() -> &'static Mode {
    static CHOSEN: std::sync::OnceLock<Mode> = std::sync::OnceLock::new();
    CHOSEN.get_or_init(|| {
        let written = |name: &str| std::env::var(name).ok();
        match Mode::read(
            written("CAO_FUZZ_MODE").as_deref(),
            written("CAO_FUZZ_SHARES").as_deref(),
        ) {
            Ok(mode) => {
                eprintln!("in mode {mode:?}");
                mode
            }
            Err(refused) => {
                eprintln!("CAO_FUZZ_MODE and CAO_FUZZ_SHARES refused: {refused}");
                panic!("CAO_FUZZ_MODE and CAO_FUZZ_SHARES refused: {refused}");
            }
        }
    })
}

fn from_the_environment(name: &str) -> Option<u64> {
    std::env::var(name).ok()?.parse().ok()
}

#[test]
#[ignore = "a campaign, run by hand: see the head of this file"]
fn a_campaign_of_random_drawings_keeps_every_rule() {
    let seconds = from_the_environment("CAO_FUZZ_SECONDS").unwrap_or(60);
    let first = from_the_environment("CAO_FUZZ_SEED").unwrap_or_else(|| {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(1, |since| since.as_nanos() as u64);
        Random::seeded(now).number() >> 16
    });
    let patience = Duration::from_secs(from_the_environment("CAO_FUZZ_PATIENCE").unwrap_or(30));
    let deadline = Instant::now() + Duration::from_secs(seconds);
    println!("campaign from seed {first}, for {seconds} s");

    let check: Check<Vec<Gesture>> =
        Arc::new(|gestures: &Vec<Gesture>| random_sketches::check(gestures));
    let quiet = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let report = campaign(
        first..,
        drawn_noisily,
        check,
        |gestures: &Vec<Gesture>| smaller(gestures),
        patience,
        || Instant::now() < deadline,
    );
    std::panic::set_hook(quiet);

    print_report(&report, patience);
    assert!(
        report.findings.is_empty(),
        "{} drawings of {} broke a rule",
        report.broken.iter().map(|(_, count)| count).sum::<usize>(),
        report.tried
    );
}

fn print_report(report: &Report<Vec<Gesture>>, patience: Duration) {
    println!("{} drawings tried", report.tried);
    for (rule, count) in &report.broken {
        println!("  {rule:?}: broken by {count}");
    }
    for finding in &report.findings {
        let shrunk = finding.shrunk.clone();
        let after = match random_sketches::apart(move || played(&shrunk), patience) {
            Ok(Err((after, _))) => format!("after gesture {after} of {}", finding.shrunk.len()),
            Ok(Ok(())) => "and keeps every rule when played again".to_string(),
            Err(silence) => format!("and gives no answer when played again: {silence:?}"),
        };
        println!(
            "\n── {:?}, seed {} ──\nas drawn: {:?}\nshrunk:   {:?}, {after}\n\n#[test]\nfn seed_{}_keeps_the_rules_of_its_areas() {{\n    random_sketches::holds({});\n}}",
            finding.flaw.rule(),
            finding.seed,
            finding.flaw,
            finding.shrunk_flaw,
            finding.seed,
            Drawing(&finding.shrunk).to_string().replace('\n', "\n    "),
        );
    }
}
