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
//!   `apply_on_the_four_top_edges_lays_one_step_and_the_body_the_kernel_gives_for_all_four`
//! - A choice the kernel declines lays nothing, shows the reason in the panel
//!   — in the plural with several edges — and keeps every edge chosen —
//!   `a_choice_the_kernel_declines_lays_nothing_says_why_in_the_plural_and_keeps_every_edge_chosen`
//! - A `.caopart` holding a blend step written with the single `edge` field
//!   opens and replays to the same body as before — no test: held in
//!   `crates/part/tests/a_blend_step_of_one_edge_still_opens.rs`, where the
//!   step's serde lives and a file frozen before this change needs no window,
//!   by `a_part_whose_blend_step_names_a_single_edge_opens_and_replays_to_the_same_body`
//! - Under the section view, Cmd + click on an edge on the hidden side adds
//!   nothing —
//!   `under_the_section_view_cmd_click_on_an_edge_on_the_hidden_side_adds_nothing`
//! - Turning « Chanfrein » into « Congé » keeps every edge chosen —
//!   `turning_the_chamfer_into_a_fillet_keeps_every_edge_chosen`

mod driver;

use glam::DVec3;

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
