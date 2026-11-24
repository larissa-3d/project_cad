//! The piece of the ball a fill is trimmed by at a sphere end (#941), read on
//! [`within`], [`nearest`] and [`in_the_disc`] themselves, and the trim
//! [`outside_sphere`] makes of it. The box the trim starts from less any
//! piece holding what of the ball the box takes in is one solid, so the body
//! a fill leaves cannot tell one such piece from a larger one: only the
//! reach the piece carries can, and since a fill leaves its part its own
//! reach (#1023) that shows on the trim alone. Here the piece's own shape is
//! held — what it holds, and how far past the box it stands — on a ball of
//! radius 100 about the origin turned about Z, and boxes square to its axis
//! with a snug of a quarter; and the snug the trim gives it, by the reach the
//! trim carries.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::{in_the_disc, nearest, outside_sphere, within};
use crate::body::exact::Exact;
use crate::body::exact::wedge::end::CLEAR;
use crate::brep::Sphere;
use crate::profile::{Contour, Frame, Run};
use crate::turning::{Axis, ON_THE_AXIS, Straight, Turn};

/// How far a corner of a profile may stand from where the arithmetic puts
/// it: each is placed by a root and a product, so what is left is rounding.
const PLACE: f64 = 1e-12;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far past the box the piece stands: a quarter of a clearance of one.
const SNUG: f64 = 0.25;

/// A ball of `radius` about the origin, turned about Z.
fn ball_of(radius: f64) -> Sphere {
    Sphere::through(DVec3::ZERO, DVec3::Z, 0.0, radius)
}

/// The eight corners of the box from `low` to `high`, square to the world's
/// axes: what [`within`] is handed, the corners of the box a trim starts
/// from.
fn box_corners(low: DVec3, high: DVec3) -> Vec<DVec3> {
    (0..8)
        .map(|corner| {
            DVec3::select(
                glam::BVec3::new(corner & 1 != 0, corner & 2 != 0, corner & 4 != 0),
                high,
                low,
            )
        })
        .collect()
}

/// The piece of the ball of radius 100 round the box from `low` to `high`.
fn piece_round(low: DVec3, high: DVec3) -> Exact {
    within(&ball_of(100.0), &box_corners(low, high), SNUG, 1.0)
        .unwrap_or_else(|declined| panic!("the piece round {low} to {high}: {declined:?}"))
}

/// The matter of the solid turned `angle` about Z from a rectangle `along`
/// it and `[inner, outer]` from it, by Pappus.
fn turned_volume(angle: f64, along: f64, [inner, outer]: [f64; 2]) -> f64 {
    angle / 2.0 * (outer * outer - inner * inner) * along
}

/// Holds that a piece holds `promised`, within [`EXACT`] of it.
fn holds(piece: &Exact, promised: f64, what: &str) {
    let volume = piece.volume();
    assert!(
        (volume - promised).abs() <= EXACT * promised,
        "{what}: the piece holds {volume}, where arithmetic promised {promised}"
    );
}

/// Holds that `got` is the profile of `corners`, each run from one to the
/// next straight where `turns` says none, and round about the origin by the
/// turn it says otherwise.
fn the_profile_is(got: Option<Contour>, corners: &[DVec2], turns: &[Option<f64>], what: &str) {
    let got = got.unwrap_or_else(|| panic!("{what}: no profile"));
    assert_eq!(
        got.corners.len(),
        corners.len(),
        "{what}: the corners {:?}, where {corners:?} were promised",
        got.corners
    );
    for (rank, (&at, &promised)) in got.corners.iter().zip(corners).enumerate() {
        assert!(
            at.distance(promised) < PLACE,
            "{what}: corner {rank} at {at}, where {promised} was promised: {:?}",
            got.corners
        );
    }
    for (rank, (run, &turn)) in got.runs.iter().zip(turns).enumerate() {
        match (run, turn) {
            (Run::Straight, None) => {}
            (Run::Round { center, turn: got }, Some(turn)) => assert!(
                center.length() < PLACE && (got - turn).abs() < PLACE,
                "{what}: run {rank} turns {got} about {center}, where {turn} about the origin \
                 was promised"
            ),
            _ => panic!("{what}: run {rank} is {run:?}, where {turn:?} was promised"),
        }
    }
}

#[test]
fn the_nearest_a_box_beside_the_axis_comes_is_where_its_side_passes_it() {
    let flats = [
        DVec3::new(3.0, -4.0, 0.0),
        DVec3::new(3.0, 4.0, 0.0),
        DVec3::new(6.0, -4.0, 0.0),
        DVec3::new(6.0, 4.0, 0.0),
    ];

    let nearest = nearest(&flats);

    assert!(
        (nearest - 3.0).abs() < PLACE,
        "a box whose side runs 3 from the axis, its corners 5 from it, comes {nearest} near it"
    );
}

#[test]
fn the_nearest_a_box_comes_is_its_corner_where_its_sides_run_away_from_the_axis() {
    let flats = [
        DVec3::new(3.0, 4.0, 0.0),
        DVec3::new(6.0, 4.0, 0.0),
        DVec3::new(6.0, 8.0, 0.0),
        DVec3::new(3.0, 8.0, 0.0),
    ];

    let nearest = nearest(&flats);

    assert!(
        (nearest - 5.0).abs() < PLACE,
        "a box whose nearest corner stands 5 from the axis comes {nearest} near it"
    );
}

#[test]
fn a_box_whose_face_holds_the_axis_comes_right_up_to_it() {
    let flats: Vec<DVec3> = [[0.0, -1.0], [0.0, 5.0], [3.0, -1.0], [3.0, 5.0]]
        .into_iter()
        .flat_map(|[x, y]| [DVec3::new(x, y, 0.0); 2])
        .collect();

    let nearest = nearest(&flats);

    assert!(
        nearest.abs() < PLACE,
        "a box whose face holds the axis comes {nearest} near it"
    );
}

#[test]
fn a_box_beside_the_axis_is_held_by_a_piece_turned_between_planes_through_the_axis_a_snug_past_it()
{
    let (low, high) = (DVec3::new(3.0, -1.0, 2.0), DVec3::new(5.0, 1.0, 6.0));

    let piece = piece_round(low, high);

    let near = 3f64.hypot(1.0);
    let far = 5f64.hypot(1.0);
    let angle = 2.0 * ((1f64 / 3.0).atan() + (SNUG / near).asin());
    holds(
        &piece,
        turned_volume(angle, 4.0 + 2.0 * SNUG, [3.0 - SNUG, far + SNUG]),
        &format!(
            "a box 3 to 5 from the axis and 1 either side of it: the piece turned {angle} about \
             it, from {} to {} off it",
            3.0 - SNUG,
            far + SNUG
        ),
    );
    let end = piece.faces_end();
    assert!(
        !piece.has_face(end) && !piece.has_face(end + 1),
        "the piece's faces answer to numbers below where its count ends, {end}"
    );
}

#[test]
fn a_box_whose_face_holds_the_axis_is_held_by_a_piece_turned_whole_from_the_axis() {
    let (low, high) = (DVec3::new(0.0, -1.0, 2.0), DVec3::new(3.0, 5.0, 6.0));

    let piece = piece_round(low, high);

    let far = 3f64.hypot(5.0);
    holds(
        &piece,
        turned_volume(TAU, 4.0 + 2.0 * SNUG, [0.0, far + SNUG]),
        "a box whose face x = 0 holds the axis: the piece turned whole, out to a snug past its \
         furthest corner",
    );
}

#[test]
fn a_box_round_the_axis_off_its_diagonals_is_held_by_a_piece_turned_whole_from_the_axis() {
    let (low, high) = (DVec3::new(-4.0, -3.0, 2.0), DVec3::new(6.0, 7.0, 6.0));

    let piece = piece_round(low, high);

    let far = 6f64.hypot(7.0);
    holds(
        &piece,
        turned_volume(TAU, 4.0 + 2.0 * SNUG, [0.0, far + SNUG]),
        "a box round the axis, whose diagonals pass it 0.71 and 2.1 off, further than its snug: \
         the piece turned whole, out to a snug past its furthest corner",
    );
}

#[test]
fn a_box_a_hair_further_from_the_axis_than_its_snug_is_held_by_a_piece_turned_whole() {
    let (low, high) = (DVec3::new(0.255, -1.0, 2.0), DVec3::new(5.0, 1.0, 6.0));
    let outer = 5f64.hypot(1.0) + SNUG;
    assert!(
        0.255 - SNUG <= ON_THE_AXIS * outer,
        "the fixture: the box's side stands within the axis's band of a snug from it"
    );

    let piece = piece_round(low, high);

    holds(
        &piece,
        turned_volume(TAU, 4.0 + 2.0 * SNUG, [0.0, outer]),
        "a box whose side stands 0.255 from the axis: a cylinder a snug inside it would stand on \
         the axis within its band, so the piece is turned whole, out to a snug past its furthest \
         corner",
    );
}

/// The whole ball of `sphere`, its half disc turned about its own axis: what
/// a fill was trimmed by before #941.
fn whole_ball(sphere: &Sphere) -> Exact {
    let (low, high) = (sphere.center - sphere.radius, sphere.center + sphere.radius);
    let half_disc = Contour {
        corners: vec![DVec2::new(low, 0.0), DVec2::new(high, 0.0)],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: DVec2::new(sphere.center, 0.0),
                turn: PI,
            },
        ],
    };
    let frame = Frame {
        origin: sphere.origin,
        u: sphere.axis,
        v: sphere.u,
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::X,
        },
        angle: TAU,
        resolution: 0.0,
        on_the_axis: ON_THE_AXIS * sphere.radius,
    };
    let straight = Straight::of(&half_disc, &[], frame, &turn, 1.0).expect("the half disc lays");
    Exact::turned(&straight, frame, &turn, straight.numbers(true)).expect("the ball turns")
}

#[test]
fn a_box_standing_out_of_the_ball_loses_to_the_piece_what_it_loses_to_the_whole_ball() {
    let sphere = ball_of(5.0);
    let ball = whole_ball(&sphere);
    for (low, high, what) in [
        (
            DVec3::new(3.0, -1.0, -1.0),
            DVec3::new(7.0, 1.0, 1.0),
            "a box beside the axis",
        ),
        (
            DVec3::new(0.0, -1.0, -1.0),
            DVec3::new(7.0, 1.0, 1.0),
            "a box whose face holds the axis",
        ),
    ] {
        let rectangle = Contour::rectangle(low.truncate(), high.truncate());
        let frame = Frame {
            origin: DVec3::Z * low.z,
            u: DVec3::X,
            v: DVec3::Y,
        };
        let solid = Exact::raised(&rectangle, &[], frame, DVec3::Z * (high.z - low.z))
            .expect("the box raises");
        let piece = within(&sphere, &box_corners(low, high), SNUG, 1.0)
            .unwrap_or_else(|declined| panic!("{what}: the piece declined, {declined:?}"));

        let trimmed = solid.cut_by(&piece).expect("the piece cuts the box");
        let by_the_ball = solid.cut_by(&ball).expect("the ball cuts the box");

        let promised = by_the_ball.volume();
        holds(
            &trimmed,
            promised,
            &format!("{what}, from {low} to {high}, less the piece, against less the whole ball"),
        );
    }
}

#[test]
fn a_rectangle_the_circle_crosses_keeps_what_lies_inside_and_runs_on_along_the_circle() {
    let profile = in_the_disc([3.0, 6.0], [0.0, 4.0], DVec2::ZERO, 5.0);

    the_profile_is(
        profile,
        &[
            DVec2::new(3.0, 0.0),
            DVec2::new(5.0, 0.0),
            DVec2::new(3.0, 4.0),
        ],
        &[None, Some(4f64.atan2(3.0)), None],
        "a rectangle from 3 to 6 along the axis and out to 4, in a disc of 5",
    );
}

#[test]
fn a_rectangle_whose_far_side_is_tangent_to_the_circle_leaves_it_to_the_circle() {
    let profile = in_the_disc([-3.0, 3.0], [0.0, 5.0], DVec2::ZERO, 5.0);

    the_profile_is(
        profile,
        &[
            DVec2::new(-3.0, 0.0),
            DVec2::new(3.0, 0.0),
            DVec2::new(3.0, 4.0),
            DVec2::new(-3.0, 4.0),
        ],
        &[None, None, Some(PI - 2.0 * 4f64.atan2(3.0)), None],
        "a rectangle from -3 to 3 along the axis and out to 5, tangent to a disc of 5",
    );
}

#[test]
fn a_rectangle_from_the_axis_round_the_whole_disc_is_the_half_disc() {
    let profile = in_the_disc([-6.0, 6.0], [0.0, 8.0], DVec2::ZERO, 5.0);

    the_profile_is(
        profile,
        &[DVec2::new(-5.0, 0.0), DVec2::new(5.0, 0.0)],
        &[None, Some(PI)],
        "a rectangle from the axis holding the whole disc of 5",
    );
}

#[test]
fn a_rectangle_inside_the_disc_is_itself() {
    let profile = in_the_disc([-1.0, 1.0], [1.0, 2.0], DVec2::ZERO, 5.0);

    the_profile_is(
        profile,
        &[
            DVec2::new(-1.0, 1.0),
            DVec2::new(1.0, 1.0),
            DVec2::new(1.0, 2.0),
            DVec2::new(-1.0, 2.0),
        ],
        &[None; 4],
        "a rectangle well inside a disc of 5",
    );
}

#[test]
fn a_rectangle_meeting_the_disc_at_a_corner_alone_or_not_at_all_has_no_profile() {
    for (heights, radii, what) in [
        (
            [3.0, 6.0],
            [4.0, 8.0],
            "a rectangle whose corner (3, 4) alone lies on the circle",
        ),
        ([6.0, 8.0], [0.0, 1.0], "a rectangle outside the disc"),
    ] {
        let profile = in_the_disc(heights, radii, DVec2::ZERO, 5.0);

        assert!(
            profile.is_none(),
            "{what}: no profile, where it gave {profile:?}"
        );
    }
}

/// The prism of a fill running out of a ball of 10 about (0, 0, −100),
/// turned about Z, along X at its equator: a right triangle, its legs 1
/// along Y and up Z from the line through the centre, raised 15 along X
/// from the plane through the centre square to it. Every corner of the
/// section stands within 1 of the edge, so the trim stands clear of the
/// prism by [`CLEAR`].
fn running_out_of_a_ball_far_down() -> (Sphere, (Contour, Frame, DVec3)) {
    let centre = DVec3::new(0.0, 0.0, -100.0);
    let section = Contour {
        corners: vec![DVec2::ZERO, DVec2::X, DVec2::Y],
        runs: vec![Run::Straight; 3],
    };
    let frame = Frame {
        origin: centre,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    (
        Sphere::through(centre, DVec3::Z, 0.0, 10.0),
        (section, frame, DVec3::X * 15.0),
    )
}

#[test]
fn the_trim_at_a_sphere_end_reaches_a_quarter_of_a_clearance_past_the_box_it_starts_from() {
    let (sphere, prism) = running_out_of_a_ball_far_down();

    let trim = outside_sphere(&prism, &sphere, DVec3::X, 100.0)
        .unwrap_or_else(|declined| panic!("the trim at the ball's end: {declined:?}"));

    let floor = 100.0 + CLEAR;
    let promised = floor + CLEAR / 4.0;
    let reach = trim.reach();
    assert!(
        (reach - promised).abs() < PLACE,
        "the trim at the end of a fill running out of a ball of 10 at its equator, 100 down: it \
         carries a reach of {reach}, where the floor of the box it starts from stands {floor} \
         down, a clearance under the prism, and the floor of the piece of the ball it takes \
         from the box a quarter of a clearance under that, at {promised}"
    );
}
