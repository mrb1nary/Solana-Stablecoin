use std::u64::MAX;

use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{self, get_feed_id_from_hex, PriceUpdateV2};

use crate::error::CustomError;
use crate::{
    Collateral, GlobalConfig, FEED_ID, MAXIMUM_AGE_PRICE_FEED, PRICE_FEED_DECIMAL_ADJUSTMENT,
};

pub fn check_health_factor_internal_function(
    collateralAcc: &Account<Collateral>,
    configAcc: &Account<GlobalConfig>,
    price_oracle: &Account<PriceUpdateV2>,
) -> Result<()> {
    let health_factor = calculate_health_factor(collateralAcc, configAcc, price_oracle)?;

    require!(
        health_factor >= configAcc.minimum_health_factor,
        CustomError::BelowMinimumHealthFactor
    );

    Ok(())
}

pub fn calculate_health_factor(
    collateralAcc: &Account<Collateral>,
    configAcc: &Account<GlobalConfig>,
    price_oracle: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let collateral_value_in_usd = get_usd_value(&collateralAcc.lamport_balance, price_oracle)?;

    let collateral_adjusted_after_liquidation_threshold =
        (collateral_value_in_usd * configAcc.liquidation_threshold) / 100;

    msg!(
        "Minted Amount : {:.9}",
        collateralAcc.amount_minted as f64 / 1e9
    );

    if collateralAcc.amount_minted == 0 {
        msg!("Health factor MAX");
        return Ok(u64::MAX);
    }

    let health_factor =
        (collateral_adjusted_after_liquidation_threshold) / collateralAcc.amount_minted;
    Ok(())
}

fn get_usd_value(amount_in_lamports: &u64, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price =
        price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE_PRICE_FEED, &feed_id)?;

    require!(price.price > 0, CustomError::InvalidPrice);

    let price_in_usd = price.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;
}
