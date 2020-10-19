//! Honestly, this is mostly taken from
//! [https://github.com/rust-embedded/rust-raspi3-OS-tutorials/blob/master/04_mailboxes/src/mbox.rs].

use core::{mem, ops};
use crate::asm::block_until;
use register::{
    mmio::{ReadOnly, WriteOnly},
    *,
};
use lazy_static::lazy_static;
use spin::Mutex;
use alloc::boxed::Box;

register_bitfields! {
    u32,
    STATUS [
        FULL OFFSET(31) NUMBITS(1) [],
        EMPTY OFFSET(30) NUMBITS(1) []
    ]
}

const VIDEOCORE_MBOX: *const RegisterBlock = (super::MMIO_BASE + 0xb880) as *const _;

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    READ: ReadOnly<u32>,                     // 0x0
    __reserved_0: [u32; 5],                  // 0x4
    STATUS: ReadOnly<u32, STATUS::Register>, // 0x18
    __reserved_1: u32,                       // 0x1c
    WRITE: WriteOnly<u32>,                   // 0x20
}

#[repr(u32)]
enum Tag {
    Last = 0,
    GetSerial = 0x1_0004,
}

const REQUEST: u32 = 0;

const REQUEST_SERIAL_NUMBER: [u32; 8] = [
    8 * 4,
    REQUEST,
    Tag::GetSerial as u32,
    8,
    8,
    0xdeadbeef,
    0xdeadbabe,
    Tag::Last as u32,
];

pub struct Mailbox {
    #[doc(hidden)]
    _private_field: (),
}

lazy_static! {
    pub static ref VC_MAILBOX: Mutex<Mailbox> = Mutex::new(Mailbox {
        _private_field: (),
    });
}

impl ops::Deref for Mailbox {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*VIDEOCORE_MBOX }
    }
}

#[repr(u32)]
enum Channel {
    Prop = 8,
}

const RESPONSE_SUCCESS: u32 = 0x8000_0000;
const RESPONSE_ERROR: u32 = 0x8000_0001;

#[derive(Debug)]
enum Error {
    /// Returned if the mbox sends a buffer which is unexpectely short
    BufferTooShort,
    /// Returned if the mbox sends `RESPONSE_ERROR`
    ResponseError,
    /// Returned if the mbox sends an unknown response word
    UnknownResponse(u32),
}

unsafe fn box_slice(ptr: *mut u32, repeat: usize) -> Box<[u32]> {
    Box::from_raw(mem::transmute::<(*mut u32, usize), *mut [u32]>((ptr, repeat)))
}

unsafe fn unbox_slice_parts(b: Box<[u32]>) -> (*mut u32, usize) {
    mem::transmute(Box::into_raw(b))
}

impl Mailbox {
    unsafe fn send(&mut self, msg: Box<[u32]>, chan: Channel)
                   -> Result<Box<[u32]>, Error> {
        let (buf_ptr, _len) = unbox_slice_parts(msg);
        let buf_ptr = buf_ptr as u32;
        
        block_until(|| !self.STATUS.is_set(STATUS::FULL), 1);

        let chan = chan as u32;
        let tagged_ptr = buf_ptr as u32 | chan;
        self.WRITE.set(tagged_ptr);

        loop {
            block_until(|| !self.STATUS.is_set(STATUS::EMPTY), 1);
            let resp = self.READ.get();
            if (resp & 0xf) == chan {
                let resp = (resp & !0xf) as *mut u32;
                let len = (*resp) as usize;
                let slice = box_slice(resp, len);
                if len <= 1 {
                    return Err(Error::BufferTooShort);
                } else {
                    let res_code = slice[1];
                    match res_code {
                        RESPONSE_SUCCESS => return Ok(box_slice(resp as *mut u32, len)),
                        RESPONSE_ERROR => return Err(Error::ResponseError),
                        _ => return Err(Error::UnknownResponse(res_code)),
                    }
                }
                
            }
        }
    }
    #[no_mangle]
    pub unsafe fn read_serial_number(&mut self) -> u64 {
        let res = self.send(
            Box::new(REQUEST_SERIAL_NUMBER),
            Channel::Prop,
        ).unwrap();
        ((res[6] as u64) << 32) + (res[5] as u64)
    }
}
