//! Several edges of the part chosen in the edge panel at once, added and taken
//! out with Cmd or Shift, and chamfered or rounded together as one step.
//!
//! Closes #979 — the whole transcription is here; criterion 6, the part file,
//! is held in `cao_part`, where the blend step and its file live.
//! - Cmd + click and Shift + click on a second edge, with the edge panel open,
//!   leave both edges chosen and lit; a plain click on a third leaves only the
//!   third —
//!   `cmd_or_shift_click_on_a_second_edge_leaves_both_chosen_and_lit_and_a_plain_click_on_a_third_leaves_only_it`
//! - Cmd or Shift + click on an edge already chosen takes it out; taking out
//!   the last one greys « Appliquer » —
//!   `cmd_or_shift_click_on_an_edge_already_chosen_takes_it_out_and_taking_out_the_last_greys_apply`
//! - The panel reads « 1 arête » with one edge chosen and « 3 arêtes » with
//!   three — `the_panel_counts_one_edge_in_the_singular_and_three_in_the_plural`
//! - « Appliquer » with the four top edges of a block chosen lays one step in
//!   the history, and the body afterwards is the one `Body::fillet_edges`
//!   (resp. `chamfer_edges`) gives for those four edges at that size —
//!   `apply_on_the_four_top_edges_lays_one_step_and_the_body_the_kernel_gives_for_all_four`;
//!   the step as the part lays, writes, reads, undoes, redoes and compacts
//!   it, in `crates/part/tests/several_edges_blended_in_one_step.rs`
//! - A choice the kernel declines lays nothing, shows the reason in the panel
//!   — in the plural with several edges — and keeps every edge chosen —
//!   `a_choice_the_kernel_declines_lays_nothing_says_why_in_the_plural_and_keeps_every_edge_chosen`
//! - A `.caopart` holding a blend step written with the single `edge` field
//!   opens and replays to the same body as before — no test: held in
//!   `crates/part/tests/a_blend_step_of_one_edge_still_opens.rs`, where the
//!   step's serde lives and a file frozen before this change needs no window,
//!   by `a_part_whose_blend_step_names_a_single_edge_opens_and_replays_to_the_same_body`
//! - Under the section view, Cmd + click on an edge on the hidden side adds
//!   nothing — no test: held in `picking_an_edge_through_a_section.rs`,
//!   beside the pocketed block and the section's gestures, by
//!   `under_the_section_view_cmd_click_on_an_edge_on_the_hidden_side_adds_nothing`
//! - Turning « Chanfrein » into « Congé » keeps every edge chosen —
//!   `turning_the_chamfer_into_a_fillet_keeps_every_edge_chosen`
//! - found on the way, beyond the issue's eight: a chamfer or a fillet
//!   holding several edges, not built once the part replays, says so of its
//!   edges in the plural in its alert — no test: held in
//!   `crates/app/src/screens/title_bar/state/tests.rs`, where the alerts'
//!   sentences are read, by
//!   `a_blend_of_several_edges_not_built_speaks_of_its_edges_in_the_plural`

mod driver;

use cao_part::history::StepKind;
use cao_solid::{Body, DrawnEdge, EdgeName};
use glam::{DVec2, DVec3};

/// The block of `driver::a_block`, as the part holds it, seen from +X, -Y
/// and +Z: the face towards +X is the near face.
struct Block {
    low: DVec3,
    high: DVec3,
}

impl Block {
    fn of(test: &str) -> Self {
        let (low, high) = the_part(test, "Bloc")
            .body()
            .bounds()
            .expect("the block holds matter");
        Self { low, high }
    }

    fn near_top(&self) -> [DVec3; 2] {
        let (low, high) = (self.low, self.high);
        [
            DVec3::new(high.x, low.y, high.z),
            DVec3::new(high.x, high.y, high.z),
        ]
    }

    fn near_left(&self) -> [DVec3; 2] {
        let (low, high) = (self.low, self.high);
        [
            DVec3::new(high.x, low.y, high.z),
            DVec3::new(high.x, low.y, low.z),
        ]
    }

    /// The top of the face towards -Y, seen.
    fn left_top(&self) -> [DVec3; 2] {
        let (low, high) = (self.low, self.high);
        [
            DVec3::new(high.x, low.y, high.z),
            DVec3::new(low.x, low.y, high.z),
        ]
    }

    /// The four edges of the top face, the near top first.
    fn the_four_top_edges(&self) -> [[DVec3; 2]; 4] {
        let (low, high) = (self.low, self.high);
        let corner = |x: f64, y: f64| DVec3::new(x, y, high.z);
        [
            self.near_top(),
            self.left_top(),
            [corner(low.x, low.y), corner(low.x, high.y)],
            [corner(low.x, high.y), corner(high.x, high.y)],
        ]
    }
}

fn the_part(test: &str, named: &str) -> cao_part::PartDocument {
    let path = driver::the_part_s_file(test, named);
    cao_part::PartDocument::load(&cao_app::adapters::files::DiskFiles, &path)
        .unwrap_or_else(|err| panic!("the part at {path:?} does not open: {err:?}"))
}

fn middle([start, end]: [DVec3; 2]) -> DVec3 {
    (start + end) / 2.0
}

fn is_the_edge(points: &[DVec3], [start, end]: [DVec3; 2]) -> bool {
    let near = |a: DVec3, b: DVec3| a.distance(b) < 1e-6;
    match (points.first(), points.last()) {
        (Some(first), Some(last)) => {
            (near(*first, start) && near(*last, end)) || (near(*first, end) && near(*last, start))
        }
        _ => false,
    }
}

/// Whether the edges drawn are these, each of them at least once and nothing
/// else.
fn are_the_edges(drawn: &[Vec<DVec3>], expected: &[[DVec3; 2]]) -> bool {
    expected
        .iter()
        .all(|edge| drawn.iter().any(|points| is_the_edge(points, *edge)))
        && drawn
            .iter()
            .all(|points| expected.iter().any(|edge| is_the_edge(points, *edge)))
}

fn chosen(app: &driver::App) -> Vec<Vec<DVec3>> {
    app.state().the_edges_chosen()
}

fn lit(app: &driver::App) -> Vec<Vec<DVec3>> {
    app.state().the_edges_lit()
}

/// The block, the Extrusion tab open and `tool` clicked in it.
fn opened_on(test: &str, tool: &str) -> (driver::App, Block) {
    let mut app = driver::a_block(test);
    let block = Block::of(test);
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, tool);
    (app, block)
}

#[test]
fn cmd_or_shift_click_on_a_second_edge_leaves_both_chosen_and_lit_and_a_plain_click_on_a_third_leaves_only_it()
 {
    for (held, modifiers) in [
        ("Cmd", egui::Modifiers::COMMAND),
        ("Shift", egui::Modifiers::SHIFT),
    ] {
        let test = format!("a_second_edge_is_added_with_{}", held.to_lowercase());
        let (mut app, block) = opened_on(&test, "Chanfrein");

        driver::click_at_world_point(&mut app, middle(block.near_top()));
        assert!(
            are_the_edges(&chosen(&app), &[block.near_top()]),
            "a plain click on the near top chose {:?}",
            chosen(&app),
        );

        driver::click_at_world_point_with(&mut app, middle(block.near_left()), modifiers);
        let both = [block.near_top(), block.near_left()];
        assert!(
            are_the_edges(&chosen(&app), &both),
            "{held} + click on the near left, after the near top, chose {:?}",
            chosen(&app),
        );
        assert!(
            are_the_edges(&lit(&app), &both),
            "{held} + click on the near left, after the near top, lights {:?}",
            lit(&app),
        );

        driver::click_at_world_point(&mut app, middle(block.left_top()));
        assert!(
            are_the_edges(&chosen(&app), &[block.left_top()]),
            "a plain click on the left top, after {held} + click, chose {:?}",
            chosen(&app),
        );
        assert!(
            are_the_edges(&lit(&app), &[block.left_top()]),
            "a plain click on the left top, after {held} + click, lights {:?}",
            lit(&app),
        );
    }
}

#[test]
fn cmd_or_shift_click_on_an_edge_already_chosen_takes_it_out_and_taking_out_the_last_greys_apply() {
    for (held, modifiers) in [
        ("Cmd", egui::Modifiers::COMMAND),
        ("Shift", egui::Modifiers::SHIFT),
    ] {
        let test = format!("an_edge_chosen_is_taken_out_with_{}", held.to_lowercase());
        let (mut app, block) = opened_on(&test, "Chanfrein");
        driver::click_at_world_point(&mut app, middle(block.near_top()));
        driver::click_at_world_point_with(&mut app, middle(block.near_left()), modifiers);
        assert!(
            are_the_edges(&chosen(&app), &[block.near_top(), block.near_left()]),
            "the fixture chooses the near top and the near left, choosing {:?}",
            chosen(&app),
        );

        driver::click_at_world_point_with(&mut app, middle(block.near_top()), modifiers);
        assert!(
            are_the_edges(&chosen(&app), &[block.near_left()]),
            "{held} + click on the near top, chosen, leaves {:?} chosen",
            chosen(&app),
        );
        assert!(
            driver::is_enabled(&app, "Appliquer"),
            "« Appliquer » greyed with the near left still chosen",
        );

        driver::click_at_world_point_with(&mut app, middle(block.near_left()), modifiers);
        assert!(
            chosen(&app).is_empty(),
            "{held} + click on the near left, the last chosen, leaves {:?} chosen",
            chosen(&app),
        );
        assert!(
            !driver::is_enabled(&app, "Appliquer"),
            "« Appliquer » offered once {held} + click took out the last edge chosen",
        );
    }
}

#[test]
fn the_panel_counts_one_edge_in_the_singular_and_three_in_the_plural() {
    let test = "the_panel_counts_the_edges_chosen";
    let (mut app, block) = opened_on(test, "Congé");

    driver::click_at_world_point(&mut app, middle(block.near_top()));
    assert!(
        driver::reads(&app, "1 arête"),
        "one edge chosen, the panel does not read « 1 arête », showing {:?}",
        driver::on_screen(&app),
    );

    driver::click_at_world_point_with(&mut app, middle(block.near_left()), egui::Modifiers::SHIFT);
    driver::click_at_world_point_with(&mut app, middle(block.left_top()), egui::Modifiers::SHIFT);
    assert_eq!(chosen(&app).len(), 3, "the fixture chooses three edges");
    assert!(
        driver::reads(&app, "3 arêtes"),
        "three edges chosen, the panel does not read « 3 arêtes », showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        !driver::reads(&app, "1 arête"),
        "three edges chosen, the panel still reads « 1 arête »",
    );
}

/// The names the body gives the edges drawn between these ends.
fn names_of(body: &Body, edges: &[[DVec3; 2]]) -> Vec<EdgeName> {
    let drawn = body.drawn_edges();
    edges
        .iter()
        .map(|edge| {
            drawn
                .iter()
                .find(|named| is_the_edge(&named.points, *edge))
                .unwrap_or_else(|| {
                    panic!(
                        "the block draws no edge from {:?} to {:?}",
                        edge[0], edge[1]
                    )
                })
                .name
                .clone()
        })
        .collect()
}

fn faces_of(body: &Body) -> usize {
    (0..body.faces_end())
        .filter(|face| body.has_face(*face))
        .count()
}

#[test]
fn apply_on_the_four_top_edges_lays_one_step_and_the_body_the_kernel_gives_for_all_four() {
    for (tool, value, modifiers) in [
        ("Congé", "Rayon :", egui::Modifiers::COMMAND),
        ("Chanfrein", "Distance :", egui::Modifiers::SHIFT),
    ] {
        let test = format!("the_four_top_edges_in_one_step_{}", tool.to_lowercase());
        let (mut app, block) = opened_on(&test, tool);
        let before = the_part(&test, "Bloc");
        let top = block.the_four_top_edges();
        let names = names_of(before.body(), &top);
        let expected = match tool {
            "Congé" => before
                .body()
                .fillet_edges(&names, 2.0)
                .map(|made| made.body),
            _ => before
                .body()
                .chamfer_edges(&names, 2.0)
                .map(|made| made.body),
        }
        .unwrap_or_else(|declined| {
            panic!("the kernel declines « {tool} » on the four top edges: {declined:?}")
        });

        driver::open_the_history(&mut app);
        let lines = driver::steps(&app);
        driver::click_at_world_point(&mut app, middle(top[0]));
        for edge in &top[1..] {
            driver::click_at_world_point_with(&mut app, middle(*edge), modifiers);
        }
        assert!(
            are_the_edges(&chosen(&app), &top),
            "the fixture chooses the four top edges, choosing {:?}",
            chosen(&app),
        );
        driver::type_beside(&mut app, value, "2");
        driver::click(&mut app, "Appliquer");

        let after = the_part(&test, "Bloc");
        assert_eq!(
            after.history.steps().len(),
            before.history.steps().len() + 1,
            "« {tool} » applied on the four top edges laid {} steps",
            after.history.steps().len() as isize - before.history.steps().len() as isize,
        );
        let laid = after.history.steps().last().expect("a step laid");
        assert_eq!(
            laid.kind(),
            StepKind::Blend,
            "« {tool} » laid a step of another kind"
        );
        assert_eq!(
            driver::steps(&app).len(),
            lines.len() + 1,
            "« {tool} » applied on the four top edges, the history reads {:?}",
            driver::steps(&app),
        );

        let made = after.body();
        let what = format!("« {tool} » of 2 on the four top edges");
        assert!(
            (made.volume() - expected.volume()).abs() <= 1e-9 * expected.volume(),
            "{what} leaves {}, where the kernel gives {}",
            made.volume(),
            expected.volume(),
        );
        assert_eq!(faces_of(made), faces_of(&expected), "{what}: faces");
        assert_eq!(
            made.drawn_edges().len(),
            expected.drawn_edges().len(),
            "{what}: edges drawn",
        );
        let (made_low, made_high) = made.bounds().expect("matter");
        let (low, high) = expected.bounds().expect("matter");
        assert!(
            made_low.abs_diff_eq(low, 1e-9) && made_high.abs_diff_eq(high, 1e-9),
            "{what} stands from {made_low:?} to {made_high:?}, where the kernel's stands from {low:?} to {high:?}",
        );
    }
}

/// A part named « Disque »: a circle drawn on the plane YZ, 40 across its
/// radius, raised 60 along X, its sketch then finished. Its two rims are
/// circles, which a chamfer does not take until #700.
fn a_disc(test: &str) -> driver::App {
    let mut app = driver::open_held_with(test, cao_prefs::Settings::default());
    driver::create_a_part(&mut app, "Disque");
    driver::start_a_sketch(&mut app);
    driver::click_exactly(&mut app, "Cercle (C)");
    driver::click_at_sketch_point(&mut app, driver::ON_THE_BLOCK);
    driver::click_at_sketch_point(&mut app, driver::ON_THE_BLOCK + DVec2::new(40.0, 0.0));
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_the_button(&mut app, "1");
    driver::click(&mut app, "Ajout de matière");
    driver::click_at_sketch_point(&mut app, driver::ON_THE_BLOCK);
    driver::type_beside(&mut app, "Hauteur :", "60");
    driver::click(&mut app, "Appliquer");
    driver::press(&mut app, egui::Key::Escape);
    driver::finish_the_sketch(&mut app);
    driver::click_the_button(&mut app, "Esquisse");
    assert!(
        !driver::a_sketch_is_open(&app),
        "the sketch the disc was raised from is still being edited, showing {:?}",
        driver::on_screen(&app),
    );
    app
}

/// Of an edge's points, the one standing furthest on screen from `away`.
fn furthest_on_screen_from(app: &driver::App, edge: &DrawnEdge, away: egui::Pos2) -> DVec3 {
    edge.points
        .iter()
        .copied()
        .max_by(|a, b| {
            let from = |point: DVec3| driver::world_point_on_screen(app, point).distance(away);
            from(*a).total_cmp(&from(*b))
        })
        .expect("an edge drawn through points")
}

fn the_middle_on_screen(app: &driver::App, edge: &DrawnEdge) -> egui::Pos2 {
    let sum = edge
        .points
        .iter()
        .map(|point| driver::world_point_on_screen(app, *point).to_vec2())
        .fold(egui::Vec2::ZERO, |sum, at| sum + at);
    (sum / edge.points.len() as f32).to_pos2()
}

#[test]
fn a_choice_the_kernel_declines_lays_nothing_says_why_in_the_plural_and_keeps_every_edge_chosen() {
    let test = "a_declined_choice_of_two_rims_says_why";
    let mut app = a_disc(test);
    let disc = the_part(test, "Disque");
    let volume = disc.body().volume();
    let mut rims = disc.body().drawn_edges();
    assert_eq!(rims.len(), 2, "a disc has two rims: {rims:?}");
    rims.sort_by(|a, b| a.points[0].x.total_cmp(&b.points[0].x));
    let [far, near] = [&rims[0], &rims[1]];
    let on_the_near_rim = furthest_on_screen_from(&app, near, the_middle_on_screen(&app, far));
    let on_the_far_rim = furthest_on_screen_from(&app, far, the_middle_on_screen(&app, near));
    let both_rims = |chosen: &[Vec<DVec3>]| {
        chosen.len() == 2
            && [near, far].iter().all(|rim| {
                chosen.iter().any(|points| {
                    points
                        .iter()
                        .all(|point| (point.x - rim.points[0].x).abs() < 1e-6)
                })
            })
    };

    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, "Chanfrein");
    driver::open_the_history(&mut app);
    let before = driver::steps(&app);
    driver::click_at_world_point(&mut app, on_the_near_rim);
    driver::click_at_world_point_with(&mut app, on_the_far_rim, egui::Modifiers::COMMAND);
    assert!(
        both_rims(&chosen(&app)),
        "the fixture chooses both rims, choosing {:?}",
        chosen(&app),
    );
    driver::click(&mut app, "Appliquer");

    assert_eq!(
        driver::steps(&app),
        before,
        "a line was added for nothing done"
    );
    let after = the_part(test, "Disque").body().volume();
    assert!(
        (after - volume).abs() <= 1e-9 * volume,
        "the disc holds {after}, where it held {volume}",
    );
    assert!(
        driver::reads(&app, "Distance :"),
        "the panel closed on a chamfer the part could not take, showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        driver::shows_or_says(&app, "ces arêtes") && !driver::shows_or_says(&app, "cette arête"),
        "two rims declined, the panel does not say why speaking of « ces arêtes », showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        both_rims(&chosen(&app)),
        "the choice declined no longer holds both rims, but {:?}",
        chosen(&app),
    );
}

#[test]
fn turning_the_chamfer_into_a_fillet_keeps_every_edge_chosen() {
    let (mut app, block) = opened_on("turning_the_chamfer_into_a_fillet", "Chanfrein");
    let three = [block.near_top(), block.near_left(), block.left_top()];
    driver::click_at_world_point(&mut app, middle(three[0]));
    for edge in &three[1..] {
        driver::click_at_world_point_with(&mut app, middle(*edge), egui::Modifiers::COMMAND);
    }
    assert!(
        are_the_edges(&chosen(&app), &three),
        "the fixture chooses three edges with « Chanfrein », choosing {:?}",
        chosen(&app),
    );

    driver::click_exactly(&mut app, "Congé");

    assert!(
        driver::reads(&app, "Rayon :") && !driver::reads(&app, "Distance :"),
        "« Congé » clicked did not turn the chamfer's panel into the fillet's, showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        are_the_edges(&chosen(&app), &three),
        "the chamfer turned into a fillet, {:?} chosen where three edges were",
        chosen(&app),
    );
    assert!(
        driver::reads(&app, "3 arêtes"),
        "the fillet's panel does not count the three edges, showing {:?}",
        driver::on_screen(&app),
    );
}
