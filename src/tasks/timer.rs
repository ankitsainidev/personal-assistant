use rodio::Source;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, BufReader, Write};
use std::ops::Add;
use std::path::Path;
use std::thread::sleep;
use std::{
    thread,
    time::{self, Duration},
};

pub fn start(duration: time::Duration, notification_dir: &Path) {
    // time_block determines the output flush frequency
    let time_block = Duration::from_millis(200);
    let mut time_passed = Duration::new(0, 0);

    while time_passed < duration {
        print!("\r{:>4}s / {}s", time_passed.as_secs(), duration.as_secs());
        io::stdout().flush().unwrap();
        thread::sleep(time_block);
        time_passed = time_passed.add(time_block);
    }

    // the final output on terminal shows task as completed
    print!("\r{:>4}s / {}s", time_passed.as_secs(), duration.as_secs());

    // thread::sleep(duration);
    println!("\nTimer Complete");

    // playing sound
    let device = rodio::default_output_device().unwrap();

    match File::open(notification_dir) {
        Ok(sound) => {
            let source = rodio::Decoder::new(BufReader::new(sound)).unwrap();
            rodio::play_raw(&device, source.convert_samples());

            // wait for sound to finish
            sleep(Duration::new(1, 0));
        }
        Err(e) => {
            if e.kind() != ErrorKind::NotFound {
                panic!();
            }

            // TODO: log when file not find
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup;
    #[test]
    fn time_accuracy() {
        // tests accuracy of 1 ms
        let start_time = time::Instant::now();
        let timer_duration = Duration::new(1, 0);

        // checks for the error handling when notification file doesn't exists
        start(timer_duration, Path::new("not_a_path"));
        let end_time = time::Instant::now();

        assert!(end_time - start_time < timer_duration + Duration::from_millis(1));
    }

    #[test]
    fn notification_setup() {
        setup::main("./testing_files");
        if !Path::new("./testing_files/.pat/notification.ogg").exists() {
            panic!("notification file not found");
        }
    }
}
