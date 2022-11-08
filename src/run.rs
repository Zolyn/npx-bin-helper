use anyhow::{Context, Result};

use crate::commands::{env, setup};
use crate::flags;
use crate::flags::AppCmd;
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

    let shell = shells::resolve_shell(app.shell).context("Failed to resolve shell type")?;

    if let AppCmd::Setup(_) = app.subcommand {
        setup::call(shell)
    } else {
        env::call(shell)?
    };

    Ok(())
}
