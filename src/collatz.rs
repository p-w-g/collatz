pub fn collatz(input: i128) -> i32 {
    if input == 1 {
        return 0;
    };
    if input % 2 == 0 {
        return collatz(input / 2);
    }

    return collatz(3 * input + 1);
}
