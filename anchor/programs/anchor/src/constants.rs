use anchor_lang::prelude::*;

pub const SEED_CONFIG: &[u8] = b"config";
pub const SEED_MINT_ACC: &[u8] = b"mint_acc";
pub const SEED_COLLATERAL_ACC: &[u8] = b"collateral_acc";
pub const SEED_SOL_ACC: &[u8] = b"sol_acc";

#[constant]
pub const MINT_DECIMALS: u8 = 9;
pub const LIQUIDATION_BONUS: u64 = 10;
pub const MIN_HEALTH_FACTOR: u64 = 1;
pub const LIQUIDATION_THRESHOLD: u64 = 50;

pub const FEED_ID: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
pub const MAXIMUM_AGE_PRICE_FEED: u64 = 60;
pub const PRICE_FEED_DECIMAL_ADJUSTMENT: u128 = 10; // price feed returns 1e8, multiple by 10 to match lamports 10e9
