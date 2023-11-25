use std::{collections::HashMap, fmt::Display};

use chrono::{self, NaiveDate, Datelike};
use rand::Rng;
use regex::Regex;

const RE_DATE_S: &str = r"^20\d{2}\/\d{1,2}\/\d{1,2}\(.+\)\r?$";
const RE_TIME_S: &str = r"^(\d{2}):(\d{2}).*";
const YMD_PATTERN: &str = r"%Y/%m/%d";

pub struct LineHistory {
    history_data: Vec<String>,
    date_indices: HashMap<String, usize>,
    date_array: Vec<NaiveDate>,

    re_date: Regex,
    re_time: Regex,
}

pub struct LineContent {
    pub date: NaiveDate,
    pub line_count: usize,
    pub line: String,
}

impl Display for LineContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = self.date.with_zero_padding();
        write!(f, "{}/{}/{} {}", date.0, date.1, date.2, self.line)
    }
}

impl LineHistory {

    pub fn from_lines(lines: &[String]) -> Self {
        let _data = lines.to_vec();

        let _re_date = Regex::new(RE_DATE_S).unwrap();

        let (_indices, _date_array) = calc_date_indices(&_data, &_re_date);

        LineHistory {
            history_data: _data,
            date_indices: _indices,
            date_array: _date_array,
            re_date: Regex::new(RE_DATE_S).unwrap(),
            re_time: Regex::new(RE_TIME_S).unwrap(),
        }
    }

    pub fn new(data: String) -> Self {
        let _data = data
            .lines()
            .map(|line| line.to_owned())
            .collect::<Vec<_>>();

        let _re_date = Regex::new(RE_DATE_S).unwrap();

        let (_indices, _date_array) = calc_date_indices(&_data, &_re_date);

        LineHistory {
            history_data: _data,
            date_indices: _indices,
            date_array: _date_array,
            re_date: _re_date,
            re_time: Regex::new(RE_TIME_S).unwrap(),
        }
    }

    pub fn search_by_date(&self, date: &NaiveDate) -> Option<String> {
        let date_input = date;
        let mut result = String::new();

        let start_line_num = self.date_indices.get(date_input.format(YMD_PATTERN).to_string().as_str())?.to_owned();

        let default_date = NaiveDate::default();
        let next_date = self.date_array.get(
            self.date_array
                .binary_search(date_input).unwrap() + 1
        ).unwrap_or(&default_date);

        let default_index = self.history_data.len();
        let next_line_num = self.date_indices.get(next_date.format(YMD_PATTERN).to_string().as_str()).unwrap_or(&default_index).to_owned();

        for (_i, line) in self.history_data[start_line_num..next_line_num].iter().enumerate() {
            // result.push_str(&create_line_with_time(line, i, &date_input));
            result.push_str(&format!("{}\n", line));
        }

        result.push_str(
            // &format!("{}行<br>", next_line_num - start_line_num)
            &format!("{}行\n", next_line_num - start_line_num)
        );

        Option::from(result)
    }

    pub fn search_by_keyword(&self, keyword: &str) -> Vec<LineContent> {
        let _keyword = &keyword
            .replace('<', "&lt;")
            .replace('>', "&gt;");
        let re_keyword = Regex::new(_keyword).unwrap();

        let mut result = Vec::<LineContent>::new();
        let mut date = NaiveDate::default();
        let mut count_start: usize = 0;

        for (i, _line) in self.history_data.iter().enumerate() {
            let mut line = _line.to_owned();

            if self.re_date.is_match(&line) {
                let date_tmp = generate_date(&line[0..10]);
                if date_tmp >= date {
                    date = date_tmp;
                    count_start = i;
                }
            } else if re_keyword.find(&line).is_some() {
                if self.re_time.is_match(&line) {
                    line =  line[6..].to_owned();
                }
                let line_count = i - count_start;

                let data = LineContent {
                    date,
                    line_count,
                    line: line.to_owned(),
                };
                result.push(data);
            }
        }

        result
    }

    pub fn search_by_random(&self) -> String {
        let mut random = rand::thread_rng();
        let random_index = random.gen_range(0..self.date_array.len());

        let date = self.date_array[random_index];

        self.search_by_date(&date).unwrap()
    }
}

fn calc_date_indices(history_data: &[String], re_date: &Regex) -> (HashMap<String, usize>, Vec<NaiveDate>) {
    let init_capacity = history_data.len()/1000usize;
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
        .map(
        |elem| elem.parse::<u16>().unwrap_or_default()
        )
        .collect::<Vec<u16>>();

    if ymd.len() != 3 {
        return NaiveDate::default();
    }

    let parse_result = NaiveDate::from_ymd_opt(
        ymd[0] as i32, ymd[1] as u32, ymd[2] as u32
    );

    parse_result.unwrap_or_default()
}

pub fn zero_padding(string: &str, length: usize) -> String {
    let mut result = String::new();
    for _ in 0..(length - string.len()) {
        result.push('0');
    }
    result.push_str(string);

    result
}

pub struct YmdString(String, String, String);

trait ZeroPadString {
    fn with_zero_padding(&self) -> YmdString;
}
    

impl ZeroPadString for NaiveDate {
    fn with_zero_padding(&self) -> YmdString {
        let year = zero_padding(&self.year().to_string(), 4);
        let month = zero_padding(&self.month().to_string(), 2);
        let day = zero_padding(&self.day().to_string(), 2);

        YmdString(year, month, day)
    }
}

#[cfg(test)]
/// cargo test -- --nocapture
mod tests {
    use super::*;
    use std::fs;

    fn read() -> String {
        fs::read_to_string("./history.txt").unwrap()
    }

    #[test]
    fn search_by_date_test() {
        let text = read();
        let history = LineHistory::new(text);
        let result = history.search_by_date(
            &NaiveDate::from_ymd_opt(2222, 1, 1).unwrap(),
        );
        assert!(result.is_none());
    }

    #[test]
    fn search_test() {
        let text = read();
        let history = LineHistory::new(text);
        let result = history.search_by_keyword("hello");
        assert_eq!(result.len(), 40);
    }

    #[test]
    fn random_test() {
        let text = read();
        let history = LineHistory::new(text);
        let result = history.search_by_random();
        assert!(!result.is_empty());
    }
}