use chrono::{self, NaiveDate};
use rand::Rng;
use regex::Regex;
use std::{collections::HashMap, fs};

use crate::line_content::LineContent;

const RE_DATE_S: &str = r"^20\d{2}\/\d{1,2}\/\d{1,2}\(.+\)\r?$";
const RE_TIME_S: &str = r"^(\d{2}):(\d{2}).*";
const YMD_PATTERN: &str = r"%Y/%m/%d";

pub struct History {
    history_data: Vec<String>,
    date_indices: HashMap<String, usize>,
    date_array: Vec<NaiveDate>,

    re_date: Regex,
    re_time: Regex,
}

impl History {
    /// Read text file and create `LineHistory` structure.
    ///  
    /// # Errors
    /// Error if file not found.
    pub fn read_from_file(path: &str) -> Result<Self, std::io::Error> {
        let data = fs::read_to_string(path)?;
        Ok(Self::new(&data))
    }

    /// Create `LineHistory` structure from lines.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn from_lines(lines: &[String]) -> Self {
        let data = lines.to_vec();

        let re_date = Regex::new(RE_DATE_S).unwrap();

        let (indices, date_array) = calc_date_indices(&data, &re_date);

        History {
            history_data: data,
            date_indices: indices,
            date_array,
            re_date: Regex::new(RE_DATE_S).unwrap(),
            re_time: Regex::new(RE_TIME_S).unwrap(),
        }
    }

    /// Create `LineHistory` structure from text.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn new(data: &str) -> Self {
        let data = data.lines().map(ToOwned::to_owned).collect::<Vec<_>>();

        let re_date = Regex::new(RE_DATE_S).unwrap();

        let (indices, date_array) = calc_date_indices(&data, &re_date);

        History {
            history_data: data,
            date_indices: indices,
            date_array,
            re_date,
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

        let start_line_num = self
            .date_indices
            .get(date_input.format(YMD_PATTERN).to_string().as_str())?
            .to_owned();

        let default_date = NaiveDate::default();
        let next_date = self
            .date_array
            .get(self.date_array.binary_search(date_input).unwrap() + 1)
            .unwrap_or(&default_date);

        let default_index = self.history_data.len();
        let mut next_line_num = self
            .date_indices
            .get(next_date.format(YMD_PATTERN).to_string().as_str())
            .unwrap_or(&default_index)
            .to_owned();

        if next_line_num != default_index {
            next_line_num -= 1;
        }

        for line in &self.history_data[start_line_num..next_line_num] {
            // result.push_str(&create_line_with_time(line, i, &date_input));
            result.push_str(&format!("{line}\n"));
        }
        result.push('\n');

        result.push_str(
            // &format!("{}行<br>", next_line_num - start_line_num)
            &format!("{}行\n", next_line_num - start_line_num),
        );

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
    ///
    /// # Panics
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn search_by_random(&self) -> String {
        let mut random = rand::thread_rng();
        let random_index = random.gen_range(0..self.date_array.len());

        let date = self.date_array[random_index];

        self.search_by_date(&date).unwrap()
    }
}

fn calc_date_indices(
    history_data: &[String],
    re_date: &Regex,
) -> (HashMap<String, usize>, Vec<NaiveDate>) {
    let init_capacity = history_data.len() / 1000usize;
    let mut result = HashMap::<String, usize>::with_capacity(init_capacity);
    let mut date_array = Vec::<NaiveDate>::with_capacity(init_capacity);
    // let mut result = HashMap::<String, usize>::new();
    // let mut date_array = Vec::<NaiveDate>::new();

    let mut current = NaiveDate::default();

    for (i, line) in history_data.iter().enumerate() {
        if !re_date.is_match(line) {
            continue;
        }
        let date_tmp = generate_date(&line[0..10]);
        if date_tmp >= current {
            current = date_tmp;

            result.insert(line[0..10].to_owned(), i);
            date_array.push(current);
        }
    }

    (result, date_array)
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
    use std::fs;

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
        assert_eq!(result.unwrap(), "2023/07/21(金)
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
");

        let result = history.search_by_date(&NaiveDate::from_ymd_opt(2023, 8, 1).unwrap());
        assert_eq!(result.unwrap(), "2023/08/01(火)\n06:00\tA\t夏だね\n06:11\tD\tおはよう\n\n3行\n");
    }

    #[test]
    fn search_test() {
        let text = read();
        let history = History::new(&text);
        let result = history.search_by_keyword("OK");
        assert_eq!(result.first().unwrap().line, "B\tOK");
        assert_eq!(result.first().unwrap().date, NaiveDate::from_ymd_opt(2020, 2, 29).unwrap());

        let result = history.search_by_keyword("あけおめ");
        assert_eq!(result.first().unwrap().line, "A\tあけおめ");
        assert_eq!(result.first().unwrap().date, NaiveDate::from_ymd_opt(2023, 7, 21).unwrap());

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
}
