use std::fmt::Display;

use chrono::{Datelike, NaiveDate};

use crate::processing::zero_padding;

pub struct LineContent {
    pub date: NaiveDate,
    pub line_count: usize,
    pub line: String,
}

impl Display for LineContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{} {}",
            zero_padding(&self.date.year().to_string(), 4),
            zero_padding(&self.date.month().to_string(), 2),
            zero_padding(&self.date.day().to_string(), 2),
            self.line
        )
    }
}
