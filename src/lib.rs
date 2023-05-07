mod collatz;
#[cfg(test)]
mod tests {
    // use std::result;

    use super::*;

    #[test]
    fn sanity_test() {
        let result: i8 = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn collatz_tests() {
        let case_1 = collatz::collatz(1);
        assert_eq!(case_1, 0);
    }
}
