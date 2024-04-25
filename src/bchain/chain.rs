use super::block::Block;

pub struct Chain {
    blocks: Vec<Block>
}

impl Chain {

    pub fn a_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn g_blocks(&self) -> &Vec<Block> {
        return &self.blocks;
    }

    pub fn build() -> Chain {
        Chain {
            blocks: vec![]
        }
    }

}