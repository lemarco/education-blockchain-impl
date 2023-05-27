use crate::lib::{difficulty::check_difficulty, hashable::Hashable, time::now};
use hex::encode;
use std::fmt::Debug;

pub type BlockHash = [u8; 32];
// #[derive(Debug)]
pub struct Block {
    pub(crate) index: u32,
    pub(crate) timestamp: u128,
    pub(crate) prev_block_hash: BlockHash,
    pub(crate) hash: BlockHash,
    pub(crate) nonce: u64,
    pub(crate) payload: String,
    pub(crate) difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block {:?} with index:{} has payload:{} and nonce:{}",
            encode(self.hash.to_vec()),
            &self.index,
            &self.payload,
            &self.nonce
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        prev_block_hash: BlockHash,
        nonce: u64,
        payload: String,
        difficulty: u128,
    ) -> Self {
        let timestamp = now();
        let timestamp_hash = timestamp.hash_sha256();
        let index_hash = index.hash_sha256();
        let payload_hash = payload.hash_sha256();
        let nonce_hash = nonce.hash_sha256();
        // let difficulty = 0x00_00_0a_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
        let difficulty_hash = difficulty.hash_sha256();
        let mut block_hash_vector: Vec<u8> = vec![];
        block_hash_vector.extend(index_hash);
        block_hash_vector.extend(timestamp_hash);
        block_hash_vector.extend(prev_block_hash);
        block_hash_vector.extend(nonce_hash);
        block_hash_vector.extend(payload_hash);
        block_hash_vector.extend(difficulty_hash);
        let hash = block_hash_vector.hash_sha256();
        Self {
            index,
            prev_block_hash,
            nonce,
            payload,
            hash,
            timestamp,
            difficulty,
        }
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::MAX) {
            self.nonce = nonce_attempt;
            let hash: [u8; 32] = self.hash_sha256();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}
