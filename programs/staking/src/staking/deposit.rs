use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use super::state::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 32 + 8,
        seeds = [b"stake", signer.key().as_ref(), vault.key().as_ref()],
        bump
    )]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(
        mut,
        seeds = [b"vault", vault.token_mint.as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut, token::mint = vault.token_mint)]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut, token::mint = vault.token_mint)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        if staking_account.owner == Pubkey::default() {
            staking_account.owner = ctx.accounts.signer.key();
        }
        staking_account.balance = staking_account.balance.checked_add(amount).ok_or_else(|| {
            error!(ErrorCode::Overflow)
        })?;

        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        anchor_spl::token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow occurred")]
    Overflow,
}