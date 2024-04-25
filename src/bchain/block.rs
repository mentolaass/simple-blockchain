pub struct Block {
    pub index: i32,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: u128,
    pub data: String,
    pub nonce: u128
}

impl Block {

    pub fn g_index(&self) -> &i32 {
        return &self.index;
    }

    pub fn g_hash(&self) -> &String {
        return &self.hash;
    }

    pub fn g_prev_hash(&self) -> &String {
        return &self.prev_hash;
    }

    pub fn g_data(&self) -> &String {
        return &self.data;
    }

    pub fn g_timestamp(&self) -> &u128 {
        return &self.timestamp;
    }

    pub fn g_nonce(&self) -> &u128 {
        return &self.nonce;
    }
}