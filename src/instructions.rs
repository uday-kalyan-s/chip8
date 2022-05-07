use crate::emulator::Emulator;
// use rand::{thread_rng, Rng};

// converts characters for a hex value to an integer
// not sure if i can implement any sort of type inference or generics here so for now it takes an explicit type
macro_rules! intify {
    ($typ: ty, $($x: expr), *) => {
        {
            let mut intval = 0;
            $(
                intval = (intval * 16) + $x.to_digit(16).unwrap() as $typ;
            )*
            intval
        }
    };
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
        loop {
            let inp = self.get_ins();
            match inp {
                // clear screen
                ('0','0','e','0',) => {self.term_reset()},

                // jump
                ('1',n1,n2,n3) => {
                    self.pc = intify!(u16, n1,n2,n3);
                },

                // set register value
                ('6',x,n1,n2) => {
                    self.registers[intify!(usize, x)] = intify!(u8, n1,n2)
                }

                // add value to register
                ('7',x,n1,n2) => {
                    let regind = intify!(usize, x);
                    self.registers[regind] = self.registers[regind].overflowing_add(intify!(u8, n1,n2)).0;
                }

                // set value of i register
                ('a',n1,n2,n3) => {
                    self.i = intify!(u16, n1,n2,n3);
                }

                // draw
                ('d',x,y, n) => {
                    self.draw(intify!(u16, x), intify!(u16, y), intify!(u16, n))
                }

                // skip if x == nn
                ('3',x,n1,n2) => {
                    if self.registers[intify!(usize, x)] == intify!(u8, n1, n2) {
                        self.pc += 2;
                    }
                }

                // skip if x != nn
                ('4',x,n1,n2) => {
                    if self.registers[intify!(usize, x)] != intify!(u8, n1, n2) {
                        self.pc += 2;
                    }
                }

                //skip if vx == vy
                ('5',x,y, '0') => {
                    if self.registers[intify!(usize, x)] == self.registers[intify!(usize, y)] {
                        self.pc += 2;
                    }
                }

                //skip if vx != vy
                ('9',x,y, '0') => {
                    if self.registers[intify!(usize, x)] != self.registers[intify!(usize, y)] {
                        self.pc += 2;
                    }
                }

                // vx = vy
                ('8',x,y,'0') =>{
                    self.registers[intify!(usize, x)] = self.registers[intify!(usize, y)]
                }

                // vx | vy
                ('8',x,y,'1') =>{
                    self.registers[intify!(usize, x)] |= self.registers[intify!(usize, y)]
                }

                // vx & vy
                ('8',x,y,'2') =>{
                    self.registers[intify!(usize, x)] &= self.registers[intify!(usize, y)]
                }

                // vx ^ vy
                ('8',x,y,'3') => {
                    self.registers[intify!(usize, x)] ^= self.registers[intify!(usize, y)]
                }

                // vx += vy
                ('8',x,y,'4') => {
                    let sum = self.registers[intify!(usize,x)].overflowing_add(self.registers[intify!(usize,y)]);
                    self.registers[intify!(usize,x)] = sum.0;
                    self.registers[15] = sum.1 as u8;
                }

                // vx -= vy
                ('8',x,y,'5') => {
                    let diff = self.registers[intify!(usize,x)].overflowing_sub(self.registers[intify!(usize,y)]);
                    self.registers[intify!(usize,x)] = diff.0;
                    self.registers[15] = diff.1 as u8;
                }

                // vx = vy - vx
                ('8',x,y,'7') => {
                    let diff = self.registers[intify!(usize,y)].overflowing_sub(self.registers[intify!(usize,x)]);
                    self.registers[intify!(usize,x)] = diff.0;
                    self.registers[15] = diff.1 as u8;
                }

                // vx = vy>>1
                ('8',x,y,'6') => {
                    let shifted_out = intify!(u8,y) & 1;
                    self.registers[intify!(usize,x)] = self.registers[intify!(usize,y)] >> 1;
                    self.registers[15] = shifted_out;
                }
                
                // vx = vy<<1
                ('8',x,y,'e') => {
                    self.registers[intify!(usize,x)] = self.registers[intify!(usize, y)] << 1;
                }

                // decimal version of values
                ('f',x,'3','3') => {
                    let mut regval = self.registers[intify!(usize, x)];
                    for i in 0..3 {
                        self.memory[(self.i + (2-i)) as usize] = regval%10;
                        regval /= 10;
                    }
                }

                // set memory to register values
                ('f',x,'5','5') => {
                    for i in 0..(intify!(usize, x)+1) {
                        self.memory[self.i as usize + i] = self.registers[i];
                    }
                }

                // execute subroutine
                ('2',n1,n2,n3) => {
                    self.stack.push_back(self.pc);
                    self.pc = intify!(u16, n1,n2,n3);
                    
                }

                // return from subroutine
                ('0','0','e','e') => {
                    self.pc = self.stack.pop_back().expect("stack underflow");
                }

                // fill registers
                ('f',x,'6','5') => {
                    let mut ival = self.i as usize;
                    for regind in 0..(intify!(usize, x)+1) {
                        self.registers[regind] = self.memory[ival];
                        ival += 1;
                    }
                }

                //jump with offset
                ('b',n1,n2,n3) => {
                    self.pc = intify!(u16, n1,n2,n3) + self.registers[0] as u16;
                }

                // random number
                ('c',x,n1,n2) => {
                    // self.registers[intify!(usize,x)] = thread_rng().gen_range(0..u8::MAX) & intify!(u8, n1,n2)
                    self.registers[intify!(usize,x)] = fastrand::u8(0..u8::MAX) & intify!(u8, n1,n2);
                }

                // no matches?
                a => panic!("instruction {:?} not implemented yet, pc: {}", a,self.pc)
            }
        }
    }
}
