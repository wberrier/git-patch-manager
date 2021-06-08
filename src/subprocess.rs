/// Make executing processes simpler in rust
/// sort of modelling after the "subprocess" module in python

/// Maybe https://crates.io/crates/execute does everything I need?
use std::process::ExitStatus;

use std::io::Result;

pub struct ShellProcess {}

impl ShellProcess {
    pub fn new_process(command: &str) -> std::process::Command {
        let mut process = std::process::Command::new("/bin/sh");
        process.arg("-c").arg(command);
        process
    }

    // TODO: better way to model capturing stderr?
    // std::process::Command doesn't seem to model capturing both at the same time?
    pub fn new_stderr_stdout_combined(command: &str) -> std::process::Command {
        let mut process = std::process::Command::new("/bin/sh");
        process.arg("-c").arg(format!("{} 2>&1", command));
        process
    }
}

// Print output as it happens
pub fn getstatus(command: &str) -> Result<ExitStatus> {
    ShellProcess::new_process(command).status()
}

// Just get output, don't care about exit code
pub fn getoutput(command: &str) -> Result<String> {
    let output = ShellProcess::new_stderr_stdout_combined(command).output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

// Get both
pub fn getstatusoutput(command: &str) -> Result<(ExitStatus, String)> {
    let output = ShellProcess::new_stderr_stdout_combined(command).output()?;

    Ok((
        output.status,
        String::from_utf8_lossy(&output.stdout).to_string(),
    ))
}
