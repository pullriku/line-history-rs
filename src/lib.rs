use std::collections::HashMap;

use chrono::{self, NaiveDate, Datelike};
use rand::Rng;
use regex::Regex;

const RE_DATE_S: &str = r"^20\d{2}\/\d{1,2}\/\d{1,2}\(.+\)\r?$";
const RE_TIME_S: &str = r"^(\d{2}):(\d{2}).*";
const YMD_PATTERN: &str = r"%Y/%m/%d";

fn re_date() -> Regex {
    Regex::new(RE_DATE_S).unwrap()
}

fn re_time() -> Regex {
    Regex::new(RE_TIME_S).unwrap()
}

pub struct LineHistory {
    pub history_data: Vec<String>,
    pub date_indices: HashMap<String, usize>,
    pub date_array: Vec<NaiveDate>,
}

pub struct LineContent {
    pub date: NaiveDate,
    pub line_count: usize,
    pub line: String,
}

impl LineHistory {
    pub fn new(data: &str) -> Self {
        let _data = data
            .replace('\r', "")
            .split('\n')
            .map(String::from)
            .collect::<Vec<String>>();

        let (_indices, _date_array) = calc_date_indices(&_data);

        LineHistory {
            history_data: _data,
            date_indices: _indices,
            date_array: _date_array,
        }
    }

    pub fn search_by_date(&self, date: &NaiveDate) -> Option<String> {
        let date_input = date;
        let mut result = String::new();

        let start_line_num = self.date_indices.get(&date_input.format(YMD_PATTERN).to_string())?.to_owned();

        let default_date = NaiveDate::default();
        let next_date = self.date_array.get(
            self.date_array
                .binary_search(date_input).unwrap() + 1
        ).unwrap_or(&default_date);

        let default_index = self.history_data.len();
        let next_line_num = self.date_indices.get(&next_date.format(YMD_PATTERN).to_string()).unwrap_or(&default_index).to_owned();

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

        let re_date = re_date();
        let re_time = re_time();

        for (i, _line) in self.history_data.iter().enumerate() {
            let mut line = _line.to_owned();

            if re_date.is_match(&line) {
                let date_tmp = generate_date(&line[0..10]);
                if date_tmp >= date {
                    date = generate_date(&line[0..10]);
                    count_start = i;
                }
            } else if re_keyword.find(&line).is_some() {
                if re_time.is_match(&line) {
                    line =  line[6..].to_owned();
                }
                let line_count = i - count_start;

                let data = LineContent {
                    date,
                    line_count,
                    line,
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

fn calc_date_indices(history_data: &[String]) ->( HashMap<String, usize>, Vec<NaiveDate>) {
    let mut result = HashMap::<String, usize>::new();
    let mut date_array = Vec::<NaiveDate>::new();

    let mut current = NaiveDate::default();

    let re_date = re_date();
    
    for (i, line) in history_data.iter().enumerate() {
        if !re_date.is_match(line) {
            continue;
        }
        let date_tmp = generate_date(&line[0..10]);
        if date_tmp >= current {
            current = date_tmp;

            result.insert(current.format(YMD_PATTERN).to_string(), i);
            date_array.push(current);
        }
    }

    (result, date_array)
}

// fn create_line_with_time(line: &str, line_count: usize, date: &NaiveDate) -> String {
//     let mut line_info: Vec<&str> = line.split('\t').collect();
//     let new_info: String;
//     if line_info.len() >= 2 {
//         new_info = format!(
//             "<a href=\"javascript:showLineInfoAlert(\'{}\',{});\">{}</a>", 
//             date.format(YMD_PATTERN), 
//             line_count,
//             line_info[0]
//         );
//         line_info[0] = &new_info;
//     }

//     format!(
//         "<span id=\"{}\">{}</span><br>\n",
//         line_count,
//         line_info.join("\t"),
//     )
// }


fn generate_date(date_string: &str) -> NaiveDate {
    let parse_result = NaiveDate::parse_from_str(date_string, YMD_PATTERN);

    match parse_result {
        Ok(date) => date,
        Err(_) => NaiveDate::default(),
    }
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

    fn init() -> LineHistory {
        LineHistory::new(&read())
    }

    fn read() -> String {
        fs::read_to_string("./history.txt").unwrap()
    }

    #[test]
    fn search_by_date_test() {
        let history = init();
        let result = history.search_by_date(
            &NaiveDate::from_ymd_opt(2222, 1, 1).unwrap(),
        );
        assert!(result.is_none());
    }

    #[test]
    fn search_test() {
        let history = init();
        let result = history.search_by_keyword("hello");
        assert_eq!(result.len(), 40);
    }

    #[test]
    fn random_test() {
        let history = init();
        let result = history.search_by_random();
        assert!(!result.is_empty());
    }
}