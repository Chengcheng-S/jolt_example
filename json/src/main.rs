pub fn main() {
    let (prove_parse, verify_parse) = guest::build_account_info();

    let data_str = r#"
            {
                "account_name": "Jane",
                
                "balance" : 2000,
            }"#;
    
    let (output, proof) = prove_parse(data_str.to_string());
    proof
    .save_to_file("./proof.json").expect("save filed");
    let is_valid = verify_parse(proof);
    println!("name: {}, balance {}", output.account_name, output.balance);
    println!("valid: {}", is_valid);
}