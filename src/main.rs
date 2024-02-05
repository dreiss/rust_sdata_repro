#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let raw_c_str = [65u8, 0u8];
    let c_str = core::ffi::CStr::from_bytes_with_nul(&raw_c_str[..]);
    let _ = core::hint::black_box(c_str);
    loop { }
}
