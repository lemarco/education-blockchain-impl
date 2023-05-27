mod lib;
use lib::block::Block;

fn main() {
    let mut block = Block::new(0, [0; 32], 881670, "Genesis".to_owned());
    println!("{:?}", &block);
    block.mine();
    println!("{:?}", &block);
}
