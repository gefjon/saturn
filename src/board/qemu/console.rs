use spin::{Mutex, MutexGuard};
use core::fmt::{Write};
use crate::driver::uart::Pl011;

static CONSOLE: Mutex<Pl011> = Mutex::new(
    unsafe { Pl011::new(0x0900_0000 as _) }
);

pub fn lock_console() -> MutexGuard<'static, impl Write> {
    CONSOLE.lock()
}

pub unsafe fn force_unlock_console() {
    if CONSOLE.is_locked() {
        CONSOLE.force_unlock();
    }
}

pub unsafe fn init_console() {}
