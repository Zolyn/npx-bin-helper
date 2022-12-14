use anyhow::{anyhow, bail, Ok, Result};
use std::ffi::OsString;

use crate::consts::NOT_UNICODE_ERR;

pub use self::{bash::Bash, fish::Fish, powershell::PowerShell, zsh::Zsh};

mod bash;
mod fish;
mod powershell;
mod zsh;

pub trait Shell {
    fn name(&self) -> &'static str;
    fn env_separator(&self) -> char {
        ':'
    }
    fn env_separator_str(&self) -> &'static str {
        ":"
    }
    fn set_env(&self, key: &str, value: &str) -> String {
        format!(r#"export {}="{}""#, key, value)
    }
    fn gen_setup_script(&self) -> &'static str;
}

pub fn resolve_shell(s: OsString) -> Result<Box<dyn Shell>> {
    let v = s.to_str().ok_or_else(|| anyhow!(NOT_UNICODE_ERR))?;
    let result: Box<dyn Shell> = match v {
        "bash" => Box::new(Bash),
        "zsh" => Box::new(Zsh),
        "fish" => Box::new(Fish),
        "pwsh" | "powershell" => Box::new(PowerShell),
        _ => bail!("Unsupported shell"),
    };

    Ok(result)
}
