#![feature(start, exposed_provenance)]
#![allow(non_snake_case)]
#![no_std]

mod constants;
mod intrinsics;
mod rehost;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    rehost::send_data(b"letsa go");

    let key: [u8; 16] = rehost::recv_data();
    let value = rehost::read_magic_value();
    let mut magic = [0; 8];

    for x in 0..8 {
        magic[x] = value[x] ^ key[x];
    }

    if &magic == b"b00tb00t" {
        rehost::send_data(&constants::FLAG.as_bytes());
    }

    loop {}
}
