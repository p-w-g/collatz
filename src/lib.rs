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
        let case_2 = collatz::collatz(2);
        let case_3 = collatz::collatz(3);
        let case_4 = collatz::collatz(4);
        let case_5 = collatz::collatz(5);
        let case_6 = collatz::collatz(6);
        let case_7 = collatz::collatz(7);
        let case_8 = collatz::collatz(8);
        let case_15 = collatz::collatz(15);
        let case_27 = collatz::collatz(27);
        let case_50 = collatz::collatz(50);

        assert_eq!(case_1, 0);
        assert_eq!(case_2, 1);
        assert_eq!(case_3, 7);
        assert_eq!(case_4, 2);
        assert_eq!(case_5, 5);
        assert_eq!(case_6, 8);
        assert_eq!(case_7, 16);
        assert_eq!(case_8, 3);
        assert_eq!(case_15, 17);
        assert_eq!(case_27, 111);
        assert_eq!(case_50, 24);
    }
}
