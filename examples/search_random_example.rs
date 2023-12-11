use line_history::LineHistory;

fn main() {
    let history = LineHistory::read_from_file("./history.txt").unwrap();
    let result = history.search_by_random();
    println!("{}", result);
}
