// Import necessary types from the `ic_cdk` crate for Candid serialization and deserialization.
// Import the necessary traits for serialization and deserialization from the `serde` crate.
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::{Deserialize, Serialize};

// Define the `Wallet` struct with fields to represent a wallet's state.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Wallet {
    // The `balance` field represents the amount of tokens in the wallet.
    pub balance: u64,
}

// Define the `Transaction` struct to represent a transaction involving a transfer of tokens.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    // The `recipient` field holds the name or identifier of the recipient of the tokens.
    pub recipient: String,
    // The `amount` field represents the number of tokens to be transferred.
    pub amount: u64,
}
