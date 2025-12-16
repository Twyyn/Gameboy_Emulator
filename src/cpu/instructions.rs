use crate::instructions;

instructions!(
    NOP(0x00, 1) => |_CPU| {
        println!("NOP");
    },

);
