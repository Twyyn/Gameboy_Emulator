#[macro_export]
macro_rules! instructions {
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
