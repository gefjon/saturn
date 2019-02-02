#![cfg(target_arch="aarch64")]
#![no_std]
#![no_main]
#![feature(asm)]

const MMIO_BASE: u32 = 0x3F00_0000;

mod boot;

mod processor_control_regs;
mod gpio;
mod uart;
mod cores;

fn sleep_forever() ->! {
    loop {
        unsafe { asm!("wfe" :::: "volatile") }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    sleep_forever()
}

fn kernel_entry() -> ! {
    let uart = uart::MiniUart::new();

    // set up serial console
    uart.init();
    uart.puts("\n[0] UART is live!\n");

    uart.puts("[1] Press a key to continue booting... ");
    uart.getc();
    uart.puts("Greetings fellow Rustacean!\n");

    // echo everything back
    loop {
        uart.send(uart.getc());
    }
}
