use crate::constants::*;
use crate::error::EscrowError;
use crate::state::EscrowState;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked};
#[derive(Accounts)]
#[instruction(seed: u16)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        seeds = [ESCROW_SEED, seed.to_le_bytes().as_ref(), maker.key().as_ref()],
        bump,
    )]
    pub escrow_state: Account<'info, EscrowState>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow_state,
    )]
    pub temporary_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn cancel_handler(ctx: Context<Cancel>, seed: u16) -> Result<()> {
    let cpi_program = ctx.accounts.token_program.to_account_info();
    // time validation
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        current_time - ctx.accounts.escrow_state.created_at >= 300,
        EscrowError::TooEarlyToCancel
    );
    let instruction_seed = seed.to_le_bytes();
    let maker_key = ctx.accounts.maker.key();
    let cpi_bump = ctx.bumps.escrow_state;
    let seeds = [
        ESCROW_SEED,
        instruction_seed.as_ref(),
        maker_key.as_ref(),
        &[cpi_bump],
    ];

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.temporary_vault.to_account_info(),
        mint: ctx.accounts.mint_a.to_account_info(),
        to: ctx.accounts.maker_ata_a.to_account_info(),
        authority: ctx.accounts.escrow_state.to_account_info(),
    };

    let signer_seeds = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program.key(), cpi_accounts, signer_seeds);
    anchor_spl::token_interface::transfer_checked(
        cpi_ctx,
        ctx.accounts.escrow_state.a_amount,
        ctx.accounts.mint_a.decimals,
    )?;
    Ok(())
}
