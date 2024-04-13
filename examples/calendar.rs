use line_history::history::History;

fn main() {
    let history = History::read_from_file("./history.txt").unwrap();
    let calendar = history.create_year_calendar(2019).unwrap();

    println!("{calendar}");
}
