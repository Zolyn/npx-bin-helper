use anyhow::Context;
use anyhow::Result;

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

    if let Some(shell) = app.shell {
        let res = shells::shell_from_os_string(shell)
            .context("Failed to parse shell type")?
            .gen_setup_command();
        print!("{}", res)
    } else {
        let res = gen_command::gen_command().context("Failed to generate command")?;
        print!("{}", res)
    }

    Ok(())
}
