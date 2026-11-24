//! Which of the sections the part keeps is on, and so cut with: worked out
//! from the hand's choices the part keeps, the order the sections were saved
//! in, the point the part is read to and the drawing open — never held apart
//! from them.

use cao_part::{PartDocument, SavedSection};

use super::section::Section;

#[derive(Default)]
pub(crate) struct KeptSections {
    /// The kept section on, by id. One at most.
    on: Option<usize>,
    /// It, as the part stands now; nothing while it is broken and cuts nothing.
    shown: Option<Section>,
    /// The section under the drawing open, which shows its cut while it is
    /// open whatever the rule and the point the part is read to.
    under_the_sketch: Option<usize>,
    /// The kept sections past the step the part is read to, by id: none of
    /// them is made yet, so none is cut with.
    unread: Vec<usize>,
}

impl KeptSections {
    pub(crate) fn is_on(&self, section: usize) -> bool {
        self.on == Some(section)
    }

    /// The section the frame cuts with.
    pub(crate) fn shown(&self) -> Option<&Section> {
        self.shown.as_ref()
    }

    /// The id of the section on, when it cuts anything.
    pub(crate) fn cutting(&self) -> Option<usize> {
        self.on.filter(|_| self.shown.is_some())
    }

    /// Sets the sections the part is not read through, and follows the part
    /// as it stands. Says whether they or the section on changed.
    pub(crate) fn leave_unread(&mut self, document: &PartDocument, sections: Vec<usize>) -> bool {
        let changed = self.unread != sections;
        self.unread = sections;
        self.choose(document) || changed
    }

    /// Follows the part and the drawing open: a drawing laid on a section
    /// shows that section's cut while it is open, and leaving it gives the
    /// rule back. Says whether the section on changed.
    pub(crate) fn keep_up_with(&mut self, document: &PartDocument, sketch: Option<usize>) -> bool {
        self.under_the_sketch = sketch.and_then(|open| document.section_under(open));
        self.choose(document)
    }

    /// What a click on a section's eye does: shuts it by hand while it is
    /// the one on, opens it by hand otherwise.
    pub(crate) fn clicked(&self, document: &mut PartDocument, section: usize) {
        match self.is_on(section) {
            true => document.set_section_choice(section, Some(false)),
            false => open_by_hand(document, section),
        }
    }

    fn choose(&mut self, document: &PartDocument) -> bool {
        let was = self.on;
        let sections = document.sections();
        let found = match self.under_the_sketch {
            Some(under) => sections.iter().find(|kept| kept.id == under),
            None => self.by_the_rule(document, &sections),
        };
        self.on = found.map(|kept| kept.id);
        self.shown = found.filter(|kept| !kept.broken).map(Section::reopened);
        was != self.on
    }

    /// Among the sections the part is read through, the one opened by hand;
    /// else the last saved, unless the hand shut it.
    fn by_the_rule<'a>(
        &self,
        document: &PartDocument,
        sections: &'a [SavedSection],
    ) -> Option<&'a SavedSection> {
        let read = || {
            sections
                .iter()
                .filter(|kept| !self.unread.contains(&kept.id))
        };
        let opened = read()
            .rev()
            .find(|kept| document.section_choice(kept.id) == Some(true));
        let last = read()
            .next_back()
            .filter(|kept| document.section_choice(kept.id).is_none());
        opened.or(last)
    }
}

/// Opens a section by hand, the one opened by hand before it given back to
/// the rule.
pub(crate) fn open_by_hand(document: &mut PartDocument, section: usize) {
    none_opened_by_hand(document);
    document.set_section_choice(section, Some(true));
}

/// Gives every section opened by hand back to the rule. One shut by hand
/// stays shut.
pub(crate) fn none_opened_by_hand(document: &mut PartDocument) {
    for kept in document.sections() {
        if document.section_choice(kept.id) == Some(true) {
            document.set_section_choice(kept.id, None);
        }
    }
}
