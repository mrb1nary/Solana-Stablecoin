use crate::{
    GlobalConfig, LIQUIDATION_BONUS, LIQUIDATION_THRESHOLD, MINT_DECIMALS, MIN_HEALTH_FACTOR,
    SEED_CONFIG, SEED_MINT_ACC,
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer=authority,
        space=8 + GlobalConfig::INIT_SPACE,
        seeds = [SEED_CONFIG],
        bump,
    )]
    pub config_acc: Account<'info, GlobalConfig>,

    #[account(
        init,
        payer=authority,
        seeds=[SEED_MINT_ACC],
        bump,
        mint::decimals = MINT_DECIMALS,
        mint::authority = mint_acc,
        mint::freeze_authority = mint_acc,
        mint::token_program = token_program,
    )]
    pub mint_acc: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_config_handler(ctx: Context<InitializeConfig>) -> Result<()> {
    let config_account = &mut ctx.accounts.config_acc;

    config_account.authority = ctx.accounts.authority.key();
    config_account.mint_account = ctx.accounts.mint_acc.key();
    config_account.liquidation_threshold = LIQUIDATION_THRESHOLD;
    config_account.liquidation_bonus = LIQUIDATION_BONUS;
    config_account.minimum_health_factor = MIN_HEALTH_FACTOR;
    config_account.bump = ctx.bumps.config_acc;
    config_account.bump_mint = ctx.bumps.mint_acc;

    msg!("Initialized Global Config Account Successfully");

    Ok(())
}
