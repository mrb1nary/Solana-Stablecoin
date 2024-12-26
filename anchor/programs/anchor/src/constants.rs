use anchor_lang::prelude::*;

pub const SEED_CONFIG: &[u8] = b"config";
pub const SEED_MINT_ACC: &[u8] = b"mint_acc";

#[constant]
pub const MINT_DECIMALS: u8 = 9;
pub const LIQUIDATION_BONUS: u8 = 10;
pub const MIN_HEALTH_FACTOR: u8 = 1;
pub const LIQUIDATION_THRESHOLD: u8 = 50;
