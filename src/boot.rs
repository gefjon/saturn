use crate::{cores::CoreNo, *};
use cortex_a::{asm::eret, regs::*};
use register::cpu::RegisterReadWrite;

// These are all defined in `/link.ld`
extern "C" {
    static __text_start: u64;

    static mut __bss_start: u64;
    static mut __bss_end: u64;

    static mut __data_start: u64;
    static mut __data_end: u64;
    static __data_loadaddr: u64;
}

#[no_mangle]
#[link_section = ".text.boot"]
pub unsafe extern "C" fn _el2_entry() -> ! {
    if cores::which_core() != CoreNo::Core0 {
        sleep_forever();
    } else {
        SP_EL1.set((&__text_start) as *const u64 as u64);
        el1_entry_core0()
    }

    

    // if CurrentEL.get() == CurrentEL::EL::EL2.value {
    //     // Enable timer counter registers for EL1
    //     CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);
    //     // No offset for reading the counters
    //     CNTVOFF_EL2.set(0);
    //     // Set EL1 execution state to AArch64
    //     HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    //     // Set up a simulated exception return.
    //     //
    //     // First, fake a saved program status, where all interrupts were
    //     // masked and SP_EL1 was used as a stack pointer.
    //     SPSR_EL2.write(
    //         SPSR_EL2::D::Masked
    //             + SPSR_EL2::A::Masked
    //             + SPSR_EL2::I::Masked
    //             + SPSR_EL2::F::Masked
    //             + SPSR_EL2::M::EL1h,
    //     );
    //     // Second, let the link register point to reset().
    //     ELR_EL2.set(el1_entry_core0 as *const () as u64);

    //     eret()
    // } else {
        
    // }
}

#[allow(unused)]
unsafe fn el1_entry_core0() -> ! {
    init_bss_data();
    core_0_main();
}

#[allow(unused)]
unsafe fn el1_entry_coren() -> ! {
    sleep_forever();
}

#[allow(unused)]
#[inline(always)]
unsafe fn entry_for(me: CoreNo) -> u64 {
    match me {
        CoreNo::Core0 => el1_entry_core0 as *const () as u64,
        _ => el1_entry_coren as *const () as u64,
    }
}

#[allow(unused)]
#[inline(always)]
unsafe fn text_start() -> u64 {
    (&__text_start) as *const u64 as u64
}

#[allow(unused)]
#[inline(always)]
unsafe fn stack_size_per_thread() -> u64 {
    text_start() / 4
}

#[allow(unused)]
#[inline(always)]
unsafe fn sp_for(corenr: CoreNo) -> u64 {
    text_start() - (corenr as u64 * stack_size_per_thread())
}

unsafe fn init_bss_data() {
    r0::zero_bss(&mut __bss_start, &mut __bss_end);
    r0::init_data(&mut __data_start, &mut __data_end, &__data_loadaddr);

    crate::GLOBAL.init((&mut __bss_end) as *mut u64 as *mut u8);

    asm::dsb()
}
