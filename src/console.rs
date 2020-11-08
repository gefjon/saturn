use core::fmt::{self, Write, Arguments};
use spin::MutexGuard;

pub use crate::board::console::CONSOLE;

pub fn lock_console() -> MutexGuard<'static, impl Write> {
    CONSOLE.lock()
}

pub unsafe fn force_unlock_console() {
    if CONSOLE.is_locked() {
        CONSOLE.force_unlock();
    }
}

pub unsafe fn init_console() {}

pub fn with_console<F, R>(f: F) -> Result<R, fmt::Error>
where
    F: FnOnce(&mut dyn Write) -> Result<R, fmt::Error>,
{
    let c = &mut *lock_console();
    f(c)
}

pub fn print_str(s: &str) -> fmt::Result {
    lock_console().write_str(s)
}

pub fn print(arg: Arguments<'_>) -> fmt::Result {
    lock_console().write_fmt(arg)
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::console::print(format_args!($($arg)*)).expect("print failed");
    };}
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::console::print_str("\n").expect("println failed");
    };};
    ($($arg:tt)*) => {{
        $crate::console::print(format_args_nl!($($arg)*)).expect("println failed");
    };};
}
