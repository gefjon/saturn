use spin::{Mutex, MutexGuard};
use core::{fmt::{self, Write}};
use crate::driver::uart::Pc16550d;

static UART2: Mutex<Pc16550d> = Mutex::new(unsafe {
    Pc16550d::new(0xff1a_0000 as _)
});

pub unsafe fn init_console() {}

pub fn lock_console() -> MutexGuard<'static, impl Write> {
    UART2.lock()
}

pub unsafe fn force_unlock_console() {
    if UART2.is_locked() {
        UART2.force_unlock();
    }
}
