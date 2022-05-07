use crate::emulator::Emulator;

impl Emulator {
    fn refresh(&mut self) {
        print!("\x1B[2J\x1B[1;1H");
        for row in self.display_data {
            for val in row {
                print!("{}",if val {"#"} else {" "})
            }
            println!("")
        }
    }

    pub fn term_reset(&mut self) {
        self.display_data = [[false; 64]; 32];
    }

    pub fn draw(&mut self, xind: u16, yind: u16, n: u16) {
        let x_start = (self.registers[xind as usize] & 63) as usize;
        let mut y = (self.registers[yind as usize] & 31) as usize;
        self.registers[15] = 0;
        let sprite_bytes = &self.memory[(self.i as usize)..(self.i as usize + n as usize)];
        let mut x = x_start;
        for row in sprite_bytes {
            let mut row_bits_second: Vec<bool> = format!("{:b}", &row).chars().map(|c| if c == '0' {false} else {true}).collect();
            let mut row_bits: Vec<bool> = vec![false; 8-row_bits_second.len()];
            row_bits.append(&mut row_bits_second);
            for bit in row_bits {
                let current_status = self.display_data[y][x];
                if bit {
                    self.display_data[y][x] = !current_status;
                    if current_status {
                        self.registers[15] = 1;
                    }
                }
                x += 1;
                if x > 63 {
                    break;
                }
            }
            x = x_start;
            y += 1;
            if y > 31 {
                break;
            }
        }
        self.refresh();
    }
}