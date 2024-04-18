#![cfg_attr(feature = "guest", no_std)]
#![no_main]

#[jolt::provable]
fn div(n: u32, m: u32) -> u32 {
    match m {
        0 => panic!("division by zero"),
        _ => n / m,
    }
}

#[jolt::provable]
fn mul(x: u32, y: u32) -> u32 {
    x + y
}
