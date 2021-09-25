use spin::MutexGuard;
use crate::asm::block_until;
use core::fmt;
use core::ops::DerefMut;

pub use crate::board::console::CONSOLE;

pub trait Console {
    /// write `byte` to `self` without first verifying that `self` is ready to recieve a
    /// byte.
    unsafe fn unchecked_write_byte(&mut self, byte: u8);
    fn can_write(&mut self) -> bool;
    fn blocking_write_byte(&mut self, byte: u8) {
        block_until(|| self.can_write(), 1);
        unsafe { self.unchecked_write_byte(byte); }
    }
    fn write_str(&mut self, s: &str) {
        for b in s.bytes() {
            self.blocking_write_byte(b);
        }
    }
}

struct ConsoleWriter<T>(T);

impl<T, U> fmt::Write for ConsoleWriter<T>
where T: DerefMut<Target = U>,
      U: Console,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s);
        Ok(())
    }
}

pub fn lock_console() -> MutexGuard<'static, impl Console> {
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
    F: FnOnce(&mut dyn fmt::Write) -> Result<R, fmt::Error>,
{
    f(&mut ConsoleWriter(lock_console()))
}

pub fn print_str(s: &str) -> fmt::Result {
    with_console(|c| c.write_str(s))
}

pub fn print(arg: fmt::Arguments<'_>) -> fmt::Result {
    with_console(|c| c.write_fmt(arg))
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
