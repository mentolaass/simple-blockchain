use std::time::{SystemTime, UNIX_EPOCH};
use bchain::{block::Block, chain::Chain, mine_block};

mod bchain;

fn main() {
    let mut blockchain: Chain = Chain::build();
    let f_block: Block = c_f_block();
    blockchain.a_block(f_block);

    loop {
        let s_bchain: usize = blockchain.g_blocks().len();
        let l_block: &Block = blockchain.g_blocks().get(s_bchain - 1).expect("");
        let g_block: Block = mine_block(l_block);

        println!("Block #{} / hash: {}", g_block.g_index(), g_block.g_hash());
        blockchain.a_block(g_block);
    }
}

fn c_f_block() -> Block {
    return Block {
        index: 0,
        data: "Block #0".to_string(),
        prev_hash: "None".to_string(),
        hash: "None".to_string(),
        nonce: 0,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("")
            .as_millis(),
    }
}