use crate::{asm::eret, cores::{which_core, CoreNo}, core_0_main, sleep_forever};
use cortex_a::{regs::*};
use register::cpu::{RegisterReadOnly, RegisterReadWrite};

// These are all defined in `/link.ld`
extern "C" {
    static __text_start: u64;

    static mut __bss_start: u64;
    static mut __bss_end: u64;

    static mut __data_start: u64;
    static mut __data_end: u64;
    static __data_loadaddr: u64;
}

#[link_section = ".text.boot"]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn _el2_entry() -> ! {
    asm!(
        "mrs x8, mpidr_el1",
        "tst x8, #3",
        "b.ne {sleep_forever}",
        "b {become_el1}",
        sleep_forever = sym sleep_forever,
        become_el1 = sym become_el1,
        options(nomem, nostack, noreturn),
    );
}

#[link_section = ".text.boot.become_el1"]
unsafe fn become_el1() ->! {
    if in_el2() {
        // put el1 in aarch64 mode
        HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);
        // mask all interrupts, set eret target to el1
        SPSR_EL2.write(SPSR_EL2::D::Masked
                           + SPSR_EL2::A::Masked
                           + SPSR_EL2::F::Masked
                           + SPSR_EL2::M::EL1h);
        // set eret link to continuation
        ELR_EL2.set(el1_entry as u64);
        SP_EL1.set(text_start());
        eret()
    } else {
        SP_EL1.set(text_start());
        el1_entry()
    }
}

#[link_section = ".text.boot.el1_entry"]
unsafe fn el1_entry() ->! {
    r0::zero_bss(&mut __bss_start, &mut __bss_end);
    r0::init_data(&mut __data_start, &mut __data_end, &__data_loadaddr);
    core_0_main()
}

#[inline(always)]
fn in_el2() -> bool {
    CurrentEL.get() == CurrentEL::EL::EL2.value
}

#[inline(always)]
unsafe fn text_start() -> u64 {
    (&__text_start) as *const u64 as u64
}
