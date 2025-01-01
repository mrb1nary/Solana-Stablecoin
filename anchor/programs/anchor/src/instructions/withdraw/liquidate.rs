use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    burn_tokens_internal, calculate_health_factor, error::CustomError, get_lamports_from_usd,
    withdraw_sol_internal_function, Collateral, GlobalConfig, SEED_CONFIG,
};

#[derive(Accounts)]

pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    pub price_oracle: Account<'info, PriceUpdateV2>,

    #[account(
        seeds = [SEED_CONFIG],
        bump= config_account.bump,
        has_one = mint_account
    )]
    pub config_account: Account<'info, GlobalConfig>,

    #[account(
        mut,
        has_one = sol_account
    )]
    pub collateral_account: Account<'info, Collateral>,
    #[account(mut)]
    pub sol_account: SystemAccount<'info>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = liquidator,
        associated_token::token_program = token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
    let health_factor = calculate_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_oracle,
    )?;

    require!(
        health_factor < ctx.accounts.config_account.minimum_health_factor,
        CustomError::AboveMinimumHealthFactor
    );

    let lamports = get_lamports_from_usd(&amount_to_burn, &ctx.accounts.price_oracle)?;

    let liquidation_bonus = lamports * ctx.accounts.config_account.liquidation_bonus / 100;

    let amount_to_liquidate = lamports + liquidation_bonus;

    msg!("*** LIQUIDATION ***");
    msg!("Bonus {}%", ctx.accounts.config_account.liquidation_bonus);
    msg!("Bonus Amount  : {:.9}", liquidation_bonus as f64 / 1e9);
    msg!("SOL Liquidated: {:.9}", amount_to_liquidate as f64 / 1e9);

    withdraw_sol_internal_function(
        &ctx.accounts.sol_account,
        &ctx.accounts.liquidator.to_account_info(),
        &ctx.accounts.system_program,
        &ctx.accounts.collateral_account.depositor,
        amount_to_liquidate,
        ctx.accounts.collateral_account.bump_sol_account,
    )?;

    burn_tokens_internal(
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.liquidator,
        &ctx.accounts.token_program,
        amount_to_burn,
    )?;

    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports();
    collateral_account.amount_minted -= amount_to_burn;

    calculate_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_oracle,
    )?;
    Ok(())
}
