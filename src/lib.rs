// Declare the modules used in this file.
// `types` and `wallet` are custom modules, while `tests` is used for unit testing.
mod types;
mod wallet;
mod tests;

// Import necessary types and traits for Candid serialization and deserialization,
// and for defining IC canister functions.
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, update, query};

// This function is called once when the canister is initialized.
// It sets up the wallet by calling `initialize_wallet` from the `wallet` module.
#[init]
fn init() {
    wallet::initialize_wallet();
}

// This update function allows sending tokens to a recipient.
// It takes the recipient's name and the amount to send as parameters,
// and calls `send_tokens` from the `wallet` module.
#[update]
fn send_tokens(recipient: String, amount: u64) -> String {
    wallet::send_tokens(recipient, amount)
}

// This update function allows receiving tokens and adding them to the wallet.
// It takes the amount of tokens to receive as a parameter,
// and calls `receive_tokens` from the `wallet` module.
#[update]
fn receive_tokens(amount: u64) {
    wallet::receive_tokens(amount);
}

// This query function allows querying the current balance of the wallet.
// It calls `get_balance` from the `wallet` module and returns the balance.
#[query]
fn get_balance() -> u64 {
    wallet::get_balance()
}
