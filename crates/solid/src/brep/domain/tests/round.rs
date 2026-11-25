//! The floors of a sphere's chart (#576): which poles a face holds, read by
//! parity off its traces, and the domain closed along them for parity; the
//! cut of a ring's chart a face winding round the tube is closed along.
//!
//! The parity is read from where a trace crosses the meridian squarely
//! (#1117), and so not from a circle's middle where it turns back a hair off
//! the meridian, nor from a slit, which bounds nothing however squarely it
//! runs; and a loop left open at a pole is crossed on that pole's floor,
//! not halfway up the sector it closes.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use crate::brep::curve::Circle;
use crate::brep::domain::crossing::heights;
use crate::brep::domain::floor::{closed_round, holds, poles_of};
use crate::brep::surface::{Sphere, Surface};
use crate::brep::trace::Trace;

fn ball() -> Surface {
    Surface::Sphere(Sphere::through(DVec3::ZERO, DVec3::Z, 0.0, 2.0))
}

/// A whole latitude circle at `phi`, run with the angle growing when
/// `forward`: the face on its left lies above it then.
fn latitude(phi: f64, forward: bool) -> Vec<Trace> {
    let (from, to) = if forward { (0.0, TAU) } else { (TAU, 0.0) };
    vec![Trace::Segment {
        from: DVec2::new(from, phi),
        to: DVec2::new(to, phi),
    }]
}

/// A whole meridian circle at `theta`, run with the angle along the
/// meridian growing when `up`.
fn meridian(theta: f64, up: bool) -> Vec<Trace> {
    let (from, to) = if up { (-PI, PI) } else { (PI, -PI) };
    vec![Trace::Segment {
        from: DVec2::new(theta, from),
        to: DVec2::new(theta, to),
    }]
}

/// How many times a ray up the meridian from `at` crosses the closed loops.
fn crossed(closed: &[Vec<Trace>], at: DVec2) -> usize {
    heights(closed, at.x, true)
        .into_iter()
        .filter(|height| *height > at.y)
        .count()
}

#[test]
fn a_face_with_no_loop_holds_both_poles_and_a_ray_up_from_inside_crosses_one_floor() {
    let poles = poles_of(&ball(), 1e-9);
    assert_eq!(poles.len(), 2);
    assert!(!poles[0].high && poles[1].high);
    assert!((poles[0].at + FRAC_PI_2).abs() < 1e-15 && (poles[1].at - FRAC_PI_2).abs() < 1e-15);
    assert!(poles[0].point.distance(DVec3::new(0.0, 0.0, -2.0)) < 1e-15);
    assert!(poles.iter().all(|pole| holds(&[], pole, false)));
    let closed = closed_round(&poles, Vec::new(), false, None);
    assert_eq!(closed.len(), 2);
    for at in [
        DVec2::new(0.7, 0.3),
        DVec2::new(-2.0, -1.4),
        DVec2::new(3.0, 1.5),
    ] {
        assert_eq!(crossed(&closed, at), 1, "{at}");
    }
}

#[test]
fn a_cap_holds_the_pole_it_lies_towards() {
    let poles = poles_of(&ball(), 1e-9);
    let cap = vec![latitude(0.4, true)];
    assert!(!holds(&cap, &poles[0], false) && holds(&cap, &poles[1], false));
    let bowl = vec![latitude(0.4, false)];
    assert!(holds(&bowl, &poles[0], false) && !holds(&bowl, &poles[1], false));

    let closed = closed_round(&poles, cap, false, None);
    assert_eq!(closed.len(), 2);
    assert_eq!(crossed(&closed, DVec2::new(1.0, 1.0)) % 2, 1);
    assert_eq!(crossed(&closed, DVec2::new(1.0, 0.0)) % 2, 0);
    let closed = closed_round(&poles, bowl, false, None);
    assert_eq!(crossed(&closed, DVec2::new(1.0, -1.0)) % 2, 1);
    assert_eq!(crossed(&closed, DVec2::new(1.0, 1.0)) % 2, 0);
}

#[test]
fn a_zone_holds_no_pole_and_a_lune_through_both_is_closed_along_them() {
    let poles = poles_of(&ball(), 1e-9);
    let caps = vec![latitude(0.8, true), latitude(-0.5, false)];
    assert!(poles.iter().all(|pole| holds(&caps, pole, false)));
    let zone = vec![latitude(0.8, false), latitude(-0.5, true)];
    assert!(poles.iter().all(|pole| !holds(&zone, pole, false)));
    let closed = closed_round(&poles, zone, false, None);
    assert_eq!(closed.len(), 2);
    assert_eq!(crossed(&closed, DVec2::new(2.0, 0.1)) % 2, 1);
    assert_eq!(crossed(&closed, DVec2::new(2.0, 1.2)) % 2, 0);
    assert_eq!(crossed(&closed, DVec2::new(2.0, -1.2)) % 2, 0);

    let lune = vec![vec![
        Trace::Segment {
            from: DVec2::new(1.0, -FRAC_PI_2),
            to: DVec2::new(1.0, FRAC_PI_2),
        },
        Trace::Segment {
            from: DVec2::new(0.0, FRAC_PI_2),
            to: DVec2::new(0.0, -FRAC_PI_2),
        },
    ]];
    assert!(poles.iter().all(|pole| !holds(&lune, pole, false)));
    let closed = closed_round(&poles, lune, false, None);
    assert_eq!(closed.len(), 1);
    assert_eq!(closed[0].len(), 4);
    assert_eq!(crossed(&closed, DVec2::new(0.5, 0.0)) % 2, 1);
    assert_eq!(crossed(&closed, DVec2::new(3.0, 0.0)) % 2, 0);
}

#[test]
fn a_face_holding_a_pole_beside_a_hole_that_winds_round_nothing_holds_it_still() {
    let poles = poles_of(&ball(), 1e-9);
    let hole = vec![vec![
        Trace::Segment {
            from: DVec2::new(1.0, 0.2),
            to: DVec2::new(1.0, 0.6),
        },
        Trace::Segment {
            from: DVec2::new(1.0, 0.6),
            to: DVec2::new(2.0, 0.6),
        },
        Trace::Segment {
            from: DVec2::new(2.0, 0.6),
            to: DVec2::new(2.0, 0.2),
        },
        Trace::Segment {
            from: DVec2::new(2.0, 0.2),
            to: DVec2::new(1.0, 0.2),
        },
    ]];
    assert!(poles.iter().all(|pole| holds(&hole, pole, false)));
    let patch: Vec<Vec<Trace>> = hole
        .iter()
        .map(|lap| lap.iter().rev().map(Trace::reversed).collect())
        .collect();
    assert!(poles.iter().all(|pole| !holds(&patch, pole, false)));
}

#[test]
fn a_flipped_face_lies_on_the_other_side_of_its_traces_and_is_joined_at_its_poles_the_other_way_round()
 {
    let poles = poles_of(&ball(), 1e-9);
    let hollow = vec![latitude(0.4, false)];
    assert!(!holds(&hollow, &poles[0], true) && holds(&hollow, &poles[1], true));
    let band = vec![latitude(0.8, true), latitude(-0.5, false)];
    assert!(poles.iter().all(|pole| !holds(&band, pole, true)));
    let closed = closed_round(&poles, hollow, true, None);
    assert_eq!(crossed(&closed, DVec2::new(1.0, 1.0)) % 2, 1);
    assert_eq!(crossed(&closed, DVec2::new(1.0, 0.0)) % 2, 0);

    let lune = vec![vec![
        Trace::Segment {
            from: DVec2::new(0.0, -FRAC_PI_2),
            to: DVec2::new(0.0, FRAC_PI_2),
        },
        Trace::Segment {
            from: DVec2::new(1.0, FRAC_PI_2),
            to: DVec2::new(1.0, -FRAC_PI_2),
        },
    ]];
    let closed = closed_round(&poles, lune, true, None);
    assert_eq!(closed.len(), 1);
    assert_eq!(closed[0].len(), 4);
    assert_eq!(crossed(&closed, DVec2::new(0.5, 0.0)) % 2, 1);
    assert_eq!(crossed(&closed, DVec2::new(3.0, 0.0)) % 2, 0);
}

#[test]
fn a_ring_s_whole_face_and_a_ring_turned_part_way_are_closed_along_the_cut_of_the_chart() {
    let seam = 0.3;
    let whole = closed_round(&[], Vec::new(), false, Some(seam));
    assert_eq!(whole.len(), 1);
    assert_eq!(whole[0].len(), 2);
    for at in [
        DVec2::new(0.7, seam),
        DVec2::new(-2.0, seam + 3.0),
        DVec2::new(3.0, seam - 3.0),
    ] {
        assert_eq!(crossed(&whole, at), 1, "{at}");
    }
    assert!(closed_round(&[], Vec::new(), false, None).is_empty());

    let part = closed_round(
        &[],
        vec![meridian(1.0, true), meridian(0.0, false)],
        false,
        Some(seam),
    );
    assert_eq!(part.len(), 3);
    assert_eq!(part[2].len(), 2, "the two cuts are one lap");
    assert_eq!(crossed(&part, DVec2::new(0.5, 0.0)) % 2, 1);
    assert_eq!(crossed(&part, DVec2::new(0.5, 2.5)) % 2, 1);
    assert_eq!(crossed(&part, DVec2::new(3.0, 0.0)) % 2, 0);
    assert_eq!(crossed(&part, DVec2::new(-0.5, 0.0)) % 2, 0);
}

/// Straight traces from each corner to the next, the last back to the first.
fn polygon(corners: &[[f64; 2]]) -> Vec<Trace> {
    (0..corners.len())
        .map(|rank| Trace::Segment {
            from: DVec2::from(corners[rank]),
            to: DVec2::from(corners[(rank + 1) % corners.len()]),
        })
        .collect()
}

/// A slit from `from` to `to`, run there and back, each of its two uses
/// first in turn.
fn slit(from: [f64; 2], to: [f64; 2]) -> [Vec<Trace>; 2] {
    let there = polygon(&[from, to]);
    let back = vec![there[1], there[0]];
    [there, back]
}

/// A slit has the face on either side and bounds nothing, however steeply
/// it runs across the meridian: a face round a hole that winds round
/// nothing, slit beside it along a latitude, holds both poles, and the patch
/// that hole would be, slit inside, holds neither — whichever use of the
/// slit comes first, and whichever loop.
#[test]
fn a_slit_beside_a_hole_or_inside_a_patch_bounds_nothing_whichever_of_its_uses_comes_first() {
    let poles = poles_of(&ball(), 1e-9);
    let hole = polygon(&[[-1.0, 0.0], [0.0, 0.5], [1.0, 0.0], [0.0, -0.5]]);
    let patch: Vec<Trace> = hole.iter().rev().map(Trace::reversed).collect();
    for (rank, slit) in slit([2.0, 0.3], [2.5, 0.3]).into_iter().enumerate() {
        for face in [
            vec![hole.clone(), slit.clone()],
            vec![slit.clone(), hole.clone()],
        ] {
            assert!(
                poles.iter().all(|pole| holds(&face, pole, false)),
                "the face round the hole, slit beside it, use {rank} of the slit first: {face:?}"
            );
        }
    }
    for (rank, slit) in slit([0.2, 0.1], [0.5, 0.1]).into_iter().enumerate() {
        for face in [
            vec![patch.clone(), slit.clone()],
            vec![slit.clone(), patch.clone()],
        ] {
            assert!(
                poles.iter().all(|pole| !holds(&face, pole, false)),
                "the patch slit inside, use {rank} of the slit first: {face:?}"
            );
        }
    }
}

/// A sector of the ball through its high pole, `turn` wide: down the
/// meridian at nought, along an arc, up the meridian at `turn` and back to
/// the pole, where the loop is left open.
fn sector(turn: f64) -> Vec<Trace> {
    polygon(&[
        [0.0, FRAC_PI_2],
        [0.0, 1.4],
        [0.3, 1.3],
        [turn, 0.2],
        [turn, FRAC_PI_2],
    ])
    .into_iter()
    .take(4)
    .collect()
}

/// A sector of the ball a quarter turn wide, through its high pole, holds
/// the low pole no more than the high one, where its loop passes: the
/// meridian it reaches the pole by runs on along the pole's floor, not
/// across the sector halfway up. The rest of the ball, the sector run the
/// other way, holds the low pole.
#[test]
fn a_sector_a_quarter_turn_wide_through_a_pole_holds_neither_pole_and_the_rest_of_the_ball_the_far_one()
 {
    let poles = poles_of(&ball(), 1e-9);
    let sector = vec![sector(FRAC_PI_2)];
    assert!(!holds(&sector, &poles[0], false), "the far pole");
    assert!(!holds(&sector, &poles[1], false), "the pole it passes");
    let rest: Vec<Vec<Trace>> = sector
        .iter()
        .map(|lap| lap.iter().rev().map(Trace::reversed).collect())
        .collect();
    assert!(
        holds(&rest, &poles[0], false),
        "the rest holds the far pole"
    );
    assert!(
        !holds(&rest, &poles[1], false),
        "the rest at the pole it passes"
    );
}

/// The same over three quarters of a turn, the floor it is closed along at
/// the pole the shorter way round.
#[test]
fn a_sector_three_quarters_of_a_turn_wide_through_a_pole_holds_neither_pole_and_the_rest_of_the_ball_the_far_one()
 {
    let poles = poles_of(&ball(), 1e-9);
    let sector = vec![sector(3.0 * FRAC_PI_2)];
    assert!(!holds(&sector, &poles[0], false), "the far pole");
    assert!(!holds(&sector, &poles[1], false), "the pole it passes");
    let rest: Vec<Vec<Trace>> = sector
        .iter()
        .map(|lap| lap.iter().rev().map(Trace::reversed).collect())
        .collect();
    assert!(
        holds(&rest, &poles[0], false),
        "the rest holds the far pole"
    );
    assert!(
        !holds(&rest, &poles[1], false),
        "the rest at the pole it passes"
    );
}

/// The whole circle of a cap of 45° about the point of [`ball`]'s chart at
/// nought and `latitude`, from where it stands furthest one way in angle,
/// through its lowest and furthest the other way — its middle, where it
/// turns back along the meridian a hair off it — then its highest: run so,
/// the cap lies on its left; run back, the rest of the ball.
fn round_about(latitude: f64, cap: bool) -> Vec<Trace> {
    let sphere = Sphere::through(DVec3::ZERO, DVec3::Z, 0.0, 2.0);
    let towards = sphere.u * latitude.cos() + sphere.axis * latitude.sin();
    let reach = 45f64.to_radians();
    let u = -sphere.v;
    let circle = Circle {
        center: sphere.middle() + towards * sphere.radius * reach.cos(),
        axis: towards,
        radius: sphere.radius * reach.sin(),
        u,
        v: towards.cross(u),
    };
    let (from, to) = if cap { (0.0, TAU) } else { (TAU, 0.0) };
    vec![Trace::Tilted {
        circle,
        sphere,
        from,
        to,
    }]
}

/// A cap about a point a hair off the equator, its circle's middle where
/// the circle turns back along the meridian, holds neither pole, and the
/// rest of the ball holds both: the meridian through that middle all but
/// touches the circle there, and the parity is read from where the circle
/// crosses the meridian squarely.
#[test]
fn a_cap_about_a_point_a_hair_off_the_equator_holds_neither_pole_and_the_rest_of_the_ball_both() {
    let poles = poles_of(&ball(), 1e-9);
    for latitude in [0.0, 9.5e-8, -9.5e-8, 1e-7, -1e-7, 1e-6, -1e-6, 1e-8, -1e-8] {
        let cap = vec![round_about(latitude, true)];
        let rest = vec![round_about(latitude, false)];
        for pole in &poles {
            let which = if pole.high { "high" } else { "low" };
            assert!(
                !holds(&cap, pole, false),
                "the cap about {latitude} of latitude holds the {which} pole"
            );
            assert!(
                holds(&rest, pole, false),
                "the rest of the ball about {latitude} of latitude leaves the {which} pole out"
            );
        }
    }
}
