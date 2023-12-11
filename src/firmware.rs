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

fn check_magic_header() -> bool {
    let header = rehost::read_magic_value();

    // Do some bit magic

    match header {
        [b'h', b'e', b'a', b'd', b'e', b'e', b'r', b'?'] => true,
        _ => false,
    }
}


#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Check if the header is set correctly
    if !check_magic_header() {
        return -1;
    }

    rehost::send_data(b"foobar");
    match &rehost::recv_data::<8>() {
        b"barfoo\x00\x00" => {}
        _ => return -1
    };

    rehost::send_data(constants::FLAG.as_bytes());

    loop {}
}
