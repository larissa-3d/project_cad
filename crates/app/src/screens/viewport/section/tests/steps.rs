//! The section view's panel laid out in numbered steps, driven the way its
//! panel drives it: which step reads done, current or to come, what ↵ does,
//! what the message says, and the section it keeps.
//!
//! Closes #739 — the whole transcription is in
//! `crates/app/src/screens/viewport/input/copying/tests/mirror_and_rectangular_steps.rs`;
//! these are the section view's bullets.
//! - The section panel becomes a step panel, in this order: « Face » (the
//!   origin planes and the clicked face), « Distance », the two tilt angles,
//!   « Devant » / « Derrière » with « Inverser », and « Enregistrer la coupe »
//!   as the main button. Each step reads done, current or to come; a
//!   presenter test checks it —
//!   `the_section_s_steps_read_done_current_or_to_come`; a face clicked in
//!   the view fills its step as a plane chosen does —
//!   `a_face_clicked_in_the_view_fills_the_face_step_and_moves_on_to_the_distance`;
//!   a step reads done only once it holds what it asks for —
//!   `a_face_step_left_waiting_for_a_face_reads_to_come_rather_than_done`;
//!   and a step clicked in the panel becomes current —
//!   `a_step_clicked_in_the_section_s_panel_becomes_current`. Decided on the
//!   way, as the copy panel does: the section opens on its face; a plane
//!   chosen or a face clicked fills the face and moves on to the distance; ↵
//!   moves on one field, the two tilts being two fields of one step; and ↵ on
//!   the last step keeps the section, as its main button does —
//!   `enter_on_the_section_s_last_step_keeps_it_in_the_part`
//! - The message pill follows the section's step; a test on the wording
//!   checks it — `the_message_pill_follows_the_step_of_the_section`; that the
//!   pill shows it, and says none of it once the section is closed, which
//!   only the whole application reaches, in
//!   `crates/app/tests/keeping_a_section.rs` by
//!   `the_title_bar_pill_says_the_section_s_step_until_the_section_is_closed`,
//!   under its own `Closes #739`. Decided on the way: each step says its own
//!   name, as the copy tools' do — « face », « distance », « inclinaisons »,
//!   « devant et derrière »
//! - The values, the section kept in the part (#716, #717) and the handle on
//!   the cutting plane are unchanged; a test compares the section produced
//!   before and after — `a_section_set_through_its_steps_is_kept_as_it_was_before`,
//!   with the values and the places written out rather than read back from
//!   the code, beside the tests already holding the panel's values, the tilts
//!   and the kept section — `tests.rs` above this file, `render/tests.rs`,
//!   `crates/app/tests/keeping_a_section.rs` and `tilting_a_section.rs` —
//!   which stay green

use cao_part::history::SideDrawing;
use cao_part::{ExtrusionMode, Operation, PartDocument, PointRef};
use cao_render::MatterInFront;
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::{DVec2, DVec3};

use super::super::{Section, SectionStep};
use crate::lang::Catalogue;
use crate::screens::sketch::StepState;

const STEP_1: &str = "Vue en coupe — étape 1 : face";
const STEP_2: &str = "Vue en coupe — étape 2 : distance";
const STEP_3: &str = "Vue en coupe — étape 3 : inclinaisons";
const STEP_4: &str = "Vue en coupe — étape 4 : devant et derrière";

/// Rotations and a projection: no solver, a few products.
const TOLERANCE: f64 = 1e-9;

/// A block 10 by 10 from 0 to 4 over XY, raised from a rectangle on XY: its
/// middle is `(5, 5, 2)`.
fn a_block() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 5.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

fn near(found: DVec3, wanted: DVec3) -> bool {
    (found - wanted).length() < TOLERANCE
}

#[test]
fn a_section_set_through_its_steps_is_kept_as_it_was_before() {
    let mut document = a_block();
    assert!(
        (document.scale() - 1.0).abs() < TOLERANCE,
        "a part with no scale of its own counts a millimetre a unit"
    );
    let mut section = Section::through_the_middle(WorkPlane::XY, document.body());

    section.choose_origin_plane(WorkPlane::YZ, document.body());
    section.type_distance(3.0, document.scale());
    section.tilt_about_u_degrees = 30.0;
    section.tilt_about_v_degrees = 60.0;
    section.next_in_front();
    section.invert();
    section.keep_in(&mut document);

    let normal = DVec3::new(3f64.sqrt() / 4.0, 3f64.sqrt() / 2.0, -0.25);
    let handle = DVec3::new(5.0 - 3f64.sqrt() / 2.0, 5.0 - 3f64.sqrt(), 2.5);
    let cut = section.cutting_plane();
    assert!(
        near(cut.normal(), normal),
        "YZ's normal, X, turned 60° about Z and then 30° about Y, is {normal}, \
         not {}",
        cut.normal()
    );
    assert!(
        near(section.handle(document.body()), handle),
        "turned about the middle of the block, YZ stays 5 from it; moved 3 \
         along its normal, the cut passes 2 from it, and the handle stands \
         there, at {handle}, not {}",
        section.handle(document.body())
    );
    assert!(
        near(section.facing(), -normal),
        "« Inverser » turns the arrow against the normal"
    );
    assert_eq!(
        (section.behind, section.in_front),
        (MatterInFront::SeeThrough, MatterInFront::Opaque),
        "the side in front stepped to see-through, then « Inverser » swapped \
         the two"
    );

    let kept = document.sections();
    assert_eq!(kept.len(), 1, "« Enregistrer la coupe » keeps one section");
    let kept = kept[0];
    assert_eq!(
        (kept.plane, kept.on, kept.broken),
        (WorkPlane::YZ, None, false)
    );
    assert!(
        (kept.offset - 3.0).abs() < TOLERANCE,
        "kept 3 along its normal, not {}",
        kept.offset
    );
    assert_eq!(
        (kept.tilt_about_u_degrees, kept.tilt_about_v_degrees),
        (30.0, 60.0)
    );
    assert!(
        near(kept.pivot, DVec3::new(5.0, 5.0, 2.0)),
        "the section is kept tilted about the middle of the block, not {}",
        kept.pivot
    );
    assert_eq!(
        (kept.behind, kept.in_front),
        (SideDrawing::SeeThrough, SideDrawing::Opaque)
    );
    let kept_cut = kept
        .cutting_plane()
        .expect("a section on an origin plane always cuts");
    assert!(
        near(kept_cut.normal(), normal)
            && kept_cut.normal().dot(handle - kept_cut.origin).abs() < TOLERANCE,
        "the part cuts where the section cut on screen: {kept_cut:?}"
    );
}

/// The section « Vue en coupe » opens on the block: along XY, through its
/// middle.
fn opened() -> (PartDocument, Section) {
    let document = a_block();
    let section = Section::through_the_middle(WorkPlane::XY, document.body());
    (document, section)
}

/// The block's top face, as a click on it hands it to the section.
fn the_top() -> WorkPlane {
    WorkPlane::from_normal(DVec3::Z * 4.0, DVec3::Z)
}

#[test]
fn the_section_s_steps_read_done_current_or_to_come() {
    let (mut document, mut section) = opened();
    assert_eq!(
        section.steps(),
        [
            (SectionStep::Face, StepState::Current),
            (SectionStep::Distance, StepState::ToCome),
            (SectionStep::Tilts, StepState::ToCome),
            (SectionStep::Sides, StepState::ToCome),
        ],
        "the section opens on its face, the plane it cuts along"
    );

    section.choose_origin_plane(WorkPlane::YZ, document.body());
    assert_eq!(
        section.steps(),
        [
            (SectionStep::Face, StepState::Done),
            (SectionStep::Distance, StepState::Current),
            (SectionStep::Tilts, StepState::ToCome),
            (SectionStep::Sides, StepState::ToCome),
        ],
        "a plane chosen fills the face, and the distance is asked for"
    );

    section.enter(&mut document);
    let on_the_tilts = [
        (SectionStep::Face, StepState::Done),
        (SectionStep::Distance, StepState::Done),
        (SectionStep::Tilts, StepState::Current),
        (SectionStep::Sides, StepState::ToCome),
    ];
    assert_eq!(
        section.steps(),
        on_the_tilts,
        "↵ on the distance moves on to the tilts"
    );

    section.enter(&mut document);
    assert_eq!(
        section.steps(),
        on_the_tilts,
        "the two tilts are two fields of one step: ↵ on the first moves on to \
         the second"
    );

    section.enter(&mut document);
    assert_eq!(
        section.steps(),
        [
            (SectionStep::Face, StepState::Done),
            (SectionStep::Distance, StepState::Done),
            (SectionStep::Tilts, StepState::Done),
            (SectionStep::Sides, StepState::Current),
        ],
        "↵ on the second tilt moves on to the sides"
    );
    assert!(document.sections().is_empty(), "moving on keeps nothing");
}

#[test]
fn a_face_clicked_in_the_view_fills_the_face_step_and_moves_on_to_the_distance() {
    let (_, mut section) = opened();

    section.wait_for_a_face();
    assert_eq!(
        section.steps()[0],
        (SectionStep::Face, StepState::Current),
        "« Face cliquée » chosen, the face is still asked for until one is \
         clicked"
    );
    section.choose_face(the_top());

    assert_eq!(
        section.steps(),
        [
            (SectionStep::Face, StepState::Done),
            (SectionStep::Distance, StepState::Current),
            (SectionStep::Tilts, StepState::ToCome),
            (SectionStep::Sides, StepState::ToCome),
        ]
    );
}

#[test]
fn a_face_step_left_waiting_for_a_face_reads_to_come_rather_than_done() {
    let (_, mut section) = opened();
    section.wait_for_a_face();

    section.activate(SectionStep::Distance);

    assert_eq!(
        section.steps(),
        [
            (SectionStep::Face, StepState::ToCome),
            (SectionStep::Distance, StepState::Current),
            (SectionStep::Tilts, StepState::ToCome),
            (SectionStep::Sides, StepState::ToCome),
        ],
        "no face is clicked yet, and a check on the face would tell the user \
         the section cuts along one"
    );
}

#[test]
fn a_step_clicked_in_the_section_s_panel_becomes_current() {
    let (_, mut section) = opened();

    section.activate(SectionStep::Sides);
    assert_eq!(
        section.steps(),
        [
            (SectionStep::Face, StepState::Done),
            (SectionStep::Distance, StepState::Done),
            (SectionStep::Tilts, StepState::Done),
            (SectionStep::Sides, StepState::Current),
        ],
        "the steps above hold what the section opened on"
    );

    section.activate(SectionStep::Distance);
    assert_eq!(
        section.steps(),
        [
            (SectionStep::Face, StepState::Done),
            (SectionStep::Distance, StepState::Current),
            (SectionStep::Tilts, StepState::ToCome),
            (SectionStep::Sides, StepState::ToCome),
        ],
        "a step clicked again, to change what it holds"
    );
}

#[test]
fn enter_on_the_section_s_last_step_keeps_it_in_the_part() {
    let (mut document, mut section) = opened();
    section.type_distance(1.5, document.scale());
    section.activate(SectionStep::Sides);

    let kept = section.enter(&mut document);

    assert!(kept, "↵ on the sides says the section was kept");
    let sections = document.sections();
    assert_eq!(
        sections.len(),
        1,
        "one ↵ on the last step keeps the section, as « Enregistrer la coupe » \
         does"
    );
    assert_eq!(sections[0].plane, WorkPlane::XY);
    assert!(
        (sections[0].offset - 1.5).abs() < TOLERANCE,
        "kept at the distance typed, not {}",
        sections[0].offset
    );
}

#[test]
fn the_message_pill_follows_the_step_of_the_section() {
    let lang = Catalogue::french();
    let (mut document, mut section) = opened();

    assert_eq!(section.said(&lang), STEP_1);
    section.wait_for_a_face();
    assert_eq!(
        section.said(&lang),
        STEP_1,
        "« Face cliquée » chosen, the face is still asked for"
    );
    section.choose_face(the_top());
    assert_eq!(section.said(&lang), STEP_2);
    section.enter(&mut document);
    assert_eq!(section.said(&lang), STEP_3);
    section.enter(&mut document);
    assert_eq!(
        section.said(&lang),
        STEP_3,
        "the two tilts are one step: the second says what the first said"
    );
    section.enter(&mut document);
    assert_eq!(section.said(&lang), STEP_4);
    section.activate(SectionStep::Distance);
    assert_eq!(
        section.said(&lang),
        STEP_2,
        "a step clicked in the panel takes the message with it"
    );
}
