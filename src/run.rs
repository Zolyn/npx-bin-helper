use anyhow::{Context, Result};

use crate::commands::{env, setup};
use crate::flags;
use crate::flags::NpxBinHelperCmd;
use crate::shells;

pub fn run() -> Result<()> {
    env_logger::builder().format_timestamp(None).init();

    let app = match flags::NpxBinHelper::from_env() {
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

    if let NpxBinHelperCmd::Setup(_) = app.subcommand {
        setup::call(shell)
    } else {
        env::call(shell)?
    };

    Ok(())
}
