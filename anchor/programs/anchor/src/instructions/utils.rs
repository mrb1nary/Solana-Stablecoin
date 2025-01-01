use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, Price, PriceUpdateV2};

use crate::error::CustomError;
use crate::{
    Collateral, GlobalConfig, FEED_ID, MAXIMUM_AGE_PRICE_FEED, PRICE_FEED_DECIMAL_ADJUSTMENT,
};

pub fn check_health_factor_internal_function(
    collateral_acc: &Account<Collateral>,
    config_acc: &Account<GlobalConfig>,
    price_oracle: &Account<PriceUpdateV2>,
) -> Result<()> {
    let health_factor = calculate_health_factor(collateral_acc, config_acc, price_oracle)?;

    require!(
        health_factor >= config_acc.minimum_health_factor.into(),
        CustomError::BelowMinimumHealthFactor
    );

    Ok(())
}

pub fn calculate_health_factor(
    collateral_acc: &Account<Collateral>,
    config_acc: &Account<GlobalConfig>,
    price_oracle: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let collateral_value_in_usd = get_usd_value(&collateral_acc.lamport_balance, price_oracle)?;

    let collateral_adjusted_after_liquidation_threshold =
        (collateral_value_in_usd * config_acc.liquidation_threshold) / 100;

    msg!(
        "Minted Amount : {:.9}",
        collateral_acc.amount_minted as f64 / 1e9
    );

    if collateral_acc.amount_minted == 0 {
        msg!("Health factor MAX");
        return Ok(u64::MAX);
    }

    let health_factor =
        (collateral_adjusted_after_liquidation_threshold) / collateral_acc.amount_minted;
    Ok(health_factor)
}

fn get_usd_value(amount_in_lamports: &u64, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price =
        price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE_PRICE_FEED, &feed_id)?;

    require!(price.price > 0, CustomError::InvalidPrice);

    let price_in_usd = price.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;

    let amount_in_usd = (*amount_in_lamports as u128 * price_in_usd) / (LAMPORTS_PER_SOL as u128);

    msg!("*** CONVERT USD TO SOL ***");
    msg!("SOL/USD Price : {:.9}", price_in_usd as f64 / 1e9);
    msg!("SOL Amount    : {:.9}", *amount_in_lamports as f64 / 1e9);
    msg!("USD Value     : {:.9}", amount_in_usd as f64 / 1e9);

    Ok(amount_in_usd as u64)
}

pub fn get_lamports_from_usd(
    amount_in_usd: &u64,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price =
        price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE_PRICE_FEED, &feed_id)?;

    // Check price is positive
    require!(price.price > 0, CustomError::InvalidPrice);

    // Adjust price to match lamports precision (9 decimals)
    // Example: Assuming 1 SOL = $2.00
    // price.price = 200_000_000 (from Pyth, 8 decimals)
    // price_in_usd = 200_000_000 * 10 = 2_000_000_000 (9 decimals)
    let price_in_usd = price.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;

    let amount_in_lamports = ((*amount_in_usd as u128) * (LAMPORTS_PER_SOL as u128)) / price_in_usd;
    msg!("*** CONVERT SOL TO USD ***");
    msg!("SOL/USD Price : {:.9}", price_in_usd as f64 / 1e9);
    msg!("USD Amount    : {:.9}", *amount_in_usd as f64 / 1e9);
    msg!("SOL Value     : {:.9}", amount_in_lamports as f64 / 1e9);

    Ok(amount_in_lamports as u64)
}
