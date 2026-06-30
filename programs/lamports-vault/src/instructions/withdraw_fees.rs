use crate::constants::*;
use crate::error::FeeVaultError;
use crate::state::FeeVaultState;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked};

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(
        mut,
        address = ADMIN_ADDR.parse::<Pubkey>().unwrap()
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub receiver: SystemAccount<'info>,

    pub mint_b: InterfaceAccount<'info, Mint>,

    // pda
    #[account(
        mut,
        seeds = [VAULT_STATE, admin.key().as_ref()],
        bump
    )]
    pub fee_vault_state: Account<'info, FeeVaultState>,

    // receiver ata
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = mint_b,
        associated_token::authority = receiver,
    )]
    pub receiver_ata_b: InterfaceAccount<'info, TokenAccount>,

    // fee vault
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = fee_vault_state,
    )]
    // convert admin address to Pubkey
    pub fee_vault: InterfaceAccount<'info, TokenAccount>,

    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn wtihdraw_fees_handler(ctx: Context<WithdrawFees>, amount: u64) -> Result<()> {
    let cpi_program = ctx.accounts.token_program.to_account_info();

    require!(
        amount <= ctx.accounts.fee_vault_state.total_collected,
        FeeVaultError::WithdrawalTooLarge
    );

    // seeds
    let admin_key = ctx.accounts.admin.key();
    let cpi_bump = ctx.bumps.fee_vault_state;
    let seeds = [VAULT_STATE, admin_key.as_ref(), &[cpi_bump]];

    // route
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.fee_vault.to_account_info(),
        mint: ctx.accounts.mint_b.to_account_info(),
        to: ctx.accounts.receiver_ata_b.to_account_info(),
        authority: ctx.accounts.fee_vault_state.to_account_info(),
    };

    let signer_seeds = &[&seeds[..]];

    // context
    let cpi_ctx = CpiContext::new_with_signer(cpi_program.key(), cpi_accounts, signer_seeds);

    // transfer
    anchor_spl::token_interface::transfer_checked(cpi_ctx, amount, ctx.accounts.mint_b.decimals)?;

    // update total collected
    ctx.accounts.fee_vault_state.total_collected = ctx
        .accounts
        .fee_vault_state
        .total_collected
        .checked_sub(amount)
        .unwrap();
    Ok(())
}
