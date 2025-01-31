#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut command = input.split_whitespace();

        let head = command.next();
        let tail = command.collect::<Vec<&str>>().join(" ");
        match head {
            Some("exit") => std::process::exit(0),
            Some("echo") => println!("{}", tail),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
