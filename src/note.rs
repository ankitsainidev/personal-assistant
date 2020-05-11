use std::time::SystemTime;
use chrono::prelude::{Local, DateTime};
#[derive(Debug)]
struct Note{
    msg: String,
    time: DateTime<Local>,
}

impl Note{
    fn new(msg: String)->Note{
        Note{
            msg: msg,
            time: Local::now()
        }
    }

}

pub fn add_new(msg: String){
    let note = Note::new(msg).time.timestamp();
}

pub fn get_all(){
    unimplemented!()

}
pub fn get_between_times(from: SystemTime, to: SystemTime){
    unimplemented!()
}
