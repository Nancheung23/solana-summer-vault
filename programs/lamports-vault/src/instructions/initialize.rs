use crate::constants::*;
use crate::state::FeeVaultState;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        mut,
        address = ADMIN_ADDR.parse::<Pubkey>().unwrap()
    )]
    pub admin: Signer<'info>,

    // PDA
    #[account(
        init,
        payer = admin,
        space = DISCRIMINATOR + FeeVaultState::INIT_SPACE,
        seeds = [VAULT_STATE, admin.key().as_ref()],
        bump,
    )]
    pub fee_vault_state: Account<'info, FeeVaultState>,

    pub mint_b: InterfaceAccount<'info, Mint>,

    // vault
    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_b,
        associated_token::authority = fee_vault_state,
    )]
    pub fee_vault: InterfaceAccount<'info, TokenAccount>,

    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.fee_vault_state.total_collected = 0;
    Ok(())
}
