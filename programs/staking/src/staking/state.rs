use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct StakingAccount {
    pub owner: Pubkey,
    pub balance: u64,
}

#[account]
#[derive(Default)]
pub struct Vault {
    pub token_mint: Pubkey,
}