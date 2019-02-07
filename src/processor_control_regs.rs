use register::{cpu::RegisterReadWrite};

struct SP {}

pub unsafe fn sp() -> impl RegisterReadWrite<u64, ()> {
    SP {}
}

impl RegisterReadWrite<u64, ()> for SP {
    fn get(&self) -> u64 {
        let reg;
        unsafe { asm!( "mov $0, sp"
                        : "=r"(reg)
                        :
                        :
                        : "volatile"
        ) }
        reg
    }
    fn set(&self, sp: u64) {
        unsafe { asm!( "mov sp, $0"
                        :
                        : "r"(sp)
                        :
                        : "volatile"
        )}
    }
}
