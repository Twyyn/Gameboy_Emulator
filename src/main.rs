use gameboy_emulator::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    cpu.step();
}
