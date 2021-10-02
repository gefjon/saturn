use crate::asm::{dsb};
use core::ops::RangeInclusive;

pub mod framealloc;

// These are all defined in `/link.ld`
extern "C" {
    pub static __text_start: u64;

    pub static mut __bss_start: u64;
    pub static mut __bss_end: u64;

    pub static mut __data_start: u64;
    pub static mut __data_end: u64;
    pub static __data_loadaddr: u64;

    pub static __kernel_end: u64;
}

pub const GIGABYTE: u64 = 0x4000_0000;
pub const PAGE_SIZE: u64 = 0x1000;

// i'm asserting that we use 48-bit virtual addresses.
const VADDR_MIN: u64 = 0;
const VADDR_MAX: u64 = 0x0000_ffff_ffff_ffff;
const VADDR_SPACE: RangeInclusive<u64> = VADDR_MIN ..= VADDR_MAX;

const KADDR_MIN: u64 = 0xffff_0000_0000_0000;
const KADDR_MAX: u64 = 0xffff_ffff_ffff_ffff;
const KADDR_SPACE: RangeInclusive<u64> = KADDR_MIN ..= KADDR_MAX;

pub use crate::board::memory::MEM_SIZE;

pub fn kernel_end() -> Paddr { Paddr(unsafe {&__kernel_end as *const u64 as u64}) }
pub fn max_phys_addr() -> Paddr { Paddr(MEM_SIZE - 1) }

pub unsafe trait Pointer: Sized {
    fn as_const<T>(self) -> *const T;
    fn as_mut<T>(self) -> *mut T;
    unsafe fn read<T>(self) -> T {
        core::ptr::read(self.as_const())
    }
    unsafe fn write<T>(self, val: T) {
        core::ptr::write(self.as_mut(), val)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// a physical address
pub struct Paddr(u64);

unsafe impl Pointer for Paddr {
    fn as_const<T>(self) -> *const T {
        self.0 as *const T
    }
    fn as_mut<T>(self) -> *mut T {
        self.0 as *mut T
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// a userspace virtual address, in the range `VADDR_MIN..=VADDR_MAX`
pub struct Vaddr(u64);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// a kernel vertial address, in the range `KADDR_MIN..=KADDR_MAX`
pub struct Kaddr(u64);

unsafe impl Pointer for Kaddr {
    fn as_const<T>(self) -> *const T {
        (self.0 - KADDR_MIN) as *const T
    }
    fn as_mut<T>(self) -> *mut T {
        (self.0 - KADDR_MIN) as *mut T
    }
}

macro_rules! impl_addrs {
    ($ty:ident $range:expr) => {
        impl core::fmt::LowerHex for $ty {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, concat!(stringify!($ty), "({:#x})"), self.0)
            }
        }
        impl core::convert::From<$ty> for u64 {
            fn from($ty (addr): $ty) -> u64 {
                addr
            }
        }
        impl core::convert::From<u64> for $ty {
            fn from(addr: u64) -> $ty {
                assert!(($range).contains(&addr));
                $ty(addr)
            }
        }
    };
    ($(($ty:ident $range:expr),)*) => {
        $(impl_addrs!($ty $range);)*
    };
}

impl_addrs!(
    (Paddr 0..=0xffff_ffff_ffff_ffff),
    (Kaddr KADDR_SPACE),
    (Vaddr VADDR_SPACE),
);

pub fn in_kaddr_space(addr: u64) -> bool {
    KADDR_SPACE.contains(&addr)
}

pub fn in_vaddr_space(addr: u64) -> bool {
    VADDR_SPACE.contains(&addr)
}

pub const fn paddr_to_kaddr(Paddr(paddr): Paddr) -> Kaddr {
    Kaddr(paddr + KADDR_MIN)
}

pub const fn kaddr_to_paddr(Kaddr(kaddr): Kaddr) -> Paddr {
    Paddr(kaddr - KADDR_MIN)
}

pub unsafe fn init_data() {
    r0::zero_bss(&mut __bss_start, &mut __bss_end);
    r0::init_data(&mut __data_start, &mut __data_end, &__data_loadaddr);

    // ensure all writes are seen by the whole system
    dsb::sy();
}
