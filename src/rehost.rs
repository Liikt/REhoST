use crate::constants;

#[inline(always)]
pub fn read_magic_value() -> [u8; 8] {
    unsafe { core::ptr::read_volatile(core::ptr::from_exposed_addr_mut(constants::MAGIC_ADDR)) }
}

#[inline(always)]
fn can_send() -> bool {
    let byte: u8 =
        unsafe { core::ptr::read_volatile(core::ptr::from_exposed_addr(constants::CHAN_ADDR)) };
    byte == 0xff
}

#[inline(always)]
fn write_buf(data: &[u8]) {
    unsafe { 
        let addr = core::ptr::from_exposed_addr_mut(constants::CHAN_ADDR);
        core::ptr::write_volatile(addr, data);
    }
}

#[inline(always)]
fn read_buf() -> [u8; 8] {
    unsafe {
        let addr = core::ptr::from_exposed_addr_mut(constants::CHAN_ADDR);
        core::ptr::read_volatile(addr)
    }
}

pub fn send_data(data: &[u8]) {
    assert!(data.len() % 8 == 0);

    let mut cur = 0;

    while cur < data.len() {
        while !can_send() {}

        write_buf(&data[cur..cur+8]);
        cur += 8;
    }
}

pub fn recv_data<const N: usize>() -> [u8; N] {
    assert!(N % 8 == 0);

    let mut cur = 0;
    let mut ret = [0; N];
    while cur < N {
        ret[cur..cur+8].copy_from_slice(&read_buf());
        cur += 8
    }
    ret
}

