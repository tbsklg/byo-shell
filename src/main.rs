use pathsearch::find_executable_in_path;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let path = env!("PATH");

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
            Some("type") => match tail.as_str() {
                "echo" | "exit" | "type" => println!("{tail} is a shell builtin"),
                "ls" => println!("{path}"),
                _ => match find_executable_in_path(&tail) {
                    Some(exe) => {
                        println!("{tail} is {}", exe.display());
                    }
                    None => {
                        println!("{tail}: not found");
                    }
                },
            },
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
