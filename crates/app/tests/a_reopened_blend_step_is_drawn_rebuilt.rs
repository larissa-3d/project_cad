//! A chamfer or a fillet step reopened is drawn rebuilt on the part before it
//! at every change of its panel, and the panel says what the kernel made of
//! the change.
//!
//! Closes #989.
//! - While a blend step is reopened, the body drawn is the part before the
//!   step with the step laid at the size, kind and edges the panel holds,
//!   redrawn when any of the three changes —
//!   `while_a_step_is_reopened_the_body_drawn_is_the_step_laid_as_the_panel_holds_it_redrawn_at_each_change`
//! - A size the kernel reduces shows the asked and the taken values in the
//!   panel, in the words of the history line —
//!   `a_size_the_kernel_reduces_shows_the_asked_and_the_taken_values_in_the_panel`
//! - A choice the kernel declines shows its reason in the panel as soon as it
//!   is made, draws the part before the step, and greys « Appliquer » —
//!   `a_choice_the_kernel_declines_says_why_as_it_is_made_draws_the_part_before_the_step_and_greys_apply`
//! - A size that does not read keeps the last body drawn and says what is
//!   wrong with it —
//!   `a_size_that_does_not_read_keeps_the_last_body_drawn_and_says_what_is_wrong`
//! - The step is rebuilt once per change of the panel, never at a frame where
//!   nothing in it changed; the later steps are not replayed before
//!   « Appliquer » —
//!   `the_step_is_rebuilt_once_per_change_and_the_later_steps_wait_for_apply`
//! - decided on the way: the step is laid on the part before it kept from the
//!   reopening, every volume opaque, so each change lays one operation — the
//!   count, in the test above; and a volume before the step made see-through
//!   changes nothing the panel says, which is what « Appliquer » meets —
//!   `with_a_volume_before_the_step_see_through_the_panel_says_what_apply_will_meet`
//! - decided on the way: a size that reads but is not above zero keeps the
//!   last body drawn and says something beside the field —
//!   `a_size_not_above_zero_keeps_the_last_body_drawn_and_says_something_beside_the_field`
//! - found in review: another step reopened after a change cancelled draws
//!   its own step, never the preview of the one before —
//!   `another_step_reopened_after_a_change_cancelled_draws_its_own_step_not_the_last_preview`

mod driver;

use cao_part::history::{BlendShape, ExtrusionMode, Operation, PointRef};
use cao_sketch::WorkPlane;
use egui_kittest::kittest::Queryable;
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

/// `driver::a_block`, a variable `r` made at 1, the four edges of the block's
/// top rounded at « r * 2 », then that fillet reopened from its history line:
/// the app, the four edges and the line.
fn a_fillet_at_twice_r_reopened(test: &str) -> (driver::App, [[DVec3; 2]; 4], String) {
    let (mut app, top) = a_fillet_at_twice_r(test);
    let line = the_fillet_reopened(&mut app);
    (app, top, line)
}

/// `driver::a_block`, a variable `r` made at 1, then the four edges of the
/// block's top rounded at « r * 2 »: the app and the four edges.
fn a_fillet_at_twice_r(test: &str) -> (driver::App, [[DVec3; 2]; 4]) {
    let mut app = driver::a_block(test);
    let top = the_four_top_edges(test);
    driver::click_the_button(&mut app, "Variables");
    driver::type_into(&mut app, "nom", "r");
    driver::type_into(&mut app, "formule", "1");
    driver::click_exactly(&mut app, "Ajouter");
    driver::click_the_button(&mut app, "Variables");

    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, "Congé");
    driver::click_at_world_point(&mut app, middle(top[0]));
    for edge in &top[1..] {
        driver::click_at_world_point_with(&mut app, middle(*edge), egui::Modifiers::COMMAND);
    }
    assert!(
        are_the_edges(&app.state().the_edges_chosen(), &top),
        "the fixture chooses the four top edges, choosing {:?}",
        app.state().the_edges_chosen(),
    );
    driver::type_beside(&mut app, "Rayon :", "r * 2");
    driver::click(&mut app, "Appliquer");
    (app, top)
}

/// Reopens the fillet of `a_fillet_at_twice_r` from its history line: the line.
fn the_fillet_reopened(app: &mut driver::App) -> String {
    driver::open_the_history(app);
    let line = driver::steps(app)
        .into_iter()
        .find(|line| line.ends_with("4 arêtes arrondies"))
        .unwrap_or_else(|| panic!("the fillet laid no line: {:?}", driver::steps(app)));
    driver::right_click(app, &line);
    driver::click_the_button(app, "Modifier le congé");
    assert!(
        driver::reads(app, "Rayon :"),
        "« Modifier le congé » on « {line} » opened no panel, showing {:?}",
        driver::on_screen(app),
    );
    line
}

/// What the view bar reads of the part under the pointer, held half a
/// millimetre in from the middle of the first top edge: matter of the block
/// whole, which a fillet or a chamfer on that edge takes away.
fn read_just_inside_the_first_top_edge(app: &mut driver::App, top: &[[DVec3; 2]; 4]) -> String {
    let across = (middle(top[2]) - middle(top[0])).normalize();
    let point = middle(top[0]) + across * 0.5;
    driver::hold_the_canvas(app);
    let at = driver::world_point_on_screen(app, point);
    app.hover_at(at);
    app.run();
    driver::the_bar_s_coordinates(app)
}

/// Where along X a reading of the bar stands: « X 10,00 Y -3,00 Z 30,10 mm ».
fn x_of(reading: &str) -> Option<f64> {
    let x = reading.strip_prefix("X ")?.split(" Y ").next()?;
    x.replace(',', ".").parse().ok()
}

/// How far from the top along X the matter the pointer meets may read and
/// still be the top: two decimals in the bar, and a pixel of aim.
const ON_THE_TOP: f64 = 0.1;

fn on_the_top(reading: &str, top: &[[DVec3; 2]; 4]) -> bool {
    x_of(reading).is_some_and(|x| (x - top[0][0].x).abs() <= ON_THE_TOP)
}

#[test]
fn while_a_step_is_reopened_the_body_drawn_is_the_step_laid_as_the_panel_holds_it_redrawn_at_each_change()
 {
    let test = "while_reopened_the_step_is_drawn_rebuilt";
    let (mut app, top, _) = a_fillet_at_twice_r_reopened(test);
    let mut wrong = Vec::new();
    let mut readings: Vec<(&str, String)> = Vec::new();

    readings.push((
        "reopened, rounded at « r * 2 »",
        read_just_inside_the_first_top_edge(&mut app, &top),
    ));
    driver::type_beside(&mut app, "Rayon :", "3");
    readings.push((
        "the radius typed 3",
        read_just_inside_the_first_top_edge(&mut app, &top),
    ));
    driver::type_beside(&mut app, "Rayon :", "4");
    readings.push((
        "the radius typed 4",
        read_just_inside_the_first_top_edge(&mut app, &top),
    ));
    driver::click_exactly(&mut app, "Chanfrein");
    assert!(
        driver::reads(&app, "Distance :"),
        "the fixture turns the fillet reopened into a chamfer, showing {:?}",
        driver::on_screen(&app),
    );
    readings.push((
        "turned into a chamfer of 4",
        read_just_inside_the_first_top_edge(&mut app, &top),
    ));
    for (at, (change, reading)) in readings.iter().enumerate() {
        if on_the_top(reading, &top) {
            wrong.push(format!(
                "{change}, the pointer meets « {reading} », the sharp top of the block before the \
                 step"
            ));
        }
        if let Some((before, earlier)) = at.checked_sub(1).map(|at| &readings[at])
            && earlier == reading
        {
            wrong.push(format!(
                "{change}, the pointer meets « {reading} », as it did, {before}"
            ));
        }
    }
    let chamfered = readings.last().expect("four readings").1.clone();

    driver::click_at_world_point_with(&mut app, middle(top[0]), egui::Modifiers::SHIFT);
    assert!(
        are_the_edges(&app.state().the_edges_chosen(), &top[1..]),
        "the fixture takes the first top edge out, leaving {:?} chosen",
        app.state().the_edges_chosen(),
    );
    let taken_out = read_just_inside_the_first_top_edge(&mut app, &top);
    if !on_the_top(&taken_out, &top) {
        wrong.push(format!(
            "the first top edge taken out, the pointer meets « {taken_out} » rather than the \
             block's sharp top beside that edge"
        ));
    }
    driver::click_at_world_point_with(&mut app, middle(top[0]), egui::Modifiers::SHIFT);
    assert!(
        are_the_edges(&app.state().the_edges_chosen(), &top),
        "the fixture puts the first top edge back, leaving {:?} chosen",
        app.state().the_edges_chosen(),
    );
    let put_back = read_just_inside_the_first_top_edge(&mut app, &top);
    if put_back != chamfered {
        wrong.push(format!(
            "the first top edge put back, the pointer meets « {put_back} », where the chamfer of \
             4 on the four edges met it at « {chamfered} »"
        ));
    }

    driver::click(&mut app, "Appliquer");
    let applied = read_just_inside_the_first_top_edge(&mut app, &top);
    if applied != chamfered {
        wrong.push(format!(
            "once applied, the pointer meets « {applied} », where the chamfer drawn before \
             « Appliquer » met it at « {chamfered} »"
        ));
    }

    assert!(
        wrong.is_empty(),
        "the fillet of the four top edges reopened and changed, the body drawn is not the step \
         laid as the panel holds it:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn a_size_the_kernel_reduces_shows_the_asked_and_the_taken_values_in_the_panel() {
    let test = "a_size_the_kernel_reduces_while_reopened";
    let (mut app, _, line) = a_fillet_at_twice_r_reopened(test);
    let laid = driver::detail_of(&app, &line);
    let reduced = "15 mm demandé, 10 mm appliqué";
    let mut wrong = Vec::new();
    if driver::shows_or_says(&app, "demandé") {
        wrong.push(format!(
            "reopened at « r * 2 », which the block's walls 10 high give whole, something reads \
             « demandé »: {:?}",
            driver::on_screen(&app),
        ));
    }

    driver::type_beside(&mut app, "Rayon :", "15");
    if !driver::reads(&app, reduced) {
        wrong.push(format!(
            "the radius typed 15, on walls 10 high, the panel does not read « {reduced} », \
             showing {:?}",
            driver::on_screen(&app),
        ));
    }
    if driver::detail_of(&app, &line) != laid {
        wrong.push(format!(
            "before « Appliquer », « {line} » reads {:?} under it, where it read {laid:?}",
            driver::detail_of(&app, &line),
        ));
    }
    driver::type_beside(&mut app, "Rayon :", "3");
    if driver::shows_or_says(&app, "demandé") {
        wrong.push(format!(
            "the radius typed back to 3, something still reads « demandé »: {:?}",
            driver::on_screen(&app),
        ));
    }
    driver::type_beside(&mut app, "Rayon :", "15");
    driver::click(&mut app, "Appliquer");
    assert_eq!(
        driver::detail_of(&app, &line).as_deref(),
        Some(reduced),
        "the fixture's fillet of 15, applied, reads otherwise under its history line",
    );

    assert!(
        wrong.is_empty(),
        "the fillet of the four top edges reopened, its radius typed 15:\n  {}",
        wrong.join("\n  "),
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

/// The centre of the disc of `a_block_chamfered_beside_a_disc`, its radius
/// and its height.
const THE_DISC: (DVec2, f64, f64) = (DVec2::new(45.0, 5.0), 5.0, 10.0);

/// A block 30 by 10 by 20 raised from XY, a disc of `THE_DISC` raised beside
/// it from the same sketch, then the edge at the top of the block's front
/// chamfered 2: a chamfer the kernel declines to move onto the disc's rim.
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
}

/// Of the disc's top rim, the point standing furthest on screen from the
/// middle of its foot: on the far side of its top, in sight from above.
fn on_the_disc_s_top_rim(app: &driver::App) -> DVec3 {
    let (centre, radius, height) = THE_DISC;
    let foot = driver::world_point_on_screen(app, centre.extend(0.0));
    (0..64)
        .map(|step| {
            let angle = std::f64::consts::TAU * f64::from(step) / 64.0;
            DVec3::new(
                centre.x + radius * angle.cos(),
                centre.y + radius * angle.sin(),
                height,
            )
        })
        .max_by(|a, b| {
            let from = |point: DVec3| driver::world_point_on_screen(app, point).distance(foot);
            from(*a).total_cmp(&from(*b))
        })
        .expect("points around the rim")
}

/// What the view bar reads of the part under the pointer, held on the top of
/// a block 30 by 10 by 20 raised from XY, half a millimetre in from the middle
/// of its front edge: matter a chamfer or a fillet of 2 there takes away.
fn read_just_inside_the_front_top_edge(app: &mut driver::App) -> String {
    driver::hold_the_canvas(app);
    let at = driver::world_point_on_screen(app, DVec3::new(15.0, 0.5, 20.0));
    app.hover_at(at);
    app.run();
    driver::the_bar_s_coordinates(app)
}

/// Whether a reading of the bar stands on the block's top, 20 high: two
/// decimals in the bar, and a pixel of aim.
fn on_the_block_s_top(reading: &str) -> bool {
    let z = reading
        .split(" Z ")
        .nth(1)
        .and_then(|z| z.split(' ').next())
        .and_then(|z| z.replace(',', ".").parse::<f64>().ok());
    z.is_some_and(|z| (z - 20.0).abs() <= ON_THE_TOP)
}

const NOT_YET_A_CHAMFER: &str =
    "Ce chanfrein n'est pas encore possible sur cette arête : la pièce reste comme elle était.";

#[test]
fn a_choice_the_kernel_declines_says_why_as_it_is_made_draws_the_part_before_the_step_and_greys_apply()
 {
    let test = "a_choice_the_kernel_declines_while_reopened";
    let mut app = driver::a_block(test);
    driver::the_part_put_away(&mut app, test, "Bloc");
    open_in_the_app(&mut app, test, &a_block_chamfered_beside_a_disc());
    let chamfered = read_just_inside_the_front_top_edge(&mut app);
    assert!(
        !on_the_block_s_top(&chamfered),
        "the fixture's chamfer is drawn where the pointer stands, which meets « {chamfered} »",
    );
    driver::open_the_history(&mut app);
    let line = driver::steps(&app)
        .into_iter()
        .find(|line| line.ends_with("Arête chanfreinée"))
        .unwrap_or_else(|| panic!("no line of the chamfer: {:?}", driver::steps(&app)));
    driver::right_click(&mut app, &line);
    driver::click_the_button(&mut app, "Modifier le chanfrein");
    assert!(
        driver::reads(&app, "Distance :") && driver::is_enabled(&app, "Appliquer"),
        "the fixture reopens « {line} » with « Appliquer » offered, showing {:?}",
        driver::on_screen(&app),
    );

    let rim = on_the_disc_s_top_rim(&app);
    driver::click_at_world_point(&mut app, rim);

    let mut wrong = Vec::new();
    if !driver::reads(&app, NOT_YET_A_CHAMFER) {
        wrong.push(format!(
            "before any « Appliquer », the panel does not read « {NOT_YET_A_CHAMFER} »"
        ));
    }
    if driver::is_enabled(&app, "Appliquer") {
        wrong.push("« Appliquer » is offered".to_string());
    }
    let drawn = read_just_inside_the_front_top_edge(&mut app);
    if !on_the_block_s_top(&drawn) {
        wrong.push(format!(
            "half a millimetre in from the block's top front edge, the pointer meets « {drawn} », \
             not the sharp top of the block before the chamfer"
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

/// Empties the field beside `label`, as a select-all and a backspace do.
fn empty_the_field_beside(app: &mut driver::App, label: &str) {
    driver::the_field_beside(app, label).focus();
    app.run();
    app.event(egui::Event::Key {
        key: egui::Key::A,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::COMMAND,
    });
    app.key_press(egui::Key::Backspace);
    app.run();
    driver::hold_the_canvas(app);
}

#[test]
fn a_size_that_does_not_read_keeps_the_last_body_drawn_and_says_what_is_wrong() {
    let test = "a_size_that_does_not_read_while_reopened";
    let (mut app, top, _) = a_fillet_at_twice_r_reopened(test);
    driver::type_beside(&mut app, "Rayon :", "3");
    let at_three = read_just_inside_the_first_top_edge(&mut app, &top);
    assert!(
        !on_the_top(&at_three, &top),
        "the fixture's fillet of 3 is drawn where the pointer stands, which meets « {at_three} »",
    );
    let mut wrong = Vec::new();

    driver::type_beside(&mut app, "Rayon :", "q");
    let unknown = "« q » n'est pas une variable de la pièce";
    if !driver::reads(&app, unknown) {
        wrong.push(format!(
            "« q » typed, the panel does not read « {unknown} »"
        ));
    }
    let drawn = read_just_inside_the_first_top_edge(&mut app, &top);
    if drawn != at_three {
        wrong.push(format!(
            "« q » typed, the pointer meets « {drawn} », where the fillet of 3 met it at \
             « {at_three} »"
        ));
    }

    empty_the_field_beside(&mut app, "Rayon :");
    assert_eq!(
        driver::field_beside(&app, "Rayon :"),
        "",
        "the fixture empties the radius",
    );
    if driver::is_enabled(&app, "Appliquer") {
        wrong.push("the radius emptied, « Appliquer » is offered".to_string());
    }
    let drawn = read_just_inside_the_first_top_edge(&mut app, &top);
    if drawn != at_three {
        wrong.push(format!(
            "the radius emptied, the pointer meets « {drawn} », where the fillet of 3 met it at \
             « {at_three} »"
        ));
    }

    assert!(
        wrong.is_empty(),
        "the fillet of the four top edges reopened, its radius typed 3 and then not a size:\n  \
         {}\n showing {:?}",
        wrong.join("\n  "),
        driver::on_screen(&app),
    );
}

/// One change made in the panel of `a_fillet_at_twice_r_reopened`, and what
/// it is said as.
type Change = (&'static str, fn(&mut driver::App, &[[DVec3; 2]; 4]));

/// Hovers over the four top edges and the block's top, then lets frames go by,
/// nothing changed in the panel.
fn idle(app: &mut driver::App, top: &[[DVec3; 2]; 4]) {
    for edge in top {
        let at = driver::world_point_on_screen(app, middle(*edge));
        app.hover_at(at);
        app.run();
    }
    read_just_inside_the_first_top_edge(app, top);
    for _ in 0..5 {
        app.run();
    }
}

/// A box 10 by 10 by 5 raised from a sketch on XY, behind the block's foot and
/// clear of it: matter laid after whatever the part already holds.
fn a_box_behind_the_block(part: &mut cao_part::PartDocument) {
    let sketch = part.sketches().len();
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    rectangle(
        part,
        sketch,
        DVec2::new(-30.0, 0.0),
        DVec2::new(-20.0, 10.0),
    );
    part.apply(Operation::Extrude {
        sketch,
        areas: part.areas_at(sketch, &[DVec2::new(-25.0, 5.0)]),
        distance: 5.0.into(),
        mode: ExtrusionMode::Add,
    });
}

#[test]
fn the_step_is_rebuilt_once_per_change_and_the_later_steps_wait_for_apply() {
    let test = "the_step_is_rebuilt_once_per_change";
    let (mut app, top) = a_fillet_at_twice_r(test);
    let mut part = driver::the_part_put_away(&mut app, test, "Bloc");
    let held = part.body().bounds();
    a_box_behind_the_block(&mut part);
    assert_ne!(
        part.body().bounds(),
        held,
        "the fixture raises a box after the fillet",
    );
    open_in_the_app(&mut app, test, &part);
    let line = the_fillet_reopened(&mut app);
    let through_the_fillet = part
        .history
        .operations()
        .iter()
        .position(|operation| matches!(operation, Operation::Blend { .. }))
        .expect("the fillet is recorded")
        + 1;
    let in_all = part.history.operations().len();
    assert!(
        through_the_fillet < in_all,
        "the fixture lays operations after « {line} »: {through_the_fillet} through it, {in_all} \
         in all",
    );
    let mut wrong = Vec::new();

    let mut built = app.state().the_blend_previews_built();
    if built == 0 {
        wrong.push("reopened, no preview of the step is built".to_string());
    }
    idle(&mut app, &top);
    if app.state().the_blend_previews_built() != built {
        wrong.push(format!(
            "reopened, frames and hovers with nothing changed in the panel built the preview {} \
             times more",
            app.state().the_blend_previews_built() - built,
        ));
    }

    let changes: [Change; 3] = [
        ("the radius typed 3", |app, _| {
            driver::type_beside(app, "Rayon :", "3");
        }),
        ("turned into a chamfer", |app, _| {
            driver::click_exactly(app, "Chanfrein");
        }),
        ("the first top edge taken out", |app, top| {
            driver::click_at_world_point_with(app, middle(top[0]), egui::Modifiers::SHIFT);
        }),
    ];
    for (change, make) in changes {
        built = app.state().the_blend_previews_built();
        make(&mut app, &top);
        let after_the_change = app.state().the_blend_previews_built();
        if after_the_change != built + 1 {
            wrong.push(format!(
                "{change}, the preview was built {} times, not once",
                after_the_change.saturating_sub(built),
            ));
        }
        idle(&mut app, &top);
        if app.state().the_blend_previews_built() != after_the_change {
            wrong.push(format!(
                "{change}, frames and hovers with nothing changed since built the preview {} \
                 times more",
                app.state().the_blend_previews_built() - after_the_change,
            ));
        }
        let laid = app.state().the_steps_laid_by_the_last_preview();
        if laid != 1 {
            wrong.push(format!(
                "{change}, the preview laid {laid} operations, not the step alone on the part \
                 before it kept from the reopening; the history holds {through_the_fillet} up to \
                 and with « {line} » and {in_all} in all",
            ));
        }
    }

    assert!(
        wrong.is_empty(),
        "the fillet of the four top edges reopened under a box raised after it, then \
         changed:\n  {}",
        wrong.join("\n  "),
    );
}

/// A block 30 by 10 by 20 and a boss 20 by 10 by 30 over its right end,
/// both raised from one sketch on XY and joined, then the edge where the boss
/// rises from the block's top rounded 2: an edge the boss carries, gone from
/// the block alone. One sketch, so that every line of the history stands on
/// screen.
fn a_block_with_a_boss_rounded_at_its_foot() -> cao_part::PartDocument {
    let mut part =
        cao_part::PartDocument::new("Bloc", "2026-11-17T09:00:00Z".parse().expect("a date"));
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    rectangle(&mut part, 0, DVec2::ZERO, DVec2::new(30.0, 10.0));
    rectangle(&mut part, 0, DVec2::new(20.0, 0.0), DVec2::new(40.0, 10.0));
    for (inside, height) in [
        (vec![DVec2::new(10.0, 5.0), DVec2::new(25.0, 5.0)], 20.0),
        (vec![DVec2::new(25.0, 5.0), DVec2::new(35.0, 5.0)], 30.0),
    ] {
        part.apply(Operation::Extrude {
            sketch: 0,
            areas: part.areas_at(0, &inside),
            distance: height.into(),
            mode: ExtrusionMode::Add,
        });
    }
    let foot_of_the_boss = the_drawn_edge(&part, |point| {
        (point.x - 20.0).abs() < 1e-9 && (point.z - 20.0).abs() < 1e-9
    });
    let outcome = part.apply(Operation::Blend {
        edges: vec![foot_of_the_boss],
        shape: BlendShape::Fillet,
        size: 2.0.into(),
    });
    assert_eq!(outcome, None, "the fixture's fillet is built");
    part
}

/// Clicks « Rendre transparente » on the row of the part panel that names
/// this volume.
fn make_see_through(app: &mut driver::App, volume: &str) {
    let row = driver::the_row(app, volume)
        .unwrap_or_else(|| panic!("no row « {volume} »: {:?}", driver::on_screen(app)))
        .rect();
    let button = app
        .query_all_by_label("Rendre transparente")
        .min_by(|one, other| {
            let off = |rect: egui::Rect| (rect.center().y - row.center().y).abs();
            off(one.rect()).total_cmp(&off(other.rect()))
        })
        .expect("a volume to make see-through");
    assert!(
        (button.rect().center().y - row.center().y).abs() < row.height(),
        "no « Rendre transparente » on the row « {volume} »",
    );
    button.click();
    app.run();
    driver::hold_the_canvas(app);
}

#[test]
fn with_a_volume_before_the_step_see_through_the_panel_says_what_apply_will_meet() {
    let test = "with_a_volume_before_the_step_see_through";
    let mut app = driver::a_block(test);
    driver::the_part_put_away(&mut app, test, "Bloc");
    open_in_the_app(&mut app, test, &a_block_with_a_boss_rounded_at_its_foot());
    make_see_through(&mut app, "Extrusion 2");
    let left = app.query_all_by_label("Rendre transparente").count();
    assert_eq!(
        left,
        1,
        "the fixture makes the boss see-through and leaves the block opaque, showing {:?}",
        driver::on_screen(&app),
    );
    driver::open_the_history(&mut app);
    let line = driver::steps(&app)
        .into_iter()
        .find(|line| line.ends_with("Arête arrondie"))
        .unwrap_or_else(|| panic!("no line of the fillet: {:?}", driver::steps(&app)));
    driver::right_click(&mut app, &line);
    driver::click_the_button(&mut app, "Modifier le congé");
    assert!(
        driver::reads(&app, "Rayon :"),
        "« Modifier le congé » on « {line} » opened no panel, showing {:?}",
        driver::on_screen(&app),
    );
    let reduced = "50 mm demandé, 10 mm appliqué";
    let declined = [
        "Cette arête n'est plus dans la pièce : choisissez-en une autre.",
        "Ce congé n'est pas encore possible sur cette arête : la pièce reste comme elle était.",
        "Le calcul de ce congé n'a pas abouti : la pièce reste comme elle était.",
    ];

    driver::type_beside(&mut app, "Rayon :", "50");

    let mut wrong = Vec::new();
    for sentence in declined {
        if driver::reads(&app, sentence) {
            wrong.push(format!("the panel reads « {sentence} »"));
        }
    }
    if !driver::is_enabled(&app, "Appliquer") {
        wrong.push("« Appliquer » is greyed".to_string());
    }
    if !driver::reads(&app, reduced) {
        wrong.push(format!("the panel does not read « {reduced} »"));
    }
    let said_before = wrong.is_empty();
    driver::click(&mut app, "Appliquer");
    let after = driver::detail_of(&app, &line);
    if said_before && after.as_deref() != Some(reduced) {
        wrong.push(format!(
            "the panel read « {reduced} », and « Appliquer » left « {line} » reading {after:?}"
        ));
    }

    assert!(
        wrong.is_empty(),
        "the fillet at the foot of a boss made see-through, reopened and typed 50, where the \
         part with the boss opaque takes 10 of it:\n  {}\n showing {:?}",
        wrong.join("\n  "),
        driver::on_screen(&app),
    );
}

/// Every text said on screen but the view bar's coordinates, which move with
/// the pointer.
fn said_but_the_coordinates(app: &driver::App) -> Vec<String> {
    driver::said(app)
        .into_iter()
        .filter(|said| !said.starts_with("X "))
        .collect()
}

#[test]
fn a_size_not_above_zero_keeps_the_last_body_drawn_and_says_something_beside_the_field() {
    let test = "a_size_not_above_zero_while_reopened";
    let (mut app, top, _) = a_fillet_at_twice_r_reopened(test);
    driver::type_beside(&mut app, "Rayon :", "3");
    let said_at_three = said_but_the_coordinates(&app);
    let at_three = read_just_inside_the_first_top_edge(&mut app, &top);
    assert!(
        !on_the_top(&at_three, &top),
        "the fixture's fillet of 3 is drawn where the pointer stands, which meets « {at_three} »",
    );
    let mut wrong = Vec::new();

    for typed in ["0", "-2"] {
        driver::type_beside(&mut app, "Rayon :", typed);
        let new: Vec<String> = said_but_the_coordinates(&app)
            .into_iter()
            .filter(|said| !said_at_three.contains(said))
            .collect();
        if new.is_empty() {
            wrong.push(format!(
                "« {typed} » typed, nothing is said that was not said at 3: {said_at_three:?}"
            ));
        }
        if driver::is_enabled(&app, "Appliquer") {
            wrong.push(format!("« {typed} » typed, « Appliquer » is offered"));
        }
        let drawn = read_just_inside_the_first_top_edge(&mut app, &top);
        if drawn != at_three {
            wrong.push(format!(
                "« {typed} » typed, the pointer meets « {drawn} », where the fillet of 3 met it \
                 at « {at_three} »"
            ));
        }
    }

    assert!(
        wrong.is_empty(),
        "the fillet of the four top edges reopened, its radius typed 3 and then not above \
         zero:\n  {}",
        wrong.join("\n  "),
    );
}

/// A block 30 by 10 by 20 raised from XY, the edge at the top of its front
/// rounded 2, then the upright edge at its back right chamfered 1: two blend
/// steps, the second clear of the first.
fn a_block_rounded_then_chamfered() -> cao_part::PartDocument {
    let mut part =
        cao_part::PartDocument::new("Bloc", "2026-11-17T09:00:00Z".parse().expect("a date"));
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    rectangle(&mut part, 0, DVec2::ZERO, DVec2::new(30.0, 10.0));
    part.apply(Operation::Extrude {
        sketch: 0,
        areas: part.areas_at(0, &[DVec2::new(15.0, 5.0)]),
        distance: 20.0.into(),
        mode: ExtrusionMode::Add,
    });
    let top_of_the_front = the_drawn_edge(&part, |point| {
        point.y.abs() < 1e-9 && (point.z - 20.0).abs() < 1e-9
    });
    let back_right = the_drawn_edge(&part, |point| {
        (point.x - 30.0).abs() < 1e-9 && (point.y - 10.0).abs() < 1e-9
    });
    for (edge, shape, size) in [
        (top_of_the_front, BlendShape::Fillet, 2.0),
        (back_right, BlendShape::Chamfer, 1.0),
    ] {
        let outcome = part.apply(Operation::Blend {
            edges: vec![edge],
            shape,
            size: size.into(),
        });
        assert_eq!(outcome, None, "the fixture's {shape:?} is built");
    }
    part
}

#[test]
fn another_step_reopened_after_a_change_cancelled_draws_its_own_step_not_the_last_preview() {
    let test = "another_step_reopened_after_a_change_cancelled";
    let mut app = driver::a_block(test);
    driver::the_part_put_away(&mut app, test, "Bloc");
    open_in_the_app(&mut app, test, &a_block_rounded_then_chamfered());
    let as_laid = read_just_inside_the_front_top_edge(&mut app);
    assert!(
        !on_the_block_s_top(&as_laid),
        "the fixture's fillet of 2 is drawn where the pointer stands, which meets « {as_laid} »",
    );
    driver::open_the_history(&mut app);
    let line_of = |app: &driver::App, name: &str| {
        driver::steps(app)
            .into_iter()
            .find(|line| line.ends_with(name))
            .unwrap_or_else(|| panic!("no line « {name} »: {:?}", driver::steps(app)))
    };
    let fillet = line_of(&app, "Arête arrondie");
    driver::right_click(&mut app, &fillet);
    driver::click_the_button(&mut app, "Modifier le congé");
    driver::type_beside(&mut app, "Rayon :", "4");
    let at_four = read_just_inside_the_front_top_edge(&mut app);
    assert_ne!(
        at_four, as_laid,
        "the fixture's fillet typed 4 is drawn otherwise than at 2",
    );
    driver::click_the_button(&mut app, "Annuler");
    assert!(
        !driver::reads(&app, "Rayon :"),
        "the fixture cancels the fillet reopened, showing {:?}",
        driver::on_screen(&app),
    );

    let chamfer = line_of(&app, "Arête chanfreinée");
    driver::right_click(&mut app, &chamfer);
    driver::click_the_button(&mut app, "Modifier le chanfrein");
    assert!(
        driver::reads(&app, "Distance :"),
        "« Modifier le chanfrein » on « {chamfer} » opened no panel, showing {:?}",
        driver::on_screen(&app),
    );

    let drawn = read_just_inside_the_front_top_edge(&mut app);
    assert_eq!(
        drawn, as_laid,
        "« {chamfer} » reopened after « {fillet} » was typed 4 and cancelled, the pointer beside \
         the fillet meets « {drawn} », where the fillet of 2 meets it at « {as_laid} » and the \
         fillet of 4 cancelled at « {at_four} »",
    );
}
