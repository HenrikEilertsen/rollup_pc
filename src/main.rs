use std::io;

mod batch;
mod state;
mod tx;
mod utils;


fn main() {
    let mut account_balance = state::get_init_account_balances();
    println!("{:?}", account_balance);
    let mut pre_account_balance = std::collections::HashMap::new();
    let mut transactions = Vec::new();
    

    loop {
        println!("Enter command");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("failed");

        let command = input.trim();

        match command {
            "bye" => {
                println!("bye");
                break;
            }
            "make tx" => {
                //This varible is created for the Fraud Proof
                pre_account_balance = account_balance.clone();
                let pre_state_root = batch::create_merkle_tree(&account_balance);
                let (updated_balance, tx_list) = tx::make_transactions(account_balance);
                transactions = tx_list;
                account_balance = updated_balance;
                println!("{:?}", account_balance);
                let post_state_root = batch::create_merkle_tree(&account_balance);
                //batch transactions and state root 
                let current_batch = batch::RollupBatch {pre_state_root: pre_state_root,transactions: transactions.clone(), post_state_root: post_state_root};
                println!("{:#?}", current_batch);
            }
            "fraud proof" => {
                //claim certain amount on one account
                println!("Enter in name and amount in this forman name:x");
                let mut fraud_input = String::new();
                std::io::stdin().read_line(&mut fraud_input).unwrap();
                let parts: Vec<&str> = fraud_input.trim().split(':').collect();
                let name = parts[0]; 
                let amount: i32 = parts[1].parse().unwrap();
                
                //create Merkle tree out of claimed account balance
                let mut claimed_account_balance = account_balance.clone();
                if let Some(value) = claimed_account_balance.get_mut(name) {*value = amount}
                let claimed_state_root = batch::create_merkle_tree(&claimed_account_balance);
                println!("Account balance after claimed amount:{:#?}, claimed state root: {}", claimed_account_balance, claimed_state_root);

                //apply transactions to previous state and create a Merkle root for comparison
                let correct_post_account_balance = tx::apply_transactions(pre_account_balance.clone(), &transactions);
                let correct_post_state: String = batch::create_merkle_tree(&correct_post_account_balance);
                
                println!("Correct state root: {}", correct_post_state);

                if correct_post_state == claimed_state_root {
                    println!("The amount is correct")
                }
                else {
                    println!("The amount is wrong")
                }


            }
            _ => println!("unkown command"),
        }


    }
}
