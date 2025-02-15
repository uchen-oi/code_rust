///# Examples
/// ```
///let result = adder::add_two(2);
///assert_eq!(result, 4);
/// ```
pub fn add_two(a: i32) -> i32 {
    return a + 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(add_two(2), 4);
    }
    #[test]
    #[ignore]
    fn add_two_and_three() {
        assert_eq!(add_two(3), 5);
    }
    #[test]
    fn add_two_and_onehundred() {
        assert_eq!(add_two(100), 102);
    }
}
