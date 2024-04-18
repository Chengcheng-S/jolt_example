#![cfg_attr(feature = "guest", no_std)]
#![no_main]

extern crate alloc;
use core::panic;

use alloc::vec::Vec;

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

#[jolt::provable]
fn alloc(n: u32) -> u32 {
    match n {
        0 => panic!("n must gaterthan zero"),
        _ => {
            let mut v = Vec::<u32>::new();
            for i in 0..=n {
                v.push(i);
            }
            v[(n - 1) as usize]
        }
    }
}
