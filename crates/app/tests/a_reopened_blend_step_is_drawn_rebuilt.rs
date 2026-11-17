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

/// `driver::a_block`, a variable `r` made at 1, the four edges of the block's
/// top rounded at « r * 2 », then that fillet reopened from its history line:
/// the app and the four edges.
fn a_fillet_at_twice_r_reopened(test: &str) -> (driver::App, [[DVec3; 2]; 4]) {
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
    driver::open_the_history(&mut app);
    let line = driver::steps(&app)
        .into_iter()
        .find(|line| line.ends_with("4 arêtes arrondies"))
        .unwrap_or_else(|| panic!("the fillet laid no line: {:?}", driver::steps(&app)));
    driver::right_click(&mut app, &line);
    driver::click_the_button(&mut app, "Modifier le congé");
    assert!(
        driver::reads(&app, "Rayon :"),
        "« Modifier le congé » on « {line} » opened no panel, showing {:?}",
        driver::on_screen(&app),
    );
    (app, top)
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
    let (mut app, top) = a_fillet_at_twice_r_reopened(test);
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
