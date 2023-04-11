use std::{io::Write, path::Path, str::SplitWhitespace};

pub fn cd(args: SplitWhitespace) {
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);
    if let Err(e) = std::env::set_current_dir(root) {
        eprintln!("{e}");
    }
}

pub fn prompt() {
    let cwd = std::env::current_dir().expect("Bad Working Directory");
    let basename = cwd.file_name().and_then(|s| s.to_str()).expect("Bad Basename");
    print!("User@System {}> ", basename);
    std::io::stdout().flush().expect("Failed to flush stdout");
}

pub fn env() {}
