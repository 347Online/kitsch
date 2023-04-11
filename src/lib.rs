pub mod builtin;
pub mod error;
pub mod shell;

use error::ShellError;

pub type ShellResult = Result<(), ShellError>;
