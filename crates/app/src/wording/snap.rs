use cao_sketch::Snap;

use crate::lang::Catalogue;

/// The name a snap is written under beside its ring: what the cursor was
/// pulled onto.
pub fn name(lang: &Catalogue, snap: Snap) -> String {
    lang.t(match snap {
        Snap::Point => "snap.point",
        Snap::Midpoint(_) => "snap.midpoint",
        Snap::Crossing(_) => "snap.crossing",
        Snap::Centre(_) => "snap.centre",
        Snap::OnCurve(_) => "snap.on_curve",
    })
}
