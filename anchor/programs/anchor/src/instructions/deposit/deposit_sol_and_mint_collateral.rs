use crate::{
    check_health_factor_internal_function, Collateral, GlobalConfig, SEED_COLLATERAL_ACC,
    SEED_CONFIG, SEED_SOL_ACC,
};

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, Token2022, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use super::{deposit_sol_internal_function, mint_token_internal_function};

#[derive(Accounts)]
pub struct DepositSolAndMintCollateral<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump= config_acc.bump,
        has_one = mint_account,
    )]
    pub config_acc: Box<Account<'info, GlobalConfig>>,

    #[account(
        init_if_needed,
        payer = depositor,
        space = 8 + Collateral::INIT_SPACE,
        seeds = [SEED_COLLATERAL_ACC, depositor.key().as_ref()],
        bump,
    )]
    pub collateral_acc: Account<'info, Collateral>,

    #[account(
        mut,
        seeds=[SEED_SOL_ACC, depositor.key().as_ref()],
        bump,
    )]
    pub sol_acc: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_account,
        associated_token::authority = depositor,
        associated_token::token_program = token_program,
    )]
    pub associated_token_acc: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub price_oracle: Account<'info, PriceUpdateV2>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_sol_and_mint_collateral_handler(
    ctx: Context<DepositSolAndMintCollateral>,
    amount_collateral: u64,
    amount_to_mint: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_acc;

    collateral_account.lamport_balance = ctx.accounts.sol_acc.lamports() + amount_collateral;
    collateral_account.amount_minted += amount_to_mint;

    if !collateral_account.is_init {
        collateral_account.is_init = true;
        collateral_account.depositor = ctx.accounts.depositor.key();
        collateral_account.sol_account = ctx.accounts.sol_acc.key();
        collateral_account.associated_token_account = ctx.accounts.associated_token_acc.key();
        collateral_account.bump = ctx.bumps.collateral_acc;
        collateral_account.bump_sol_account = ctx.bumps.sol_acc;
    }

    check_health_factor_internal_function(
        &ctx.accounts.collateral_acc,
        &ctx.accounts.config_acc,
        &ctx.accounts.price_oracle,
    )?;

    deposit_sol_internal_function(
        &ctx.accounts.depositor,
        &ctx.accounts.sol_acc,
        &ctx.accounts.system_program,
        amount_collateral,
    )?;

    mint_token_internal_function(
        &ctx.accounts.mint_account,
        &ctx.accounts.associated_token_acc,
        &ctx.accounts.token_program,
        ctx.accounts.config_acc.bump_mint,
        amount_to_mint,
    )?;

    Ok(())
}
