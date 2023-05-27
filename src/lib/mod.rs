mod hashable;
mod time;
use hashable::Hashable;
use time::now;
type BlockHash = [u8; 32];
#[derive(Debug)]
pub struct Block {
    index: u32,
    timestamp: u128,
    prev_block_hash: BlockHash,
    hash: BlockHash,
    nonce: u64,
    payload: String,
}

impl Block {
    pub fn new(index: u32, prev_block_hash: BlockHash, nonce: u64, payload: String) -> Self {
        let timestamp = now();
        let timestamp_hash = timestamp.hash_sha256();
        let index_hash = index.hash_sha256();
        let payload_hash = payload.hash_sha256();
        let nonce_hash = nonce.hash_sha256();
        let mut block_hash_vector: Vec<u8> = vec![];
        block_hash_vector.extend(index_hash);
        block_hash_vector.extend(timestamp_hash);
        block_hash_vector.extend(prev_block_hash);
        block_hash_vector.extend(nonce_hash);
        block_hash_vector.extend(payload_hash);

        let hash = block_hash_vector.hash_sha256();
        Self {
            index,
            prev_block_hash,
            nonce,
            payload,
            hash,
            timestamp,
        }
    }
}
