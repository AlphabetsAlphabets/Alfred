use std::io::{Write, stdin, stdout};

mod run_process;
mod config;

fn main() {
    loop {
        config::read_config();
        print!("> ");

        let input = user_input();

        process_user_input(input);
    }
}

fn user_input() -> String {
    let mut input = String::new();
    
    // This is required to show output from the `print!` macro. If this isn't
    // included this output will be shown during termination.
    stdout().flush().expect("Unable to flush.");

    stdin()
        .read_line(&mut input)
        .expect("Unable to read line.");

    input
}

fn process_user_input(input: String) {
    let mut main_command = input.trim().split_whitespace();
    let command = main_command.next().expect("End of iterator.");
    let args = main_command;

    run_process::run_command(command, args);
}
