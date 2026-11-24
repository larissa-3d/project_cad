//! How far a coaxial plane, cylinder or cone stands from touching a round,
//! or from passing through a rim on it, read in the round's meridian
//! half-plane, for decision 12 to move it a hair off onto it (#626,
//! `canonical/tangent.rs`). A sphere is read about the axis of the surface
//! or of the rim where that axis holds its centre (#956).

use glam::{DVec2, DVec3};

use super::{coaxial, read_about, round};
use crate::brep::curve::Circle;
use crate::brep::relation::parallel;
use crate::brep::relation::revolved::{Frame, Meridian, Met, met_in_the_half_plane};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder, Surface};

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
/// or where their touch would stand on the mirror lobe, at `ρ < 0`. A
/// sphere is read about the axis of a cylinder or a cone holding its centre.
pub(in crate::brep) fn untouched(surface: &Surface, other: &Surface, scale: Scale) -> Option<Gap> {
    let about = match other {
        Surface::Cylinder(Cylinder { origin, axis, .. })
        | Surface::Cone(Cone { origin, axis, .. }) => read_about(surface, *origin, *axis, scale),
        Surface::Plane(_) | Surface::Sphere(_) | Surface::Torus(_) | Surface::Elliptic(_) => None,
    };
    let (frame, center, radius) = about.or_else(|| round(surface))?;
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
/// of `surface`, a sphere or a torus, or any circle on a sphere, read about
/// its axis: none as for `untouched`, or where the rim stands about another
/// axis.
pub(in crate::brep) fn off_the_rim(
    surface: &Surface,
    rim: &Circle,
    other: &Surface,
    scale: Scale,
) -> Option<Gap> {
    let about = read_about(surface, rim.center, rim.axis, scale);
    let (frame, _, _) = about.or_else(|| round(surface))?;
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
