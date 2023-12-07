//! `intrinsics` implements the intrinsics needed by rustc. Because we don't need
//! float support, we won't use them (or maybe we will and people have to hook
//! these ....

#[no_mangle]
fn __hexagon_divsf3() {
    unimplemented!();
}

#[no_mangle]
fn __hexagon_muldf3() {
    unimplemented!();
}

#[no_mangle]
fn __hexagon_divdf3() {
    unimplemented!();
}

#[no_mangle]
fn __hexagon_udivdi3() {
    unimplemented!();
}

#[no_mangle]
fn __hexagon_udivsi3() {
    unimplemented!();
}
