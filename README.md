# line-history

## Description

search for specific content in LINE chat log files saved in `.txt` format.

## Example

Reads a LINE chat history file (`history.txt`), filters out errors, and searches for messages from a specific date (December 20, 2024). It then prints the messages along with the sender's name and timestamp.

```rust
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
```
