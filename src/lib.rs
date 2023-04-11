pub mod shell;
pub mod builtin;
pub mod error;

use std::{process::{Stdio, Child, Command}};
use error::ShellError;

pub type ShellResult = Result<(), ShellError>;