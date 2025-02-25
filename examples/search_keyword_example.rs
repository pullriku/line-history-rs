use line_history::{history::ignore_errors, read_from_file};

fn main() {
    read_from_file!("./history.txt", let src, let history);
    let history = ignore_errors(history);
    let result = history.search_by_keyword("本");
    println!("{:?}", result.collect::<Vec<_>>());
}
