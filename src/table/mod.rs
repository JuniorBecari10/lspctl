use colored::Colorize;

use crate::table::{cell::Cell, rendered::Rendered, row::Row};

pub mod cell;
pub mod fields;
mod rendered;
pub mod row;
pub mod util;

pub const LONG_COL: usize = 15;
pub const SHORT_COL: usize = 6;

struct Column {
    header: String,
    min_width: usize,
}

pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Row>,
    gap: usize,
    placeholder: Rendered,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            gap: 2,
            placeholder: Self::make_placeholder("—"),
        }
    }

    fn make_placeholder(text: &str) -> Rendered {
        Rendered::new(text.dimmed().to_string(), util::display_width(text))
    }

    pub fn column(mut self, header: impl Into<String>, min_width: usize) -> Self {
        self.columns.push(Column {
            header: header.into(),
            min_width,
        });
        self
    }

    pub fn push_row(&mut self, row: Row) -> &mut Self {
        debug_assert_eq!(
            row.cells.len(),
            self.columns.len(),
            "row has a different number of cells than the table has columns"
        );
        self.rows.push(row);
        self
    }

    pub fn print(&self) {
        print!("{}", self.render());
    }

    pub fn render(&self) -> String {
        let widths = self.column_widths();
        let total = widths.iter().sum::<usize>() + self.gap * self.columns.len().saturating_sub(1);

        let mut out = String::new();

        let headers: Vec<(String, usize)> = self
            .columns
            .iter()
            .map(|c| (c.header.bold().to_string(), util::display_width(&c.header)))
            .collect();
        self.write_line(
            &mut out,
            headers.iter().map(|(s, w)| (s.as_str(), *w)),
            &widths,
            None,
        );

        out.push_str(&util::rule(total).to_string());
        out.push('\n');

        for row in &self.rows {
            let markers = row.render_markers();
            self.write_line(
                &mut out,
                row.cells.iter().map(|c| {
                    let r = self.resolve(c);
                    (r.text.as_str(), r.width)
                }),
                &widths,
                markers.as_deref(),
            );
        }

        out
    }

    /// The cell's own content, or the table's placeholder if it's absent.
    fn resolve<'a>(&'a self, cell: &'a Cell) -> &'a Rendered {
        cell.0.as_ref().unwrap_or(&self.placeholder)
    }

    fn column_widths(&self) -> Vec<usize> {
        self.columns
            .iter()
            .enumerate()
            .map(|(i, col)| {
                self.rows
                    .iter()
                    .map(|r| self.resolve(&r.cells[i]).width)
                    .fold(
                        col.min_width.max(util::display_width(&col.header)),
                        usize::max,
                    )
            })
            .collect()
    }

    fn write_line<'a>(
        &self,
        out: &mut String,
        cells: impl Iterator<Item = (&'a str, usize)>,
        widths: &[usize],
        suffix: Option<&str>,
    ) {
        let gap = " ".repeat(self.gap);
        let last = widths.len().saturating_sub(1);

        for (i, (text, width)) in cells.enumerate() {
            if i > 0 {
                out.push_str(&gap);
            }
            out.push_str(text);

            // The last column is only padded if something follows it.
            if i < last || suffix.is_some() {
                out.push_str(&" ".repeat(widths[i].saturating_sub(width)));
            }
        }

        if let Some(suffix) = suffix {
            out.push_str(&gap);
            out.push_str(suffix);
        }

        out.push('\n');
    }
}
