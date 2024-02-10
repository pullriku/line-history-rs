#[must_use]
pub fn zero_padding(string: &str, length: usize) -> String {
    let mut result = String::new();
    for _ in 0..(length - string.len()) {
        result.push('0');
    }
    result.push_str(string);

    result
}
