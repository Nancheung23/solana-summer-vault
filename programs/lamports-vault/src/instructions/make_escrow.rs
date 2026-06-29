use crate::constants::*;
use crate::EscrowError;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed: u16)]
pub struct Make<'info> {
    // executer
    #[account(mut)]
    pub maker: Signer<'info>,

    // initialize escrow state
    #[account(
        init,
        payer = maker,
        space = ANCHOR_DISCRIMINATOR_LENGTH + Escrow::INIT_SPACE,
        seeds = [ESCROW_SEED, maker.key().as_ref(), &seed.to_le_bytes()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    // exist mints
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    // make ata account
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub make_ata_a: InterfaceAccount<'info, TokenAccount>,
    // initialized vault, is a TA
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault_a: InterfaceAccount<'info, TokenAccount>,

    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn make_handler(ctx: Context<Make>, seed: u16, amount_a: u64, amount_b: u64) -> Result<()> {
    // value check
    require!(amount_a > 0 && amount_b > 0, EscrowError::InvalidAmount);
    // initialize escrow PDA account
    ctx.accounts.escrow.set_inner(Escrow {
        // signer
        maker: ctx.accounts.maker.key(),
        // authority
        vault_authority: ctx.accounts.maker.key(),
        // key of TA (save tokens)
        vault_pda: ctx.accounts.vault_a.key(),
        mint_a: ctx.accounts.mint_a.key(),
        mint_b: ctx.accounts.mint_b.key(),
        amount_a,
        amount_b,
        // the random seed
        seed,
        bump: ctx.bumps.escrow,
        timestamp: Clock::get()?.unix_timestamp,
    });
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = TransferChecked {
        // from ata account
        from: ctx.accounts.make_ata_a.to_account_info(),
        // transfer mint a tokens
        mint: ctx.accounts.mint_a.to_account_info(),
        // to vault
        to: ctx.accounts.vault_a.to_account_info(),
        // same as signer
        authority: ctx.accounts.maker.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program.key(), cpi_accounts);
    transfer_checked(cpi_ctx, amount_a, ctx.accounts.mint_a.decimals)
}
