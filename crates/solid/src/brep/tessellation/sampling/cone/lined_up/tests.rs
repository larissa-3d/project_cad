//! Which coaxial walls a round's grid is lined up with, read on the ring of
//! tube 2 about a ring radius of 10 about z, its outer equator of 12 and its
//! inner one of 8 on the plane z = 0, against walls built by hand: how each
//! stands beside the ring, whether its face reaches the equators' plane, and
//! what the ring's steps a turn become.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::super::super::divisions_in;
use super::super::super::round::Rounded;
use super::{Beside, beside, kept_by_walls, reaches_the_equators};
use crate::brep::curve::{Circle, Curve, Line};
use crate::brep::surface::{Cylinder, Sphere, Surface, Torus};
use crate::brep::tessellation::tests::fixtures::Build;
use crate::brep::topology::{Body, Coedge, SurfaceId};

/// The tolerance the walls are read against.
const EPS: f64 = 1e-8;

/// The ring's rows a turn along its meridian.
const ROWS: usize = 96;

/// The drawing's tolerance and the span it is drawn over.
const FINENESS: (f64, f64) = (1e-2, 20.0);

/// The ring's own steps a turn, a multiple of none of the walls' grids here.
const STEPS: usize = 236;

/// The most steps a turn is cut into.
const CAP: usize = 1024;

/// The ring of tube 2 about a ring radius of 10 about z through the origin.
fn ring() -> Rounded {
    Rounded::Torus(Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(0.0, 10.0),
        2.0,
        DVec2::new(0.0, 12.0),
    ))
}

/// The wall of `radius` about z through the origin, its angle read from
/// where the ring's is.
fn wall(radius: f64) -> Cylinder {
    Cylinder::about(DVec3::ZERO, DVec3::Z, radius)
}

/// How `wall` stands beside the ring, given `rims`, read within `eps` and
/// `reach`, its face reaching the equators' plane or not.
fn beside_the_ring(
    wall: &Cylinder,
    rims: &[&Circle],
    (eps, reach): (f64, f64),
    reaching: bool,
) -> Option<Beside> {
    beside(&ring(), wall, rims, ROWS, (eps, reach), || reaching)
}

/// The steps a turn of a wall of `radius`.
fn grid(radius: f64) -> usize {
    divisions_in(radius, FINENESS.0, FINENESS.1)
}

/// The least multiple of `of` that is no fewer than `steps`.
fn multiple(steps: usize, of: usize) -> usize {
    steps.div_ceil(of) * of
}

/// The steps a turn of `round`, lined up with the walls of `body`.
fn kept(round: &Rounded, body: &Body, steps: usize, cap: usize) -> usize {
    kept_by_walls(body, round, (steps, ROWS, cap), FINENESS, EPS)
}

/// The ball of 5 about the origin, turned about z.
fn ball() -> Rounded {
    Rounded::Sphere(Sphere::through(DVec3::ZERO, DVec3::Z, 0.0, 5.0))
}

/// A body of one wall of `radius`, its face between the circles at `low`
/// and `high`.
fn wall_between(radius: f64, low: f64, high: f64) -> Body {
    let mut build = Build::new();
    let id = build.surface(Surface::Cylinder(wall(radius)));
    let (bottom, top) = (build.circle(id, low, None), build.circle(id, high, None));
    build.face(
        id,
        true,
        vec![
            vec![Coedge {
                edge: bottom,
                forward: false,
            }],
            vec![Coedge {
                edge: top,
                forward: true,
            }],
        ],
    );
    build.finish(20.0)
}

/// The radius of the circle of the ring's row `rank` along the meridian.
fn row(rank: f64) -> f64 {
    10.0 + 2.0 * (rank * TAU / ROWS as f64).cos()
}

#[test]
fn a_wall_on_the_ring_s_inner_or_outer_equator_touches_it_wherever_its_origin_stands_along_the_axis()
 {
    for radius in [8.0, 12.0] {
        for along in [0.0, 5.0] {
            let standing = Cylinder {
                origin: DVec3::Z * along,
                ..wall(radius)
            };

            let found = beside_the_ring(&standing, &[], (EPS, 1e-3), false);

            assert!(
                matches!(found, Some(Beside::Touching(gap)) if gap <= EPS),
                "a wall of {radius} whose origin stands {along} along the axis: {found:?}"
            );
        }
    }
}

#[test]
fn a_wall_off_the_ring_s_axis_or_leaning_from_it_stands_beside_it_nowhere_even_at_an_equator_s_radius()
 {
    let off = Cylinder::about(DVec3::X * 3.0, DVec3::Z, 8.0);
    let leaning = Cylinder::about(DVec3::ZERO, DVec3::new(1e-8, 0.0, 1.0), 8.0);

    for (what, standing) in [("3 off the axis", off), ("leaning 1e-8", leaning)] {
        let found = beside_the_ring(&standing, &[], (EPS, 1e-3), true);

        assert!(found.is_none(), "a wall of 8 {what}: {found:?}");
    }
}

#[test]
fn a_wall_as_far_off_the_axis_or_leaning_by_as_much_as_the_tolerance_stands_on_it() {
    let off = Cylinder::about(DVec3::X * 0.5, DVec3::Z, 8.0);
    let leaning = Cylinder::about(DVec3::ZERO, DVec3::new(1e-3, 0.0, 1.0), 8.0);
    let leans = leaning.axis.cross(ring().axis()).length() * leaning.radius;

    for (what, standing, eps) in [
        ("0.5 off the axis", off, 0.5),
        ("leaning by the tolerance", leaning, leans),
    ] {
        let found = beside_the_ring(&standing, &[], (eps, 1e-3), false);

        assert!(
            matches!(found, Some(Beside::Touching(_))),
            "a wall of 8 {what}, read within {eps}: {found:?}"
        );
    }
}

#[test]
fn a_wall_sharing_a_rim_with_the_ring_a_hair_inside_the_circle_of_a_row_far_from_its_equators_stands_apart_from_it_by_that_hair()
 {
    let radius = row(10.0) - 1e-4;
    let standing = wall(radius);
    let latitude = ((radius - 10.0) / 2.0).acos();
    let rim = Circle::about(DVec3::Z * (2.0 * latitude.sin()), DVec3::Z, radius);

    let sharing = beside_the_ring(&standing, &[&rim], (EPS, 1e-3), false);
    let sharing_none = beside_the_ring(&standing, &[], (EPS, 1e-3), false);

    assert!(
        matches!(sharing, Some(Beside::Apart(gap)) if (gap - 1e-4).abs() <= 1e-12),
        "the wall sharing the rim: {sharing:?}"
    );
    assert!(
        sharing_none.is_none(),
        "the wall sharing no rim: {sharing_none:?}"
    );
}

/// A body of one wall of 8, its face held by one edge along it, the line
/// x = 8, y = 0, from z = `low` to z = `high`.
fn wall_along(low: f64, high: f64) -> (Body, SurfaceId) {
    let mut build = Build::new();
    let id = build.surface(Surface::Cylinder(wall(8.0)));
    let ends = [
        build.vertex(DVec3::new(8.0, 0.0, low)),
        build.vertex(DVec3::new(8.0, 0.0, high)),
    ];
    let line = Line {
        origin: DVec3::X * 8.0,
        direction: DVec3::Z,
    };
    let edge = build.edge(Curve::Line(line), Some(ends), low, high);
    build.face(
        id,
        false,
        vec![vec![Coedge {
            edge,
            forward: true,
        }]],
    );
    (build.finish(20.0), id)
}

#[test]
fn a_wall_s_face_reaches_the_equators_plane_where_its_edges_read_along_their_length_come_within_the_window_of_it()
 {
    for (low, high, reaches) in [
        (0.1, 4.0, true),
        (-3.0, 3.0, true),
        (-3.0, -1.0, false),
        (0.6, 4.0, false),
    ] {
        let (body, id) = wall_along(low, high);

        let found = reaches_the_equators(&body, id, &ring(), 0.5);

        assert_eq!(
            found, reaches,
            "a face from z = {low} to z = {high}, within 0.5 of the plane z = 0"
        );
    }
}

#[test]
fn the_ring_s_steps_become_a_multiple_of_a_touching_wall_s_grid_where_the_two_start_a_whole_step_of_it_apart_and_stay_as_they_are_half_a_step_off()
 {
    let theirs = grid(8.0);
    assert_ne!(multiple(STEPS, theirs), STEPS, "the ring's own steps");
    let Rounded::Torus(torus) = ring() else {
        unreachable!("the ring is a torus")
    };
    for (steps_apart, lined_up) in [(1.0, true), (0.5, false)] {
        let (sin, cos) = (TAU / theirs as f64 * steps_apart).sin_cos();
        let u = torus.u * cos + torus.v * sin;
        let turned = Cylinder {
            u,
            v: DVec3::Z.cross(u),
            ..wall(8.0)
        };
        let mut build = Build::new();
        build.surface(Surface::Cylinder(turned));
        let body = build.finish(20.0);

        let steps = kept(&ring(), &body, STEPS, CAP);

        let expected = if lined_up {
            multiple(STEPS, theirs)
        } else {
            STEPS
        };
        assert_eq!(
            steps, expected,
            "a wall of 8 whose grid starts {steps_apart} of its steps from the ring's"
        );
    }
}

#[test]
fn a_touching_wall_whose_multiple_would_pass_the_most_a_turn_is_cut_into_leaves_the_ring_s_steps_as_they_were()
 {
    let theirs = grid(8.0);
    let mut build = Build::new();
    build.surface(Surface::Cylinder(wall(8.0)));
    let body = build.finish(20.0);
    let steps = theirs + 4;

    for (cap, expected) in [(2 * theirs - 4, steps), (2 * theirs, 2 * theirs)] {
        assert_eq!(
            kept(&ring(), &body, steps, cap),
            expected,
            "{steps} steps against a wall of {theirs}, cut into {cap} at most"
        );
    }
}

#[test]
fn a_circle_on_a_wall_through_the_ring_s_tube_centre_that_is_no_latitude_of_the_ring_is_no_rim_it_shares_with_it()
 {
    assert_ne!(multiple(STEPS, grid(10.0)), STEPS, "the ring's own steps");
    let mut build = Build::new();
    let id = build.surface(Surface::Cylinder(wall(10.0)));
    build.circle(id, 5.0, None);
    let body = build.finish(20.0);

    assert_eq!(kept(&ring(), &body, STEPS, CAP), STEPS);
}

/// Walls inside the ring's inner equator, or the ball's equator, by half,
/// eight tenths of, or twice the sag of their chords twice over, `h`: the
/// round stays within `h` of such a wall over √(2 r h) either side of the
/// equators' plane, the window the wall's face is held to. Each face runs
/// 4 up from a share of that window off the plane, under it or over it.
#[test]
fn a_wall_inside_an_equator_lines_the_round_up_where_it_stands_within_twice_its_sag_of_it_and_its_face_comes_within_the_window_of_the_equators_plane()
 {
    let sag = |radius: f64| 2.0 * radius * (1.0 - (PI / grid(radius) as f64).cos());
    for (round, equator, what) in [(ring(), 8.0, "the ring"), (ball(), 5.0, "the ball")] {
        for (inside, from, lined_up) in [
            (0.5, 0.9, true),
            (0.5, 2.0, false),
            (0.8, -2.0, true),
            (2.0, -2.0, false),
        ] {
            let radius = equator - inside * sag(equator);
            let window = (2.0 * round.radius() * sag(radius)).sqrt();
            let theirs = grid(radius);
            assert_ne!(multiple(STEPS, theirs), STEPS, "the round's own steps");
            let low = from * window;

            let steps = kept(&round, &wall_between(radius, low, low + 4.0), STEPS, CAP);

            let expected = if lined_up {
                multiple(STEPS, theirs)
            } else {
                STEPS
            };
            assert_eq!(
                steps, expected,
                "{what} beside a wall {inside} of its sag inside its equator, its face from \
                 {from} windows of {window} off the equators' plane"
            );
        }
    }
}
