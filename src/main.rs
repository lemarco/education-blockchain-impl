mod lib;
use lib::Block;

fn main() {
    let block = Block::new(0, [0; 32], 0, "".to_owned());
    println!("{:?}", &block);
}
