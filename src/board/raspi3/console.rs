use spin::Mutex;
use crate::driver::uart::Pl011;

pub static CONSOLE: Mutex<Pl011> = Mutex::new(
    unsafe { Pl011::new(0x3F20_1000usize as *mut u8) }
);
