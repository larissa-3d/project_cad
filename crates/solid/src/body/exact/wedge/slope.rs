//! A cut beside a fill (#791): an edge whose faces meet outwards and one
//! whose faces meet inwards, facing each other across the face they share,
//! whose set-backs meet there — a tread's nose and foot set back by half its
//! depth — the face they shared gone, whichever edge is named first (#812).
//! Where their flats lie at an angle the kernel tells from none — a riser
//! turned or leaning by a hair among them — they are one slope, the fill
//! raised on the cut's plane and cut back to the riser's; where it tells it
//! — a stair whose front leans — two flats meeting in a crease. A chain of
//! them, each fill between two cuts — a flight's noses and feet, each tread
//! as deep as each riser is high — is one slope across it, every fill raised
//! on the first cut's plane (#813), and every other cut too, so that a flight
//! whose riser strays from one plane by a hair is one slope (#929). The
//! front's upright at an end of the
//! nose is mitred against the slope as against the nose's own flat (#814).
//! Where the fill takes the riser whole, the slope is laid through the
//! riser's top edge instead, the cut raised on it too, so that it meets the
//! top beyond along that edge however the riser is turned (#928).

use glam::{DVec2, DVec3};

use super::chain::TANGENT;
use super::end::behind;
use super::flush::Flush;
use super::reading::reaching;
use super::{Prism, SHORTEST, Wedge, raised_over};
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, FaceId, Plane, Scale, Surface, VertexId};
use crate::profile::Contour;

/// The plane of the face `id`, where it is flat.
fn plane_of(body: &brep::Body, id: FaceId) -> Option<Plane> {
    match body.surface(body.face(id).surface) {
        Surface::Plane(plane) => Some(*plane),
        _ => None,
    }
}

/// How far a point of the box from `low` to `high` may stand from `point`.
fn farthest((low, high): (DVec3, DVec3), point: DVec3) -> f64 {
    (((low + high) / 2.0 - point).abs() + (high - low) / 2.0).length()
}

/// How far the run of `wedge`'s prism raised along `along`, the slope's way,
/// goes on past the face its edge does not share, within `bounds`: as far
/// as its edge and that way part across them, and the shortest set-back
/// the kernel keeps, [`SHORTEST`] hairs.
fn past(along: DVec3, wedge: &Wedge, scale: Scale, bounds: (DVec3, DVec3)) -> f64 {
    let part = along.cross(wedge.along).length() * farthest(bounds, wedge.point);
    part + SHORTEST * Scale::HAIR * scale.eps()
}

impl Wedge {
    /// The prism this wedge is cut off or filled in with so that its flat
    /// lies on `plane`, the slope, which holds `along`, the slope's way:
    /// raised along `along` over a section square to it, as
    /// [`Wedge::squared_off`] raises one along the edge's own way, its first
    /// run on `plane` from the face the two edges do not share to the face of
    /// rank `shared`, which they do. That other face runs along `along` only
    /// within what the edge and that way part by: the run goes on `past`
    /// behind the line it meets that face on at this edge's point — into the
    /// air past a cut's, into the matter past a fill's, for the prism to be
    /// cut back to the face's own plane, as [`Exact::sloped`] cuts it.
    fn raised_on(
        &self,
        along: DVec3,
        plane: &Plane,
        (shared, past): (usize, f64),
        scale: Scale,
        bounds: (DVec3, DVec3),
    ) -> Prism {
        let square = |way: DVec3| (way - along * along.dot(way)).normalize();
        let straight = Wedge {
            point: self.point,
            along,
            faces: self.faces,
            numbers: self.numbers.clone(),
            outward: self.outward.map(square),
            into: self.into.map(square),
            reach: self.reach,
            dip: self.dip,
            inwards: self.inwards,
        };
        let (across, facing) = straight.square();
        let margin = straight.margin(across, facing, scale);
        let off = straight.off();
        let (frame, travel) = straight.raised_along(across, margin, bounds);
        let height = plane.normal.dot(plane.origin - self.point);
        let on_plane = |from: DVec3, way: DVec3| {
            from + way * ((height - plane.normal.dot(from)) / plane.normal.dot(way))
        };
        let [first, second] = [0, 1].map(|side| {
            let behind = if side == shared { 0.0 } else { past };
            let outward = straight.outward[side] * off;
            let end = on_plane(outward * behind, straight.into[side]);
            [end, end + outward * (margin - behind)]
        });
        let mut corners = vec![second[0], first[0], first[1]];
        corners.extend(straight.rim(margin));
        corners.push(second[1]);
        let flat = |at: DVec3| DVec2::new(at.dot(frame.u), at.dot(frame.v));
        let outline = Contour::straight(corners.into_iter().map(flat).collect());
        (outline, frame, travel)
    }

    /// Where this wedge, a fill beside `cut`, takes whole on `given`, the
    /// body as given, the face `cut` does not share — its set-back `value`
    /// up that face reaching within [`past`] of the face's far side, a riser
    /// as high as the set-back — the plane through that far side parallel to
    /// its own flat: the slope that meets the face beyond along that edge,
    /// where the cut's own plane, turned from it by a hair, would cross the
    /// edge, leaving a sliver of the riser below it at one end and standing
    /// over the top at the other. None where it does not take it whole.
    fn taken_whole(&self, given: &brep::Body, cut: &Wedge, value: f64) -> Option<Plane> {
        let shared = (self.faces.iter()).position(|face| cut.faces.contains(face))?;
        let way = self.into[1 - shared];
        let reach = reaching(given, self.faces[1 - shared], way) - way.dot(self.point);
        if reach - value > past(cut.along, self, given.scale(), given.bounds()?) {
            return None;
        }
        let normal = (self.into[0] - self.into[1]).cross(self.along);
        Some(Plane::through(self.point + way * reach, normal).0)
    }

    /// How near `slope` stands to this wedge's edge `edge` on `given`, the
    /// body as given: the least of its set-backs along either face, at either
    /// end of the edge. None where the edge has no ends.
    fn nearest_on(&self, given: &brep::Body, edge: EdgeId, slope: &Plane) -> Option<f64> {
        let set_back = |(end, into): (VertexId, DVec3)| {
            slope.normal.dot(slope.origin - given.vertex(end).point) / slope.normal.dot(into)
        };
        let ends = given.edge(edge).ends?;
        let set_backs = (ends.into_iter()).flat_map(|end| self.into.map(|into| (end, into)));
        Some(set_backs.map(set_back).fold(f64::INFINITY, f64::min))
    }
}

impl Exact {
    /// The body with the edges of `chain`, each with its wedge on `given`,
    /// the body as given, and its ends read there, as an edge meeting another
    /// laid before it is — cut off where their faces meet outwards and filled
    /// in where they meet inwards, each two facing each other across a face
    /// they share in a chain, their set-backs meeting there at `value` — on
    /// this body, whatever the list laid before them, the cuts first, then
    /// the fills on what the cuts left, each in the order of the numbers its
    /// faces answer to, whatever order the names give, so that the body is
    /// one whatever it is — a cut's prism runs on a margin past its set-back,
    /// which would lie flush on, or cut into, a fill laid before it — each cut
    /// by the prism `tool` raises over it, the flats numbered from where the steps' numbers
    /// ended in the order they are laid, then each by its own edge's number
    /// in `numbers`, as [`Exact::paired`] numbers them, its corners at a
    /// mitre with a face numbered in `mitred`, laid before it, left there —
    /// the front's upright at the nose's end named first. Where every flat lies
    /// at an angle the kernel tells from none to the first cut's, within
    /// [`TANGENT`] — each facing the way between the ways its edge's two
    /// faces face — they are one slope, every other cut raised on that cut's
    /// plane, as [`Exact::cut_on`] lays it, and every fill, as
    /// [`Exact::filled_on_slope`] lays it — but where one cut and
    /// one fill lay it and the fill takes its other face whole, as
    /// [`Wedge::taken_whole`] reads it: there the cut is raised on the plane
    /// through that face's far side, as [`Exact::cut_on`] lays it, and the
    /// fill on the cut's flat along its own way. With the body, the nearest
    /// the slope stands to any of its edges, as [`Wedge::nearest_on`] reads
    /// it, or `value` where it stands nowhere nearer and for a crease, which
    /// the step then reports as taken. Two flats at an angle
    /// the kernel tells meet in a crease, the fill laid by the prism `tool`
    /// raises. Declined as [`Exact::cut_off`] says, and `Unsupported` before
    /// anything is laid where a chain of more than two holds a crease — a
    /// flight whose front leans, until #926 — where the kernel left the
    /// slope in more faces than one, or where the
    /// crease's fill left something else than its one face, as
    /// [`Exact::cut_along`] reads it.
    pub(super) fn sloped(
        &self,
        given: &brep::Body,
        chain: &[(EdgeId, &Wedge)],
        numbers: &[u32],
        (value, mitred): (f64, &[u32]),
        tool: &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
    ) -> Result<(Exact, f64), Declined> {
        let (mut cuts, mut fills): (Vec<usize>, Vec<usize>) =
            (0..chain.len()).partition(|&rank| !chain[rank].1.inwards);
        for laying in [&mut cuts, &mut fills] {
            laying.sort_by_key(|&rank| chain[rank].1.numbers.clone());
        }
        let first = chain[*cuts.first().ok_or(Declined::Unsupported)?].1;
        let facing = first.square().1;
        let crease =
            (chain.iter()).any(|(_, wedge)| wedge.square().1.cross(facing).length() > TANGENT);
        if crease && chain.len() > 2 {
            return Err(Declined::Unsupported);
        }
        let whole = match (&cuts[..], &fills[..]) {
            (&[cut], &[fill]) if !crease => {
                let fill = chain[fill].1;
                (fill.taken_whole(given, chain[cut].1, value)).map(|slope| (fill.along, slope))
            }
            _ => None,
        };
        let fill_faces: Vec<FaceId> = fills.iter().flat_map(|&rank| chain[rank].1.faces).collect();
        let flush = Flush::at(given, chain[cuts[0]].0);
        let mut laid = match &whole {
            Some((along, slope)) => self.cut_on(&flush, first, (*along, &fill_faces), slope)?,
            None => self.cut_or_filled(&flush, first, value, tool)?,
        };
        let way = whole.map_or(first.along, |(along, _)| along);
        let made = self.next as usize;
        let slope = (laid.named(made))
            .find_map(|face| plane_of(&laid.brep, face))
            .ok_or(Declined::Unsupported)?;
        for &rank in &cuts[1..] {
            let (edge, wedge) = chain[rank];
            let flush = Flush::at(given, edge);
            laid = laid.cut_on(&flush, wedge, (way, &fill_faces), &slope)?;
        }
        let cut_faces: Vec<FaceId> = cuts.iter().flat_map(|&rank| chain[rank].1.faces).collect();
        for &rank in &fills {
            let (edge, wedge) = chain[rank];
            let flush = Flush::at(given, edge);
            let filled = if crease {
                let filled = laid.cut_or_filled(&flush, wedge, value, tool)?;
                if !filled.cut_along(&[wedge], &laid, 1, &[], &[], &[]) {
                    return Err(Declined::Unsupported);
                }
                filled
            } else {
                laid.filled_on_slope(given, (&flush, wedge), &cut_faces, (way, &slope))?
            };
            laid = filled;
        }
        let one = (1..chain.len()).all(|more| laid.named(made).eq(laid.named(made + more)));
        if !crease && !one {
            return Err(Declined::Unsupported);
        }
        let nearest = (chain.iter())
            .filter(|_| !crease)
            .filter_map(|&(edge, wedge)| wedge.nearest_on(given, edge, &slope))
            .fold(value, f64::min);
        let order: Vec<usize> = cuts.iter().chain(&fills).copied().collect();
        let wedges: Vec<&Wedge> = order.iter().map(|&rank| chain[rank].1).collect();
        let numbers: Vec<u32> = order.iter().map(|&rank| numbers[rank]).collect();
        Ok((laid.paired(self, &wedges, &numbers, mitred)?, nearest))
    }

    /// The body with the edge `flush` reads on the body as given, as `wedge`,
    /// cut off on `slope`, which holds `along`, the slope's way: raised on
    /// that plane along it, as [`Wedge::raised_on`] raises it, from the face
    /// of `wedge` that none of `fill_faces`, those of the fills it is laid
    /// with, is, its run going on past it into the air by [`past`], so that
    /// the slope crosses that face whichever way the edge strays from the
    /// slope's way. Declined `Unsupported` where it shares no face with a
    /// fill; otherwise as the trims and the cut decline.
    fn cut_on(
        &self,
        flush: &Flush,
        wedge: &Wedge,
        (along, fill_faces): (DVec3, &[FaceId]),
        slope: &Plane,
    ) -> Result<Exact, Declined> {
        let shared = (wedge.faces.iter())
            .position(|face| fill_faces.contains(face))
            .ok_or(Declined::Unsupported)?;
        let scale = self.brep.scale();
        let bounds = (self.brep.bounds())
            .and_then(|bounds| flush.carried_over(wedge, bounds))
            .ok_or(Declined::Unsupported)?;
        let past = past(along, wedge, scale, bounds);
        let prism = wedge.raised_on(along, slope, (shared, past), scale, bounds);
        self.laid_flush(flush, wedge, &prism, raised_over(&prism)?)
    }

    /// The body with the edge `flush` reads on `given`, the body as given,
    /// as `inward`, filled in on `slope`, the plane of the flat laid beside
    /// it, which holds `along`, the slope's way: raised on that plane, as
    /// [`Wedge::raised_on`] raises it, from the face of `inward` among
    /// `cut_faces`, those of the cuts it is laid with, read on `given` too,
    /// its run going on behind its other face by [`past`], then cut back to
    /// that face's own plane, as [`behind`] cuts it — so that it ends on that
    /// face whichever way it strays from the slope's way, and never stands
    /// past it, over the face beyond, where the slope takes it whole.
    /// Declined `Unsupported` where neither face of `inward` is a cut's, or
    /// the other is not flat; otherwise as the trim and the join decline.
    fn filled_on_slope(
        &self,
        given: &brep::Body,
        (flush, inward): (&Flush, &Wedge),
        cut_faces: &[FaceId],
        (along, slope): (DVec3, &Plane),
    ) -> Result<Exact, Declined> {
        let shared = (inward.faces.iter())
            .position(|face| cut_faces.contains(face))
            .ok_or(Declined::Unsupported)?;
        let other = inward.faces[1 - shared];
        let riser = plane_of(given, other).ok_or(Declined::Unsupported)?;
        let scale = self.brep.scale();
        let bounds = self.brep.bounds().ok_or(Declined::Unsupported)?;
        let past = past(along, inward, scale, bounds);
        let prism = inward.raised_on(along, slope, (shared, past), scale, bounds);
        let behind = behind(&prism, &riser, -inward.outward[1 - shared])?;
        let tool = raised_over(&prism)?.cut_by(&behind)?;
        self.laid_flush(flush, inward, &prism, tool)
    }
}
