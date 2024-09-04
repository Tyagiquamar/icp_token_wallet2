use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Wallet {
    pub balance: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub recipient: String,
    pub amount: u64,
}
