use chrono::{self, NaiveDate};
use rand::Rng;
use regex::Regex;
use std::{collections::BTreeMap, fs};

use crate::line_content::LineContent;

const RE_DATE_S: &str = r"^20\d{2}\/\d{1,2}\/\d{1,2}\(.+\)\r?$";
const RE_TIME_S: &str = r"^(\d{2}):(\d{2}).*";
// const YMD_PATTERN: &str = r"%Y/%m/%d";

pub struct History {
    history_data: Vec<String>,
    pub(crate) date_indices: BTreeMap<NaiveDate, usize>,
    // date_array: Vec<NaiveDate>,
    re_date: Regex,
    re_time: Regex,
}

impl History {
    /// Read text file and create `LineHistory` structure.
    ///  
    /// # Errors
    /// Error if file not found.
    pub fn read_from_file(path: &str) -> Result<Self, std::io::Error> {
        let data: String = fs::read_to_string(path)?.split('\n').skip(3).collect();
        Ok(Self::new(&data))
    }

    /// Create `LineHistory` structure from text.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn new(data: &str) -> Self {
        let re_date = Regex::new(RE_DATE_S).unwrap();

        let data = data
            .lines()
            .skip_while(|line| !re_date.is_match(line))
            .map(ToOwned::to_owned)
            .collect::<Vec<String>>();

        Self::from_lines(data)
    }

    /// Create `LineHistory` structure from lines.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn from_lines(mut lines: Vec<String>) -> Self {
        let re_date = Regex::new(RE_DATE_S).unwrap();

        lines = lines
            .into_iter()
            .skip_while(|line| !re_date.is_match(line))
            .collect();

        let indices = calc_date_indices(&lines, &re_date);

        History {
            history_data: lines,
            date_indices: indices,
            re_date: Regex::new(RE_DATE_S).unwrap(),
            re_time: Regex::new(RE_TIME_S).unwrap(),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.history_data.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.history_data.is_empty()
    }

    /// Search history by date.
    ///
    /// # Panics
    /// Panics if date not found.
    #[must_use]
    pub fn search_by_date(&self, date: &NaiveDate) -> Option<String> {
        let date_input = date;
        let mut result = String::new();

        let start_line_num = self.date_indices.get(date_input)?.to_owned();

        let next_date = self.date_indices.keys().find(|&&date| date > *date_input);

        let next_line_num = if let Some(next_date) = next_date {
            *self.date_indices.get(next_date)? - 1
        } else {
            self.history_data.len()
        };

        for line in &self.history_data[start_line_num..next_line_num] {
            // result.push_str(&create_line_with_time(line, i, &date_input));
            result.push_str(&format!("{line}\n"));
        }
        result.push('\n');

        result.push_str(&format!("{}行\n", next_line_num - start_line_num));

        Option::from(result)
    }

    /// Search history by keyword.
    ///
    /// # Panics
    /// Panics if keyword is not correct regex.
    ///
    #[must_use]
    pub fn search_by_keyword(&self, keyword: &str) -> Vec<LineContent> {
        let re_keyword = Regex::new(keyword).unwrap();

        let mut result = Vec::<LineContent>::new();
        let mut date = NaiveDate::default();
        let mut count_start: usize = 0;

        for (i, line) in self.history_data.iter().enumerate() {
            let mut line = line.to_owned();

            if self.re_date.is_match(&line) {
                let date_tmp = generate_date(&line[0..10]);
                if date_tmp >= date {
                    date = date_tmp;
                    count_start = i;
                }
            } else if re_keyword.find(&line).is_some() {
                if self.re_time.is_match(&line) {
                    line = line[6..].to_owned();
                }
                let line_count = i - count_start;

                let line_content = LineContent {
                    date,
                    line_count,
                    line: line.clone(),
                };
                result.push(line_content);
            }
        }

        result
    }

    /// Search history by random.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn search_by_random(&self) -> String {
        let mut random = rand::thread_rng();
        let random_index = random.gen_range(0..self.date_indices.len());

        let date = self.date_indices.keys().nth(random_index).unwrap();

        self.search_by_date(date).unwrap()
    }

    #[must_use]
    pub fn before(&self, date: &NaiveDate) -> Option<String> {
        let date = self.date_indices.keys().filter(|&&d| d >= *date).min()?;
        let index = self.date_indices.get(date)?;

        if *index == 0 {
            return None;
        }
        let index = index - 1;

        self.history_data
            .iter()
            .take(index)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>()
            .join("\n")
            .into()
    }

    #[must_use]
    pub fn between(&self, start_date: &NaiveDate, end_date: &NaiveDate) -> Option<String> {
        if start_date > end_date {
            return None;
        }

        let iter = self
            .date_indices
            .keys()
            .filter(|&&d| *start_date <= d && d < *end_date);
        let start = self.date_indices.get(iter.clone().min()?)?;

        let iter_max = iter.max()?;
        let next_date = self.date_indices.keys().find(|&&date| date > *iter_max);
        let end = if let Some(next_date) = next_date {
            *self.date_indices.get(next_date)? - 1
        } else {
            self.history_data.len()
        };

        self.history_data
            .iter()
            .skip(*start)
            .take(end - start)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>()
            .join("\n")
            .into()
    }

    #[must_use]
    pub fn after(&self, date: &NaiveDate) -> Option<String> {
        let date = self.date_indices.keys().filter(|&&d| d >= *date).min()?;
        let index = self.date_indices.get(date)?;

        self.history_data
            .iter()
            .skip(*index)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>()
            .join("\n")
            .into()
    }
}

fn calc_date_indices(history_data: &[String], re_date: &Regex) -> BTreeMap<NaiveDate, usize> {
    let mut current = NaiveDate::default();

    history_data
        .iter()
        .enumerate()
        .filter(|(_i, line)| re_date.is_match(line))
        // check increasing
        .filter(|(_i, line)| {
            let date_tmp = generate_date(&line[0..10]);
            if date_tmp > current {
                current = date_tmp;
                true
            } else {
                false
            }
        })
        .map(|(i, line)| (generate_date(&line[0..10]), i))
        .collect()
}

fn generate_date(date_string: &str) -> NaiveDate {
    let ymd = date_string
        .split('/')
        .map(|elem| elem.parse::<u16>().unwrap_or_default())
        .collect::<Vec<u16>>();

    if ymd.len() != 3 {
        return NaiveDate::default();
    }

    let parse_result =
        NaiveDate::from_ymd_opt(i32::from(ymd[0]), u32::from(ymd[1]), u32::from(ymd[2]));

    parse_result.unwrap_or_default()
}

#[cfg(test)]
/// cargo test -- --nocapture
mod tests {
    use super::*;

    const CONTENT: &str = "[LINE]MyGroupのトーク履歴
保存日時：2024/01/01 00:00

2020/02/29(土)
13:00\tC\t衛生面に気をつけよう
13:00\tA\tおはよう
13:01\tB\tOK

2023/07/21(金)
01:00\tD\t夏だね
01:01\tA\t\"過去の会話でも見ようか
2017/01/01(日)
00:00\tA\tあけおめ
000:01\tB\tおめでとう

2020/02/29(土)
13:00\tC\t衛生面に気をつけよう
13:00\tA\tおはよう
13:01\tB\tOK\"
01:02\tB\tUnit Test はクリアできたかな？

2023/08/01(火)
06:00\tA\t夏だね
06:11\tD\tおはよう
";

    fn read() -> String {
        CONTENT.to_string()
    }

    #[test]
    fn search_by_date_test() {
        let text = read();
        let history = History::new(&text);
        let result = history.search_by_date(&NaiveDate::from_ymd_opt(2020, 2, 29).unwrap());
        assert_eq!(result.unwrap(), "2020/02/29(土)\n13:00\tC\t衛生面に気をつけよう\n13:00\tA\tおはよう\n13:01\tB\tOK\n\n4行\n");

        let result = history.search_by_date(&NaiveDate::from_ymd_opt(2023, 7, 21).unwrap());
        assert_eq!(
            result.unwrap(),
            "2023/07/21(金)
01:00\tD\t夏だね
01:01\tA\t\"過去の会話でも見ようか
2017/01/01(日)
00:00\tA\tあけおめ
000:01\tB\tおめでとう

2020/02/29(土)
13:00\tC\t衛生面に気をつけよう
13:00\tA\tおはよう
13:01\tB\tOK\"
01:02\tB\tUnit Test はクリアできたかな？

12行
"
        );

        let result = history.search_by_date(&NaiveDate::from_ymd_opt(2023, 8, 1).unwrap());
        assert_eq!(
            result.unwrap(),
            "2023/08/01(火)\n06:00\tA\t夏だね\n06:11\tD\tおはよう\n\n3行\n"
        );
    }

    #[test]
    fn search_test() {
        let text = read();
        let history = History::new(&text);
        let result = history.search_by_keyword("OK");
        assert_eq!(result.first().unwrap().line, "B\tOK");
        assert_eq!(
            result.first().unwrap().date,
            NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()
        );

        let result = history.search_by_keyword("あけおめ");
        assert_eq!(result.first().unwrap().line, "A\tあけおめ");
        assert_eq!(
            result.first().unwrap().date,
            NaiveDate::from_ymd_opt(2023, 7, 21).unwrap()
        );

        let result = history.search_by_keyword("よう");
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn random_test() {
        let text = read();
        let history = History::new(&text);
        let result = history.search_by_random();
        assert!(!result.is_empty());
    }

    #[test]
    fn before_test() {
        let text = read();
        let history = History::new(&text);
        let result = history.before(&NaiveDate::from_ymd_opt(2020, 2, 29).unwrap());
        assert!(result.is_none());
    }

    #[test]
    fn between_test() {
        let text = read();
        let history = History::new(&text);
        let result = history
            .between(
                &NaiveDate::from_ymd_opt(2020, 2, 29).unwrap(),
                &NaiveDate::from_ymd_opt(2023, 7, 21).unwrap(),
            )
            .unwrap();
        assert_eq!(result.lines().count(), 4);

        let result = history
            .between(
                &NaiveDate::from_ymd_opt(2020, 2, 29).unwrap(),
                &NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
            )
            .unwrap();
        assert_eq!(result.lines().count(), 17);

        let result = history.between(&NaiveDate::MIN, &NaiveDate::MAX).unwrap();
        assert_eq!(result.lines().count(), 21);
    }

    #[test]
    fn after_test() {
        let text = read();
        let history = History::new(&text);
        let result = history
            .after(&NaiveDate::from_ymd_opt(2023, 7, 21).unwrap())
            .unwrap();
        assert_eq!(result.lines().count(), 16);
    }
}
