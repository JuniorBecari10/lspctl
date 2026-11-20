use crate::{operations::model::Marker, table::cell::Cell};

pub struct Row {
    pub cells: Vec<Cell>,
    markers: Vec<Marker>,
}

impl Row {
    pub fn new(cells: impl IntoIterator<Item = Cell>) -> Self {
        Self {
            cells: cells.into_iter().collect(),
            markers: Vec::new(),
        }
    }

    // adds a marker. accepts Marker and Option<Marker>
    pub fn marker(mut self, marker: impl Into<Option<Marker>>) -> Self {
        self.markers.extend(marker.into());
        self
    }

    pub fn markers(mut self, markers: impl IntoIterator<Item = Marker>) -> Self {
        self.markers.extend(markers);
        self
    }

    // all markers rendered and space-separated, or None if there are none.
    pub fn render_markers(&self) -> Option<String> {
        if self.markers.is_empty() {
            return None;
        }

        Some(
            self.markers
                .iter()
                .map(Marker::render)
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}
