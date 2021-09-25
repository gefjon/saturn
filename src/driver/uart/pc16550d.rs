///! See [../../../doc/pc16550d.pdf]

use tock_registers::{
    register_bitfields,
    registers::{ReadOnly, ReadWrite, WriteOnly},
    interfaces::{Readable, Writeable}
};
use crate::{console::Console, asm::block_until};
use core::fmt::{self, Write};

register_bitfields! {
    u8,
    /// Receiver Buffer Register
    ///
    /// word 0, read-only
    RBR [
        /// Data byte received on the serial input port (sin) in UART
        /// mode, or the serial infrared input (sir_in) in infrared
        /// mode. The data in this register is valid only if the Data
        /// Ready (DR) bit in the Line Status Register (LCR) is
        /// set. If in non-FIFO mode (FIFO_MODE == NONE) or FIFOs are
        /// disabled (FCR[0] set to zero), the data in the RBR must be
        /// read before the next data arrives, otherwise it is
        /// overwritten, resulting in an over-run error. If in FIFO
        /// mode (FIFO_MODE != NONE) and FIFOs are enabled (FCR[0] set
        /// to one), this register accesses the head of the receive
        /// FIFO. If the receive FIFO is full and this register is not
        /// read before the next data character arrives, then the data
        /// already in the FIFO is preserved, but any incoming data
        /// are lost and an over-run error occurs.
        data_input OFFSET(0) NUMBITS(8) []
    ],
    /// Transmitter Holding Register
    ///
    /// word 0, write-only
    THR [
        /// Data to be transmitted on the serial output port (sout) in
        /// UART mode or the serial infrared output (sir_out_n) in
        /// infrared mode. Data should only be written to the THR when
        /// the THR Empty (THRE) bit (LSR[5]) is set. If in non-FIFO
        /// mode or FIFOs are disabled (FCR[0] = 0) and THRE is set,
        /// writing a single character to the THR clears the THRE. Any
        /// additional writes to the THR before the THRE is set again
        /// causes the THR data to be overwritten. If in FIFO mode and
        /// FIFOs are enabled (FCR[0] = 1) and THRE is set, x number
        /// of characters of data may be written to the THR before the
        /// FIFO is full. The number x (default=16) is determined by
        /// the value of FIFO Depth that you set during
        /// configuration. Any attempt to write data when the FIFO is
        /// full results in the write data being lost.
        data_output OFFSET(0) NUMBITS(8) []
    ],
    /// Divisor Latch Least Significant
    ///
    /// word 0, read-write when DLAB=1
    DLL [
        /// Lower 8-bits of a 16-bit, read/write, Divisor Latch
        /// register that contains the baud rate divisor for the
        /// UART. This register may only be accessed when the DLAB bit
        /// (LCR[7]) is set and the UART is not busy (USR[0] is
        /// zero). The output baud rate is equal to the serial clock
        /// (sclk) frequency divided by sixteen times the value of the
        /// baud rate divisor, as follows: baud rate = (serial clock
        /// freq) / (16 * divisor). Note that with the Divisor Latch
        /// Registers (DLL and DLH) set to zero, the baud clock is
        /// disabled and no serial communications occur. Also, once
        /// the DLH is set, at least 8 clock cycles of the slowest
        /// UART clock should be allowed to pass before transmitting
        /// or receiving data.
        baud_rate_divisor_L OFFSET(0) NUMBITS(8) []
    ],
    /// Interrupt Enable Register
    ///
    /// word 1, read-write
    IER [
        /// Programmable THRE Interrupt Mode Enable
        /// This is used to enable/disable the generation of THRE Interrupt.
        prog_thre_int_en OFFSET(7) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Enable Modem Status Interrupt. This is used to
        /// enable/disable the generation of Modem Status
        /// Interrupt. This is the fourth highest priority interrupt.
        modem_status_int_en OFFSET(3) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Enable Receiver Line Status Interrupt. This is used to
        /// enable/disable the generation of Receiver Line Status
        /// Interrupt. This is the highest priority interrupt.
        receive_line_status_int_en OFFSET(2) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Enable Transmit Holding Register Empty Interrupt.
        trans_hold_empty_int_en OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Enable Received Data Available Interrupt. This is used to
        /// enable/disable the generation of Received Data Available
        /// Interrupt and the Character Timeout Interrupt (if in FIFO
        /// mode and FIFOs enabled). These are the second highest
        /// priority interrupts.
        receive_data_available_int_en OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],
    /// Divisor Latch Most Significant
    ///
    /// word 1, read-write when DLAB=1
    DLM [
        /// Upper 8 bits of a 16-bit, read/write, Divisor Latch
        /// register that contains the baud rate divisor for the UART.
        baud_rate_divisor_H OFFSET(0) NUMBITS(8) []
    ],
    /// Interrupt Identify Register
    ///
    /// word 2, read-only
    IIR [
        /// FIFOs Enabled. This is used to indicate whether the FIFOs
        /// are enabled or disabled
        fifoes_en OFFSET(6) NUMBITS(2) [
            Disabled = 0b00,
            Enabled = 0b11
        ],
        /// Interrupt ID. This indicates the highest priority pending
        /// interrupt.
        int_id OFFSET(0) NUMBITS(4) [
            ModemStatus = 0,
            None = 1,
            ThrEmpty = 0b10,
            ReceiveAvail = 0b1000,
            ReceiveStatus = 0b110,
            BusyDetect = 0b111,
            CharTimeout = 0b1100
        ]
    ],
    /// FIFO Control Register
    ///
    /// word 2, write-only
    FCR [
        /// RCVR Trigger. This is used to select the trigger level in
        /// the receiver FIFO at which the Received Data Available
        /// Interrupt is generated. In auto flow control mode it is
        /// used to determine when the rts_n signal is de-asserted. It
        /// also determines when the dma_rx_req_n signal is asserted
        /// in certain modes of operation.
        rcvr_trigger OFFSET(6) NUMBITS(2) [
            /// One character in FIFO
            OneChar = 0,
            /// FIFO 1/4 full
            Quarter = 1,
            /// FIFO 1/2 full
            Half = 2,
            /// FIFO 2 less than full
            Almost = 3
        ],
        /// TX Empty Trigger. This is used to select the empty
        /// threshold level at which the THRE Interrupts are generated
        /// when the mode is active. It also determines when the
        /// dma_tx_req_n signal is asserted when in certain modes of
        /// operation.
        tx_empty_trigger OFFSET(4) NUMBITS(2) [
            /// FIFO empty
            Empty = 0,
            /// FIFO 2 less than full
            Almost = 1,
            /// FIFO 1/4 full
            Quarter = 2,
            /// FIFO 1/2 full
            Half = 3
        ],
        /// DMA Mode. This determines the DMA signalling mode used for
        /// the dma_tx_req_n and dma_rx_req_n output signals when
        /// additional DMA handshaking signals are not selected .
        dma_mode OFFSET(3) NUMBITS(1) [
            Mode0 = 0,
            /// Character timeout
            Mode11100 = 1
        ],
        /// XMIT FIFO Reset. This resets the control portion of the
        /// transmit FIFO and treats the FIFO as empty. This also
        /// de-asserts the DMA TX request and single signals when
        /// additional DMA handshaking signals are selected . Note
        /// that this bit is 'self-clearing'. It is not necessary to
        /// clear this bit.
        xmit_fifo_reset OFFSET(2) NUMBITS(1) [],
        /// RCVR FIFO Reset. This resets the control portion of the
        /// receive FIFO and treats the FIFO as empty. This also
        /// de-asserts the DMA RX request and single signals when
        /// additional DMA handshaking signals are selected. Note that
        /// this bit is 'self-clearing'. It is not necessary to clear
        /// this bit.
        rcvr_fifo_reset OFFSET(1) NUMBITS(1) [],
        /// FIFO Enable. FIFO Enable. This enables/disables the
        /// transmit (XMIT) and receive (RCVR) FIFOs. Whenever the
        /// value of this bit is changed both the XMIT and RCVR
        /// controller portion of FIFOs is reset.
        fifo_en OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],
    /// Line Control Register
    ///
    /// word 3, read-write
    LCR [
        /// Divisor Latch Access Bit. Writeable only when UART is not
        /// busy (USR[0] is zero), always readable. This bit is used
        /// to enable reading and writing of the Divisor Latch
        /// register (DLL and DLH) to set the baud rate of the
        /// UART. This bit must be cleared after initial baud rate
        /// setup in order to access other registers.
        div_lat_access OFFSET(7) NUMBITS(1) [
            Unlatched = 0,
            Latched = 1
        ],
        /// Break Control Bit. This is used to cause a break condition
        /// to be transmitted to the receiving device. If set to one
        /// the serial output is forced to the spacing (logic 0)
        /// state. When not in Loopback Mode, as determined by MCR[4],
        /// the sout line is forced low until the Break bit is
        /// cleared. If MCR[6] set to one, the sir_out_n line is
        /// continuously pulsed. When in Loopback Mode, the break
        /// condition is internally looped back to the receiver and
        /// the sir_out_n line is forced low.
        break_ctrl OFFSET(6) NUMBITS(1) [],
        /// Even Parity Select. Writeable only when UART is not busy
        /// (USR[0] is zero), always readable. This is used to select
        /// between even and odd parity, when parity is enabled (PEN
        /// set to one).
        even_parity_sel OFFSET(4) NUMBITS(1) [
            /// An odd number of logic 1s is transmitted or checked.
            OddOnes = 0,
            /// An even number of logic 1s is transmitted or checked.
            EvenOnes = 1
        ],
        /// Parity Enable. Writeable only when UART is not busy
        /// (USR[0] is zero), always readable. This bit is used to
        /// enable and disable parity generation and detection in
        /// transmitted and received serial character respectively
        parity_en OFFSET(3) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Number of stop bits. Writeable only when UART is not busy
        /// (USR[0] is zero), always readable. This is used to select
        /// the number of stop bits per character that the peripheral
        /// transmits and receives. Note that regardless of the number
        /// of stop bits selected, the receiver checks only the first
        /// stop bit.
        stop_bits_num OFFSET(2) NUMBITS(1) [
            /// If set to zero, one stop bit is transmitted in the
            /// serial data.
            OneBit = 0,
            /// If set to one and the data bits are set to 5 (LCR[1:0]
            /// set to zero) one and a half stop bits is
            /// transmitted. Otherwise, two stop bits are transmitted.
            TwoBit = 1
        ],
        /// Data Length Select. Writeable only when UART is not busy
        /// (USR[0] is zero), always readable. This is used to select
        /// the number of data bits per character that the peripheral
        /// transmits and receives.
        data_length_sel OFFSET(0) NUMBITS(2) [
            Five = 0,
            Six = 1,
            Seven = 2,
            Eight = 3
        ]
    ],
    /// Modem Control Register
    ///
    /// word 4, read-write
    MCR [
        /// SIR Mode Enable. This is used to enable/disable the IrDA SIR Mode.
        sir_mode_en OFFSET(6) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Auto Flow Control Enable.
        auto_flow_ctrl_en OFFSET(5) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        loopback OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// This is used to directly control the user-designated
        /// Output2 (out2_n) output. The value written to this
        /// location is inverted and driven out on out2_n.
        out2 OFFSET(3) NUMBITS(1) [
            Deasserted = 0,
            Asserted = 1
        ],
        /// This is used to directly control the user-designated
        /// Output2 (out2_n) output. The value written to this
        /// location is inverted and driven out on out2_n.
        out1 OFFSET(2) NUMBITS(1) [
            Deasserted = 0,
            Asserted = 1
        ],
        /// Request to Send. This is used to directly control the
        /// Request to Send (rts_n) output. The Request To Send
        /// (rts_n) output is used to inform the modem or data set
        /// that the UART is ready to exchange data.
        req_to_send OFFSET(1) NUMBITS(1) [],
        /// Data Terminal Ready. This is used to directly control the
        /// Data Terminal Ready (dtr_n) output. The value written to
        /// this location is inverted and driven out on dtr_n.
        data_terminal_ready OFFSET(0) NUMBITS(1) [
            Deasserted = 0,
            Asserted = 1
        ]
    ],
    /// Line Status Register
    ///
    /// word 5, read-only (?)
    LSR [
        /// Receiver FIFO Error bit. This bit is relevant FIFOs are
        /// enabled (FCR[0] set to one). This is used to indicate if
        /// there is at least one parity error, framing error, or
        /// break indication in the FIFO.
        receiver_fifo_error OFFSET(7) NUMBITS(1) [
            NoError = 0,
            Error = 1
        ],
        /// Transmitter Empty bit. If FIFOs enabled (FCR[0] set to
        /// one), this bit is set whenever the Transmitter Shift
        /// Register and the FIFO are both empty. If FIFOs are
        /// disabled, this bit is set whenever the Transmitter Holding
        /// Register and the Transmitter Shift Register are both
        /// empty.
        trans_empty OFFSET(6) NUMBITS(1) [
            NonEmpty = 0,
            Empty = 1
        ],
        /// Transmit Holding Register Empty bit. If THRE mode is
        /// disabled (IER[7] set to zero) and regardless of FIFO's
        /// being implemented/enabled or not, this bit indicates that
        /// the THR or TX FIFO is empty. This bit is set whenever data
        /// is transferred from the THR or TX FIFO to the transmitter
        /// shift register and no new data has been written to the THR
        /// or TX FIFO. This also causes a THRE Interrupt to occur, if
        /// the THRE Interrupt is enabled. If IER[7] set to one and
        /// FCR[0] set to one respectively, the functionality is
        /// switched to indicate the transmitter FIFO is full, and no
        /// longer controls THRE interrupts, which are then controlled
        /// by the FCR[5:4] threshold setting.
        trans_hold_reg_empty OFFSET(5) NUMBITS(1) [
            NonEmpty = 0,
            Empty = 1
        ],
        /// Break Interrupt bit. This is used to indicate the
        /// detection of a break sequence on the serial input data.
        break_int OFFSET(4) NUMBITS(1) [
            NoBreak = 0,
            Break = 1
        ],
        /// Framing Error bit. This is used to indicate the occurrence
        /// of a framing error in the receiver. A framing error occurs
        /// when the receiver does not detect a valid STOP bit in the
        /// received data.
        framing_error OFFSET(3) NUMBITS(1) [
            NoError = 0,
            Error = 1
        ],
        /// Parity Error bit. This is used to indicate the occurrence
        /// of a parity error in the receiver if the Parity Enable
        /// (PEN) bit (LCR[3]) is set.
        parity_error OFFSET(2) NUMBITS(1) [
            NoError = 0,
            Error = 1
        ],
        /// Overrun error bit. This is used to indicate the occurrence
        /// of an overrun error. This occurs if a new data character
        /// was received before the previous data was read.
        overrun_error OFFSET(1) NUMBITS(1) [
            NoError = 0,
            Error = 1
        ],
        /// Data Ready bit. This is used to indicate that the receiver
        /// contains at least one character in the RBR or the receiver
        /// FIFO.
        data_ready OFFSET(0) NUMBITS(1) [
            NoData = 0,
            Data = 1
        ]
    ],
    /// Modem Status Register
    ///
    /// word 6, read-only (?)
    MSR [
        /// Data Carrier Detect. This is used to indicate the current
        /// state of the modem control line dcd_n.
        data_carrier_detect OFFSET(7) NUMBITS(1) [],
        /// Ring Indicator. This is used to indicate the current state
        /// of the modem control line ri_n.
        ring_indicator OFFSET(6) NUMBITS(1) [],
        /// Data Set Ready. This is used to indicate the current state
        /// of the modem control line dsr_n.
        data_set_ready OFFSET(5) NUMBITS(1) [],
        /// Clear to Send. This is used to indicate the current state
        /// of the modem control line cts_n.
        clear_to_send OFFSET(4) NUMBITS(1) [],
        /// Delta Data Carrier Detect. This is used to indicate that
        /// the modem control line dcd_n has changed since the last
        /// time the MSR was read.
        delta_data_carrier_detect OFFSET(3) NUMBITS(1) [],
        /// Trailing Edge of Ring Indicator. This is used to indicate
        /// that a change on the input ri_n (from an active-low to an
        /// inactive-high state) has occurred since the last time the
        /// MSR was read.
        trailing_edge_ring_indicator OFFSET(2) NUMBITS(1) [],
        /// Delta Data Set Ready. This is used to indicate that the
        /// modem control line dsr_n has changed since the last time
        /// the MSR was read.
        delta_data_set_ready OFFSET(1) NUMBITS(1) [],
        /// Delta Clear to Send. This is used to indicate that the
        /// modem control line cts_n has changed since the last time
        /// the MSR was read.
        delta_clear_to_send OFFSET(0) NUMBITS(1) []
    ],
    /// Scratchpad Register
    ///
    /// word 7, read-write
    SCR [
        /// This register is for programmers to use as a temporary
        /// storage space.
        temp_store_space OFFSET(0) NUMBITS(8) []
    ]
}

define_register_block! {
    pub Pc16550d {
        0x00 => rbr: ReadOnly<u8, RBR::Register>,
        0x00 => thr: WriteOnly<u8, THR::Register>,
        0x00 => dll: ReadWrite<u8, DLL::Register>,
        0x04 => ier: ReadWrite<u8, IER::Register>,
        0x04 => dlm: ReadWrite<u8, DLM::Register>,
        0x08 => iir: ReadOnly<u8, IIR::Register>,
        0x08 => fcr: WriteOnly<u8, FCR::Register>,
        0x0c => lcr: ReadWrite<u8, LCR::Register>,
        0x10 => mcr: ReadWrite<u8, MCR::Register>,
        0x14 => lsr: ReadWrite<u8, LSR::Register>,
        0x18 => msr: ReadWrite<u8, MSR::Register>,
        0x1c => scr: ReadWrite<u8, SCR::Register>,
    }
}

unsafe impl Send for Pc16550d {}

impl Console for Pc16550d {
    fn can_write(&mut self) -> bool {
        self.lsr().matches_all(LSR::trans_hold_reg_empty::Empty)
    }
    unsafe fn unchecked_write_byte(&mut self, b: u8) {
        self.thr().set(b);
    }
    fn can_read(&mut self) -> bool {
        self.lsr().matches_all(LSR::data_ready::Data)
    }
    unsafe fn unchecked_read_byte(&mut self) -> u8 {
        self.rbr().get()
    }
}
