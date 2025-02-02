use pathsearch::find_executable_in_path;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

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

        let tail = command.collect::<Vec<&str>>();
        let args = tail.join(" ");
        match head {
            Some("exit") => std::process::exit(0),
            Some("echo") => println!("{}", tail.join(" ")),
            Some("type") => match args.as_str() {
                "echo" | "exit" | "type" => println!("{args} is a shell builtin"),
                "ls" => println!("{path}"),
                _ => match find_executable_in_path(&args) {
                    Some(exe) => {
                        println!("{args} is {}", exe.display());
                    }
                    None => {
                        println!("{args}: not found");
                    }
                },
            },
            Some(_) => match find_executable_in_path(&args) {
                Some(path) => {
                    Command::new(path)
                        .args(tail)
                        .status()
                        .expect("failed to execute process");
                }
                None => todo!(),
            },
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
