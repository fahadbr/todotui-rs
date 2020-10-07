#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod app;
mod event;
mod flags;
mod state;
mod todo;

use std::{error::Error, fs::File};
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    flags::parse();
    match app::start_term() {
        Ok(()) => Ok(()),
        Err(e) => {
            let mut f = File::create("/tmp/todotui-rs.log")?;
            writeln!(f, "fatal: {}", e)?;
            Err(e)
        }
    }
}
