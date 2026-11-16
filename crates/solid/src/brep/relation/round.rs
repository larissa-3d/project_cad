//! How a sphere or a torus meets another surface (#576). A surface of its
//! axis — a plane square to it, a cylinder, a cone, a sphere or a torus
//! about it — is decided as its meridian circle against a line or a circle
//! of the meridian half-plane (`revolved/arc.rs`), read in the round's own
//! frame, or in the first of two rounds' by an order read off their bits; a
//! plane holding its axis as the meridian circles it cuts, with the poles
//! they cross at. Any other plane cuts a sphere along the circle about the
//! foot of its centre (#805), a circle of another axis than the sphere's,
//! or touches it at that foot alone within the tolerance (#939), and a
//! cylinder whose axis does not hold its centre pierces it along one or two
//! loops (#805, `meet/pierced.rs`). Anything else — such a plane against a
//! torus, a cone or a round of another axis, a cylinder of another axis
//! against a torus or through a sphere's centre, an elliptic wall — is
//! unsupported, which declines a boolean unless the two faces stand clear
//! of each other; none of it is a conic's, so `at_a_slant` answers false
//! and `clear` may pass the faces by.
//!
//! How far a coaxial plane, cylinder or cone stands from touching a round,
//! or from passing through a rim on it, is read here too, in the same
//! half-plane, for decision 12 to move it a hair off onto it (#626,
//! `canonical/tangent.rs`).

use glam::{DVec2, DVec3};

use super::revolved::{Frame, Meridian, Met, in_the_box, met_in_the_half_plane, met_round};
use super::{Relation, parallel, square};
use crate::brep::curve::{Circle, Wall};
use crate::brep::meet::{Configuration, Meeting};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder, Plane, Sphere, Surface, Torus};

/// A sphere or a torus as the frame of its axis and its meridian circle,
/// about `(h_c, R)` and of radius `r`; none for any other surface.
fn round(surface: &Surface) -> Option<(Frame, DVec2, f64)> {
    match surface {
        Surface::Sphere(sphere) => Some((
            Frame::of_sphere(sphere),
            DVec2::new(sphere.center, 0.0),
            sphere.radius,
        )),
        Surface::Torus(torus) => Some((Frame::of_torus(torus), torus.center, torus.radius)),
        Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => None,
    }
}

pub(super) fn relation(one: &Surface, other: &Surface, scale: Scale) -> Relation {
    match (round(one), round(other)) {
        (Some(first), Some(second)) => {
            if goes_first(one, other) {
                against(one, first, other, scale)
            } else {
                against(other, second, one, scale)
            }
        }
        (Some(first), None) => against(one, first, other, scale),
        (None, Some(second)) => against(other, second, one, scale),
        (None, None) => Relation::Unsupported,
    }
}

/// How far a plane square to a round's axis, a cylinder or a cone about it,
/// stands from touching the round, or from passing through a rim on it
/// (#626), read in the round's meridian half-plane: the line moves `gap`
/// along `way` to get there, through `at`, the circle of the round it
/// touches it along once there, or the rim. `origin` and `axis` are the
/// round's.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::brep) struct Gap {
    pub origin: DVec3,
    pub axis: DVec3,
    pub way: DVec2,
    pub gap: f64,
    pub at: Circle,
}

/// How `other` stands from touching `surface`, a sphere or a torus, at the
/// foot of the meridian circle's centre: none where `surface` is neither,
/// `other` no plane square to its axis nor a cylinder or a cone about it,
/// or where their touch would stand on the mirror lobe, at `ρ < 0`.
pub(in crate::brep) fn untouched(surface: &Surface, other: &Surface, scale: Scale) -> Option<Gap> {
    let (frame, center, radius) = round(surface)?;
    let line = line_of(&frame, other, scale)?;
    let Met::Near { foot, gap, .. } = met_in_the_half_plane(center, radius, &line, scale.eps())
    else {
        return None;
    };
    let towards = center - foot;
    (foot.y >= -scale.eps() && towards.length() > 0.0).then(|| {
        let way = towards.normalize();
        let touch = center - way * radius;
        Gap {
            origin: frame.origin,
            axis: frame.axis,
            way,
            gap,
            at: frame.circle(touch.x, touch.y.max(0.0)),
        }
    })
}

/// How `other` stands from passing through `rim`, a circle about the axis
/// of `surface`, a sphere or a torus: none as for `untouched`, or where the
/// rim stands about another axis.
pub(in crate::brep) fn off_the_rim(
    surface: &Surface,
    rim: &Circle,
    other: &Surface,
    scale: Scale,
) -> Option<Gap> {
    let (frame, _, _) = round(surface)?;
    if !coaxial(&frame, rim.center, rim.axis, scale) {
        return None;
    }
    let at = DVec2::new((rim.center - frame.origin).dot(frame.axis), rim.radius);
    let (normal, offset) = line_of(&frame, other, scale)?.line();
    let away = normal.dot(at) - offset;
    Some(Gap {
        origin: frame.origin,
        axis: frame.axis,
        way: normal * away.signum(),
        gap: away.abs(),
        at: *rim,
    })
}

/// `other` as a line of the meridian half-plane of `frame`'s axis: a plane
/// square to it, a cylinder or a cone about it; none for anything else.
fn line_of(frame: &Frame, other: &Surface, scale: Scale) -> Option<Meridian> {
    match other {
        Surface::Plane(plane) if parallel(plane.normal, frame.axis, scale) => {
            Some(Meridian::Level(frame.level(plane)))
        }
        Surface::Cylinder(Cylinder { origin, axis, .. })
        | Surface::Cone(Cone { origin, axis, .. })
            if coaxial(frame, *origin, *axis, scale) =>
        {
            Meridian::of_surface(frame, other)
        }
        Surface::Plane(_)
        | Surface::Cylinder(_)
        | Surface::Cone(_)
        | Surface::Sphere(_)
        | Surface::Torus(_)
        | Surface::Elliptic(_) => None,
    }
}

/// Of two rounds, whether the first is the one whose frame the pair is read
/// in: an order read off the surfaces alone, so that the answer is the same
/// whichever came first.
fn goes_first(one: &Surface, other: &Surface) -> bool {
    let bits = |surface: &Surface| -> Vec<f64> {
        match surface {
            Surface::Sphere(sphere) => [sphere.origin, sphere.axis]
                .into_iter()
                .flat_map(|vector| vector.to_array())
                .chain([0.0, sphere.center, 0.0, sphere.radius])
                .collect(),
            Surface::Torus(torus) => [torus.origin, torus.axis]
                .into_iter()
                .flat_map(|vector| vector.to_array())
                .chain([1.0, torus.center.x, torus.center.y, torus.radius])
                .collect(),
            Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
                Vec::new()
            }
        }
    };
    bits(one)
        .into_iter()
        .zip(bits(other))
        .map(|(mine, theirs)| mine.total_cmp(&theirs))
        .find(|order| order.is_ne())
        .is_none_or(|order| order.is_lt())
}

/// The round `surface` against `other`: a plane square to its axis or
/// holding it, or a surface of revolution about its axis, each read in the
/// round's frame; any other plane, or a cylinder of another axis, against a
/// sphere.
fn against(
    surface: &Surface,
    (frame, center, radius): (Frame, DVec2, f64),
    other: &Surface,
    scale: Scale,
) -> Relation {
    let (origin, axis) = match other {
        Surface::Plane(plane) => {
            return with_plane(surface, (&frame, center, radius), plane, scale);
        }
        Surface::Cylinder(Cylinder { origin, axis, .. })
        | Surface::Cone(Cone { origin, axis, .. })
        | Surface::Sphere(Sphere { origin, axis, .. })
        | Surface::Torus(Torus { origin, axis, .. }) => (*origin, *axis),
        Surface::Elliptic(_) => return Relation::Unsupported,
    };
    if !coaxial(&frame, origin, axis, scale) {
        return match (surface, other) {
            (Surface::Sphere(sphere), Surface::Cylinder(cylinder)) => {
                pierced(sphere, cylinder, scale)
            }
            _ => Relation::Unsupported,
        };
    }
    match Meridian::of_surface(&frame, other) {
        Some(meridian) => met_round(&frame, center, radius, meridian, scale),
        None => Relation::Unsupported,
    }
}

/// Whether the line through `origin` along `axis` is the frame's axis within
/// the tolerance: parallel across the box, and the two a hair apart at most,
/// as a cone reads it. A surface further off is unsupported; decision 8
/// moved one within its hair onto the axis before.
fn coaxial(frame: &Frame, origin: DVec3, axis: DVec3, scale: Scale) -> bool {
    let between = origin - frame.origin;
    parallel(axis, frame.axis, scale)
        && (between - frame.axis * frame.axis.dot(between)).length() <= scale.eps()
}

/// Square to the axis, the circles at the plane's height, from the formula a
/// cylinder of the axis reads it with; holding it, the meridian circles it
/// cuts; any other plane, the section of a sphere (#805) — a torus's at a
/// slant or off its axis is a quartic, unsupported.
fn with_plane(
    surface: &Surface,
    (frame, center, radius): (&Frame, DVec2, f64),
    plane: &Plane,
    scale: Scale,
) -> Relation {
    if parallel(plane.normal, frame.axis, scale) {
        let level = Meridian::Level(frame.level(plane));
        return met_round(frame, center, radius, level, scale);
    }
    if square(plane.normal, frame.axis, scale) && plane.distance(frame.origin).abs() <= scale.eps()
    {
        return meridians(frame, center, radius, plane, scale);
    }
    match surface {
        Surface::Sphere(sphere) => section(sphere, plane, scale),
        Surface::Torus(_)
        | Surface::Plane(_)
        | Surface::Cylinder(_)
        | Surface::Cone(_)
        | Surface::Elliptic(_) => Relation::Unsupported,
    }
}

/// A plane at a slant to the axis of `sphere`, or holding a direction of it
/// off it: the circle about the foot of the centre on the plane, square to
/// the plane, of radius √(r² − d²), `d` the centre's distance from the
/// plane, written as a product; apart where the plane stands further than
/// the tolerance off the sphere. Within the tolerance of touching it, the
/// plane touches it at that foot alone, off its poles (#939), as a plane
/// square to the axis touches it at a pole.
fn section(sphere: &Sphere, plane: &Plane, scale: Scale) -> Relation {
    let (middle, radius) = (sphere.middle(), sphere.radius);
    let standing = plane.distance(middle);
    let foot = middle - plane.normal * standing;
    let away = standing.abs();
    let gap = away - radius;
    if gap > scale.eps() {
        return Relation::Apart;
    }
    if gap >= -scale.eps() {
        return Relation::Apex(foot);
    }
    Relation::Circle(Circle::about(
        foot,
        plane.normal,
        ((radius - away) * (radius + away)).sqrt(),
    ))
}

/// A cylinder of another axis than the axis of `sphere`: the loops where its
/// rulings pierce the sphere, as `meet/pierced.rs` reads them — one where it
/// takes in part of the ball, two where it passes right through it — and
/// apart where it misses the ball or holds it inside, each gap measured
/// where the two surfaces come nearest. Within the tolerance of touching the
/// ball from outside or from inside, of a figure of eight — the cylinder
/// through it touching it from inside along its far side — or of its axis
/// holding the centre: unsupported.
fn pierced(sphere: &Sphere, cylinder: &Cylinder, scale: Scale) -> Relation {
    let eps = scale.eps();
    let between = sphere.middle() - cylinder.origin;
    let off = (between - cylinder.axis * cylinder.axis.dot(between)).length();
    let (wall, ball) = (cylinder.radius, sphere.radius);
    let outside = (off - wall).abs() - ball;
    if outside > eps {
        return Relation::Apart;
    }
    let through = ball - (off + wall);
    if off <= eps || outside >= -eps || through.abs() <= eps {
        return Relation::Unsupported;
    }
    let meeting = Meeting::of(&Wall::Cylinder(*cylinder), &Wall::Sphere(*sphere), scale);
    match meeting.configuration {
        Configuration::OneLoop | Configuration::TwoLoops => Relation::Meet(meeting),
        Configuration::Apart
        | Configuration::Contact
        | Configuration::FigureOfEight
        | Configuration::TwoEllipses => Relation::Unsupported,
    }
}

/// Holding the axis, the meridian circles the plane cuts: one great circle
/// through a sphere's centre, or a torus's two tube circles, one each side
/// of the axis across the plane, each about the meridian circle's centre and
/// of its radius, built as the turn builds the meridian edges of a partial
/// turn; with the poles where they stand within the box.
fn meridians(frame: &Frame, center: DVec2, radius: f64, plane: &Plane, scale: Scale) -> Relation {
    let on_the_axis = frame.point(center.x);
    let circles = if center.y == 0.0 {
        vec![Circle::about(on_the_axis, plane.normal, radius)]
    } else {
        let across = plane.normal.cross(frame.axis).normalize();
        let [one, other] = [-1.0, 1.0].map(|side| {
            Circle::about(
                on_the_axis + across * (side * center.y),
                plane.normal,
                radius,
            )
        });
        if other.center.to_array() < one.center.to_array() {
            vec![other, one]
        } else {
            vec![one, other]
        }
    };
    Relation::Meridians {
        circles,
        poles: poles(frame, center, radius, scale),
    }
}

/// The points of the axis the meridian circle reaches, in the box: a
/// sphere's two poles, a spindle torus's two, a horn's one, a ring's none.
fn poles(frame: &Frame, center: DVec2, radius: f64, scale: Scale) -> Vec<DVec3> {
    let big = center.y.abs();
    if big > radius {
        return Vec::new();
    }
    let half = ((radius - big) * (radius + big)).sqrt();
    let mut heights = vec![center.x - half];
    if half > 0.0 {
        heights.push(center.x + half);
    }
    heights
        .into_iter()
        .map(|h| frame.point(h))
        .filter(|pole| in_the_box(*pole, scale))
        .collect()
}
