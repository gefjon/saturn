#![cfg(target_arch = "aarch64")]
#![no_std]
#![no_main]
#![feature(
    asm,
    naked_functions,
    format_args_nl,
    panic_info_message,
)]

const MMIO_BASE: u32 = 0x3F00_0000;

mod asm;
mod boot;
mod console;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    {
        use core::fmt::Write;
        let _ = console::print_str("\nKernel panic\n");
        let mut c = console::lock_console();
        let _ = c.write_str("\nKernel panic\n");
        if let Some(msg) = info.message() {
            let _ = c.write_fmt(format_args!(": {}", msg));
        }
        if let Some(loc) = info.location() {
            let _ = c.write_fmt(format_args!("\n\tin file '{}' at line {}, column {}",
                                             loc.file(), loc.line(), loc.column()));
        }
        let _ = c.write_str("\n");
    }
    sleep_forever()
}

fn sleep_forever() -> ! {
    use cortex_a::asm::wfe;
    loop { wfe(); }
}

fn core_0_main() -> ! {
    let _ = console::print_str("Hello from a print_str call!\n");
    let _ = console::print_str("Hello from another print_str call!\n");
    let name = "you nerd!";
    let _ = console::print(format_args!("This string is being formatted, {}", name));
    let _ = console::print_str("Hello from after a format_args!\n");
    println!("Hello from a println call, {}", "Phoebe");
    panic!("Check out this panic message!");
}
