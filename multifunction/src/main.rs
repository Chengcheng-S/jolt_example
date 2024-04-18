pub fn main() {
    let (prove_div, verify_div) = guest::build_div();

    let (output, proof) = prove_div(50, 25);
    let is_valid = verify_div(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);

    let (prove_mul, verify_mul) = guest::build_mul();

    let (output, proof) = prove_mul(50, 10);
    let is_valid = verify_mul(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);
}
