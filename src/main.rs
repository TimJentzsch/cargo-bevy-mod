use clap::Parser;
use cli::{CargoCommand, Command};

pub(crate) mod cargo;
mod cli;
pub(crate) mod env;
mod new;
mod run;
pub(crate) mod wasm_bindgen;

fn main() {
    let cmd = CargoCommand::parse();
    cmd.exec();
}
