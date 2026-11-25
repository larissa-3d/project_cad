//! The latitudes a wall touches a sphere face along, read off bodies, and
//! the stretches of a face along a latitude, read off outlines laid by hand
//! in `(θ, φ)`: the face on the left of every segment, on either side of the
//! latitude.

use glam::{DVec2, DVec3};

use super::super::Outline;
use super::touched;
use crate::brep::surface::Surface;
use crate::brep::topology::Body;
use crate::brep::turned::tests::round::{ball, turned, zone};
use crate::profile::{Contour, Frame};

/// A tube along y about the origin, from y = −10 to 10, `outer` across
/// outside, bored `inner` across when there is one.
fn along_y(outer: f64, inner: Option<f64>) -> Body {
    let frame = Frame {
        origin: DVec3::Y * -10.0,
        u: DVec3::Z,
        v: DVec3::X,
    };
    let holes: Vec<Contour> = inner
        .map(|radius| Contour::circle(DVec2::ZERO, radius))
        .into_iter()
        .collect();
    Body::raised(
        &Contour::circle(DVec2::ZERO, outer),
        &holes,
        frame,
        DVec3::Y * 20.0,
    )
    .expect("the tube stands on its own")
}

/// A slab 10 thick above the plane y = 0.
fn above_the_equator() -> Body {
    let frame = Frame {
        origin: DVec3::ZERO,
        u: DVec3::Z,
        v: DVec3::X,
    };
    let side = Contour::rectangle(DVec2::splat(-20.0), DVec2::splat(20.0));
    Body::raised(&side, &[], frame, DVec3::Y * 10.0).expect("the slab stands on its own")
}

/// The latitudes [`touched`] finds on each sphere face of `body`.
fn touched_on_each_sphere_face(body: &Body) -> Vec<Vec<f64>> {
    body.face_ids()
        .filter_map(|id| {
            let face = body.face(id);
            match body.surface(face.surface) {
                Surface::Sphere(sphere) => Some(touched(body, face, sphere, body.scale().eps())),
                _ => None,
            }
        })
        .collect()
}

/// The zone of the ball of radius 5 about y between y = −3 and 3 in a
/// tube whose bore is the ball's radius: the bore touches each of the two
/// sphere faces along the equator, their one latitude a wall touches. The
/// ball bored along y by a bore of 2, which meets its sphere along two
/// circles about its axis the bore's wall runs along, but not of its
/// radius; and the ball cut at its equator by a plane, which leaves an edge
/// of its radius about its axis but no cylinder along it: no latitude
/// either.
#[test]
fn a_wall_touches_a_sphere_face_only_along_an_edge_of_its_own_radius_about_its_axis_a_cylinder_runs_along()
 {
    let zone = turned(&zone(), 360.0);
    let in_the_bore = zone
        .joined(&along_y(7.0, Some(5.0)))
        .expect("the zone joins the tube");
    let touched = touched_on_each_sphere_face(&in_the_bore);
    assert_eq!(touched.len(), 2, "two sphere faces: {touched:?}");
    for found in &touched {
        assert!(
            matches!(found[..], [equator] if equator.abs() <= 1e-12),
            "the equator alone: {found:?}"
        );
    }

    let ball = turned(&ball(), 360.0);
    let bored = ball
        .cut_by(&along_y(2.0, None))
        .expect("the bore cuts the ball");
    let halved = ball
        .cut_by(&above_the_equator())
        .expect("the plane cuts the ball");
    for (what, body) in [("bored", bored), ("halved", halved)] {
        let touched = touched_on_each_sphere_face(&body);
        assert!(!touched.is_empty(), "{what}: a sphere face");
        assert!(
            touched.iter().all(Vec::is_empty),
            "{what}: no latitude a wall touches, {touched:?}"
        );
    }
}

/// The latitude the outlines below run along, off the equator so that a
/// sign or a sum about it shows.
const LATITUDE: f64 = 0.3;

/// The step of the grid about the axis.
const STEP: f64 = 0.25;

/// How far into the face from the latitude the next row of the grid stands.
const ROW: f64 = 0.2;

/// The outline of `loops`, each given as `(θ, d)`, `d` how far into the
/// face from the latitude, the face on the left of each as it runs: laid
/// above the latitude when `above`, mirrored below it otherwise, each loop
/// then turned so that the face stays on its left.
fn laid(loops: &[&[[f64; 2]]], above: bool) -> Outline {
    let mut outline = Outline::default();
    let mut sample = 0;
    for lap in loops {
        let mut placed: Vec<DVec2> = (lap.iter())
            .map(|&[angle, into]| {
                DVec2::new(
                    angle,
                    if above {
                        LATITUDE + into
                    } else {
                        LATITUDE - into
                    },
                )
            })
            .collect();
        if !above {
            placed.reverse();
        }
        let ids: Vec<usize> = (placed.iter())
            .map(|&at| {
                sample += 1;
                outline.point(at, sample)
            })
            .collect();
        for (index, &from) in ids.iter().enumerate() {
            outline.segments.push([from, ids[(index + 1) % ids.len()]]);
        }
    }
    outline
}

/// The angles about the axis of the points of each stretch of the face
/// along the latitude, up to the row on its side.
fn angles_of_the_stretches(outline: &Outline, above: bool) -> Vec<Vec<f64>> {
    let row = if above {
        LATITUDE + ROW
    } else {
        LATITUDE - ROW
    };
    (outline.stretches(LATITUDE, row, above, STEP).iter())
        .map(|stretch| {
            stretch
                .iter()
                .map(|&point| outline.points[point].x)
                .collect()
        })
        .collect()
}

/// Holds that the face of `loops`, laid on either side of the latitude, has
/// the stretches at `angles` along it.
fn stretches_either_side(loops: &[&[[f64; 2]]], angles: &[&[f64]], what: &str) {
    let angles: Vec<Vec<f64>> = angles.iter().map(|stretch| stretch.to_vec()).collect();
    for above in [true, false] {
        assert_eq!(
            angles_of_the_stretches(&laid(loops, above), above),
            angles,
            "{what}, the face {} the latitude",
            if above { "above" } else { "below" }
        );
    }
}

/// A face running along the latitude from 0 to 1, a point of it at 0.375
/// between two angles of the grid, a hole beyond the row: one stretch, from
/// the first angle of the grid past where the run starts to the last before
/// it ends, through the point off the grid.
#[test]
fn a_face_running_along_the_latitude_is_one_stretch_from_the_first_angle_of_the_grid_past_the_run_s_start_to_the_last_before_its_end()
 {
    stretches_either_side(
        &[
            &[
                [0.0, 0.0],
                [0.25, 0.0],
                [0.375, 0.0],
                [0.5, 0.0],
                [0.75, 0.0],
                [1.0, 0.0],
                [1.0, 0.7],
                [0.0, 0.7],
            ],
            &[
                [0.25, 0.4],
                [0.25, 0.5],
                [0.5, 0.5],
                [0.75, 0.5],
                [0.75, 0.4],
            ],
        ],
        &[&[0.25, 0.5, 0.75]],
        "a hole beyond the row",
    );
}

/// The face running along the latitude from 0 to 1.5.
const ALONG: [[f64; 2]; 9] = [
    [0.0, 0.0],
    [0.25, 0.0],
    [0.5, 0.0],
    [0.75, 0.0],
    [1.0, 0.0],
    [1.25, 0.0],
    [1.5, 0.0],
    [1.5, 0.7],
    [0.0, 0.7],
];

/// A hole between the latitude and the row, from 0.55 to 0.7 about the
/// axis: the stretch is broken between 0.5 and 0.75, and taken up again
/// past them. So it is where the hole rests on the row. Where its sides
/// stand on 0.5 and 0.75, the stretch is broken on either side of each, the
/// strip's rungs there running along them.
#[test]
fn a_boundary_coming_between_the_latitude_and_the_row_breaks_the_stretch_at_the_angles_of_the_grid_either_side_of_it()
 {
    for (hole, angles, what) in [
        (
            [[0.55, 0.05], [0.55, 0.1], [0.7, 0.1], [0.7, 0.05]],
            &[&[0.25, 0.5][..], &[0.75, 1.0, 1.25]][..],
            "a hole inside the strip",
        ),
        (
            [[0.55, ROW], [0.55, 0.3], [0.7, 0.3], [0.7, ROW]],
            &[&[0.25, 0.5][..], &[0.75, 1.0, 1.25]][..],
            "a hole resting on the row",
        ),
        (
            [[0.5, 0.05], [0.5, 0.1], [0.75, 0.1], [0.75, 0.05]],
            &[&[1.0, 1.25][..]][..],
            "a hole whose sides stand on two angles of the grid",
        ),
    ] {
        stretches_either_side(&[&ALONG, &hole], angles, what);
    }
}

/// A face running along the latitude from 0 to 1, whose boundary then
/// leaves it away from the face, the face going on past the latitude
/// beyond 1: the point at 1 ends the run along the latitude, and stands in
/// no stretch.
#[test]
fn the_end_of_a_run_along_the_latitude_stands_in_no_stretch_even_where_the_boundary_leaves_it_away_from_the_face()
 {
    stretches_either_side(
        &[&[
            [0.0, 0.0],
            [0.25, 0.0],
            [0.5, 0.0],
            [0.75, 0.0],
            [1.0, 0.0],
            [1.0, -0.3],
            [1.5, -0.3],
            [1.5, 0.7],
            [0.0, 0.7],
        ]],
        &[&[0.25, 0.5, 0.75]],
        "a run turning away from the face at its end",
    );
}
