use register::register_bitfields;

register_bitfields! {
    u32,
    pub AuxiliaryInterruptStatus [
        /// If set the SPI 2 module has an interrupt pending.
        SPI2_IRQ OFFSET(2) NUMBITS(1) [],
        /// If set the SPI 1 module has an interrupt pending.
        SPI1_IRQ OFFSET(1) NUMBITS(1) [],
        /// If set the mini UART has an interrupt pending.
        MINI_UART_IRQ OFFSET(0) NUMBITS(1) []
    ],
    pub AuxiliaryEnables [
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
    pub MiniUartIoData [
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
    pub MiniUartInterruptEnable [
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
    pub MiniUartInterruptIdentify [
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
    pub MiniUartLineControl [
        /// Mode the UART works in
        DATA_SIZE OFFSET(0) NUMBITS(2) [
            SevenBit = 0b00,
            EightBit = 0b11
        ]
    ],
    pub MiniUartModemControl [
        /// If clear the UART1_RTS line is high
        /// If set the UART1_RTS line is low
        /// This bit is ignored if the RTS is used for auto-flow
        /// control. See the Mini Uart Extra Control register
        /// description
        RTS OFFSET(1) NUMBITS(1) []
    ],
    pub MiniUartLineStatus [
        /// This bit is set if the transmit FIFO can accept at least
        /// one byte.
        TX_EMPTY OFFSET(5) NUMBITS(1) [],

        /// This bit is set if the receive FIFO holds at least 1
        /// symbol.
        DATA_READY OFFSET(0) NUMBITS(1) []
    ],
    pub MiniUartModemStatus [
        /// This bit is the inverse of the UART1_CTS input
        /// Thus:
        /// - If set the UART1_CTS pin is low
        /// - If clear the UART1_CTS pin is high
        CTS_STATUS OFFSET(5) NUMBITS(1) []
    ],
    pub MiniUartScratch [
        /// One whole byte extra on top of the 134217728 provided by
        /// the SDC
        SCRATCH OFFSET(0) NUMBITS(8) []
    ],
    pub MiniUartExtraControl [
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
    pub MiniUartExtraStatus [
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
    pub MiniUartBaudrate [
        /// Mini UART baudrate counter
        RATE OFFSET(0) NUMBITS(16) []
    ]
}
