use crate::constants::*;
use crate::error::FeeVaultError;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct FeeVaultState {
    pub admin: Pubkey,
    pub total_collected: u64,
    pub bump: u8,
}

impl FeeVaultState {
    // impl validate withdraw
    pub fn validate_withdraw(&self) -> Result<()> {
        require!(
            self.total_collected >= MIN_WITHDRAW_AMOUNT,
            FeeVaultError::WithdrawalTooLittle
        );
        Ok(())
    }
}
