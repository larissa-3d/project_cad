//! The panel an edge of the part is chamfered or rounded from: which of the
//! two, the edges chosen, and the size typed.

use cao_part::history::{BlendShape, Operation};
use cao_part::{Formula, Outcome, PartDocument, Reduced, Unusable, Variables, is_a_blend_size};
use cao_prefs::Command;
use cao_solid::{Declined, EdgeName};

/// What is wrong with a size typed in the edge panel.
#[derive(Clone, Debug, PartialEq)]
pub enum WrongSize {
    Unusable(Unusable),
    NotAboveZero,
}

/// What the edge panel remembers between frames. Closed while `shape` is
/// nothing.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BlendState {
    pub shape: Option<BlendShape>,
    pub edges: Vec<EdgeName>,
    /// The distance or the radius typed, in millimetres.
    pub size_input: String,
    /// Why the kernel declined the last « Appliquer », until the edges
    /// chosen change.
    pub declined: Option<Declined>,
    /// The number of the step the panel was reopened on, which « Appliquer »
    /// changes in place rather than laying a new one.
    pub reopened: Option<u32>,
    /// The size asked and the one taken of the step reopened as the panel
    /// holds it, when the kernel took less.
    pub reduced: Option<Reduced>,
}

impl BlendState {
    /// The panel reopened on the blend step at `step` in the history, as the
    /// part now replays it: its kind, its size as written and its edges.
    /// Nothing for any other step.
    pub fn reopened(document: &PartDocument, step: usize) -> Option<Self> {
        let raised_by = document.history.number_at(step)?;
        let Operation::Blend { shape, size, .. } = document.raised_as(raised_by)? else {
            return None;
        };
        Some(Self {
            shape: Some(*shape),
            edges: document.edges_before(raised_by),
            size_input: document.variables().written(size),
            declined: None,
            reopened: Some(raised_by),
            reduced: None,
        })
    }

    /// Opens the panel on a chamfer or a fillet, or turns the one open into
    /// the other, keeping the edges chosen and the size typed.
    pub fn offer(&mut self, shape: BlendShape) {
        if !self.is_active() {
            self.size_input = "1".to_string();
        }
        self.shape = Some(shape);
        self.declined = None;
    }

    /// Chooses this edge alone, in place of those chosen.
    pub fn choose(&mut self, edge: EdgeName) {
        self.edges = vec![edge];
        self.declined = None;
    }

    /// Adds an edge to those chosen, or takes it out when it is one of them.
    pub fn toggle(&mut self, edge: EdgeName) {
        match self.edges.iter().position(|chosen| *chosen == edge) {
            Some(at) => {
                self.edges.remove(at);
            }
            None => self.edges.push(edge),
        }
        self.declined = None;
    }

    /// The command a pressed chord stands for: the one apply chord applies
    /// the blend while the edge panel is open, the extrusion otherwise.
    pub fn answering(&self, command: Command) -> Command {
        match command {
            Command::ExtrusionApply if self.is_active() => Command::BlendApply,
            other => other,
        }
    }

    pub fn is_active(&self) -> bool {
        self.shape.is_some()
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }

    /// A panel reopened on a step closes once the history moves under it: the
    /// step it names may no longer be the one in effect. One armed afresh
    /// stays.
    pub fn history_moved(&mut self) {
        if self.reopened.is_some() {
            self.close();
        }
    }

    /// The size typed, as written and as what it comes to, when it is one an
    /// edge can be given.
    pub fn size(&self, variables: &Variables) -> Option<(Formula, f64)> {
        variables
            .size_of(&self.size_input)
            .ok()
            .filter(|(_, value)| is_a_blend_size(*value))
    }

    /// What is wrong with the size typed, when it is none an edge can be
    /// given; nothing for a field left empty.
    pub fn wrong(&self, variables: &Variables) -> Option<WrongSize> {
        if self.size_input.trim().is_empty() {
            return None;
        }
        match variables.size_of(&self.size_input) {
            Err(wrong) => Some(WrongSize::Unusable(wrong)),
            Ok((_, value)) if !is_a_blend_size(value) => Some(WrongSize::NotAboveZero),
            Ok(_) => None,
        }
    }

    /// Takes what the kernel made of the step reopened as the panel holds it:
    /// true when that changes what the panel says.
    pub fn tried(&mut self, declined: Option<Declined>, reduced: Option<Reduced>) -> bool {
        let said = (self.declined, self.reduced);
        (self.declined, self.reduced) = (declined, reduced);
        said != (declined, reduced)
    }

    /// Whether there is enough to apply: an edge chosen, and a usable size;
    /// for a step reopened, one the kernel does not decline as it stands.
    pub fn is_ready(&self, variables: &Variables) -> bool {
        let declined = self.reopened.is_some() && self.declined.is_some();
        !declined
            && self.shape.is_some()
            && !self.edges.is_empty()
            && self.size(variables).is_some()
    }

    /// The step the panel holds, as « Appliquer » would lay it: its kind, an
    /// edge chosen, and a usable size.
    pub fn laid(&self, variables: &Variables) -> Option<Operation> {
        let (shape, (size, _)) = (self.shape?, self.size(variables)?);
        let edges = self.edges.clone();
        (!edges.is_empty()).then_some(Operation::Blend { edges, shape, size })
    }
}

/// Chamfers or rounds the chosen edges as one step, or changes the step
/// reopened to them, and closes the panel; a step left as it was records
/// nothing. Edges the kernel declines record nothing and leave the panel open,
/// saying why. True when the part changed.
pub fn apply_blend(document: &mut PartDocument, blend: &mut BlendState) -> bool {
    let Some(laid) = blend.laid(document.variables()) else {
        return false;
    };
    let operation = match blend.reopened {
        Some(raised_by) if leaves_as_it_was(document, raised_by, &laid) => {
            blend.close();
            return false;
        }
        Some(raised_by) => Operation::EditMatter {
            raised_by,
            to: Box::new(laid),
        },
        None => laid,
    };
    if let Some(Outcome::BlendDeclined(reason)) = document.apply(operation) {
        blend.declined = Some(reason);
        return false;
    }
    blend.close();
    true
}

/// Whether `laid` is the step of that number as it stands: its kind, its size
/// as written, and its edges whatever their order, none of them lost.
fn leaves_as_it_was(document: &PartDocument, raised_by: u32, laid: &Operation) -> bool {
    let (
        Some(Operation::Blend {
            edges: was,
            shape,
            size,
        }),
        Operation::Blend { edges, .. },
    ) = (document.raised_as(raised_by), laid)
    else {
        return false;
    };
    let found = document.edges_before(raised_by);
    let same_edges = found.len() == was.len()
        && edges.len() == found.len()
        && edges.iter().all(|edge| found.contains(edge));
    same_edges
        && matches!(laid, Operation::Blend { shape: now, size: typed, .. } if now == shape && typed == size)
}
