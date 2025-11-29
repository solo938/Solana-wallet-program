use anchor_lang::prelude::*;

#[error_code]
pub enum WalletError {
    #[msg("Insufficient balance")]
    InsufficientBalance,
    
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Invalid recipient")]
    InvalidRecipient,
    
    #[msg("Transaction failed")]
    TransactionFailed,
}
