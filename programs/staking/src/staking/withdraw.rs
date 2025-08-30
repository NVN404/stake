use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use super::state::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"stake", signer.key().as_ref(), vault.key().as_ref()],
        bump,
        constraint = staking_account.owner == signer.key() @ ErrorCode::Unauthorized
    )]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(
        seeds = [b"vault", vault.token_mint.as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut, token::mint = vault.token_mint)]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut, token::mint = vault.token_mint)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        if amount > staking_account.balance {
            return Err(ErrorCode::InsufficientBalance.into());
        }
        staking_account.balance = staking_account.balance.checked_sub(amount).ok_or_else(|| {
            error!(ErrorCode::Underflow)
        })?;

        let vault_seeds = &[
            b"vault".as_ref(),
            ctx.accounts.vault.token_mint.as_ref(),
            &[ctx.bumps.vault],
        ];
        let signer_seeds = &[&vault_seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        anchor_spl::token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance for withdrawal")]
    InsufficientBalance,
    #[msg("Arithmetic underflow occurred")]
    Underflow,
    #[msg("Unauthorized access")]
    Unauthorized,
}