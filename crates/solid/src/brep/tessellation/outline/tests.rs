//! A face laid out in its surface's parameters, read without a body: loops
//! given as raw parameters, as a surface reads its samples.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::Outline;
use super::cylinder::{CLEAR, clear_first};
use super::pierced::pierces;
use super::round::Read;
use crate::brep::curve::{Curve, Wall};
use crate::brep::meet::Meeting;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Sphere, Surface};
use crate::brep::tessellation::sampling::{Rounded, Samples};
use crate::brep::topology::Body;
use crate::brep::turned::tests::round::{ball, turned};
use crate::profile::{Contour, Frame};

fn empty() -> Outline {
    Outline {
        points: Vec::new(),
        samples: Vec::new(),
        segments: Vec::new(),
        at: Default::default(),
        walls: Vec::new(),
        strips: Vec::new(),
    }
}

/// The corners of a polygon in `(θ, h)`, each side cut into steps short
/// enough to unwrap, each angle read back into `(−π, π]` as a surface
/// reads it.
fn sampled(corners: &[[f64; 2]]) -> Vec<DVec2> {
    let steps = 64;
    (0..corners.len())
        .flat_map(|index| {
            let (from, to) = (
                DVec2::from(corners[index]),
                DVec2::from(corners[(index + 1) % corners.len()]),
            );
            (0..steps).map(move |step| from.lerp(to, step as f64 / steps as f64))
        })
        .map(|at| DVec2::new((at.x + PI).rem_euclid(TAU) - PI, at.y))
        .collect()
}

/// The area the triangles cover in the parameters.
fn covered(outline: &Outline, triangles: &[[usize; 3]]) -> f64 {
    triangles
        .iter()
        .map(|&[a, b, c]| {
            let [a, b, c] = [a, b, c].map(|corner| outline.points[corner]);
            (b - a).perp_dot(c - a).abs() / 2.0
        })
        .sum()
}

/// A band sheared round a cylinder over more than a turn, from `θ = −0.81`
/// to `7.09`, without winding: at each height its angles span 1.11, a turn
/// further on 2.77 higher. The hole's raw angles, `[−0.3, −0.1]` at height
/// 2.9, stand in the band only a turn on, `[5.98, 6.18]` (86751972230147).
#[test]
fn a_hole_in_the_far_turn_of_a_face_wider_than_a_turn_is_laid_where_the_face_holds_it() {
    let band = [[-0.81, 0.0], [0.3, 0.0], [7.09, 3.0], [5.98, 3.0]];
    let hole = [[-0.3, 2.85], [-0.3, 2.95], [-0.1, 2.95], [-0.1, 2.85]];
    let raws = vec![sampled(&band), sampled(&hole)];
    let laps: Vec<Vec<usize>> = vec![
        (0..raws[0].len()).collect(),
        (raws[0].len()..raws[0].len() + raws[1].len()).collect(),
    ];
    let mut outline = empty();
    outline
        .round_laid(&raws, &laps, 64, |_| true)
        .expect("a face that does not wind is laid as it is");
    let triangles = outline.cut().expect("the region is swept");
    let area = |corners: &[[f64; 2]]| {
        (0..corners.len())
            .map(|index| {
                DVec2::from(corners[index])
                    .perp_dot(DVec2::from(corners[(index + 1) % corners.len()]))
            })
            .sum::<f64>()
            .abs()
            / 2.0
    };
    let region = area(&band) - area(&hole);
    let swept = covered(&outline, &triangles);
    assert!(
        (swept - region).abs() <= 1e-9 * region,
        "the triangles cover {swept}, the face {region}"
    );
}

/// The ball of radius 5 about the origin, turned about Y.
fn ball_turned_about_y() -> Body {
    turned(&ball(), 360.0)
}

/// A cylinder of `radius` along `axis`, its axis `off` from the origin
/// towards `out`, from 10 before the origin's foot on it to 10 past.
fn bore(axis: DVec3, out: DVec3, radius: f64, off: f64) -> Body {
    let axis = axis.normalize();
    let u = out.reject_from(axis).normalize();
    let frame = Frame {
        origin: u * off - axis * 10.0,
        u,
        v: axis.cross(u),
    };
    Body::raised(
        &Contour::circle(DVec2::ZERO, radius),
        &[],
        frame,
        axis * 20.0,
    )
    .expect("a bore stands on its own")
}

/// A slab 10 thick past the plane `off` from the origin along `normal`.
fn slab(normal: DVec3, off: f64) -> Body {
    let normal = normal.normalize();
    let u = normal.any_orthonormal_vector();
    let frame = Frame {
        origin: normal * off,
        u,
        v: normal.cross(u),
    };
    let side = Contour::rectangle(DVec2::splat(-20.0), DVec2::splat(20.0));
    Body::raised(&side, &[], frame, normal * 10.0).expect("a slab stands on its own")
}

/// How many triangles the fallback for a sphere face no other chart cuts,
/// [`Outline::pierced`], cuts the first sphere face of `body` into, none
/// where it refuses the face.
fn by_the_fallback(body: &Body) -> Option<usize> {
    let tolerance = 1e-3 * body.scale().reach();
    let (id, sphere) = body
        .face_ids()
        .find_map(|id| match body.surface(body.face(id).surface) {
            Surface::Sphere(sphere) => Some((id, *sphere)),
            _ => None,
        })
        .expect("a face on the sphere");
    let mut samples = Samples::of(body, tolerance);
    Outline::pierced(body, &mut samples, id, &sphere, tolerance)
        .map(|(_, triangles)| triangles.len())
}

#[test]
fn the_curve_a_cylinder_meets_a_ball_along_pierces_it_only_on_its_own_sphere_read_in_its_own_chart()
{
    let scale = Scale::of(40.0);
    let eps = scale.eps();
    let own = Sphere::through(DVec3::ZERO, DVec3::Y, 0.0, 5.0);
    let notch = Cylinder::about(DVec3::new(0.0, 0.0, 4.5), DVec3::X, 1.5);
    let [meet, ..] = Meeting::of(&Wall::Cylinder(notch), &Wall::Sphere(own), scale).components[..]
    else {
        panic!("the notch meets the ball");
    };
    let curve = Curve::Meet(meet);
    assert!(pierces(&curve, Read::Tilted, &Rounded::Sphere(own), eps));
    for (read, what) in [(Read::Own, "own"), (Read::About([0, 1]), "about")] {
        assert!(!pierces(&curve, read, &Rounded::Sphere(own), eps), "{what}");
    }
    for other in [
        Sphere::through(DVec3::ZERO, DVec3::Y, 0.0, 4.0),
        Sphere::through(DVec3::X * 12.0, DVec3::Y, 0.0, 5.0),
    ] {
        assert!(
            !pierces(&curve, Read::Tilted, &Rounded::Sphere(other), eps),
            "{other:?}"
        );
    }
    let latitude = Curve::Circle(own.circle(3.0, 4.0));
    assert!(!pierces(
        &latitude,
        Read::Tilted,
        &Rounded::Sphere(own),
        eps
    ));
}

/// The fallback lays out a ball bored or notched off its axis, the bead
/// whose loops wind round the axis, a notch whose curve passes a twentieth
/// of the radius off a pole and a hair more, a ball cut at a slant well off
/// a pole, alone or notched, and one with a cap cut about its axis a
/// twenty-fifth of the radius off a pole and notched, the cap's circle one of
/// its axis; and a ball cut by flats of two axes or three, bounded by their
/// circles alone (#940): x and z ≥ 4, whose sphere holds both poles and two
/// holes, x and z ≥ 2, one loop of two arcs, and x, y and z ≥ 2, one loop of
/// three holding one pole. It refuses a notch whose curve passes nearer a
/// pole, and a ball cut at a slant whose circle passes nearer a pole, alone
/// or notched: between circles and the curve a cylinder meets the sphere
/// along, what it lays out is told by how near a pole they come, not by which
/// of the two they are. A face with any other curve it refuses still.
#[test]
fn the_fallback_lays_out_a_sphere_face_unless_a_curve_comes_within_a_twentieth_of_its_radius_of_a_pole()
 {
    let ball = ball_turned_about_y();
    let notch = bore(DVec3::X, DVec3::Z, 1.5, 4.5);
    let tilted = DVec3::new(0.28, 0.96, 0.0);
    let cut = |by: &Body| ball.cut_by(by).expect("the ball is cut");
    let laid = [
        ("a notch", cut(&notch)),
        ("a bore", cut(&bore(DVec3::X, DVec3::Z, 1.5, 2.5))),
        ("the bead", cut(&bore(DVec3::Y, DVec3::X, 1.0, 0.5))),
        (
            "a notch 0.053 of the radius off a pole",
            cut(&bore(DVec3::X, DVec3::Y, 0.3, 4.85)),
        ),
        (
            "a slant well off a pole",
            cut(&slab(tilted, 4.0)).cut_by(&notch).expect("notched"),
        ),
        (
            "a cap about the axis",
            cut(&slab(DVec3::Y, 4.996)).cut_by(&notch).expect("notched"),
        ),
        ("a slant alone", cut(&slab(tilted, 4.0))),
        (
            "x and z ≥ 4",
            cut(&slab(DVec3::X, 4.0))
                .cut_by(&slab(DVec3::Z, 4.0))
                .expect("cut twice"),
        ),
        (
            "x and z ≥ 2",
            cut(&slab(DVec3::X, 2.0))
                .cut_by(&slab(DVec3::Z, 2.0))
                .expect("cut twice"),
        ),
        (
            "x, y and z ≥ 2",
            cut(&slab(DVec3::X, 2.0))
                .cut_by(&slab(DVec3::Y, 2.0))
                .and_then(|twice| twice.cut_by(&slab(DVec3::Z, 2.0)))
                .expect("cut three times"),
        ),
    ];
    for (what, body) in &laid {
        let triangles = by_the_fallback(body);
        assert!(
            triangles.is_some_and(|count| count > 0),
            "{what}: {triangles:?}"
        );
    }
    let refused = [
        (
            "a notch 0.039 of the radius off a pole",
            cut(&bore(DVec3::X, DVec3::Y, 0.2, 4.95)),
        ),
        (
            "a slant near a pole",
            cut(&slab(tilted, 4.75)).cut_by(&notch).expect("notched"),
        ),
        ("a slant near a pole alone", cut(&slab(tilted, 4.75))),
    ];
    for (what, body) in &refused {
        assert_eq!(by_the_fallback(body), None, "{what}");
    }
}

/// The ranks of the grid's `steps` angles [`clear_first`] tries a face cut
/// open at, read back from the angles it gives.
fn tried(raws: &[Vec<DVec2>], steps: usize) -> Vec<usize> {
    let step = TAU / steps as f64;
    clear_first(raws, steps)
        .into_iter()
        .map(|angle| {
            let rank = (angle / step).round();
            assert!(
                (angle - rank * step).abs() <= 1e-12,
                "{angle} is not an angle of a grid of {steps} steps"
            );
            rank as usize
        })
        .collect()
}

/// The ranks of a grid of `steps` angles, those of `beside` taken out and
/// put last in the order given.
fn clear_then(steps: usize, beside: &[usize]) -> Vec<usize> {
    (0..steps)
        .filter(|rank| !beside.contains(rank))
        .chain(beside.iter().copied())
        .collect()
}

/// A sample at `height`, `share` of a step past the angle `rank` of a grid
/// of `steps` angles a turn.
fn off_the_grid(steps: usize, rank: f64, share: f64, height: f64) -> DVec2 {
    let step = TAU / steps as f64;
    DVec2::new(rank * step + share * step, height)
}

/// Of a grid of 24 steps, a face whose samples stand a twentieth of a step
/// past the angle 3 and nine hundredths short of it, eight hundredths short
/// of 15 and two past 20, over two loops, is tried cut open at every other
/// angle first, by rank — 7 among them, its sample three tenths of a step
/// off — then at 15, 3 and 20: the angle whose nearest sample stands
/// furthest first.
#[test]
fn a_face_is_tried_cut_open_at_every_angle_no_sample_is_beside_first_then_where_the_nearest_stands_furthest()
 {
    let steps = 24;
    let at = |rank, share| off_the_grid(steps, rank, share, 1.0);
    let raws = vec![
        vec![at(3.0, 0.05), at(7.0, -0.3), at(3.0, -0.09)],
        vec![at(20.0, 0.02), at(15.0, -0.08)],
    ];
    assert_eq!(tried(&raws, steps), clear_then(steps, &[15, 3, 20]));
}

/// A sample no further than [`CLEAR`] from an angle of the grid stands on
/// it and leaves it clear: on the angle 5, a tenth of `CLEAR` past 9 and
/// exactly `CLEAR` past 0. Twice `CLEAR` past 12 is beside it.
#[test]
fn a_sample_no_further_than_clear_from_an_angle_stands_on_it_and_leaves_it_clear() {
    let steps = 24;
    let step = TAU / steps as f64;
    let raws = vec![vec![
        DVec2::new(5.0 * step, 0.0),
        DVec2::new(9.0 * step + CLEAR / 10.0, 0.0),
        DVec2::new(CLEAR, 0.0),
        DVec2::new(12.0 * step + 2.0 * CLEAR, 0.0),
    ]];
    assert_eq!(tried(&raws, steps), clear_then(steps, &[12]));
}

/// A sample a tenth of a step from an angle of the grid, or eleven
/// hundredths, leaves it clear; nine hundredths is beside it — on a grid of
/// 8, 24 or 32 steps a turn.
#[test]
fn a_sample_is_beside_an_angle_of_the_grid_under_a_tenth_of_a_step_from_it() {
    for steps in [8, 24, 32] {
        let step = TAU / steps as f64;
        let raws = vec![vec![
            DVec2::new(0.1 * step, 0.0),
            off_the_grid(steps, 4.0, 0.11, 0.0),
            off_the_grid(steps, 6.0, -0.09, 0.0),
        ]];
        assert_eq!(
            tried(&raws, steps),
            clear_then(steps, &[6]),
            "{steps} steps"
        );
    }
}

/// A sample read a turn away from an angle of the grid is beside it: of a
/// grid of 24 steps, four hundredths of a step short of a whole turn is
/// beside the angle 0, six past half a turn back beside 12, and two short
/// of three steps back beside 21.
#[test]
fn a_sample_read_a_turn_away_from_an_angle_of_the_grid_is_beside_it() {
    let steps = 24;
    let at = |rank, share| off_the_grid(steps, rank, share, 2.0);
    let raws = vec![vec![at(24.0, -0.04), at(-12.0, 0.06), at(-3.0, -0.02)]];
    assert_eq!(tried(&raws, steps), clear_then(steps, &[12, 0, 21]));
}

/// Two angles whose nearest samples stand as near are tried by rank, the
/// lowest first, whichever sample comes first: of a grid of 24 steps, a
/// sixty-fourth of a radian past the angle 17 and short of 5 — a power of
/// two, so that both stand exactly as far — after eight hundredths of a
/// step past 2.
#[test]
fn two_angles_whose_nearest_samples_stand_as_near_are_tried_lowest_rank_first() {
    let steps = 24;
    let step = TAU / steps as f64;
    let hair = 1.0 / 64.0;
    let raws = vec![vec![
        DVec2::new(17.0 * step + hair, 0.0),
        DVec2::new(5.0 * step - hair, 0.0),
        off_the_grid(steps, 2.0, 0.08, 0.0),
    ]];
    assert_eq!(tried(&raws, steps), clear_then(steps, &[2, 5, 17]));
}
