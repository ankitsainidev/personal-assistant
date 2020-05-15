use rodio::Source;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::ops::Add;
use std::path::Path;
use std::{thread, time};

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
    let sound = File::open(notification_file).unwrap();
    let source = rodio::Decoder::new(BufReader::new(sound)).unwrap();
    rodio::play_raw(&device, source.convert_samples());
    use std::thread::sleep;
    use std::time::Duration;

    // wait for sound to finish
    sleep(Duration::new(1, 0));
}
