//! Linux-specific implementations

use crate::JumpEntry;

/// Section name of
#[doc(hidden)]
#[macro_export]
macro_rules! os_static_key_sec_name {
    () => {
        "__static_keys"
    };
}

// See https://sourceware.org/binutils/docs/ld/Input-Section-Example.html, modern linkers
// will generate these two symbols indicating the start and end address of __static_keys
// section. Note that the end address is excluded.
extern "Rust" {
    /// Address of this static is the start address of __static_keys section
    #[link_name = "__start___static_keys"]
    pub static mut JUMP_ENTRY_START: JumpEntry;
    /// Address of this static is the end address of __static_keys section (excluded)
    #[link_name = "__stop___static_keys"]
    pub static mut JUMP_ENTRY_STOP: JumpEntry;
}