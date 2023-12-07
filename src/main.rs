#![feature(start)]
#![allow(non_snake_case)]
#![no_std]

pub mod intrinsics;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    loop {}
}
