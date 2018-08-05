use crate::block::Block;

const DIFFICULTY: usize = 5;

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![Block::new(0, "Genesis Block", "")],
        }
    }

    pub fn blocks(&mut self) -> &[Block] {
        self.blocks.as_mut_slice()
    }

    pub fn add_block(&mut self, data: &str) {
        let prev_block = self.blocks[self.blocks.len() - 1].clone();
        let new_block = Block::new(prev_block.index() + 1, data.into(), prev_block.hash());

        let mined_block = if self.is_mined(&new_block) {
            new_block
        } else {
            self.mine_block(new_block)
        };

        self.blocks.push(mined_block);
    }

    pub fn check_block(new_block: Block, prev_block: Block) -> bool {
        if prev_block.index() + 1 != new_block.index() {
            return false;
        } else if prev_block.hash() != new_block.prev_hash() {
            return false;
        }
        // TODO check if block hash is correct

        true
    }

    fn is_mined(&self, block: &Block) -> bool {
        let prefix = "0".repeat(DIFFICULTY);
        block.hash().starts_with(prefix.as_str())
    }

    fn mine_block(&self, block: Block) -> Block {
        let mut mined_block: Block = block.clone();

        while !self.is_mined(&mined_block) {
            mined_block = Block::new_nonce(&mined_block, mined_block.nonce() + 1);
        }

        mined_block
    }
}
