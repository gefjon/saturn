/// the register crate exports a similar macro named
/// `register_structs`, which does The Wrong Thing. notably, that
/// version discards the offsets passed into it and generates a
/// `#[repr(C)]` struct, whereas this version respects the supplied
/// offsets, allowing sparse register blocks and overlapping
/// registers.
macro_rules! define_register_block {
    ($struct_vis:vis $struct_name:ident {
        $($offset:literal => $field_vis:vis $field_name:ident: $field_type:ty,)*
    }) => {
        $struct_vis struct $struct_name {
            base: *mut u8,
        }
        impl $struct_name {
            $struct_vis const unsafe fn new(base: *mut u8) -> Self { Self {
                base,
            } }
            $(
                $field_vis fn $field_name(&mut self) -> &mut $field_type { unsafe {
                    &mut *(self.base.add($offset) as *mut $field_type)
                } }
            )*
        }
    };
}

pub mod uart;
