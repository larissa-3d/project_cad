//! The extrusion and the revolution set in the step panel, driven through the
//! whole application: what the panel holds, what it agrees with, and every way
//! of raising or cutting matter it takes over from the row it replaces.
//!
//! Closes #740.
//! - Extruding a sketch opens a step panel titled « Extrusion droite »
//!   (« Révolution » for a revolution), context the sketch, with the sections
//!   « Aires », « Matière », « Profondeur », « Autour d'un axe », « Inverser le
//!   sens » and « Annuler » / « Appliquer ↵ » —
//!   `extruding_a_sketch_opens_the_step_panel_on_that_sketch_with_its_sections`,
//!   `around_an_axis_titles_the_panel_revolution`
//! - « Ajout » / « Enlèvement » is set in the ribbon or in the panel, and the
//!   two always agree — `a_cut_chosen_in_the_panel_turns_the_ribbon_to_a_cut`,
//!   `a_cut_chosen_in_the_ribbon_turns_the_panel_to_a_cut`
//! - The depth accepts a formula with variables, shown as chips, with « = 30
//!   mm » on the right and the completion it has today —
//!   `a_depth_written_with_a_variable_shows_it_as_a_chip_and_what_it_comes_to`;
//!   the completion is held where every field's is, in
//!   `crates/app/tests/completion.rs`, by
//!   `the_start_of_a_name_in_the_extrusion_panel_is_completed_too`
//! - « Autour d'un axe » replaces the depth with an angle and an axis (U, V or
//!   a clicked trait) —
//!   `around_an_axis_puts_an_angle_and_an_axis_in_place_of_the_depth`,
//!   `a_trait_clicked_around_an_axis_is_the_axis_turned_about`
//! - The view draws the extruded volume alone, not fused into the part,
//!   see-through, green for an addition and red for a cut, and redraws it at
//!   every keystroke — no test: what a frame draws is wgpu's, which the driver
//!   does not reach. The volume alone is held in
//!   `crates/part/tests/previewing_a_step_of_matter.rs`; what is previewed,
//!   following every keystroke, in
//!   `crates/app/src/screens/extrusion/state/tests.rs`; the frame carrying it
//!   see-through in the colour of its mode in
//!   `crates/app/src/screens/viewport/render/extrusion/tests.rs`, each under
//!   its own block for #740. How the GPU blends it is checked by eye, the
//!   renderer having no test but the `offscreen` example
//! - The extrusion row under the ribbon is gone —
//!   `extruding_a_sketch_lays_no_row_under_the_ribbon`
//! - Every way of extruding today, and editing an existing extrusion, work as
//!   before — from the panel, a straight addition, a cut and a reversed one —
//!   `an_addition_applied_from_the_panel_raises_the_area_by_its_depth`,
//!   `a_cut_applied_from_the_panel_takes_its_depth_out_of_the_part`,
//!   `a_reversed_extrusion_applied_from_the_panel_goes_the_other_way_along_the_plane`;
//!   a revolution about U, V or a clicked trait —
//!   `a_revolution_applied_from_the_panel_turns_about_u_v_or_a_clicked_trait`;
//!   an extrusion reopened from its alert's « Modifier », turned the other way
//!   in the panel and applied, changes its own step — held in
//!   `crates/app/tests/the_alerts.rs` by
//!   `modify_on_an_extrusion_s_alert_reopens_it_with_its_values`: an alert's
//!   « Modifier » is the only way to reopen a step, and that file builds the
//!   part the kernel declines it needs. The tests that drove an extrusion
//!   through the row, `driver::a_block` first, go through the panel instead
//!   and pass as they did

mod driver;

use cao_part::{ExtrusionMode, Operation, PartDocument, RevolutionAxis};
use cao_sketch::SketchAxis;
use egui::accesskit::{Role, Toggled};
use egui_kittest::kittest::{NodeT, Queryable};
use glam::DVec3;

/// `driver::INSIDE_THE_RECTANGLE` in the world, the sketch lying on YZ: where
/// the area is clicked once the sketch is finished and the view no longer
/// reads its plane.
const INSIDE_THE_RECTANGLE: DVec3 = DVec3::new(0.0, 62.61, -15.15);

/// Everything the panel of the step says that the user reads in the order the
/// issue lists it: its title, its sections, and its two buttons.
const THE_STRAIGHT_PANEL: [&str; 8] = [
    "Extrusion droite",
    "Aires",
    "Matière",
    "Profondeur",
    "Autour d'un axe",
    "Inverser le sens",
    "Annuler",
    "Appliquer ↵",
];

/// What the row under the ribbon wrote while an area was chosen: the label
/// before its height, its count of the areas chosen, and its checkbox to push
/// the other way. None of them is a word the panel uses.
const THE_OLD_ROW: [&str; 3] = ["Hauteur :", "1 aire(s)", "Sens inverse"];

/// The rectangle drawn from `driver::A_POINT_ABOVE_THE_ORIGIN` to
/// `driver::A_POINT_BELOW_AND_RIGHT`: 162.5 along Y by 125 along Z, from
/// (−25, −37.5), raised along X.
const THE_RECTANGLE_S_AREA: f64 = 162.5 * 125.0;

/// On the rectangle's left side, at u = −25, in the world: where a trait is
/// clicked to turn about.
const ON_THE_LEFT_SIDE: DVec3 = DVec3::new(0.0, -25.0, 25.0);

/// How far from the arithmetic a volume of flat-sided matter may stand.
const VOLUME: f64 = 1e-6;

/// How far a face of a block may stand from where it was raised to.
const PLACE: f64 = 1e-6;

/// A part named « Bloc », nothing in it.
fn a_part(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Bloc");
    app
}

/// A rectangle drawn on YZ, its sketch then finished, which is where the tool
/// is offered on it.
fn draw_the_rectangle_and_finish(app: &mut driver::App) {
    driver::start_a_sketch(app);
    driver::click(app, "Rectangle (R)");
    driver::click_at_sketch_point(app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at_sketch_point(app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::finish_the_sketch(app);
}

/// « Ajout de matière » taken and the rectangle's area clicked: an extrusion
/// being set.
fn choose_the_area_to_add_matter(app: &mut driver::App) {
    driver::click_in_the_ribbon(app, "Ajout de matière");
    driver::click_at_world_point(app, INSIDE_THE_RECTANGLE);
}

/// A part with the rectangle drawn and its area chosen to add matter.
fn an_area_chosen_to_add_matter(test: &str) -> driver::App {
    let mut app = a_part(test);
    draw_the_rectangle_and_finish(&mut app);
    choose_the_area_to_add_matter(&mut app);
    app
}

/// The control standing over the view whose name reads exactly this: the
/// panel's, never the ribbon's of the same name.
fn the_panel_s<'a>(app: &'a driver::App, label: &str) -> Option<egui_kittest::Node<'a>> {
    app.get_all_by(|_| true).find(|node| {
        let read = node.accesskit_node();
        read.role() != Role::TextRun
            && read.label().as_deref() == Some(label)
            && read.bounding_box().is_some()
            && driver::stands_inside_the_view(app, node.rect())
    })
}

fn click_in_the_panel(app: &mut driver::App, label: &str) {
    let node = the_panel_s(app, label).unwrap_or_else(|| {
        panic!(
            "the panel offers no « {label} »; over the view: {:?}",
            read_over_the_view(app)
        )
    });
    node.click();
    app.run();
    driver::hold_the_canvas(app);
}

fn is_lit(node: Option<egui_kittest::Node<'_>>) -> bool {
    node.is_some_and(|node| node.accesskit_node().toggled() == Some(Toggled::True))
}

/// Whether the panel's control of that name is lit.
fn the_panel_lights(app: &driver::App, label: &str) -> bool {
    is_lit(the_panel_s(app, label))
}

/// Whether the ribbon's button whose name starts with this is lit.
fn the_ribbon_lights(app: &driver::App, label: &str) -> bool {
    is_lit(driver::the_ribbon_s(app, label))
}

/// Every run of the text painted reading exactly `text`, with whether it is
/// laid on a tint: a variable's name is, as a chip in the field.
fn runs_painted(app: &driver::App, text: &str) -> Vec<(String, bool)> {
    let mut pending: Vec<&egui::Shape> = app
        .output()
        .shapes
        .iter()
        .map(|clipped| &clipped.shape)
        .collect();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            egui::Shape::Text(written) if written.galley.text() == text => {
                let job = &written.galley.job;
                return job
                    .sections
                    .iter()
                    .map(|section| {
                        let bytes = section.byte_range.start.0..section.byte_range.end.0;
                        let run = job.text[bytes].to_owned();
                        (run, section.format.background != egui::Color32::TRANSPARENT)
                    })
                    .filter(|(run, _)| !run.is_empty())
                    .collect();
            }
            _ => {}
        }
    }
    Vec::new()
}

/// The part as the application put it away, opened from its file.
fn put_away(app: &mut driver::App, test: &str) -> PartDocument {
    driver::the_part_put_away(app, test, "Bloc")
}

fn assert_volume(part: &PartDocument, expected: f64, what: &str) {
    let volume = part.body().volume();
    assert!(
        (volume - expected).abs() <= VOLUME * expected,
        "{what} holds {volume}, not {expected}",
    );
}

/// Where the part's matter stands along X, the rectangle's normal.
fn along_x(part: &PartDocument, what: &str) -> (f64, f64) {
    let (low, high) = part
        .body()
        .bounds()
        .unwrap_or_else(|| panic!("{what} holds no matter"));
    (low.x, high.x)
}

fn assert_along_x(part: &PartDocument, from: f64, to: f64, what: &str) {
    let (low, high) = along_x(part, what);
    assert!(
        (low - from).abs() < PLACE && (high - to).abs() < PLACE,
        "{what} stands from {low} to {high} along X, not from {from} to {to}",
    );
}

/// What a hand does in the application.
type Gesture = fn(&mut driver::App);

/// The last revolution the part records.
fn the_last_turn(part: &PartDocument) -> Option<(RevolutionAxis, ExtrusionMode)> {
    part.history
        .applied_operations()
        .iter()
        .rev()
        .find_map(|operation| match operation {
            Operation::Revolve { axis, mode, .. } => Some((*axis, *mode)),
            _ => None,
        })
}

/// Every text on screen, as a name or as a label's value, with whether it
/// stands over the view. A container `egui` lays out has no bounds to read.
fn read_on_screen(app: &driver::App) -> Vec<(String, bool)> {
    app.get_all_by(|_| true)
        .filter(|node| node.accesskit_node().bounding_box().is_some())
        .filter_map(|node| {
            let read = node.accesskit_node();
            let text = read.label().or_else(|| read.value())?;
            Some((text, driver::stands_inside_the_view(app, node.rect())))
        })
        .collect()
}

/// Every text standing over the view: what a floating panel reads, and never
/// the ribbon's « Extrusion droite », the title bar's « Annuler » or the
/// tree's rows, which stand beside it.
fn read_over_the_view(app: &driver::App) -> Vec<String> {
    read_on_screen(app)
        .into_iter()
        .filter_map(|(text, over)| over.then_some(text))
        .collect()
}

/// Every text standing outside the view: the ribbon, the row under it, the
/// tree and the title bar.
fn read_outside_the_view(app: &driver::App) -> Vec<String> {
    read_on_screen(app)
        .into_iter()
        .filter_map(|(text, over)| (!over).then_some(text))
        .collect()
}

#[test]
fn around_an_axis_titles_the_panel_revolution() {
    let mut app = an_area_chosen_to_add_matter("around_an_axis_titles_the_panel_revolution");

    click_in_the_panel(&mut app, "Autour d'un axe");

    let read = read_over_the_view(&app);
    assert!(
        read.iter().any(|over| over == "Révolution")
            && !read.iter().any(|over| over == "Extrusion droite"),
        "« Autour d'un axe » switched on, the panel is not titled « Révolution »; over the \
         view: {read:?}",
    );
}

#[test]
fn around_an_axis_puts_an_angle_and_an_axis_in_place_of_the_depth() {
    let mut app = an_area_chosen_to_add_matter("around_an_axis_puts_an_angle_and_an_axis");

    click_in_the_panel(&mut app, "Autour d'un axe");
    let turning = read_over_the_view(&app);
    click_in_the_panel(&mut app, "Autour d'un axe");
    let straight = read_over_the_view(&app);

    let holds = |read: &[String], text: &str| read.iter().any(|over| over == text);
    let in_place: Vec<&str> = ["Angle", "axe horizontal", "axe vertical"]
        .into_iter()
        .filter(|text| !holds(&turning, text))
        .collect();
    assert!(
        in_place.is_empty() && !holds(&turning, "Profondeur"),
        "around an axis, the panel lacks {in_place:?} or still asks for a depth; over the view: \
         {turning:?}",
    );
    assert!(
        holds(&straight, "Profondeur") && !holds(&straight, "Angle"),
        "« Autour d'un axe » switched off again, the panel does not ask for its depth back; \
         over the view: {straight:?}",
    );
}

#[test]
fn a_trait_clicked_around_an_axis_is_the_axis_turned_about() {
    let mut app = an_area_chosen_to_add_matter("a_trait_clicked_around_an_axis");
    click_in_the_panel(&mut app, "Autour d'un axe");
    assert!(
        the_panel_lights(&app, "axe vertical"),
        "around an axis, the panel does not start on « axe vertical »; over the view: {:?}",
        read_over_the_view(&app),
    );

    driver::click_at_world_point(&mut app, ON_THE_LEFT_SIDE);

    let read = read_over_the_view(&app);
    let named: Vec<&String> = read
        .iter()
        .filter(|over| over.starts_with("trait "))
        .collect();
    assert!(
        named.len() == 1 && the_panel_lights(&app, named[0]),
        "the rectangle's side clicked, the panel does not hold it lit as the axis; over the \
         view: {read:?}",
    );
    assert!(
        !the_panel_lights(&app, "axe vertical") && !the_panel_lights(&app, "axe horizontal"),
        "the side clicked, an axis of the sketch is still lit; over the view: {read:?}",
    );
}

#[test]
fn a_cut_chosen_in_the_panel_turns_the_ribbon_to_a_cut() {
    let mut app = an_area_chosen_to_add_matter("a_cut_chosen_in_the_panel");
    assert!(
        the_panel_lights(&app, "Ajout") && the_ribbon_lights(&app, "Ajout de matière"),
        "the fixture wants an addition lit in the panel and in the ribbon",
    );

    click_in_the_panel(&mut app, "Enlèvement");

    assert!(
        the_panel_lights(&app, "Enlèvement") && !the_panel_lights(&app, "Ajout"),
        "« Enlèvement » clicked in the panel, the panel does not light it alone",
    );
    assert!(
        the_ribbon_lights(&app, "Enlèvement de matière")
            && !the_ribbon_lights(&app, "Ajout de matière"),
        "« Enlèvement » chosen in the panel, the ribbon still lights an addition",
    );
}

#[test]
fn a_cut_chosen_in_the_ribbon_turns_the_panel_to_a_cut() {
    let mut app = an_area_chosen_to_add_matter("a_cut_chosen_in_the_ribbon");

    driver::click_in_the_ribbon(&mut app, "Enlèvement de matière");

    assert!(
        the_panel_lights(&app, "Enlèvement") && !the_panel_lights(&app, "Ajout"),
        "« Enlèvement de matière » chosen in the ribbon, the panel still lights « Ajout »; over \
         the view: {:?}",
        read_over_the_view(&app),
    );
}

#[test]
fn a_depth_written_with_a_variable_shows_it_as_a_chip_and_what_it_comes_to() {
    let mut app = a_part("a_depth_written_with_a_variable");
    driver::click_the_button(&mut app, "Variables");
    driver::type_into(&mut app, "nom", "thickness");
    driver::type_into(&mut app, "formule", "15");
    driver::click_exactly(&mut app, "Ajouter");
    driver::click_the_button(&mut app, "Variables");
    draw_the_rectangle_and_finish(&mut app);
    choose_the_area_to_add_matter(&mut app);

    driver::type_the_depth(&mut app, "thickness * 2");
    app.run();

    assert_eq!(
        driver::the_depth(&app),
        "thickness * 2",
        "the depth does not hold the formula typed",
    );
    let read = read_over_the_view(&app);
    assert!(
        read.iter().any(|over| over == "= 30 mm"),
        "« thickness * 2 » with thickness at 15, the panel does not read « = 30 mm »; over the \
         view: {read:?}",
    );
    assert_eq!(
        runs_painted(&app, "thickness * 2"),
        [("thickness".to_owned(), true), (" * 2".to_owned(), false)],
        "the variable is not painted as a chip in the depth, and the rest of the formula as text",
    );
}

#[test]
fn an_addition_applied_from_the_panel_raises_the_area_by_its_depth() {
    let test = "an_addition_applied_from_the_panel";
    let mut app = an_area_chosen_to_add_matter(test);
    driver::type_the_depth(&mut app, "30");

    click_in_the_panel(&mut app, "Appliquer ↵");

    let part = put_away(&mut app, test);
    assert_volume(
        &part,
        THE_RECTANGLE_S_AREA * 30.0,
        "the rectangle raised 30",
    );
    assert_along_x(&part, 0.0, 30.0, "the rectangle raised 30");
}

#[test]
fn a_cut_applied_from_the_panel_takes_its_depth_out_of_the_part() {
    let test = "a_cut_applied_from_the_panel";
    let mut app = an_area_chosen_to_add_matter(test);
    driver::type_the_depth(&mut app, "30");
    click_in_the_panel(&mut app, "Appliquer ↵");

    driver::click_in_the_ribbon(&mut app, "Enlèvement de matière");
    driver::click_at_world_point(&mut app, INSIDE_THE_RECTANGLE);
    driver::type_the_depth(&mut app, "10");
    click_in_the_panel(&mut app, "Appliquer ↵");

    let part = put_away(&mut app, test);
    assert_volume(
        &part,
        THE_RECTANGLE_S_AREA * 20.0,
        "the block 30 deep cut 10 into",
    );
    assert_along_x(&part, 10.0, 30.0, "the block 30 deep cut 10 into");
}

#[test]
fn a_reversed_extrusion_applied_from_the_panel_goes_the_other_way_along_the_plane() {
    let test = "a_reversed_extrusion_applied_from_the_panel";
    let mut app = an_area_chosen_to_add_matter(test);
    driver::type_the_depth(&mut app, "30");

    click_in_the_panel(&mut app, "Inverser le sens");
    click_in_the_panel(&mut app, "Appliquer ↵");

    let part = put_away(&mut app, test);
    assert_volume(
        &part,
        THE_RECTANGLE_S_AREA * 30.0,
        "the rectangle raised 30 the other way",
    );
    assert_along_x(&part, -30.0, 0.0, "the rectangle raised 30 the other way");
}

#[test]
fn a_revolution_applied_from_the_panel_turns_about_u_v_or_a_clicked_trait() {
    let ways: [(&str, Gesture, Option<RevolutionAxis>); 3] = [
        (
            "u",
            |app| click_in_the_panel(app, "axe horizontal"),
            Some(RevolutionAxis::Sketch(SketchAxis::U)),
        ),
        (
            "v",
            |app| click_in_the_panel(app, "axe vertical"),
            Some(RevolutionAxis::Sketch(SketchAxis::V)),
        ),
        (
            "a_trait",
            |app| driver::click_at_world_point(app, ON_THE_LEFT_SIDE),
            None,
        ),
    ];
    for (about, choose_the_axis, axis) in ways {
        let test = format!("a_revolution_applied_from_the_panel_about_{about}");
        let mut app = an_area_chosen_to_add_matter(&test);
        click_in_the_panel(&mut app, "Autour d'un axe");
        choose_the_axis(&mut app);

        click_in_the_panel(&mut app, "Appliquer ↵");

        let part = put_away(&mut app, &test);
        let turn = the_last_turn(&part);
        match axis {
            Some(axis) => assert_eq!(
                turn,
                Some((axis, ExtrusionMode::Add)),
                "turned about {about}, the part records another turn",
            ),
            None => {
                let Some((RevolutionAxis::Segment(side), ExtrusionMode::Add)) = turn else {
                    panic!("turned about the side clicked, the part records {turn:?}");
                };
                let (start, end) = part.sketches()[0].endpoints(side);
                assert!(
                    [start, end].iter().all(|end| (end.x + 25.0).abs() < PLACE),
                    "turned about the side clicked, the part turns about the trait from {start} \
                     to {end}",
                );
            }
        }
        assert!(
            part.body().volume() > 0.0,
            "turned about {about}, the part holds no matter",
        );
    }
}

#[test]
fn extruding_a_sketch_opens_the_step_panel_on_that_sketch_with_its_sections() {
    let app = an_area_chosen_to_add_matter("extruding_a_sketch_opens_the_step_panel");

    let read = read_over_the_view(&app);
    let missing: Vec<&str> = THE_STRAIGHT_PANEL
        .into_iter()
        .filter(|text| !read.iter().any(|over| over == text))
        .collect();
    assert!(
        missing.is_empty(),
        "the sketch extruded, no panel over the view reads {missing:?}; over the view: {read:?}, \
         on screen: {:?}, said: {:?}",
        driver::on_screen(&app),
        driver::said(&app),
    );
    assert!(
        read.iter().any(|over| over.contains("Esquisse 1")),
        "the panel does not say it extrudes « Esquisse 1 »; over the view: {read:?}",
    );
}

#[test]
fn extruding_a_sketch_lays_no_row_under_the_ribbon() {
    let app = an_area_chosen_to_add_matter("extruding_a_sketch_lays_no_row");

    let read = read_outside_the_view(&app);
    let left: Vec<&str> = THE_OLD_ROW
        .into_iter()
        .filter(|text| read.iter().any(|outside| outside == text))
        .collect();
    assert!(
        left.is_empty(),
        "an area chosen to extrude, the row under the ribbon still reads {left:?}; outside the \
         view: {read:?}",
    );
}
