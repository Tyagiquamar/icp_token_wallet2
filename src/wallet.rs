// Import necessary types from the `types` module
use crate::types::{Wallet, Transaction};

// Declare a static mutable variable `WALLET` that will hold an optional `Wallet` instance.
// `static mut` is used here, which allows mutable global state but is generally discouraged in Rust
// due to safety concerns. Unsafe code blocks are required to work with it.
static mut WALLET: Option<Wallet> = None;

// Function to initialize the wallet with a balance of 0.
// This function sets the global `WALLET` variable to a `Wallet` instance.
pub fn initialize_wallet() {
    unsafe {
        // Initialize `WALLET` with a `Wallet` object having a balance of 0.
        WALLET = Some(Wallet { balance: 0 });
    }
}

// Function to send tokens from the wallet to a recipient.
// It takes the recipient's name and the amount to be sent as parameters.
// Returns a message indicating whether the transaction was successful or if there was an error.
pub fn send_tokens(recipient: String, amount: u64) -> String {
    unsafe {
        // Access the mutable reference of the `WALLET`.
        let wallet = WALLET.as_mut().unwrap();
        if wallet.balance >= amount {
            // Deduct the amount from the wallet balance.
            wallet.balance -= amount;
            // Return a success message indicating the amount sent and the recipient.
            format!("Sent {} tokens to {}", amount, recipient)
        } else {
            // Return an error message if the balance is insufficient.
            "Insufficient balance.".to_string()
        }
    }
}

// Function to receive tokens and add them to the wallet's balance.
// It takes the amount to be received as a parameter.
pub fn receive_tokens(amount: u64) {
    unsafe {
        // Access the mutable reference of the `WALLET`.
        let wallet = WALLET.as_mut().unwrap();
        // Increase the wallet balance by the received amount.
        wallet.balance += amount;
    }
}

// Function to get the current balance of the wallet.
// Returns the wallet's balance as a `u64` value.
pub fn get_balance() -> u64 {
    unsafe {
        // Access the immutable reference of the `WALLET` and return its balance.
        let wallet = WALLET.as_ref().unwrap();
        wallet.balance
    }
}
