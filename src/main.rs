use std::io;

mod collatz;

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

    let steps_taken = collatz::collatz(starting_number);

    println!("it took {steps_taken} to reach 1")
}
