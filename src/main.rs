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
pub mod boot;
mod console;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    {
        use core::fmt::Write;
        unsafe {
            console::force_unlock_console();
        }
        let mut c = console::lock_console();
        let _ = c.write_str("\nKernel panic");
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
    let _ = console::print_str("Hello from a print_str call!\n")
        .expect("print_str failed");

    println!("Hello from a println call, {}", "Phoebe");

    panic!("Check out this panic message!");
}
