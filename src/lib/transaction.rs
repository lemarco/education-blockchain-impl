use std::collections::HashSet;

use super::{block::Hash, Hashable};

type Address = String;

#[derive(Debug)]
pub struct Output {
    pub to: Address,
    pub value: u64,
}
#[derive(Debug)]
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Transaction {
    pub fn input_sum(&self) -> u64 {
        self.inputs.iter().map(|item| item.value).sum()
    }
    pub fn output_sum(&self) -> u64 {
        self.outputs.iter().map(|item| item.value).sum()
    }
    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.inputs
            .iter()
            .map(|item| item.hash_sha256())
            .collect::<HashSet<Hash>>()
    }
    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|item| item.hash_sha256())
            .collect::<HashSet<Hash>>()
    }
    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}
