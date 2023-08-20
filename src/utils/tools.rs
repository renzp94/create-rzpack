use std::io::{Error, Result};
use std::process::{Command, Output};

pub fn run_command(command: &str, args: &[&str]) -> Result<Output> {
    let output = Command::new(command).args(args).output()?;

    if output.status.success() {
        Ok(output)
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Command execution failed",
        ))
    }
}
