use std::SystemTime;

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

pub fn add_new(msg: String){

}
pub fn get_all() -> [Note]{

}
pub fn get_between_times(from: SystemTime, to: SystemTime) -> [Note]{

}
