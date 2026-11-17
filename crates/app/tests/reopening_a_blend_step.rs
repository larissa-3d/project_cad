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
//! - decided while the work went on, beyond the issue's eleven: « Appliquer »
//!   on a reopened step with nothing changed records nothing, so that Cmd+Z
//!   then takes the step itself back rather than a change that changed nothing
//!   — `apply_on_a_reopened_step_with_nothing_changed_records_nothing_and_cmd_z_takes_the_step_back`

mod driver;

use cao_part::history::{BlendShape, ExtrusionMode, Operation, PointRef};
use cao_sketch::WorkPlane;
use egui_kittest::kittest::{NodeT, Queryable};
use glam::{DVec2, DVec3};

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

/// `a_block_blended_at_twice_r`, the step then reopened from its line's menu.
fn reopened(test: &str, kind: &Kind) -> (driver::App, [[DVec3; 2]; 4], String) {
    let (mut app, top, line) = a_block_blended_at_twice_r(test, kind);
    driver::right_click(&mut app, &line);
    driver::click_the_button(&mut app, kind.edit);
    assert!(
        driver::reads(&app, kind.value),
        "« {} » on « {line} » opened no panel, showing {:?}",
        kind.edit,
        driver::on_screen(&app),
    );
    (app, top, line)
}

fn hover_at_world_point(app: &mut driver::App, point: DVec3) {
    driver::hold_the_canvas(app);
    let at = driver::world_point_on_screen(app, point);
    app.hover_at(at);
    app.run();
}

/// Where along X the view bar reads the point of the part under the pointer:
/// « X 10,00 Y -3,00 Z 30,10 mm ».
fn x_read_under_the_pointer(app: &driver::App) -> Option<f64> {
    let reading = driver::the_bar_s_coordinates(app);
    let x = reading.strip_prefix("X ")?.split(" Y ").next()?;
    x.replace(',', ".").parse().ok()
}

/// On the block's top, which faces +X, half a millimetre in from the middle
/// of its edge `top[0]`: matter of the block whole, which a fillet of 2 on
/// that edge takes away.
fn just_inside_the_first_top_edge(top: &[[DVec3; 2]; 4]) -> DVec3 {
    let across = (middle(top[2]) - middle(top[0])).normalize();
    middle(top[0]) + across * 0.5
}

/// How far from the top along X the matter the pointer meets may read and
/// still be the top: two decimals in the bar, and a pixel of aim.
const ON_THE_TOP: f64 = 0.1;

#[test]
fn while_a_blend_step_is_reopened_the_body_drawn_is_the_part_before_it_and_its_edges_are_lit() {
    let test = "while_reopened_the_part_before_the_step_is_drawn";
    let (mut app, top, _) = reopened(test, &FILLET);
    let top_x = top[0][0].x;
    let mut wrong = Vec::new();

    if !are_the_edges(&app.state().the_edges_lit(), &top) {
        wrong.push(format!(
            "{:?} are lit, not the four top edges of the block before the step",
            app.state().the_edges_lit(),
        ));
    }

    hover_at_world_point(&mut app, just_inside_the_first_top_edge(&top));
    match x_read_under_the_pointer(&app) {
        Some(x) if (x - top_x).abs() <= ON_THE_TOP => {}
        _ => wrong.push(format!(
            "half a millimetre in from a top edge, the pointer meets « {} », where the block \
             before the fillet stands with its top at X {top_x}",
            driver::the_bar_s_coordinates(&app),
        )),
    }

    let low_x = the_part(test)
        .body()
        .bounds()
        .expect("the block holds matter")
        .0
        .x;
    let along_the_left_top = [
        DVec3::new(top_x, top[0][0].y, top[0][0].z),
        DVec3::new(low_x, top[0][0].y, top[0][0].z),
    ];
    driver::click_at_world_point_with(
        &mut app,
        middle(along_the_left_top),
        egui::Modifiers::COMMAND,
    );
    let mut five = top.to_vec();
    five.push(along_the_left_top);
    if !are_the_edges(&app.state().the_edges_chosen(), &five) {
        wrong.push(format!(
            "Cmd + click on the edge running from the top's corner along X chose {:?}, not the \
             four top edges and that edge whole, as the block before the step has it",
            app.state().the_edges_chosen(),
        ));
    }

    assert!(
        wrong.is_empty(),
        "a fillet of the four top edges, reopened from its line, does not show the block as it \
         stood before it:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn apply_on_a_reopened_step_lays_no_new_step_and_its_history_line_reads_the_new_values() {
    for change in ["the size typed 3", "the far top edge taken out"] {
        let test = format!(
            "apply_on_a_reopened_step_{}",
            change.replace(' ', "_").to_lowercase()
        );
        let (mut app, top, line) = reopened(&test, &FILLET);
        let lines = driver::steps(&app);
        let detail = driver::detail_of(&app, &line);
        let rank = line.split_once(". ").expect("a numbered line").0;
        let (expected_line, expected_detail) = match change {
            "the size typed 3" => {
                driver::type_beside(&mut app, FILLET.value, "3");
                (line.clone(), Some("3 mm".to_string()))
            }
            _ => {
                driver::click_at_world_point_with(
                    &mut app,
                    middle(top[2]),
                    egui::Modifiers::COMMAND,
                );
                assert!(
                    are_the_edges(&app.state().the_edges_chosen(), &[top[0], top[1], top[3]],),
                    "the fixture takes the far top edge out, leaving {:?} chosen",
                    app.state().the_edges_chosen(),
                );
                (format!("{rank}. 3 arêtes arrondies"), detail.clone())
            }
        };

        driver::click(&mut app, "Appliquer");

        let mut wrong = Vec::new();
        let now = driver::steps(&app);
        if now.len() != lines.len() {
            wrong.push(format!("the history reads {now:?} where it read {lines:?}"));
        }
        if !now.contains(&expected_line) {
            wrong.push(format!("no line reads « {expected_line} »: {now:?}"));
        } else if driver::detail_of(&app, &expected_line) != expected_detail {
            wrong.push(format!(
                "« {expected_line} » reads {:?} under it, where {expected_detail:?} is wanted",
                driver::detail_of(&app, &expected_line),
            ));
        }
        if now.iter().any(|line| line.contains("modifiée")) {
            wrong.push(format!(
                "a line says the step was changed in other words: {now:?}"
            ));
        }
        if driver::reads(&app, FILLET.value) {
            wrong.push("the panel is still open".to_string());
        }
        hover_at_world_point(&mut app, just_inside_the_first_top_edge(&top));
        if x_read_under_the_pointer(&app).is_some_and(|x| (x - top[0][0].x).abs() <= ON_THE_TOP) {
            wrong.push(format!(
                "half a millimetre in from a top edge still rounded, the pointer meets « {} », \
                 the block's sharp top: the part before the step is still drawn",
                driver::the_bar_s_coordinates(&app),
            ));
        }

        assert!(
            wrong.is_empty(),
            "« Appliquer » on the fillet of the four top edges reopened, {change}:\n  {}",
            wrong.join("\n  "),
        );
    }
}

/// Whether the pointer, half a millimetre in from the first top edge, meets
/// the fillet laid on it rather than the block's sharp top: the whole part
/// drawn. What it reads otherwise.
fn the_fillet_drawn(app: &mut driver::App, top: &[[DVec3; 2]; 4]) -> Result<(), String> {
    hover_at_world_point(app, just_inside_the_first_top_edge(top));
    match x_read_under_the_pointer(app) {
        Some(x) if (x - top[0][0].x).abs() > ON_THE_TOP => Ok(()),
        _ => Err(format!(
            "half a millimetre in from a top edge, the pointer meets « {} », not the fillet laid \
             on it",
            driver::the_bar_s_coordinates(app),
        )),
    }
}

fn the_lines_and_their_details(app: &driver::App) -> Vec<(String, Option<String>)> {
    driver::steps(app)
        .into_iter()
        .map(|line| {
            let detail = driver::detail_of(app, &line);
            (line, detail)
        })
        .collect()
}

fn changes_in(part: &cao_part::PartDocument) -> usize {
    part.history
        .operations()
        .iter()
        .filter(|operation| matches!(operation, Operation::EditMatter { .. }))
        .count()
}

#[test]
fn a_reopened_step_with_every_edge_taken_out_greys_apply() {
    let test = "a_reopened_step_with_every_edge_taken_out";
    let (mut app, top, _) = reopened(test, &FILLET);

    for edge in top {
        driver::click_at_world_point_with(&mut app, middle(edge), egui::Modifiers::SHIFT);
    }

    assert!(
        app.state().the_edges_chosen().is_empty(),
        "Shift + click on each of the four edges of the step leaves {:?} chosen",
        app.state().the_edges_chosen(),
    );
    assert!(
        !driver::is_enabled(&app, "Appliquer"),
        "« Appliquer » offered on a reopened step with every edge taken out",
    );
}

#[test]
fn cancel_on_a_reopened_step_records_nothing_and_draws_the_whole_part_again() {
    let test = "cancel_on_a_reopened_step";
    let (mut app, top, _) = reopened(test, &FILLET);
    let lines = the_lines_and_their_details(&app);
    let held = the_part(test).history;
    driver::type_beside(&mut app, FILLET.value, "3");
    driver::click_at_world_point_with(&mut app, middle(top[2]), egui::Modifiers::COMMAND);

    driver::click_the_button(&mut app, "Annuler");

    let mut wrong = Vec::new();
    if driver::reads(&app, FILLET.value) {
        wrong.push("the panel is still open".to_string());
    }
    if the_part(test).history != held {
        wrong.push(format!(
            "the history moved: {} changes are recorded",
            changes_in(&the_part(test)),
        ));
    }
    if the_lines_and_their_details(&app) != lines {
        wrong.push(format!(
            "the history reads {:?} where it read {lines:?}",
            the_lines_and_their_details(&app),
        ));
    }
    wrong.extend(the_fillet_drawn(&mut app, &top).err());

    assert!(
        wrong.is_empty(),
        "« Annuler » on the fillet of the four top edges reopened, its size typed 3 and an edge \
         taken out:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn apply_on_a_reopened_step_with_nothing_changed_records_nothing_and_cmd_z_takes_the_step_back() {
    let test = "apply_on_a_reopened_step_with_nothing_changed";
    let (mut app, top, line) = reopened(test, &FILLET);
    let lines = the_lines_and_their_details(&app);

    driver::click(&mut app, "Appliquer");

    let mut wrong = Vec::new();
    if driver::reads(&app, FILLET.value) {
        wrong.push("the panel is still open".to_string());
    }
    if changes_in(&the_part(test)) != 0 {
        wrong.push(format!(
            "{} changes are recorded for nothing changed",
            changes_in(&the_part(test)),
        ));
    }
    if the_lines_and_their_details(&app) != lines {
        wrong.push(format!(
            "the history reads {:?} where it read {lines:?}",
            the_lines_and_their_details(&app),
        ));
    }
    driver::press_with_command(&mut app, egui::Key::Z);
    hover_at_world_point(&mut app, just_inside_the_first_top_edge(&top));
    if !x_read_under_the_pointer(&app).is_some_and(|x| (x - top[0][0].x).abs() <= ON_THE_TOP) {
        wrong.push(format!(
            "after Cmd+Z, half a millimetre in from a top edge, the pointer meets « {} », where \
             « {line} » taken back leaves the block's sharp top",
            driver::the_bar_s_coordinates(&app),
        ));
    }

    assert!(
        wrong.is_empty(),
        "« Appliquer » on the fillet of the four top edges reopened, nothing changed:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn undoing_or_rewinding_while_a_step_is_reopened_closes_the_panel() {
    let mut failures = Vec::new();
    for how in ["undone", "rewound"] {
        let test = format!("{how}_while_a_step_is_reopened");
        let (mut app, top, line) = a_block_blended_at_twice_r(&test, &FILLET);
        driver::click_the_button(&mut app, "Variables");
        driver::type_into(&mut app, "nom", "s");
        driver::type_into(&mut app, "formule", "1");
        driver::click_exactly(&mut app, "Ajouter");
        driver::click_the_button(&mut app, "Variables");
        driver::open_the_history(&mut app);
        driver::right_click(&mut app, &line);
        driver::click_the_button(&mut app, FILLET.edit);
        assert!(
            driver::reads(&app, FILLET.value),
            "the fixture reopens « {line} », showing {:?}",
            driver::on_screen(&app),
        );

        match how {
            "undone" => driver::press_with_command(&mut app, egui::Key::Z),
            _ => driver::click_the_button(&mut app, &line),
        }

        let mut wrong = Vec::new();
        if driver::reads(&app, FILLET.value) {
            wrong.push("the panel is still open".to_string());
        }
        wrong.extend(the_fillet_drawn(&mut app, &top).err());
        if !wrong.is_empty() {
            failures.push(format!(
                "the variable laid after « {line} » {how}:\n    {}",
                wrong.join("\n    ")
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "the history moved while a step is reopened:\n  {}",
        failures.join("\n  "),
    );
}

/// The edge the drawing of `part` holds whose every point satisfies `on`.
fn the_drawn_edge(
    part: &cao_part::PartDocument,
    on: impl Fn(DVec3) -> bool,
) -> cao_solid::EdgeName {
    part.body()
        .drawn_edges()
        .into_iter()
        .find(|edge| edge.points.iter().all(|point| on(*point)))
        .expect("the edge is drawn")
        .name
}

fn rectangle(part: &mut cao_part::PartDocument, sketch: usize, corner: DVec2, opposite: DVec2) {
    part.apply(Operation::AddRectangle {
        sketch,
        corner: PointRef::New(corner),
        opposite: PointRef::New(opposite),
        construction: false,
    });
}

/// Opens in `app`, standing on the start menu, `part` written where the
/// application keeps the part « Bloc » of `test`.
fn open_in_the_app(app: &mut driver::App, test: &str, part: &cao_part::PartDocument) {
    part.save(
        &cao_app::adapters::files::DiskFiles,
        &driver::the_part_s_file(test, "Bloc"),
        "2026-11-17T10:00:00Z".parse().expect("a date"),
    )
    .expect("the part is written");
    app.query_all_by_label("Bloc")
        .last()
        .expect("the start menu lists the part")
        .click();
    app.run();
    driver::hold_the_canvas(app);
    assert!(
        !driver::a_sketch_is_open(app),
        "the part opened on a sketch being edited, showing {:?}",
        driver::on_screen(app),
    );
}

/// The centre of the disc of `a_block_chamfered_beside_a_disc`, its radius
/// and its height.
const THE_DISC: (DVec2, f64, f64) = (DVec2::new(45.0, 5.0), 5.0, 10.0);

/// A block 30 by 10 by 20 raised from XY, a disc of `THE_DISC` raised beside
/// it from the same sketch, short enough a history for every line to stand on
/// screen, then the edge at the top of the block's front chamfered 2: the part
/// `changing_a_blend_step.rs` declines a change of, with one edge to its
/// chamfer.
fn a_block_chamfered_beside_a_disc() -> cao_part::PartDocument {
    let mut part =
        cao_part::PartDocument::new("Bloc", "2026-11-17T09:00:00Z".parse().expect("a date"));
    let (centre, radius, height) = THE_DISC;
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    rectangle(&mut part, 0, DVec2::ZERO, DVec2::new(30.0, 10.0));
    part.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(centre),
        radius,
        rim: Vec::new(),
        construction: false,
    });
    for (inside, distance) in [(DVec2::new(15.0, 5.0), 20.0), (centre, height)] {
        part.apply(Operation::Extrude {
            sketch: 0,
            areas: part.areas_at(0, &[inside]),
            distance: distance.into(),
            mode: ExtrusionMode::Add,
        });
    }
    let top_of_the_front = the_drawn_edge(&part, |point| {
        point.y.abs() < 1e-9 && (point.z - 20.0).abs() < 1e-9
    });
    let outcome = part.apply(Operation::Blend {
        edges: vec![top_of_the_front],
        shape: BlendShape::Chamfer,
        size: 2.0.into(),
    });
    assert_eq!(outcome, None, "the fixture's chamfer is built");
    part
}

/// Of the disc's top rim, the point standing furthest on screen from the
/// middle of its foot: on the far side of its top, in sight from above.
fn on_the_disc_s_top_rim(app: &driver::App) -> DVec3 {
    let (centre, radius, height) = THE_DISC;
    let around = |z: f64| {
        (0..64).map(move |step| {
            let angle = std::f64::consts::TAU * f64::from(step) / 64.0;
            DVec3::new(
                centre.x + radius * angle.cos(),
                centre.y + radius * angle.sin(),
                z,
            )
        })
    };
    let foot = driver::world_point_on_screen(app, centre.extend(0.0));
    around(height)
        .max_by(|a, b| {
            let from = |point: DVec3| driver::world_point_on_screen(app, point).distance(foot);
            from(*a).total_cmp(&from(*b))
        })
        .expect("points around the rim")
}

const NOT_YET_A_CHAMFER: &str =
    "Ce chanfrein n'est pas encore possible sur cette arête : la pièce reste comme elle était.";

#[test]
fn a_reopened_step_whose_new_choice_the_kernel_declines_keeps_the_panel_open_with_the_reason_and_the_edges()
 {
    let test = "a_reopened_step_the_kernel_declines";
    let mut app = driver::a_block(test);
    driver::the_part_put_away(&mut app, test, "Bloc");
    open_in_the_app(&mut app, test, &a_block_chamfered_beside_a_disc());
    driver::open_the_history(&mut app);
    let line = driver::steps(&app)
        .into_iter()
        .find(|line| line.ends_with("Arête chanfreinée"))
        .unwrap_or_else(|| panic!("no line of the chamfer: {:?}", driver::steps(&app)));
    let lines = the_lines_and_their_details(&app);
    let held = the_part(test).history;
    driver::right_click(&mut app, &line);
    assert!(
        driver::a_button_reads(&app, CHAMFER.edit),
        "a right-click on « {line} » offers no « {} », showing {:?}",
        CHAMFER.edit,
        driver::on_screen(&app),
    );
    driver::click_the_button(&mut app, CHAMFER.edit);
    let rim = on_the_disc_s_top_rim(&app);
    driver::click_at_world_point(&mut app, rim);
    let (centre, radius, height) = THE_DISC;
    let on_the_rim = |chosen: &[Vec<DVec3>]| {
        chosen.len() == 1
            && chosen[0].iter().all(|point| {
                (point.z - height).abs() < 1e-6
                    && (point.truncate().distance(centre) - radius).abs() < 1e-6
            })
    };
    assert!(
        on_the_rim(&app.state().the_edges_chosen()),
        "the fixture chooses the disc's top rim in place of the chamfer's edge, choosing {:?}",
        app.state().the_edges_chosen(),
    );

    driver::click(&mut app, "Appliquer");

    let mut wrong = Vec::new();
    if !driver::reads(&app, CHAMFER.value) {
        wrong.push("the panel closed".to_string());
    }
    if !driver::reads(&app, NOT_YET_A_CHAMFER) {
        wrong.push(format!("the panel does not read « {NOT_YET_A_CHAMFER} »"));
    }
    if !on_the_rim(&app.state().the_edges_chosen()) {
        wrong.push(format!(
            "{:?} are chosen, not the disc's top rim",
            app.state().the_edges_chosen(),
        ));
    }
    if the_part(test).history != held {
        wrong.push(format!(
            "the history moved: {} changes are recorded",
            changes_in(&the_part(test)),
        ));
    }
    if the_lines_and_their_details(&app) != lines {
        wrong.push(format!(
            "the history reads {:?} where it read {lines:?}",
            the_lines_and_their_details(&app),
        ));
    }
    assert!(
        wrong.is_empty(),
        "the chamfer of the block's top front, reopened and moved to the rim of a disc the \
         kernel declines:\n  {}\n showing {:?}",
        wrong.join("\n  "),
        driver::on_screen(&app),
    );
}

/// A block 30 by 10 by 20 notched at its foot across its back, the edge at
/// the foot of its front chamfered 2, then the notch moved across the front:
/// the chamfer's edge is parted in two, and the step not built.
fn a_block_whose_chamfer_lost_its_edge() -> cao_part::PartDocument {
    let mut part =
        cao_part::PartDocument::new("Bloc", "2026-11-17T09:00:00Z".parse().expect("a date"));
    let on_xy = Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    };
    part.apply(on_xy.clone());
    rectangle(&mut part, 0, DVec2::ZERO, DVec2::new(30.0, 10.0));
    part.apply(Operation::Extrude {
        sketch: 0,
        areas: part.areas_at(0, &[DVec2::new(15.0, 5.0)]),
        distance: 20.0.into(),
        mode: ExtrusionMode::Add,
    });
    part.apply(on_xy);
    rectangle(&mut part, 1, DVec2::new(10.0, 7.0), DVec2::new(20.0, 13.0));
    rectangle(&mut part, 1, DVec2::new(10.0, -3.0), DVec2::new(20.0, 3.0));
    let notch = |part: &cao_part::PartDocument, at: DVec2| Operation::Extrude {
        sketch: 1,
        areas: part.areas_at(1, &[at]),
        distance: 5.0.into(),
        mode: ExtrusionMode::Cut,
    };
    part.apply(notch(&part, DVec2::new(15.0, 10.0)));
    let notched = part.history.steps().last().expect("the notch").operations()[0];
    let foot_of_the_front =
        the_drawn_edge(&part, |point| point.y.abs() < 1e-9 && point.z.abs() < 1e-9);
    let outcome = part.apply(Operation::Blend {
        edges: vec![foot_of_the_front],
        shape: BlendShape::Chamfer,
        size: 2.0.into(),
    });
    assert_eq!(outcome, None, "the fixture's chamfer is built");
    part.apply(Operation::EditMatter {
        raised_by: notched,
        to: Box::new(notch(&part, DVec2::new(15.0, 0.0))),
    });
    part
}

#[test]
fn modify_on_the_alert_of_a_blend_step_not_rebuilt_opens_the_panel_on_it_and_show_offers_nothing() {
    let test = "modify_on_the_alert_of_a_blend_step";
    let mut app = driver::a_block(test);
    driver::the_part_put_away(&mut app, test, "Bloc");
    open_in_the_app(&mut app, test, &a_block_whose_chamfer_lost_its_edge());
    driver::click_the_button(&mut app, "1 alerte");
    assert!(
        driver::reads(&app, "Arête chanfreinée · non construite"),
        "the fixture's alert is the chamfer's, showing {:?}",
        driver::on_screen(&app),
    );
    let buttons = |app: &driver::App, label: &str| {
        app.query_all_by_label(label)
            .filter(|node| node.accesskit_node().role() == egui::accesskit::Role::Button)
            .count()
    };

    let mut wrong = Vec::new();
    if buttons(&app, "Montrer") != 0 {
        wrong.push("the alert offers « Montrer »".to_string());
    }
    if buttons(&app, "Modifier") == 1 {
        driver::click_the_button(&mut app, "Modifier");
        if !driver::reads(&app, CHAMFER.value) || driver::reads(&app, CHAMFER.other_value) {
            wrong.push("« Modifier » opened no chamfer's panel".to_string());
        } else if driver::field_beside(&app, CHAMFER.value) != "2" {
            wrong.push(format!(
                "the panel reads « {} », not the chamfer's 2",
                driver::field_beside(&app, CHAMFER.value),
            ));
        }
    } else {
        wrong.push(format!(
            "the alert offers {} « Modifier », not one",
            buttons(&app, "Modifier"),
        ));
    }

    assert!(
        wrong.is_empty(),
        "the alert of a chamfer whose edge is lost:\n  {}\n showing {:?}",
        wrong.join("\n  "),
        driver::on_screen(&app),
    );
}
