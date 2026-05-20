use std::collections::HashMap;
use std::io::{self, Write};
use rand::RngExt;

pub fn generate_random_number(min: i32, max: i32) -> i32 {
    let mut rng = rand::rng();
    let value = rng.random_range(min..max);
    value
}

pub fn get_random_key<'a>(keys: &'a Vec<String>, not_available: Option<&String>) -> &'a String {
    let filtered_keys: Vec<&String> = if let Some(exclude) = not_available {
        keys.iter().filter(|k| *k != exclude).collect()
    } else {
        keys.iter().collect()
    };

    let mut rng = rand::rng();
    let index = rng.random_range(0..filtered_keys.len());

    filtered_keys[index]
}

pub fn parse_fraud_proof_inp(account_balances: &HashMap<String, i32>) -> (String, i32) {
    loop {
        println!("Enter in name and amount in this forman <name:x>");
        let _ = io::stdout().flush(); 
        let mut fraud_input = String::new();
        io::stdin().read_line(&mut fraud_input).unwrap();

        let trimmed_input = fraud_input.trim();
        if trimmed_input.is_empty() {
            println!("Input is empty.");
            continue;
        }
        let parts: Vec<&str> = trimmed_input.split(':').collect();
        if parts.len() != 2 {
            println!("Error: Invalid format");
            continue;
        }
        let name = parts[0].to_string();
        let amount_str = parts[1];
        let amount: i32 = match amount_str.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: '{}' is not a valid number.", amount_str);
                continue;
            }
        };

        if !account_balances.contains_key(&name) {
            println!("Error: Account '{}' is not in account list", name);
            continue;
        }

        return (name, amount);
    }
}
