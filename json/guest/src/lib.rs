#![no_main]

use serde_json;
use types::Account;

#[jolt::provable]
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

#[jolt::provable(stack_size = 10000000, memory_size = 10000000)]
fn account_info(info: String) -> Account {
    
    let person: Account = serde_json::from_str(&info).expect("parse json to Account failed");

    println!("{:#?}", person);
    person
}
