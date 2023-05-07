mod collatz;
mod prompt;
fn main() {
    println!("For which number you want to count steps?");

    let starting_number = prompt::prompt_number();
    let steps_taken = collatz::collatz(starting_number);

    println!("it took {steps_taken} to reach 1")
}
