use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Withdrawal too Large")]
    WithdrawalTooLarge,
}
