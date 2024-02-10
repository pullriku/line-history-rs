use line_history::history::History;

fn main() {
    let history = History::read_from_file("./history.txt").unwrap();
    let result = history.search_by_random();
    println!("{}", result);
}
