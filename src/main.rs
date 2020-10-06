#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod app;
mod flags;
mod event;
mod state;
mod todo;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    flags::parse();
    app::start_term()
}

