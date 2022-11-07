use std::process;

mod commands;
mod consts;
mod flags;
mod run;
mod shells;

const APP_ERROR_MSG: &str = "[NPX_BIN_HELPER]: An error was occurred:";

fn main() {
    run::run().unwrap_or_else(|e| {
        eprintln!("{} {:?}", APP_ERROR_MSG, e);
        process::exit(1);
    })
}
