use anchor_lang::prelude::*;

#[error_code]
pub enum FeeVaultError {
    #[msg("Withdrawal too Large")]
    WithdrawalTooLarge,
    #[msg("Withdrawal too Little")]
    WithdrawalTooLittle,
}

#[error_code]
pub enum EscrowError {
    #[msg("amount of tokens should be over 0")]
    InvalidAmount,
    #[msg("High fees bps: over 10%")]
    FeeTooHigh,
    #[msg("Amount not match")]
    AmountNotMatch,
    #[msg("Too Early to cancel")]
    TooEarlyToCancel,
}
