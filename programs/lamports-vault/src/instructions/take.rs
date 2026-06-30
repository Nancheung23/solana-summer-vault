use crate::constants::*;
use crate::state::{EscrowState, FeeVaultState};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked};
#[derive(Accounts)]
#[instruction(seed: u16)]
pub struct Take<'info> {
    // taker
    #[account(mut)]
    pub taker: Signer<'info>,

    // maker: System Account -> verify if this wallet is on-chain
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    // mints
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    // PDA
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [ESCROW_SEED, seed.to_le_bytes().as_ref(), maker.key().as_ref()],
        bump
    )]
    pub escrow_state: Account<'info, EscrowState>,

    // vault
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow_state
    )]
    pub temporary_vault: InterfaceAccount<'info, TokenAccount>,

    // maker's receiving ata
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // taker's deposit ata
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // taker's receive ata
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // fee vault
    #[account(mut,
    address = ADMIN_ADDR.parse::<Pubkey>().unwrap())]
    // convert admin address to Pubkey
    pub fee_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [VAULT_STATE, fee_vault.key().as_ref()],
        bump
    )]
    pub fee_vault_state: Account<'info, FeeVaultState>,
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn take_handler(ctx: Context<Take>, seed: u16) -> Result<()> {
    // token program
    let cpi_program = ctx.accounts.token_program.to_account_info();

    // calculate fee
    let fee_amount = (ctx.accounts.escrow_state.b_amount as u128
        * ctx.accounts.escrow_state.fees_bps as u128
        / 10000) as u64;
    let remain_amount = ctx.accounts.escrow_state.b_amount - fee_amount;

    // longer life of seeds
    let instruction_seed = seed.to_le_bytes();
    let maker_key = ctx.accounts.escrow_state.maker;
    let cpi_bump = ctx.bumps.escrow_state;

    // seeds
    let seeds = [
        ESCROW_SEED,
        instruction_seed.as_ref(),
        maker_key.as_ref(),
        &[cpi_bump],
    ];

    // transfer accounts
    let transfer_fees_b = TransferChecked {
        from: ctx.accounts.taker_ata_b.to_account_info(),
        mint: ctx.accounts.mint_b.to_account_info(),
        to: ctx.accounts.fee_vault.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };

    let transfer_b = TransferChecked {
        from: ctx.accounts.taker_ata_b.to_account_info(),
        mint: ctx.accounts.mint_b.to_account_info(),
        to: ctx.accounts.maker_ata_b.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };

    let transfer_a = TransferChecked {
        from: ctx.accounts.temporary_vault.to_account_info(),
        mint: ctx.accounts.mint_a.to_account_info(),
        to: ctx.accounts.taker_ata_a.to_account_info(),
        authority: ctx.accounts.escrow_state.to_account_info(),
    };

    // cpi ctxs
    let signer_seeds = &[&seeds[..]];
    // transfer to vault
    let cpi_ctx_fees = CpiContext::new(cpi_program.key(), transfer_fees_b);
    // transfer to maker
    let cpi_ctx_b = CpiContext::new(cpi_program.key(), transfer_b);
    // transfer to taker
    let cpi_ctx_a = CpiContext::new_with_signer(cpi_program.key(), transfer_a, signer_seeds);

    //transfer receive token to vault
    anchor_spl::token_interface::transfer_checked(
        cpi_ctx_fees,
        fee_amount,
        ctx.accounts.mint_b.decimals,
    )?;
    // transfer receive token to maker
    anchor_spl::token_interface::transfer_checked(
        cpi_ctx_b,
        remain_amount,
        ctx.accounts.mint_b.decimals,
    )?;
    // transfer deposit token to taker
    anchor_spl::token_interface::transfer_checked(
        cpi_ctx_a,
        ctx.accounts.escrow_state.a_amount,
        ctx.accounts.mint_a.decimals,
    )?;
    // update total_collected
    ctx.accounts.fee_vault_state.total_collected = ctx
        .accounts
        .fee_vault_state
        .total_collected
        .checked_add(fee_amount)
        .unwrap();
    Ok(())
}
