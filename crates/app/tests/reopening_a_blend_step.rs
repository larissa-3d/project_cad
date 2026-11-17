//! A chamfer or a fillet step reopened from its history line or its alert,
//! its edge panel filled with the step, the part drawn as it stood before it,
//! and the step changed in place or left as it was.
//!
//! Closes #988 — the gestures, the panel and the view are here; what the part
//! records and replays is in `crates/part/tests/changing_a_blend_step.rs`.
//! - « Modifier le congé » (resp. « Modifier le chanfrein ») in the menu of a
//!   blend step's history line opens the edge panel on that step's kind, its
//!   size as written (a formula stays one) and its edges, all chosen —
//!   `modify_in_a_blend_line_s_menu_opens_the_edge_panel_on_its_kind_its_size_as_written_and_its_edges`
//! - « Modifier » on the alert of a blend step that no longer rebuilds does the
//!   same; « Montrer » still offers nothing for it —
//!   `modify_on_the_alert_of_a_blend_step_not_rebuilt_opens_the_panel_on_it_and_show_offers_nothing`
//! - While a blend step is reopened, the body drawn is the part as it stood
//!   just before that step, and the step's edges are the ones lit —
//!   `while_a_blend_step_is_reopened_the_body_drawn_is_the_part_before_it_and_its_edges_are_lit`
//! - « Appliquer » on a reopened step with a new size, kind or set of edges
//!   records one `EditMatter` change; the body afterwards is the one the part
//!   gives with the step laid that way, and the history row reads the new
//!   values — the gesture and the row, by
//!   `apply_on_a_reopened_step_lays_no_new_step_and_its_history_line_reads_the_new_values`;
//!   the change recorded and the body, in the part's file
//! - « Annuler » on a reopened step records nothing and the body drawn is the
//!   whole part again —
//!   `cancel_on_a_reopened_step_records_nothing_and_draws_the_whole_part_again`
//! - A reopened step whose new choice the kernel declines records nothing,
//!   keeps the panel open with the reason, and keeps the edges chosen — the
//!   panel's half, by
//!   `a_reopened_step_whose_new_choice_the_kernel_declines_keeps_the_panel_open_with_the_reason_and_the_edges`;
//!   nothing recorded, in the part's file
//! - A reopened step whose edges are all taken out has « Appliquer » greyed —
//!   `a_reopened_step_with_every_edge_taken_out_greys_apply`
//! - Undoing or rewinding the history while a step is reopened closes the
//!   panel — `undoing_or_rewinding_while_a_step_is_reopened_closes_the_panel`

mod driver;

use glam::DVec3;

/// The four edges of the top of `driver::a_block`'s block, as the part held
/// before any of them was blended: its top faces +X.
fn the_four_top_edges(test: &str) -> [[DVec3; 2]; 4] {
    let (low, high) = the_part(test)
        .body()
        .bounds()
        .expect("the block holds matter");
    let corner = |y: f64, z: f64| DVec3::new(high.x, y, z);
    [
        [corner(low.y, high.z), corner(high.y, high.z)],
        [corner(high.y, high.z), corner(high.y, low.z)],
        [corner(high.y, low.z), corner(low.y, low.z)],
        [corner(low.y, low.z), corner(low.y, high.z)],
    ]
}

fn the_part(test: &str) -> cao_part::PartDocument {
    let path = driver::the_part_s_file(test, "Bloc");
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

/// One kind of blend step, as the app names it everywhere.
struct Kind {
    tool: &'static str,
    value: &'static str,
    other_value: &'static str,
    line: &'static str,
    edit: &'static str,
}

const FILLET: Kind = Kind {
    tool: "Congé",
    value: "Rayon :",
    other_value: "Distance :",
    line: "4 arêtes arrondies",
    edit: "Modifier le congé",
};

const CHAMFER: Kind = Kind {
    tool: "Chanfrein",
    value: "Distance :",
    other_value: "Rayon :",
    line: "4 arêtes chanfreinées",
    edit: "Modifier le chanfrein",
};

/// `driver::a_block`, a variable `r` made at 1, then the four edges of the
/// block's top blended at « r * 2 » with `kind`'s tool, the history open: the
/// app, the four edges, and the history line of the step.
fn a_block_blended_at_twice_r(test: &str, kind: &Kind) -> (driver::App, [[DVec3; 2]; 4], String) {
    let mut app = driver::a_block(test);
    let top = the_four_top_edges(test);
    driver::click_the_button(&mut app, "Variables");
    driver::type_into(&mut app, "nom", "r");
    driver::type_into(&mut app, "formule", "1");
    driver::click_exactly(&mut app, "Ajouter");
    driver::click_the_button(&mut app, "Variables");

    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, kind.tool);
    driver::click_at_world_point(&mut app, middle(top[0]));
    for edge in &top[1..] {
        driver::click_at_world_point_with(&mut app, middle(*edge), egui::Modifiers::COMMAND);
    }
    assert!(
        are_the_edges(&app.state().the_edges_chosen(), &top),
        "« {} »: the fixture chooses the four top edges, choosing {:?}",
        kind.tool,
        app.state().the_edges_chosen(),
    );
    driver::type_beside(&mut app, kind.value, "r * 2");
    driver::click(&mut app, "Appliquer");
    driver::open_the_history(&mut app);
    let line = driver::steps(&app)
        .into_iter()
        .find(|line| line.ends_with(kind.line))
        .unwrap_or_else(|| {
            panic!(
                "« {} » of the four top edges laid no line « {} »: {:?}",
                kind.tool,
                kind.line,
                driver::steps(&app),
            )
        });
    assert!(
        !driver::reads(&app, kind.value),
        "« {} » applied left its panel open",
        kind.tool,
    );
    (app, top, line)
}

#[test]
fn modify_in_a_blend_line_s_menu_opens_the_edge_panel_on_its_kind_its_size_as_written_and_its_edges()
 {
    for kind in [FILLET, CHAMFER] {
        let test = format!("a_blend_line_s_menu_reopens_{}", kind.tool.to_lowercase());
        let (mut app, top, line) = a_block_blended_at_twice_r(&test, &kind);

        driver::right_click(&mut app, &line);
        assert!(
            driver::a_button_reads(&app, kind.edit),
            "a right-click on « {line} » offers no « {} », showing {:?}",
            kind.edit,
            driver::on_screen(&app),
        );
        driver::click_the_button(&mut app, kind.edit);

        let mut wrong = Vec::new();
        if !driver::reads(&app, kind.value) || driver::reads(&app, kind.other_value) {
            wrong.push(format!(
                "the panel is not the « {} »'s, reading « {} » {} and « {} » {}",
                kind.tool,
                kind.value,
                driver::reads(&app, kind.value),
                kind.other_value,
                driver::reads(&app, kind.other_value),
            ));
        } else if driver::field_beside(&app, kind.value) != "r * 2" {
            wrong.push(format!(
                "the size reads « {} », not « r * 2 » as written",
                driver::field_beside(&app, kind.value),
            ));
        }
        if !driver::reads(&app, "4 arêtes") {
            wrong.push("the panel does not count « 4 arêtes »".to_string());
        }
        if !are_the_edges(&app.state().the_edges_chosen(), &top) {
            wrong.push(format!(
                "{:?} are chosen, not the four top edges",
                app.state().the_edges_chosen(),
            ));
        }
        assert!(
            wrong.is_empty(),
            "« {} » on « {line} » does not open the panel on the step:\n  {}\n showing {:?}",
            kind.edit,
            wrong.join("\n  "),
            driver::on_screen(&app),
        );
    }
}
