
use clap::{App, crate_authors, crate_name, crate_version};

pub fn parse() -> clap::ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!(", "))
        .get_matches()
}
