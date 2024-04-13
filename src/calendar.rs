use chrono::{Datelike, NaiveDate};
use text_calendar::YearCalendar;

use crate::history::History;

#[allow(clippy::module_name_repetitions)]
pub use text_calendar::{BasicMarker, Calendar, Marker, MonthCalendar};

impl History {
    /// Create month calendar.
    #[must_use]
    pub fn create_month_calendar(&self, year: i32, month: u32) -> Option<MonthCalendar> {
        self.create_month_calendar_with_marker(year, month, BasicMarker::SquareBrackets)
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    /// Create month calendar with marker.
    pub fn create_month_calendar_with_marker(
        &self,
        year: i32,
        month: u32,
        marker: impl Marker + 'static,
    ) -> Option<MonthCalendar> {
        let mut calendar = MonthCalendar::new(year, month, chrono::Weekday::Sun, 4, marker)?;

        for key in self.date_indices.keys() {
            if key.year() == year && key.month() == month {
                calendar.mark(NaiveDate::from_ymd_opt(key.year(), key.month(), key.day()).unwrap());
            }
        }

        Some(calendar)
    }

    #[must_use]
    pub fn create_year_calendar(&self, year: i32) -> Option<YearCalendar> {
        self.create_year_calendar_with_marker(year, BasicMarker::SquareBrackets)
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn create_year_calendar_with_marker(
        &self,
        year: i32,
        marker: impl Marker + Clone + 'static,
    ) -> Option<YearCalendar> {
        let mut calendar = YearCalendar::new(year, chrono::Weekday::Sun, 4, marker);

        self.date_indices
            .keys()
            .filter(|k| k.year() == year)
            .for_each(|key| {
                calendar.mark(NaiveDate::from_ymd_opt(key.year(), key.month(), key.day()).unwrap());
            });

        Some(calendar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "[LINE]MyGroupのトーク履歴
保存日時：2024/01/01 00:00

2024/02/01(木)
00:00\tA\tおはよう

2024/02/11(日)
00:00\tA\tおはよう

2024/02/15(木)
00:00\tA\tおはよう

2024/02/26(月)
00:00\tA\tおはよう

2024/02/29(木)
23:59\tA\t\"おやすみ
2024/02/01(木)
00:00\tA\tおはよう\"
";

    #[test]
    fn cal_test() {
        let history = History::new(CONTENT);
        let calendar = history.create_month_calendar(2024, 2).unwrap();
        let expected = "          February          
 Su  Mo  Tu  We  Th  Fr  Sa 
                [1 ] 2   3  
 4   5   6   7   8   9   10 
[11] 12  13  14 [15] 16  17 
 18  19  20  21  22  23  24 
 25 [26] 27  28 [29]        ";
        // println!("{calendar}");
        assert_eq!(calendar.to_string(), expected);
    }
}
