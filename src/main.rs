mod emulator;
mod instructions;
mod term;
mod timer;

fn main() {
    let mut emulator = emulator::Emulator::new();
    emulator.load_font("font.txt");

    // displays tests
    emulator.load_program("test_opcode.ch8");
    
    // displays ibm logo
    // emulator.load_program("Logo.ch8");
    emulator.run();
}