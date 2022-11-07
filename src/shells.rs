use anyhow::{anyhow, bail, Ok, Result};
use std::ffi::OsString;

use crate::consts::NOT_UNICODE_ERR;

pub enum Shells {
    Bash,
    Zsh,
}

pub fn parse_shell_type(s: OsString) -> Result<Shells> {
    let v = s.to_str().ok_or_else(|| anyhow!(NOT_UNICODE_ERR))?;
    let result = match v {
        "bash" => Shells::Bash,
        "zsh" => Shells::Zsh,
        _ => bail!("Unsupported shell"),
    };

    Ok(result)
}
