/// Create zero padding string.
///
/// # Examples
/// ```rust
/// use line_history::processing::zero_padding;
///
/// let result = zero_padding(3, 3);
/// assert_eq!(result, "003");
/// ```
#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub fn zero_padding(number: usize, length: u8) -> String {
    let mut result = String::new();
    let string = number.to_string();

    for _ in 0..(length as isize - string.len() as isize) {
        result.push('0');
    }
    result.push_str(&string);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_padding_test() {
        let result = zero_padding(1, 2);
        assert_eq!(result, "01");

        let result = zero_padding(100_000, 2);
        assert_eq!(result, "100000");
    }
}
