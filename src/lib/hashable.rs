use super::Block;
use crate::lib::transaction::{Output, Transaction};
pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash_sha256(&self) -> [u8; 32] {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
            .try_into()
            .unwrap()
    }
}
impl Hashable for u32 {
    fn bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Hashable for u64 {
    fn bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Hashable for u128 {
    fn bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Hashable for String {
    fn bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}
impl Hashable for Vec<u8> {
    fn bytes(&self) -> Vec<u8> {
        self.clone()
    }
}
impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend(self.inputs.iter().flat_map(|item| item.bytes()));
        output.extend(self.outputs.iter().flat_map(|item| item.bytes()));
        output
    }
}
impl Hashable for Vec<Transaction> {
    fn bytes(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend(self.iter().flat_map(|item| item.bytes()));
        output
    }
}
impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend(self.value.bytes());
        output.extend(self.to.bytes());

        output
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let timestamp_hash: [u8; 32] = self.timestamp.hash_sha256();
        let index_hash = self.index.hash_sha256();
        let payload_hash = self.transactions.hash_sha256();
        let nonce_hash = self.nonce.hash_sha256();
        let difficulty_hash = self.difficulty.hash_sha256();
        let mut block_hash_vector: Vec<u8> = vec![];
        block_hash_vector.extend(index_hash);
        block_hash_vector.extend(timestamp_hash);
        block_hash_vector.extend(self.prev_block_hash);
        block_hash_vector.extend(nonce_hash);
        block_hash_vector.extend(payload_hash);
        block_hash_vector.extend(difficulty_hash);
        block_hash_vector.hash_sha256().to_vec()
    }
}
