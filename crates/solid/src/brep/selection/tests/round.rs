//! Which side of each other a sphere and a cylinder whose axis runs through
//! its centre stand on, read off the rims their pair crosses along (#956):
//! in the meridian half-plane of the rims' axis, whatever axis the sphere's
//! chart was laid about. A sphere read in its own chart has no meridian the
//! rims of another axis lie on, and the stretch it reads at could stand on
//! the other side of them.

use glam::DVec3;

use super::*;
use crate::brep::relation::{Relation, relation};
use crate::brep::surface::{Cylinder, Sphere};

const REACH: f64 = 10.0;

/// Far below the tolerance and far above rounding at this reach.
const ROUNDING: f64 = 1e-12;

fn above(first: &Surface, second: &Surface, place: DVec3) -> Option<bool> {
    let scale = Scale::of(REACH);
    let related = relation(first, second, scale);
    wrapped::stands_above(first, second, &related, place, scale, ROUNDING)
}

/// The sphere of radius 5 about the origin, its chart laid about `axis`.
fn sphere_about(axis: DVec3) -> Surface {
    Surface::Sphere(Sphere::through(DVec3::ZERO, axis, 0.0, 5.0))
}

/// The point of the sphere of radius 5 about the origin at height `z` and
/// at `azimuth` radians round z.
fn on_the_sphere(z: f64, azimuth: f64) -> DVec3 {
    let (sin, cos) = azimuth.sin_cos();
    DVec3::new(cos, sin, 0.0) * (25.0 - z * z).sqrt() + DVec3::Z * z
}

/// The sphere of radius 5 about the origin, its chart laid about z, about x
/// and about a slant, against the cylinder along z through its centre of
/// radius 3, crossing it along the circles z = ±4, and of radius a hair
/// under 5, crossing it along two circles some seven ten-thousandths either
/// side of its equator about z. Between the rims the sphere stands outside
/// the cylinder, its wall inside the sphere; beyond them, the other way
/// round — read at places all round z, either way the pair comes, as the
/// sphere about z reads them.
#[test]
fn a_sphere_and_a_cylinder_through_its_centre_stand_on_the_sides_their_rims_part_whatever_axis_the_sphere_s_chart_was_laid_about()
 {
    let eps = Scale::of(REACH).eps();
    for (radius, between, beyond) in [
        (3.0, [-3.0, 0.0, 3.5], [-4.5, 4.2, 4.8]),
        (5.0 - 5.0 * eps, [-3e-4, 0.0, 3e-4], [-0.5, 2.0, 4.5]),
    ] {
        let wall = Surface::Cylinder(Cylinder::about(DVec3::ZERO, DVec3::Z, radius));
        for axis in [DVec3::Z, DVec3::X, DVec3::new(2.0, -1.0, 3.0).normalize()] {
            let ball = sphere_about(axis);
            assert!(
                matches!(relation(&ball, &wall, Scale::of(REACH)), Relation::Rims(ref rims) if rims.len() == 2),
                "the sphere about {axis} and the cylinder of {radius} cross along two rims"
            );
            for (heights, inside) in [(&between[..], true), (&beyond[..], false)] {
                for z in heights {
                    for azimuth in [0.0, 1.0, 2.5, 4.0] {
                        let place = on_the_sphere(*z, azimuth);
                        let what = format!(
                            "the sphere about {axis} and the cylinder of {radius}, at {place}"
                        );
                        assert_eq!(
                            above(&wall, &ball, place),
                            Some(inside),
                            "{what}: the sphere outside the wall between the rims"
                        );
                        assert_eq!(
                            above(&ball, &wall, place),
                            Some(!inside),
                            "{what}: the wall outside the sphere beyond the rims"
                        );
                    }
                }
            }
        }
    }
}
