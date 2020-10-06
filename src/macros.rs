macro_rules! implement_register {
    ($reg_arch:ty) => {
        impl Register for $reg_arch {
            fn to_i32(&self) -> i32 {
                *self as i32
            }
        }
    };
}

macro_rules! implement_emulator {
    ($emu_type_doc:meta, $emu_instance_doc:meta, $cpu:ident, $arch:expr, $reg:ty,
    $( $special_register:ident [$( $mode:pat => $register:expr $(,)?)* ] $(,)?)* ) => {
        #[$emu_type_doc]
        pub struct $cpu<'a> {
            emu: Box<Unicorn<'a>>,
            mode: Mode,
            arch: Arch,
        }

        impl std::fmt::Debug for $cpu<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "<{:?} Cpu, {:?} mode>", self.arch, self.mode)
            }
        }

        impl<'a> Cpu<'a> for $cpu<'a> {
            type Reg = $reg;

            #[$emu_instance_doc]
            fn new(mode: Mode) -> Result<Self> {
                let emu = Unicorn::new($arch, mode);
                match emu {
                    Ok(x) => Ok(Self { emu: x, mode, arch: $arch }),
                    Err(x) => Err(x),
                }
            }

            fn emu(&self) -> &Unicorn<'a> {
                &self.emu
            }

            fn emu_mut(&mut self) -> &mut Unicorn<'a> {
                &mut self.emu
            }

            fn mode(&self) -> Mode {
                self.mode
            }

            fn arch(&self) -> Arch {
                self.arch
            }

            $(
                #[allow(unreachable_patterns)]
                fn $special_register(&self) -> <$cpu<'_> as Cpu<'_>>::Reg {
                    match self.mode() {
                      $(
                          $mode => $register,
                       )*
                       _ => unreachable!("{} is not implemented for the {:?} architecture", stringify!($special_register), $arch)
                    }
                }
            )*
        }

        unsafe impl Send for $cpu<'_> {}
    };
}

macro_rules! destructure_hook {
    ($hook_type:path, $hook:ident) => {{
        let $hook_type { unicorn, callback } = unsafe { &mut *$hook };
        (unsafe { &**unicorn }, callback)
    }};
}
