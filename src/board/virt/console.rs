use crate::sync::Mutex;
use crate::driver::uart::Pl011;

pub static CONSOLE: Mutex<Pl011> = Mutex::new(
    unsafe { Pl011::new(0x0900_0000 as _) }
);
