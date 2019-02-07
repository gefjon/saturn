use crate::{*, cores::CoreNo, processor_control_regs::sp};
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
#[link_section=".text.boot"]
pub unsafe extern fn _start() -> ! {
    use cores::{which_core};

    let me = which_core();
    sp().set(sp_for(me));

    init_bss_data(me);
    
    match me {
       CoreNo::Core0 => core_0_main(),
        _ => sleep_forever(),
    }
}

#[inline(always)]
unsafe fn text_start() -> u64 {
    (&__text_start) as *const u64 as u64
}

#[inline(always)]
unsafe fn stack_size_per_thread() -> u64 {
    text_start() / 4
}

#[inline(always)]
unsafe fn sp_for(corenr: CoreNo) -> u64 {
    text_start() - (corenr as u64 * stack_size_per_thread())
}


unsafe fn init_bss_data(corenr: CoreNo) {
    if CoreNo::Core0 == corenr {
        r0::zero_bss(&mut __bss_start, &mut __bss_end);
        r0::init_data(&mut __data_start, &mut __data_end, &__data_loadaddr);
    }
    asm::dsb()
}

