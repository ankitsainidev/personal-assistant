#[macro_use]
extern crate clap;
use clap::App;
use std::io::{stdin, stdout, Write};
mod note;
use sqlx::{SqliteConnection, Connect, Executor};
use tokio;
// fn main() {
//     let yaml = load_yaml!("cli.yml");
//     let matches = App::from_yaml(yaml).get_matches();
//     if let Some(matches) = matches.subcommand_matches("note"){
//         let msg = matches.value_of("note").unwrap();
//         note::add_new(String::from(msg));
//     }
// }
#[tokio::main]
async fn main()-> Result<(), sqlx::Error> {
    let path = "/home/akanksha/code/rust/pat/data/data.db";
    println!("{:?}", &format!("sqlite://{}", path));
    let mut conn = SqliteConnection::connect(&format!("sqlite://{}", path)).await?;
    let query = conn.execute("select * from notes").await?;
    println!("{:?}", query);
    conn.execute("insert into notes values (\"Hello\", 3)").await?;
    Ok(())
}
