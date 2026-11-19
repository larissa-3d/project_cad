//! What solid · body/drawn/corner_normals.rs is held to, below what a real
//! body reaches: the seam of a raised post comes out with corners that agree
//! to the bit, so a point handed out a few ulps apart is built here by hand.

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
