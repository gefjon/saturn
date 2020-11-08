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

use core::default::Default;
use core::fmt::{self, Write};

struct ByteBuf {
    buf: [u8; 1024],
    fill: usize,
}

impl fmt::Write for ByteBuf {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            if self.fill == 1024 {
                return Err(fmt::Error)
            }
            self.buf[self.fill] = b;
            self.fill += 1;
        }
        Ok(())
    }
}

impl Default for ByteBuf {
    fn default() -> Self { Self {
        buf: [0; 1024],
        fill: 0,
    } }
}

impl ByteBuf {
    fn as_str(&self) -> &str { unsafe {
        core::str::from_utf8_unchecked(
            core::slice::from_raw_parts((&self.buf) as *const u8, self.fill)
        )
    } }
}

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
    console::print_str("Hello from a print_str call!\n")
        .expect("print_str failed");

    console::print_str("I'm writing a whole lot in an attempt to fill up the FIFO!\n")
        .expect("print_str failed");

    {
        let s = [b'a'; 2048];
        console::print_str(unsafe { core::str::from_utf8_unchecked(&s) })
            .expect("print_str failed");
    }
    console::print_str("\nHow cool was that?\n")
        .expect("print_str failed");

    {
        let mut buf = ByteBuf::default();
        buf.write_fmt(format_args_nl!("Will it work if I format into a buffer?"));

        console::print_str("I wrote into a buffer!\n");
        
        console::print_str(buf.as_str())
            .expect("print_str failed");
    }

    println!("This part doesn't work, though... :(");
    
    panic!("Check out this panic message!");
}
