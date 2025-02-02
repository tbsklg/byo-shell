use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    let stdin = io::stdin();
    let path = env!("PATH");
    let paths = path.split(":").collect::<Vec<_>>();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let cmds = input.split_whitespace().collect::<Vec<_>>();

        let cmd = cmds[0];
        let args = &cmds[1..];

        match cmd {
            "exit" => std::process::exit(0),
            "echo" => println!("{}", args.join(" ")),
            "type" => match args.join(" ").as_str() {
                "echo" | "exit" | "type" => println!("{} is a shell builtin", args.join(" ")),
                "ls" => println!("{path}"),
                _ => match exec_in_path(&paths, cmd) {
                    Some(entry) => println!("{} is {}", args[0], entry.display()),
                    None => println!("{}: not found", args[0]),
                },
            },
            other => match exec_in_path(&paths, cmd) {
                Some(path) => {
                    let proc = std::process::Command::new(path)
                        .args(args)
                        .output()
                        .unwrap();
                    io::stdout().write_all(&proc.stdout).unwrap();
                }
                None => {
                    println!("{other}: command not found");
                    break;
                }
            },
        }
    }

    fn exec_in_path(paths: &Vec<&str>, exec_name: &str) -> Option<PathBuf> {
        for path in paths.iter() {
            let entry = PathBuf::from(path).join(exec_name);
            if entry.exists() {
                return Some(entry);
            }
        }
        None
    }
}
