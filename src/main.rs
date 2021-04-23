use std::io::{Write, stdin, stdout};

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        let mut input = String::new();
        
        // This is required to show output from the `print!` macro. If this isn't
        // included this output will be shown during termination.
        stdout().flush().expect("Unable to flush.");

        stdin()
            .read_line(&mut input)
            .expect("Unable to read line.");

        // This creates a struct called SplitWhitespace
        let mut main_command = input.trim().split_whitespace();

        // A weird property of `next` is that on the first call it returns the
        // first element in a collection.
        let command = main_command.next().expect("End of iterator.");

        // `next` also "removes" the viewed element from the collection,
        // so all that's left is everything after the viewed element
        let args = main_command;

        // Some commands have to chnage the interals of a shell
        // And therefore must be implemented manually. An example is the
        // cd (change directory) command
        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => return,
            command => {
                let mut child = Command::new(command)
                    .args(args)
                    .spawn()
                    .expect("Command invalid.");

                child.wait().expect("Process not waited on.");
            }
        };
    }
}
