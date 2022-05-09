use std::collections::VecDeque;
use std::fs;
use crate::timer::Timer;

pub struct Emulator {
    pub stack: VecDeque<u16>,
    pub memory: [u8; 4096],
    pub i: u16,
    pub registers: [u8; 16],
    pub display_data: [[bool; 64]; 32],
    pub timer: Timer,

    // points to current instruction in memory
    pub pc: u16
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            stack: VecDeque::new(),
            memory: [0; 4096],
            i: 0,
            registers: [0; 16],
            pc: 512,
            display_data: [[false; 64]; 32],
            timer: Timer::new()

        }
    }

    pub fn load_font(&mut self, path: &str) {
        let font_data_raw = fs::read_to_string(path).unwrap();

        let mut mem_pos = 80;
        for i in font_data_raw.split(",") {
            let byte = u8::from_str_radix(&i.trim()[2..], 16).unwrap();
            self.memory[mem_pos] = byte;
            mem_pos += 1;
        }
    }

    pub fn load_program(&mut self, path: &str) {
        let mut mem_pos = 512;
        for i in fs::read(path).unwrap() {
            self.memory[mem_pos] = i;
            mem_pos += 1;
        }
    }
}