use std::{collections::HashMap, path::Path, str::SplitWhitespace};

pub fn cd(args: SplitWhitespace) {
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);
    if let Err(e) = std::env::set_current_dir(root) {
        eprintln!("{e}");
    }
}

pub fn env() {
    
}
