//! Block implement of blockchain

use super::*;
use crate::transaction::Transaction;
use bincode::serialize;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use merkle_cbt::merkle_tree::Merge;
use merkle_cbt::merkle_tree::CBMT;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

const TARGET_HEXS: usize = 4;

/// Block keeps block headers
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Block {
    Header: String,
    timestamp: u128,
    transactions: Vec<Transaction>,
    prev_block_hash: String,
    prev_block_hash_other: String,
    hash: String,
    nonce: i32,
    height: i32,
} 
impl Block {
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
    }
    /*
    pub fn get_prev_hash_other(&self) -> String {
        self.prev_block_hash_other.clone()
    }
    */
    pub fn get_transaction(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    /// NewBlock creates and returns Block
    pub fn new_block(
        transactions: Vec<Transaction>,
        prev_block_hash: String,
        prev_block_hash_other: String,
        Header: String,
        height: i32,
    ) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();
        let mut block = Block {
        
            timestamp,
            transactions,
            prev_block_hash,
            prev_block_hash_other,
            Header,
            hash: String::new(),
            nonce: 0,
            height,
        };
        block.run_proof_of_work()?;
        Ok(block)
    }

    /// NewGenesisBlock creates and returns genesis Block
    /// Add here Header
    pub fn new_genesis_block(coinbase: Transaction, chain: i32) -> Block {
        let Header = match chain {
            1 => "Chain 1",
            2 => "Chain 2",
            _ => panic!("Unknown chain index!")
        };
        let string = "Genessis block";

        Block::new_block(vec![coinbase], String::new(), string.to_string(),Header.to_string() , 0).unwrap()  
    }

    /// Run performs a proof-of-work
    fn run_proof_of_work(&mut self) -> Result<()> {
        println!("Mining the block");
        while !self.validate()? {
            self.nonce += 1;
        }

            /*
          if chain 1 
             
          */
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    /// HashTransactions returns a hash of the transactions in the block
    fn hash_transactions(&self) -> Result<Vec<u8>> {
        let mut transactions = Vec::new();
        for tx in &self.transactions {
            transactions.push(tx.hash()?.as_bytes().to_owned());
        }
        let tree = CBMT::<Vec<u8>, MergeVu8>::build_merkle_tree(transactions);

        Ok(tree.root())
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.Header.clone(),
            self.prev_block_hash.clone(),
            self.prev_block_hash_other.clone(),
            self.hash_transactions()?,
            self.timestamp,
            TARGET_HEXS,
            self.nonce,
        );
        let bytes = serialize(&content)?;
        Ok(bytes)
    }

    /// Validate validates block's PoW
    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1: Vec<u8> = Vec::new();
        vec1.resize(TARGET_HEXS, '0' as u8);
        Ok(&hasher.result_str()[0..TARGET_HEXS] == String::from_utf8(vec1)?)
    }
}

struct MergeVu8 {}

impl Merge for MergeVu8 {
    type Item = Vec<u8>;
    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        let mut hasher = Sha256::new();
        let mut data: Vec<u8> = left.clone();
        data.append(&mut right.clone());
        hasher.input(&data);
        let mut re: [u8; 32] = [0; 32];
        hasher.result(&mut re);
        re.to_vec()
    }
}
