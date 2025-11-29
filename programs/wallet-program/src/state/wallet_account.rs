use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct WalletAccount {
    pub owner: Pubkey,           // 32 bytes
    pub created_at: i64,         // 8 bytes
    pub transaction_count: u64,  // 8 bytes
    pub bump: u8,                // 1 byte
}

impl WalletAccount {
    pub const SPACE: usize = 32 + 8 + 8 + 1; // 49 bytes
    
    /// Initialize a new wallet account
    pub fn initialize(&mut self, owner: Pubkey, bump: u8) {
        self.owner = owner;
        self.created_at = Clock::get().unwrap().unix_timestamp;
        self.transaction_count = 0;
        self.bump = bump;
    }
    
    /// Increment transaction count
    pub fn increment_transaction_count(&mut self) {
        self.transaction_count += 1;
    }
    
    /// Validate wallet ownership
    pub fn validate_owner(&self, expected_owner: &Pubkey) -> bool {
        self.owner == *expected_owner
    }
}