use crate::{core_0_main, sleep_forever};

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
    asm!(
        // first, initialize the el1 stack pointer
        "adr x8, {text_start}",
        "msr sp_el1, x8",
        // then, if we're already in el1, go to el1_entry
        "mrs x8, currentel",
        "cmp w8, #0x8",
        "b.ne {el1_entry}",
        // otherwise, we're in el2. construct an exception from which
        // to return.
        // set hcr_el2 so that el1 runs in aarch64 mode
        "mov w8, #-0x80000000",
        "msr hcr_el2, x8",
        // set spsr_el2 to disable interrupts and to resume into aarch64 mode
        "mov w9, #0x345",
        "msr spsr_el2, x9",
        // set the exception resume address to el1_entry
        "adr x10, {el1_entry}",
        "msr elr_el2, x10",
        "eret",
        text_start = sym __text_start,
        el1_entry = sym el1_entry,
        options(nomem, nostack, noreturn),
    )
}

#[link_section = ".text.boot.el1_entry"]
unsafe fn el1_entry() ->! {
    r0::zero_bss(&mut __bss_start, &mut __bss_end);
    r0::init_data(&mut __data_start, &mut __data_end, &__data_loadaddr);
    core_0_main()
}
