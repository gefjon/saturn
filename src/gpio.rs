#![cfg(target_arch="aarch64")]

use core::ops;
use register::{*, mmio::{ReadOnly, ReadWrite, WriteOnly}};

register_bitfields! {
    u32,
    AlternateFunctionSelect0 [
        RESERVED OFFSET(30) NUMBITS(2) [],
        FSEL9 OFFSET(27) NUMBITS(3) [],
        FSEL8 OFFSET(24) NUMBITS(3) [],
        FSEL7 OFFSET(21) NUMBITS(3) [],
        FSEL6 OFFSET(18) NUMBITS(3) [],
        FSEL5 OFFSET(15) NUMBITS(3) [],
        FSEL4 OFFSET(12) NUMBITS(3) [],
        FSEL3 OFFSET(9) NUMBITS(3) [],
        FSEL2 OFFSET(6) NUMBITS(3) [],
        FSEL1 OFFSET(3) NUMBITS(3) [],
        FSEL0 OFFSET(0) NUMBITS(3) []
    ],
    AlternateFunctionSelect1 [
        RESERVED OFFSET(30) NUMBITS(2) [],
        FSEL19 OFFSET(27) NUMBITS(3) [],
        FSEL18 OFFSET(24) NUMBITS(3) [],
        FSEL17 OFFSET(21) NUMBITS(3) [],
        FSEL16 OFFSET(18) NUMBITS(3) [],
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            RXD0 = 0b100,
            SD7 = 0b101,
            Reserved = 0b110,
            // Unused = 0b111,
            // Unused = 0b011,
            RXD1 = 0b010
        ],
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            TXD0 = 0b100,
            SD6 = 0b101,
            Reserved = 0b110,
            // Unused = 0b111,
            // Unused = 0b011,
            TXD1 = 0b010
        ],
        FSEL13 OFFSET(9) NUMBITS(3) [],
        FSEL12 OFFSET(6) NUMBITS(3) [],
        FSEL11 OFFSET(3) NUMBITS(3) [],
        FSEL10 OFFSET(0) NUMBITS(3) []
    ],
    AlternateFunctionSelect2 [
        RESERVED OFFSET(30) NUMBITS(2) [],
        FSEL29 OFFSET(27) NUMBITS(3) [],
        FSEL28 OFFSET(24) NUMBITS(3) [],
        FSEL27 OFFSET(21) NUMBITS(3) [],
        FSEL26 OFFSET(18) NUMBITS(3) [],
        FSEL25 OFFSET(15) NUMBITS(3) [],
        FSEL24 OFFSET(12) NUMBITS(3) [],
        FSEL23 OFFSET(9) NUMBITS(3) [],
        FSEL22 OFFSET(6) NUMBITS(3) [],
        FSEL21 OFFSET(3) NUMBITS(3) [],
        FSEL20 OFFSET(0) NUMBITS(3) []
    ],
    AlternateFunctionSelect3 [
        RESERVED OFFSET(30) NUMBITS(2) [],
        FSEL39 OFFSET(27) NUMBITS(3) [],
        FSEL38 OFFSET(24) NUMBITS(3) [],
        FSEL37 OFFSET(21) NUMBITS(3) [],
        FSEL36 OFFSET(18) NUMBITS(3) [],
        FSEL35 OFFSET(15) NUMBITS(3) [],
        FSEL34 OFFSET(12) NUMBITS(3) [],
        FSEL33 OFFSET(9) NUMBITS(3) [],
        FSEL32 OFFSET(6) NUMBITS(3) [],
        FSEL31 OFFSET(3) NUMBITS(3) [],
        FSEL30 OFFSET(0) NUMBITS(3) []
    ],
    AlternateFunctionSelect4 [
        RESERVED OFFSET(30) NUMBITS(2) [],
        FSEL49 OFFSET(27) NUMBITS(3) [],
        FSEL48 OFFSET(24) NUMBITS(3) [],
        FSEL47 OFFSET(21) NUMBITS(3) [],
        FSEL46 OFFSET(18) NUMBITS(3) [],
        FSEL45 OFFSET(15) NUMBITS(3) [],
        FSEL44 OFFSET(12) NUMBITS(3) [],
        FSEL43 OFFSET(9) NUMBITS(3) [],
        FSEL42 OFFSET(6) NUMBITS(3) [],
        FSEL41 OFFSET(3) NUMBITS(3) [],
        FSEL40 OFFSET(0) NUMBITS(3) []
    ],
    AlternateFunctionSelect5 [
        RESERVED OFFSET(30) NUMBITS(2) [],
        FSEL53 OFFSET(9) NUMBITS(3) [],
        FSEL52 OFFSET(6) NUMBITS(3) [],
        FSEL51 OFFSET(3) NUMBITS(3) [],
        FSEL50 OFFSET(0) NUMBITS(3) []
    ],
    
    OutputSet0 [
        SET OFFSET(0) NUMBITS(32) []
    ],
    OutputSet1 [
        RESERVED OFFSET(22) NUMBITS(10) [],
        SET OFFSET(0) NUMBITS(22) []
    ],
    
    OutputClear0 [
        CLR OFFSET(0) NUMBITS(32) []
    ],
    OutputClear1 [
        RESERVED OFFSET(22) NUMBITS(10) [],
        CLR OFFSET(0) NUMBITS(22) []
    ],
    
    PinLevel0 [
        LEV OFFSET(0) NUMBITS(32) []
    ],
    PinLevel1 [
        RESERVED OFFSET(22) NUMBITS(10) [],
        LEV OFFSET(0) NUMBITS(22) []
    ],
    
    EventDetectStatus0 [
        EDS OFFSET(0) NUMBITS(32) []
    ],
    EventDetectStatus1 [
        RESERVED OFFSET(22) NUMBITS(10) [],
        EDS OFFSET(0) NUMBITS(22) []
    ],
    
    RisingEdgeDetectEnable0 [
        REN OFFSET(0) NUMBITS(32) []
    ],
    RisingEdgeDetectEnable1 [
        RESERVED OFFSET(22) NUMBITS(10) [],
        REN OFFSET(0) NUMBITS(22) []
    ],
    
    FallingEdgeDetectEnable0 [
         FEN OFFSET(0) NUMBITS(32) []
    ],
    FallingEdgeDetectEnable1 [
        RESERVED OFFSET(22) NUMBITS(10) [],
        FEN OFFSET(0) NUMBITS(22) []
    ],
    
    HighDetectEnable0 [
         HEN OFFSET(0) NUMBITS(32) []
    ],
    HighDetectEnable1 [
        RESERVED OFFSET(22) NUMBITS(10) [],
        HEN OFFSET(0) NUMBITS(32) []
    ],
    
    LowDetectEnable0 [
        LEN OFFSET(0) NUMBITS(32) []
    ],
    LowDetectEnable1 [
        RESERVED OFFSET(22) NUMBITS(10) [],
        LEN OFFSET(0) NUMBITS(22) []
    ],

    AsyncRisingEdgeDetectEnable0 [
        AREN OFFSET(0) NUMBITS(32) []
    ],
    AsyncRisingEdgeDetectEnable1 [
        RESERVED OFFSET(22) NUMBITS(10) [],
        AREN OFFSET(0) NUMBITS(22) []
    ],

    AsyncFallingEdgeDetectEnable0 [
        AFEN OFFSET(0) NUMBITS(32) []
    ],
    AsyncFallingEdgeDetectEnable1 [
        RESERVED OFFSET(22) NUMBITS(10) [],
        AFEN OFFSET(0) NUMBITS(22) []
    ],

    PullUpDown [
        UNUSED OFFSET(2) NUMBITS(30) [],
        PUD OFFSET(0) NUMBITS(2) [
            Off = 0b00,
            EnablePullDownControl = 0b01,
            EnablePullUpControl = 0b10,
            Reserved = 0b11
        ]
    ],

    PullUpDownClock0 [
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ],
    PullUpDownClock1 [
        RESERVED OFFSET(22) NUMBITS(10) [],
        PUDCLK OFFSET(0) NUMBITS(22) []
    ]
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct RegisterBlock {
    /// 0x00 - Function Select 0
    FSEL_0: ReadWrite<u32, AlternateFunctionSelect0::Register>,
    /// 0x04 - Function Select 1
    FSEL_1: ReadWrite<u32, AlternateFunctionSelect1::Register>,
    /// 0x08 - Function Select 2
    FSEL_2: ReadWrite<u32, AlternateFunctionSelect2::Register>,
    /// 0x0C - Function Select 3
    FSEL_3: ReadWrite<u32, AlternateFunctionSelect3::Register>,
    /// 0x10 - Function Select 4
    FSEL_4: ReadWrite<u32, AlternateFunctionSelect4::Register>,
    /// 0x14 - Function Select 0
    FSEL_5: ReadWrite<u32, AlternateFunctionSelect5::Register>,

    /// 0x18
    __reserved_0: u32,

    /// 0x1c - Pin Output Set 0
    SET_0: WriteOnly<u32, OutputSet0::Register>,
    /// 0x20 - Pin Output Set 1
    SET_1: WriteOnly<u32, OutputSet1::Register>,

    /// 0x24
    __reserved_1: u32,

    /// 0x28 - Pin Output Clear 0
    CLR_0: WriteOnly<u32, OutputClear0::Register>,
    /// 0x2C - Pin Output Clear 1
    CLR_1: WriteOnly<u32, OutputClear1::Register>,

    /// 0x30
    __reserved_2: u32,

    /// 0x34 - Pin Level 0
    LEV_0: ReadOnly<u32, PinLevel0::Register>,
    /// 0x38 - Pin Level 1
    LEV_1: ReadOnly<u32, PinLevel1::Register>,

    /// 0x3C
    __reserved_3: u32,

    /// 0x40
    EDS_0: ReadWrite<u32, EventDetectStatus0::Register>,
    /// 0x44
    EDS_1: ReadWrite<u32, EventDetectStatus1::Register>,

    /// 0x48
    __reserved_4: u32,

    /// 0x4C
    REN_0: ReadWrite<u32, RisingEdgeDetectEnable0::Register>,
    /// 0x50
    REN_1: ReadWrite<u32, RisingEdgeDetectEnable1::Register>,

    /// 0x54
    __reserved_5: u32,

    /// 0x58
    FEN_0: ReadWrite<u32, FallingEdgeDetectEnable0::Register>,
    /// 0x5C
    FEN_1: ReadWrite<u32, FallingEdgeDetectEnable1::Register>,

    /// 0x60
    __reserved_6: u32,

    /// 0x64
    HEN_0: ReadWrite<u32, HighDetectEnable0::Register>,
    /// 0x68
    HEN_1: ReadWrite<u32, HighDetectEnable1::Register>,

    /// 0x6C
    __reserved_7: u32,

    /// 0x70
    LEN_0: ReadWrite<u32, LowDetectEnable0::Register>,
    /// 0x74
    LEN_1: ReadWrite<u32, LowDetectEnable1::Register>,

    /// 0x78
    __reserved_8: u32,

    /// 0x7C
    AREN_0: ReadWrite<u32, AsyncRisingEdgeDetectEnable0::Register>,
    /// 0x80
    AREN_1: ReadWrite<u32, AsyncRisingEdgeDetectEnable1::Register>,

    /// 0x84
    __reserved_9: u32,

    /// 0x88
    AFEN_0: ReadWrite<u32, AsyncFallingEdgeDetectEnable0::Register>,
    /// 0x8C
    AFEN_1: ReadWrite<u32, AsyncFallingEdgeDetectEnable1::Register>,

    /// 0x90
    __reserved_10: u32,

    /// 0x94
    PUD: ReadWrite<u32, PullUpDown::Register>,
    /// 0x98
    PUD_CLK_0: ReadWrite<u32, PullUpDownClock0::Register>,
    /// 0x9C
    PUD_CLK_1: ReadWrite<u32, PullUpDownClock1::Register>,
}

pub struct Gpio;

impl ops::Deref for Gpio {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*GPIO_BLOCK }
    }
}

const GPIO_BLOCK: *const RegisterBlock =
    (super::MMIO_BASE + 0x20_0000) as *const _;

impl Gpio {
    /// Invariant: must not be contested by another thread or a
    /// previously created `Gpio`
    pub unsafe fn new() -> Self {
        Gpio
    }
    /// Map pins 14 & 15 to UART1
    pub fn init_uart1(&self) {
        let alt_modes_for_pins_14_15
            = AlternateFunctionSelect1::FSEL14::TXD1
            + AlternateFunctionSelect1::FSEL15::RXD1;
        self.FSEL_1.modify(alt_modes_for_pins_14_15);

        self.PUD.set(0);

        crate::asm::block(150);

        let assert_clocks_14_15
            = PullUpDownClock0::PUDCLK15::AssertClock
            + PullUpDownClock0::PUDCLK14::AssertClock;
        self.PUD_CLK_0.write(assert_clocks_14_15);

        crate::asm::block(150);

        self.PUD_CLK_0.set(0);
    }
}
