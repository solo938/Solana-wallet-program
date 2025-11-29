use anchor_lang::prelude::*;
use crate::state::WalletAccount;

#[derive(Accounts)]
pub struct InitializeWallet<'info> {
    #[account(
        init, 
        payer = owner, 
        space = 8 + WalletAccount::INIT_SPACE, 
        seeds = [b"wallet", owner.key().as_ref()], 
        bump
    )]
    pub wallet: Account<'info, WalletAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeWallet>) -> Result<()> {
    let wallet = &mut ctx.accounts.wallet;
    
    // Initialize wallet account
    wallet.owner = ctx.accounts.owner.key();
    wallet.created_at = Clock::get()?.unix_timestamp;
    wallet.transaction_count = 0;
    wallet.bump = ctx.bumps.wallet;

    // Emit initialization event
    emit!(WalletInitialized {
        owner: wallet.owner,
        wallet: wallet.key(),
        timestamp: wallet.created_at,
    });

    msg!("Wallet initialized for owner: {}", wallet.owner);
    Ok(())
}

#[event]
pub struct WalletInitialized {
    pub owner: Pubkey,
    pub wallet: Pubkey,
    pub timestamp: i64,
}