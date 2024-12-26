use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Collateral {
    pub is_init: bool,
    pub depositor: Pubkey,
    pub sol_account: Pubkey,
    pub associated_token_account: Pubkey,
    pub lamport_balance: u64, //Current balance in sol_account for health check
    pub amount_minted: u64,
    pub bump: u8,
    pub bump_sol_account: u8,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct GlobalConfig {
    pub authority: Pubkey,
    pub mint_account: Pubkey,
    pub liquidation_threshold: u8,
    pub liquidation_bonus: u8,
    pub minimum_health_factor: u8,
    pub bump: u8,
    pub bump_mint: u8,
}
