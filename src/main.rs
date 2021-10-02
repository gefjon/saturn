#![cfg(target_arch = "aarch64")]
#![no_std]
#![no_main]
#![feature(
    asm,
    naked_functions,
    format_args_nl,
    panic_info_message,
    const_panic,
)]

#[cfg(feature = "virt")]
#[path = "board/virt/mod.rs"]
mod board;

#[cfg(feature = "rockpro64")]
#[path = "board/rockpro64/mod.rs"]
mod board;

#[cfg(feature = "raspi3")]
#[path = "board/raspi3/mod.rs"]
mod board;

mod asm;
mod boot;
mod console;
mod driver;
mod memory;

use core::convert::From;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        console::force_unlock_console();
    }

    let _ = console::with_writing(|c| {
        c.write_str("\nKernel panic")?;

        if let Some(msg) = info.message() {
            c.write_fmt(format_args!(": {}", msg))?;
        }

        if let Some(loc) = info.location() {
            c.write_fmt(format_args!("\n\tin file '{}' at line {}, column {}",
                                     loc.file(), loc.line(), loc.column()))?;
        }

        c.write_str("\n")
    });

    sleep_forever()
}

fn sleep_forever() -> ! {
    use cortex_a::asm::wfe;
    loop { wfe(); }
}

fn echo_loop() -> ! {
    console::with_console(|c| {
        loop {
            let byte = c.blocking_read_byte();
            c.blocking_write_byte(byte);
        } } )
}

fn core_0_main() -> ! {
    console::print_str("Hello from a print_str call!\n")
        .expect("print_str failed");

    println!("Hello from a println call, {}", "Phoebe");

    let pc = asm::get_pc();
    
    if memory::in_kaddr_space(pc) {
        let pc = memory::Kaddr::from(pc);
        panic!(
            "We're in the high address space at {:x} when we should be in the low address space at {:x}!",
            pc, memory::kaddr_to_paddr(pc),
        );
    } else {
        let pc = memory::Paddr::from(pc);
        println!("We are in the low address space, and all is well with the world.");
        println!(
            "PC is currently {:x}, which would be {:x} in the high address space",
            pc, memory::paddr_to_kaddr(pc),
        );
    }

    println!("kernel ends at {:x}", memory::kernel_end());

    if let Some(block) = memory::framealloc::alloc_frame(memory::PAGE_SIZE) {
        println!("Successfully allocated the block {:x}", block);
        unsafe { memory::framealloc::free_frame(block, memory::PAGE_SIZE) };
        println!("Successfully freed that block.");
    } else {
        println!("Failed to alloc a block!");
    }
    
    println!("Now echoing:");

    echo_loop()
}
