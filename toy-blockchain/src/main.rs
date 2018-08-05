extern crate sha2;

mod utils;
mod block;
mod blockchain;

use crate::blockchain::Blockchain;


fn main() {
    println!("TaichiCoin Staging!\n");

    let mut blockchain = Blockchain::new();
    blockchain.add_block("Send 1 BTC to Joseph");
    blockchain.add_block("Send 2 more BTC to Joseph");

    for block in blockchain.blocks() {
        println!("Index: {}", block.index());
        println!("Prev. hash: {}", block.prev_hash());
        println!("Data: {}", block.data());
        println!("Hash: {}\n", block.hash());
    }
}
