pub fn collatz(input: i128) -> i128 {
    if input == 1 {
        0
    } else if input % 2 == 0 {
        collatz(input / 2) + 1
    } else {
        collatz(3 * input + 1) + 1
    }
}
