use {
    serde::{Deserialize, Serialize},
    crate::miner::Miner,
};

// ------------------- JSON Payload (REST)

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub club_name: String,
    pub total_hash_rate: i32, //MH/s
    pub total_shares_mined: i32,
    pub total_workers_online: i32,
    pub workers_online: Vec<Miner>,
}

// ------------------- POST Request Body for new Miner

#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletRequest {
    club_name: String,
}


// ------------------- DAO(database access object) Object (DB Table Records)

pub struct WalletDAO {
    pub address: String,
    pub club_name: String,
}