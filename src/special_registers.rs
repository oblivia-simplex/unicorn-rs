/// For identifying special registers for each Cpu and mode
macro_rules! special_register {
    ($func_name:ident, $( $mode:ident => $reg:expr),* ) => {
        pub fn $func_name(mode: libunicorn_sys::unicorn_const::Mode) -> impl Register {
            match mode {
                $(
                   libunicorn_sys::unicorn_const::Mode::$mode => $reg,
                )*
                _ => panic!("invalid mode"),
            }
        }
    }
}

pub mod x86 {
    use crate::x86_const::*;
    use crate::Register;

    special_register! {
        stack_pointer,
        MODE_32 => RegisterX86::ESP,
        MODE_64 => RegisterX86::RSP
    }

    special_register! {
        frame_pointer,
        MODE_32 => RegisterX86::EBP,
        MODE_64 => RegisterX86::RBP
    }

    special_register! {
        program_counter,
        MODE_32 => RegisterX86::EIP,
        MODE_64 => RegisterX86::RIP
    }
}
