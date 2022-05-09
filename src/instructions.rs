use std::fmt::Debug;

use crate::emulator::Emulator;
// use rand::{thread_rng, Rng};

fn dehexmul<T>(hexes: Vec<char>) -> T
where T: TryFrom<u32>, <T as TryFrom<u32>>::Error: Debug
{
    let mut intval: u32 = 0;
    for i in hexes {
        intval = (intval*16) + i.to_digit(16).unwrap();
    }
    intval.try_into().unwrap()
}

fn dehex(chr: char) -> usize {
    chr.to_digit(16).unwrap() as usize
}

impl Emulator {

    fn get_ins(&mut self) -> (char, char, char, char) {
        let fc: Vec<char> = format!("{:x}",self.memory[self.pc as usize]).chars().collect();
        let sc: Vec<char> = format!("{:x}",self.memory[(self.pc+1) as usize]).chars().collect();
        self.pc += 2;
        // idk man
        (
            if fc.len() == 1 {'0'} else {fc[0]},
            if fc.len() == 1 {fc[0]} else {fc[1]},
            if sc.len() == 1 {'0'} else {sc[0]},
            if sc.len() == 1 {sc[0]} else {sc[1]},
        )
    }

    pub fn run(&mut self) {
        // start timer
        self.timer.run();

        // loop
        loop {
            let inp = self.get_ins();
            match inp {
                // clear screen
                ('0','0','e','0',) => {self.term_reset()},

                // jump
                ('1',n1,n2,n3) => {
                    self.pc = dehexmul(vec![n1,n2,n3]);
                },

                // set register value
                ('6',x,n1,n2) => {
                    self.registers[dehex(x)] = dehexmul(vec![n1,n2]);
                }

                // add value to register
                ('7',x,n1,n2) => {
                    let regind = dehex(x);
                    self.registers[regind] = self.registers[regind].overflowing_add(dehexmul(vec![n1,n2])).0;
                }

                // set value of i register
                ('a',n1,n2,n3) => {
                    self.i = dehexmul(vec![n1,n2,n3]);
                }

                // draw
                ('d',x,y, n) => {
                    self.draw(dehexmul(vec![x]), dehexmul(vec![y]), dehexmul(vec![n]))
                }

                // skip if x == nn
                ('3',x,n1,n2) => {
                    if self.registers[dehex(x)] == dehexmul(vec![n1,n2]) {
                        self.pc += 2;
                    }
                }

                // skip if x != nn
                ('4',x,n1,n2) => {
                    if self.registers[dehex(x)] != dehexmul(vec![n1,n2]) {
                        self.pc += 2;
                    }
                }

                //skip if vx == vy
                ('5',x,y, '0') => {
                    if self.registers[dehex(x)] == self.registers[dehex(y)] {
                        self.pc += 2;
                    }
                }

                //skip if vx != vy
                ('9',x,y, '0') => {
                    if self.registers[dehex(x)] != self.registers[dehex(y)] {
                        self.pc += 2;
                    }
                }

                // vx = vy
                ('8',x,y,'0') =>{
                    self.registers[dehex(x)] = self.registers[dehex(y)]
                }

                // vx | vy
                ('8',x,y,'1') =>{
                    self.registers[dehex(x)] |= self.registers[dehex(y)]
                }

                // vx & vy
                ('8',x,y,'2') =>{
                    self.registers[dehex(x)] &= self.registers[dehex(y)]
                }

                // vx ^ vy
                ('8',x,y,'3') => {
                    self.registers[dehex(x)] ^= self.registers[dehex(y)]
                }

                // vx += vy
                ('8',x,y,'4') => {
                    let sum = self.registers[dehex(x)].overflowing_add(self.registers[dehex(y)]);
                    self.registers[dehex(x)] = sum.0;
                    self.registers[15] = sum.1 as u8;
                }

                // vx -= vy
                ('8',x,y,'5') => {
                    let diff = self.registers[dehex(x)].overflowing_sub(self.registers[dehex(y)]);
                    self.registers[dehex(x)] = diff.0;
                    self.registers[15] = diff.1 as u8;
                }

                // vx = vy - vx
                ('8',x,y,'7') => {
                    let diff = self.registers[dehex(y)].overflowing_sub(self.registers[dehex(x)]);
                    self.registers[dehex(x)] = diff.0;
                    self.registers[15] = diff.1 as u8;
                }

                // vx = vy>>1
                ('8',x,y,'6') => {
                    let shifted_out = dehexmul::<u8>(vec![y]) & 1;
                    self.registers[dehex(x)] = self.registers[dehex(y)] >> 1;
                    self.registers[15] = shifted_out;
                }
                
                // vx = vy<<1
                ('8',x,y,'e') => {
                    self.registers[dehex(x)] = self.registers[dehex(y)] << 1;
                }

                // decimal version of values
                ('f',x,'3','3') => {
                    let mut regval: u8 = self.registers[dehex(x)];
                    for i in 0..3 {
                        self.memory[(self.i + (2-i)) as usize] = regval%10;
                        regval /= 10;
                    }
                }

                // set memory to register values
                ('f',x,'5','5') => {
                    for i in 0..(dehex(x)+1) {
                        self.memory[self.i as usize + i] = self.registers[i];
                    }
                }

                // execute subroutine
                ('2',n1,n2,n3) => {
                    self.stack.push_back(self.pc);
                    self.pc = dehexmul(vec![n1,n2,n3])
                    
                }

                // return from subroutine
                ('0','0','e','e') => {
                    self.pc = self.stack.pop_back().expect("stack underflow");
                }

                // fill registers
                ('f',x,'6','5') => {
                    let mut ival = self.i as usize;
                    for regind in 0..(dehex(x)+1) {
                        self.registers[regind] = self.memory[ival];
                        ival += 1;
                    }
                }

                //jump with offset
                ('b',n1,n2,n3) => {
                    self.pc = dehexmul::<u16>(vec![n1,n2,n3]) + self.registers[0] as u16;
                }

                // random number
                ('c',x,n1,n2) => {
                    // self.registers[dehex(x)] = thread_rng().gen_range(0..u8::MAX) & intify!(u8, n1,n2)
                    self.registers[dehex(x)] = fastrand::u8(0..u8::MAX) & dehexmul::<u8>(vec![n1,n2]);
                }

                // set delay timer
                ('f',x,'1','5') => {
                    self.timer.set_delay(self.registers[dehex(x)]);
                }

                // get value of timer
                ('f',x,'0','7') => {
                    self.registers[dehex(x)] = self.timer.get_delay();
                }

                // set sound timer
                ('f',x,'1','8') => {
                    self.timer.set_sound(self.registers[dehex(x)]);
                }

                // no matches?
                a => panic!("instruction {:?} not implemented yet, pc: {}", a,self.pc)
            }
        }
    }
}
