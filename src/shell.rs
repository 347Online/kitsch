use std::{
    io::Write,
    process::{Child, Command, Stdio},
};

use crate::{builtin, ShellResult};

pub struct Shell {
    previous_command: Option<Child>,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            previous_command: None,
        }
    }

    pub fn run(&mut self) -> ShellResult {
        loop {
            print!("> ");
            std::io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read stdin");

            let mut commands = input.trim().split(" | ").peekable();
            self.previous_command = None;

            while let Some(command) = commands.next() {
                let mut parts = command.split_whitespace();
                let command = if let Some(command) = parts.next() {
                    command
                } else {
                    ""
                };
                let vars = std::env::vars();
                let mut vars_string = String::new();
                for (a, b) in vars {
                    if !vars_string.is_empty() {
                        vars_string += "\n";
                    }
                    vars_string += &format!("{}: {}", a, b);
                }

                let args = parts;

                match command {
                    "" => (),
                    "exit" => return Ok(()),

                    "cd" => {
                        builtin::cd(args);
                        self.previous_command = None;
                    }

                    command => {
                        let stdin = match self.previous_command.take() {
                            Some(output) => {
                                Stdio::from(output.stdout.expect("Failed to grab from stdout"))
                            }
                            None => Stdio::inherit(),
                        };

                        let stdout = if commands.peek().is_some() {
                            Stdio::piped()
                        } else {
                            Stdio::inherit()
                        };

                        let output = Command::new(command)
                            .args(args)
                            .stdin(stdin)
                            .stdout(stdout)
                            .spawn();

                        match output {
                            Ok(output) => self.previous_command = Some(output),
                            Err(e) => {
                                self.previous_command = None;
                                eprintln!("{e}");
                            }
                        }
                    }
                }
            }

            if let Some(ref mut final_command) = self.previous_command {
                final_command.wait();
            }
        }
    }
}
