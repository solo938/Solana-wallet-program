use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("4YRCTpxEYkfGSk3t1Nb1RQ23NM2PKE58kojQoTa9x9gr");


pub mod errors;
pub mod instructions;
pub mod state;


// use errors::*;
// use instructions::*;

#[program]
pub mod wallet_program {
    use super::*;

    /// Initialize a new wallet account
    /// Fixed: Typo in function name (intialize -> initialize)
    pub fn initialize_wallet(ctx: Context<InitializeWallet>) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        // Fixed: Changed user to owner to match the account name
        wallet.owner = ctx.accounts.owner.key();
        wallet.created_at = Clock::get()?.unix_timestamp;
        wallet.transaction_count = 0;
        // Fixed: Removed dereference, bumps returns u8 directly
        wallet.bump = ctx.bumps.wallet;

        msg!("Wallet initialized for: {}", wallet.owner);
        Ok(())
    }

    /// Transfer SOL from wallet to recipient
    /// Fixed: Transfer should come from sender, not wallet PDA
    pub fn transfer_sol(
        ctx: Context<TransferSol>,
        amount: u64,
    ) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        
        // Verify sufficient balance
        require!(
            ctx.accounts.sender.lamports() >= amount,
            WalletError::InsufficientBalance
        );
        
        // Fixed: Transfer from sender to recipient, not from wallet PDA
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.sender.key(),
            &ctx.accounts.recipient.key(),
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.sender.to_account_info(),
                ctx.accounts.recipient.to_account_info(),
            ],
        )?;
        
        wallet.transaction_count += 1;

        emit!(TransferEvent {
            from: ctx.accounts.sender.key(),
            to: ctx.accounts.recipient.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Transferred {} lamports from {} to {}", 
            amount, 
            ctx.accounts.sender.key(), 
            ctx.accounts.recipient.key()
        );
        Ok(())
    }

    /// Transfer SPL tokens between token accounts
    /// Fixed: Authority should be the token account owner, not wallet PDA
    pub fn transfer_spl_token(
        ctx: Context<TransferSPLToken>,
        amount: u64,
    ) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Verify sufficient token balance
        require!(
            ctx.accounts.from_token_account.amount >= amount,
            WalletError::InsufficientBalance
        );

        // Fixed: Authority should be the signer (authority), not wallet PDA
        let cpi_accounts = Transfer {
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, amount)?;
        wallet.transaction_count += 1;

        emit!(TokenTransferEvent {
            from: ctx.accounts.from_token_account.key(),
            to: ctx.accounts.to_token_account.key(),
            amount,
            // Fixed: Added mint field to match event structure
            mint: ctx.accounts.from_token_account.mint,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Transferred {} tokens from {} to {}", 
            amount, 
            ctx.accounts.from_token_account.key(), 
            ctx.accounts.to_token_account.key()
        );
        Ok(())
    }
}

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

// Fixed: Multiple issues in TransferSol accounts
#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(
        mut, 
        seeds = [b"wallet", sender.key().as_ref()], 
        bump = wallet.bump,
        // Removed has_one = owner since wallet.owner should match sender
    )]
    pub wallet: Account<'info, WalletAccount>,

    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK: This is the recipient of the SOL transfer
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

// Fixed: Multiple issues in TransferSPLToken accounts
#[derive(Accounts)]
pub struct TransferSPLToken<'info> {
    #[account(
        mut, 
        seeds = [b"wallet", authority.key().as_ref()], 
        bump = wallet.bump,
    )]
    pub wallet: Account<'info, WalletAccount>,

    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct WalletAccount {
    pub owner: Pubkey,
    pub created_at: i64,
    pub transaction_count: u64,
    pub bump: u8,
}

#[event]
pub struct TransferEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct TokenTransferEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub mint: Pubkey,
    pub timestamp: i64,
}

#[error_code]
pub enum WalletError {
    #[msg("Insufficient balance")]
    InsufficientBalance,
    
    #[msg("Invalid recipient")]
    InvalidRecipient,
    
    #[msg("Transaction failed")]
    TransactionFailed,
}