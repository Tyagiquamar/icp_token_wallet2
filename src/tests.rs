#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receive_tokens() {
        receive_tokens(100);
        assert_eq!(get_balance(), 100);
    }

    #[test]
    fn test_send_tokens() {
        receive_tokens(100);
        let result = send_tokens("recipient_principal".to_string(), 50);
        assert_eq!(result, "Sent 50 tokens to recipient_principal");
        assert_eq!(get_balance(), 50);
    }

    #[test]
    fn test_insufficient_balance() {
        let result = send_tokens("recipient_principal".to_string(), 200);
        assert_eq!(result, "Insufficient balance.");
    }
}
