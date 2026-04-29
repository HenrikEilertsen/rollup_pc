use rs_merkle::{MerkleTree, MerkleProof, algorithms::Sha256};
use sha2::{Digest, Sha256 as Sha2Hasher};
use core::hash;
use std::{collections::HashMap, result};
use crate::tx::Transaction; 
#[derive(Debug)]
pub struct RollupBatch {
    pub pre_state_root: String,
    pub transactions: Vec<Transaction>,
    pub post_state_root: String,
}

//fncreating the Merkle Tree
pub fn create_merkle_tree (account_balance: &HashMap<String, i32>) -> String {
    let mut addresses: Vec<&String> = account_balance.keys().collect();
    addresses.sort();

    let leaves: Vec<[u8; 32]> = addresses.iter().map(|address|{
        let balance = account_balance.get(*address).unwrap();
        //this turns it into name:balance ("Alice:100")
        let data = format!("{}:{}", address, balance);
        
        //init hasher
        let mut hasher = Sha2Hasher::new();
        //update the hasher with data
        hasher.update(data.as_bytes());
        //hashes it
        let result = hasher.finalize();
        
        //fixed size of 0's
        let mut hash =  [0u8; 32];
        //copies result of hashing into prev declaration
        hash.copy_from_slice(&result);
        hash
    }).collect();

    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

    let root = merkle_tree.root().unwrap_or([0u8; 32]);
    hex::encode(root)
}

//fn batching everything together