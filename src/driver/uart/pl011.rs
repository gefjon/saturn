///! see [../../../doc/pl011.pdf]

use tock_registers::{
    register_bitfields,
    registers::{ReadOnly, ReadWrite, WriteOnly},
    interfaces::{Readable, Writeable},
};
use crate::{asm::block_until};
use crate::console::Console;

// there are a lot more registers, but i don't care about them
register_bitfields! {
    u16,
    /// Data Register
    ///
    /// word 0, r/w
    DR [
        overrun_error OFFSET(11) NUMBITS(1) [],
        break_error OFFSET(10) NUMBITS(1) [],
        parity_error OFFSET(9) NUMBITS(1) [],
        framing_error OFFSET(8) NUMBITS(1) [],
        data OFFSET(0) NUMBITS(8) []
    ],
    /// Flag Register
    ///
    /// at offset 0x18, which isn't a round number of words.
    FR [
        ring_indicator OFFSET(8) NUMBITS(1) [],
        trans_fifo_empty OFFSET(7) NUMBITS(1) [],
        recv_fifo_full OFFSET(6) NUMBITS(1) [],
        trans_fifo_full OFFSET(5) NUMBITS(1) [],
        recv_fifo_empty OFFSET(4) NUMBITS(1) [],
        busy OFFSET(3) NUMBITS(1) [],
        data_carrier_detect OFFSET(2) NUMBITS(1) [],
        data_set_ready OFFSET(1) NUMBITS(1) [],
        clear_to_send OFFSET(0) NUMBITS(1) []
    ]
}

define_register_block! {
    pub Pl011 {
        0x00 => dr: ReadWrite<u16, DR::Register>,
        0x18 => fr: ReadOnly<u16, FR::Register>,
    }
}

unsafe impl Send for Pl011 {}

impl Console for Pl011 {
    unsafe fn unchecked_write_byte(&mut self, byte: u8) {
        self.dr().set(byte as u16);
    }
    fn can_write(&mut self) -> bool {
        !self.fr().is_set(FR::trans_fifo_full)
    }
    unsafe fn unchecked_read_byte(&mut self) -> u8 {
        self.dr().get() as u8
    }
    fn can_read(&mut self) -> bool {
        !self.fr().is_set(FR::recv_fifo_empty)
    }
}
