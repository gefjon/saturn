#![cfg(target_arch="aarch64")]
#![no_std]
#![no_main]
#![feature(asm, try_from)]

const MMIO_BASE: u32 = 0x3F00_0000;

mod boot;

mod fundamentals;
mod asm;
mod gc;
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

fn core_0_main() -> ! {
    let uart = uart::UART_1.lock();

    uart.write("\n[0] UART is live!\n");

    uart.write("[1] Press a key to continue booting... ");

    uart.recieve();
    
    uart.write("Greetings fellow Rustacean!\n");

    // echo everything back
    loop {
           let c = uart.recieve();
           uart.send(c);
    }
}
