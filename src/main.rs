#![cfg(target_arch="aarch64")]
#![no_std]
#![no_main]
#![feature(asm)]

const MMIO_BASE: u32 = 0x3F00_0000;

mod boot;

mod asm;
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
    let uart = unsafe { uart::Uart1::new() };

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
