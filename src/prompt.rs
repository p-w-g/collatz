use std::io;
pub fn prompt_number() -> i128 {
    let mut users_input = String::new();

    io::stdin()
        .read_line(&mut users_input)
        .expect("Failed to read the line");

    let users_input: i128 = users_input.trim().parse().expect("Please type a number!");

    users_input
}
