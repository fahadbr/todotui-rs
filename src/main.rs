#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod runner;
mod event;
mod flags;
mod app;
mod todo;
mod filters;

use std::{error::Error, fs::File};
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    flags::parse();
    match runner::start_term() {
        Ok(()) => Ok(()),
        Err(e) => {
            // TODO: only doing this because errors dont print to the console
            // find a better way
            let mut f = File::create("/tmp/todotui-rs.log")?;
            writeln!(f, "fatal: {}", e)?;
            Err(e)
        }
    }
}
