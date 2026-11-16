//! What a body answers about the matter it holds, whichever kernel computed
//! it.
//!
//! Closes #805 — its second slice's faces only. The relation and the
//! crossings they stand on are answered in `brep/relation/tests/round.rs` and
//! `brep/relation/tests/crossing/pierced.rs`, each file with its own criteria.
//! - A face of the sphere or of the cylinder bounded by the curve a cylinder
//!   of another axis meets a sphere along is traced and drawn (tessellated,
//!   closed outline) — where the cylinder passes right through the ball,
//!   `a_ball_bored_right_through_off_its_axis_is_built_and_drawn_within_the_drawing_s_tolerance`;
//!   where it takes in part of it,
//!   `a_ball_notched_by_a_cylinder_off_its_axis_is_built_and_drawn_within_the_drawing_s_tolerance`
//!
//! Read as: the ball less the cylinder built exact, holding the ball less
//! what the cylinder holds inside it, which a quadrature gives; its triangles
//! closed, uncrossed and enclosing that within the drawing's tolerance; for a
//! cylinder along the ball's axis, square to it and at a slant, each loop
//! clear of the poles of the ball's chart. Three declines that pinned a ball
//! bored off its axis are turned round with it, each now holding the
//! arithmetic's volume: `brep/combine/tests/round.rs`'s hole through a ball,
//! and in `cao_part`, the hole drilled through a turned ball and the bowed
//! turn whose sphere crosses a raised cylinder off its centre. A ring bored
//! off its axis stays declined until the third slice, and stands in for the
//! ball wherever a declined step was wanted: under the slanted circle of
//! `cao_part`'s `changing_an_extrusion.rs` and `cao_app`'s `the_alerts.rs`,
//! and as the broken step of `the_exact_kernel_turns_an_arc.rs`; the bowed
//! turn's broken step is kept beside an elliptic shaft.
//!
//! Beside the two criteria's balls, the review's cases: a bead drilled a
//! little off its centre, whose loops wind round the ball's axis; a bore
//! counterbored about its own axis or touching it along a ruling that ends
//! on the equator; a shell and two balls of one radius bored by one hole,
//! whose spheres share a centre or a radius and none of each other's loops;
//! and the same hole bored twice, and the plug a bore takes out joined back,
//! which meet the bore's loops a second time.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::mesh::tests::fan;
use crate::profile::{Contour, Frame, Run};
use crate::sweep::Loop;
use crate::turning::{Axis, Turn};

const FLAT: Frame = Frame {
    origin: DVec3::ZERO,
    u: DVec3::X,
    v: DVec3::Y,
};

const TOP: usize = 1;

fn exact(outline: Contour, holes: Vec<Contour>) -> Profile<'static> {
    Profile {
        exact: Some((outline, holes)),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

fn disc(center: DVec2, radius: f64) -> Contour {
    Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    }
}

fn raised(part: &Body, profile: &Profile, height: f64) -> Body {
    part.tool_raised(profile, FLAT, DVec3::Z * height)
        .expect("the kernel raises it")
}

#[test]
fn a_circle_raised_encloses_the_arithmetics_volume() {
    let body = raised(
        &Body::default(),
        &exact(disc(DVec2::ZERO, 20.0), vec![]),
        10.0,
    );

    let arithmetic = PI * 20.0 * 20.0 * 10.0;
    assert!(
        (body.volume() - arithmetic).abs() < 1e-9 * arithmetic,
        "{} against {arithmetic}",
        body.volume(),
    );
}

fn block(low: DVec2, high: DVec2, height: f64) -> Body {
    raised(
        &Body::default(),
        &exact(Contour::rectangle(low, high), vec![]),
        height,
    )
}

fn bored_block() -> Body {
    let block = block(DVec2::ZERO, DVec2::new(30.0, 20.0), 10.0);
    let bore = raised(
        &block,
        &exact(disc(DVec2::new(15.0, 10.0), 4.0), vec![]),
        10.0,
    );
    block.difference(&bore).expect("the kernel bores it")
}

#[test]
fn every_triangle_drawn_belongs_to_a_face_of_the_exact_body() {
    let body = bored_block();

    let drawn = body.triangles();
    let by_face: Vec<[DVec3; 3]> = (0..body.faces_end())
        .flat_map(|face| body.triangles_of(face).collect::<Vec<_>>())
        .collect();

    assert!(!drawn.is_empty());
    assert_eq!(by_face.len(), drawn.len(), "each triangle on one face");
    assert!(drawn.iter().all(|triangle| by_face.contains(triangle)));
    assert!(
        (0..body.faces_end())
            .all(|face| !body.has_face(face) || body.triangles_of(face).count() > 0),
        "every face the body has is drawn",
    );
    assert_eq!(body.triangles_of(body.faces_end()).count(), 0);
}

#[test]
fn a_ray_meets_the_face_the_exact_body_has_there_with_its_exact_normal() {
    let post = raised(
        &Body::default(),
        &exact(disc(DVec2::ZERO, 10.0), vec![]),
        10.0,
    );
    let wall = 2;

    let hit = post
        .ray_hit(DVec3::new(20.0, 0.3, 5.0), DVec3::NEG_X)
        .expect("the post's wall");

    let on_the_circle = DVec3::new((100.0f64 - 0.09).sqrt(), 0.3, 0.0);
    assert_eq!(hit.face, wall);
    assert!(
        (hit.distance - (20.0 - on_the_circle.x)).abs() < 1e-3 * 10.0,
        "{}",
        hit.distance,
    );
    assert!(
        hit.normal.dot(on_the_circle / 10.0) > (1e-4f64).cos(),
        "the circle's own normal, not a facet's: {}",
        hit.normal,
    );

    let top = post
        .ray_hit(DVec3::new(1.0, 2.0, 30.0), DVec3::NEG_Z)
        .expect("the post's top");
    assert_eq!(top.face, TOP);
    assert_eq!(top.normal, DVec3::Z);
}

#[test]
fn a_ray_inside_a_bore_meets_its_wall_facing_back_at_it() {
    let body = bored_block();
    let center = DVec3::new(15.0, 10.0, 5.0);

    let hit = body.ray_hit(center, DVec3::X).expect("the bore's wall");

    assert!(body.has_face(hit.face) && !body.is_flat(hit.face));
    assert!(
        hit.normal.dot(DVec3::NEG_X) > (1e-4f64).cos(),
        "{}",
        hit.normal
    );
}

#[test]
fn the_top_of_a_raised_profile_offers_its_exact_plane_and_its_corners() {
    let post = raised(
        &Body::default(),
        &exact(disc(DVec2::ZERO, 20.0), vec![]),
        10.0,
    );

    let plane = post.plane_of(TOP).expect("the post has a top");

    assert!(post.is_flat(TOP));
    assert_eq!(plane.normal, DVec3::Z);
    for rim in [
        DVec3::new(20.0, 0.0, 10.0),
        DVec3::new(0.0, 20.0, 10.0),
        DVec3::new(-20.0, 0.0, 10.0),
        DVec3::new(0.0, -20.0, 10.0),
    ] {
        assert!(
            plane
                .corners
                .iter()
                .any(|corner| corner.distance(rim) < 1e-12),
            "{rim} among {:?}",
            plane.corners,
        );
    }
    assert!(plane.corners.iter().all(|corner| corner.z == 10.0));
    assert_eq!(post.plane_of(post.faces_end()), None);
}

#[test]
fn a_wall_raised_from_an_arc_is_not_flat() {
    let post = raised(
        &Body::default(),
        &exact(disc(DVec2::ZERO, 20.0), vec![]),
        10.0,
    );
    let wall = 2;

    assert!(post.has_face(wall));
    assert!(!post.is_flat(wall));
    assert_eq!(post.plane_of(wall), None);
}

fn polygon_area(points: &[DVec2]) -> f64 {
    (0..points.len())
        .map(|index| points[index].perp_dot(points[(index + 1) % points.len()]))
        .sum::<f64>()
        / 2.0
}

fn sampled_disc(center: DVec2, radius: f64) -> (Vec<DVec2>, Vec<Option<usize>>, Vec<[DVec2; 3]>) {
    let points: Vec<DVec2> = (0..48)
        .map(|step| center + DVec2::from_angle(TAU * step as f64 / 48.0) * radius)
        .collect();
    let curves = vec![Some(0); points.len()];
    let triangles = (1..points.len() - 1)
        .map(|index| [points[0], points[index], points[index + 1]])
        .collect();
    (points, curves, triangles)
}

fn ring_turned_about_y() -> Body {
    let outline = [
        DVec2::new(30.0, 0.0),
        DVec2::new(34.0, 0.0),
        DVec2::new(34.0, 4.0),
        DVec2::new(30.0, 4.0),
    ];
    let triangles = [
        [outline[0], outline[1], outline[2]],
        [outline[0], outline[2], outline[3]],
    ];
    let profile = Profile {
        exact: None,
        sampled: Loop::straight(&outline),
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let ring = Body::default()
        .tool_turned(&profile, FLAT, &about_v(&profile, TAU))
        .expect("the flats never decline");
    assert!(!ring.is_empty(), "a ring on one side of its axis");
    ring
}

#[test]
fn a_revolution_joined_to_an_exact_body_is_flats_from_then_on() {
    let block = block(DVec2::ZERO, DVec2::splat(10.0), 10.0);
    let ring = ring_turned_about_y();
    let part = block.union(&ring).expect("the flats never decline");

    assert!(
        (part.volume() - (1000.0 + ring.volume())).abs() < 1e-6,
        "{} against {}",
        part.volume(),
        1000.0 + ring.volume(),
    );

    let (points, curves, triangles) = sampled_disc(DVec2::splat(5.0), 3.0);
    let profile = Profile {
        exact: Some((disc(DVec2::splat(5.0), 3.0), vec![])),
        sampled: Loop {
            points: &points,
            curves: &curves,
        },
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let boss = part
        .tool_raised(&profile, FLAT, DVec3::Z * 20.0)
        .expect("the flats never decline");
    let flats = polygon_area(&points) * 20.0;
    assert!(
        (boss.volume() - flats).abs() < 1e-9 * flats,
        "raised by the flats: {} against {flats}",
        boss.volume(),
    );
}

#[test]
fn a_profile_the_exact_kernel_cannot_raise_is_raised_by_the_flats() {
    let part = block(DVec2::ZERO, DVec2::splat(10.0), 10.0);
    let (points, curves, triangles) = sampled_disc(DVec2::splat(5.0), 3.0);
    let oval = Profile {
        exact: None,
        sampled: Loop {
            points: &points,
            curves: &curves,
        },
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };

    let boss = part
        .tool_raised(&oval, FLAT, DVec3::Z * 20.0)
        .expect("the flats never decline");
    let joined = part.union(&boss).expect("the flats never decline");

    let flats = polygon_area(&points) * 10.0;
    assert!(
        (joined.volume() - (1000.0 + flats)).abs() < 1e-6,
        "{} against {}",
        joined.volume(),
        1000.0 + flats,
    );
}

#[test]
fn an_exact_body_behind_a_plane_keeps_only_what_is_behind_it() {
    let body = bored_block();

    let behind = body.behind(DVec3::Z, 4.0);

    let (low, high) = behind.bounds().expect("something is left");
    assert!(high.z <= 4.0 + 1e-9 && low.z >= -1e-9, "{low} {high}");
    assert!((high.x - 30.0).abs() < 1e-9 && (high.y - 20.0).abs() < 1e-9);
    let bore = behind
        .ray_hit(DVec3::new(15.0, 10.0, 2.0), DVec3::X)
        .expect("the bore's wall, below the plane");
    assert!(
        (bore.distance - 4.0).abs() < 1e-3 * 30.0,
        "{}",
        bore.distance
    );
    assert_eq!(behind.ray_hit(DVec3::new(15.0, 10.0, 6.0), DVec3::X), None);
}

/// A ball of radius 5 about the origin: a half disc turned whole about Y.
fn ball_about_y() -> Body {
    ball_turned_about_y(TAU)
}

/// The half disc of radius 5 on the side x ≥ 0 of the plane z = 0, turned
/// `turn` about Y.
fn ball_turned_about_y(turn: f64) -> Body {
    ball_turned_about_y_of(5.0, DVec3::ZERO, turn)
}

/// A ball of `radius` about `at`: a half disc turned whole about the line
/// through `at` along Y.
fn ball_about_y_of(radius: f64, at: DVec3) -> Body {
    ball_turned_about_y_of(radius, at, TAU)
}

/// The half disc of `radius` on the side x ≥ 0 about `at`, turned `turn`
/// about the line through `at` along Y.
fn ball_turned_about_y_of(radius: f64, at: DVec3, turn: f64) -> Body {
    let sampled: &'static [DVec2] = Box::leak(
        (0..=32)
            .map(|step| DVec2::from_angle(-PI / 2.0 + PI * f64::from(step) / 32.0) * radius)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let half = Contour {
        corners: vec![DVec2::new(0.0, -radius), DVec2::new(0.0, radius)],
        runs: vec![
            Run::Round {
                center: DVec2::ZERO,
                turn: PI,
            },
            Run::Straight,
        ],
    };
    let profile = Profile {
        exact: Some((half, Vec::new())),
        sampled: Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    Body::default()
        .tool_turned(
            &profile,
            Frame { origin: at, ..FLAT },
            &about_v(&profile, turn),
        )
        .expect("the exact kernel turns the ball")
}

/// The box from `low` to `high`, raised along Z.
fn boxed(low: DVec3, high: DVec3) -> Body {
    Body::default()
        .tool_raised(
            &exact(Contour::rectangle(low.truncate(), high.truncate()), vec![]),
            Frame {
                origin: DVec3::Z * low.z,
                ..FLAT
            },
            DVec3::Z * (high.z - low.z),
        )
        .expect("the kernel raises it")
}

/// A slab 10 thick along `normal` from the plane `offset` along it, its
/// sides 20 from the normal's line either way: past the plane, it holds all
/// of a ball of radius 5 about the origin.
fn slab_along(normal: DVec3, offset: f64) -> Body {
    let normal = normal.normalize();
    let u = normal.any_orthonormal_vector();
    Body::default()
        .tool_raised(
            &exact(
                Contour::rectangle(DVec2::splat(-20.0), DVec2::splat(20.0)),
                vec![],
            ),
            Frame {
                origin: normal * offset,
                u,
                v: normal.cross(u),
            },
            normal * 10.0,
        )
        .expect("the slab raises")
}

/// The ball of radius 5, and what a cap of it `height` high holds.
const BALL_OF_5: f64 = 500.0 * PI / 3.0;

fn cap_of_5(height: f64) -> f64 {
    PI * height * height * (15.0 - height) / 3.0
}

/// The ball cut by a slab at a slant whose face stands `offset` along its
/// normal, (1, 1, 0), from the ball's centre, the slab holding what lies past
/// the face; and the matter the arithmetic leaves, the ball less that cap.
/// The ball's pole at y = 5 stands 5/√2 along the normal.
fn cut_at_a_slant(offset: f64) -> (Result<Body, Declined>, f64) {
    let slab = slab_along(DVec3::new(1.0, 1.0, 0.0), offset);
    (
        ball_about_y().difference(&slab),
        BALL_OF_5 - cap_of_5(5.0 - offset),
    )
}

/// Holds that a body cut from the ball is exact, holds `kept`, and that its
/// triangles are closed, uncrossed and enclose that within what the drawing
/// promises over its curved faces, `curved` in all: each triangle within the
/// drawing's tolerance of its face, at the reach of the body's own box.
fn built_and_drawn(body: &Body, kept: f64, curved: f64, what: &str) {
    assert!(
        body.is_exact() && (body.volume() - kept).abs() <= 1e-9 * kept,
        "{what}: holds {}, where the arithmetic leaves {kept}",
        body.volume()
    );
    drawn_within(body, kept, curved, what);
}

/// Holds that a body's triangles are closed, uncrossed and enclose `kept`
/// within what the drawing promises over its curved faces, `curved` in all,
/// at the reach of the body's own box.
fn drawn_within(body: &Body, kept: f64, curved: f64, what: &str) {
    let triangles = body.triangles();
    assert_eq!(crate::soundness::closed(&triangles), Ok(()), "{what}");
    assert_eq!(crate::soundness::uncrossed(&triangles), Ok(()), "{what}");
    let (low, high) = body.bounds().expect("a body has a box");
    let reach = crate::brep::Scale::of(low.abs().max(high.abs()).max_element()).reach();
    let promised = super::drawn::DRAWN * reach * curved;
    let enclosed = crate::soundness::enclosed(&triangles);
    assert!(
        (enclosed - kept).abs() <= promised,
        "{what}: its triangles enclose {enclosed}, where it holds {kept}, more than the \
         {promised} the drawing's tolerance gives"
    );
}

/// The circle a slab at a slant leaves on the ball passes its chart's pole
/// as near as its face does, through it where the face stands on it: near or
/// through, the cut is built, holds the matter the arithmetic leaves, and its
/// triangles are closed and uncrossed.
#[test]
fn a_ball_cut_at_a_slant_through_or_close_by_a_pole_of_its_chart_holds_the_matter_the_arithmetic_leaves()
 {
    let pole = 5.0 / 2f64.sqrt();
    for off in [
        -0.5, -0.1, -0.05, -0.01, -1e-4, 0.0, 1e-4, 0.01, 0.05, 0.1, 0.5,
    ] {
        let (cut, kept) = cut_at_a_slant(pole + off);
        let what = format!("the ball cut at a slant {off} along the normal off its pole");
        let body = cut.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
        assert!(
            body.is_exact() && (body.volume() - kept).abs() <= 1e-9 * kept,
            "{what}: holds {}, where the arithmetic leaves {kept}",
            body.volume()
        );
        let triangles = body.triangles();
        assert_eq!(crate::soundness::closed(&triangles), Ok(()), "{what}");
        assert_eq!(crate::soundness::uncrossed(&triangles), Ok(()), "{what}");
    }
}

/// Wherever the slab's face stands, clear of the pole, through the centre
/// or off it, keeping one pole of the ball's chart or both: the cut is built
/// and drawn as [`built_and_drawn`] holds, its sphere 2πr (r + offset).
#[test]
fn a_ball_cut_at_a_slant_is_drawn_within_the_drawing_s_tolerance_of_what_it_holds() {
    for offset in [-4.0, -2.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.5] {
        let (cut, kept) = cut_at_a_slant(offset);
        let what = format!("the ball cut at a slant {offset} from its centre");
        let body = cut.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
        built_and_drawn(&body, kept, TAU * 5.0 * (5.0 + offset), &what);
    }
}

/// The ball sliced between two planes at a slant, 2 either side of its
/// centre along (1, 1, 0): its sphere a zone whose two circles turn about
/// that one line, read about it, 2πr × 4.
#[test]
fn a_ball_sliced_between_two_parallel_planes_at_a_slant_is_built_and_drawn_within_the_drawing_s_tolerance()
 {
    let normal = DVec3::new(1.0, 1.0, 0.0);

    let sliced = ball_about_y()
        .difference(&slab_along(normal, 2.0))
        .and_then(|cut| cut.difference(&slab_along(-normal, 2.0)));

    let what = "the ball sliced between two parallel planes at a slant";
    let body = sliced.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    built_and_drawn(
        &body,
        BALL_OF_5 - 2.0 * cap_of_5(3.0),
        TAU * 5.0 * 4.0,
        what,
    );
}

/// What two caps of the ball hold in common, their planes `offset` from its
/// centre along two directions square to each other, nothing where they keep
/// apart: over every place `a` along the first from `offset` out to
/// e = √(25 − offset²), the slice of the disc of radius c = √(25 − a²) past
/// the chord `offset` along the second, c² (π/2 − asin(offset/c))
/// − offset √(c² − offset²), summed by Simpson's rule with a = e − s², under
/// which the slice's vanishing at e is smooth.
fn two_caps_square_to_each_other_share(offset: f64) -> f64 {
    let reach = (25.0 - offset * offset).sqrt();
    if reach <= offset {
        return 0.0;
    }
    let slice = |s: f64| {
        let a = reach - s * s;
        let square = 25.0 - a * a;
        let across = square * (FRAC_PI_2 - (offset / square.sqrt()).min(1.0).asin())
            - offset * (square - offset * offset).max(0.0).sqrt();
        across * 2.0 * s
    };
    let (steps, end) = (1 << 12, (reach - offset).sqrt());
    let step = end / f64::from(steps);
    let inner: f64 = (1..steps)
        .map(|index| slice(step * f64::from(index)) * if index % 2 == 1 { 4.0 } else { 2.0 })
        .sum();
    (slice(0.0) + inner + slice(end)) * step / 3.0
}

/// The ball cut by two flats at a slant, square to each other, 2 from its
/// centre along (1, 1, 0) and (1, −1, 0): the one takes its pole at y = 5,
/// the other the one at y = −5, and the sphere left holds neither, bounded by
/// two circles of two axes, neither the ball's — the face #805's own chart
/// lays out. Its sphere is under the whole sphere less one cap, 70π.
#[test]
fn a_ball_cut_by_two_flats_square_to_each_other_across_both_poles_is_built_and_drawn_within_the_drawing_s_tolerance()
 {
    let cut = ball_about_y()
        .difference(&slab_along(DVec3::new(1.0, 1.0, 0.0), 2.0))
        .and_then(|cut| cut.difference(&slab_along(DVec3::new(1.0, -1.0, 0.0), 2.0)));

    let what = "the ball cut by two flats square to each other across both poles";
    let body = cut.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    built_and_drawn(
        &body,
        BALL_OF_5 - 2.0 * cap_of_5(3.0) + two_caps_square_to_each_other_share(2.0),
        70.0 * PI,
        what,
    );
}

/// The ball cut by two flats whose caps overlap, each reaching 62° from its
/// direction, in a plane holding the ball's axis: the first along a
/// direction 60° off one of its poles, which it takes two degrees inside its
/// rim; the second 20° off the other, which it takes well inside. The sphere
/// left holds neither pole, and its first circle passes two degrees by the
/// pole the first cap took, nearer than the chart can lay a face out: the
/// step is declined — whichever pole, and whichever plane holding the axis
/// the two directions lie in. The limit is the chart's, not the face's: a
/// chart about another axis stands that circle well off its poles. #940
/// draws a face bounded by circles of several axes whatever poles of the
/// chart it holds, not one an edge of which comes within the chart's reach
/// of a pole, and leaves this one declined until #962 reads it about another
/// axis.
#[test]
fn a_ball_cut_by_two_flats_whose_circle_passes_two_degrees_by_a_pole_of_its_chart_is_declined_as_undrawn()
 {
    let reach = 5.0 * 62f64.to_radians().cos();
    let (sin, cos) = 60f64.to_radians().sin_cos();
    let (aside, under) = 20f64.to_radians().sin_cos();
    for (pole, across) in [
        (1.0, DVec3::X),
        (-1.0, DVec3::X),
        (1.0, DVec3::Z),
        (-1.0, DVec3::new(-1.0, 0.0, 1.0).normalize()),
    ] {
        let first = ball_about_y()
            .difference(&slab_along(across * sin + DVec3::Y * (pole * cos), reach))
            .expect("one flat cuts the ball");

        let declined = first
            .difference(&slab_along(
                across * aside - DVec3::Y * (pole * under),
                reach,
            ))
            .err();

        assert_eq!(
            declined,
            Some(Declined::Undrawn),
            "a ball cut by two flats along {across} either side of its pole {pole}, the sphere \
             left bounded by a circle two degrees by that pole, which #962 is to draw about \
             another axis"
        );
    }
}

/// What two discs of radii `one` and `other` share, their centres `apart`.
fn shared_by_two_discs(one: f64, other: f64, apart: f64) -> f64 {
    if apart >= one + other {
        return 0.0;
    }
    if apart <= (one - other).abs() {
        return PI * one.min(other).powi(2);
    }
    [(one, other), (other, one)]
        .map(|(near, far)| {
            let angle = ((apart * apart + near * near - far * far) / (2.0 * apart * near))
                .clamp(-1.0, 1.0)
                .acos();
            near * near * (angle - angle.sin() * angle.cos())
        })
        .iter()
        .sum()
}

/// The matter a cylinder of `radius` whose axis passes `off` from the centre
/// of a ball of radius `ball` holds inside the ball: over each plane square
/// to the cylinder's axis, `t` from the centre's foot, what its section
/// shares with the ball's, of radius √(ball² − t²), summed by Simpson's rule
/// between the heights where the two sections start or stop crossing, each
/// run laid along t = from + (to − from)(1 − cos πu)/2, under which the
/// share's kinks there are smooth.
fn bore_inside_a_ball(radius: f64, off: f64, ball: f64) -> f64 {
    let shared = |t: f64| shared_by_two_discs((ball * ball - t * t).max(0.0).sqrt(), radius, off);
    let mut heights: Vec<f64> = [off + radius, (off - radius).abs()]
        .into_iter()
        .filter(|&reach| reach < ball)
        .map(|reach| (ball * ball - reach * reach).sqrt())
        .chain([0.0, ball])
        .collect();
    heights.sort_by(f64::total_cmp);
    let steps: u32 = 1 << 12;
    let step = 1.0 / f64::from(steps);
    let half: f64 = heights
        .windows(2)
        .map(|run| {
            let (from, to) = (run[0], run[1]);
            let at = |u: f64| {
                let t = from + (to - from) * (1.0 - (PI * u).cos()) / 2.0;
                shared(t) * (to - from) * PI * (PI * u).sin() / 2.0
            };
            let inner: f64 = (1..steps)
                .map(|index| at(step * f64::from(index)) * if index % 2 == 1 { 4.0 } else { 2.0 })
                .sum();
            (at(0.0) + inner + at(1.0)) * step / 3.0
        })
        .sum();
    2.0 * half
}

/// A bore of `radius` along `axis`, its axis `off` from the origin towards
/// `out`, from 10 before the origin's foot on it to 10 past.
fn bore_off_the_centre(axis: DVec3, out: DVec3, radius: f64, off: f64) -> Body {
    bore_between(axis, out, radius, off, [-10.0, 10.0])
}

/// The same bore, from `from` to `to` along `axis` past the origin's foot.
fn bore_between(axis: DVec3, out: DVec3, radius: f64, off: f64, [from, to]: [f64; 2]) -> Body {
    let axis = axis.normalize();
    let u = out.reject_from(axis).normalize();
    let frame = Frame {
        origin: u * off + axis * from,
        u,
        v: axis.cross(u),
    };
    Body::default()
        .tool_raised(
            &exact(disc(DVec2::ZERO, radius), vec![]),
            frame,
            axis * (to - from),
        )
        .expect("a bore stands on its own")
}

/// The ball of radius 5 about the origin less a bore of `radius` along
/// `axis`, `off` from its centre towards `out`: built, it holds the ball less
/// what the bore holds inside it, as [`built_and_drawn`] holds, over the
/// ball's whole sphere and the bore's wall across it, 2π·5·10 + 2πr·10.
fn bored_off_the_centre(axis: DVec3, out: DVec3, radius: f64, off: f64, what: &str) {
    let bored = ball_about_y().difference(&bore_off_the_centre(axis, out, radius, off));
    let body = bored.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    built_and_drawn(
        &body,
        BALL_OF_5 - bore_inside_a_ball(radius, off, 5.0),
        TAU * 5.0 * 10.0 + TAU * radius * 10.0,
        what,
    );
}

/// A bore passing right through the ball, r + d < 5, its axis off the
/// ball's centre: along the ball's own axis, square to it and at a slant.
/// Its wall meets the sphere along two loops, each a hole in the sphere and
/// an end of the wall's band.
#[test]
fn a_ball_bored_right_through_off_its_axis_is_built_and_drawn_within_the_drawing_s_tolerance() {
    for (axis, out, radius, off) in [
        (DVec3::Y, DVec3::X, 1.0, 2.0),
        (DVec3::X, DVec3::Z, 1.5, 2.5),
        (DVec3::ONE, DVec3::new(1.0, -1.0, 0.0), 2.0, 1.0),
    ] {
        let what = format!("the ball bored along {axis}, of radius {radius}, {off} off its centre");
        bored_off_the_centre(axis, out, radius, off, &what);
    }
}

/// A cylinder taking in part of the ball, |r − d| < 5 < r + d, its axis off
/// the ball's centre: along the ball's own axis, square to it and at a
/// slant. Its wall meets the sphere along one loop, a hole in the sphere and
/// the whole outline of the wall's patch.
#[test]
fn a_ball_notched_by_a_cylinder_off_its_axis_is_built_and_drawn_within_the_drawing_s_tolerance() {
    for (axis, out, radius, off) in [
        (DVec3::Y, DVec3::X, 2.0, 4.0),
        (DVec3::X, DVec3::Z, 1.5, 4.5),
        (DVec3::ONE, DVec3::new(1.0, 0.0, -1.0), 2.5, 4.0),
    ] {
        let what =
            format!("the ball notched along {axis}, of radius {radius}, {off} off its centre");
        bored_off_the_centre(axis, out, radius, off, &what);
    }
}

/// A bead drilled a little off its centre: a bore of radius 1 along the
/// ball's own axis and 0.5 off it, which holds the axis and both poles of the
/// ball's chart, each of its two loops winding round the axis.
#[test]
fn a_ball_bored_along_its_axis_a_little_off_its_centre_is_built_and_drawn_within_the_drawing_s_tolerance()
 {
    bored_off_the_centre(
        DVec3::Y,
        DVec3::X,
        1.0,
        0.5,
        "the ball bored along its own axis, of radius 1, 0.5 off it",
    );
}

/// The ball bored along X, of radius 1, 1.5 off its centre towards Z, then
/// counterbored from x = 0 on, of radius 2: about the bore's own axis, or
/// about the line 1 below it, the counterbore's wall then touching the
/// bore's along its top ruling, whose ends on the sphere stand on the
/// equator of the ball's chart. The ball keeps all but the bore's half
/// inside it on one side and the counterbore's on the other.
#[test]
fn a_ball_bored_off_its_axis_then_counterbored_about_its_bore_or_touching_it_is_built_and_drawn_within_the_drawing_s_tolerance()
 {
    let bored = ball_about_y()
        .difference(&bore_off_the_centre(DVec3::X, DVec3::Z, 1.0, 1.5))
        .expect("the ball is bored");
    for (off, how) in [
        (1.5, "about the bore's axis"),
        (0.5, "touching the bore along its top ruling"),
    ] {
        let what = format!("the bored ball counterbored {how}");
        let counterbore = bore_between(DVec3::X, DVec3::Z, 2.0, off, [0.0, 10.0]);
        let body = bored
            .difference(&counterbore)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
        let taken = (bore_inside_a_ball(1.0, 1.5, 5.0) + bore_inside_a_ball(2.0, off, 5.0)) / 2.0;
        built_and_drawn(
            &body,
            BALL_OF_5 - taken,
            TAU * 5.0 * 10.0 + TAU * 10.0 + TAU * 2.0 * 10.0,
            &what,
        );
    }
}

/// The ball bored off its axis, then bored again by the same hole: the second
/// cut meets the sphere along the very loops the first left, and takes
/// nothing more.
#[test]
fn a_ball_bored_off_its_axis_and_bored_again_by_the_same_hole_holds_what_it_held() {
    let bore = bore_off_the_centre(DVec3::X, DVec3::Z, 1.0, 1.5);
    let bored = ball_about_y().difference(&bore).expect("the ball is bored");
    let what = "the bored ball bored again by the same hole";
    let again = bored
        .difference(&bore)
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    built_and_drawn(
        &again,
        BALL_OF_5 - bore_inside_a_ball(1.0, 1.5, 5.0),
        TAU * 5.0 * 10.0 + TAU * 10.0,
        what,
    );
}

/// The plug a bore off the ball's axis takes out, the ball less the bored
/// ball, is the bore's piece inside the ball; joined back with the bored
/// ball, it gives the ball whole. The two meet along the bore's loops, on one
/// sphere.
#[test]
fn the_plug_a_bore_off_a_ball_s_axis_takes_out_joins_back_with_the_bored_ball_into_the_ball() {
    let bored = ball_about_y()
        .difference(&bore_off_the_centre(DVec3::X, DVec3::Z, 1.0, 1.5))
        .expect("the ball is bored");
    let what = "the plug the bore takes out of the ball";
    let plug = ball_about_y()
        .difference(&bored)
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    let curved = TAU * 5.0 * 10.0 + TAU * 10.0;
    built_and_drawn(&plug, bore_inside_a_ball(1.0, 1.5, 5.0), curved, what);
    let what = "the bored ball and its plug joined back";
    let whole = bored
        .union(&plug)
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    built_and_drawn(&whole, BALL_OF_5, curved, what);
}

/// A shell, the ball of radius 5 less the ball of radius 3 about its centre,
/// bored off its axis right through both spheres: the loops on the one
/// sphere are none of the other's, though the two turn about one centre.
#[test]
fn a_hollow_ball_bored_off_its_axis_is_built_and_drawn_within_the_drawing_s_tolerance() {
    let shell = ball_about_y()
        .difference(&ball_about_y_of(3.0, DVec3::ZERO))
        .expect("the ball is hollowed");
    let what = "the hollow ball bored off its axis";
    let body = shell
        .difference(&bore_off_the_centre(DVec3::X, DVec3::Z, 1.0, 1.5))
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    let taken = bore_inside_a_ball(1.0, 1.5, 5.0) - bore_inside_a_ball(1.0, 1.5, 3.0);
    built_and_drawn(
        &body,
        BALL_OF_5 - 36.0 * PI - taken,
        TAU * 5.0 * 10.0 + TAU * 3.0 * 6.0 + TAU * 10.0,
        what,
    );
}

/// Two balls of radius 5, about the origin and 12 along X from it, bored
/// through by one hole along X 1.5 off both centres: the loops on the one
/// sphere are none of the other's, though the two are of one radius.
#[test]
fn two_balls_of_one_radius_bored_through_by_one_hole_are_built_and_drawn_within_the_drawing_s_tolerance()
 {
    let two = ball_about_y()
        .union(&ball_about_y_of(5.0, DVec3::X * 12.0))
        .expect("the two balls stand apart");
    let what = "two balls of one radius bored through by one hole";
    let body = two
        .difference(&bore_between(DVec3::X, DVec3::Z, 1.0, 1.5, [-10.0, 22.0]))
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
    built_and_drawn(
        &body,
        2.0 * (BALL_OF_5 - bore_inside_a_ball(1.0, 1.5, 5.0)),
        2.0 * TAU * 5.0 * 10.0 + TAU * 32.0,
        what,
    );
}

#[test]
fn a_disc_s_box_reaches_its_rim_exactly() {
    let post = raised(
        &Body::default(),
        &exact(disc(DVec2::ZERO, 20.0), vec![]),
        10.0,
    );

    let (low, high) = post.bounds().expect("the post is there");

    assert!(low.distance(DVec3::new(-20.0, -20.0, 0.0)) < 1e-12, "{low}");
    assert!(
        high.distance(DVec3::new(20.0, 20.0, 10.0)) < 1e-12,
        "{high}"
    );
}

/// A bore of radius three raised twenty from `origin` along the normal of a
/// plane leaning thirty degrees about X.
fn slanted_bore(part: &Body, origin: DVec3) -> Body {
    let (sin, cos) = (30.0f64).to_radians().sin_cos();
    let slanted = Frame {
        origin,
        u: DVec3::X,
        v: DVec3::new(0.0, cos, sin),
    };
    part.tool_raised(
        &exact(disc(DVec2::ZERO, 3.0), vec![]),
        slanted,
        slanted.normal() * 20.0,
    )
    .expect("a bore stands on its own")
}

#[test]
fn a_slant_and_a_frustum_across_a_post_are_built_and_a_point_or_a_frustum_within_its_slope_are_handed_back_as_declines()
 {
    let post = raised(
        &Body::default(),
        &exact(disc(DVec2::splat(10.0), 8.0), vec![]),
        10.0,
    );
    let bore = slanted_bore(&post, DVec3::new(10.0, 10.0, -5.0));
    let bored = post.difference(&bore).expect("a skew bore is built");
    assert!(bored.is_exact());
    assert!(
        bored.volume() < post.volume() && bored.volume() > post.volume() - bore.volume(),
        "{} out of {}, the bore {}",
        bored.volume(),
        post.volume(),
        bore.volume()
    );

    let sides = [
        DVec2::ZERO,
        DVec2::new(8.0, 0.0),
        DVec2::new(2.0, 7.5),
        DVec2::new(0.0, 7.5),
    ];
    let triangles = fan(&sides);
    let frustum = straight(&sides, &triangles);
    let cone = post
        .tool_turned(&frustum, FLAT, &about_v(&frustum, TAU))
        .expect("a frustum turns");
    let cut = post
        .difference(&cone)
        .expect("a frustum across the post is built");
    assert!(cut.is_exact());
    assert!(
        cut.volume() < post.volume() && cut.volume() > post.volume() - cone.volume(),
        "{} out of {}, the frustum {}",
        cut.volume(),
        post.volume(),
        cone.volume()
    );

    let tip = [DVec2::ZERO, DVec2::new(8.0, 0.0), DVec2::new(0.0, 10.0)];
    let tip_triangles = fan(&tip);
    let point = straight(&tip, &tip_triangles);
    let pointed = post
        .tool_turned(&point, FLAT, &about_v(&point, TAU))
        .expect("a point turns");
    assert_eq!(
        post.difference(&pointed),
        Err(Declined::Unsupported),
        "a curve on a face holding its apex"
    );

    let (sin, cos) = 10f64.to_radians().sin_cos();
    let leaning = Frame {
        origin: DVec3::new(1.0, 0.0, 0.0),
        u: DVec3::new(cos, sin, 0.0),
        v: DVec3::new(-sin, cos, 0.0),
    };
    let within = cut
        .tool_turned(&frustum, leaning, &about_v(&frustum, TAU))
        .expect("a frustum turns");
    assert_eq!(cut.difference(&within), Err(Declined::Unsupported));
}

#[test]
fn a_slant_through_a_block_s_floor_and_top_is_cut_exactly() {
    let part = block(DVec2::ZERO, DVec2::splat(20.0), 10.0);
    let bore = slanted_bore(&part, DVec3::new(10.0, 14.0, -5.0));

    let cut = part.difference(&bore).expect("the kernel cuts it");

    assert!(cut.is_exact());
    let arithmetic = 4000.0 - 60.0 * 3f64.sqrt() * PI;
    assert!(
        (cut.volume() - arithmetic).abs() <= 1e-9 * arithmetic,
        "{} against {arithmetic}",
        cut.volume()
    );
}

#[test]
fn a_slant_through_a_block_s_floor_and_out_through_its_side_is_cut_exactly() {
    let part = block(DVec2::ZERO, DVec2::splat(20.0), 10.0);
    let bore = slanted_bore(&part, DVec3::new(10.0, 10.0, -5.0));

    let cut = part.difference(&bore).expect("the kernel cuts it");

    assert!(cut.is_exact());
    let (half, slope) = (
        3.0 / (30.0f64).to_radians().cos(),
        (30.0f64).to_radians().tan(),
    );
    let level = |z: f64| (10.0 - slope * (z + 5.0)) / half;
    let swept =
        |c: f64| PI * c - (c * c.acos() - (1.0 - c * c).sqrt()) - (1.0 - c * c).powf(1.5) / 3.0;
    let whole = (10.0 - half) / slope - 5.0;
    let bored = 3.0 * half * (PI * whole + (swept(level(10.0)) - swept(1.0)) / (-slope / half));
    let arithmetic = 4000.0 - bored;
    assert!(
        (cut.volume() - arithmetic).abs() <= 1e-9 * arithmetic,
        "{} against {arithmetic}",
        cut.volume()
    );
    let triangles = cut.triangles();
    assert_eq!(crate::soundness::closed(&triangles), Ok(()));
    assert_eq!(crate::soundness::uncrossed(&triangles), Ok(()));
}

#[test]
fn a_kernel_that_stops_on_a_bug_hands_back_a_decline() {
    let stopped: Result<(), Declined> = caught::caught(|| panic!("a shape nobody tried"));

    assert_eq!(stopped, Err(Declined::Panicked));
}

thread_local! {
    static HEARD: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[test]
fn a_stop_the_kernel_catches_never_reaches_the_hook_that_writes_crashes_down() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        HEARD.set(HEARD.get() + 1);
        previous(info);
    }));
    caught::silence_caught_panics();

    let stopped: Result<(), Declined> = caught::caught(|| panic!("a shape nobody tried"));
    let heard_from_the_kernel = HEARD.get();
    let crashed = std::panic::catch_unwind(|| panic!("a bug of the application's own"));

    assert_eq!(stopped, Err(Declined::Panicked));
    assert_eq!(heard_from_the_kernel, 0, "a decline is no crash");
    assert!(crashed.is_err());
    assert_eq!(HEARD.get(), 1, "a crash outside the kernel is still heard");
}

#[test]
fn two_blocks_numbered_and_named_alike_but_of_different_sizes_are_not_one_body() {
    let short = block(DVec2::ZERO, DVec2::splat(10.0), 10.0);
    let long = block(DVec2::ZERO, DVec2::new(20.0, 10.0), 10.0);

    assert_eq!(short.faces_end(), long.faces_end());
    assert!(short.pieces().is_empty() && long.pieces().is_empty());
    assert_eq!(short.edge_names(), long.edge_names());
    assert_eq!(short.vertex_names(), long.vertex_names());
    assert_ne!(short, long, "the matter tells them apart");
    assert_eq!(short, short.clone(), "and a block is one with itself");
}

#[test]
fn a_tools_faces_are_numbered_above_every_number_the_part_gave() {
    let part = block(DVec2::ZERO, DVec2::splat(10.0), 10.0);
    let boss = block(DVec2::splat(2.0), DVec2::splat(5.0), 20.0);

    let joined = part.union(&boss).expect("the kernel joins them");

    assert_eq!(part.faces_end(), 6, "a floor, a top and four walls");
    assert_eq!(joined.faces_end(), 12);
    assert!(joined.has_face(TOP) && joined.has_face(6 + TOP));
    assert!(!joined.has_face(12));
}

#[test]
fn a_top_a_trench_parts_in_two_gives_its_second_piece_a_number_no_step_gives() {
    let part = block(DVec2::ZERO, DVec2::new(30.0, 10.0), 10.0);
    let trench = part
        .tool_raised(
            &exact(
                Contour::rectangle(DVec2::new(10.0, -1.0), DVec2::new(20.0, 11.0)),
                vec![],
            ),
            Frame {
                origin: DVec3::Z * 5.0,
                ..FLAT
            },
            DVec3::Z * 10.0,
        )
        .expect("a trench stands on its own");

    let cut = part.difference(&trench).expect("the kernel cuts it");

    let met = [5.0, 25.0].map(|along| {
        cut.ray_hit(DVec3::new(along, 5.0, 20.0), DVec3::NEG_Z)
            .expect("the ray meets the top")
            .face
    });
    assert!(
        met.contains(&TOP),
        "one piece keeps the top's number: {met:?}"
    );
    let fresh = met
        .into_iter()
        .find(|&face| face != TOP)
        .unwrap_or_else(|| panic!("the other piece answers to another: {met:?}"));
    assert_eq!(cut.parted_from(fresh), Some(TOP));
    assert_eq!(
        cut.faces_end(),
        12,
        "the block's six numbers and the trench's six: parting the top took none"
    );
    assert!(
        fresh >= cut.faces_end(),
        "the second piece answers to {fresh}, which no step gives"
    );
    let sides: Vec<f64> = [TOP, fresh]
        .iter()
        .map(|&face| {
            let xs: Vec<f64> = cut
                .triangles_of(face)
                .flatten()
                .map(|corner| corner.x)
                .collect();
            assert!(!xs.is_empty(), "face {face} is drawn");
            xs.iter().sum::<f64>() / xs.len() as f64
        })
        .collect();
    assert!(
        (sides[0] < 10.0) != (sides[1] < 10.0),
        "one piece on each side of the trench: {sides:?}",
    );
}

#[test]
fn a_step_the_kernel_declined_still_moves_the_count_past_its_numbers() {
    let mut part = block(DVec2::ZERO, DVec2::splat(10.0), 10.0);
    let declined = exact(disc(DVec2::splat(5.0), 3.0), vec![]);

    part.count_past(&declined);

    assert_eq!(part.faces_end(), 6 + 3, "a floor, a top and one wall");
    let after = block(DVec2::splat(2.0), DVec2::splat(5.0), 20.0);
    let joined = part.union(&after).expect("the kernel joins them");
    assert!(joined.has_face(9 + TOP));
}

#[test]
fn a_revolution_after_a_declined_step_numbers_its_faces_past_every_number_the_exact_body_gave() {
    let mut part = block(DVec2::ZERO, DVec2::splat(10.0), 10.0);
    part.count_past(&exact(
        Contour::rectangle(DVec2::splat(2.0), DVec2::splat(5.0)),
        vec![],
    ));
    let ring = ring_turned_about_y();

    let turned = part.union(&ring).expect("the flats never decline");

    assert_eq!(turned.faces_end(), 12 + ring.faces_end());
    assert!(
        (6..12).all(|face| !turned.has_face(face)),
        "the declined step's numbers stay its own",
    );
    assert!((12..turned.faces_end()).all(|face| turned.has_face(face)));
    let raised = turned
        .union(&flats_block(1.0, 1.0, 20.0))
        .expect("the flats never decline");
    assert!(raised.has_face(turned.faces_end() + TOP));
}

#[test]
fn a_revolution_after_a_cut_numbers_its_faces_past_the_tool_s_faces_the_cut_threw_away() {
    let part = block(DVec2::ZERO, DVec2::splat(10.0), 10.0);
    let notch = part
        .tool_raised(
            &exact(
                Contour::rectangle(DVec2::splat(-1.0), DVec2::new(5.0, 11.0)),
                vec![],
            ),
            Frame {
                origin: DVec3::Z * -1.0,
                ..FLAT
            },
            DVec3::Z * 12.0,
        )
        .expect("a notch stands on its own");
    let notched = part.difference(&notch).expect("the kernel cuts it");
    let ring = ring_turned_about_y();

    let turned = notched.union(&ring).expect("the flats never decline");

    assert_eq!(notched.faces_end(), 12);
    assert_eq!(turned.faces_end(), 12 + ring.faces_end());
    assert!((12..turned.faces_end()).all(|face| turned.has_face(face)));
}

/// A block the flats raise, its top one flat face stored as two triangles.
fn flats_block(width: f64, depth: f64, height: f64) -> Body {
    let corners = [
        DVec2::ZERO,
        DVec2::new(width, 0.0),
        DVec2::new(width, depth),
        DVec2::new(0.0, depth),
    ];
    let triangles = [
        [corners[0], corners[1], corners[2]],
        [corners[0], corners[2], corners[3]],
    ];
    let profile = Profile {
        exact: None,
        sampled: Loop::straight(&corners),
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    Body::default()
        .tool_raised(&profile, FLAT, DVec3::Z * height)
        .expect("the flats never decline")
}

#[test]
fn a_face_of_the_flats_is_drawn_with_the_triangles_it_was_raised_from() {
    let body = flats_block(10.0, 6.0, 4.0);

    let triangles: Vec<[DVec3; 3]> = body.triangles_of(TOP).collect();

    assert_eq!(triangles.len(), 2, "{triangles:?}");
    assert!(
        triangles.iter().flatten().all(|corner| corner.z == 4.0),
        "every triangle of the top lies on it: {triangles:?}",
    );
    assert_eq!(body.triangles_of(body.faces_end()).count(), 0);
}

#[test]
fn a_flat_face_of_the_flats_offers_its_plane_with_every_corner_of_every_piece() {
    let body = flats_block(10.0, 6.0, 4.0);

    let plane = body.plane_of(TOP).expect("the block has a top");

    assert!(body.is_flat(TOP));
    assert!(plane.normal.dot(DVec3::Z) > 0.999, "{}", plane.normal);
    assert_eq!(plane.corners.len(), 6, "two triangles: {:?}", plane.corners);
    assert_eq!(body.plane_of(body.faces_end()), None);
    let hit = body
        .ray_hit(DVec3::new(5.0, 2.0, 20.0), DVec3::NEG_Z)
        .expect("the top of the block");
    assert_eq!(hit.face, TOP);
    assert!((hit.distance - 16.0).abs() < 1e-9, "{}", hit.distance);
}

#[test]
fn an_exact_body_is_drawn_closed_uncrossed_and_round_enough() {
    let body = bored_block();

    let triangles = body.triangles();

    assert_eq!(crate::soundness::closed(&triangles), Ok(()));
    assert_eq!(crate::soundness::uncrossed(&triangles), Ok(()));
    let flats = 30.0 * 20.0 * 10.0 - polygon_area(&sampled_disc(DVec2::ZERO, 4.0).0) * 10.0;
    let drawn = crate::soundness::enclosed(&triangles);
    assert!(
        (drawn - body.volume()).abs() <= (flats - body.volume()).abs(),
        "drawn no coarser than the flats: {drawn} against {flats}, exactly {}",
        body.volume(),
    );
}

#[test]
fn a_bore_a_hair_inside_a_wall_is_built_though_the_sliver_between_draws_to_nothing() {
    let raise = |part: &Body, contour: Contour, low: f64, height: f64| {
        let at = Frame {
            origin: DVec3::Z * low,
            ..FLAT
        };
        part.tool_raised(&exact(contour, vec![]), at, DVec3::Z * height)
            .expect("the kernel raises it")
    };
    let empty = Body::default();
    let block = empty
        .union(&raise(
            &empty,
            Contour::rectangle(DVec2::new(2.0, -3.0), DVec2::new(10.0, 5.0)),
            7.0,
            4.0,
        ))
        .expect("the kernel raises it");
    let bored = block
        .difference(&raise(
            &block,
            disc(DVec2::new(6.00000006, 1.0), 3.0),
            6.0,
            8.0,
        ))
        .expect("the kernel bores it");

    let joined = bored.union(&raise(&bored, disc(DVec2::new(9.0, 5.0), 2.0), -2.0, 10.0));

    assert!(joined.is_ok(), "{joined:?}");
}

#[test]
fn a_hole_touching_its_outline_at_one_point_is_raised_and_drawn_closed_and_uncrossed() {
    let touching = [
        (
            Contour::rectangle(DVec2::ZERO, DVec2::new(20.0, 10.0)),
            20.0 * 10.0,
            DVec2::new(10.0, 4.0),
            4.0,
        ),
        (
            disc(DVec2::new(5.0, 7.0), 5.0),
            PI * 5.0 * 5.0,
            DVec2::new(1.0, 7.0),
            1.0,
        ),
        (
            Contour {
                corners: vec![
                    DVec2::new(-10.0, -5.0),
                    DVec2::new(10.0, -5.0),
                    DVec2::new(10.0, 5.0),
                    DVec2::new(-10.0, 5.0),
                ],
                runs: vec![
                    Run::Straight,
                    Run::Round {
                        center: DVec2::new(10.0, 0.0),
                        turn: PI,
                    },
                    Run::Straight,
                    Run::Round {
                        center: DVec2::new(-10.0, 0.0),
                        turn: PI,
                    },
                ],
            },
            20.0 * 10.0 + PI * 5.0 * 5.0,
            DVec2::new(12.0, 0.0),
            3.0,
        ),
    ];
    for (outline, area, center, radius) in touching {
        let plate = Body::default()
            .tool_raised(
                &exact(outline, vec![disc(center, radius)]),
                FLAT,
                DVec3::Z * 5.0,
            )
            .expect("the kernel raises it");

        let part = Body::default().union(&plate).expect("the plate is drawn");

        let triangles = part.triangles();
        assert_eq!(crate::soundness::closed(&triangles), Ok(()));
        assert_eq!(crate::soundness::uncrossed(&triangles), Ok(()));
        let arithmetic = (area - PI * radius * radius) * 5.0;
        assert!(
            (part.volume() - arithmetic).abs() < 1e-9 * arithmetic,
            "{} against {arithmetic}",
            part.volume(),
        );
    }
}

fn straight<'a>(points: &'a [DVec2], triangles: &'a [[DVec2; 3]]) -> Profile<'a> {
    Profile {
        exact: Some((Contour::straight(points.to_vec()), vec![])),
        sampled: Loop::straight(points),
        sampled_holes: Vec::new(),
        triangles,
    }
}

fn about_v(area: &Profile, angle: f64) -> Turn {
    let v = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    Turn::of(v, angle, 0.0, area)
}

#[test]
fn a_profile_handed_without_its_runs_or_to_the_flats_is_the_flats_revolution() {
    let ring = Contour::rectangle(DVec2::new(30.0, 0.0), DVec2::new(34.0, 4.0)).corners;
    let ring_triangles = fan(&ring);
    let sampled_only = Profile {
        exact: None,
        ..straight(&ring, &ring_triangles)
    };
    let mut cases = vec![(ring_turned_about_y(), straight(&ring, &ring_triangles))];
    for part in [
        Body::default(),
        block(DVec2::ZERO, DVec2::splat(10.0), 10.0),
    ] {
        cases.push((part, sampled_only.clone()));
    }
    for (part, profile) in cases {
        for angle in [TAU, -TAU, PI / 2.0, -PI] {
            let turned = part
                .tool_turned(&profile, FLAT, &about_v(&profile, angle))
                .expect("the flats never decline");
            let flats = sweep::revolution(
                profile.sampled,
                &[],
                profile.triangles,
                |point| FLAT.at(point),
                DVec2::ZERO,
                DVec2::Y,
                angle,
            )
            .expect("a profile on one side of its axis");
            assert_eq!(
                turned,
                Body::raised_flats(flats),
                "{:?} turned {angle} rad",
                profile.sampled.points,
            );
        }
    }
}

#[test]
fn a_profile_the_exact_kernel_does_not_turn_is_declined_never_the_flats_revolution() {
    let ring = Contour::rectangle(DVec2::new(30.0, 0.0), DVec2::new(34.0, 4.0)).corners;
    let ring_triangles = fan(&ring);
    let mut bulging = Contour::straight(ring.clone());
    bulging.runs[1] = Run::Round {
        center: DVec2::new(32.0, 2.0),
        turn: 1e-6,
    };
    let rounded = Profile {
        exact: Some((bulging, vec![])),
        ..straight(&ring, &ring_triangles)
    };
    for part in [
        Body::default(),
        block(DVec2::ZERO, DVec2::splat(10.0), 10.0),
    ] {
        for angle in [TAU, -TAU, PI / 2.0, -PI] {
            let turn = about_v(&rounded, angle);
            assert_eq!(
                part.tool_turned(&rounded, FLAT, &turn),
                Err(Declined::Profile),
                "turned {angle} rad",
            );
            let mut declined = part.clone();
            declined.count_past_turned(&rounded, FLAT, &turn);
            assert!(
                declined.is_exact(),
                "turned {angle} rad: the part stays exact"
            );
            let flats = sweep::revolution(
                rounded.sampled,
                &[],
                rounded.triangles,
                |point| FLAT.at(point),
                DVec2::ZERO,
                DVec2::Y,
                angle,
            )
            .expect("a profile on one side of its axis");
            assert_eq!(
                declined.faces_end(),
                part.faces_end() + flats.faces_end(),
                "turned {angle} rad: counted past the numbers the flats gave the turn",
            );
        }
    }
}

#[test]
fn a_straight_profile_turned_on_an_exact_part_never_comes_back_as_flats() {
    let ring = Contour::rectangle(DVec2::new(30.0, 0.0), DVec2::new(34.0, 4.0)).corners;
    let ring_triangles = fan(&ring);
    let slanted = [
        DVec2::new(30.0, 0.0),
        DVec2::new(34.0, 0.0),
        DVec2::new(33.0, 4.0),
        DVec2::new(30.0, 4.0),
    ];
    let slanted_triangles = fan(&slanted);
    for profile in [
        straight(&ring, &ring_triangles),
        straight(&slanted, &slanted_triangles),
    ] {
        for part in [
            Body::default(),
            block(DVec2::ZERO, DVec2::splat(10.0), 10.0),
        ] {
            for angle in [TAU, -TAU, PI / 2.0, -PI] {
                let turned = part.tool_turned(&profile, FLAT, &about_v(&profile, angle));
                assert!(
                    turned.as_ref().map_or(true, Body::is_exact),
                    "{:?} turned {angle} rad: {turned:?}",
                    profile.sampled.points,
                );
            }
        }
    }
}

#[test]
fn an_area_across_its_axis_is_declined_and_a_turn_too_short_makes_nothing() {
    let across = Contour::rectangle(DVec2::new(-1.0, 0.0), DVec2::new(4.0, 4.0)).corners;
    let triangles = fan(&across);
    let profile = straight(&across, &triangles);
    assert_eq!(
        Body::default().tool_turned(&profile, FLAT, &about_v(&profile, TAU)),
        Err(Declined::Profile),
    );

    let ring = Contour::rectangle(DVec2::new(30.0, 0.0), DVec2::new(34.0, 4.0)).corners;
    let triangles = fan(&ring);
    let profile = straight(&ring, &triangles);
    let nothing = Body::default()
        .tool_turned(&profile, FLAT, &about_v(&profile, 1e-5))
        .expect("nothing is no decline");
    assert!(nothing.is_empty());
}

#[test]
fn a_disc_a_hair_thick_turned_on_an_exact_part_is_nothing_counted_past_its_numbers() {
    let disc = [
        DVec2::new(0.0, 0.0),
        DVec2::new(5.0, 0.0),
        DVec2::new(5.0, 1e-8),
        DVec2::new(0.0, 1e-8),
    ];
    let triangles = fan(&disc);
    let profile = straight(&disc, &triangles);
    let turn = about_v(&profile, TAU);
    let nothing = Body::default()
        .tool_turned(&profile, FLAT, &turn)
        .expect("nothing is no decline");
    assert!(nothing.is_exact() && nothing.is_empty(), "{nothing:?}");
    assert_eq!(
        nothing.faces_end(),
        numbers_turned_whole(&profile, FLAT, &turn).expect("a whole turn") as usize,
    );
}

#[test]
fn a_turn_declined_on_an_exact_body_counts_past_the_numbers_the_flats_give_it() {
    let on_the_axis = [
        DVec2::new(0.0, 0.0),
        DVec2::new(5.0, 0.0),
        DVec2::new(5.0, 2.0),
        DVec2::new(0.0, 2.0),
    ];
    let triangles = fan(&on_the_axis);
    let profile = straight(&on_the_axis, &triangles);
    for (angle, numbers) in [(TAU, 3), (PI / 2.0, 6), (1e-5, 0)] {
        let mut part = block(DVec2::ZERO, DVec2::splat(10.0), 10.0);
        part.count_past_turned(&profile, FLAT, &about_v(&profile, angle));
        assert_eq!(part.faces_end(), 6 + numbers, "turned {angle} rad");
    }

    let mut flats = ring_turned_about_y();
    let end = flats.faces_end();
    flats.count_past_turned(&profile, FLAT, &about_v(&profile, TAU));
    assert_eq!(flats.faces_end(), end, "the flats never decline");
}

#[test]
fn a_body_tells_whether_the_exact_kernel_computed_it() {
    let part = block(DVec2::ZERO, DVec2::splat(10.0), 10.0);
    assert!(part.is_exact());
    assert!(Body::default().is_exact());
    let ring = ring_turned_about_y();
    assert!(!ring.is_exact());
    assert!(
        !part
            .union(&ring)
            .expect("the flats never decline")
            .is_exact()
    );
}

/// A shaft of radius five about Z, thirty high, cut by a block whose floor
/// leans a ten-thousandth of a degree off square to it: a step the kernel
/// builds and reads unstably (#552).
fn shaft_cut_a_ten_thousandth_of_a_degree_off_square() -> Body {
    let shaft = raised(
        &Body::default(),
        &exact(disc(DVec2::ZERO, 5.0), vec![]),
        30.0,
    );
    let (sin, cos) = (1e-4f64).to_radians().sin_cos();
    let leaning = Frame {
        origin: DVec3::new(0.0, 0.0, 20.0),
        u: DVec3::X,
        v: DVec3::new(0.0, cos, sin),
    };
    let tool = shaft
        .tool_raised(
            &exact(
                Contour::rectangle(DVec2::splat(-10.0), DVec2::splat(10.0)),
                vec![],
            ),
            leaning,
            leaning.normal() * 20.0,
        )
        .expect("a block raises");
    shaft.difference(&tool).expect("the kernel cuts it")
}

#[test]
fn a_cut_at_an_unstable_lean_is_handed_up_with_its_note() {
    let cut = shaft_cut_a_ten_thousandth_of_a_degree_off_square();

    let unstable = cut.unstable().expect("the step is noted");
    assert_eq!(unstable.lean, crate::Lean::Square);
    assert!(
        (unstable.degrees - 1e-4).abs() <= 1e-10,
        "{}°",
        unstable.degrees
    );
}

#[test]
fn a_tool_that_takes_nothing_hands_back_the_part_without_a_note() {
    let cut = shaft_cut_a_ten_thousandth_of_a_degree_off_square();

    let again = cut
        .difference(&Body::default())
        .expect("nothing to take is taken");

    assert_eq!(again.unstable(), None);
}

mod chamfer;
mod fillet;
mod planes_of_several_axes;
mod section;
mod through_a_pole;
mod touching_a_ball;
