use anyhow::{bail, Error};
use std::process::Command;
use std::str;

fn main() -> Result<(), Error> {
    // execute iponfig
    let output = Command::new("ipconfig.exe").output()?;
    if !output.status.success() {
        bail!("ipconfig failed");
    }

    // find the section with WSL in the name
    let output = str::from_utf8(&output.stdout)?
        .split("WSL")
        .nth(1)
        .ok_or_else(|| Error::msg("No WSL listing in ipconfig"))?;

    // split on IPv4 address so we get a string starting with what we want
    let output = output
        .split("IPv4 Address. . . . . . . . . . . : ")
        .nth(1)
        .ok_or_else(|| Error::msg("No IPv4 address found"))?;

    // first line should just include the IP address
    let ip = output
        .lines()
        .next()
        .ok_or_else(|| Error::msg("No IPv4 address found"))?;

    // print it
    println!("{}", ip);

    Ok(())
}
