use crate::lib::block::Hash;
use crate::lib::Block;
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockChainValidationError {
    MismatchIndex,
    InvalidHash,
    Achronologic,
    MismatchPreviousHash,
    InvalidGenesisBlock,
    InvalidInput,
    InsufficientValue,
    InvalidCoinbaseTx,
}
#[derive(Debug)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl BlockChain {
    // pub fn add_block(&mut self, block: Block) -> Result<(), BlockChainValidationError> {}
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            unspent_outputs: HashSet::new(),
        }
    }
    pub fn add_block(&mut self, block: Block) -> Result<(), BlockChainValidationError> {
        for (i, block) in self.blocks.iter().enumerate() {
            if i as u32 != block.index {
                return Err(BlockChainValidationError::MismatchIndex);
            }

            if i == 0 && block.prev_block_hash != [0; 32] {
                return Err(BlockChainValidationError::InvalidGenesisBlock);
            };
            let prev = &self.blocks[i - 1];
            if block.prev_block_hash != prev.hash {
                return Err(BlockChainValidationError::InvalidHash);
            };

            let prev = &self.blocks[i - 1];
            if prev.timestamp >= block.timestamp {
                return Err(BlockChainValidationError::Achronologic);
            }
            if block.prev_block_hash != prev.hash {
                return Err(BlockChainValidationError::MismatchPreviousHash);
            }
            if let Some((coinbase, txs)) = block.transactions.split_first() {
                if !coinbase.is_coinbase() {
                    return Err(BlockChainValidationError::InvalidCoinbaseTx);
                }
                let mut total_fee = 0;
                let mut block_created: HashSet<Hash> = HashSet::new();
                let mut block_spent: HashSet<Hash> = HashSet::new();
                for tx in txs {
                    let input_hashes = tx.input_hashes();
                    if !(&input_hashes - &self.unspent_outputs).is_empty()
                        || !(&input_hashes & &block_spent).is_empty()
                    {
                        return Err(BlockChainValidationError::InvalidInput);
                    }
                    let input = tx.input_sum();
                    let output = tx.output_sum();
                    if output > input {
                        return Err(BlockChainValidationError::InsufficientValue);
                    }

                    total_fee += input - output;
                    block_spent.extend(input_hashes);
                    block_created.extend(tx.output_hashes());
                }
                if coinbase.output_sum() < total_fee {
                    return Err(BlockChainValidationError::InvalidCoinbaseTx);
                } else {
                    block_created.extend(coinbase.output_hashes());
                }
                self.unspent_outputs
                    .retain(|output| !block_spent.contains(output));
                self.unspent_outputs.extend(block_created);
            }
        }
        self.blocks.push(block);
        Ok(())
    }
}
