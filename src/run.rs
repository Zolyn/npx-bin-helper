use anyhow::{Context, Result};

use crate::flags;
use crate::gen_command;
use crate::shells;

pub fn run() -> Result<()> {
    env_logger::init();

    let app = match flags::App::from_env() {
        Ok(v) => v,
        Err(e) => {
            if e.is_help() {
                println!("{}", e);
                return Ok(());
            } else {
                return Err(e).context("Failed to parse arguments");
            }
        }
    };

    let result = if let Some(shell) = app.shell {
        shells::shell_from_os_string(shell)
            .context("Failed to parse shell type")?
            .gen_setup_command()
    } else {
        gen_command::gen_command().context("Failed to generate command")?
    };

    print!("{}", result);

    Ok(())
}
