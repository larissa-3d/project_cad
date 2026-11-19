//! Which way the body faces at each corner it is drawn with, which is what a
//! curved face is lit smooth by (#343): one way at a point of a face, however
//! many of its triangles meet there and however many ulps apart the kernel
//! hands that point out, and one way per face where faces meet.

use super::*;

const SAME_POINT: f64 = 1e-6;
const SAME_WAY: f64 = 1e-9;

type Lit = ([DVec3; 3], [DVec3; 3]);

fn lit(body: &Body) -> Vec<Lit> {
    let lit = body.triangles_with_normals();
    let drawn: Vec<[DVec3; 3]> = lit.iter().map(|(corners, _)| *corners).collect();
    assert_eq!(
        drawn,
        body.triangles(),
        "the triangles lit are not the triangles drawn"
    );
    lit
}

fn lies_level(corners: &[DVec3; 3]) -> bool {
    corners
        .iter()
        .all(|corner| (corner.z - corners[0].z).abs() <= SAME_POINT)
}

fn corners_of_the_wall(lit: &[Lit]) -> Vec<(DVec3, DVec3)> {
    lit.iter()
        .filter(|(corners, _)| !lies_level(corners))
        .flat_map(|(corners, normals)| corners.iter().copied().zip(normals.iter().copied()))
        .collect()
}

fn holds_a_smooth_round_wall(body: &Body, facets_apart: f64) {
    let wall = corners_of_the_wall(&lit(body));
    assert!(!wall.is_empty(), "the wall is drawn");

    for (index, (point, normal)) in wall.iter().enumerate() {
        let outwards = (*point * DVec3::new(1.0, 1.0, 0.0)).normalize();
        assert!(
            normal.angle_between(outwards) < facets_apart,
            "the wall at {point} faces {normal}, not out of the cylinder, {outwards}",
        );
        for (other, other_normal) in &wall[index + 1..] {
            if point.distance(*other) <= SAME_POINT {
                assert!(
                    normal.distance(*other_normal) <= SAME_WAY,
                    "the wall faces {normal} and {other_normal} at {point}: two of its \
                     triangles meeting there are lit apart",
                );
            }
        }
    }
}

fn holds_its_rim_apart(body: &Body, top: f64) {
    let lit = lit(body);
    let (mut on_the_top, mut on_the_wall) = (0, 0);
    for (corners, normals) in &lit {
        let level = lies_level(corners);
        for (corner, normal) in corners.iter().zip(normals) {
            if (corner.z - top).abs() > SAME_POINT {
                continue;
            }
            if level {
                on_the_top += 1;
                assert!(
                    normal.distance(DVec3::Z) <= SAME_WAY,
                    "the top faces {normal} at its rim, {corner}",
                );
            } else {
                on_the_wall += 1;
                assert!(
                    normal.z.abs() <= SAME_WAY,
                    "the wall faces {normal} at the rim, {corner}: it leans into the top",
                );
            }
        }
    }
    assert!(
        on_the_top > 0 && on_the_wall > 0,
        "the rim is drawn on the top {on_the_top} times and on the wall {on_the_wall}",
    );
}

fn an_exact_post() -> Body {
    let post = raised(
        &Body::default(),
        &exact(disc(DVec2::ZERO, 20.0), vec![]),
        10.0,
    );
    assert!(post.is_exact(), "the exact kernel declined the post");
    post
}

#[test]
fn a_point_of_a_curved_wall_faces_one_way_from_every_triangle_meeting_there() {
    holds_a_smooth_round_wall(&an_exact_post(), 2f64.to_radians());
}

#[test]
fn where_a_curved_wall_meets_a_flat_top_each_faces_its_own_way() {
    holds_its_rim_apart(&an_exact_post(), 10.0);
}

#[test]
fn every_corner_of_a_flat_face_faces_the_way_the_face_does() {
    let block = block(DVec2::ZERO, DVec2::new(30.0, 20.0), 10.0);

    for (corners, normals) in lit(&block) {
        let [a, b, c] = corners;
        let facing = (b - a).cross(c - a).normalize();
        for (corner, normal) in corners.iter().zip(normals) {
            assert!(
                normal.distance(facing) <= SAME_WAY,
                "a face facing {facing} faces {normal} at its corner {corner}",
            );
        }
    }
}

#[test]
fn the_flats_light_a_round_wall_smooth_and_keep_its_rim_apart() {
    let (points, curves, triangles) = sampled_disc(DVec2::ZERO, 20.0);
    let profile = Profile {
        exact: None,
        sampled: Loop {
            points: &points,
            curves: &curves,
        },
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let post = Body::default()
        .tool_raised(&profile, FLAT, DVec3::Z * 10.0)
        .expect("the flats never decline");
    assert!(
        !post.is_exact(),
        "the post was meant to be raised by the flats"
    );

    holds_a_smooth_round_wall(&post, 5f64.to_radians());
    holds_its_rim_apart(&post, 10.0);
}
