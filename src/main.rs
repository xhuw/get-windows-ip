use anyhow::{bail, Error};
use std::process::Command;
use std::str;

fn main() -> Result<(), Error> {
    let output = Command::new("ipconfig.exe").output()?;
    if !output.status.success() {
        bail!("ipconfig failed");
    }

    let output = str::from_utf8(&output.stdout)?
        .split("WSL")
        .nth(1)
        .ok_or(Error::msg("No WSL listing in ipconfig"))?;

    let output = output.split("IPv4 Address. . . . . . . . . . . : ")
        .nth(1)
        .ok_or(Error::msg("No IPv4 address found"))?;

    let ip = output.lines().nth(0).ok_or(Error::msg("No IPv4 address found"))?;

    println!("{}", ip);

    Ok(())
}
