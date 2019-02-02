use crate::{*, processor_control_regs::sp};
use register::cpu::RegisterReadWrite;

#[no_mangle]
#[link_section=".text.boot"]
pub unsafe extern fn _start() -> ! {
    use cores::{which_core, CoreNo::*};

    match which_core() {
        Core0 => reset(),
        _ => sleep_forever(),
    }
}

/// Reset function.
///
/// Initializes the bss section before calling into the user's `main()`.
unsafe fn reset() -> ! {
    // These are all defined in `/link.ld`
    extern "C" {
        static __text_start: u64;
        
        static mut __bss_start: u64;
        static mut __bss_end: u64;
        
        static mut __data_start: u64;
        static mut __data_end: u64;
        static __data_loadaddr: u64;
    }

    sp().set(&__text_start as *const u64 as u64);
    
    asm!( "mov sp, $0"
           : 
           : "r"(&__text_start as *const u64 as u64)
           :
           : "volatile"
    );
    
    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    r0::init_data(&mut __data_start, &mut __data_end, &__data_loadaddr);

    kernel_entry()
}

