use line_history::LineHistory;

fn main() {
    let history = LineHistory::read_from_file("./history.txt").unwrap();
    let result = history.search_by_keyword("a");
    for elem in result {
        println!("{}", elem);
    }
}
