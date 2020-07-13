use anyhow::{bail, Result};
use std::process::{Command, Stdio};
use std::str;
use which::which;

/// Check if `fzf` is found in the user's PATH
pub fn is_fzf_installed() -> bool {
    which("fzf").is_ok()
}

/// Find a configuration to activate using `fzf` for fuzzy searching
pub fn fuzzy_find_config() -> Result<String> {
    let child = Command::new("fzf")
        .arg("--ansi")
        .arg("--no-preview")
        .env(
            "FZF_DEFAULT_COMMAND",
            format!("{} list", std::env::current_exe().unwrap().display()),
        )
        .stdout(Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;
    let choice = str::from_utf8(&output.stdout)?
        .trim_start_matches('*')
        .trim()
        .to_owned();

    if choice.is_empty() {
        bail!("No configuration selected")
    } else {
        Ok(choice)
    }
}
