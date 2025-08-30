use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use super::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + 32,
        seeds = [b"vault", token_mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub token_mint: Account<'info, anchor_spl::token::Mint>,
    #[account(
        init,
        payer = signer,
        token::mint = token_mint,
        token::authority = vault
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vault.token_mint = ctx.accounts.token_mint.key();
        Ok(())
    }
}