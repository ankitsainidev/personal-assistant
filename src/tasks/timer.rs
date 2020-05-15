use rodio::Source;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, BufReader, Write};
use std::ops::Add;
use std::path::Path;
use std::thread::sleep;
use std::{
    thread,
    time::{self, Duration, Instant},
};

pub fn start(duration: time::Duration, home_dir: &str) {
    // time_block determines the output flush frequency
    let time_block = Duration::from_millis(200);
    let mut time_passed = Duration::new(0, 0);
    while time_passed < duration {
        print!("\r{:>4}s / {}s", time_passed.as_secs(), duration.as_secs());
        io::stdout().flush().unwrap();
        thread::sleep(time_block);
        time_passed = time_passed.add(time_block);
    }
    print!("\r{:>4}s / {}s", time_passed.as_secs(), duration.as_secs());

    // thread::sleep(duration);
    println!("\nTimer Complete");

    // playing sound
    let device = rodio::default_output_device().unwrap();
    let home_path = Path::new(home_dir);
    let notification_file = home_path.join(".pat").join("notification.ogg");

    match File::open(notification_file) {
        Ok(sound) => {
            let source = rodio::Decoder::new(BufReader::new(sound)).unwrap();
            rodio::play_raw(&device, source.convert_samples());

            // wait for sound to finish
            sleep(Duration::new(1, 0));
        }
        Err(e) => {
            // Ignoring file not exists helps in testing

            if e.kind() != ErrorKind::NotFound {
                panic!();
            }

            // TODO: log when file not find
        }
    }
}

#[test]
fn time_accuracy() {
    // tests accuracy of 1 ms
    let start_time = Instant::now();

    let timer_duration = Duration::new(1, 0);

    start(timer_duration, "None");
    let end_time = Instant::now();

    assert!(end_time - start_time < timer_duration + Duration::from_millis(1));
}
