//! How far off a cutting plane a place of the body still lies on it: what
//! `Body::on_a_plane` answers, held to what it is for rather than to how it
//! is reckoned.

use super::*;

/// A box over the ground, from `low` to `high` and `height` up, raised by the
/// exact kernel.
fn exact_box(low: DVec2, high: DVec2, height: f64) -> Body {
    let body = block(low, high, height);
    assert!(body.is_exact(), "the exact kernel declined the box");
    body
}

type Raise = fn(DVec2, DVec2, f64) -> Body;

/// The same box raised by the flats, its outline handed over without the
/// exact contour.
fn flats_box(low: DVec2, high: DVec2, height: f64) -> Body {
    let corners = [
        low,
        DVec2::new(high.x, low.y),
        high,
        DVec2::new(low.x, high.y),
    ];
    let triangles = [
        [corners[0], corners[1], corners[2]],
        [corners[0], corners[2], corners[3]],
    ];
    let profile = Profile {
        exact: None,
        sampled: Loop::straight(&corners),
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let body = Body::default()
        .tool_raised(&profile, FLAT, DVec3::Z * height)
        .expect("the flats never decline");
    assert!(!body.is_exact(), "the box was raised exact");
    body
}

/// A box standing off the origin towards -X and -Y, its farthest coordinate
/// the near corner's x, `size` from the origin, with every other coordinate
/// nearer.
fn off_the_origin(raise: Raise, size: f64) -> Body {
    raise(
        DVec2::new(-size, -0.6 * size),
        DVec2::new(-0.5 * size, -0.3 * size),
        0.4 * size,
    )
}

const KERNELS: [(&str, Raise); 2] = [("exact", exact_box), ("flats", flats_box)];

/// Where planes are asked about: through the origin, near it, through the
/// body, and far off it either way.
const OFFSETS: [f64; 6] = [0.0, 0.5, 40.0, -40.0, 500.0, -1e4];

#[test]
fn an_exact_body_lies_on_any_plane_by_its_own_size_twice_as_far_off_it_when_twice_as_large() {
    let small = off_the_origin(exact_box, 100.0);
    let large = off_the_origin(exact_box, 200.0);
    let tolerance = small.on_a_plane(0.0);

    for offset in OFFSETS {
        assert_eq!(
            small.on_a_plane(offset),
            tolerance,
            "the exact box 100 out lies on a plane {offset} from the origin otherwise than on \
             one through it",
        );
        let twice = large.on_a_plane(offset);
        assert!(
            tolerance > 0.0 && (twice / tolerance - 2.0).abs() < 1e-12,
            "a plane {offset} from the origin: {tolerance} off it for the exact box 100 out, \
             {twice} for the one 200 out",
        );
    }
}

#[test]
fn flats_lie_on_a_plane_by_how_far_the_plane_stands_from_the_origin_whatever_their_size() {
    let small = off_the_origin(flats_box, 100.0);
    let large = off_the_origin(flats_box, 200.0);

    for offset in OFFSETS {
        assert_eq!(
            large.on_a_plane(offset),
            small.on_a_plane(offset),
            "a plane {offset} from the origin: flats 200 out lie on it otherwise than flats 100 \
             out",
        );
        assert_eq!(
            small.on_a_plane(-offset),
            small.on_a_plane(offset),
            "flats lie on a plane {offset} from the origin otherwise than on one as far the \
             other way",
        );
    }
    for far in [40.0, 500.0, 1e4] {
        let (once, twice) = (small.on_a_plane(far), small.on_a_plane(2.0 * far));
        assert!(
            once > 0.0 && (twice / once - 2.0).abs() < 1e-12,
            "flats: {once} off a plane {far} from the origin, {twice} off one twice as far",
        );
    }
    let floor = small.on_a_plane(1.0);
    for near in [0.0, 0.5] {
        assert_eq!(
            small.on_a_plane(near),
            floor,
            "flats lie on a plane {near} from the origin otherwise than on one a unit from it: \
             no nearer than that is a place told apart",
        );
    }
}

#[test]
fn what_lies_on_a_plane_through_a_body_is_far_below_its_size_and_far_above_the_rounding_there() {
    for (kernel, raise) in KERNELS {
        for size in [1.0, 100.0, 5000.0] {
            let body = off_the_origin(raise, size);
            let through_its_top = 0.4 * size;
            let tolerance = body.on_a_plane(through_its_top);

            assert!(
                tolerance < 1e-6 * size,
                "{kernel}, a box {size} out: {tolerance} off a plane still on it is no hair",
            );
            assert!(
                tolerance > 1e3 * f64::EPSILON * size,
                "{kernel}, a box {size} out: {tolerance} off a plane is within the rounding of \
                 the coordinates there",
            );
        }
    }
}

/// How far along `normal`, from the origin, the body reaches.
fn reach_along(body: &Body, normal: DVec3) -> f64 {
    let (low, high) = body.bounds().expect("something is left");
    low.dot(normal).max(high.dot(normal))
}

#[test]
fn a_face_standing_that_far_in_front_of_a_cutting_plane_is_kept_behind_it() {
    let faces = [
        (
            "the top of a box 100 out",
            DVec2::new(-100.0, -60.0),
            DVec2::new(-50.0, -30.0),
            40.0,
            DVec3::Z,
        ),
        (
            "the side towards +X of a box 100 out",
            DVec2::new(-100.0, -60.0),
            DVec2::new(-50.0, -30.0),
            40.0,
            DVec3::X,
        ),
        (
            "the top of a box half a unit about the origin",
            DVec2::splat(-0.5),
            DVec2::splat(0.25),
            0.5,
            DVec3::Z,
        ),
    ];
    for (kernel, raise) in KERNELS {
        for (face, low, high, height, normal) in faces {
            let body = raise(low, high, height);
            let at = reach_along(&body, normal);
            let plane = at - 0.9 * body.on_a_plane(at);
            let tolerance = body.on_a_plane(plane);
            assert!(
                at - plane <= tolerance,
                "{kernel}, {face}: the fixture's face stands {} in front of the plane, beyond \
                 the {tolerance} the body says still lies on it",
                at - plane,
            );

            let grazed = body.behind(normal, plane);
            assert!(
                reach_along(&grazed, normal) >= at - tolerance,
                "{kernel}, {face}: a plane {} behind it took it back to {}",
                at - plane,
                reach_along(&grazed, normal),
            );
            assert!(
                (grazed.volume() - body.volume()).abs() < 1e-9 * body.volume(),
                "{kernel}, {face}: a plane {} behind it took matter away: {} of {}",
                at - plane,
                grazed.volume(),
                body.volume(),
            );

            let cut = body.behind(normal, at - 1e3 * tolerance);
            assert!(
                reach_along(&cut, normal) <= at - 5e2 * tolerance,
                "{kernel}, {face}: a plane a thousand times as far behind it left it at {}",
                reach_along(&cut, normal),
            );
        }
    }
}
