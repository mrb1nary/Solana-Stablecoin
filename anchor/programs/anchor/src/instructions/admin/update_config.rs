use anchor_lang::prelude::*;

use crate::{GlobalConfig, SEED_CONFIG};


#[derive(Accounts)]
pub struct UpdateConfig<'info>{
    #[account(
        mut, 
        seeds = [SEED_CONFIG],
        bump = config_acc.bump,
    )]
    pub config_acc: Account<'info, GlobalConfig>
}

pub fn update_config_handler(ctx: Context<UpdateConfig>, invoker: Pubkey, min_health_factor: u64 )->Result<()>{
    
    let config_account = &mut ctx.accounts.config_acc;

    if config_account.authority != invoker {
        msg!("Unauthorized");
    }

    config_account.minimum_health_factor = min_health_factor;
    msg!("Updated Config Account Successfully!");
    
    Ok(())
}