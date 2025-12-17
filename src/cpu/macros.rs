#[macro_export]
macro_rules! gb_instructions {
    (
        $(
            $mnemonic:ident($opcode:expr, $cycles:expr) => $body:expr
        ),* $(,)?
    ) => {

        fn invalid(CPU: &mut $crate::cpu::CPU) -> u8 {
            unimplemented!("Unknown or invalid opcode at PC: 0x{:04X}", CPU.registers.PC);
        }

        $(
            fn $mnemonic(CPU: &mut $crate::cpu::CPU) -> u8 {
                $body(CPU);
                $cycles
            }
        )*

        pub const OPCODE_TABLE: [fn(&mut $crate::cpu::CPU) -> u8; 256] = {
            let mut table = [invalid as fn(&mut $crate::cpu::CPU) -> u8; 256];
            $(table[$opcode] = $mnemonic;)*
            table
        };
    };
}

#[macro_export]
macro_rules! gb_cb_instructions {
    (
        $(
            $mnemonic:ident($opcode:expr, $cycles:expr) => $body:expr
        ),* $(,)?
    ) => {

        fn invalid_cb(CPU: &mut $crate::cpu::CPU) -> u8 {
            unimplemented!("Unknown or invalid opcode at PC: 0x{:04X}", CPU.registers.PC);
        }

        $(
            fn $mnemonic(CPU: &mut $crate::cpu::CPU) -> u8 {
                $body(CPU);
                $cycles
            }
        )*

        pub const CB_OPCODE_TABLE: [fn(&mut $crate::cpu::CPU) -> u8; 256] = {
            let mut table = [invalid_cb as fn(&mut $crate::cpu::CPU) -> u8; 256];
            $(table[$opcode] = $mnemonic;)*
            table
        };
    };
}
