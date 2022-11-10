use log::error;
use std::process;

mod commands;
mod consts;
mod flags;
mod generate;
mod run;
mod shells;

fn main() {
    run::run().unwrap_or_else(|e| {
        error!("{:?}", e);
        process::exit(1);
    })
}
