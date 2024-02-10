use chrono::NaiveDate;
use line_history::history::History;

fn main() {
    let history = History::read_from_file("./history.txt").unwrap();
    let result = history.search_by_date(&NaiveDate::from_ymd_opt(2022, 1, 1).unwrap());
    println!("{}", result.unwrap());
}
