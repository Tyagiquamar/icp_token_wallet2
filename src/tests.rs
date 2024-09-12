// This attribute marks the module as containing test code. It ensures that this module is only compiled when running tests.
#[cfg(test)]
mod tests {
    // Import everything from the parent module to access the functions being tested.
    use super::*;

    // Test the `receive_tokens` function.
    #[test]
    fn test_receive_tokens() {
        // Call `receive_tokens` to add 100 tokens to the wallet.
        receive_tokens(100);
        // Assert that the wallet balance is now 100.
        assert_eq!(get_balance(), 100);
    }

    // Test the `send_tokens` function for a successful transaction.
    #[test]
    fn test_send_tokens() {
        // First, add 100 tokens to the wallet.
        receive_tokens(100);
        // Attempt to send 50 tokens to a recipient.
        let result = send_tokens("recipient_principal".to_string(), 50);
        // Assert that the transaction was successful and the message is correct.
        assert_eq!(result, "Sent 50 tokens to recipient_principal");
        // Assert that the wallet balance is now 50 (100 - 50).
        assert_eq!(get_balance(), 50);
    }

    // Test the `send_tokens` function when the balance is insufficient.
    #[test]
    fn test_insufficient_balance() {
        // Attempt to send 200 tokens when the wallet balance is 0.
        let result = send_tokens("recipient_principal".to_string(), 200);
        // Assert that the message indicates insufficient balance.
        assert_eq!(result, "Insufficient balance.");
    }
}
