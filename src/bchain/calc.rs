use sha256::Sha256Digest;
use super::block::Block;

pub fn calc_hash(block: Block) -> String {
    let header: String = get_block_header(block);
    return Sha256Digest::digest(header);
}

fn get_block_header(block: Block) -> String {
    return block.g_index().to_string() + block.g_prev_hash() + block.g_data() + &block.g_timestamp().to_string() + &block.g_nonce().to_string();
}