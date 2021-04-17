use std::io;
use std::process::Command;
use std::io::Write;

fn main() {
    print!("> ");
    let mut input = String::new();
    io::stdout().flush().expect("Unabel to flush.");

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read line.");

    let command = input.trim();

    Command::new(command)
        .spawn()
        .expect("Invalid command.");

    let mut child = Command::new(command)
        .spawn()
        .expect("Command invalid.");

    child.wait().expect("Process not waited on.");
}
