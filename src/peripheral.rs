#![feature(start, exposed_provenance)]
#![allow(non_snake_case)]
#![no_std]

mod constants;
mod intrinsics;
mod rehost;

use core::panic::PanicInfo;
use groestl::{Digest, Groestl512};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let initial: [u8; 8] = rehost::recv_data();

    let mut hasher = Groestl512::default();
    hasher.update(initial);
    let hash = hasher.finalize();
    let mut secret = [0; constants::SECRET.len()];

    for x in 0..hash.len() {
        secret[x] = hash[x] ^ constants::SECRET.as_bytes()[x];
    }

    rehost::send_data(&secret);

    loop {}
}
