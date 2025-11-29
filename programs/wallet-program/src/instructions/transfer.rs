use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::WalletAccount;
use crate::errors::WalletError;

pub fn transfer_sol_handler(
    ctx: Context<TransferSol>,
    amount: u64,
) -> Result<()> {
    let wallet = &mut ctx.accounts.wallet;
    
    // Verify wallet ownership
    require!(
        wallet.validate_owner(&ctx.accounts.sender.key()),
        WalletError::Unauthorized
    );

    // Verify sufficient balance
    require!(
        ctx.accounts.sender.to_account_info().lamports() >= amount,
        WalletError::InsufficientBalance
    );
    
    // Transfer from sender to recipient
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
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;
    
    wallet.increment_transaction_count();

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

pub fn transfer_spl_token_handler(
    ctx: Context<TransferSPLToken>,
    amount: u64,
) -> Result<()> {
    let wallet = &mut ctx.accounts.wallet;

    // Verify wallet ownership
    require!(
        wallet.validate_owner(&ctx.accounts.authority.key()),
        WalletError::Unauthorized
    );

    // Verify sufficient token balance
    require!(
        ctx.accounts.from_token_account.amount >= amount,
        WalletError::InsufficientBalance
    );

    // Transfer tokens using the token program
    let cpi_accounts = Transfer {
        from: ctx.accounts.from_token_account.to_account_info(),
        to: ctx.accounts.to_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    token::transfer(cpi_ctx, amount)?;
    wallet.increment_transaction_count();

    emit!(TokenTransferEvent {
        from: ctx.accounts.from_token_account.key(),
        to: ctx.accounts.to_token_account.key(),
        amount,
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

#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(
        mut, 
        seeds = [b"wallet", sender.key().as_ref()], 
        bump = wallet.bump,
    )]
    pub wallet: Account<'info, WalletAccount>,

    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK: This is the recipient of the SOL transfer
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

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