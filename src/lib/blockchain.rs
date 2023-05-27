use crate::lib::Block;

use super::{difficulty::check_difficulty, Hashable};
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if !check_difficulty(&block.hash_sha256(), block.difficulty) {
                println!("Difficulty doesn't match");
                return false;
            }
            if i as u32 != block.index {
                println!("Index doesn't match");
                return false;
            }
            if i == 0 {
                if block.prev_block_hash != [0; 32] {
                    println!("Wrong genesis block");
                    return false;
                }
                return true;
            }
            let prev = &self.blocks[i - 1];
            if prev.timestamp >= block.timestamp {
                println!("Wrong timestamp in block. Not increased");
                return false;
            }
            if block.prev_block_hash != prev.hash {
                println!("Chaining incostitant");
                return false;
            }
        }
        true
    }
}
