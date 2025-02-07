#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use anyhow::{Context, Error};
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command as ProcessCommand};

#[derive(Eq, PartialEq, Debug)]
enum Command {
    Exit,
    Echo,
    Type,
    Pwd,
    Ls,
    Cd,
    Program(String),
}

impl From<&str> for Command {
    fn from(cmd: &str) -> Self {
        match cmd {
            "exit" => Self::Exit,
            "echo" => Self::Echo,
            "type" => Self::Type,
            "pwd" => Self::Pwd,
            "ls" => Self::Ls,
            "cd" => Self::Cd,
            other => Self::Program(other.to_string()),
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit => write!(f, "exit"),
            Self::Echo => write!(f, "echo"),
            Self::Type => write!(f, "type"),
            Self::Pwd => write!(f, "pwd"),
            Self::Ls => write!(f, "ls"),
            Self::Cd => write!(f, "cd"),
            Self::Program(name) => write!(f, "{name}"),
        }
    }
}

struct Shell {
    builtins: Vec<Command>,
    paths: Vec<String>,
}

impl From<&str> for Shell {
    fn from(path: &str) -> Self {
        Self {
            builtins: vec![
                Command::Echo,
                Command::Exit,
                Command::Pwd,
                Command::Cd,
                Command::Type,
            ],
            paths: path.split(':').map(String::from).collect(),
        }
    }
}

impl Shell {
    fn execute(&self, cmd: &Command, args: &[String]) -> Result<(), Error> {
        match cmd {
            Command::Exit => process::exit(0),
            Command::Ls => {
                println!("{}", self.paths.join(":"));
            }
            Command::Echo => {
                println!("{}", args.join(" "));
            }
            Command::Pwd => {
                println!("{}", env::current_dir()?.display());
            }
            Command::Type => {
                if args.is_empty() {
                    eprintln!("type: missing argument");
                    return Ok(());
                }
                let arg_cmd = Command::from(args[0].as_str());
                if self.builtins.contains(&arg_cmd) {
                    println!("{arg_cmd} is a shell builtin");
                } else if let Some(entry) = self.exec_in_path(&args[0]) {
                    println!("{arg_cmd} is {}", entry.display());
                } else {
                    println!("{arg_cmd}: not found");
                }
            }
            Command::Cd => {
                if std::env::set_current_dir(Path::new(args[0].as_str())).is_err() {
                    println!("cd: {}: No such file or directory", args[0]);
                }
            }
            Command::Program(prog) => {
                if let Some(_path) = self.exec_in_path(prog) {
                    if let Err(e) = ProcessCommand::new(prog).args(args).status() {
                        eprintln!("Failed to execute {prog}: {e}");
                    }
                } else {
                    println!("{prog}: command not found");
                }
            }
        }
        Ok(())
    }

    fn exec_in_path(&self, prog: &str) -> Option<PathBuf> {
        self.paths.iter().find_map(|path| {
            let entry = Path::new(path).join(prog);
            if entry.exists() && entry.is_file() {
                Some(entry)
            } else {
                None
            }
        })
    }
}

fn main() -> Result<(), Error> {
    let path = env::var("PATH").context("PATH variable not set")?;
    let stdin = io::stdin();
    let shell = Shell::from(path.as_str());

    loop {
        print!("$ ");
        io::stdout().flush().context("Failed to flush stdout")?;

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        let cmds: Vec<&str> = input.split_whitespace().collect();
        if cmds.is_empty() {
            continue;
        }
        let args: Vec<String> = cmds[1..].iter().map(ToString::to_string).collect();
        if let Err(e) = shell.execute(&Command::from(cmds[0]), &args) {
            eprintln!("Execution error: {e}");
        }
    }
}
