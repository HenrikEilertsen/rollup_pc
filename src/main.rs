use std::io;

mod batch;
mod state;
mod tx;
mod utils;


fn main() {
    let mut current_batch = batch::RollupBatch::default();
    let mut account_balance = state::get_init_account_balances();
    println!("{:?}", account_balance);
    let mut pre_account_balance = std::collections::HashMap::new();
    //let mut transactions = Vec::new();
    

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
                current_batch.pre_state_root = batch::create_merkle_tree(&account_balance);
                let (updated_balance, tx_list) = tx::make_transactions(account_balance);
                current_batch.transactions = tx_list;
                account_balance = updated_balance;
                println!("{:?}", account_balance);
                current_batch.post_state_root = batch::create_merkle_tree(&account_balance);
            }
            "fraud proof" => {
                let (name, amount) = utils::parse_fraud_proof_inp(&account_balance);
                
                //create Merkle tree out of claimed account balance
                let mut claimed_account_balance = account_balance.clone();
                if let Some(value) = claimed_account_balance.get_mut(&name) {*value = amount}
                let claimed_state_root = batch::create_merkle_tree(&claimed_account_balance);
                println!("Account balance after claimed amount:{:#?}, claimed state root: {}", claimed_account_balance, claimed_state_root);

                //apply transactions to previous state and create a Merkle root for comparison
                let correct_post_account_balance = tx::apply_transactions(pre_account_balance.clone(), &current_batch.transactions);
                let correct_post_state: String = batch::create_merkle_tree(&correct_post_account_balance);
                println!("Correct state root: {}", correct_post_state);

                if correct_post_state == claimed_state_root {
                    println!("The amount is correct")
                }
                else {
                    println!("The amount is wrong")
                }


            }
            "print batch" => {
                println!("{:#?}", current_batch);
            }
            _ => println!("unkown command"),
        }


    }
}
