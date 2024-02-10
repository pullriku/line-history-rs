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
            zero_padding(self.date.year().try_into().unwrap(), 4),
            zero_padding(self.date.month().try_into().unwrap(), 2),
            zero_padding(self.date.day().try_into().unwrap(), 2),
            self.line
        )
    }
}
