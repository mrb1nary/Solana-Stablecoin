use anchor_lang::prelude::*;

use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    check_health_factor_internal_function, Collateral, GlobalConfig, SEED_COLLATERAL_ACC,
    SEED_CONFIG,
};

use super::{burn_tokens_internal, withdraw_sol_internal_function};

#[derive(Accounts)]
pub struct RedeemCollateral<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump= config_acc.bump,
        has_one = mint_account,
    )]
    pub config_acc: Box<Account<'info, GlobalConfig>>,

    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACC, depositor.key().as_ref()],
        has_one = sol_account,
        has_one = associated_token_account,
        bump = collateral_acc.bump,
    )]
    pub collateral_acc: Account<'info, Collateral>,
    #[account(mut)]
    pub sol_account: SystemAccount<'info>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub associated_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub price_oracle: Account<'info, PriceUpdateV2>,
    pub system_program: Program<'info, System>,
}

pub fn redeem_collateral_handler_function(
    ctx: Context<RedeemCollateral>,
    amount_collateral: u64,
    amount_to_burn: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_acc;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() - amount_collateral;
    collateral_account.amount_minted -= amount_to_burn;

    check_health_factor_internal_function(
        &ctx.accounts.collateral_acc,
        &ctx.accounts.config_acc,
        &ctx.accounts.price_oracle,
    )?;

    burn_tokens_internal(
        &ctx.accounts.mint_account,
        &ctx.accounts.associated_token_account,
        &ctx.accounts.depositor,
        &ctx.accounts.token_program,
        amount_collateral,
    )?;

    withdraw_sol_internal_function(
        &ctx.accounts.sol_account,
        &ctx.accounts.depositor.to_account_info(),
        &ctx.accounts.system_program,
        &ctx.accounts.depositor.key(),
        amount_collateral,
        ctx.accounts.collateral_acc.bump_sol_account,
    )?;

    Ok(())
}
