pub mod builtin;
pub mod error;
pub mod shell;

use error::ShellError;
use std::process::{Child, Command, Stdio};

pub type ShellResult = Result<(), ShellError>;
