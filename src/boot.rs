use crate::{console, core_0_main, memory, sleep_forever};

#[link_section = ".text.boot.el2_entry"]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn _el2_entry() -> ! {
    asm!(
        // if this core0, go through the initialization routine. if
        // it's another core, sleep_forever.
        "mrs x8, mpidr_el1",
        // this tst examines aff0
        "tst x8, #0xff",
        "b.ne {sleep_forever}",
        // this tst examines aff1
        "tst x8, #0xff00",
        "b.ne {sleep_forever}",
        // todo: care about aff2 & aff3?
        // become_el1 takes a continuation as its first argument
        "adr x0, {el1_entry}",
        "b {become_el1}",

        sleep_forever = sym sleep_forever,
        el1_entry = sym el1_entry,
        become_el1 = sym become_el1,

        options(noreturn),
    );
}

#[link_section = ".text.boot"]
#[naked]
unsafe extern "C" fn become_el1(_entry: extern "C" fn() -> !) ->! {
    // arg is in x0
    asm!(
        // if we're in el1, go to el2_lower_to_el1.
        "mrs x8, currentel",
        "cmp w8, #0x8",
        "b.eq {el2_lower_to_el1}",
        // otherwise, we're already in el1. jump to _entry.
        "br x0",
        
        el2_lower_to_el1 = sym el2_lower_to_el1,

        options(noreturn),
    )
}

#[link_section = ".text.boot"]
#[naked]
/// construct an exception from which to return, then return from it into el1
unsafe extern "C" fn el2_lower_to_el1(_entry: extern "C" fn() -> !) -> ! {
    // arg is in x0
    asm!(
        // set hcr_el2 so that el1 runs in aarch64 mode
        "mov w8, #-0x80000000",
        "msr hcr_el2, x8",
        // set spsr_el2 to disable interrupts and to resume into aarch64 mode
        "mov w9, #0x345",
        "msr spsr_el2, x9",
        // set the exception resume address to the argument
        "msr elr_el2, x0",
        "eret",

        options(noreturn),
    )
}

#[link_section = ".text.boot"]
#[naked]
unsafe extern "C" fn el1_entry() -> ! {
    asm!(
        // use sp_elx at elx
        "msr spsel, #1",

        // set the stack pointer to just before the beginning of the code section
        "adr x9, {text_start}",
        "mov sp, x9",
        "b {init_and_enter}",
        
        text_start = sym memory::__text_start,

        init_and_enter = sym init_and_enter,
        
        options(noreturn),
    )
}

#[link_section = ".text.boot"]
unsafe extern "C" fn init_and_enter() -> ! {
    memory::init_data();
    console::init_console();
    memory::framealloc::init_frame_allocator(memory::kernel_end(), memory::max_phys_addr());
    core_0_main()
}
