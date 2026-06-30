use anchor_lang::prelude::*;
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

// have to declare id otherwise instructions won't locate: Anchor.toml -> program_id
declare_id!("9bLFXWQAEm8GAkNX4NWashck88JJVtnFxyGAy9Gc5xe4");
#[program]
pub mod lamports_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize_handler(ctx)
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>, amount: u64) -> Result<()> {
        instructions::withdraw_fees_handler(ctx, amount)
    }

    pub fn make(
        ctx: Context<Make>,
        seed: u16,
        a_amount: u64,
        b_amount: u64,
        fee_bps: u16,
    ) -> Result<()> {
        instructions::make_handler(ctx, seed, a_amount, b_amount, fee_bps)
    }

    pub fn take(ctx: Context<Take>, seed: u16) -> Result<()> {
        instructions::take_handler(ctx, seed)
    }

    pub fn cancel(ctx: Context<Cancel>, seed: u16) -> Result<()> {
        instructions::cancel_handler(ctx, seed)
    }
}
