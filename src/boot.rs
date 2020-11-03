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
        // set_stack_pointer takes a continuation as its first argument
        "adr x0, {el1_entry}",
        "b {set_stack_pointer}",

        sleep_forever = sym sleep_forever,
        el1_entry = sym el1_entry,
        set_stack_pointer = sym set_stack_pointer,

        options(nomem, nostack, noreturn),
    );
}

#[link_section = ".text.boot"]
#[naked]
unsafe extern "C" fn set_stack_pointer(_entry: extern "C" fn() -> !) -> ! {
    // arg is in x0, passed untouched to later fns
    asm!(
        // set the stack pointer to just before the beginning of the code section
        "adr x8, {text_start}",
        "msr sp_el1, x8",
        "b {become_el1}",

        text_start = sym memory::__text_start,
        become_el1 = sym become_el1,

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

        options(nomem, nostack, noreturn),
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

        options(nomem, nostack, noreturn),
    )
}

#[link_section = ".text.boot"]
#[naked]
unsafe extern "C" fn el1_entry() ->! {
    memory::init_data();
    console::init_console();
    core_0_main()
}
