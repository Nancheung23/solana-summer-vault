use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub maker: Pubkey,
    pub vault_authority: Pubkey,
    pub vault_pda: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
    pub seed: u16,
    pub bump: u8,
    // add timestamp field
    pub timestamp: i64,
}
