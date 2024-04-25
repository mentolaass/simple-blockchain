use self::{block::Block, calc::calc_hash};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod block;
pub mod calc;
pub mod chain;

pub fn mine_block(l_block: &Block) -> Block {
    let index: i32 = l_block.g_index() + 1;
    let timestamp: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("")
        .as_millis();
    let data: String = "Block #".to_string() + &index.to_string();
    let prev_hash = l_block.g_hash();
    
    let t_block: Block = Block {
        index: index,
        prev_hash: prev_hash.to_owned(),
        timestamp: timestamp,
        data: data,
        hash: "".to_string(),
        nonce: l_block.nonce
    };

    let mut block_hash = calc_hash(t_block);
    let t_data: String = "Block #".to_string() + &index.to_string();
    let mut nonce: u128 = 0;

    loop {
        if block_hash[0..4] == "0000".to_string() {
            return Block {
                index: index,
                prev_hash: prev_hash.to_owned(),
                timestamp: timestamp,
                data: t_data,
                hash: block_hash,
                nonce: nonce
            };
        }

        nonce += 1;

        block_hash = calc_hash(Block {
            index: index,
            prev_hash: prev_hash.to_owned(),
            timestamp: timestamp,
            data: t_data.to_owned(),
            hash: block_hash,
            nonce: nonce
        });
    }
}