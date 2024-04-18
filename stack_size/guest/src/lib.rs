#![cfg_attr(feature = "guest", no_std)]
#![no_main]

extern crate alloc;

#[jolt::provable(stack_size = 10000, memory_size = 100000)]
fn fib(n: u32) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }
    b
}

#[jolt::provable(max_input_size = 10000, max_output_size = 10000)]
fn sum(input: &[u8]) -> u32 {
    let mut sum = 0u32;

    input.iter().for_each(|x| sum += *x as u32);
    sum
}
