use std::fs;
use std::io::{ErrorKind, Write};
use std::path::Path;

pub fn main(dir: &str) {
    let home_path = Path::new(dir);
    let pat_path = home_path.join(".pat");
    home_dir(&pat_path);
    timer_sound(&pat_path);
    config(&pat_path);
}

fn home_dir(pat_path: &Path) {
    let dir_exists = pat_path.exists();
    if !dir_exists {
        match fs::create_dir_all(pat_path) {
            Ok(_val) => {
                println!("Welcome. For help run `pat`");
            }
            Err(e) => {
                if e.kind() != ErrorKind::AlreadyExists {
                    panic!("Problem creating directory");
                }
            }
        }
    }
}

fn timer_sound(pat_path: &Path) {
    /* the notification sound bytes are read at compile time
    and saved to a file at runtime which can be used to play sound.
    Increase the binary size but helps making it a standalone binary */

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
    }
}

fn config(pat_path: &Path) {
    // writes the weather configuration json file to ~/.pat/config.json

    let config_default_content = include_bytes!("static/config.json");
    match fs::File::create(pat_path.join("config.json")) {
        Ok(mut buff) => {
            buff.write_all(config_default_content)
                .expect("Can't write to weather config file");
        }
        Err(e) => {
            if e.kind() != ErrorKind::AlreadyExists {
                println!("Can't create weather config file.");
            }
        }
    }
}
