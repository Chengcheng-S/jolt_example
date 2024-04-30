pub async fn deal_zkp() {
    let (prove_fib, verify_fib) = guest::build_fib();

    let (output, proof) = prove_fib(186);
    proof.save_to_file("./proof.txt").expect("save filed");
    let is_valid = verify_fib(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);
}

pub async fn deal_parse_json() {
    let (prove_parse, verify_parse) = guest::build_account_info();

    let data_str = r#"
            {
                "name": "Jane Doe",
                
                "balance" : 2000,
            }"#
    .to_string();
    let (output, proof) = prove_parse(data_str);
    let is_valid = verify_parse(proof);
    println!("name: {}, balance {}", output.account_name, output.balance);
    println!("valid: {}", is_valid);
}
