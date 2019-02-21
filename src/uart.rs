#![allow(non_snake_case)]

use crate::gpio::Gpio;
use core::{fmt, ops};
use lazy_static::lazy_static;
use register::mmio::{ReadOnly, ReadWrite, WriteOnly};
use spin::Mutex;

mod registers;
use registers::*;

lazy_static! {
    pub static ref UART_1: Mutex<Uart1> = Mutex::new(unsafe { Uart1::new() });
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct RegisterBlock {
    /// 0x00 - Auxiliary Interrupt Status
    AUX_IRQ: ReadOnly<u32, AuxiliaryInterruptStatus::Register>,
    /// 0x04 - Auxiliary Enables
    AUX_ENABLES: ReadWrite<u32, AuxiliaryEnables::Register>,

    /// 0x08..0x40
    __reserved_0: [u8; 0x38],

    /// 0x40 - Mini Uart I/O Data
    AUX_MU_IO: ReadWrite<u32, MiniUartIoData::Register>,
    /// 0x44 - Mini Uart Interrupt Enable
    AUX_MU_IER: ReadWrite<u32, MiniUartInterruptEnable::Register>,
    /// 0x48 - Mini Uart Interrupt Identify
    AUX_MU_IIR: WriteOnly<u32, MiniUartInterruptIdentify::Register>,
    /// 0x4C - Mini Uart Line Control
    AUX_MU_LCR: WriteOnly<u32, MiniUartLineControl::Register>,
    /// 0x50 - Mini Uart Modem Control
    AUX_MU_MCR: ReadWrite<u32, MiniUartModemControl::Register>,
    /// 0x54 - Mini Uart Line Status
    AUX_MU_LSR: ReadOnly<u32, MiniUartLineStatus::Register>,
    /// 0x58 - Mini Uart Modem Status
    AUX_MU_MSR: ReadOnly<u32, MiniUartModemStatus::Register>,
    /// 0x5C - Mini Uart Scratch
    AUX_MU_SCRATCH: ReadWrite<u32, MiniUartScratch::Register>,
    /// 0x60 - Mini Uart Extra Control
    AUX_MU_CNTL: ReadWrite<u32, MiniUartExtraControl::Register>,
    /// 0x64 - Mini Uart Extra Status
    AUX_MU_STAT: ReadOnly<u32, MiniUartExtraStatus::Register>,
    /// 0x68 - Mini Uart Baudrate
    AUX_MU_BAUD: WriteOnly<u32, MiniUartBaudrate::Register>,

    /// 0x6c..0x7e
    __reserved_1: [u8; 0x14],

    /// 0x80 - SPI 1 Control register 0
    AUX_SPI1_CNTL0: u32,
    /// 0x84 - SPI 1 Control register 1
    AUX_SPI1_CNTL1: u32,
    /// 0x88 - SPI 1 Status
    AUX_SPI1_STAT: u32,
    /// 0x8c
    __unused_0: u32,
    /// 0x90 - SPI 1 Data
    AUX_SPI1_IO: u32,
    /// 0x94 - SPI 1 Peek
    AUX_SPI1_PEEK: u32,

    /// 0x98..0xbe
    __reserved_2: [u8; 0x28],

    /// 0xC0 - SPI 2 Control register 0
    AUX_SPI2_CNTL0: u32,
    /// 0xC4 - SPI 2 Control register 1
    AUX_SPI2_CNTL1: u32,
    /// 0xC8 - SPI 2 Status
    AUX_SPI2_STAT: u32,
    /// 0xCC
    __unused_1: u32,
    /// 0xD0 - SPI 2 Data
    AUX_SPI2_IO: u32,
    /// 0xD4 - SPI 2 Peek
    AUX_SPI2_PEEK: u32,
}

pub struct Uart1 {}

impl Uart1 {
    /// Invariant: Must not be contested by another thread or a
    /// previous UART1 use
    unsafe fn new() -> Self {
        let this = Uart1 {};
        this.init_uart1();
        this
    }
    pub fn send(&self, byte: u8) {
        crate::asm::block_until(
            || self.AUX_MU_LSR.is_set(MiniUartLineStatus::TX_EMPTY),
            1, // TODO: figure out what the right number of cycles to
               // block for is
        );
        self.AUX_MU_IO.set(u32::from(byte))
    }
    pub fn recieve(&self) -> u8 {
        crate::asm::block_until(
            || self.AUX_MU_LSR.is_set(MiniUartLineStatus::DATA_READY),
            1, // TODO: figure out what the right number of cycles to
               // block for is
        );
        self.AUX_MU_IO.get() as u8
    }
}

impl fmt::Write for Uart1 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.send(b);
        }
        Ok(())
    }
}

impl ops::Deref for Uart1 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*AUX_PERIPHERALS_BLOCK }
    }
}

const AUX_PERIPHERALS_BLOCK: *const RegisterBlock = (super::MMIO_BASE + 0x21_5000) as *const _;

impl RegisterBlock {
    /// Initialize UART1 on GPIO pins 14 & 15
    ///
    /// I'm gonna be honest, I have a lot of questions about this
    /// method. I'm only sort of sure what it does because I basically
    /// just copied it from
    /// [https://github.com/rust-embedded/rust-raspi3-OS-tutorials/blob/master/03_uart1/src/uart.rs]
    unsafe fn init_uart1(&self) {
        self.AUX_ENABLES
            .modify(AuxiliaryEnables::MINI_UART_ENABLE::SET);
        self.AUX_MU_IER.set(0);
        self.AUX_MU_CNTL.set(0);
        self.AUX_MU_LCR
            .write(MiniUartLineControl::DATA_SIZE::EightBit);
        self.AUX_MU_MCR.set(0);
        self.AUX_MU_IER.set(0);
        self.AUX_MU_IIR
            .write(MiniUartInterruptIdentify::FIFO_CLEAR::All);
        self.AUX_MU_BAUD.write(MiniUartBaudrate::RATE.val(270)); // 115200 baud

        Gpio::new().init_uart1();

        self.AUX_MU_CNTL
            .write(MiniUartExtraControl::RX_EN::Enabled + MiniUartExtraControl::TX_EN::Enabled);
    }
}

#[doc(hidden)]
/// Intended only to be called by `print!`
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    UART_1.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (crate::uart::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
