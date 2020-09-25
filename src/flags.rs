
use clap::*;

pub fn parse() -> clap::ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!(", "))
        .get_matches()
}
