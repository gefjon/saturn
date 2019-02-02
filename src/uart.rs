#![allow(non_snake_case)]

use core::ops;
use register::{*, mmio::{ReadWrite, ReadOnly, WriteOnly}};
use crate::gpio::Gpio;

register_bitfields! {
    u32,
    AuxiliaryInterruptStatus [
        /// If set the SPI 2 module has an interrupt pending.
        SPI2_IRQ OFFSET(2) NUMBITS(1) [],
        /// If set the SPI 1 module has an interrupt pending.
        SPI1_IRQ OFFSET(1) NUMBITS(1) [],
        /// If set the mini UART has an interrupt pending.
        MINI_UART_IRQ OFFSET(0) NUMBITS(1) []
    ],
    AuxiliaryEnables [
        /// If set the SPI 2 module is enabled.
        /// If clear the SPI 2 module is disabled. That also disables
        /// any SPI 1 module register access
        SPI2_ENABLE OFFSET(2) NUMBITS(1) [],
        /// If set the SPI 1 module is enabled.
        /// If clear the SPI 1 module is disabled. That also disables
        /// any SPI 1 module register access
        SPI1_ENABLE OFFSET(1) NUMBITS(1) [],
        /// If set the mini UART is enabled. The UART will immediately
        /// start receiving data, especially if the UART1_RX line is
        /// low.
        /// If clear the mini UART is disabled. That also disables any
        /// mini UART register access
        MINI_UART_ENABLE OFFSET(0) NUMBITS(1) []
    ],
    MiniUartIoData [
        /// Data written is put in the transmit FIFO (Provided it is
        /// not full)
        /// (Only If bit 7 of the line control register (DLAB bit) is
        /// clear)
        ///
        /// Data read is taken from the receive FIFO (Provided it is
        /// not empty)
        /// (Only If bit 7 of the line control register (DLAB bit) is
        /// clear)
        DATA OFFSET(0) NUMBITS(8) []
    ],
    MiniUartInterruptEnable [
        /// Must be set to 1 to receive interrupts.
        ONES OFFSET(2) NUMBITS(2) [
            Ok = 0b11
        ],
        /// If this bit is set the interrupt line is asserted whenever
        /// the transmit FIFO is empty.
        /// If this bit is clear no transmit interrupts are generated.
        EnableTransmit OFFSET(1) NUMBITS(1) [],
        /// If this bit is set the interrupt line is asserted whenever
        /// the receive FIFO holds at least 1 byte.
        /// If this bit is clear no receive interrupts are generated.
        EnableRecieve OFFSET(0) NUMBITS(1) []
    ],
    MiniUartInterruptIdentify [
        /// Both bits always read as 1 as the FIFOs are always enabled
        FIFO_ENABLES OFFSET(6) NUMBITS(2) [],
        /// Always read as zero
        ZEROS OFFSET(4) NUMBITS(2) [],
        /// Always read as zero as the mini UART has no timeout
        /// function
        TIMEOUT OFFSET(3) NUMBITS(1) [],
        /// On read this register shows the interrupt ID bit
        /// - 00 : No interrupts
        /// - 01 : Transmit holding register empty
        /// - 10 : Receiver holds valid byte
        /// - 11 : <Not possible>
        INTERRUPT_ID OFFSET(1) NUMBITS(2) [
            None = 0b00,
            TransmitHoldingEmpty = 0b01,
            RecieverHoldsValidByte = 0b10
        ],
        /// On write:
        /// - Writing with bit 1 set will clear the receive FIFO
        /// - Writing with bit 2 set will clear the transmit FIFO
        FIFO_CLEAR OFFSET(1) NUMBITS(2) [
            Rx = 0b01,
            Tx = 0b10,
            All = 0b11
        ],
        /// This bit is clear whenever an interrupt is pending
        INTERRUPT_PENDING OFFSET(0) NUMBITS(1) []
    ],
    MiniUartLineControl [
        /// Mode the UART works in
        DATA_SIZE OFFSET(0) NUMBITS(2) [
            SevenBit = 0b00,
            EightBit = 0b11
        ]
    ],
    MiniUartModemControl [
        /// If clear the UART1_RTS line is high
        /// If set the UART1_RTS line is low
        /// This bit is ignored if the RTS is used for auto-flow
        /// control. See the Mini Uart Extra Control register
        /// description
        RTS OFFSET(1) NUMBITS(1) []
    ],
    MiniUartLineStatus [
        /// This bit is set if the transmit FIFO can accept at least
        /// one byte.
        TX_EMPTY OFFSET(5) NUMBITS(1) [],

        /// This bit is set if the receive FIFO holds at least 1
        /// symbol.
        DATA_READY OFFSET(0) NUMBITS(1) []
    ],
    MiniUartModemStatus [
        /// This bit is the inverse of the UART1_CTS input
        /// Thus:
        /// - If set the UART1_CTS pin is low
        /// - If clear the UART1_CTS pin is high
        CTS_STATUS OFFSET(5) NUMBITS(1) []
    ],
    MiniUartScratch [
        /// One whole byte extra on top of the 134217728 provided by
        /// the SDC
        SCRATCH OFFSET(0) NUMBITS(8) []
    ],
    MiniUartExtraControl [
        /// If this bit is set the mini UART transmitter is enabled.
        /// If this bit is clear the mini UART transmitter is disabled.
        TX_EN OFFSET(1) NUMBITS(1) [
            Enabled = 1,
            Disabled = 0
        ],

        /// If this bit is set the mini UART receiver is enabled.
        /// If this bit is clear the mini UART receiver is disabled.
        RX_EN OFFSET(0) NUMBITS(1) [
            Enabled = 1,
            Disabled = 0
        ]
    ],
    MiniUartExtraStatus [
        /// These bits shows how many symbols are stored in the
        /// transmit FIFO
        /// The value is in the range 0-8
        TRANSMIT_FILL_LEVEL OFFSET(24) NUMBITS(4) [],
        /// These bits shows how many symbols are stored in the receive FIFO
        /// The value is in the range 0-8
        RECIEVE_FILL_LEVEL OFFSET(16) NUMBITS(4) [],
        /// This bit is set if the transmitter is idle and the
        /// transmit FIFO is empty.
        /// It is a logic AND of bits 2 and 8
        TRANSMIT_DONE OFFSET(9) NUMBITS(1) [],
        /// If this bit is set the transmitter FIFO is empty. Thus it
        /// can accept 8 symbols.
        TRANSMIT_EMPTY OFFSET(8) NUMBITS(1) [],
        /// This bit shows the status of the UART1_CTS line.
        CTS OFFSET(7) NUMBITS(1) [],
        /// This bit shows the status of the UART1_RTS line.
        RTS OFFSET(6) NUMBITS(1) [],
        /// This is the inverse of bit 1
        TRANSMIT_FULL OFFSET(5) NUMBITS(1) [],
        /// This bit is set if there was a receiver overrun. That is:
        /// one or more characters arrived whilst the receive FIFO was
        /// full. The newly arrived characters have been
        /// discarded. This bit is cleared each time the
        /// AUX_MU_LSR_REG register is read.
        RECEIVE_OVERRUN OFFSET(4) NUMBITS(1) [],
        /// If this bit is set the transmitter is idle.
        /// If this bit is clear the transmitter is idle.
        TRANSMIT_IDLE OFFSET(3) NUMBITS(1) [],
        /// If this bit is set the receiver is idle.
        /// If this bit is clear the receiver is busy.
        /// This bit can change unless the receiver is disabled
        RECIEVE_IDLE OFFSET(2) NUMBITS(1) [],
        /// If this bit is set the mini UART transmitter FIFO can
        /// accept at least one more symbol.
        /// If this bit is clear the mini UART transmitter FIFO is
        /// full
        SPACE_AVAILABLE OFFSET(1) NUMBITS(1) [],
        /// If this bit is set the mini UART receive FIFO contains at
        /// least 1 symbol
        /// If this bit is clear the mini UART receiver FIFO is empty
        SYMBOL_AVAILABLE OFFSET(0) NUMBITS(1) []
    ],
    /// Mini Uart Baudrate
    MiniUartBaudrate [
        /// Mini UART baudrate counter
        RATE OFFSET(0) NUMBITS(16) []
    ]
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
    pub unsafe fn new() -> Self {
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
        self.AUX_MU_IO.set(byte as u32)
    }
    pub  fn recieve(&self) -> u8 {
        crate::asm::block_until(
            || self.AUX_MU_LSR.is_set(MiniUartLineStatus::DATA_READY),
            1, // TODO: figure out what the right number of cycles to
               // block for is
        );
        self.AUX_MU_IO.get() as u8
    }
    pub fn write(&self, s: &str) {
        for b in s.bytes() {
            self.send(b); 
        }
    }
}

impl ops::Deref for Uart1 {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*AUX_PERIPHERALS_BLOCK }
    }
}

const AUX_PERIPHERALS_BLOCK: *const RegisterBlock =
    (super::MMIO_BASE + 0x21_5000) as *const _;

impl RegisterBlock {
    /// Initialize UART1 on GPIO pins 14 & 15
    ///
    /// I'm gonna be honest, I have a lot of questions about this
    /// method. I'm only sort of sure what it does, but I've tried my
    /// best to convert the nasty register interaction into named
    /// methods, which hopefully makes them a bit more readable.
    unsafe fn init_uart1(&self) {
        self.AUX_ENABLES.modify(AuxiliaryEnables::MINI_UART_ENABLE::SET);
        self.AUX_MU_IER.set(0);
        self.AUX_MU_CNTL.set(0);
        self.AUX_MU_LCR.write(MiniUartLineControl::DATA_SIZE::EightBit);
        self.AUX_MU_MCR.set(0);
        self.AUX_MU_IER.set(0);
        self.AUX_MU_IIR.write(MiniUartInterruptIdentify::FIFO_CLEAR::All);
        self.AUX_MU_BAUD.write(MiniUartBaudrate::RATE.val(270)); // 115200 baud

        Gpio::new().init_uart1();
        
        // crate::gpio::gpio().init_uart1();
        self.AUX_MU_CNTL
            .write(MiniUartExtraControl::RX_EN::Enabled + MiniUartExtraControl::TX_EN::Enabled);
    }
}
