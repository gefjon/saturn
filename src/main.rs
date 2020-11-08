#![cfg(target_arch = "aarch64")]
#![no_std]
#![no_main]
#![feature(
    asm,
    naked_functions,
    format_args_nl,
    panic_info_message,
    const_in_array_repeat_expressions,
)]

#[cfg(feature = "qemu")]
#[path = "board/qemu/mod.rs"]
mod board;

#[cfg(feature = "rockpro64")]
#[path = "board/rockpro64/mod.rs"]
mod board;

mod asm;
mod boot;
mod console;
mod driver;
mod memory;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        console::force_unlock_console();
    }

    let _ = console::with_console(|c| {
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

fn core_0_main() -> ! {
    let _ = console::print_str("Hello from a print_str call!\n")
        .expect("print_str failed");

    println!("Hello from a println call, {}", "Phoebe");

    panic!("Check out this panic message!");
}
