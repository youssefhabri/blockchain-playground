use sha2::{Digest, Sha256};
use crate::utils::time_now;

#[derive(Clone)]
pub struct Block {
    index: i32,
    timestamp: u64,
    data: String,
    prev_hash: String,
    hash: String,
    nonce: u32
}

impl Block {
    pub fn new(index: i32, data: &str, prev_hash: &str) -> Self {
        let mut block = Block {
            index,
            timestamp: time_now(),
            data: data.into(),
            prev_hash: prev_hash.into(),
            hash: String::new(),
            nonce: 0,
        };
        block.hash_block();

        block
    }

    pub fn new_nonce(block: &Block, nonce: u32) -> Self {
        let mut new_block = Block {
            index: block.index(),
            timestamp: block.timestamp(),
            data: block.data().into(),
            prev_hash: block.prev_hash().into(),
            hash: String::new(),
            nonce: nonce,
        };
        new_block.hash_block();

        new_block
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn prev_hash(&self) -> &str {
        self.prev_hash.as_ref()
    }

    pub fn data(&self) -> &str {
        self.data.as_ref()
    }

    pub fn hash(&self) -> &str {
        self.hash.as_ref()
    }

    pub fn nonce(&self) -> u32 {
        self.nonce
    }

    pub fn hash_block(&mut self) {
        let input: String = format!(
            "{}{}{}{}{}",
            self.index,
            self.prev_hash,
            self.data,
            time_now(),
            self.nonce
        );

        self.hash = format!("{:x}", Sha256::digest(input.as_bytes()));
    }
}
