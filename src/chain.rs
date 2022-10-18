use crate::block::Block;
use crate::network::NetworkSetting;

pub struct Chain<'a> {
    pub network_conf: &'a NetworkSetting,
    pub blocks: Vec<Block<'a>>,
}

impl<'a> Chain<'a> {
    pub fn new(config: &'a NetworkSetting) -> Self {
        Chain {
            network_conf: &config,
            blocks: Vec::<Block>::new(),
        }
    }

    pub fn add_block(&mut self, block: Block<'a>) -> Option<()>{
        if block.is_valid(){
            println!("{}", block.hash);
            self.blocks.push(block);
            Some(())
        }else{
            None
        }
    }
}

