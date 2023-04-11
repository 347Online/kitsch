use diy_shell::{ShellResult, shell::Shell};

fn main() -> ShellResult {
    let mut shell = Shell::new();

    shell.run()
}
