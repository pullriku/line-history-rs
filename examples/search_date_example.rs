use chrono::NaiveDate;
use line_history::{history::ignore_errors, read_from_file};

fn main() {
    read_from_file!("./history.txt", let src, let history);
    let history = ignore_errors(history);

    let result = history
        .search_by_date(&NaiveDate::from_ymd_opt(2024, 12, 20).unwrap())
        .unwrap();

    for chat in result.chats() {
        println!(
            "{}({}): {}",
            chat.sender().unwrap_or_default(),
            chat.time(),
            chat.message_lines().join("\n")
        );
    }
}
