use std::collections::HashMap;
use rand::RngExt;


//get the state
pub fn get_init_account_balances()  -> HashMap<String, i32> {
    let names = ["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Heidi"];
    let mut account_balance: HashMap<String, i32> = HashMap::new();

    let mut rng = rand::rng();
    for name in names.iter() {
        let value = rng.random_range(0..100);
        account_balance.insert(name.to_string(), value);
    }

    account_balance

}

