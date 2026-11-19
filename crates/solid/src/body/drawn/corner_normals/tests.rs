//! What solid · body/drawn/corner_normals.rs is held to, below what a whole
//! body reaches: the seam of a raised post comes out with corners that agree
//! to the bit. The copies of one point a few ulps apart come from a cut
//! across the body, a sketch plane's or a section's, which works a point on
//! an edge out from either end of it; they are built here by hand.

use std::f64::consts::{FRAC_PI_2, FRAC_PI_3};

use super::*;

#[test]
fn one_point_of_a_face_handed_out_a_few_ulps_apart_faces_one_way() {
    let point = DVec3::new(3.0, 0.0, 0.0);
    let a_hair_off = point + DVec3::X * 2e-15;
    let (up, out) = (DVec3::new(0.0, 4.0, 1.0), DVec3::new(0.0, 0.0, 4.0));
    let fold = DVec3::new(-2.0, 2.0, 0.0);
    let face = 7;

    let lit = smoothed(vec![
        ([point, up, out], face),
        ([a_hair_off, fold, up], face),
    ]);

    let (here, there) = (lit[0].1[0], lit[1].1[0]);
    assert!(
        here.distance(there) <= 1e-12,
        "one point of a face, {point} and {a_hair_off}, faces {here} from one triangle \
         and {there} from the other",
    );
    let first_facing = (up - point).cross(out - point).normalize();
    assert!(
        here.distance(first_facing) > 1e-3,
        "the point faces {here}, its first triangle's own way: nothing was averaged",
    );
}

#[test]
fn one_point_of_a_face_handed_out_one_ulp_apart_half_way_between_two_multiples_of_eps_faces_one_way()
 {
    let point = DVec3::new(3.0, 0.0, 0.0);
    let an_ulp_below = DVec3::new(f64::from_bits(3.0f64.to_bits() - 1), 0.0, 0.0);
    let (up, out) = (DVec3::new(0.0, 4.0, 1.0), DVec3::new(0.0, 0.0, 4.0));
    let far = DVec3::new(-1024.0, 2.0, 0.0);
    let face = 7;
    let eps = Scale::of(1024.0).eps();
    assert!(
        ((point.x / eps).fract() - 0.5).abs() < 1e-6,
        "3 mm was meant to sit half-way between two multiples of eps at a reach of 1024",
    );

    let lit = smoothed(vec![
        ([point, up, out], face),
        ([an_ulp_below, far, up], face),
    ]);

    let (here, there) = (lit[0].1[0], lit[1].1[0]);
    assert!(
        here.distance(there) <= 1e-12,
        "one point of a face, {point} and an ulp below it, faces {here} from one \
         triangle and {there} from the other",
    );
}

#[test]
fn a_corner_faces_the_average_of_its_triangles_weighed_by_the_angle_each_opens_there() {
    let corner = DVec3::new(5.0, 3.0, 2.0);
    let (along_x, along_y) = (corner + DVec3::X * 4.0, corner + DVec3::Y * 4.0);
    let sixty_degrees_up = corner + DVec3::new(0.0, 0.5, 3f64.sqrt() / 2.0) * 4.0;
    let face = 7;

    let lit = smoothed(vec![
        ([corner, along_x, along_y], face),
        ([corner, along_y, sixty_degrees_up], face),
    ]);

    let weighed = (DVec3::Z * FRAC_PI_2 + DVec3::X * FRAC_PI_3).normalize();
    for (corners, normals) in &lit {
        assert!(
            normals[0].distance(weighed) <= 1e-12,
            "the corner {} of {corners:?} faces {}, where a right angle facing up and \
             sixty degrees facing along x weigh to {weighed}",
            corners[0],
            normals[0],
        );
    }
}
