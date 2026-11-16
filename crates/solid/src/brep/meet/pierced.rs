//! A cylinder and a sphere whose centre its axis does not hold, read as a
//! pair (#805). In the frame whose third axis is the cylinder's, whose second
//! runs from it towards the centre and whose origin is on it, the two are
//! `x² + y² = a²` and `x² + (y − d)² + (z − e)² = b²`, so that on the
//! cylinder `(z − e)² = 2d (y − y₀)`, `y₀ = (a² + d² − b²) / 2d`: a root
//! vanishing at `y₀` alone. The curve spans `y` over `[max(−a, y₀), a]`, `x`
//! vanishing at its high end; one loop where `y₀` stands within the
//! cylinder's section, two where the cylinder passes right through the ball.
//! With the span's half-width `l`, the root is `√(2d) √(g + 2l s²)`, `g` its
//! gap at the low end, so the sphere's root factors as a second wall's does
//! there (`pair.rs`), and every gap is written as a product, so that none is
//! found by taking two close numbers from each other.

use super::pair::{Pair, ROUNDING};
use super::section::Section;
use crate::brep::surface::{Cylinder, Sphere};

impl Pair {
    /// The pair of `cylinder` and `ball`, the centre off the axis: its high
    /// end where the cylinder's ruling nearest the centre stands, the root of
    /// the sphere never vanishing there.
    pub(super) fn pierced(cylinder: &Cylinder, ball: &Sphere) -> Pair {
        let third = cylinder.axis;
        let between = ball.middle() - cylinder.origin;
        let off = between - third * third.dot(between);
        let (a, b, d) = (cylinder.radius, ball.radius, off.length());
        let two = off / d;
        let one = two.cross(third);
        let (near, far) = (d - a, d + a);
        let reach = (b - near) * (b + near) / (2.0 * d);
        let beyond = (b - far) * (b + far) / (2.0 * d);
        let (mut low, mut x_gap, mut z_gap) = if beyond > 0.0 {
            (-a, 0.0, beyond)
        } else {
            (a - reach, -beyond, 0.0)
        };
        let rounding = ROUNDING * (1.0 + cylinder.origin.length() + ball.middle().length() + a + b);
        if x_gap.max(z_gap) <= rounding {
            (low, x_gap, z_gap) = (-a, 0.0, 0.0);
        }
        Pair {
            origin: cylinder.origin,
            axes: [one, two, third],
            a,
            b,
            d,
            e: between.dot(third),
            low,
            high: a,
            x_gaps: [x_gap, 0.0],
            z_gaps: [z_gap, f64::INFINITY],
            rounding,
            sections: [Section::Round, Section::Ball],
            across: third,
            slant: None,
        }
    }
}
