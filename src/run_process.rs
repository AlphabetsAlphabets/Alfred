use std::env;
use std::str::SplitWhitespace;
use std::path::Path;
use std::process::Command;

pub fn run_command(command: &str, args: SplitWhitespace) {
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
