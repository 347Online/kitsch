use diy_shell::{shell::Shell, ShellResult};

fn main() -> ShellResult {
    let mut shell = Shell::new();

    shell.run()
}
