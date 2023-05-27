mod lib;
use lib::block::Block;

use crate::lib::BlockChain;

fn main() {
    let difficulty = 0x00_00_0a_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
    let mut block = Block::new(0, [0; 32], 0, "Genesis".to_owned(), difficulty);
    println!("{:?}", &block);
    block.mine();
    println!("{:?}", &block);
    let mut blockchain = BlockChain {
        blocks: vec![block],
    };
}
