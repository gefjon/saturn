#![cfg(target_arch = "aarch64")]
#![no_std]
#![no_main]
#![feature(
    llvm_asm,
    naked_functions,
    panic_info_message,
    alloc_error_handler
)]

const MMIO_BASE: u32 = 0x3F00_0000;

mod allocate;
use crate::allocate::Allocator;

extern crate alloc;

#[global_allocator]
/// This is initialized in `boot::init_bss_data`
static GLOBAL: Allocator = Allocator::uninit();

mod boot;
mod mailbox;
// mod fundamentals;
mod asm;
// mod gc;
mod gpio;
#[macro_use]
mod uart;
mod cores;

use register::cpu::RegisterReadOnly;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("KERNEL PANIC");

    if let Some(loc) = info.location() {
        print!(" @ {}", loc);
    }

    if let Some(msg) = info.message() {
        print!(": {}", msg);
    }

    println!();

    sleep_forever()
}

fn sleep_forever() -> ! {
    loop {
        unsafe { llvm_asm!("wfe" :::: "volatile") }
    }
}

fn core_0_main() -> ! {
    println!();
    println!("Hello, brave new world!");

    println!(
        "I am in EL{}",
        cortex_a::regs::CurrentEL.read(cortex_a::regs::CurrentEL::EL)
    );

    unsafe {
        GLOBAL.try_an_alloc();

        println!(
            "My serial number is {:x}",
            mailbox::VC_MAILBOX.lock().read_serial_number(),
        );
    }
    
    println!("I don't know what to do next, so I'm gonna panic.");
    panic!("Check out this panic message, though!");
}
