//! The curve a cylinder meets a sphere along (#805), read on the sphere: its
//! `(θ, φ)` where the sphere reads its points, the angle unwrapped along the
//! curve and across its periods, the two derivatives the differences of those
//! readings, the reading's two guards, and where the curve turns back.

use super::*;
use crate::brep::meet::Meeting;
use crate::brep::surface::Sphere;

/// A ball of radius 3 about `(h = 5)` on a slanted axis through a point off
/// the origin, so that its centre stands off the axis of the world's origin
/// in the ball's own `(u, v)`.
fn ball() -> Sphere {
    Sphere::through(
        DVec3::new(1.0, -2.0, 3.0),
        DVec3::new(0.0, 1.0, 1.0),
        5.0,
        3.0,
    )
}

/// The cylinder of `radius` along `axis`, read in the ball's own `(u, v,
/// axis)`, its axis `off` from the ball's centre towards `out`, the
/// cylinder's own origin run back along it.
fn wall(axis: [f64; 3], out: [f64; 3], radius: f64, off: f64) -> Cylinder {
    let ball = ball();
    let world = |[u, v, w]: [f64; 3]| ball.u * u + ball.v * v + ball.axis * w;
    let axis = world(axis).normalize();
    let out = world(out).reject_from(axis).normalize();
    Cylinder::about(ball.middle() + out * off - axis * 7.0, axis, radius)
}

/// The components of the curve `cylinder` meets the ball along.
fn components(cylinder: &Cylinder) -> Vec<Meet> {
    Meeting::of(&Wall::Cylinder(*cylinder), &Wall::Sphere(ball()), scale()).components
}

/// Cylinders meeting the ball along one loop or two: along the ball's axis,
/// square to it and at a slant, some holding a pole, whose loop round it
/// winds round the axis.
fn walls() -> Vec<Cylinder> {
    let slant = [0.7, 0.3, 1.0];
    vec![
        wall([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 2.0, 2.5),
        wall([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 1.0, 1.5),
        wall([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 1.0, 0.5),
        wall([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 1.0, 1.5),
        wall([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 2.0, 2.5),
        wall(slant, [1.0, -1.0, 0.0], 1.2, 1.0),
        wall(slant, [1.0, -1.0, 0.0], 2.0, 2.5),
    ]
}

/// How many times a loop winds round the ball's axis over its period, read
/// off its points alone: the angle the ball reads them at, its steps summed
/// each within half a turn.
fn winding(meet: &Meet) -> f64 {
    let ball = ball();
    let period = meet.period().expect("a loop closes");
    let steps = 2000;
    let angle = |step: u32| {
        ball.parameters(meet.point(period * f64::from(step) / f64::from(steps)))
            .x
    };
    let swept: f64 = (0..steps)
        .map(|step| around(angle(step + 1) - angle(step)))
        .sum();
    (swept / TAU).round()
}

#[test]
fn the_curve_a_cylinder_meets_a_ball_along_is_seen_where_the_ball_reads_its_points_its_angle_unbroken_along_it_and_across_its_periods()
 {
    let ball = ball();
    let mut wound = 0;
    for cylinder in walls() {
        let found = components(&cylinder);
        assert!(!found.is_empty(), "{cylinder:?}");
        for meet in found {
            let period = meet.period().expect("a loop closes");
            let winds = winding(&meet);
            wound += usize::from(winds != 0.0);
            let steps = 600;
            let mut last: Option<f64> = None;
            for step in 0..=steps {
                let t = -period + 3.0 * period * f64::from(step) / f64::from(steps);
                let [seen, ..] = meet.seen_on(false, t);
                let read = ball.parameters(meet.point(t));
                assert!(
                    around(seen.x - read.x).abs() < 1e-11 && (seen.y - read.y).abs() < 1e-11,
                    "{meet:?} at {t}: seen at {seen}, the ball reads {read}"
                );
                if let Some(last) = last {
                    assert!(
                        (seen.x - last).abs() < 0.5,
                        "{meet:?} at {t}: the angle leaps from {last} to {}",
                        seen.x
                    );
                }
                last = Some(seen.x);
                let [later, ..] = meet.seen_on(false, t + period);
                let gained = later.x - seen.x;
                assert!(
                    (gained - winds * TAU).abs() < 1e-9,
                    "{meet:?} at {t}: a period on, the angle gains {gained}, where the loop winds \
                     {winds} times round the axis"
                );
            }
        }
    }
    assert!(wound >= 3, "{wound} loops wind round the axis");
}

#[test]
fn the_curve_a_cylinder_meets_a_ball_along_turns_on_the_ball_as_the_differences_of_its_readings_say()
 {
    let step = 1e-5;
    for cylinder in walls() {
        for meet in components(&cylinder) {
            let period = meet.period().expect("a loop closes");
            for share in 0..48 {
                let t = period * (f64::from(share) + 0.37) / 48.0;
                let [_, speed, bend] = meet.seen_on(false, t);
                let [[ahead, ahead_speed, _], [behind, behind_speed, _]] =
                    [t + step, t - step].map(|at| meet.seen_on(false, at));
                let differed = (ahead - behind) / (2.0 * step);
                let bent = (ahead_speed - behind_speed) / (2.0 * step);
                assert!(
                    (speed - differed).length() < 1e-6 * (1.0 + speed.length()),
                    "{meet:?} at {t}: the speed is {speed}, the differences give {differed}"
                );
                assert!(
                    (bend - bent).length() < 1e-5 * (1.0 + bend.length()),
                    "{meet:?} at {t}: the bend is {bend}, the differences give {bent}"
                );
            }
        }
    }
}

/// Three tenths of a thousandth of the radius under the pole's guard, a loop
/// a cylinder of radius 0.01 along the ball's axis cuts 0.012 off it, its
/// angle turning a few hundredths between two places read; and a loop a
/// cylinder of radius 1 cuts 1.01 off it, passing a three-hundredth of the
/// radius from the axis, whose angle swings round it by more than an eighth
/// of a turn between two places. Neither is charted on the ball, and both are
/// seen at no number there; the loops of a cylinder further off are, and so
/// is the loop a cylinder of radius 2 cuts 2.06 off the axis, taking in part
/// of the ball, whose angle turns by up to a third of a radian between two
/// of the places a period is read at, 512 a turn of its parameter.
#[test]
fn a_loop_on_a_ball_within_a_thousandth_of_its_radius_of_its_axis_or_swinging_round_it_between_two_places_read_is_not_charted()
 {
    let along = [0.0, 0.0, 1.0];
    let out = [1.0, 0.0, 0.0];
    for (cylinder, why) in [
        (wall(along, out, 0.01, 0.012), "near"),
        (wall(along, out, 1.0, 1.01), "swinging"),
    ] {
        let found = components(&cylinder);
        assert!(!found.is_empty(), "{why}: {cylinder:?}");
        for meet in found {
            assert!(!meet.charted_on_the_ball(), "{why}: {meet:?}");
            let [seen, speed, bend] = meet.seen_on(false, 0.3);
            assert!(
                seen.is_nan() && speed.is_nan() && bend.is_nan(),
                "{why}: {meet:?} seen at {seen}"
            );
        }
    }
    for meet in components(&wall(along, out, 1.0, 1.1)) {
        assert!(meet.charted_on_the_ball(), "{meet:?}");
    }
    let ball = ball();
    for meet in components(&wall(along, out, 2.0, 2.06)) {
        let period = meet.period().expect("a loop closes");
        let places = 512 * (period / TAU).round() as u32;
        let angle = |place: u32| {
            let at = period * f64::from(place) / f64::from(places);
            ball.parameters(meet.point(at)).x
        };
        let turned = (0..places)
            .map(|place| around(angle(place + 1) - angle(place)).abs())
            .fold(0.0, f64::max);
        assert!(
            turned > 0.2 && turned < 0.35,
            "the loop turns {turned} round the axis between two of {places} places"
        );
        assert!(meet.charted_on_the_ball(), "{meet:?}");
    }
}

/// On the cylinder, a loop turns back where its angle there stands still:
/// once a turn of its parameter where the cylinder takes in part of the ball,
/// never where it passes right through it. On the ball, whose readings are
/// laid out where the curve's samples are read, it is said to turn nowhere.
#[test]
fn the_curve_a_cylinder_meets_a_ball_along_turns_back_on_the_cylinder_where_its_angle_there_stands_still_and_never_on_the_ball()
 {
    for cylinder in walls() {
        for meet in components(&cylinder) {
            let period = meet.period().expect("a loop closes");
            let steps = 4000;
            let rate = |at: f64| meet.seen_on(true, at)[1].x;
            let places: Vec<f64> = (0..steps)
                .map(|step| period * (f64::from(step) + 0.5) / f64::from(steps))
                .collect();
            let still: Vec<f64> = (0..steps as usize)
                .filter(|&at| {
                    let next = if at + 1 == places.len() {
                        places[0] + period
                    } else {
                        places[at + 1]
                    };
                    rate(places[at]) * rate(next) < 0.0
                })
                .map(|at| places[at])
                .collect();
            let turns = meet.turns(true);
            assert_eq!(
                turns.len(),
                still.len(),
                "{meet:?}: turns back at {turns:?}, its angle stands still near {still:?}"
            );
            for turn in &turns {
                assert!(
                    still.iter().any(|near| {
                        let apart = (near - turn).rem_euclid(period);
                        apart.min(period - apart) <= 2.0 * period / f64::from(steps)
                    }),
                    "{meet:?}: turns back at {turn}, its angle stands still near {still:?}"
                );
            }
            assert!(
                meet.turns(false).is_empty(),
                "{meet:?}: on the ball {:?}",
                meet.turns(false)
            );
        }
    }
    for meet in components(&wall([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 2.0, 2.5)) {
        assert!(!meet.turns(true).is_empty(), "{meet:?}");
    }
}
