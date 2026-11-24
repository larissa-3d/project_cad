//! A section the part keeps, as the section view takes it up and hands it
//! back: the part says how each side is drawn in words of its own, which the
//! renderer's are mapped to here.

use cao_part::history::{Operation, SectionAsked, SideDrawing};
use cao_part::{PartDocument, SavedSection};
use cao_render::MatterInFront;

use super::Section;
use crate::screens::viewport::reading::Reading;

impl Section {
    /// The section view on a kept section, as the part holds it now.
    pub(crate) fn reopened(saved: &SavedSection) -> Self {
        let mut section = Self::new(saved.plane);
        section.offset = saved.offset;
        section.tilt_about_u_degrees = saved.tilt_about_u_degrees;
        section.tilt_about_v_degrees = saved.tilt_about_v_degrees;
        section.middle = saved.pivot;
        section.pivot = saved.pivot;
        section.behind = drawn_as(saved.behind);
        section.in_front = drawn_as(saved.in_front);
        section.on = saved.on;
        section.editing = Some(saved.id);
        section
    }

    pub(crate) fn asked(&self) -> SectionAsked {
        SectionAsked {
            plane: self.plane,
            on: self.on,
            offset: self.offset,
            behind: kept_as(self.behind),
            in_front: kept_as(self.in_front),
            tilt_about_u_degrees: self.tilt_about_u_degrees,
            tilt_about_v_degrees: self.tilt_about_v_degrees,
            pivot: cao_part::pivot_kept_on(self.plane, self.pivot),
        }
    }

    /// Keeps the section in the part read whole, as
    /// [`Self::keep_where_read`] does.
    #[cfg_attr(not(test), expect(dead_code))]
    pub(crate) fn keep_in(&mut self, document: &mut PartDocument) {
        self.keep_where_read(document, &mut Reading::default());
    }

    /// Keeps the section in the part: a new one, put in where the part is
    /// read as any key step is, the one opened by hand before it given back
    /// to the rule; or the one it was reopened from, changed. Kept again
    /// later, it changes the same one — while the part still has it: undone
    /// or deleted, keeping it makes a new one.
    pub(in crate::screens::viewport) fn keep_where_read(
        &mut self,
        document: &mut PartDocument,
        reading: &mut Reading,
    ) {
        let to = self.asked();
        let standing = document.sections();
        let editing = self
            .editing
            .filter(|section| standing.iter().any(|kept| kept.id == *section));
        match editing {
            Some(section) => {
                reading.record(document, Operation::EditSection { section, to });
            }
            None => {
                reading.record(document, Operation::SaveSection(to));
                self.editing = document.sections().last().map(|kept| kept.id);
            }
        }
    }
}

fn drawn_as(side: SideDrawing) -> MatterInFront {
    match side {
        SideDrawing::Hidden => MatterInFront::Hidden,
        SideDrawing::SeeThrough => MatterInFront::SeeThrough,
        SideDrawing::Opaque => MatterInFront::Opaque,
    }
}

fn kept_as(drawn: MatterInFront) -> SideDrawing {
    match drawn {
        MatterInFront::Hidden => SideDrawing::Hidden,
        MatterInFront::SeeThrough => SideDrawing::SeeThrough,
        MatterInFront::Opaque => SideDrawing::Opaque,
    }
}
