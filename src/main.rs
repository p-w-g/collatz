use std::io;

fn main() {
    let mut starting_number = String::new();

    println!("For which number you want to count steps?");
    io::stdin()
        .read_line(&mut starting_number)
        .expect("Failed to read the line");

    let starting_number: i128 = starting_number
        .trim()
        .parse()
        .expect("Please type a number!");

    let steps_taken = collatz(starting_number);

    println!("it took {steps_taken} to reach 1")
}

fn collatz(input: i128) -> i32 {
    if input == 1 {
        return 0;
    };
    if input % 2 == 0 {
        return collatz(input / 2);
    }

    return collatz(3 * input + 1);
}
