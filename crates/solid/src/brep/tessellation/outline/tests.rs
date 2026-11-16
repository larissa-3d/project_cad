//! A face laid out in its surface's parameters, read without a body: loops
//! given as raw parameters, as a surface reads its samples.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::Outline;
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

/// How many triangles the fallback for a sphere face bounded by a
/// cylinder's curve cuts the sphere face of `body` into, none where it
/// refuses the face.
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
/// a pole and notched, and one with a cap cut about its axis a twenty-fifth
/// of the radius off a pole and notched, the cap's circle one of its axis.
/// It refuses a notch whose curve passes nearer a pole, a ball cut at a slant
/// whose circle passes nearer a pole and notched, and a face with no
/// cylinder's curve at all.
#[test]
fn the_fallback_lays_out_a_ball_bored_off_its_axis_unless_a_curve_comes_within_a_twentieth_of_its_radius_of_a_pole()
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
        ("a slant alone", cut(&slab(tilted, 4.0))),
    ];
    for (what, body) in &refused {
        assert_eq!(by_the_fallback(body), None, "{what}");
    }
}
