use crate::constants::*;
// EscrowError is an enum
use crate::error::EscrowError;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EscrowState {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub a_amount: u64,
    pub b_amount: u64,
    pub created_at: i64,
    pub fees_bps: u16,
    pub seed: u16,
    pub bump: u8,
}

impl EscrowState {
    // impl validation for fee rate
    pub fn validate_fee(&self) -> Result<()> {
        require!(self.fees_bps >= MAX_BPS, EscrowError::FeeTooHigh);
        Ok(())
    }
}
