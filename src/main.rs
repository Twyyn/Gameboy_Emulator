mod cpu;
mod memory;
use crate::cpu::registers;

fn main() {
    let register = registers::Register::new();
    println!("{:?}", register.AF());
    println!("{:?}", register.BC());
    println!("{:?}", register.DE());
    println!("{:?}", register.HL());
    println!("{:?}", register.Flags());
    println!("{:?}", register.Z_Flag());
}
