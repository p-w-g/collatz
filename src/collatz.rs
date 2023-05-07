pub fn collatz(input: i128) -> i32 {
    if input == 1 {
        0
    } else if input % 2 == 0 {
        collatz(input / 2)
    } else {
        collatz(3 * input + 1)
    }
}
