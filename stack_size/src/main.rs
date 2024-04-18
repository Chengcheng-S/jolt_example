pub fn main() {
    let (prove_fib, verify_fib) = guest::build_fib();

    let (output, proof) = prove_fib(10);
    let is_valid = verify_fib(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);

    let (prove_sum, verify_sum) = guest::build_sum();

    let (output, proof) = prove_sum(&[5u8; 100]);
    let is_valid = verify_sum(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);
}
