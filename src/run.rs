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

    let shell = shells::parse_shell_type(app.shell).context("Failed to parse shell type")?;

    let result = if let AppCmd::Setup(_) = app.subcommand {
        setup::create_setup(shell).gen_setup_script()
    } else {
        env::gen_env_settings(shell).context("Failed to generate command")?
    };

    print!("{}", result);

    Ok(())
}
