pub mod block;
pub mod trx;
pub mod network;
pub mod chain;

use crate::block::Block;
use crate::network::NetworkSetting;
use crate::trx::Trx;
use crate::chain::Chain;



fn main() {
    let setting: NetworkSetting = NetworkSetting::new(3);
    
    let mut chain: Chain = Chain::new(&setting);

    let mut block1: Block = Block::new(&setting);
    block1.add_trx(Trx::new("me".to_string(), "you".to_string(), 23.3, 1.2));
    block1.add_trx(Trx::new("me".to_string(), "you".to_string(), 23.3, 1.2));
    block1.add_trx(Trx::new("me".to_string(), "you".to_string(), 23.3, 1.2));
    block1.add_trx(Trx::new("me".to_string(), "you".to_string(), 23.3, 1.2));

    let mut block2: Block = Block::new(&setting);
    block2.add_trx(Trx::new("me".to_string(), "you".to_string(), 23.3, 1.2));
    block2.add_trx(Trx::new("me".to_string(), "you".to_string(), 23.3, 1.2));
    block2.add_trx(Trx::new("me".to_string(), "you".to_string(), 23.3, 1.2));
    block2.add_trx(Trx::new("me".to_string(), "you".to_string(), 23.3, 1.2));
    block2.add_trx(Trx::new("me".to_string(), "you".to_string(), 23.3, 1.2));
    block2.add_trx(Trx::new("me".to_string(), "you".to_string(), 23.3, 1.2));
    block2.add_trx(Trx::new("me".to_string(), "you".to_string(), 23.3, 1.2));

    block1.mine();
    block2.mine();

    chain.add_block(block1);
    chain.add_block(block2);
}
