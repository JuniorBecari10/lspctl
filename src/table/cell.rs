use colored::ColoredString;

use crate::table::{rendered::Rendered, util};

pub struct Cell(pub Option<Rendered>);

impl Cell {
    pub fn absent() -> Self {
        Self(None)
    }
}

impl From<&str> for Cell {
    fn from(s: &str) -> Self {
        Self(Some(Rendered::new(s.to_string(), util::display_width(s))))
    }
}

impl From<String> for Cell {
    fn from(s: String) -> Self {
        let width = util::display_width(&s);
        Self(Some(Rendered::new(s, width)))
    }
}

impl From<ColoredString> for Cell {
    fn from(s: ColoredString) -> Self {
        // ColoredString derefs to the unstyled text, so this is the visible width
        let width = util::display_width(&s);
        Self(Some(Rendered::new(s.to_string(), width)))
    }
}

impl<T: Into<Cell>> From<Option<T>> for Cell {
    fn from(value: Option<T>) -> Self {
        value.map_or_else(Cell::absent, Into::into)
    }
}
