use crate::utils;
use std::{collections::HashMap};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: i32,
}

pub fn make_transactions(mut account_balance: HashMap<String, i32>) -> (HashMap<String, i32>, Vec<Transaction>) {
    let amount_of_transaction: i32 = utils::generate_random_number(5, 20);
    //not borrowing as the account_balance will change
    let keys: Vec<String> = account_balance.keys().cloned().collect();

    //to keep history of transactions for the batching
    let mut tx_list:Vec<Transaction> = Vec::new();

    for n in 1..amount_of_transaction {
        let sender: &String = utils::get_random_key(&keys, None);
        //skip broke senders
        if account_balance[sender] <= 0 {continue;}
        let receiver: &String = utils::get_random_key(&keys, Some(sender));

        let amount: i32 = utils::generate_random_number(1, account_balance[sender]);

        *account_balance.get_mut(sender).unwrap() -= amount;
        *account_balance.get_mut(receiver).unwrap() += amount;
        
        let current_tx = Transaction{sender: sender.clone(), receiver: receiver.clone(), amount};

        tx_list.push(current_tx);        

        println!("{} sends {} -> {}", sender, amount, receiver);

    }

    (account_balance, tx_list)
}


pub fn apply_transactions(mut account_balance: HashMap<String, i32>, transactions: &Vec<Transaction>) -> HashMap<String, i32> {
    for tx in transactions{
        *account_balance.get_mut(&tx.sender).unwrap() -= tx.amount;
        *account_balance.get_mut(&tx.receiver).unwrap() += tx.amount;
    }
    account_balance
}