//! Where the view shows a point of the drawing, for the driver tests that
//! click it there: through the camera and the canvas of the last frame drawn.
#![cfg(feature = "test-support")]

use glam::{DVec2, DVec3};

use super::CaoApp;
use crate::screens::part_tree::state::{PartTree, Points};
use crate::screens::{OpenPart, Screen};

/// A row of the Part tree the view lights, named as the tree names it and the
/// sketch it stands under.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LitRow {
    pub sketch: String,
    pub row: String,
}

impl CaoApp {
    /// Where a point of the world lands on screen: nothing while no part is
    /// drawn, or for a point behind the camera.
    pub fn where_the_view_shows(&self, world: DVec3) -> Option<egui::Pos2> {
        self.part_opened()?.viewport.where_it_shows(world)
    }

    /// The same for a point of the sketch being drawn, in that sketch's own
    /// coordinates: nothing while no sketch is.
    pub fn where_the_view_shows_the_sketch_point(&self, point: DVec2) -> Option<egui::Pos2> {
        let part = self.part_opened()?;
        let sketch = part.doc.sketches().get(part.editor.active_sketch()?)?;
        part.viewport.where_it_shows(sketch.plane.to_world(point))
    }

    /// The row of the Part tree the hand is on, as the view lights it.
    pub fn the_row_lit_by_the_hand(&self) -> Option<LitRow> {
        let part = self.part_opened()?;
        self.named(part, part.viewport.hovered_in_the_tree()?)
    }

    /// The row of the Part tree a click lit in the view.
    pub fn the_row_lit_by_a_click(&self) -> Option<LitRow> {
        let part = self.part_opened()?;
        self.named(part, part.viewport.lit()?)
    }

    /// The points of the edge of the part the hand lights, while the edge
    /// panel is open.
    pub fn the_edge_lit_by_the_hand(&self) -> Option<Vec<DVec3>> {
        let part = self.part_opened()?;
        let name = part.viewport.edge_under_the_hand.as_ref()?;
        part.viewport.edge_points(part.doc.body(), name)
    }

    /// The points of the edge the edge panel holds as chosen, while it holds
    /// just one.
    pub fn the_edge_chosen(&self) -> Option<Vec<DVec3>> {
        match <[Vec<DVec3>; 1]>::try_from(self.the_edges_chosen()) {
            Ok([edge]) => Some(edge),
            Err(_) => None,
        }
    }

    /// The points of every edge the edge panel holds as chosen, on the part
    /// they are named on: the part as it stood before the step reopened.
    pub fn the_edges_chosen(&self) -> Vec<Vec<DVec3>> {
        self.part_opened().map_or_else(Vec::new, |part| {
            let before = part
                .blend
                .reopened
                .map(|step| part.doc.shown_before(step).body);
            let body = before.as_ref().unwrap_or(part.doc.body());
            part.blend
                .edges
                .iter()
                .filter_map(|name| part.viewport.edge_points(body, name))
                .collect()
        })
    }

    /// The points of every edge the view lights: those chosen and the one
    /// under the hand.
    pub fn the_edges_lit(&self) -> Vec<Vec<DVec3>> {
        self.part_opened().map_or_else(Vec::new, |part| {
            let viewport = &part.viewport;
            viewport
                .edges_chosen
                .iter()
                .chain(&viewport.edge_under_the_hand)
                .filter_map(|name| viewport.edge_points(part.doc.body(), name))
                .collect()
        })
    }

    fn named(&self, part: &OpenPart, lit: Points) -> Option<LitRow> {
        let tree = PartTree::of(&part.doc, self.remembered.lang());
        let drawing = tree
            .sketches
            .iter()
            .find(|drawing| drawing.sketch == lit.sketch())?;
        let rows = [
            &drawing.strokes,
            &drawing.areas,
            &drawing.dimensions,
            &drawing.rules,
        ];
        let row = rows.into_iter().flatten().find(|row| row.points == lit)?;
        Some(LitRow {
            sketch: drawing.name.clone(),
            row: row.name.clone(),
        })
    }

    fn part_opened(&self) -> Option<&OpenPart> {
        match &self.screen {
            Screen::PartOpened(part) => Some(part),
            Screen::StartMenu => None,
        }
    }
}
