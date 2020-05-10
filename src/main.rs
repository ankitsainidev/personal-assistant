#[macro_use]
extern crate clap;
use std::io::{stdin, stdout, Write};
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches.subcommand_matches("note"){

    }
}
