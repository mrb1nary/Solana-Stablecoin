pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5o3NtP3cm4YbGw6oFXg7fZK2TiZqHnuDGfVxdZrv4CwB");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        initialize_config_handler(ctx)
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        invoker: Pubkey,
        min_health_factor: u64,
    ) -> Result<()> {
        update_config_handler(ctx, invoker, min_health_factor)
    }

    pub fn deposit_sol_and_mint_collateral(
        ctx: Context<DepositSolAndMintCollateral>,
        amount_collateral: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        deposit_sol_and_mint_collateral_handler(ctx, amount_collateral, amount_to_mint)
    }

    pub fn redeem_collateral_and_burn_tokens(
        ctx: Context<RedeemCollateral>,
        amount_collateral: u64,
        amount_to_burn: u64,
    ) -> Result<()> {
        redeem_collateral_handler_function(ctx, amount_collateral, amount_to_burn)
    }

    pub fn liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
        process_liquidate(ctx, amount_to_burn)
    }
}
