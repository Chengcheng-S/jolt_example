pub fn main() {
    let (prove_into_string, verify_into_string) = guest::build_into_string();

    let (output, proof) = prove_into_string(50);
    let is_valid = verify_into_string(proof);

    println!("output: {:?}", output);
    println!("valid: {}", is_valid);


    let (prove_string_concat,verify_string_concat) = guest::build_string_concat();
    let (output, proof) = prove_string_concat(60);
    let is_valid = verify_string_concat(proof);
    println!("output: {:?}", output);
    println!("valid: {}", is_valid);
}
