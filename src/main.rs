#[macro_use]
extern crate clap;
use clap::App;
use serde_json;
use sqlx::Connection;
use std::convert::TryInto;
use std::io::{self, ErrorKind, Write};
use std::{env, fs, path::Path};
use tokio;

mod database;
mod tasks;

#[tokio::main]
async fn main() {
    // setting up the cli interface
    let yaml = load_yaml!("static/cli.yml");

    let home_dir = env::var("HOME").expect("Can't reach home directory.");
    let matches = App::from_yaml(yaml).get_matches();

    // checks for database and weather_config file
    startup(&home_dir[..]);

    let weather_config: Config = serde_json::from_str(include_str!("static/config.json")).unwrap();

    let mut db = database::db().await.expect("can't load database. ");
    let mut handler = Handler::new(&mut db, weather_config);
    match matches.subcommand() {
        ("note", Some(matches)) => handler.note(matches).await,
        ("weather", Some(_matches)) => handler.weather().await,
        ("save", Some(matches)) => handler.save(matches).await,
        ("todo", Some(matches)) => handler.todo(matches).await,
        ("quote", Some(_matches)) => handler.quote().await,
        ("timer", Some(matches)) => handler.timer(matches, &home_dir[..]),
        _ => introduction(),
    }

    db.close().await.expect("Error closing database");
}

fn introduction() {
    // Prints all the content of src/static/introduction.txt to screen

    let introduction = include_str!("static/introduction.txt");
    println!("{}", introduction);
}

fn startup(dir: &str) {
    let home_path = Path::new(dir);
    let pat_path = home_path.join(".pat");

    match fs::create_dir(pat_path.clone()) {
        Ok(_val) => {
            println!("Welcome. For help run `pat`");
        }
        Err(e) => {
            if e.kind() != ErrorKind::AlreadyExists {
                panic!("Problem creating directory");
            }
        }
    };

    /* include_bytes adds to the size of binary
        but helps in reducing the script to standalone binary */
    let data = include_bytes!("static/notification.ogg");
    let mut pos = 0;
    match fs::File::create(pat_path.join("notification.ogg")) {
        Ok(mut buff) => {
            while pos < data.len() {
                let bytes_written = buff.write(&data[pos..]).expect("Error writing sound file");
                pos += bytes_written;
            }
        }
        Err(e) => {
            if e.kind() != ErrorKind::AlreadyExists {
                println!("Can't store sound file required for timer");
            }
        }
    };
}

// Struct definitions for deserializing weather config including Api and location

#[derive(Debug, serde::Deserialize)]
struct Apis {
    open_weather_map: String,
}
#[derive(Debug, serde::Deserialize)]
struct Location {
    postal_code: String,
    country_code: String,
}
#[derive(Debug, serde::Deserialize)]
struct Locations {
    home: Location,
}
#[derive(Debug, serde::Deserialize)]
struct Config {
    api_keys: Apis,
    locations: Locations,
}

/*
Implements Handler which is responsible to handle all of the arguments passed
*/

// handles all the argument matches
struct Handler<'a> {
    db: &'a mut sqlx::SqliteConnection,
    weather_config: Config,
}

/* map argument matches to their corresponding tasks */
impl Handler<'_> {
    pub fn new(db: &mut sqlx::SqliteConnection, config: Config) -> Handler {
        Handler {
            db: db,
            weather_config: config,
        }
    }

    pub async fn quote(&self) {
        println!("{}", tasks::quote::quote().await.expect("Can't get quote"));
    }

    pub async fn todo(&mut self, matches: &clap::ArgMatches<'_>) {
        let desc = matches.value_of("desc").unwrap().to_string();
        if desc == "done".to_string() {
            if let Some(id) = matches.value_of("id") {
                tasks::todo::mark_done(id.to_string().parse::<i32>().unwrap(), &mut self.db)
                    .await
                    .expect("Can't mark complete");
            } else {
                println!("Please provide id of todo to mark completed");
            }
        } else if desc == "clean".to_string() {
            tasks::todo::clean(&mut self.db)
                .await
                .expect("Can't clean the database");
        } else if desc == "all".to_string() {
            let todos = tasks::todo::get_all(&mut self.db)
                .await
                .expect("Can't get todo list");
            for todo in todos {
                println!("{}", todo);
            }
        } else {
            tasks::todo::add_new(desc, &mut self.db)
                .await
                .expect("Can't add todo");
        }
    }

    pub async fn note(&mut self, matches: &clap::ArgMatches<'_>) {
        let msg = matches.value_of("note").unwrap().to_string();
        if msg == "all".to_string() {
            let notes = tasks::note::get_all(&mut self.db)
                .await
                .expect("Problem getting notes");

            for note in notes {
                println!("{}", note);
            }
        } else if msg == "delete".to_string() {
            let id = matches.value_of("id").unwrap().to_string();
            tasks::note::delete(id.parse::<i32>().unwrap(), &mut self.db)
                .await
                .expect("Error deleting note");
        } else {
            tasks::note::add_new(msg, &mut self.db)
                .await
                .expect("Problem taking note");
        }
    }

    pub async fn weather(&self) {
        // deconstructing the weather config
        let Config {
            api_keys: Apis {
                open_weather_map: weather_api_key,
            },
            locations:
                Locations {
                    home:
                        Location {
                            postal_code,
                            country_code,
                        },
                },
        } = &self.weather_config;
        println!(
            "{}",
            tasks::weather::now(
                weather_api_key.to_string(),
                postal_code.to_string(),
                country_code.to_string()
            )
            .await
            .expect("can't get weather data")
        );
    }

    pub async fn save(&mut self, matches: &clap::ArgMatches<'_>) {
        let msg = matches.value_of("key").unwrap();
        match msg {
            cmd @ "delete" | cmd @ "copy" => {
                let key = matches.value_of("value").unwrap().to_string();
                match cmd {
                    "delete" => {
                        if tasks::save::does_exists(key.clone(), &mut self.db).await {
                            tasks::save::delete(key, &mut self.db)
                                .await
                                .expect("can't delete");
                            println!("Save deleted successfully")
                        } else {
                            println!("No such save");
                        }
                    }

                    "copy" => {
                        if tasks::save::does_exists(key.clone(), &mut self.db).await {
                            let _saved = tasks::save::get(key, &mut self.db)
                                .await
                                .expect("can't get");
                            // Some problems using clipboard crate
                            // TODO: implement to copy value to clipboard
                            unimplemented!()
                        } else {
                            println!("No such save");
                        }
                    }
                    _ => unreachable!(),
                }
            }

            "all" => {
                let saves = tasks::save::get_all(&mut self.db)
                    .await
                    .expect("can't get saves");
                for saved in saves {
                    println!("{}", saved);
                }
            }

            _ => {
                let msg = msg.to_string();
                match matches.value_of("value") {
                    // case of new save `pat save number 33`
                    Some(value) => match tasks::save::get(msg.clone(), &mut self.db).await {
                        // if there's already an save
                        Ok(saved) => {
                            use text_io::read;
                            println!("a save already exists for this key. \n {}", saved);
                            print!("replace(y/n)? :");
                            io::stdout().flush().unwrap();

                            let command: String = read!("{}\n");

                            if command == "y" || command == "Y" {
                                tasks::save::delete(msg.clone(), &mut self.db)
                                    .await
                                    .expect("can't delete");
                                tasks::save::add_new(msg, value.to_string(), &mut self.db)
                                    .await
                                    .expect("can't add");
                                println!("replaced successfully");
                            } else {
                                println!("exiting");
                            }
                        }

                        // No previous save with same key
                        _ => {
                            tasks::save::add_new(msg, value.to_string(), &mut self.db)
                                .await
                                .expect("can't add");
                            println!("added successfully");
                        }
                    },

                    // Case of retrieval like `pat save number`
                    None => {
                        let saved = tasks::save::get(msg, &mut self.db)
                            .await
                            .expect("No such pair");
                        println!("{}", saved);
                    }
                }
            }
        }
    }

    pub fn timer(&self, matches: &clap::ArgMatches<'_>, home_dir: &str) {
        let str_duration = matches.value_of("duration").unwrap_or("10");
        let duration =
            std::time::Duration::new(str_duration.parse::<i32>().unwrap().try_into().unwrap(), 0);
        tasks::timer::start(duration, home_dir);
    }
}
