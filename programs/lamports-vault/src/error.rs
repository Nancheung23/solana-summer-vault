use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Withdrawal too Large")]
    WithdrawalTooLarge,
}

#[error_code]
pub enum EscrowError {
    #[msg("amount of tokens should be over 0")]
    InvalidAmount,
}
