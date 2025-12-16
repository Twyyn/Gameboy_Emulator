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

        pub const DISPATCH: [fn(&mut $crate::cpu::CPU) -> u8; 256] = {
            let mut opcode_table = [invalid as fn(&mut $crate::cpu::CPU) -> u8; 256];
            $(opcode_table[$opcode] = $mnemonic;)*
            opcode_table
        };
    };
}
