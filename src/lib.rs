mod types;
mod wallet;
mod tests;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, update, query};

#[init]
fn init() {
    wallet::initialize_wallet();
}

#[update]
fn send_tokens(recipient: String, amount: u64) -> String {
    wallet::send_tokens(recipient, amount)
}

#[update]
fn receive_tokens(amount: u64) {
    wallet::receive_tokens(amount);
}

#[query]
fn get_balance() -> u64 {
    wallet::get_balance()
}
