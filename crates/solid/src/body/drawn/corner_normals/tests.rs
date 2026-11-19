//! What solid · body/drawn/corner_normals.rs is held to, below what a whole
//! body reaches: the seam of a raised post comes out with corners that agree
//! to the bit. The copies of one point a few ulps apart come from a cut
//! across the body, a sketch plane's or a section's, which works a point on
//! an edge out from either end of it; they are built here by hand.

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
