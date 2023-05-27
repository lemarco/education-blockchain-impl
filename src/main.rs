mod lib;
use std::collections::HashSet;

use lib::{
    block::Block,
    transaction::{Output, Transaction},
};

use crate::lib::BlockChain;

fn main() {
    let difficulty = 0x00_00_0a_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
    let mut genesis = Block::new(
        0,
        [0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![Output {
                to: "first".to_string(),
                value: 50,
            }],
        }],
        difficulty,
    );

    genesis.mine();

    let mut blockchain = BlockChain::new();
    blockchain.add_block(genesis).expect("Failed to add block");
    println!("{:?}", blockchain);
    // blockchain.verify().unwrap();
}
