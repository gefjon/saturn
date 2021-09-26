use crate::asm::{dsb};

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

const GIGABYTE: u64 = 0x4000_0000;

// some terminology:
// `paddr`: a physical address, in the range `0..MEMORY_SIZE`.
// `kaddr`: a kernel address, in the range `KADDR_MIN..=KADDR_MAX`
// `vaddr`: a userspace virtual address, in the range `0..VADDR_MAX` (TBD)

// i'm asserting that we use 48-bit virtual addresses.
const VADDR_MAX: u64 = 0x0000_ffff_ffff_ffff;
const KADDR_MIN: u64 = 0xffff_0000_0000_0000;

pub fn in_kaddr_space() -> bool {
    crate::asm::get_pc() >= KADDR_MIN
}

pub const fn paddr_to_kaddr(paddr: u64) -> u64 {
    paddr + KADDR_MIN
}

pub const fn kaddr_to_paddr(kaddr: u64) -> u64 {
    kaddr - KADDR_MIN
}

pub unsafe fn init_data() {
    r0::zero_bss(&mut __bss_start, &mut __bss_end);
    r0::init_data(&mut __data_start, &mut __data_end, &__data_loadaddr);

    // ensure all writes are seen by the whole system
    dsb::sy();
}
