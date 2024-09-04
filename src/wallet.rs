use crate::types::{Wallet, Transaction};

static mut WALLET: Option<Wallet> = None;

pub fn initialize_wallet() {
    unsafe {
        WALLET = Some(Wallet { balance: 0 });
    }
}

pub fn send_tokens(recipient: String, amount: u64) -> String {
    unsafe {
        let wallet = WALLET.as_mut().unwrap();
        if wallet.balance >= amount {
            wallet.balance -= amount;
            // Log the transaction or handle sending the tokens to the recipient
            format!("Sent {} tokens to {}", amount, recipient)
        } else {
            "Insufficient balance.".to_string()
        }
    }
}

pub fn receive_tokens(amount: u64) {
    unsafe {
        let wallet = WALLET.as_mut().unwrap();
        wallet.balance += amount;
    }
}

pub fn get_balance() -> u64 {
    unsafe {
        let wallet = WALLET.as_ref().unwrap();
        wallet.balance
    }
}
