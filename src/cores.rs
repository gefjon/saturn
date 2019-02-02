#![cfg(target_arch="aarch64")]

use core::convert::{From, Into};

use register::{cpu::RegisterReadWrite, *};

register_bitfields! {
    u64,
    MPIDR_EL1 [
        CORE_NR OFFSET(0) NUMBITS(2) [
            Core0 = 0b00,
            Core1 = 0b01,
            Core2 = 0b10,
            Core3 = 0b11
        ]
    ]
}

struct ArmSystemRegister<R>(core::marker::PhantomData<R>);

impl RegisterReadWrite<u64, MPIDR_EL1::Register> for ArmSystemRegister<MPIDR_EL1::Register> {
    #[inline]
    fn get(&self) -> u64 { unsafe {
        let res;
        asm!("mrs $0, mpidr_el1"
             : "=r"(res)
        );
        res
    } }
    #[inline]
    fn set(&self, value: u64) { unsafe {
        asm!("msr mpidr_el1 $0"
             :
             : "r"(value)
             :
             : "volatile"
        );
    } }
}

/// Multiprocessor Affinity Register
static MPIDR_EL1
    : ArmSystemRegister<MPIDR_EL1::Register>
    = ArmSystemRegister(core::marker::PhantomData);

use core::hint::unreachable_unchecked;

// exists as a hack because I can't figure out how to get the macro
// to export the same enum from `MPIDR_EL1::CORE_NR::Value`
pub enum CoreNo {
    Core0,
    Core1,
    Core2,
    Core3,
}

impl From<MPIDR_EL1::CORE_NR::Value> for CoreNo {
    fn from(nr: MPIDR_EL1::CORE_NR::Value) -> CoreNo {
        use self::MPIDR_EL1::CORE_NR::Value;
        
        match nr {
            Value::Core0 => CoreNo::Core0,
            Value::Core1 => CoreNo::Core1,
            Value::Core2 => CoreNo::Core2,
            Value::Core3 => CoreNo::Core3,
        }
    }
}

/// Returns the `CoreNo` corresponding to the active thread
pub fn which_core() -> CoreNo {
    MPIDR_EL1.read_as_enum(MPIDR_EL1::CORE_NR)
        .map(<MPIDR_EL1::CORE_NR::Value>::into)
        .unwrap_or_else(#[inline] || unsafe { unreachable_unchecked() })
}
