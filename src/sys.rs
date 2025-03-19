//! Perform system actions

use anyhow::Result;
use std::process::{Command, exit};

const POWEROFF_PATH: &str = "/sbin/poweroff";
const REBOOT_PATH: &str = "/sbin/reboot";

fn run_prog(prog_path: &str) -> Result<i32> {
    let code = Command::new(prog_path).status()?.code();

    Ok(code.unwrap_or(0))
}

pub fn reboot() -> Result<i32> {
    run_prog(REBOOT_PATH)
}

pub fn poweroff() -> Result<i32> {
    run_prog(POWEROFF_PATH)
}

pub fn exit_prog() -> ! {
    exit(0)
}
