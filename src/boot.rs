use crate::{console, core_0_main, memory, sleep_forever};

#[link_section = ".text.boot.el2_entry"]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn _el2_entry() -> ! {
    asm!(
        "mrs x8, mpidr_el1",
        
        // bit 31 indicates a uniprocessor system
        "tbnz x8, #30, {core0_highel_entry}",

        // this examines aff0, aff1, and aff2, deciding we are core0
        // if all are zero.
        //
        // FIXME: examine aff3?
        "tst x8, #0xffffff",
        "b.ne {coren_highel_entry}",
        "b {core0_highel_entry}",

        core0_highel_entry = sym core0_highel_entry,
        coren_highel_entry = sym coren_highel_entry,
        options(nostack, noreturn),
    );
}

#[link_section = ".text.boot"]
#[naked]
unsafe extern "C" fn core0_highel_entry() -> ! {
    asm!(
        // become_el1 takes a continuation as its first argument
        "adr x0, {set_sp}",
        "b {become_el1}",

        set_sp = sym set_sp,
        become_el1 = sym become_el1,

        options(nostack, noreturn),
    );
}

#[link_section = ".text.boot"]
#[naked]
unsafe extern "C" fn coren_highel_entry() -> ! {
    asm!(
        "b {sleep_forever}",

        sleep_forever = sym sleep_forever,

        options(nomem, nostack, noreturn),
    )
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

        options(nostack, noreturn),
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

        options(nostack, noreturn),
    )
}

#[link_section = ".text.boot"]
#[naked]
unsafe extern "C" fn set_sp() -> ! {
    asm!(
        // use sp_elx at elx
        "msr spsel, #1",
        
        // set the stack pointer to just before the beginning of the code section
        "adr x8, {text_start}",
        "mov sp, x8",

        "b {core0_el1_entry}",

        text_start = sym memory::__text_start,
        core0_el1_entry = sym core0_el1_entry,
        
        options(nostack, noreturn),
    );
}

#[link_section = ".text.boot"]
#[naked]
unsafe extern "C" fn core0_el1_entry() -> !{
    memory::init_data();

    console::init_console();

    core_0_main()
}
