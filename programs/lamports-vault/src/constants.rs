use anchor_lang::prelude::*;
#[constant]
pub const DISCRIMINATOR: usize = 8;

#[constant]
pub const ESCROW_SEED: &[u8] = b"escrow";

#[constant]
pub const MAX_BPS: u16 = 1000;

#[constant]
pub const MIN_WITHDRAW_AMOUNT: u64 = 1_000_000_000; // 1 sol

#[constant]
pub const ADMIN_ADDR: &str = "HNZsqu8wnc1kmRBxeFAT91ka9KBtvZ7vkELN5jJELa8c";
