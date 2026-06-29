use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

#[constant]
pub const VAULT_STATE_SEED: &[u8] = b"vault_state";

#[constant]
pub const VAULT_SEED: &[u8] = b"vault";

#[constant]
pub const ANCHOR_DISCRIMINATOR_LENGTH: usize = 8;

#[constant]
pub const ESCROW_SEED: &[u8] = b"escrow";
