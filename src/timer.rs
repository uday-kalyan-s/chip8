use std::sync::{Arc, Mutex};
use std::thread;

pub struct Timer {
    // delay timer and sound timer
    counter: Arc<Mutex<(u8, u8)>>
}

impl Timer {

    pub fn get_delay(&self) -> u8 {
        let counter = self.counter.lock().unwrap();
        return counter.0;
    }

    pub fn new() -> Self {
        let counter = Arc::new(Mutex::new((0,0)));
        Self {
            counter
        }
    }

    pub fn run(&mut self) {
        let counter = Arc::clone(&self.counter);
        thread::spawn(move || {
            let mut cnt = counter.lock().unwrap();
            let (mut d,mut s) = *cnt;
            if d >= 6 {
                d -= 6;
            }
            else {
                d = 0;
            }
            if s >= 6 {
                s -= 6;
            }
            else {
                s = 0;
            }
            *cnt = (d,s);
        });
    }

    pub fn set_delay(&mut self, value: u8) {
        let mut counter = self.counter.lock().unwrap();
        counter.0 = value;
    }

    pub fn set_sound(&mut self, value: u8) {
        let mut counter = self.counter.lock().unwrap();
        counter.1 = value;
    }
}