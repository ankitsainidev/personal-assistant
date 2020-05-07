#[macro_use]
extern crate clap;
use std::io::{stdin, stdout, Write};
use std::time::SystemTime;
use clap::App;
struct Note{
    msg: String,
    time: SystemTime,
}
impl Note{
    fn new(&self, msg: String)->Note{
        Note{
            msg: msg,
            time: SystemTime::now()
        }
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches.subcommand_matches("note"){

    }
}
