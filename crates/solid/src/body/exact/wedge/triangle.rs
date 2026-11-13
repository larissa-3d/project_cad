//! The triangle three chamfers at a convex corner are closed with (#699):
//! its corners where the set-backs of each two of the corner's edges cross on
//! the face the two share, the corner cut off flat through them, across the
//! point the three flats would otherwise meet at. Two corners at the ends of
//! an edge whose flat has no width between them — the edge across a tongue's
//! tip as wide as two distances (#837) — meet on a ridge in its place: the
//! later corner's cut takes that flat whole. At a concave corner, a
//! pocket's bottom (#801), the three fills leave a hollow behind that point,
//! which the same prism fills, joined, cut back into the corner's walls.
//! Four or more chamfers meeting outwards (#804), a pyramid's apex, are closed
//! by the polygon through the crossings where those lie on one plane: the
//! tip past it cut off by the same prism before any flat is laid, so that
//! each flat, carried past the corner, meets air there, and the face the cut
//! leaves, trimmed by the flats to that polygon, numbered after them.

use glam::{DVec2, DVec3};

use super::Prism;
use super::closing::Corner;
use super::overlap::CLEARED;
use super::sphere::meeting_at;
use crate::body::exact::{Exact, PIECES};
use crate::brep::{Declined, FaceId};
use crate::profile::{Contour, Frame};

/// How much wider than the triangle the prism that cuts the corner off
/// stands, about the triangle's middle: the point the three flats meet at
/// stands over the triangle at a square corner, and off its middle at one
/// that is not.
const SPREAD: f64 = 3.0;

/// How far past the corner that prism reaches, as a share of how high the
/// corner stands over the triangle.
const PAST: f64 = 0.125;

/// The body with the corner cut off flat through the crossings by the prism
/// [`across`] raises, the prism's floor the face it lays, a flat of `flats`
/// it takes whole gone where its edge runs to another corner the step closes.
/// Declined `Unsupported` where the cut left anything but that face, every
/// corner of it a corner of one of the three `flats`, as
/// [`Exact::cut_along`] reads it.
pub(in crate::body::exact) fn cut_across(
    body: &Exact,
    corner: &Corner,
    flats: &[u32],
) -> Result<Exact, Declined> {
    let (outline, frame, travel) = across(&corner.crossings, corner.at);
    let tool = Exact::raised(&outline, &[], frame, travel)?;
    let cut = body.cut_by(&Exact { next: 1, ..tool })?;
    let taken: Vec<u32> = (0..flats.len())
        .filter(|&at| corner.between[at] && !cut.has_face(flats[at] as usize))
        .map(|at| flats[at])
        .collect();
    if cut.cut_along(&[], body, 1, flats, &[], &taken) {
        Ok(cut)
    } else {
        Err(Declined::Unsupported)
    }
}

/// The body with a concave corner closed flat through the crossings: the
/// hollow the three fills leave behind them filled by the prism [`across`]
/// raises, cut back as [`Corner::sunk`] keeps it inside the corner's walls,
/// joined, its floor the face it lays. Declined as [`cut_across`] declines —
/// a face of the prism left standing, past a wall thinner than the prism is
/// sunk into it.
pub(in crate::body::exact) fn joined_across(
    body: &Exact,
    corner: &Corner,
    flats: &[u32],
) -> Result<Exact, Declined> {
    let (outline, frame, travel) = across(&corner.crossings, corner.at);
    let reach = (outline.corners.iter())
        .map(|at| at.length())
        .fold(0.0, f64::max)
        + frame.origin.distance(corner.at)
        + travel.length();
    let tool = corner.sunk(Exact::raised(&outline, &[], frame, travel)?, reach)?;
    let joined = body.joined(&Exact { next: 1, ..tool })?;
    if joined.cut_along(&[], body, 1, flats, &[], &[]) {
        Ok(joined)
    } else {
        Err(Declined::Unsupported)
    }
}

/// The tip of a corner four or more chamfers meet at, cut from the body before
/// any of them is laid: the prism [`across`] raises over the crossings, its
/// floor the one face it leaves. Declined `Unsupported` where the crossings
/// lie on no one plane, within the body's tolerance.
pub(in crate::body::exact) fn tip(
    body: &Exact,
    corner: &Corner,
) -> Result<(Exact, usize), Declined> {
    let eps = body.brep.scale().eps();
    let (outline, frame, travel) = across(&corner.crossings, corner.at);
    let normal = frame.u.cross(frame.v);
    if (corner.crossings.iter()).any(|&at| normal.dot(at - frame.origin).abs() > eps) {
        return Err(Declined::Unsupported);
    }
    Ok((Exact::raised(&outline, &[], frame, travel)?, 1))
}

/// The body with the corner [`tip`] cleared closed once its flats are laid:
/// the face the tip left, trimmed by them to the polygon through the
/// crossings, numbered where the steps' numbers end. Declined `Unsupported`
/// where no one such face meets the first crossing, or it is anything but
/// one face, every corner of it a corner of one of the `flats`, as
/// [`Exact::cut_along`] reads it.
pub(in crate::body::exact) fn polygon_across(
    body: &Exact,
    corner: &Corner,
    flats: &[u32],
) -> Result<Exact, Declined> {
    let eps = body.brep.scale().eps();
    let cleared = |face: &FaceId| {
        (body.brep.numbers(*face).iter()).any(|number| (CLEARED..PIECES).contains(number))
    };
    let left: Vec<FaceId> = (meeting_at(&body.brep, corner.crossings[0], eps).into_iter())
        .filter(cleared)
        .collect();
    let [face] = left[..] else {
        return Err(Declined::Unsupported);
    };
    let mut brep = body.brep.clone();
    for &number in body.brep.numbers(face) {
        brep.rename(&[face], number, body.next);
    }
    let closed = body.patched(brep, body.next + 1)?;
    if closed.cut_along(&[], body, 1, flats, &[], &[]) {
        Ok(closed)
    } else {
        Err(Declined::Unsupported)
    }
}

/// The prism a corner is cut off flat with, or filled flat with, through
/// `crossings`: raised from the plane of the first three toward `corner`,
/// over the polygon they make spread [`SPREAD`] times about its middle, to
/// [`PAST`] past the corner. Its floor lies on that plane; what stands on the
/// corner's side of it near the corner is the point the flats meet at.
fn across(crossings: &[DVec3], corner: DVec3) -> Prism {
    let (first, second, third) = (crossings[0], crossings[1], crossings[2]);
    let middle = crossings.iter().sum::<DVec3>() / crossings.len() as f64;
    let u = (second - first).normalize();
    let normal = u.cross(third - first).normalize();
    let v = normal.cross(u);
    let outline = (crossings.iter())
        .map(|&at| {
            let off = (at - middle) * SPREAD;
            DVec2::new(off.dot(u), off.dot(v))
        })
        .collect();
    let frame = Frame {
        origin: middle,
        u,
        v,
    };
    let travel = normal * normal.dot(corner - middle) * (1.0 + PAST);
    (Contour::straight(outline), frame, travel)
}
