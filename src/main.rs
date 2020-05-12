#[macro_use]
extern crate clap;
use clap::App;
// use std::io::{stdin, stdout, Write};
mod tasks;
use tasks::*;
mod database;
use tokio;
use std::fs;

fn introduction(){
    let introduction = fs::read_to_string("data/introduction.txt")
                        .expect("Something went wrong");
    println!("{}", introduction);
}

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("static/cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if !database::db_exists(){
        introduction();
    }

    let mut db = database::db().await.expect("can't load database. ");
    if let Some(matches) = matches.subcommand_matches("note"){
        let msg = matches.value_of("note").unwrap();
        // println!("{:?}", note::get_all(&mut db).await.expect("can't get it"));
        note::add_new(String::from(msg), &mut db).await.expect("Problem storing");
    }
    else if let Some(matches) = matches.subcommand_matches("weather"){
        println!("{:?}", "matched weather");
    }
    else{
        println!("{:?}", "Weather is nice");
    }
}
